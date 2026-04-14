//! Editor ↔ animation integration contracts and failure-mode policy.
//!
//! This crate materializes the cross-subsystem types from
//! `docs/design/integration/editor-animation.md` for deterministic unit tests
//! (including TC-IR-5.3.F1–F5).

#![forbid(unsafe_code)]
#![warn(clippy::all)]
// Design-derived surface area is documented in `docs/design/integration/editor-animation.md`.
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
pub use bone_overlay::{bone_overlay_visible, WARN_BONE_INDEX_OUT_OF_RANGE};
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
        assert!(!bone_overlay_visible(idx, bone_count));
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
