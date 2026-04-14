//! Orthographic and reverse-Z perspective projections (R-2.3.5, R-2.3.6).

use glam::{Mat4, Vec4};

/// Right-handed orthographic projection with depth range `[0, 1]` (Vulkan-style clip Z).
///
/// `near` and `far` are positive distances along the camera forward axis (`-Z` in view space).
#[must_use]
pub fn orthographic_rh_z01(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Mat4 {
    Mat4::orthographic_rh(left, right, bottom, top, near, far)
}

/// Reverse-Z, right-handed perspective with an infinite far plane and clip Z in `[0, 1]`.
#[must_use]
pub fn reverse_z_infinite_perspective_rh(fovy_radians: f32, aspect: f32, z_near: f32) -> Mat4 {
    Mat4::perspective_infinite_reverse_rh(fovy_radians, aspect, z_near)
}

/// Homogeneous divide: `clip.xyz() / clip.w`, returning `None` if `w` is zero.
#[must_use]
pub fn ndc_from_clip(clip: Vec4) -> Option<glam::Vec3> {
    let w = clip.w;
    if w == 0.0 {
        return None;
    }
    Some(clip.truncate() / w)
}

#[cfg(test)]
mod tests {
    use super::{ndc_from_clip, orthographic_rh_z01, reverse_z_infinite_perspective_rh};
    use glam::Vec4;

    /// TC-2.3.5.1 — orthographic projection: projected X does not depend on view-space depth.
    #[test]
    fn test_orthographic_projection_no_fov() {
        let proj = orthographic_rh_z01(-10.0, 10.0, -10.0, 10.0, 0.1, 100.0);
        let p_near = Vec4::new(1.0, 0.0, -10.0, 1.0);
        let p_far = Vec4::new(1.0, 0.0, -100.0, 1.0);
        let n0 = ndc_from_clip(proj * p_near).expect("ndc");
        let n1 = ndc_from_clip(proj * p_far).expect("ndc");
        assert!((n0.x - n1.x).abs() < 1e-5, "ortho must not foreshorten X");
    }

    /// TC-2.3.6.1 — reverse-Z infinite far: nearer geometry maps to higher NDC Z than distant.
    #[test]
    fn test_reverse_z_infinite_far() {
        let proj = reverse_z_infinite_perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1);
        let clip_near = proj * Vec4::new(0.0, 0.0, -1.0, 1.0);
        let clip_far = proj * Vec4::new(0.0, 0.0, -10_000.0, 1.0);
        let ndc_near = ndc_from_clip(clip_near).expect("ndc");
        let ndc_far = ndc_from_clip(clip_far).expect("ndc");
        assert!(ndc_near.z > ndc_far.z, "reverse-Z monotonicity");
        assert!((0.0..=1.0).contains(&ndc_near.z));
        assert!((0.0..=1.0).contains(&ndc_far.z));
    }
}
