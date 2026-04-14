//! Integration tests mapped from `docs/design/integration/animation-physics-test-cases.md`.

use glam::{Mat4, Quat, Vec3};
use harmonius_integration_animation_physics::{
    animation_eval_policy, apply_root_motion_for_body, bone_collider_sync,
    clamp_linear_angular_velocity, clamp_orientation_swing, clamp_to_swing_cone,
    compose_root_motion, inherit_root_linear_velocity, init_ragdoll_isometries_from_palette,
    ragdoll_recovery_fallback, recovery_blend_weight, AnimationEvalPolicy, BodyKind, BoneIndex,
    CharacterController, ColliderShape, Handle, LinearAngular, LogSink, RagdollBone,
    RagdollConstraint, RagdollDef, RagdollDefStore, RagdollRef, RagdollTransition, RootMotionDelta,
    RootMotionPipeline, WeaponColliderState,
};
use rkyv::ser::serializers::AllocSerializer;
use rkyv::ser::Serializer;
use rkyv::Deserialize;
use rkyv::Infallible;

#[derive(Default)]
struct VecLog(Vec<String>);

impl LogSink for VecLog {
    fn warn(&mut self, message: String) {
        self.0.push(message);
    }

    fn error(&mut self, message: String) {
        self.0.push(message);
    }
}

fn sample_ragdoll_def() -> RagdollDef {
    RagdollDef {
        bone_bodies: vec![
            RagdollBone {
                bone_index: BoneIndex(0),
                shape: ColliderShape::Sphere { radius: 0.2 },
                mass: 1.0,
                friction: 0.5,
                restitution: 0.1,
            },
            RagdollBone {
                bone_index: BoneIndex(1),
                shape: ColliderShape::Capsule {
                    half_height: 0.3,
                    radius: 0.05,
                },
                mass: 0.8,
                friction: 0.4,
                restitution: 0.05,
            },
        ],
        constraints: vec![RagdollConstraint {
            parent_bone: BoneIndex(0),
            child_bone: BoneIndex(1),
            twist_limit: 0.4,
            swing_limit: 0.9,
        }],
    }
}

/// TC-IR-1.3.1.U1 — RagdollDef rkyv round-trip.
#[test]
fn tc_ir_1_3_1_u1_ragdoll_def_rkyv_round_trip() {
    let def = sample_ragdoll_def();
    let mut serializer = AllocSerializer::<4096>::default();
    serializer.serialize_value(&def).unwrap();
    let bytes = serializer.into_serializer().into_inner();
    let archived = unsafe { rkyv::archived_root::<RagdollDef>(&bytes) };
    let round: RagdollDef = archived.deserialize(&mut Infallible).unwrap();
    assert_eq!(def, round);
}

/// TC-IR-1.3.1.U2 — RagdollBone handle lookup.
#[test]
fn tc_ir_1_3_1_u2_ragdoll_bone_handle_lookup() {
    let mut store = RagdollDefStore::default();
    let def = sample_ragdoll_def();
    let h = store.insert(def);
    let bone = store.bone(h, BoneIndex(1)).expect("bone 1");
    assert!(matches!(bone.shape, ColliderShape::Capsule { .. }));
}

/// TC-IR-1.3.3.U1 — Root motion compose (rigid sequence).
#[test]
fn tc_ir_1_3_3_u1_root_motion_compose() {
    let first = RootMotionDelta {
        translation: Vec3::X,
        rotation: Quat::IDENTITY,
    };
    let second = RootMotionDelta {
        translation: Vec3::Y,
        rotation: Quat::from_rotation_z(core::f32::consts::FRAC_PI_2),
    };
    let c = compose_root_motion(first, second);
    assert!((c.translation - Vec3::new(1.0, 1.0, 0.0)).length() < 1e-4);
    let expected_r = first.rotation * second.rotation;
    assert!((c.rotation - expected_r).length() < 1e-4);
}

/// TC-IR-1.3.3.U2 — Delta consumed and cleared to identity.
#[test]
fn tc_ir_1_3_3_u2_delta_consumed_and_cleared() {
    let mut log = VecLog::default();
    let delta = RootMotionDelta {
        translation: Vec3::new(0.0, 0.0, 1.2),
        rotation: Quat::IDENTITY,
    };
    let (out, cleared) = apply_root_motion_for_body(delta, BodyKind::CharacterController, &mut log);
    assert!(!out.discarded);
    assert_eq!(cleared, RootMotionDelta::identity());
}

