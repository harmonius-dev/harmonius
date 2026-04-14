//! Histogram metering and temporal EV adaptation.

/// Auto exposure parameters for tests.
#[derive(Clone, Copy, Debug)]
pub struct AutoExposureParams {
    /// Minimum EV100 clamp.
    pub min_ev: f32,
    /// Maximum EV100 clamp.
    pub max_ev: f32,
    /// Adaptation speed in EV per second toward the target.
    pub adapt_rate_ev_per_s: f32,
}

/// Builds a 64-bin log-luminance histogram for `luminance` image (TC-2.9.4.1).
pub fn luminance_histogram_64(luminance: &[f32]) -> [u32; 64] {
    let mut bins = [0u32; 64];
    for &l in luminance {
        let log_l = l.max(1e-6).log2();
        let norm = ((log_l + 16.0) / 32.0).clamp(0.0, 1.0);
        let idx = (norm * 63.0).round() as usize;
        bins[idx] += 1;
    }
    bins
}

/// Peak bin index for a histogram.
pub fn histogram_peak_bin(bins: &[u32; 64]) -> usize {
    bins.iter()
        .enumerate()
        .max_by_key(|(_, c)| *c)
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// EV100 estimate from uniform luminance field (simplified camera response).
pub fn ev100_from_uniform_luminance(luminance: f32) -> f32 {
    (luminance.max(1e-6)).log2()
}

/// Smooths EV100 toward `target` over `dt_seconds` (TC-2.9.4.2).
pub fn smooth_ev100(
    prev_ev100: f32,
    target_ev100: f32,
    params: &AutoExposureParams,
    dt_seconds: f32,
) -> f32 {
    let delta = target_ev100 - prev_ev100;
    let step = params.adapt_rate_ev_per_s * dt_seconds;
    if delta > 0.0 {
        (prev_ev100 + step.min(delta)).clamp(params.min_ev, params.max_ev)
    } else {
        (prev_ev100 - step.min(-delta)).clamp(params.min_ev, params.max_ev)
    }
}
