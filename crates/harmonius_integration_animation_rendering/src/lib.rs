//! Animation ↔ rendering integration contracts (IR-1.4.1–IR-1.4.5).
//!
//! This crate captures the transient GPU-upload structs, instanced skinning grouping,
//! and render-thread scheduling contracts described in `docs/design/integration/animation-rendering.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod arena;
mod bridge;
mod grouping;
mod handle;
mod lod;
mod morph;
mod render_frame;
mod resource;
mod skinner;
mod types;

pub use arena::allocate_with_arena_overflow_policy;
pub use arena::ArenaOverflowOutcome;
pub use bridge::plan_skinned_draws_from_render_frame;
pub use bridge::AnimationRenderBridge;
pub use bridge::SkinnedDrawCommand;
pub use grouping::build_skinning_dispatches_sorted;
pub use grouping::debug_record_hashmap_touch;
pub use grouping::grouping_hashmap_touch_count;
pub use grouping::make_row;
pub use grouping::reset_grouping_hashmap_touch_count;
pub use handle::GpuBuffer;
pub use handle::Handle;
pub use lod::invalid_lod_warning_count;
pub use lod::lod_tier_from_distance_m;
pub use lod::reset_invalid_lod_warning_count;
pub use lod::sanitize_lod_tier;
pub use lod::HalfRateStaleTracker;
pub use morph::clamp_morph_targets;
pub use morph::morph_pass_required;
pub use render_frame::RenderFrame;
pub use resource::GpuBufferTable;
pub use resource::ResourceError;
pub use skinner::dispatch_skinning_passes_for_proxy;
pub use skinner::half_rate_force_full_after_stale;
pub use skinner::next_batch_size_after_gpu_timeout;
pub use skinner::zero_blend_weights;
pub use skinner::ComputePass;
pub use skinner::GpuSkinner;
pub use types::AnimationLodTier;
pub use types::BlendDescriptor;
pub use types::BonePaletteGpu;
pub use types::ClipSetId;
pub use types::EntityId;
pub use types::GpuArenaBuffer;
pub use types::InstancedSkinningRow;
pub use types::MorphTargets;
pub use types::SkeletonId;
pub use types::SkinnedMeshProxy;
pub use types::SkinningDispatch;
pub use types::SkinningMode;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_blend_active() -> BlendDescriptor {
        BlendDescriptor {
            clip_indices: [1, 0, 0, 0, 0, 0, 0, 0],
            clip_weights: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            active_count: 1,
        }
    }

    fn sample_morph_four_active() -> MorphTargets {
        MorphTargets {
            target_indices: [1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            weights: [
                0.25, 0.25, 0.25, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            active_count: 4,
        }
    }

    fn sample_mesh_dqs_with_morph(
        bone: Handle<GpuBuffer>,
        morph: Option<Handle<GpuBuffer>>,
        tier: AnimationLodTier,
    ) -> SkinnedMeshProxy {
        SkinnedMeshProxy {
            bone_palette: bone,
            bone_count: 64,
            skinning_mode: SkinningMode::Dqs,
            morph_buffer: morph,
            lod_tier: tier,
        }
    }

    #[test]
    fn tc_ir_1_4_1_1_skinning_dispatch_one_entity() {
        let bone = Handle::from_raw(1, 1);
        let morph_buf = Handle::from_raw(2, 1);
        let mesh = sample_mesh_dqs_with_morph(bone, Some(morph_buf), AnimationLodTier::Full);
        let blend = sample_blend_active();
        let morph = sample_morph_four_active();
        let mut skinner = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut skinner, &blend, &morph, &mesh, true);
        assert!(
            skinner
                .log
                .iter()
                .any(|p| matches!(p, ComputePass::Skinning { .. })),
            "expected a skinning dispatch, got {:?}",
            skinner.log
        );
    }

    #[test]
    fn tc_ir_1_4_1_2_lbs_vs_dqs_select() {
        let bone = Handle::from_raw(1, 1);
        let mesh_lbs = sample_mesh_dqs_with_morph(bone, None, AnimationLodTier::Full);
        let mesh_dqs = SkinnedMeshProxy {
            skinning_mode: SkinningMode::Dqs,
            ..mesh_lbs
        };
        let mesh_lbs = SkinnedMeshProxy {
            skinning_mode: SkinningMode::Lbs,
            ..mesh_lbs
        };
        let blend = sample_blend_active();
        let morph = MorphTargets {
            target_indices: [0; 16],
            weights: [0.0; 16],
            active_count: 0,
        };
        let mut lbs = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut lbs, &blend, &morph, &mesh_lbs, true);
        let mut dqs = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut dqs, &blend, &morph, &mesh_dqs, true);
        assert_eq!(
            lbs.log,
            vec![ComputePass::Skinning {
                mode: SkinningMode::Lbs
            }]
        );
        assert_eq!(
            dqs.log,
            vec![ComputePass::Skinning {
                mode: SkinningMode::Dqs
            }]
        );
    }

    #[test]
    fn tc_ir_1_4_2_1_morph_before_skin() {
        let bone = Handle::from_raw(1, 1);
        let morph_buf = Handle::from_raw(2, 1);
        let mesh = sample_mesh_dqs_with_morph(bone, Some(morph_buf), AnimationLodTier::Full);
        let blend = sample_blend_active();
        let morph = sample_morph_four_active();
        let mut skinner = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut skinner, &blend, &morph, &mesh, true);
        assert_eq!(skinner.log[0], ComputePass::MorphAccum);
        assert!(matches!(skinner.log[1], ComputePass::Skinning { .. }));
    }

    #[test]
    fn tc_ir_1_4_2_2_zero_morph_skip() {
        let bone = Handle::from_raw(1, 1);
        let morph_buf = Handle::from_raw(2, 1);
        let mesh = sample_mesh_dqs_with_morph(bone, Some(morph_buf), AnimationLodTier::Full);
        let blend = sample_blend_active();
        let morph = MorphTargets {
            target_indices: [0; 16],
            weights: [0.0; 16],
            active_count: 4,
        };
        let mut skinner = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut skinner, &blend, &morph, &mesh, true);
        assert!(!skinner.log.contains(&ComputePass::MorphAccum));
    }

    #[test]
    fn tc_ir_1_4_3_1_lod_full_near() {
        assert_eq!(lod_tier_from_distance_m(5.0), AnimationLodTier::Full);
    }

    #[test]
    fn tc_ir_1_4_3_2_lod_vat_far() {
        assert_eq!(lod_tier_from_distance_m(120.0), AnimationLodTier::Vat);
        let bone = Handle::from_raw(1, 1);
        let mesh = sample_mesh_dqs_with_morph(bone, None, AnimationLodTier::Vat);
        let blend = sample_blend_active();
        let morph = MorphTargets {
            target_indices: [0; 16],
            weights: [0.0; 16],
            active_count: 0,
        };
        let mut skinner = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut skinner, &blend, &morph, &mesh, true);
        assert!(!skinner
            .log
            .iter()
            .any(|p| matches!(p, ComputePass::Skinning { .. })));
    }

    #[test]
    fn tc_ir_1_4_3_3_lod_half_rate_skip_every_other_frame() {
        let bone = Handle::from_raw(1, 1);
        let mesh = sample_mesh_dqs_with_morph(bone, None, AnimationLodTier::HalfRate);
        let blend = sample_blend_active();
        let morph = MorphTargets {
            target_indices: [0; 16],
            weights: [0.0; 16],
            active_count: 0,
        };
        let mut on = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut on, &blend, &morph, &mesh, true);
        let mut off = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut off, &blend, &morph, &mesh, false);
        assert!(matches!(on.log[0], ComputePass::Skinning { .. }));
        assert_eq!(off.log, vec![ComputePass::HalfRateSkip]);
    }

    #[test]
    fn tc_ir_1_4_4_1_snapshot_has_bones() {
        let bone = Handle::from_raw(7, 2);
        let proxy = SkinnedMeshProxy {
            bone_palette: bone,
            bone_count: 72,
            skinning_mode: SkinningMode::Lbs,
            morph_buffer: None,
            lod_tier: AnimationLodTier::Full,
        };
        let arena = Handle::from_raw(3, 1);
        let row = make_row(1, 1, 0, arena, 72, SkinningMode::Lbs);
        let bridge = AnimationRenderBridge::new(vec![proxy], vec![row]);
        let frame = bridge.snapshot_render_frame(99);
        assert_eq!(frame.frame_index, 99);
        assert_eq!(frame.skinned_meshes.len(), 1);
        assert_eq!(frame.skinned_meshes[0].bone_palette, bone);
        assert_eq!(frame.skinning_dispatches.len(), 1);
    }

    #[test]
    fn tc_ir_1_4_4_2_render_reads_snap_only() {
        let bone = Handle::from_raw(1, 1);
        let proxy = SkinnedMeshProxy {
            bone_palette: bone,
            bone_count: 10,
            skinning_mode: SkinningMode::Lbs,
            morph_buffer: None,
            lod_tier: AnimationLodTier::Full,
        };
        let frame = RenderFrame::new(1, vec![proxy], vec![]);
        let draws = plan_skinned_draws_from_render_frame(&frame);
        assert_eq!(draws, vec![SkinnedDrawCommand { bone_palette: bone }]);
    }

    #[test]
    fn tc_ir_1_4_5_1_thousand_instances_single_batch() {
        reset_grouping_hashmap_touch_count();
        let arena = Handle::from_raw(1, 1);
        let mut rows = Vec::new();
        for i in 0..1000 {
            rows.push(make_row(5, 9, i, arena, 64, SkinningMode::Lbs));
        }
        let disp = build_skinning_dispatches_sorted(&mut rows);
        assert_eq!(disp.len(), 1);
        assert_eq!(disp[0].instance_count, 1000);
        assert_eq!(grouping_hashmap_touch_count(), 0);
    }

    #[test]
    fn tc_ir_1_4_5_2_three_skeleton_batches() {
        let arena = Handle::from_raw(1, 1);
        let mut rows = vec![
            make_row(0, 1, 0, arena, 64, SkinningMode::Lbs),
            make_row(1, 1, 1, arena, 64, SkinningMode::Lbs),
            make_row(2, 1, 2, arena, 64, SkinningMode::Lbs),
        ];
        let disp = build_skinning_dispatches_sorted(&mut rows);
        assert_eq!(disp.len(), 3);
    }

    #[test]
    fn tc_ir_1_4_1_n1_arena_full_demotion() {
        let arena = GpuArenaBuffer {
            buffer: Handle::from_raw(0, 1),
            capacity: 10,
            used: 10,
        };
        let outcome = allocate_with_arena_overflow_policy(&arena, 5, 5);
        assert_eq!(outcome.demoted_to_reduced_bones, 5);
        assert_eq!(outcome.demoted_to_vat, 0);
    }

    #[test]
    fn tc_ir_1_4_1_n2_arena_full_to_vat() {
        let arena = GpuArenaBuffer {
            buffer: Handle::from_raw(0, 1),
            capacity: 10,
            used: 10,
        };
        let outcome = allocate_with_arena_overflow_policy(&arena, 8, 3);
        assert_eq!(outcome.demoted_to_reduced_bones, 3);
        assert_eq!(outcome.demoted_to_vat, 5);
    }

    #[test]
    fn tc_ir_1_4_1_n3_gpu_timeout_halve_batch() {
        assert_eq!(next_batch_size_after_gpu_timeout(64), 32);
        assert_eq!(next_batch_size_after_gpu_timeout(1), 1);
    }

    #[test]
    fn tc_ir_1_4_1_n4_zero_blend_weight_bind_pose() {
        let bone = Handle::from_raw(1, 1);
        let mesh = sample_mesh_dqs_with_morph(bone, None, AnimationLodTier::Full);
        let blend = BlendDescriptor {
            clip_indices: [0; 8],
            clip_weights: [0.0; 8],
            active_count: 2,
        };
        let morph = MorphTargets {
            target_indices: [0; 16],
            weights: [0.0; 16],
            active_count: 0,
        };
        let mut skinner = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut skinner, &blend, &morph, &mesh, true);
        assert_eq!(skinner.log, vec![ComputePass::BindPose]);
    }

    #[test]
    fn tc_ir_1_4_2_n1_morph_overflow_clamp() {
        let mut targets = MorphTargets {
            target_indices: core::array::from_fn(|i| i as u16),
            weights: core::array::from_fn(|i| if i < 32 { 1.0 } else { 0.0 }),
            active_count: 32,
        };
        targets = clamp_morph_targets(targets);
        assert_eq!(targets.active_count, 16);
    }

    #[test]
    fn tc_ir_1_4_2_n2_no_morph_buffer_skin_only() {
        let bone = Handle::from_raw(1, 1);
        let mesh = sample_mesh_dqs_with_morph(bone, None, AnimationLodTier::Full);
        let blend = sample_blend_active();
        let morph = sample_morph_four_active();
        let mut skinner = GpuSkinner::new();
        dispatch_skinning_passes_for_proxy(&mut skinner, &blend, &morph, &mesh, true);
        assert!(!skinner.log.contains(&ComputePass::MorphAccum));
    }

    #[test]
    fn tc_ir_1_4_3_n1_invalid_lod_fallback_full() {
        reset_invalid_lod_warning_count();
        assert_eq!(sanitize_lod_tier(99), AnimationLodTier::Full);
        assert_eq!(invalid_lod_warning_count(), 1);
    }

    #[test]
    fn tc_ir_1_4_3_n2_half_rate_stale_force_full() {
        let mut tracker = HalfRateStaleTracker::new();
        assert!(!half_rate_force_full_after_stale(&mut tracker, false));
        assert!(!half_rate_force_full_after_stale(&mut tracker, false));
        assert!(half_rate_force_full_after_stale(&mut tracker, false));
        assert!(!half_rate_force_full_after_stale(&mut tracker, true));
    }

    #[test]
    fn tc_ir_1_4_4_n1_stale_handle_skips_validation() {
        let mut table = GpuBufferTable::default();
        let h = table.allocate();
        assert!(table.validate(h).is_ok());
        table.free(h).unwrap();
        assert_eq!(table.validate(h), Err(ResourceError::StaleHandle));
    }

    #[test]
    fn tc_ir_1_4_5_n1_hashmap_guard() {
        reset_grouping_hashmap_touch_count();
        debug_record_hashmap_touch();
        assert_eq!(grouping_hashmap_touch_count(), 1);
        reset_grouping_hashmap_touch_count();
        let arena = Handle::from_raw(1, 1);
        let mut rows = vec![make_row(0, 0, 0, arena, 8, SkinningMode::Lbs)];
        let _ = build_skinning_dispatches_sorted(&mut rows);
        assert_eq!(grouping_hashmap_touch_count(), 0);
    }

    #[test]
    fn tc_ir_1_4_5_b2_grouping_sort_10k_smoke() {
        let arena = Handle::from_raw(1, 1);
        let mut rows = Vec::new();
        for i in 0..10_000 {
            rows.push(make_row(i % 3, i % 2, i, arena, 32, SkinningMode::Lbs));
        }
        let disp = build_skinning_dispatches_sorted(&mut rows);
        assert!(!disp.is_empty());
        let total: u32 = disp.iter().map(|d| d.instance_count).sum();
        assert_eq!(total, 10_000);
    }
}
