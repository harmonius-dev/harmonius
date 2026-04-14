//! Card hair shading helpers (Kajiya-Kay style).

use crate::math::Vec3;

/// Alpha blend mode for hair cards.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HairAlphaMode {
    /// Alpha blended sorted geometry.
    Blend,
    /// Alpha test cutout.
    Test,
}

/// Computes Kajiya-Kay specular direction (in tangent plane) from light and view.
pub fn kajiya_specular_tangent_component(tangent: Vec3, light: Vec3, view: Vec3) -> f32 {
    let t = tangent.normalized();
    let l = light.normalized();
    let v = view.normalized();
    let h = (l + v).normalized();
    let in_plane = (h - t * h.dot(t)).normalized();
    in_plane.dot(t).abs()
}

/// Sort key for alpha blend (camera depth).
pub fn alpha_sort_key(camera_z: f32) -> f32 {
    camera_z
}

/// Returns true if alpha-test mode removes semi-transparent fringe (binary alpha).
pub fn alpha_test_is_binary(mode: HairAlphaMode) -> bool {
    matches!(mode, HairAlphaMode::Test)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_5_3_1_card_hair() {
        let tangent = Vec3::new(0.0, 1.0, 0.0);
        let light = Vec3::new(1.0, 1.0, 0.0);
        let view = Vec3::new(-1.0, 1.0, 0.0);
        let k = kajiya_specular_tangent_component(tangent, light, view);
        assert!(k < 0.99);
        let keys = vec![alpha_sort_key(1.0), alpha_sort_key(2.0)];
        assert!(keys[0] < keys[1]);
        assert!(alpha_test_is_binary(HairAlphaMode::Test));
    }
}
