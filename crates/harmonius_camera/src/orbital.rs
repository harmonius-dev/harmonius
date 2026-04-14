//! Orbital follow modes and recentering.

use glam::{Quat, Vec3};

/// Orbit surface configuration.
#[derive(Clone, Debug, PartialEq)]
pub enum OrbitMode {
    /// Single-radius sphere.
    Sphere {
        /// Radius in meters.
        radius: f32,
    },
    /// Three-ring radii for vertical blending.
    ThreeRing {
        top_radius: f32,
        middle_radius: f32,
        bottom_radius: f32,
    },
}

/// Auto-recenter configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct RecenterConfig {
    /// Seconds of idle input before recentering starts.
    pub wait_time: f32,
    /// Seconds to complete recentering.
    pub completion_time: f32,
}

/// Orbital follow parameters used by evaluation helpers.
#[derive(Clone, Debug, PartialEq)]
pub struct OrbitalFollow {
    /// Orbit mode.
    pub mode: OrbitMode,
    /// Horizontal axis range (degrees).
    pub horizontal_range: (f32, f32),
    /// Vertical axis range (degrees).
    pub vertical_range: (f32, f32),
    /// Horizontal axis wraps.
    pub horizontal_wrap: bool,
    /// Optional recentering configuration.
    pub recenter: Option<RecenterConfig>,
    /// Offset from target to orbit center.
    pub target_offset: Vec3,
}

/// Evaluates an orbital camera position for the current axes (degrees).
#[must_use]
pub fn evaluate_orbital_follow(
    follow: &OrbitalFollow,
    target: Vec3,
    yaw_deg: f32,
    pitch_deg: f32,
) -> Vec3 {
    let yaw = yaw_deg.to_radians();
    let pitch = pitch_deg.to_radians();
    let radius = match &follow.mode {
        OrbitMode::Sphere { radius } => *radius,
        OrbitMode::ThreeRing {
            top_radius,
            middle_radius,
            bottom_radius,
        } => {
            let (vmin, vmax) = follow.vertical_range;
            let t = ((pitch_deg - vmin) / (vmax - vmin).max(1e-4)).clamp(0.0, 1.0);
            if t <= 0.5 {
                top_radius + (middle_radius - top_radius) * (t / 0.5)
            } else {
                middle_radius + (bottom_radius - middle_radius) * ((t - 0.5) / 0.5)
            }
        }
    };

    let rot = Quat::from_rotation_y(yaw) * Quat::from_rotation_x(pitch);
    let dir = rot * Vec3::new(0.0, 0.0, 1.0);
    target + follow.target_offset + dir * radius
}

/// Advances recentering progress after `idle_time` exceeds `wait_time`.
#[must_use]
pub fn evaluate_orbital_recenter(
    config: &RecenterConfig,
    idle_time: f32,
    yaw_deg: f32,
    default_yaw_deg: f32,
    dt: f32,
) -> f32 {
    if idle_time < config.wait_time {
        return yaw_deg;
    }
    let alpha = (dt / config.completion_time.max(1e-4)).clamp(0.0, 1.0);
    yaw_deg + (default_yaw_deg - yaw_deg) * alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.4.1 — horizontal orbit increases angle at fixed radius.
    #[test]
    fn tc_13_25_4_1_orbital_sphere_mode() {
        let follow = OrbitalFollow {
            mode: OrbitMode::Sphere { radius: 5.0 },
            horizontal_range: (-180.0, 180.0),
            vertical_range: (-30.0, 30.0),
            horizontal_wrap: true,
            recenter: None,
            target_offset: Vec3::ZERO,
        };
        let target = Vec3::ZERO;
        let p0 = evaluate_orbital_follow(&follow, target, 0.0, 0.0);
        let p1 = evaluate_orbital_follow(&follow, target, 360.0 / 60.0, 0.0);
        assert!((p0.distance(target) - 5.0).abs() < 1e-3);
        assert!((p1.distance(target) - 5.0).abs() < 1e-3);
        assert!((p0 - p1).length() > 0.01);
    }

    /// TC-13.25.4.2 — three-ring mode interpolates radius with pitch.
    #[test]
    fn tc_13_25_4_2_orbital_three_ring() {
        let follow = OrbitalFollow {
            mode: OrbitMode::ThreeRing {
                top_radius: 3.0,
                middle_radius: 5.0,
                bottom_radius: 7.0,
            },
            horizontal_range: (-180.0, 180.0),
            vertical_range: (-45.0, 45.0),
            horizontal_wrap: true,
            recenter: None,
            target_offset: Vec3::ZERO,
        };
        let target = Vec3::ZERO;
        let top = evaluate_orbital_follow(&follow, target, 0.0, -45.0);
        let mid = evaluate_orbital_follow(&follow, target, 0.0, 0.0);
        let bottom = evaluate_orbital_follow(&follow, target, 0.0, 45.0);
        assert!((top.distance(target) - 3.0).abs() < 1e-2);
        assert!((mid.distance(target) - 5.0).abs() < 1e-2);
        assert!((bottom.distance(target) - 7.0).abs() < 1e-2);
    }

    /// TC-13.25.4.3 — recentering begins after idle wait and moves toward default.
    #[test]
    fn tc_13_25_4_3_orbital_recentering() {
        let config = RecenterConfig {
            wait_time: 1.0,
            completion_time: 1.0,
        };
        let yaw_before = 45.0;
        let yaw_idle = evaluate_orbital_recenter(&config, 0.5, yaw_before, 0.0, 1.0 / 60.0);
        assert!((yaw_idle - yaw_before).abs() < 1e-3);
        let yaw_after = evaluate_orbital_recenter(&config, 1.5, yaw_before, 0.0, 1.0 / 60.0);
        assert!(yaw_after < yaw_before);
    }
}
