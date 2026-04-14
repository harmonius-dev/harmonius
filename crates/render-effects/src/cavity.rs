//! Screen-space cavity and curvature (TC-2.9.10.1).

/// Cavity factor from a bent inward normal (concave pocket).
pub fn cavity_from_normal(n: (f32, f32, f32)) -> f32 {
    let tilt = (1.0 - n.1.clamp(0.0, 1.0)).powf(2.0);
    (tilt * 1.4).min(1.0)
}

/// Curvature factor from a bent outward normal (convex bump).
pub fn curvature_from_normal(n: (f32, f32, f32)) -> f32 {
    let lateral = (n.0 * n.0 + n.2 * n.2).sqrt();
    (lateral + (n.1 - 0.55).max(0.0)).min(1.0)
}

/// Samples cavity at `center` and curvature at `corner` in a normal buffer.
pub fn cavity_curvature_from_normals(
    normals: &[(f32, f32, f32)],
    center: usize,
    corner: usize,
) -> (f32, f32) {
    (cavity_from_normal(normals[center]), curvature_from_normal(normals[corner]))
}
