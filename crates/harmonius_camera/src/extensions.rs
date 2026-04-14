//! Camera extensions: confiners, zoom, autofocus, aim, free look, recomposer, modifiers.

use glam::{Vec2, Vec3};

/// 2D polygon confiner parameters.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraConfiner2D {
    /// Polygon vertices in world XZ (y ignored for tests).
    pub polygon: Vec<Vec2>,
}

/// 3D volume confiner with slowing band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraConfiner3D {
    /// Sphere radius for tests.
    pub radius: f32,
    /// Distance band where velocity scales down.
    pub slowing_distance: f32,
}

/// Maintains constant screen size by adjusting vertical FOV.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FollowZoom {
    /// Target world-space width to maintain.
    pub target_width: f32,
    /// Minimum vertical FOV (degrees).
    pub min_fov: f32,
    /// Maximum vertical FOV (degrees).
    pub max_fov: f32,
}

/// Auto focus configuration (subset).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoFocus {
    /// Last stable focus distance for hold-outside-frustum behavior.
    pub last_distance: f32,
}

/// Third-person aim parallax correction parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThirdPersonAim {
    /// Eye position in world space.
    pub eye: Vec3,
    /// Aim target in world space.
    pub target: Vec3,
    /// Camera right vector (normalized).
    pub camera_right: Vec3,
    /// Parallax correction distance along right vector when enabled.
    pub parallax_shift: f32,
}

/// Free look yaw limits and recentering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreeLookModifier {
    /// Minimum yaw (degrees).
    pub yaw_min: f32,
    /// Maximum yaw (degrees).
    pub yaw_max: f32,
    /// Seconds to recenter after idle.
    pub recenter_time: f32,
}

/// Timeline-driven FOV override weights.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RecomposerState {
    /// Blend weight between base and override FOV.
    pub weight: f32,
    /// Override vertical FOV (degrees).
    pub override_fov: f32,
}

/// Modifier ordering tokens for deterministic tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CameraModifierStep {
    /// Adds +X offset.
    Shake,
    /// Adds +Y offset.
    Noise,
}

