//! Render-space transform interpolation (extract phase, R-2.10.8).

use glam::Vec3;

/// Linearly interpolates `prev` → `next` with factor `alpha` in `[0, 1]`.
#[must_use]
pub fn interpolate_translation(prev: Vec3, next: Vec3, alpha: f32) -> Vec3 {
    prev.lerp(next, alpha)
}

#[cfg(test)]
mod tests {
    use super::interpolate_translation;
    use glam::Vec3;

    /// TC-2.10.8.1 — half alpha between `(0,0,0)` and `(10,0,0)` is `(5,0,0)`.
    #[test]
    fn test_transform_interpolation_alpha() {
        let prev = Vec3::ZERO;
        let next = Vec3::new(10.0, 0.0, 0.0);
        let mid = interpolate_translation(prev, next, 0.5);
        assert!((mid - Vec3::new(5.0, 0.0, 0.0)).length() < 1e-5);
    }
}