/// TC-IR-1.3.4.U1 — recovery blend weight ramps monotonically 0→1.
#[test]
fn tc_ir_1_3_4_u1_recovery_blend_weight_monotonic() {
    let duration = 0.5;
    let mut prev = -1.0f32;
    for i in 0..=10 {
        let t = (i as f32 / 10.0) * duration;
        let w = recovery_blend_weight(t, duration);
        assert!(w >= prev);
        prev = w;
    }
    assert!((recovery_blend_weight(duration, duration) - 1.0).abs() < 1e-5);
}

/// TC-IR-1.3.4.U2 — per-bone velocity clamp (100 m/s linear, 50 rad/s angular).
#[test]
fn tc_ir_1_3_4_u2_per_bone_velocity_clamp() {
    let v = LinearAngular {
        linear: Vec3::new(200.0, 0.0, 0.0),
        angular: Vec3::new(0.0, 100.0, 0.0),
    };
    let c = clamp_linear_angular_velocity(v, 100.0, 50.0);
    assert!((c.linear.length() - 100.0).abs() < 1e-3);
    assert!((c.angular.length() - 50.0).abs() < 1e-3);
}

/// TC-IR-1.3.1.U3 — swing direction clamped to cone limit.
#[test]
fn tc_ir_1_3_1_u3_cone_twist_swing_clamp() {
    let axis = Vec3::Y;
    let dir = Vec3::new(1.0, 0.1, 0.0).normalize();
    let limit = 0.2_f32;
    let clamped = clamp_to_swing_cone(dir, axis, limit);
    let angle = clamped.dot(axis).clamp(-1.0, 1.0).acos();
    assert!(angle <= limit + 1e-3);
}

/// TC-IR-1.3.1.1 — Ragdoll init from palette poses.
#[test]
fn tc_ir_1_3_1_1_ragdoll_init_from_pose() {
    let mut store = RagdollDefStore::default();
    let h = store.insert(sample_ragdoll_def());
    let palette = vec![
        Mat4::from_translation(Vec3::X),
        Mat4::from_translation(Vec3::Y * 2.0),
    ];
    let out = init_ragdoll_isometries_from_palette(
        &store,
        RagdollRef { def: h },
        &palette,
        "test_skeleton",
    );
    assert_eq!(out.body_transforms.len(), 2);
    assert!((out.body_transforms[0].w_axis.truncate() - Vec3::X).length() < 1e-4);
}

/// TC-IR-1.3.1.2 — velocity inheritance adds root linear.
#[test]
fn tc_ir_1_3_1_2_velocity_inheritance() {
    let root = Vec3::new(1.0, 0.0, 0.0);
    let per_bone = vec![Vec3::ZERO, Vec3::Y];
    let v = inherit_root_linear_velocity(root, per_bone);
    assert!((v[0] - root).length() < 1e-5);
    assert!((v[1] - (root + Vec3::Y)).length() < 1e-5);
}

/// TC-IR-1.3.1.3 — animation paused when ragdoll active without recovery clip.
#[test]
fn tc_ir_1_3_1_3_animation_stops_on_ragdoll() {
    let ragdoll = RagdollTransition {
        blend_weight: 0.0,
        recovery_timer: None,
    };
    assert_eq!(
        animation_eval_policy(Some(ragdoll)),
        AnimationEvalPolicy::PausedForRagdoll
    );
}

/// TC-IR-1.3.2.1 — hit box follows bone row from palette.
#[test]
fn tc_ir_1_3_2_1_hit_box_follows_bone() {
    let palette = vec![
        Mat4::IDENTITY,
        Mat4::from_translation(Vec3::new(0.5, 1.0, 0.0)),
    ];
    let mut log = VecLog::default();
    let mut warned = false;
    let out = bone_collider_sync(
        Some(&palette),
        1,
        Mat4::IDENTITY,
        &mut log,
        "entity_a",
        &mut warned,
    );
    assert!(!out.skipped_missing_palette);
    let m = out.world.expect("world");
    assert!((m.w_axis.truncate() - Vec3::new(0.5, 1.0, 0.0)).length() < 1e-4);
}

