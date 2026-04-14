//! Third-person follow helpers (shoulder blend and collision retraction).

use glam::Vec3;

use crate::ids::LayerMask;

/// Third-person follow configuration (subset used by tests).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThirdPersonFollow {
    /// Lateral shoulder offset.
    pub shoulder_offset: Vec3,
    /// Vertical arm length.
    pub vertical_arm_length: f32,
    /// `0.0` = left shoulder, `1.0` = right shoulder.
    pub camera_side: f32,
    /// Minimum distance from obstacles.
    pub camera_radius: f32,
    /// Damping when moving into collision.
    pub collision_damping: f32,
    /// Damping when recovering from collision.
    pub recovery_damping: f32,
    /// Collision layer mask.
    pub collision_layers: LayerMask,
}

/// Interpolates shoulder offset between left and right based on `camera_side`.
#[must_use]
pub fn evaluate_third_person_shoulder(follow: &ThirdPersonFollow) -> Vec3 {
    let left = follow.shoulder_offset;
    let right = Vec3::new(
        -follow.shoulder_offset.x,
        follow.shoulder_offset.y,
        follow.shoulder_offset.z,
    );
    left.lerp(right, follow.camera_side.clamp(0.0, 1.0))
}

/// Retracts the camera along the ray from `target` to `desired` when an obstacle is present.
#[must_use]
pub fn evaluate_third_person_collision_retraction(
    follow: &ThirdPersonFollow,
    target: Vec3,
    desired: Vec3,
    obstacle_distance: Option<f32>,
) -> Vec3 {
    let dir = (desired - target).normalize_or_zero();
    if dir.length_squared() <= 1e-6 {
        return desired;
    }
    let arm = follow.vertical_arm_length.max(1e-3);
    match obstacle_distance {
        None => desired,
        Some(hit) => {
            let max_len = (hit - follow.camera_radius).max(0.0);
            let len = arm.min(max_len);
            target + dir * len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.5.1 — shoulder blend moves laterally with `camera_side`.
    #[test]
    fn tc_13_25_5_1_third_person_shoulder_blend() {
        let follow = ThirdPersonFollow {
            shoulder_offset: Vec3::new(0.5, 0.0, 0.0),
            vertical_arm_length: 4.0,
            camera_side: 0.0,
            camera_radius: 0.2,
            collision_damping: 0.1,
            recovery_damping: 0.2,
            collision_layers: LayerMask(1),
        };
        let left = evaluate_third_person_shoulder(&follow);
        let mut follow_right = follow;
        follow_right.camera_side = 1.0;
        let right = evaluate_third_person_shoulder(&follow_right);
        assert!((left - right).length() > 0.1);
    }

    /// TC-13.25.5.2 — collision shortens the arm to the reported hit distance.
    #[test]
    fn tc_13_25_5_2_third_person_collision() {
        let follow = ThirdPersonFollow {
            shoulder_offset: Vec3::ZERO,
            vertical_arm_length: 5.0,
            camera_side: 0.0,
            camera_radius: 0.0,
            collision_damping: 0.1,
            recovery_damping: 0.2,
            collision_layers: LayerMask(1),
        };
        let target = Vec3::ZERO;
        let desired = Vec3::new(0.0, 0.0, 5.0);
        let retracted =
            evaluate_third_person_collision_retraction(&follow, target, desired, Some(3.0));
        assert!((retracted - Vec3::new(0.0, 0.0, 3.0)).length() < 1e-3);
    }
}
