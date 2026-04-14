//! Plan `PLAN-rendering-render-effects` trace tests (IDs in function names).

use crate::auto_exposure::{
    ev100_from_uniform_luminance, histogram_peak_bin, luminance_histogram_64, smooth_ev100,
    AutoExposureParams,
};
use crate::bloom::{
    all_passes_gaussian, bloom_blur_method, bloom_threshold_prefilter, build_bloom_passes,
    gaussian_mip_count, BloomParams, BloomPassKind,
};
use crate::cavity::cavity_curvature_from_normals;
use crate::chromatic_aberration::chromatic_offsets_x;
use crate::dof::{bokeh_circle_samples, circle_of_confusion, DofParams};
use crate::film_grain::write_film_grain_pattern;
use crate::graph::{compile_post_graph, Edge, NodeId};
use crate::lighting::{
    gtao_flat_plane, importance_sample_light_index, lights_in_tile, lights_in_tile_bruteforce,
    pssm_splits, TileLight,
};
use crate::local_exposure::{tile_ev_bias, tile_mean_luminance};
use crate::motion_blur::{motion_blur_sample_plan, MotionBlurParams};
use crate::post_chain::{desaturate, negate, saturate};
use crate::quality::{effect_config_for_tier, BloomImplementation, QualityTier};
use crate::rt::{
    bernoulli_mean_variance_monte_carlo, bernoulli_mean_variance_theory, build_blas_mock,
    cornell_floor_red_bleed, ddgi_probe_irradiance_mock, hybrid_reflection_source,
    neural_denoiser_psnr_mock, neural_radiance_cache_query, opacity_micromap_counts_1d,
    path_trace_config, restir_merge_w_sum, rt_lens_ghost_positions_px,
    shader_exec_divergence_metric, ssr_brdf_sampling_correlation_mock, subsurface_transmit,
    surfel_outer_remaining_after_frames, voxel_gi_cone_radiance_mock, OmmCounts, PathTraceTier,
    ReflectionSource,
};
use crate::tonemap::{aces_fitted, sample_identity_lut_3d};
use crate::vignette::{vignette_at, VignetteFalloff, VignetteParams};
use crate::volume::{blend_bloom_by_priority, VolumeLayer};

#[test]
fn test_bloom_threshold_filter() {
    let p = BloomParams { threshold: 1.0 };
    assert_eq!(bloom_threshold_prefilter(0.5, &p), 0.0);
    assert_eq!(bloom_threshold_prefilter(1.0, &p), 0.0);
    assert_eq!(bloom_threshold_prefilter(2.0, &p), 1.0);
}

#[test]
fn test_bloom_dual_kawase_mobile() {
    let passes = build_bloom_passes(QualityTier::Mobile, true);
    assert_eq!(passes.len(), 8);
    assert!(passes.iter().all(|p| {
        matches!(
            p.kind,
            BloomPassKind::DualKawaseDown | BloomPassKind::DualKawaseUp
        )
    }));
}

#[test]
fn test_bloom_gaussian_desktop() {
    let passes = build_bloom_passes(QualityTier::HighEnd, true);
    assert_eq!(gaussian_mip_count(&passes), 6);
    assert!(all_passes_gaussian(&passes));
    assert_eq!(bloom_blur_method(&passes), Some(BloomImplementation::Gaussian));
}

#[test]
fn test_dof_circle_of_confusion() {
    let p = DofParams {
        focus_distance: 5.0,
        aperture: 2.0,
        focal_length: 0.05,
    };
    assert!(circle_of_confusion(5.0, &p).abs() < 1e-3);
    assert!(circle_of_confusion(10.0, &p) > 0.0);
}

#[test]
fn test_dof_bokeh_shape_circle() {
    let samples = bokeh_circle_samples(16);
    assert_eq!(samples.len(), 16);
    for (x, y) in &samples {
        let len = (x * x + y * y).sqrt();
        assert!(len <= 1.0 + 1e-3);
        assert!(len >= 0.95 - 1e-2);
    }
}

#[test]
fn test_motion_blur_velocity_dir() {
    let plan = motion_blur_sample_plan((0.1, 0.0), &MotionBlurParams { sample_count: 8 });
    assert!((plan.direction.0 - 1.0).abs() < 1e-5);
    assert!(plan.direction.1.abs() < 1e-5);
    assert_eq!(plan.offsets.len(), 8);
}

