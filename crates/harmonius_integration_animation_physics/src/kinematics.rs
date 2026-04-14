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

/// Twist quaternion about `twist_axis` using the vector projection of the imaginary part.
fn twist_swing_decompose(q: Quat, twist_axis: Vec3) -> (Quat, Quat) {
    let q = q.normalize();
    let a = twist_axis.normalize_or_zero();
    if a.length_squared() < 1e-12 {
        return (Quat::IDENTITY, q);
    }
    let p = Vec3::new(q.x, q.y, q.z);
    let twist_v = a * a.dot(p);
    let twist_len = (twist_v.length_squared() + q.w * q.w).sqrt();
    if twist_len < 1e-12 {
        return (Quat::IDENTITY, q);
    }
    let mut twist = Quat::from_xyzw(
        twist_v.x / twist_len,
        twist_v.y / twist_len,
        twist_v.z / twist_len,
        q.w / twist_len,
    );
    if twist.w < 0.0 {
        twist = Quat::from_xyzw(-twist.x, -twist.y, -twist.z, -twist.w);
    }
    let swing = (q * twist.conjugate()).normalize();
    (swing, twist)
}

/// Clamps twist about `twist_axis` on `relative_rotation` (child in parent space) to ±limit.
#[must_use]
pub fn clamp_joint_twist(
    relative_rotation: Quat,
    twist_axis: Vec3,
    twist_limit_rad: f32,
) -> Quat {
    let a = twist_axis.normalize_or_zero();
    if a.length_squared() < 1e-12 {
        return relative_rotation;
    }
    let (swing, twist) = twist_swing_decompose(relative_rotation, twist_axis);
    let (t_axis, t_angle) = twist.to_axis_angle();
    let sign = if t_axis.dot(a) < 0.0 { -1.0 } else { 1.0 };
    let clamped = (t_angle * sign).clamp(-twist_limit_rad, twist_limit_rad);
    let twist_c = Quat::from_axis_angle(a, clamped);
    (swing * twist_c).normalize()
}