/// TC-IR-1.3.4.1 — recovery blend weight approaches 1 over duration (blend 0→1).
#[test]
fn tc_ir_1_3_4_1_recovery_blends_toward_animation() {
    let w_mid = recovery_blend_weight(0.25, 0.5);
    assert!(w_mid > 0.4 && w_mid < 0.6);
}

/// TC-IR-1.3.3.3 — zero-frame root latency: buffered frame N, consumed Phase 5 on N+1.
#[test]
fn tc_ir_1_3_3_3_zero_frame_root_latency() {
    let mut pipe = RootMotionPipeline::default();
    let delta = RootMotionDelta {
        translation: Vec3::Z,
        rotation: Quat::IDENTITY,
    };
    pipe.buffer_from_animation(delta);
    pipe.advance_frame();
    let consumed = pipe.consume_at_phase5_start().expect("consumed");
    assert_eq!(consumed.translation, Vec3::Z);
}

/// TC-IR-1.3.2.3 — zero-frame bone sync: palette from frame N read at Phase 5 on N+1.
#[test]
fn tc_ir_1_3_2_3_zero_frame_bone_sync() {
    let mut log = VecLog::default();
    let mut warned = false;
    let palette_n = vec![Mat4::from_translation(Vec3::X * 3.0)];
    // Frame N: animation wrote palette (held externally).
    // Frame N+1: physics reads that palette at Phase 5 start.
    let last = Mat4::IDENTITY;
    let out = bone_collider_sync(
        Some(&palette_n),
        0,
        last,
        &mut log,
        "sync_entity",
        &mut warned,
    );
    assert!((out.world.expect("m").w_axis.truncate() - Vec3::X * 3.0).length() < 1e-4);
}

/// TC-IR-1.3.5.1 — weapon collider layers follow hit window.
#[test]
fn tc_ir_1_3_5_1_weapon_collider_layers() {
    let w = WeaponColliderState {
        active_layers: 0b1010,
        inactive_layers: 0b0001,
        hit_window_active: true,
    };
    assert_eq!(w.current_layers(), 0b1010);
    let w2 = WeaponColliderState {
        active_layers: 0b1010,
        inactive_layers: 0b0001,
        hit_window_active: false,
    };
    assert_eq!(w2.current_layers(), 0b0001);
}

/// TC-IR-1.3.5.2 — weapon tracks hand bone matrix.
#[test]
fn tc_ir_1_3_5_2_weapon_tracks_hand_bone() {
    let hand = Mat4::from_translation(Vec3::new(0.0, 1.5, 0.2));
    let palette = vec![Mat4::IDENTITY, hand];
    let w = WeaponColliderState {
        active_layers: 1,
        inactive_layers: 0,
        hit_window_active: true,
    };
    let m = w.world_from_palette(Some(&palette), 1).expect("hand");
    assert!((m.w_axis.truncate() - hand.w_axis.truncate()).length() < 1e-4);
}

/// TC-IR-1.3.3.2 — root rotation feeds through compose for CC-facing tests (quat multiply).
#[test]
fn tc_ir_1_3_3_2_root_rotation_turns_character() {
    let turn = RootMotionDelta {
        translation: Vec3::ZERO,
        rotation: Quat::from_rotation_y(core::f32::consts::FRAC_PI_4),
    };
    let mut log = VecLog::default();
    let (out, _) = apply_root_motion_for_body(turn, BodyKind::CharacterController, &mut log);
    assert!(!out.discarded);
    assert!(out.desired_speed < 1e-3);
}

/// TC-IR-1.3.1.N1 — missing bone index logs warning and freezes to identity.
#[test]
fn tc_ir_1_3_1_n1_ragdoll_def_missing_bone() {
    let mut store = RagdollDefStore::default();
    let h = store.insert(sample_ragdoll_def());
    let palette = vec![Mat4::IDENTITY]; // bone 1 missing
    let out = init_ragdoll_isometries_from_palette(&store, RagdollRef { def: h }, &palette, "sk");
    assert_eq!(out.log.missing_bones.len(), 1);
    assert_eq!(out.body_transforms[1], Mat4::IDENTITY);
}

