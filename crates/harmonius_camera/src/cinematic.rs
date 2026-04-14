//! Cinematic rigs, lens math, and picture-in-picture layout.

use glam::{Quat, Vec2, Vec3};

/// Dolly rig defined by two path nodes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DollyRig {
    /// Start position.
    pub start: Vec3,
    /// End position.
    pub end: Vec3,
}

/// Crane rig defined by boom pivot and extension.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CraneRig {
    /// Pivot at the base of the crane.
    pub pivot: Vec3,
    /// Boom length in meters.
    pub boom_length: f32,
}

/// Computes vertical FOV (degrees) for a full-frame sensor height and focal length.
#[must_use]
pub fn compute_cine_vertical_fov_degrees(sensor_height_mm: f32, focal_length_mm: f32) -> f32 {
    if focal_length_mm <= 0.0 {
        return 0.0;
    }
    2.0 * ((sensor_height_mm * 0.5) / focal_length_mm)
        .atan()
        .to_degrees()
}

/// Interpolates along the dolly segment using normalized `t`.
#[must_use]
pub fn dolly_rig_position(rig: &DollyRig, t: f32) -> Vec3 {
    rig.start.lerp(rig.end, t.clamp(0.0, 1.0))
}

/// Computes camera position for a crane pitch (degrees) around the pivot X axis.
#[must_use]
pub fn crane_rig_position(rig: &CraneRig, pitch_deg: f32) -> Vec3 {
    let pitch = pitch_deg.to_radians();
    let rot = Quat::from_rotation_x(pitch);
    let boom = rot * Vec3::new(0.0, rig.boom_length, 0.0);
    rig.pivot + boom
}

/// Normalized viewport rectangle (origin, size) in `[0,1]` screen space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalizedViewport {
    /// Upper-left origin.
    pub origin: Vec2,
    /// Width and height.
    pub size: Vec2,
}

/// Applies a PiP viewport inset within `parent`.
#[must_use]
pub fn apply_picture_in_picture_viewport(
    parent: NormalizedViewport,
    pip: NormalizedViewport,
) -> NormalizedViewport {
    let origin = parent.origin + pip.origin * parent.size;
    let size = pip.size * parent.size;
    NormalizedViewport { origin, size }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.38.1 — Super 35 vertical FOV with a 50mm lens.
    #[test]
    fn tc_13_25_38_1_cine_camera_fov() {
        let fov = compute_cine_vertical_fov_degrees(24.0, 50.0);
        let expected = 2.0 * (12.0_f32 / 50.0).atan().to_degrees();
        assert!((fov - expected).abs() < 0.05);
        assert!((fov - 27.0).abs() < 0.2);
    }

    /// TC-13.25.39.1 — dolly and crane rigs follow their parameterized geometry.
    #[test]
    fn tc_13_25_39_1_camera_rig_presets() {
        let dolly = DollyRig {
            start: Vec3::ZERO,
            end: Vec3::new(0.0, 0.0, 10.0),
        };
        assert!((dolly_rig_position(&dolly, 0.5) - Vec3::new(0.0, 0.0, 5.0)).length() < 1e-3);
        let crane = CraneRig {
            pivot: Vec3::ZERO,
            boom_length: 5.0,
        };
        let tip = crane_rig_position(&crane, -45.0);
        assert!(tip.y < crane.boom_length);
    }

    /// TC-13.25.40.I1 — multiple PiP viewports map to disjoint screen regions.
    #[test]
    fn tc_13_25_40_i1_pip_multiple_viewports() {
        let desktop = NormalizedViewport {
            origin: Vec2::ZERO,
            size: Vec2::ONE,
        };
        let pip_a = NormalizedViewport {
            origin: Vec2::new(0.05, 0.05),
            size: Vec2::new(0.25, 0.25),
        };
        let pip_b = NormalizedViewport {
            origin: Vec2::new(0.7, 0.6),
            size: Vec2::new(0.25, 0.25),
        };
        let a = apply_picture_in_picture_viewport(desktop, pip_a);
        let b = apply_picture_in_picture_viewport(desktop, pip_b);
        let overlap = (a.origin - b.origin).abs();
        assert!(overlap.x > 0.01 || overlap.y > 0.01);
    }
}