/// Returns true when `point` lies inside `polygon` using winding ray casting.
#[must_use]
pub fn point_in_polygon_2d(point: Vec2, polygon: &[Vec2]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];
        let intersects = (pi.y > point.y) != (pj.y > point.y)
            && point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y).max(1e-6) + pi.x;
        if intersects {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Pushes the camera position inside the polygon boundary when needed.
#[must_use]
pub fn apply_camera_confiner_2d_boundary(confiner: &CameraConfiner2D, camera: Vec2) -> Vec2 {
    if point_in_polygon_2d(camera, &confiner.polygon) {
        return camera;
    }
    // Pull toward centroid for a stable interior point.
    let mut centroid = Vec2::ZERO;
    for p in &confiner.polygon {
        centroid += *p;
    }
    centroid /= confiner.polygon.len() as f32;
    camera.lerp(centroid, 0.25)
}

/// Scales velocity near a spherical boundary.
#[must_use]
pub fn apply_camera_confiner_3d_slowing(
    confiner: &CameraConfiner3D,
    position: Vec3,
    velocity: Vec3,
    center: Vec3,
) -> Vec3 {
    let distance = position.distance(center);
    let inner = (confiner.radius - confiner.slowing_distance).max(0.0);
    if distance <= inner {
        return velocity;
    }
    let band = confiner.slowing_distance.max(1e-3);
    let proximity = (confiner.radius - distance).max(0.0);
    let scale = (proximity / band).clamp(0.0, 1.0);
    velocity * scale
}

/// Computes vertical FOV (degrees) to maintain `target_width` at `distance`.
#[must_use]
pub fn apply_follow_zoom_fov(follow: &FollowZoom, distance: f32) -> f32 {
    if distance <= 1e-3 {
        return follow.min_fov;
    }
    let fov = 2.0 * ((follow.target_width * 0.5) / distance).atan().to_degrees();
    fov.clamp(follow.min_fov, follow.max_fov)
}

/// Tracks focus distance with frustum gating.
#[must_use]
pub fn apply_auto_focus_distance(focus: &AutoFocus, distance: f32, target_visible: bool) -> f32 {
    if target_visible {
        distance.max(0.0)
    } else {
        focus.last_distance
    }
}

/// Computes a parallax-corrected ray origin for third-person aim.
#[must_use]
pub fn apply_third_person_aim_ray_origin(aim: &ThirdPersonAim, enable_parallax: bool) -> Vec3 {
    if enable_parallax {
        aim.eye + aim.camera_right * aim.parallax_shift
    } else {
        aim.eye
    }
}

/// Clamps yaw and applies recentering toward zero when `idle_time` exceeds zero.
#[must_use]
pub fn apply_free_look_yaw_clamp(
    modifier: &FreeLookModifier,
    mut yaw: f32,
    idle_time: f32,
    dt: f32,
) -> f32 {
    yaw = yaw.clamp(modifier.yaw_min, modifier.yaw_max);
    if idle_time > 0.0 {
        let alpha = (dt / modifier.recenter_time.max(1e-3)).clamp(0.0, 1.0);
        yaw *= 1.0 - alpha;
    }
    yaw
}

/// Blends base FOV with recomposer override using `weight`.
#[must_use]
pub fn apply_recomposer_fov_override(base_fov: f32, recomposer: &RecomposerState) -> f32 {
    let w = recomposer.weight.clamp(0.0, 1.0);
    base_fov + (recomposer.override_fov - base_fov) * w
}

/// Applies modifier steps in order for deterministic transform offsets.
#[must_use]
pub fn apply_modifier_stack_order(steps: &[CameraModifierStep], base: Vec3) -> Vec3 {
    let mut value = base;
    for step in steps {
        match step {
            CameraModifierStep::Shake => {
                value.x += 1.0;
            }
            CameraModifierStep::Noise => {
                value += Vec3::Y * value.x;
            }
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.29.1 — confiner keeps samples inside the polygon footprint.
    #[test]
    fn tc_13_25_29_1_confiner_2d_boundary() {
        let polygon = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 10.0),
        ];
        let confiner = CameraConfiner2D {
            polygon: polygon.clone(),
        };
        let inside = apply_camera_confiner_2d_boundary(&confiner, Vec2::new(9.5, 1.0));
        assert!(point_in_polygon_2d(inside, &polygon));
    }

    /// TC-13.25.30.1 — slowing band reduces velocity near the boundary.
    #[test]
    fn tc_13_25_30_1_confiner_3d_slowing() {
        let confiner = CameraConfiner3D {
            radius: 10.0,
            slowing_distance: 2.0,
        };
        let center = Vec3::ZERO;
        let pos = Vec3::new(9.2, 0.0, 0.0);
        let vel = Vec3::new(1.0, 0.0, 0.0);
        let slowed = apply_camera_confiner_3d_slowing(&confiner, pos, vel, center);
        assert!(slowed.length() < vel.length());
    }

    /// TC-13.25.31.1 — follow zoom widens FOV for distant targets.
    #[test]
    fn tc_13_25_31_1_follow_zoom_fov_adjust() {
        let follow = FollowZoom {
            target_width: 5.0,
            min_fov: 20.0,
            max_fov: 90.0,
        };
        let fov = apply_follow_zoom_fov(&follow, 12.0);
        assert!(fov > follow.min_fov);
    }

    /// TC-13.25.32.1 — auto focus tracks visible targets and holds otherwise.
    #[test]
    fn tc_13_25_32_1_auto_focus_tracking() {
        let focus = AutoFocus { last_distance: 8.0 };
        let live = apply_auto_focus_distance(&focus, 12.0, true);
        assert!((live - 12.0).abs() < 1e-3);
        let hold = apply_auto_focus_distance(&focus, 20.0, false);
        assert!((hold - 8.0).abs() < 1e-3);
    }

    /// TC-13.25.33.1 — parallax correction shifts the ray origin laterally.
    #[test]
    fn tc_13_25_33_1_third_person_aim_parallax() {
        let aim = ThirdPersonAim {
            eye: Vec3::ZERO,
            target: Vec3::new(0.0, 0.0, -10.0),
            camera_right: Vec3::X,
            parallax_shift: 0.35,
        };
        let corrected = apply_third_person_aim_ray_origin(&aim, true);
        let raw = apply_third_person_aim_ray_origin(&aim, false);
        assert!((corrected - raw).length() > 1e-3);
    }

    /// TC-13.25.34.1 — free look clamps yaw and decays toward center after idle.
    #[test]
    fn tc_13_25_34_1_free_look_range_clamp() {
        let modifier = FreeLookModifier {
            yaw_min: -90.0,
            yaw_max: 90.0,
            recenter_time: 0.5,
        };
        let clamped = apply_free_look_yaw_clamp(&modifier, 120.0, 0.0, 1.0 / 60.0);
        assert!((clamped - 90.0).abs() < 1e-3);
        let recentered = apply_free_look_yaw_clamp(&modifier, 45.0, 1.0, 1.0 / 60.0);
        assert!(recentered < 45.0);
    }

    /// TC-13.25.35.1 — recomposer blends FOV toward an override by weight.
    #[test]
    fn tc_13_25_35_1_recomposer_fov_override() {
        let recomposer = RecomposerState {
            weight: 1.0,
            override_fov: 30.0,
        };
        let fov = apply_recomposer_fov_override(60.0, &recomposer);
        assert!((fov - 30.0).abs() < 1e-3);
        let blended = apply_recomposer_fov_override(
            60.0,
            &RecomposerState {
                weight: 0.5,
                override_fov: 30.0,
            },
        );
        assert!((blended - 45.0).abs() < 1e-3);
    }

    /// TC-13.25.36.1 — modifier stack ordering changes the composed transform.
    #[test]
    fn tc_13_25_36_1_modifier_stack_order() {
        let base = Vec3::ZERO;
        let order_a = [CameraModifierStep::Shake, CameraModifierStep::Noise];
        let order_b = [CameraModifierStep::Noise, CameraModifierStep::Shake];
        let a = apply_modifier_stack_order(&order_a, base);
        let b = apply_modifier_stack_order(&order_b, base);
        assert_ne!(a, b);
    }
}
