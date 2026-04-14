//! Deterministic helpers for the Rendering ↔ VFX integration seam.
//!
//! Covers pure scheduling, budgeting, and data-path contracts from
//! `docs/design/integration/rendering-vfx.md` without GPU I/O.
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(missing_docs)]

pub mod budget;
pub mod compiler;
pub mod decals;
pub mod froxel;
pub mod lights;
pub mod lod;
pub mod particles;
pub mod render_graph;
pub mod screen_effects;
pub mod snapshot;
pub mod types;

pub use types::{
    Aabb, BlendMode, DecalEntry, DecalPassDesc, DispatchQueue, EffectBudget, EmitterLodComponent,
    FroxelInjection, GpuBufferView, LodTier, MobileVolumetricCaps, ParticleLight,
    ParticleRenderPassDesc, RenderMode, ScreenEffectKind, ScreenEffectPassDesc, SortKey, VfxEdge,
    VfxGraph, VfxNode, VfxNodeKind,
};

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::budget::tick_spawn_rate_scale;
    use super::compiler::{CompiledPassDesc, VfxGraphCompiler};
    use super::decals::{
        apply_decal_albedo_channel, decal_alpha_for_slope, lru_eviction_fifo_probe,
        sort_decals_by_priority, stack_decal_albedo_on_wall,
    };
    use super::froxel::{
        apply_fog_density_injection, apply_weather_scattering_bump, clamp_froxel_density,
        populate_mobile_screen_tile_fog,
    };
    use super::lights::{
        cluster_slots_used, particle_light_overflow_dropped, select_brightest_particle_lights,
    };
    use super::lod::evaluate_emitter_lod;
    use super::particles::{
        radix_sort_skipped, ribbon_indirect_vertex_count, sort_particle_indices_back_to_front,
        sprite_indirect_instance_count, ParticleInstance,
    };
    use super::render_graph::{
        frame_passes_with_vfx_tail, particle_sim_then_draw_pipeline, recover_particle_sim_dispatch,
        RecordedPass,
    };
    use super::screen_effects::{heat_haze_distortion_strength, shockwave_ring_radius};
    use super::snapshot::drain_passes_to_snapshot;
    use super::types::{
        Aabb, DecalEntry, DispatchQueue, EffectBudget, EmitterLodComponent, FroxelInjection,
        LodTier, MobileVolumetricCaps, ParticleLight, RenderMode, VfxGraph, VfxNode, VfxNodeKind,
    };

    #[test]
    fn tc_ir_3_7_1_1_particle_sim_precedes_draw() {
        let p = particle_sim_then_draw_pipeline(RenderMode::Sprite);
        let sim = p.iter().position(|x| *x == RecordedPass::ParticleSimCompute).unwrap();
        let draw = p
            .iter()
            .position(|x| matches!(x, RecordedPass::ParticleDrawIndirect(_)))
            .unwrap();
        assert!(sim < draw);
    }

    #[test]
    fn tc_ir_3_7_1_2_barrier_between_sim_and_draw() {
        let p = particle_sim_then_draw_pipeline(RenderMode::Ribbon);
        let sim = p.iter().position(|x| *x == RecordedPass::ParticleSimCompute).unwrap();
        let bar = p.iter().position(|x| *x == RecordedPass::BarrierSimToDraw).unwrap();
        let draw = p
            .iter()
            .position(|x| matches!(x, RecordedPass::ParticleDrawIndirect(_)))
            .unwrap();
        assert!(sim < bar && bar < draw);
    }

    #[test]
    fn tc_ir_3_7_1_f1_fence_timeout_falls_back_to_graphics() {
        assert_eq!(
            recover_particle_sim_dispatch(true),
            DispatchQueue::Graphics
        );
        assert_eq!(
            recover_particle_sim_dispatch(false),
            DispatchQueue::AsyncCompute
        );
    }

    #[test]
    fn tc_ir_3_7_2_1_sprite_indirect_count() {
        assert_eq!(sprite_indirect_instance_count(500), 500);
    }

    #[test]
    fn tc_ir_3_7_2_2_ribbon_indirect_nonzero() {
        assert!(ribbon_indirect_vertex_count(4) >= 3);
    }

    #[test]
    fn tc_ir_3_7_2_3_alpha_sort_back_to_front() {
        let cam = Vec3::ZERO;
        let parts = vec![
            ParticleInstance {
                position: Vec3::new(0.0, 0.0, 1.0),
            },
            ParticleInstance {
                position: Vec3::new(0.0, 0.0, 5.0),
            },
        ];
        let order = sort_particle_indices_back_to_front(cam, &parts);
        assert_eq!(order[0], 1);
        assert_eq!(order[1], 0);
    }

    #[test]
    fn tc_ir_3_7_2_f1_radix_sort_skipped_when_scratch_small() {
        assert!(radix_sort_skipped(1000, 512));
        assert!(!radix_sort_skipped(100, 512));
    }

    #[test]
    fn tc_ir_3_7_3_1_fog_density_inside_aabb() {
        let mut grid = vec![0.0_f32; 2];
        let centers = vec![
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(10.0, 10.0, 10.0),
        ];
        let inj = FroxelInjection {
            density: 0.25,
            scattering: Vec3::ZERO,
            absorption: Vec3::ZERO,
            world_aabb: Aabb {
                min: Vec3::ZERO,
                max: Vec3::ONE,
            },
        };
        apply_fog_density_injection(&mut grid, &centers, &inj);
        assert!(grid[0] > 0.0);
        assert_eq!(grid[1], 0.0);
    }

    #[test]
    fn tc_ir_3_7_3_2_weather_scattering_bump() {
        let base = Vec3::splat(0.1);
        let bumped = apply_weather_scattering_bump(base, 1.0);
        assert!(bumped.length() > base.length());
    }

    #[test]
    fn tc_ir_3_7_3_f1_density_clamp() {
        assert_eq!(clamp_froxel_density(10.0, 2.0), 2.0);
    }

    #[test]
    fn tc_ir_3_7_3_s1_mobile_screen_tile_fallback() {
        let caps = MobileVolumetricCaps {
            gpu_froxel_supported: false,
            async_compute_supported: false,
        };
        let mut tiles = [0.0_f32; 144];
        assert!(populate_mobile_screen_tile_fog(caps, true, &mut tiles));
        assert!(tiles[0] > 0.0);
    }

    #[test]
    fn tc_ir_3_7_4_1_decal_changes_albedo() {
        let base = Vec3::splat(0.2);
        let mut decals = vec![DecalEntry {
            priority: 1,
            albedo: Vec3::new(0.9, 0.1, 0.1),
            surface_slope_deg: 0.0,
        }];
        sort_decals_by_priority(&mut decals);
        let out = stack_decal_albedo_on_wall(base, &decals);
        assert_ne!(out, base);
    }

    #[test]
    fn tc_ir_3_7_4_2_decal_priority_layering() {
        let mut decals = vec![
            DecalEntry {
                priority: 1,
                albedo: Vec3::splat(0.2),
                surface_slope_deg: 0.0,
            },
            DecalEntry {
                priority: 2,
                albedo: Vec3::splat(0.8),
                surface_slope_deg: 0.0,
            },
        ];
        sort_decals_by_priority(&mut decals);
        let top = stack_decal_albedo_on_wall(Vec3::splat(0.1), &decals);
        let bottom_first = apply_decal_albedo_channel(
            0.1,
            decals[0].albedo.x,
            decal_alpha_for_slope(decals[0].surface_slope_deg, 60.0, 80.0),
        );
        let both = apply_decal_albedo_channel(
            bottom_first,
            decals[1].albedo.x,
            decal_alpha_for_slope(decals[1].surface_slope_deg, 60.0, 80.0),
        );
        assert!((both - top.x).abs() < 1e-5);
    }

    #[test]
    fn tc_ir_3_7_4_3_slope_fade() {
        assert!(decal_alpha_for_slope(80.0, 60.0, 80.0) <= 1e-6);
    }

    #[test]
    fn tc_ir_3_7_4_f1_atlas_lru_eviction() {
        let evicted = lru_eviction_fifo_probe(2, 4);
        assert_eq!(evicted.len(), 2);
    }

    #[test]
    fn tc_ir_3_7_5_1_cluster_slots() {
        let lights: Vec<_> = (0..10)
            .map(|i| ParticleLight {
                intensity: 1.0,
                ordinal: i,
            })
            .collect();
        assert_eq!(cluster_slots_used(&lights), 10);
    }

    #[test]
    fn tc_ir_3_7_5_2_brightest_heap_budget() {
        let lights: Vec<_> = (0..1000)
            .map(|i| ParticleLight {
                intensity: i as f32,
                ordinal: i,
            })
            .collect();
        let picked = select_brightest_particle_lights(&lights, 100);
        assert_eq!(picked.len(), 100);
        let min_picked = picked.iter().map(|l| l.intensity).fold(f32::INFINITY, f32::min);
        let max_rest = (0..1000)
            .filter(|i| !picked.iter().any(|p| p.ordinal == *i))
            .map(|i| i as f32)
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(min_picked >= max_rest);
    }

    #[test]
    fn tc_ir_3_7_5_f1_overflow_counter() {
        let lights = vec![ParticleLight {
            intensity: 1.0,
            ordinal: 0,
        }; 500];
        assert_eq!(particle_light_overflow_dropped(&lights, 100), 400);
    }

    #[test]
    fn tc_ir_3_7_6_1_heat_haze_strength() {
        assert!(heat_haze_distortion_strength(true) > 0.0);
        assert_eq!(heat_haze_distortion_strength(false), 0.0);
    }

    #[test]
    fn tc_ir_3_7_6_2_shockwave_radius_half_at_mid_time() {
        assert!((shockwave_ring_radius(0.5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn tc_ir_3_7_7_1_reduced_at_distance() {
        let mut comp = EmitterLodComponent {
            full_distance: 50.0,
            reduced_distance: 300.0,
            impostor_distance: 350.0,
            cull_distance: 400.0,
            hysteresis_pct: 0.2,
            current_tier: LodTier::Full,
        };
        assert_eq!(evaluate_emitter_lod(200.0, &mut comp), LodTier::Reduced);
    }

    #[test]
    fn tc_ir_3_7_7_2_culled_beyond_cull_distance() {
        let mut comp = EmitterLodComponent {
            full_distance: 50.0,
            reduced_distance: 200.0,
            impostor_distance: 350.0,
            cull_distance: 400.0,
            hysteresis_pct: 0.2,
            current_tier: LodTier::Impostor,
        };
        assert_eq!(evaluate_emitter_lod(500.0, &mut comp), LodTier::Culled);
    }

    #[test]
    fn tc_ir_3_7_7_3_hysteresis_limits_flapping() {
        let mut comp = EmitterLodComponent {
            full_distance: 100.0,
            reduced_distance: 250.0,
            impostor_distance: 300.0,
            cull_distance: 500.0,
            hysteresis_pct: 0.2,
            current_tier: LodTier::Full,
        };
        let a = evaluate_emitter_lod(105.0, &mut comp);
        let b = evaluate_emitter_lod(95.0, &mut comp);
        let c = evaluate_emitter_lod(105.0, &mut comp);
        assert_eq!(a, LodTier::Full);
        assert_eq!(b, LodTier::Full);
        assert_eq!(c, LodTier::Full);
    }

    #[test]
    fn tc_ir_3_7_7_f1_spawn_scale_drops_when_over_budget() {
        let mut budget = EffectBudget {
            max_live_particles: 1000,
            max_particle_lights: 8,
            max_decals: 64,
            max_screen_effects: 4,
            spawn_rate_scale: 1.0,
        };
        budget = tick_spawn_rate_scale(budget, 2500);
        assert!(budget.spawn_rate_scale < 1.0);
    }

    #[test]
    fn tc_ir_3_7_1_s2_compiler_emits_three_pass_rows() {
        let graph = VfxGraph {
            nodes: vec![
                VfxNode {
                    kind: VfxNodeKind::ParticleSim,
                },
                VfxNode {
                    kind: VfxNodeKind::Decal,
                },
                VfxNode {
                    kind: VfxNodeKind::ScreenEffect,
                },
            ],
            edges: Vec::new(),
        };
        let out = VfxGraphCompiler::compile(&graph);
        assert_eq!(out.len(), 3);
        assert!(matches!(out[0], CompiledPassDesc::Particle(_)));
        assert_eq!(out[1], CompiledPassDesc::DecalStub);
        assert!(matches!(out[2], CompiledPassDesc::Screen(_)));
    }

    #[test]
    fn tc_ir_3_7_1_s1_snapshot_is_clone_only_consumer_view() {
        let graph = VfxGraph {
            nodes: vec![VfxNode {
                kind: VfxNodeKind::ParticleSim,
            }],
            edges: Vec::new(),
        };
        let compiled = VfxGraphCompiler::compile(&graph);
        let snap = drain_passes_to_snapshot(compiled);
        let clone = snap.clone();
        assert_eq!(snap, clone);
    }

    #[test]
    fn frame_tail_orders_screen_before_tonemap() {
        let tail = frame_passes_with_vfx_tail(RenderMode::Mesh);
        let screen = tail
            .iter()
            .position(|p| matches!(p, RecordedPass::ScreenEffectPass(_)))
            .unwrap();
        let tone = tail.iter().position(|p| *p == RecordedPass::TonemapPass).unwrap();
        assert!(screen < tone);
    }
}
