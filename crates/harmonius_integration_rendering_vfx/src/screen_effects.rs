//! Analytic helpers for post-process screen passes (IR-3.7.6).

/// Normalized shockwave ring radius in `[0, 1]` for `t` in `[0, 1]`.
///
/// TC-IR-3.7.6.2 — at `t = 0.5` the ring sits at half the maximum radius.
pub fn shockwave_ring_radius(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

/// Scalar heat-haze distortion strength for acceptance probes.
///
/// TC-IR-3.7.6.1 — non-zero strength when a source is active in view.
pub fn heat_haze_distortion_strength(active: bool) -> f32 {
    if active {
        0.35
    } else {
        0.0
    }
}
