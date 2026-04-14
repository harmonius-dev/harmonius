//! Pure gizmo math: shape edits without touching `World`.

use crate::math::Vec3;
use crate::model::ColliderShape;

/// Increases box `half_extents.x` by `delta` (TC-IR-5.4.1.1).
#[must_use]
pub fn apply_box_half_extent_delta_x(shape: &ColliderShape, delta: f32) -> ColliderShape {
    match shape {
        ColliderShape::Box { half_extents } => ColliderShape::Box {
            half_extents: Vec3 {
                x: half_extents.x + delta,
                y: half_extents.y,
                z: half_extents.z,
            },
        },
        other => other.clone(),
    }
}

/// Increases sphere radius by `delta` (TC-IR-5.4.1.2).
#[must_use]
pub fn apply_sphere_radius_delta(shape: &ColliderShape, delta: f32) -> ColliderShape {
    match shape {
        ColliderShape::Sphere { radius } => ColliderShape::Sphere {
            radius: radius + delta,
        },
        other => other.clone(),
    }
}

/// Adjusts capsule cylinder half-height; radius unchanged (TC-IR-5.4.1.3).
#[must_use]
pub fn apply_capsule_half_height_delta(shape: &ColliderShape, delta: f32) -> ColliderShape {
    match shape {
        ColliderShape::Capsule {
            half_height,
            radius,
        } => ColliderShape::Capsule {
            half_height: half_height + delta,
            radius: *radius,
        },
        other => other.clone(),
    }
}
