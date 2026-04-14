//! Vegetation rule evaluation against analytic terrain heightfields.

use glam::Vec3;

/// Clears instances within `clearance_m` of a polyline in XZ.
pub fn clear_along_polyline(points: &[Vec3], instances: &[Vec3], clearance_m: f32) -> Vec<Vec3> {
    let r2 = clearance_m * clearance_m;
    instances
        .iter()
        .copied()
        .filter(|inst| {
            !points.windows(2).any(|w| {
                let a = w[0];
                let b = w[1];
                let ab = b - a;
                let t = ((*inst - a).dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
                let closest = a + ab * t;
                inst.distance_squared(closest) <= r2
            })
        })
        .collect()
}

/// Height function `y = f(x,z)` for tests.
pub type HeightFn = fn(f32, f32) -> f32;

/// Computes terrain slope angle (degrees) from height partials.
pub fn slope_degrees(f: HeightFn, x: f32, z: f32, h: f32) -> f32 {
    let dx = (f(x + h, z) - f(x - h, z)) / (2.0 * h);
    let dz = (f(x, z + h) - f(x, z - h)) / (2.0 * h);
    (dx * dx + dz * dz).sqrt().atan().to_degrees()
}

/// Applies a max-slope rule in degrees.
pub fn vegetation_allowed(slope: f32, max_slope_deg: f32) -> bool {
    slope < max_slope_deg
}

/// Filters instance positions by terrain slope under `terrain`.
pub fn filter_by_slope(positions: &[Vec3], terrain: HeightFn, max_slope_deg: f32) -> Vec<Vec3> {
    positions
        .iter()
        .copied()
        .filter(|p| {
            let s = slope_degrees(terrain, p.x, p.z, 0.25);
            vegetation_allowed(s, max_slope_deg)
        })
        .collect()
}
