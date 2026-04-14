//! Polyline-based signed-distance helpers for spline SDF acceptance tests.

use glam::Vec3;

/// Samples a polyline as linear segments.
pub fn polyline_distance(p: Vec3, points: &[Vec3]) -> f32 {
    let mut best = f32::MAX;
    for w in points.windows(2) {
        let a = w[0];
        let b = w[1];
        let ab = b - a;
        let t = ((p - a).dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
        let closest = a + ab * t;
        best = best.min(p.distance(closest));
    }
    best
}

/// Rasterizes a coarse SDF texture by brute-force segment distance.
pub fn sdf_texture_brute(points: &[Vec3], origin: Vec3, step: f32, res: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(res * res);
    for j in 0..res {
        for i in 0..res {
            let x = origin.x + i as f32 * step;
            let z = origin.z + j as f32 * step;
            let p = Vec3::new(x, origin.y, z);
            out.push(polyline_distance(p, points));
        }
    }
    out
}

/// Fast approximate SDF by sampling the same grid (placeholder for GPU texture path).
pub fn sdf_texture_fast(points: &[Vec3], origin: Vec3, step: f32, res: usize) -> Vec<f32> {
    sdf_texture_brute(points, origin, step, res)
}
