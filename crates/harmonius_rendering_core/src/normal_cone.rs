//! Meshlet normal-cone backface culling predicate (R-2.3.3, TC-2.3.3.1).
//!
//! Matches the common meshoptimizer-style test: compare the normalized view vector from the
//! meshlet toward the camera against the clustered cone axis and a scalar cutoff.

use glam::Vec3;

/// Unit view from meshlet reference position toward the camera, unit cone axis, and cosine-style
/// cutoff. Returns `true` when the meshlet is cone-culled (treated as backfacing).
#[must_use]
pub fn meshlet_cone_culled(view_to_camera: Vec3, cone_axis: Vec3, cone_cutoff: f32) -> bool {
    view_to_camera.dot(cone_axis) >= cone_cutoff
}

#[cfg(test)]
mod tests {
    use super::meshlet_cone_culled;
    use glam::Vec3;

    fn unit_sphere_vec3(mut seed: u32) -> Vec3 {
        let u1 = (seed as f32) * (1.0 / u32::MAX as f32);
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let u2 = (seed as f32) * (1.0 / u32::MAX as f32);
        let z = 1.0 - 2.0 * u1;
        let r = (1.0 - z * z).max(0.0).sqrt();
        let theta = std::f32::consts::TAU * u2;
        Vec3::new(r * theta.cos(), r * theta.sin(), z)
    }

    /// TC-2.3.3.1 — random cone axes split by the camera plane yield ~50% cone rejections.
    #[test]
    fn test_normal_cone_backface_cull() {
        let view = Vec3::Z;
        let mut axes = Vec::with_capacity(10_000);
        let mut x = 1_u32;
        for _ in 0..10_000 {
            // Cheap deterministic quasi-uniform directions (no `rand` dependency).
            let v = unit_sphere_vec3(x);
            x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            axes.push(v);
        }
        let culled = axes
            .iter()
            .filter(|axis| meshlet_cone_culled(view, **axis, 0.0))
            .count();
        let expected = 5_000_usize;
        let tol = (expected / 10).max(400);
        assert!(
            culled.abs_diff(expected) <= tol,
            "culled={culled}, expected ~{expected} ± {tol}"
        );
    }
}
