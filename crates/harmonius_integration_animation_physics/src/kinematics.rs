//! Velocity clamps and swing-cone limits from the integration failure-mode table.

use glam::{Quat, Vec3};

/// Linear and angular velocity pair for one rigid body.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearAngular {
    /// Linear velocity (m/s).
    pub linear: Vec3,
    /// Angular velocity (rad/s).
    pub angular: Vec3,
}

/// Clamps linear magnitude to `max_linear` and angular magnitude to `max_angular` (per bone).
#[must_use]
pub fn clamp_linear_angular_velocity(
    v: LinearAngular,
    max_linear: f32,
    max_angular: f32,
) -> LinearAngular {
    LinearAngular {
        linear: clamp_vec3_length(v.linear, max_linear),
        angular: clamp_vec3_length(v.angular, max_angular),
    }
}

fn clamp_vec3_length(v: Vec3, max_len: f32) -> Vec3 {
    let len = v.length();
    if len <= max_len || len < 1e-12 {
        return v;
    }
    v * (max_len / len)
}

/// Returns a unit direction whose swing angle from `twist_axis` is at most `swing_limit_rad`.
///
/// `twist_axis` should be a unit vector; non-unit values are normalized.
#[must_use]
pub fn clamp_to_swing_cone(mut direction: Vec3, twist_axis: Vec3, swing_limit_rad: f32) -> Vec3 {
    let axis = twist_axis.normalize_or_zero();
    if axis.length_squared() < 1e-12 {
        return direction.normalize_or_zero();
    }
    direction = direction.normalize_or_zero();
    if direction.length_squared() < 1e-12 {
        return axis;
    }
    let dot = direction.dot(axis).clamp(-1.0, 1.0);
    let angle = dot.acos();
    if angle <= swing_limit_rad {
        return direction;
    }
    let perp = (direction - axis * dot).normalize_or_zero();
    if perp.length_squared() < 1e-12 {
        return axis;
    }
    (axis * swing_limit_rad.cos() + perp * swing_limit_rad.sin()).normalize_or_zero()
}

/// Normalized quaternion after swing clamp using `clamp_to_swing_cone` on the rotated basis vector.
#[must_use]
pub fn clamp_orientation_swing(
    rotation: Quat,
    local_forward: Vec3,
    parent_twist_axis: Vec3,
    swing_limit_rad: f32,
) -> Quat {
    let world_forward = rotation * local_forward;
    let clamped = clamp_to_swing_cone(world_forward, parent_twist_axis, swing_limit_rad);
    Quat::from_rotation_arc(world_forward, clamped) * rotation
}