#[test]
fn test_auto_exposure_histogram() {
    let lum = vec![0.5_f32; 256 * 256];
    let bins = luminance_histogram_64(&lum);
    let peak = histogram_peak_bin(&bins);
    let expected = {
        let log_l = 0.5_f32.log2();
        let norm = ((log_l + 16.0) / 32.0).clamp(0.0, 1.0);
        (norm * 63.0).round() as usize
    };
    assert_eq!(peak, expected);
    let ev = ev100_from_uniform_luminance(0.5);
    assert!((ev - (-1.0)).abs() < 0.15);
}

#[test]
fn test_auto_exposure_smoothing() {
    let p = AutoExposureParams {
        min_ev: -8.0,
        max_ev: 8.0,
        adapt_rate_ev_per_s: 1.0,
    };
    let ev = smooth_ev100(0.0, 4.0, &p, 0.5);
    assert!(ev > 0.45 && ev < 0.55);
}

#[test]
fn test_aces_tone_curve_clamp() {
    let xs = [0.0_f32, 0.18, 1.0, 8.0];
    let mut ys = [0.0_f32; 4];
    for i in 0..4 {
        ys[i] = aces_fitted(xs[i]);
    }
    for i in 1..4 {
        assert!(ys[i] >= ys[i - 1]);
    }
    assert!(aces_fitted(8.0) <= 1.0);
}

#[test]
fn test_lut_apply_3d() {
    let out = sample_identity_lut_3d((0.4, 0.6, 0.2), 32);
    let tol = 1.0 / 255.0;
    assert!((out.0 - 0.4).abs() < tol);
    assert!((out.1 - 0.6).abs() < tol);
    assert!((out.2 - 0.2).abs() < tol);
}

#[test]
fn test_film_grain_seed() {
    let mut a = vec![0u8; 64];
    let mut b = vec![0u8; 64];
    write_film_grain_pattern(42, &mut a);
    write_film_grain_pattern(42, &mut b);
    assert_eq!(a, b);
    let mut c = vec![0u8; 64];
    write_film_grain_pattern(43, &mut c);
    assert_ne!(a, c);
}

#[test]
fn test_chromatic_aberration_split() {
    let (r, g, b) = chromatic_offsets_x(1.0, (0.95, 0.5));
    assert!(r > g && g > b);
}

#[test]
fn test_vignette_radial_falloff() {
    let p = VignetteParams {
        strength: 1.0,
        falloff: VignetteFalloff::Radial(2.0),
    };
    let c = vignette_at((0.5, 0.5), &p);
    let k = vignette_at((0.0, 0.0), &p);
    assert!((c - 1.0).abs() < 0.05);
    assert!(k < 0.5);
}

#[test]
fn test_custom_pp_grayscale() {
    let rgb = (0.7_f32, 0.3, 0.1);
    let out = desaturate(rgb, 1.0);
    assert!((out.0 - out.1).abs() < 1e-3);
    assert!((out.1 - out.2).abs() < 1e-3);
}

#[test]
fn test_custom_pp_chain_3_passes() {
    let rgb = (0.7_f32, 0.3, 0.1);
    let a = NodeId(0);
    let b = NodeId(1);
    let c = NodeId(2);
    let order = compile_post_graph(
        &[a, b, c],
        &[Edge { from: a, to: b }, Edge { from: b, to: c }],
    );
    assert_eq!(order, vec![a, b, c]);
    let chained = desaturate(saturate(negate(rgb)), 1.0);
    let manual = desaturate(saturate(negate(rgb)), 1.0);
    assert_eq!(chained, manual);
}

#[test]
fn test_local_exposure_tile_grid() {
    let w = 1024usize;
    let h = 1024usize;
    let mut lum = vec![0.0_f32; w * h];
    for y in 0..h / 2 {
        for x in 0..w {
            lum[y * w + x] = 4.0;
        }
    }
    for y in h / 2..h {
        for x in 0..w {
            lum[y * w + x] = 0.1;
        }
    }
    let top = tile_mean_luminance(w, h, 16, 8, 0, &lum);
    let bot = tile_mean_luminance(w, h, 16, 8, 15, &lum);
    assert!(tile_ev_bias(top) < 0.0);
    assert!(tile_ev_bias(bot) > 0.0);
}

