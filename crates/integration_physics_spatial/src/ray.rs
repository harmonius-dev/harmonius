//! Ray queries against axis-aligned bounds.

use crate::aabb::Aabb;
use crate::math::Vec3;

/// Ray used for broadphase / narrowphase queries.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayQuery {
    /// Ray origin.
    pub origin: Vec3,
    /// Ray direction (need not be normalized).
    pub dir: Vec3,
    /// Maximum ray parameter along `dir`.
    pub max_t: f32,
}

/// Conservative hit record for a ray test.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayHit {
    /// Hit distance along the ray direction.
    pub t: f32,
}

/// Returns the nearest positive hit distance for a ray against an AABB, if any.
#[must_use]
pub fn intersect_ray_aabb(ray: &RayQuery, bounds: &Aabb) -> Option<f32> {
    let inv_x = if ray.dir.x == 0.0 {
        f32::INFINITY
    } else {
        1.0 / ray.dir.x
    };
    let inv_y = if ray.dir.y == 0.0 {
        f32::INFINITY
    } else {
        1.0 / ray.dir.y
    };
    let inv_z = if ray.dir.z == 0.0 {
        f32::INFINITY
    } else {
        1.0 / ray.dir.z
    };

    let mut t_min = 0.0_f32;
    let mut t_max = ray.max_t;

    let ox = ray.origin.x;
    let oy = ray.origin.y;
    let oz = ray.origin.z;

    let tx1 = (bounds.min.x - ox) * inv_x;
    let tx2 = (bounds.max.x - ox) * inv_x;
    let t_near = tx1.min(tx2);
    let t_far = tx1.max(tx2);
    t_min = t_min.max(t_near);
    t_max = t_max.min(t_far);
    if t_min > t_max {
        return None;
    }

    let ty1 = (bounds.min.y - oy) * inv_y;
    let ty2 = (bounds.max.y - oy) * inv_y;
    let t_near = ty1.min(ty2);
    let t_far = ty1.max(ty2);
    t_min = t_min.max(t_near);
    t_max = t_max.min(t_far);
    if t_min > t_max {
        return None;
    }

    let tz1 = (bounds.min.z - oz) * inv_z;
    let tz2 = (bounds.max.z - oz) * inv_z;
    let t_near = tz1.min(tz2);
    let t_far = tz1.max(tz2);
    t_min = t_min.max(t_near);
    t_max = t_max.min(t_far);
    if t_min > t_max {
        return None;
    }

    if t_max < 0.0 {
        return None;
    }

    let t_hit = if t_min >= 0.0 { t_min } else { t_max };
    if t_hit > ray.max_t {
        return None;
    }

    Some(t_hit)
}
