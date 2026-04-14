//! Deterministic CPU stepping for individual simulation modules (`TC-11.1.2.*` subset).

use glam::Vec3;

/// Applies euler gravity integration: `velocity + gravity * dt`.
#[must_use]
pub fn apply_gravity(velocity: Vec3, gravity: Vec3, dt: f32) -> Vec3 {
    velocity + gravity * dt
}

/// Applies linear drag: `velocity * (1.0 - drag * dt)` (clamped so velocity does not flip sign
/// unexpectedly for extreme coefficients).
#[must_use]
pub fn apply_drag(velocity: Vec3, drag: f32, dt: f32) -> Vec3 {
    let factor = (1.0 - drag * dt).max(0.0);
    velocity * factor
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.2.1` — gravity module sample.
    #[test]
    fn tc_11_1_2_1_gravity_module() {
        let v = Vec3::ZERO;
        let g = Vec3::new(0.0, -9.81, 0.0);
        let dt = 1.0 / 60.0;
        let out = apply_gravity(v, g, dt);
        let expected = -0.1635;
        assert!((out.y - expected).abs() < 1e-4, "got {} expected {}", out.y, expected);
        assert_eq!(out.x, 0.0);
        assert_eq!(out.z, 0.0);
    }

    /// `TC-11.1.2.3` — drag module sample.
    #[test]
    fn tc_11_1_2_3_drag_module() {
        let v = Vec3::splat(10.0);
        let out = apply_drag(v, 0.5, 1.0);
        assert_approx_vec3(out, Vec3::splat(5.0), 1e-5);
    }

    fn assert_approx_vec3(a: Vec3, b: Vec3, eps: f32) {
        assert!(
            (a - b).length() <= eps,
            "expected {a:?} ~= {b:?} (eps {eps})"
        );
    }
}