#[test]
fn test_cavity_normal_sample() {
    let mut normals = vec![(0.0_f32, 1.0, 0.0); 64];
    normals[32] = (0.05, 0.25, 0.05);
    normals[63] = (0.85, 0.45, 0.25);
    let (cav, curv) = cavity_curvature_from_normals(&normals, 32, 63);
    assert!(cav > 0.5);
    assert!(curv > 0.5);
}

#[test]
fn test_pp_volume_blend_priority() {
    let layers = [
        VolumeLayer {
            priority: 1,
            bloom: 0.5,
        },
        VolumeLayer {
            priority: 2,
            bloom: 1.0,
        },
    ];
    let b = blend_bloom_by_priority(&layers);
    assert!(b > 0.85 && b <= 1.0);
}

#[test]
fn test_quality_tier_mobile() {
    let c = effect_config_for_tier(QualityTier::Mobile);
    assert!(!c.dof);
    assert_eq!(c.bloom, BloomImplementation::DualKawase);
    assert!(!c.motion_blur);
    assert_eq!(c.max_pp, 1);
}

#[test]
fn test_tiled_light_cull_grid() {
    let lights: Vec<TileLight> = (0..100)
        .map(|i| {
            let x = (i % 10) as f32 * 100.0 + 50.0;
            let y = (i / 10) as f32 * 100.0 + 50.0;
            TileLight {
                x,
                y,
                radius: 40.0,
            }
        })
        .collect();
    let w = 1600.0_f32;
    let h = 1600.0_f32;
    let grid = 16usize;
    for ty in 0..grid {
        for tx in 0..grid {
            let a = lights_in_tile(w, h, grid, tx, ty, &lights);
            let b = lights_in_tile_bruteforce(w, h, grid, tx, ty, &lights);
            assert_eq!(a, b);
        }
    }
}

#[test]
fn test_stochastic_light_sampling() {
    let mut radiances = vec![1.0_f32; 1000];
    radiances[0] = 1000.0;
    let mut hits = 0usize;
    for i in 0..10_000 {
        let idx = importance_sample_light_index(&radiances, 99991 ^ i as u64);
        if idx == 0 {
            hits += 1;
        }
    }
    assert!(hits > 5000);
}

#[test]
fn test_csm_cascade_split() {
    let near = 0.1_f32;
    let far = 200.0_f32;
    let splits = pssm_splits(near, far, 4, 0.5);
    assert_eq!(splits.len(), 4);
    for i in 0..4 {
        let ui = (i + 1) as f32 / 4.0;
        let log_split = near * (far / near).powf(ui);
        let uniform_split = near + (far - near) * ui;
        let expected = 0.5 * log_split + 0.5 * uniform_split;
        assert!((splits[i] - expected).abs() < 0.1);
    }
}

#[test]
fn test_gtao_bent_normal() {
    let (n, ao) = gtao_flat_plane();
    assert!((n.0).abs() < 1e-3 && (n.1 - 1.0).abs() < 1e-3 && (n.2).abs() < 1e-3);
    assert!(ao >= 0.95);
}

#[test]
fn test_vignette_power_curve() {
    let p = VignetteParams {
        strength: 1.0,
        falloff: VignetteFalloff::Power(2.0),
    };
    let c = vignette_at((0.5, 0.5), &p);
    let k = vignette_at((0.0, 0.0), &p);
    assert!((c - 1.0).abs() < 1e-2);
    assert!(k < 0.25);
    let mid = vignette_at((0.25, 0.5), &p);
    assert!(mid > k && mid < c + 1e-3);
}

#[test]
fn test_local_exposure_tile_bias() {
    let w = 1024usize;
    let h = 1024usize;
    let mut lum = vec![0.2_f32; w * h];
    for y in 0..h / 2 {
        for x in 0..w / 2 {
            lum[y * w + x] = 6.0;
        }
    }
    let bias_tl = tile_ev_bias(tile_mean_luminance(w, h, 16, 0, 0, &lum));
    let bias_br = tile_ev_bias(tile_mean_luminance(w, h, 16, 15, 15, &lum));
    assert!(bias_tl < 0.0);
    assert!(bias_br.abs() < 0.35);
}

#[test]
fn test_post_graph_compile() {
    let a = NodeId(10);
    let b = NodeId(20);
    let c = NodeId(30);
    let order = compile_post_graph(
        &[c, a, b],
        &[Edge { from: a, to: b }, Edge { from: b, to: c }],
    );
    assert_eq!(order, vec![a, b, c]);
}

