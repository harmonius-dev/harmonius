//! Rotation behaviors for virtual cameras.

use glam::{Quat, Vec2, Vec3};

use crate::types::ScreenRect;

/// Adaptive rotation composer configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RotationComposer {
    /// Offset in target-local space.
    pub target_offset: Vec3,
    /// Screen-space anchor.
    pub screen_position: Vec2,
    /// Dead zone in normalized coordinates.
    pub dead_zone: ScreenRect,
    /// Soft zone in normalized coordinates.
    pub soft_zone: ScreenRect,
    /// Rotational damping (seconds).
    pub damping: f32,
    /// Lookahead time (seconds).
    pub lookahead_time: f32,
}

/// Rigid look-at behavior marker.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HardLookAt;

/// Input-driven pan and tilt configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanTilt {
    /// Horizontal range (degrees).
    pub pan_range: (f32, f32),
    /// Horizontal wrap.
    pub pan_wrap: bool,
    /// Vertical clamp (degrees).
    pub tilt_clamp: (f32, f32),
}

/// Reference frame tag for pan/tilt (subset for tests).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanTiltReference {
    World,
}

/// Copies tracking target rotation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RotateWithFollowTarget;

/// Returns an incremental rotation delta (axis-angle vector) for composer dead-zoning.
#[must_use]
pub fn evaluate_rotation_composer_delta(composer: &RotationComposer, subject_screen: Vec2) -> Vec3 {
    let anchor = composer.screen_position;
    let err = subject_screen - anchor;
    let dead = &composer.dead_zone;
    if subject_screen.x >= dead.min.x
        && subject_screen.x <= dead.max.x
        && subject_screen.y >= dead.min.y
        && subject_screen.y <= dead.max.y
    {
        return Vec3::ZERO;
    }
    let soft = &composer.soft_zone;
    let clamped = subject_screen.clamp(soft.min, soft.max);
    Vec3::new(
        (clamped - subject_screen).x,
        (clamped - subject_screen).y,
        err.x,
    ) / composer.damping.max(1e-3)
}

/// Computes a look rotation that centers `target` from `eye` with +Z up convention.
#[must_use]
pub fn evaluate_hard_look_at_rotation(eye: Vec3, target: Vec3, _up: Vec3) -> Quat {
    let dir = (target - eye).normalize_or_zero();
    Quat::from_rotation_arc(-Vec3::Z, dir)
}

/// Applies pan/tilt clamps in degrees.
#[must_use]
pub fn evaluate_pan_tilt_clamped_rotation(
    pan_tilt: &PanTilt,
    mut pan_deg: f32,
    mut tilt_deg: f32,
) -> (f32, f32) {
    let (pmin, pmax) = pan_tilt.pan_range;
    if pan_tilt.pan_wrap {
        let span = pmax - pmin;
        if span.abs() > 1e-3 {
            pan_deg = ((pan_deg - pmin).rem_euclid(span)) + pmin;
        }
    } else {
        pan_deg = pan_deg.clamp(pmin, pmax);
    }
    let (tmin, tmax) = pan_tilt.tilt_clamp;
    tilt_deg = tilt_deg.clamp(tmin, tmax);
    (pan_deg, tilt_deg)
}

/// Copies the target rotation for rotate-with behaviors.
#[must_use]
pub fn evaluate_rotate_with_target(target_rotation: Quat) -> Quat {
    target_rotation
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.9.1 — rotation composer ignores targets inside the dead zone.
    #[test]
    fn tc_13_25_9_1_rotation_composer_dead_zone() {
        let composer = RotationComposer {
            target_offset: Vec3::ZERO,
            screen_position: Vec2::splat(0.5),
            dead_zone: ScreenRect {
                min: Vec2::splat(0.4),
                max: Vec2::splat(0.6),
            },
            soft_zone: ScreenRect {
                min: Vec2::splat(0.2),
                max: Vec2::splat(0.8),
            },
            damping: 0.5,
            lookahead_time: 0.0,
        };
        let delta = evaluate_rotation_composer_delta(&composer, Vec2::splat(0.5));
        assert_eq!(delta, Vec3::ZERO);
    }

    /// TC-13.25.10.1 — hard look-at keeps the target along the forward axis.
    #[test]
    fn tc_13_25_10_1_hard_look_at_centered() {
        let eye = Vec3::new(0.0, 2.0, 5.0);
        let targets = [
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(-2.0, 3.0, 0.0),
        ];
        for target in targets {
            let rot = evaluate_hard_look_at_rotation(eye, target, Vec3::Y);
            let forward = rot * -Vec3::Z;
            let to_target = (target - eye).normalize();
            let align = forward.dot(to_target);
            assert!(align > 0.999, "align={align}");
        }
    }

    /// TC-13.25.11.1 — tilt clamps beyond 90°.
    #[test]
    fn tc_13_25_11_1_pan_tilt_clamp() {
        let pan_tilt = PanTilt {
            pan_range: (-180.0, 180.0),
            pan_wrap: false,
            tilt_clamp: (-90.0, 90.0),
        };
        let (_, tilt) = evaluate_pan_tilt_clamped_rotation(&pan_tilt, 0.0, 120.0);
        assert!((tilt - 90.0).abs() < 1e-3);
    }

    /// TC-13.25.12.1 — rotate-with copies target orientation.
    #[test]
    fn tc_13_25_12_1_rotate_with_target() {
        let target_rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
        let cam_rot = evaluate_rotate_with_target(target_rot);
        assert!((cam_rot - target_rot).length() < 1e-4);
    }
}