/// TC-IR-1.3.1.N2 — invalid handle: no bodies, stale flagged.
#[test]
fn tc_ir_1_3_1_n2_invalid_ragdoll_ref() {
    let store = RagdollDefStore::default();
    let stale = RagdollRef {
        def: Handle::new(0, 99),
    };
    let out = init_ragdoll_isometries_from_palette(&store, stale, &[], "sk");
    assert!(out.body_transforms.is_empty());
    assert!(out.log.stale_handle);
}

/// TC-IR-1.3.3.N1 — sleeping dynamic body triggers wake path in outcome.
#[test]
fn tc_ir_1_3_3_n1_root_motion_on_sleeping() {
    let mut log = VecLog::default();
    let delta = RootMotionDelta {
        translation: Vec3::ONE,
        rotation: Quat::IDENTITY,
    };
    let (out, _) =
        apply_root_motion_for_body(delta, BodyKind::Dynamic { sleeping: true }, &mut log);
    assert!(out.woke_body);
}

/// TC-IR-1.3.3.N2 — static body discards delta.
#[test]
fn tc_ir_1_3_3_n2_root_motion_on_static() {
    let mut log = VecLog::default();
    let delta = RootMotionDelta {
        translation: Vec3::ONE,
        rotation: Quat::IDENTITY,
    };
    let (out, _) = apply_root_motion_for_body(delta, BodyKind::Static, &mut log);
    assert!(out.discarded);
    assert!(log.0.iter().any(|s| s.contains("static")));
}

/// TC-IR-1.3.4.N1 — missing recovery clip snaps to bind pose.
#[test]
fn tc_ir_1_3_4_n1_missing_recovery_clip() {
    let bind = vec![Mat4::from_scale(Vec3::splat(0.9))];
    let mut log = VecLog::default();
    let (poses, transition) = ragdoll_recovery_fallback(&bind, &mut log);
    assert_eq!(poses.len(), 1);
    assert_eq!(transition.blend_weight, 1.0);
    assert!(log.0.iter().any(|s| s.contains("recovery clip missing")));
}

/// TC-IR-1.3.2.N1 — missing palette skips sync (warn once).
#[test]
fn tc_ir_1_3_2_n1_bone_palette_missing() {
    let mut log = VecLog::default();
    let mut warned = false;
    let out = bone_collider_sync(None, 0, Mat4::IDENTITY, &mut log, "no_palette", &mut warned);
    assert!(out.skipped_missing_palette);
    assert!(log.0.iter().any(|s| s.contains("BonePaletteGpu missing")));
}

/// TC-IR-1.3.4.N2 — constraint violation style velocities clamp per bone.
#[test]
fn tc_ir_1_3_4_n2_constraint_violation_velocity_clamp() {
    let v = LinearAngular {
        linear: Vec3::splat(500.0),
        angular: Vec3::splat(200.0),
    };
    let c = clamp_linear_angular_velocity(v, 100.0, 50.0);
    assert!(c.linear.length() <= 100.0 + 1e-3);
    assert!(c.angular.length() <= 50.0 + 1e-3);
}

/// TC-IR-1.3.4.N3 — orientation swing beyond limit is clamped.
#[test]
fn tc_ir_1_3_4_n3_swing_beyond_cone() {
    let rot = Quat::from_rotation_x(1.2);
    let local_forward = Vec3::X;
    let parent_twist = Vec3::Y;
    let clamped = clamp_orientation_swing(rot, local_forward, parent_twist, 0.15);
    let world = clamped * local_forward;
    let angle = world.dot(parent_twist).clamp(-1.0, 1.0).acos();
    assert!(angle <= 0.15 + 0.05);
}

/// Character controller mapping helper for root-motion integration sketches.
#[test]
fn tc_ir_1_3_3_1_root_motion_moves_character_controller() {
    let mut log = VecLog::default();
    let delta = RootMotionDelta {
        translation: Vec3::new(0.0, 0.0, 2.0),
        rotation: Quat::IDENTITY,
    };
    let (out, _) = apply_root_motion_for_body(delta, BodyKind::CharacterController, &mut log);
    let cc = CharacterController {
        desired_direction: out.desired_direction,
        speed: out.desired_speed,
    };
    assert!((cc.desired_direction - Vec3::Z).length() < 1e-4);
    assert!((cc.speed - 2.0).abs() < 1e-4);
}
