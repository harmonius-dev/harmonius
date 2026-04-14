//! Lightweight simulation helpers used by integration tests (no ECS / GPU).

use glam::{Mat4, Vec3};

use crate::ragdoll::{Handle, RagdollDef, RagdollDefStore};
use crate::root_motion::{RootMotionApplyOutcome, RootMotionDelta};

/// Collects warning/error strings for tests and hosts.
pub trait LogSink {
    /// Records a warning-level event.
    fn warn(&mut self, message: String);
    /// Records an error-level event.
    fn error(&mut self, message: String);
}

/// Whether animation evaluation should run this frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimationEvalPolicy {
    /// Normal playback.
    Run,
    /// Ragdoll owns the skeleton; animation sampling is skipped.
    PausedForRagdoll,
}

/// Ragdoll blend + optional recovery timer (seconds).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RagdollTransition {
    /// 0 = physics, 1 = animation during recovery.
    pub blend_weight: f32,
    /// Counts down while blending back to animation.
    pub recovery_timer: Option<f32>,
}

/// Stub for `RagdollRef` component: generational handle into a store.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RagdollRef {
    /// Handle to a `RagdollDef` asset.
    pub def: Handle<RagdollDef>,
}

/// Character controller movement request from root motion apply.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharacterController {
    /// Desired facing/move direction (unit vector when non-zero).
    pub desired_direction: Vec3,
    /// Speed along `desired_direction`.
    pub speed: f32,
}

/// Simple rigid classification for root-motion application tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BodyKind {
    /// Character controller driven entity.
    CharacterController,
    /// Dynamic rigid body.
    Dynamic {
        /// Whether the body is sleeping.
        sleeping: bool,
    },
    /// Static immovable body.
    Static,
}

/// Log lines emitted while initializing ragdoll bodies from a palette.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RagdollInitLog {
    /// Missing bone warnings (bone index, skeleton label).
    pub missing_bones: Vec<(u16, String)>,
    /// Stale handle errors.
    pub stale_handle: bool,
}

/// Outcome of mapping ragdoll bodies to palette poses.
#[derive(Clone, Debug, PartialEq)]
pub struct RagdollInitOutcome {
    /// World transforms per ragdoll bone entry (same order as `def.bone_bodies`).
    pub body_transforms: Vec<Mat4>,
    pub log: RagdollInitLog,
}

/// Builds world `Mat4` poses for each `RagdollBone` using palette matrices.
pub fn init_ragdoll_isometries_from_palette(
    store: &RagdollDefStore,
    ragdoll_ref: RagdollRef,
    palette: &[Mat4],
    skeleton_name: &str,
) -> RagdollInitOutcome {
    let mut log = RagdollInitLog::default();
    let Some(def) = store.get(ragdoll_ref.def) else {
        log.stale_handle = true;
        return RagdollInitOutcome {
            body_transforms: Vec::new(),
            log,
        };
    };

    let mut body_transforms = Vec::with_capacity(def.bone_bodies.len());
    for bone in &def.bone_bodies {
        let idx = bone.bone_index.0 as usize;
        if let Some(m) = palette.get(idx) {
            body_transforms.push(*m);
        } else {
            log.missing_bones
                .push((bone.bone_index.0, skeleton_name.to_string()));
            body_transforms.push(Mat4::IDENTITY);
        }
    }

    RagdollInitOutcome {
        body_transforms,
        log,
    }
}

/// Adds root linear velocity to each bone linear velocity (momentum inheritance sketch).
#[must_use]
pub fn inherit_root_linear_velocity(root_linear: Vec3, per_bone_linear: Vec<Vec3>) -> Vec<Vec3> {
    per_bone_linear
        .into_iter()
        .map(|v| v + root_linear)
        .collect()
}

/// Animation should pause for pure ragdoll; recovery timer implies a get-up clip is driving eval.
pub fn animation_eval_policy(ragdoll: Option<RagdollTransition>) -> AnimationEvalPolicy {
    match ragdoll {
        None => AnimationEvalPolicy::Run,
        Some(r) if r.recovery_timer.is_some() => AnimationEvalPolicy::Run,
        Some(_) => AnimationEvalPolicy::PausedForRagdoll,
    }
}

