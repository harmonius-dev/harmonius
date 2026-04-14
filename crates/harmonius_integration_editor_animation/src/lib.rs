//! Editor ↔ animation integration contracts and failure-mode policy.
//!
//! This crate materializes the cross-subsystem types from
//! `docs/design/integration/editor-animation.md` for deterministic unit tests
//! (including TC-IR-5.3.F1–F5).
//!
//! **Not in this slice:** worker frame arenas, MPSC pose readback, `PreUpdate` drains, and
//! benchmark harnesses from the design stay deferred until editor viewport wiring lands; see the
//! plan progress file for evidence scope.

#![forbid(unsafe_code)]
#![warn(clippy::all)]
// Public API mirrors the design; module-level `//!` docs cover each file.
#![allow(missing_docs)]

pub mod animation_data;
pub mod bone_overlay;
pub mod clip;
pub mod commands;
pub mod handle;
pub mod ids;
pub mod pose;
pub mod preview;
pub mod skinning;
pub mod state_graph;
pub mod world;

pub use animation_data::{
    AnimEventMarker, BlendSample2D, Keyframe, TangentPair, TrackKey,
};
pub use bone_overlay::{
    bone_overlay_outcome, bone_overlay_visible, BoneOverlayOutcome, WARN_BONE_INDEX_OUT_OF_RANGE,
};
pub use clip::{AnimationClip, AnimationClipTable};
pub use commands::{
    BlendSampleEditCommand, CommandError, EditorCommand, EventMarkerEditCommand,
    KeyframeEditCommand, TangentEditCommand,
};
pub use handle::Handle;
pub use ids::{BoneTrackIndex, Entity, ParameterId, StateId, StringId};
pub use pose::BonePoseSnapshot;
pub use preview::{
    apply_preview_action, AnimPreviewCommand, PreviewAction, PreviewApplyOutcome, PreviewPoseKind,
    WARN_BLEND_SPACE_EMPTY, WARN_INVALID_CLIP_HANDLE,
};
pub use skinning::{
    SkinningDispatchStatus, SoftwareSkinningFake, WARN_SKINNING_DISPATCH_FAILED,
};
pub use state_graph::{
    StateGraphDef, Transition, TransitionWalkOutcome, WARN_STATE_GRAPH_CYCLE,
    walk_always_true_transitions,
};
pub use world::World;

#[cfg(test)]
mod tests {
    use glam::{Mat4, Vec3};

    use super::*;

    #[test]
    fn tc_ir_5_3_2_1_hermite_tangent_data_readable() {
        let entity = Entity(1);
        let track = BoneTrackIndex(0);
        let tangents = TangentPair {
            in_tangent: 0.25,
            out_tangent: 0.75,
        };
        let mut world = World::default();
        world.set_tangent(entity, track, 0, tangents);
        assert_eq!(world.tangent(entity, track, 0), Some(tangents));
    }

    #[test]
    fn tc_ir_5_3_2_2_keyframe_edit_undo_round_trip() {
        let entity = Entity(1);
        let track = BoneTrackIndex(0);
        let key = TrackKey { entity, track };
        let old = Some(Keyframe { time: 0.0 });
        let new = Some(Keyframe { time: 0.5 });
        let cmd = KeyframeEditCommand {
            entity,
            track,
            old_keyframe: old,
            new_keyframe: new,
        };
        let mut world = World::default();
        world.set_keyframe(key, old);
        cmd.execute(&mut world).expect("execute");
        assert_eq!(world.keyframe(key), new);
        cmd.undo(&mut world).expect("undo");
        assert_eq!(world.keyframe(key), old);
    }

    #[test]
    fn tc_ir_5_3_2_3_bezier_tangent_handles_stored() {
        let entity = Entity(2);
        let track = BoneTrackIndex(1);
        let bezier = TangentPair {
            in_tangent: 1.0,
            out_tangent: 3.0,
        };
        let mut world = World::default();
        world.set_tangent(entity, track, 2, bezier);
        assert_eq!(world.tangent(entity, track, 2), Some(bezier));
    }

