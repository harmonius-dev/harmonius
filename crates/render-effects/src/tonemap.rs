//! ACES tonemap and 3D LUT sampling.

/// Applies a compact ACES fitted curve (TC-2.9.5.1).
pub fn aces_fitted(x: f32) -> f32 {
    let a = 2.51_f32;
    let b = 0.03_f32;
    let c = 2.43_f32;
    let d = 0.59_f32;
    let e = 0.14_f32;
    let x = x.max(0.0);
    let y = ((x * (a * x + b)).min(f32::MAX)) / (x * (c * x + d) + e).max(1e-6);
    y.min(1.0)
}

/// Samples a `dim^3` identity 3D LUT (TC-2.9.5.2).
pub fn sample_identity_lut_3d(rgb: (f32, f32, f32), _dim: usize) -> (f32, f32, f32) {
    let clamp = |v: f32| v.clamp(0.0, 1.0);
    (clamp(rgb.0), clamp(rgb.1), clamp(rgb.2))
}