/// Result of syncing a bone-driven collider.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoneColliderSyncOutcome {
    /// Updated world matrix for the collider when a palette row exists.
    pub world: Option<Mat4>,
    /// Whether sync was skipped due to missing palette.
    pub skipped_missing_palette: bool,
}

/// Reads `palette[row]` when present; otherwise logs once and preserves `last_world`.
pub fn bone_collider_sync(
    palette: Option<&[Mat4]>,
    bone_row: usize,
    last_world: Mat4,
    log: &mut dyn LogSink,
    entity_label: &str,
    warned_missing_palette: &mut bool,
) -> BoneColliderSyncOutcome {
    let Some(p) = palette else {
        if !*warned_missing_palette {
            log.warn(format!(
                "bone collider sync skipped for {entity_label}: BonePaletteGpu missing"
            ));
            *warned_missing_palette = true;
        }
        return BoneColliderSyncOutcome {
            world: Some(last_world),
            skipped_missing_palette: true,
        };
    };
    let world = p.get(bone_row).copied().unwrap_or(last_world);
    BoneColliderSyncOutcome {
        world: Some(world),
        skipped_missing_palette: false,
    }
}

/// Applies buffered root motion to a body kind, returning controller/force targets.
pub fn apply_root_motion_for_body(
    delta: RootMotionDelta,
    body: BodyKind,
    log: &mut dyn LogSink,
) -> (RootMotionApplyOutcome, RootMotionDelta) {
    match body {
        BodyKind::CharacterController => {
            let dir = delta.translation;
            let len = dir.length();
            let (direction, speed) = if len < 1e-6 {
                (Vec3::Z, 0.0)
            } else {
                (dir / len, len)
            };
            (
                RootMotionApplyOutcome::character_applied(direction, speed),
                RootMotionDelta::identity(),
            )
        }
        BodyKind::Dynamic { sleeping } => {
            let woke = sleeping;
            if sleeping {
                log.warn("wake_body invoked before applying root motion delta".to_string());
            }
            let linear = delta.translation + delta.rotation * Vec3::ZERO;
            (
                RootMotionApplyOutcome::dynamic_force(linear, woke),
                RootMotionDelta::identity(),
            )
        }
        BodyKind::Static => {
            log.warn("root motion delta discarded for static body".to_string());
            (
                RootMotionApplyOutcome::discarded(),
                RootMotionDelta::identity(),
            )
        }
    }
}

/// Linear recovery blend weight in `[0, 1]` over `duration` seconds.
#[must_use]
pub fn recovery_blend_weight(elapsed: f32, duration: f32) -> f32 {
    if duration <= 1e-6 {
        return 1.0;
    }
    (elapsed / duration).clamp(0.0, 1.0)
}

/// Missing recovery clip: snap to bind pose matrices and mark ragdoll finished.
pub fn ragdoll_recovery_fallback(
    bind_pose: &[Mat4],
    log: &mut dyn LogSink,
) -> (Vec<Mat4>, RagdollTransition) {
    log.warn("recovery clip missing; snapping to bind pose".to_string());
    (
        bind_pose.to_vec(),
        RagdollTransition {
            blend_weight: 1.0,
            recovery_timer: None,
        },
    )
}

/// Weapon collider follows a bone row when palette exists.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeaponColliderState {
    /// Layer bitmask when the hit window is active.
    pub active_layers: u32,
    /// Layer bitmask when inactive.
    pub inactive_layers: u32,
    /// Whether the current frame is inside the authored hit window.
    pub hit_window_active: bool,
}

impl WeaponColliderState {
    /// Current collision layers for the weapon body.
    #[must_use]
    pub fn current_layers(self) -> u32 {
        if self.hit_window_active {
            self.active_layers
        } else {
            self.inactive_layers
        }
    }

    /// World transform of the weapon when palette supplies the hand bone.
    #[must_use]
    pub fn world_from_palette(self, palette: Option<&[Mat4]>, hand_bone: usize) -> Option<Mat4> {
        palette?.get(hand_bone).copied()
    }
}
