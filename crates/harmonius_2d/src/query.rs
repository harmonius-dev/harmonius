//! Raycasts and overlap queries in 2D.

use glam::Vec2;

/// Hit record for a ray vs collider test.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayHit2d {
    /// Hit distance along the ray direction (scaled by ray length unit).
    pub t: f32,
    /// World-space contact point.
    pub point: Vec2,
    /// Outward-pointing normal from the hit surface.
    pub normal: Vec2,
    /// User-provided entity id for the hit primitive.
    pub entity: u32,
}

/// Ray vs axis-aligned box. `max_t` uses the same length scale as `dir` (not required normalized).
#[must_use]
pub fn raycast_aabb(
    origin: Vec2,
    dir: Vec2,
    max_t: f32,
    aabb_min: Vec2,
    aabb_max: Vec2,
    entity: u32,
) -> Option<RayHit2d> {
    let mut t_min = 0.0_f32;
    let mut t_max = max_t;

    for axis in 0..2 {
        let o = if axis == 0 { origin.x } else { origin.y };
        let d = if axis == 0 { dir.x } else { dir.y };
        let bn = if axis == 0 { aabb_min.x } else { aabb_min.y };
        let bx = if axis == 0 { aabb_max.x } else { aabb_max.y };

        if d.abs() < 1e-8 {
            if o < bn || o > bx {
                return None;
            }
        } else {
            let inv = 1.0 / d;
            let mut t1 = (bn - o) * inv;
            let mut t2 = (bx - o) * inv;
            if t1 > t2 {
                core::mem::swap(&mut t1, &mut t2);
            }
            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        }
    }

    let t_hit = if t_min >= 0.0 { t_min } else { t_max };
    if t_hit < 0.0 || t_hit > max_t {
        return None;
    }

    let point = origin + dir * t_hit;
    let eps = 1e-4;
    let mut normal = Vec2::ZERO;
    for axis in 0..2 {
        let o = if axis == 0 { origin.x } else { origin.y };
        let d = if axis == 0 { dir.x } else { dir.y };
        let bn = if axis == 0 { aabb_min.x } else { aabb_min.y };
        let bx = if axis == 0 { aabb_max.x } else { aabb_max.y };
        if d.abs() < 1e-8 {
            continue;
        }
        let inv = 1.0 / d;
        let t1 = (bn - o) * inv;
        let t2 = (bx - o) * inv;
        let te = t1.min(t2);
        if (te - t_hit).abs() < eps {
            if axis == 0 {
                normal.x = if te == t1 { -1.0 } else { 1.0 };
            } else {
                normal.y = if te == t1 { -1.0 } else { 1.0 };
            }
        }
    }
    if normal.length_squared() < 1e-8 {
        normal = Vec2::new(-1.0, 0.0);
    } else {
        normal = normal.normalize();
    }

    Some(RayHit2d {
        t: t_hit,
        point,
        normal,
        entity,
    })
}

/// Return entity ids whose positions fall inside the circle (`TC-10.5.13.2`).
#[must_use]
pub fn circle_overlap_entities(positions: &[(u32, Vec2)], center: Vec2, radius: f32) -> Vec<u32> {
    let r2 = radius * radius;
    let mut out: Vec<u32> = positions
        .iter()
        .filter(|(_, p)| (*p - center).length_squared() <= r2)
        .map(|(id, _)| *id)
        .collect();
    out.sort_unstable();
    out
}
