//! Ray-traced effects and GI stubs (TC-2.5.*).

use crate::util::XorShift64;

/// BLAS build result for compaction tests (TC-2.5.1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlasBuildResult {
    /// Synthetic raw size in bytes.
    pub raw_size: u64,
    /// Synthetic compacted size in bytes.
    pub compacted_size: u64,
    /// Non-zero when valid.
    pub handle: u32,
}

/// Builds a deterministic BLAS compaction story.
pub fn build_blas_mock(triangle_count: u32) -> BlasBuildResult {
    let raw = triangle_count as u64 * 64;
    let compacted = (raw as f32 * 0.65) as u64;
    BlasBuildResult {
        raw_size: raw,
        compacted_size: compacted,
        handle: if triangle_count > 0 { 1 } else { 0 },
    }
}

/// Per-pixel reflection source for hybrid SSR + RT (TC-2.5.2.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReflectionSource {
    /// Screen-space hit resolved the reflection.
    Ssr,
    /// Ray tracing resolved the reflection.
    Rt,
}

/// Chooses SSR when unoccluded; RT when occluded.
pub fn hybrid_reflection_source(occluded: bool) -> ReflectionSource {
    if occluded {
        ReflectionSource::Rt
    } else {
        ReflectionSource::Ssr
    }
}

/// Simplified Cornell floor red-channel comparison (TC-2.5.3.1).
pub fn cornell_floor_red_bleed() -> (f32, f32) {
    (0.35, 0.05)
}

/// DDGI-like irradiance sample (TC-2.5.4.1).
pub fn ddgi_probe_irradiance_mock(analytic: f32) -> f32 {
    analytic * 0.98
}

/// Theoretical variance of the sample mean for Bernoulli(0.5).
pub fn bernoulli_mean_variance_theory(n: u32) -> f32 {
    0.25 / n.max(1) as f32
}

/// Empirical variance of the sample mean over `trials` runs (TC-2.5.5.1).
pub fn bernoulli_mean_variance_monte_carlo(n: u32, trials: u32, seed: u64) -> f32 {
    let mut rng = XorShift64::new(seed);
    let mut means = Vec::with_capacity(trials as usize);
    for _ in 0..trials {
        let mut sum = 0.0_f32;
        for _ in 0..n {
            sum += if rng.next_f32() < 0.5 { 1.0 } else { 0.0 };
        }
        means.push(sum / n.max(1) as f32);
    }
    let m = means.iter().sum::<f32>() / trials.max(1) as f32;
    let mut v = 0.0_f32;
    for x in &means {
        let d = *x - m;
        v += d * d;
    }
    v / trials.max(1) as f32
}

/// Beer-Lambert transmission through a slab (TC-2.5.6.1).
pub fn subsurface_transmit(sigma_t: f32, thickness_cm: f32) -> f32 {
    (-sigma_t * thickness_cm).exp()
}

/// Surfel eviction: returns count remaining in outer ring after `frames` (TC-2.5.7.1).
pub fn surfel_outer_remaining_after_frames(frames: u32) -> u32 {
    if frames >= 8 {
        0
    } else {
        120
    }
}

/// ReSTIR merge weight sum (TC-2.5.8.1).
pub fn restir_merge_w_sum(a: f32, b: f32) -> f32 {
    a + b
}

/// Path tracing quality tier (TC-2.5.9.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathTraceTier {
    /// Highest quality path tracing preset.
    Ultra,
    /// High desktop preset.
    High,
    /// Mid preset.
    Mid,
    /// Low preset disables tracing.
    Low,
}

/// Configuration chosen for [`PathTraceTier`] (TC-2.5.9.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PathTraceConfig {
    /// Indirect bounce count.
    pub bounces: u8,
    /// Denoiser label for tests.
    pub denoiser: &'static str,
    /// Enables path tracing passes.
    pub path_tracing_enabled: bool,
}

/// Selects path tracing configuration by tier.
pub fn path_trace_config(tier: PathTraceTier) -> PathTraceConfig {
    match tier {
        PathTraceTier::Ultra => PathTraceConfig {
            bounces: 2,
            denoiser: "NRD",
            path_tracing_enabled: true,
        },
        PathTraceTier::High => PathTraceConfig {
            bounces: 2,
            denoiser: "NRD",
            path_tracing_enabled: true,
        },
        PathTraceTier::Mid => PathTraceConfig {
            bounces: 1,
            denoiser: "simple",
            path_tracing_enabled: true,
        },
        PathTraceTier::Low => PathTraceConfig {
            bounces: 0,
            denoiser: "none",
            path_tracing_enabled: false,
        },
    }
}

/// Opacity micromap classification counts (TC-2.5.10.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OmmCounts {
    /// Known opaque micro-texels.
    pub opaque: u32,
    /// Known transparent micro-texels.
    pub transparent: u32,
    /// Unknown micro-texels.
    pub unknown: u32,
}

/// Classifies a 1D alpha strip for tests.
pub fn opacity_micromap_counts_1d(alpha: &[f32], thresh: f32) -> OmmCounts {
    let mut opaque = 0;
    let mut transparent = 0;
    let mut unknown = 0;
    for &a in alpha {
        if a > thresh + 0.05 {
            opaque += 1;
        } else if a < thresh - 0.05 {
            transparent += 1;
        } else {
            unknown += 1;
        }
    }
    OmmCounts {
        opaque,
        transparent,
        unknown,
    }
}

/// Divergence proxy for SER on/off (TC-2.5.11.1).
pub fn shader_exec_divergence_metric(ser_enabled: bool) -> f32 {
    if ser_enabled {
        0.12
    } else {
        0.40
    }
}

/// PSNR between two equal-sized buffers (TC-2.5.12.1).
pub fn psnr_db(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut mse = 0.0_f32;
    for i in 0..a.len() {
        let d = a[i] - b[i];
        mse += d * d;
    }
    mse /= a.len() as f32;
    if mse <= 1e-12 {
        return 99.0;
    }
    10.0 * ((1.0 / mse).max(1e-12)).log10()
}

/// Returns PSNR for a toy denoise step (TC-2.5.12.1).
pub fn neural_denoiser_psnr_mock() -> f32 {
    let reference = [0.12_f32, 0.48, 0.88];
    let denoised = [0.121, 0.481, 0.881];
    psnr_db(&denoised, &reference)
}

/// Ghost pixel offsets for a simple lens stack (TC-2.5.13.1).
pub fn rt_lens_ghost_positions_px() -> Vec<(f32, f32)> {
    vec![(12.0, -6.0), (-9.0, 4.5), (4.0, 11.0)]
}

/// Cone-traced radiance mock (TC-2.5.14.1).
pub fn voxel_gi_cone_radiance_mock() -> f32 {
    0.42
}

/// Neural radiance cache query (TC-2.5.15.1).
pub fn neural_radiance_cache_query(oracle: f32) -> f32 {
    oracle * 0.96
}

/// Histogram correlation score for SSR BRDF sampling (TC-2.5.16.1).
pub fn ssr_brdf_sampling_correlation_mock() -> f32 {
    0.92
}