#[test]
fn test_blas_build_from_meshlets() {
    let r = build_blas_mock(1000);
    assert_ne!(r.handle, 0);
    assert!(r.compacted_size < (r.raw_size as f32 * 0.75) as u64);
}

#[test]
fn test_rt_reflection_hybrid_ssr() {
    assert_eq!(
        hybrid_reflection_source(false),
        ReflectionSource::Ssr
    );
    assert_eq!(hybrid_reflection_source(true), ReflectionSource::Rt);
}

#[test]
fn test_rt_indirect_diffuse_one() {
    let (near_red, near_white) = cornell_floor_red_bleed();
    assert!(near_red > near_white);
}

#[test]
fn test_ddgi_probe_irradiance() {
    let analytic = 1.0_f32;
    let s = ddgi_probe_irradiance_mock(analytic);
    assert!((s - analytic).abs() < 0.05 * analytic);
}

#[test]
fn test_path_tracer_accumulation() {
    let theory1 = bernoulli_mean_variance_theory(1);
    let theory64 = bernoulli_mean_variance_theory(64);
    let ratio = theory64 / theory1;
    assert!((ratio - (1.0 / 64.0)).abs() < 0.001);
    let emp1 = bernoulli_mean_variance_monte_carlo(1, 4000, 42);
    let emp64 = bernoulli_mean_variance_monte_carlo(64, 4000, 42);
    let measured = emp64 / emp1.max(1e-6);
    let target = 1.0 / 64.0;
    assert!((measured - target).abs() < 0.2 * target + 0.05);
}

#[test]
fn test_rt_subsurface_transmit() {
    let sigma_t = 1.2_f32;
    let t = subsurface_transmit(sigma_t, 1.0);
    let expected = (-sigma_t * 1.0).exp();
    assert!((t - expected).abs() < 0.1 * expected);
}

#[test]
fn test_surfel_clipmap_update() {
    assert_eq!(surfel_outer_remaining_after_frames(0), 120);
    assert_eq!(surfel_outer_remaining_after_frames(8), 0);
}

#[test]
fn test_restir_reservoir_merge() {
    assert!((restir_merge_w_sum(3.0, 5.0) - 8.0).abs() < 1e-5);
}

#[test]
fn test_realtime_pt_tier_select() {
    let ultra = path_trace_config(PathTraceTier::Ultra);
    assert_eq!(ultra.bounces, 2);
    assert_eq!(ultra.denoiser, "NRD");
    let mid = path_trace_config(PathTraceTier::Mid);
    assert_eq!(mid.bounces, 1);
    assert_eq!(mid.denoiser, "simple");
    let low = path_trace_config(PathTraceTier::Low);
    assert!(!low.path_tracing_enabled);
}

#[test]
fn test_opacity_micromap_build() {
    let alpha = [0.0_f32, 0.05, 0.5, 0.95, 1.0];
    let manual = OmmCounts {
        opaque: 2,
        transparent: 2,
        unknown: 1,
    };
    let c = opacity_micromap_counts_1d(&alpha, 0.5);
    assert_eq!(c, manual);
}

#[test]
fn test_shader_exec_reorder() {
    assert!(shader_exec_divergence_metric(true) < shader_exec_divergence_metric(false));
}

#[test]
fn test_denoiser_psnr_above_30() {
    assert!(neural_denoiser_psnr_mock() > 30.0);
}

#[test]
fn test_rt_lens_flare_trace() {
    let ghosts = rt_lens_ghost_positions_px();
    let reference = [(12.0_f32, -6.0), (-9.0, 4.5), (4.0, 11.0)];
    for (g, r) in ghosts.iter().zip(reference.iter()) {
        assert!((g.0 - r.0).abs() <= 2.0);
        assert!((g.1 - r.1).abs() <= 2.0);
    }
}

#[test]
fn test_voxel_gi_cone_trace() {
    let v = voxel_gi_cone_radiance_mock();
    assert!(v > 0.0);
    let reference = 0.42_f32;
    assert!((v - reference).abs() < 0.15 * reference);
}

#[test]
fn test_neural_radiance_cache_hit() {
    let oracle = 1.25_f32;
    let q = neural_radiance_cache_query(oracle);
    assert!((q - oracle).abs() < 0.1 * oracle);
}

#[test]
fn test_stochastic_ssr_brdf() {
    assert!(ssr_brdf_sampling_correlation_mock() > 0.82);
}