    #[test]
    fn tc_ir_5_3_7_1_add_event_marker_at_normalized_time() {
        let entity = Entity(3);
        let marker = AnimEventMarker {
            normalized_time: 0.75,
            event_kind: 42,
        };
        let cmd = EventMarkerEditCommand {
            entity,
            old_marker: None,
            new_marker: Some(marker),
        };
        let mut world = World::default();
        cmd.execute(&mut world).expect("execute");
        assert_eq!(world.event_marker(entity), Some(marker));
    }

    #[test]
    fn tc_ir_5_3_f1_invalid_clip_handle_t_pose() {
        let mut table = AnimationClipTable::default();
        let live = table.allocate();
        table.free(live);
        let bogus = live;
        let action = PreviewAction::Play {
            clip: bogus,
            speed: 1.0,
        };
        let out = apply_preview_action(&action, &table, 1);
        assert_eq!(out.pose, PreviewPoseKind::TPose);
        assert!(out.warnings.contains(&WARN_INVALID_CLIP_HANDLE));
    }

    #[test]
    fn tc_ir_5_3_f2_blend_space_empty_bind_pose() {
        let table = AnimationClipTable::default();
        let action = PreviewAction::SetBlendParam {
            param: ParameterId(0),
            value: 0.5,
        };
        let out = apply_preview_action(&action, &table, 0);
        assert_eq!(out.pose, PreviewPoseKind::BindPose);
        assert!(out.warnings.contains(&WARN_BLEND_SPACE_EMPTY));
    }

    #[test]
    fn tc_ir_5_3_f3_bone_index_out_of_range_skipped() {
        let bone_count = 10u32;
        let idx = bone_count;
        let out = bone_overlay_outcome(idx, bone_count);
        assert!(!out.visible);
        assert!(out.warnings.contains(&WARN_BONE_INDEX_OUT_OF_RANGE));
        assert!(bone_overlay_visible(0, bone_count));
    }

    #[test]
    fn tc_ir_5_3_f4_state_graph_cycle_warns() {
        let a = StateId(0);
        let b = StateId(1);
        let graph = StateGraphDef {
            transitions: vec![
                Transition {
                    from: a,
                    to: b,
                    always_true: true,
                },
                Transition {
                    from: b,
                    to: a,
                    always_true: true,
                },
            ],
        };
        let out = walk_always_true_transitions(&graph, a, 16);
        assert!(out.warnings.contains(&WARN_STATE_GRAPH_CYCLE));
        assert!(out.visited.len() < 16);
    }

    #[test]
    fn walk_always_true_transitions_max_steps_includes_terminal_state() {
        let a = StateId(0);
        let b = StateId(1);
        let c = StateId(2);
        let graph = StateGraphDef {
            transitions: vec![
                Transition {
                    from: a,
                    to: b,
                    always_true: true,
                },
                Transition {
                    from: b,
                    to: c,
                    always_true: true,
                },
            ],
        };
        let out = walk_always_true_transitions(&graph, a, 2);
        assert_eq!(out.final_state, Some(c));
        assert_eq!(out.visited, vec![a, b, c]);
        assert!(out.warnings.is_empty());
    }

    #[test]
    fn tc_ir_5_3_f5_skinning_dispatch_failure_bind_pose() {
        let bind = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let animated = Mat4::from_translation(Vec3::new(9.0, 0.0, 0.0));
        let mut fake = SoftwareSkinningFake::new();
        fake.set_fail_next_dispatch(true);
        let (matrix, warnings) = fake.skin_world_matrix(bind, animated);
        assert_eq!(matrix, bind);
        assert!(warnings.contains(&WARN_SKINNING_DISPATCH_FAILED));
    }

    #[test]
    fn tc_ir_5_3_2_4_bezier_tangent_undo_round_trip() {
        let entity = Entity(1);
        let track = BoneTrackIndex(0);
        let old = TangentPair {
            in_tangent: 0.0,
            out_tangent: 1.0,
        };
        let new = TangentPair {
            in_tangent: 0.5,
            out_tangent: 2.0,
        };
        let cmd = TangentEditCommand {
            entity,
            track,
            keyframe_index: 0,
            old_tangent: old,
            new_tangent: new,
        };
        let mut world = World::default();
        cmd.execute(&mut world).expect("execute");
        assert_eq!(world.tangent(entity, track, 0), Some(new));
        cmd.undo(&mut world).expect("undo");
        assert_eq!(world.tangent(entity, track, 0), Some(old));
    }
}
