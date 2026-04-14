//! Feeler-ray obstacle avoidance against simple static geometry.

use glam::{Vec2, Vec3};

/// Result of casting a ray against static geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayCastHit {
    /// Distance along the ray to the hit point.
    pub distance: f32,
    /// World-space surface normal at the hit.
    pub normal: Vec3,
}

/// A ray in world space for static obstacle queries.
#[derive(Clone, Copy, Debug)]
pub struct Ray3 {
    /// Ray origin.
    pub origin: Vec3,
    /// Unit direction.
    pub direction: Vec3,
}

/// Minimal spatial query surface for steering tests (static walls).
pub trait SpatialQuery {
    /// Cast `ray` up to `max_distance` against static obstacles.
    fn ray_cast(&self, ray: &Ray3, max_distance: f32) -> Option<RayCastHit>;
}

/// Axis-aligned wall segment in XZ (y-up world); infinite height slab.
#[derive(Clone, Copy, Debug)]
pub struct WallSegment {
    /// Wall start in XZ mapped to `Vec3` with `y = 0`.
    pub a: Vec3,
    /// Wall end in XZ.
    pub b: Vec3,
    /// Half thickness used for penetration tests.
    pub thickness: f32,
}

/// Static walls for unit tests and simple scenes.
#[derive(Clone, Debug, Default)]
pub struct StaticWalls {
    /// Wall segments in order.
    pub walls: Vec<WallSegment>,
}

impl SpatialQuery for StaticWalls {
    fn ray_cast(&self, ray: &Ray3, max_distance: f32) -> Option<RayCastHit> {
        let mut best: Option<RayCastHit> = None;
        for w in &self.walls {
            let p0 = Vec2::new(w.a.x, w.a.z);
            let p1 = Vec2::new(w.b.x, w.b.z);
            let ro = Vec2::new(ray.origin.x, ray.origin.z);
            let rd = Vec2::new(ray.direction.x, ray.direction.z);
            let rd_len = rd.length();
            if rd_len < f32::EPSILON {
                continue;
            }
            let rd_u = rd / rd_len;
            let seg = p1 - p0;
            let seg_len = seg.length();
            if seg_len < f32::EPSILON {
                continue;
            }
            let seg_u = seg / seg_len;
            let n2 = Vec2::new(-seg_u.y, seg_u.x);
            let denom = rd_u.perp_dot(seg_u);
            if denom.abs() < 1e-6 {
                continue;
            }
            let wv = p0 - ro;
            let t = wv.perp_dot(seg_u) / denom;
            let u = wv.perp_dot(rd_u) / denom;
            if t >= 0.0 && t <= max_distance && u >= 0.0 && u <= seg_len {
                let hit = RayCastHit {
                    distance: t,
                    normal: Vec3::new(n2.x, 0.0, n2.y).normalize_or_zero(),
                };
                best = Some(match best {
                    None => hit,
                    Some(prev) if hit.distance < prev.distance => hit,
                    Some(prev) => prev,
                });
            }
        }
        best
    }
}

/// Distribute `feeler_count` yaw offsets in a forward cone (radians).
pub fn distribute_feeler_angles(count: u8) -> smallvec::SmallVec<[f32; 8]> {
    use smallvec::smallvec;
    use std::f32::consts::FRAC_PI_4;
    let half_spread = FRAC_PI_4;
    if count <= 1 {
        return smallvec![0.0];
    }
    let step = (2.0 * half_spread) / (count - 1) as f32;
    let mut out = smallvec![];
    for i in 0..count {
        out.push(-half_spread + step * i as f32);
    }
    out
}

fn rotate_y(dir: Vec3, yaw: f32) -> Vec3 {
    let c = yaw.cos();
    let s = yaw.sin();
    Vec3::new(dir.x * c - dir.z * s, dir.y, dir.x * s + dir.z * c)
}

/// Feeler-ray obstacle avoidance against `spatial` static geometry.
pub fn compute_obstacle_avoidance(
    position: Vec3,
    heading: Vec3,
    feeler_count: u8,
    feeler_length: f32,
    spatial: &impl SpatialQuery,
) -> Vec3 {
    let mut force = Vec3::ZERO;
    let angles = distribute_feeler_angles(feeler_count);
    let h = heading.normalize_or_zero();
    for angle in angles.iter().copied() {
        let dir = rotate_y(h, angle).normalize_or_zero();
        let ray = Ray3 {
            origin: position,
            direction: dir,
        };
        if let Some(hit) = spatial.ray_cast(&ray, feeler_length) {
            let overshoot = feeler_length - hit.distance;
            if overshoot > 0.0 {
                force += hit.normal * (overshoot / feeler_length);
            }
        }
    }
    force
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dist_point_segment_xz(p: Vec3, a: Vec3, b: Vec3) -> f32 {
        let p2 = Vec2::new(p.x, p.z);
        let a2 = Vec2::new(a.x, a.z);
        let b2 = Vec2::new(b.x, b.z);
        let ab = b2 - a2;
        let t = ((p2 - a2).dot(ab) / ab.length_squared().max(f32::EPSILON)).clamp(0.0, 1.0);
        (p2 - (a2 + ab * t)).length()
    }

    #[test]
    fn tc_7_2_2_1_obstacle_no_clip() {
        let walls = StaticWalls {
            walls: vec![
                WallSegment {
                    a: Vec3::new(6.0, 0.0, -2.0),
                    b: Vec3::new(6.0, 0.0, 2.0),
                    thickness: 0.2,
                },
                WallSegment {
                    a: Vec3::new(2.0, 0.0, 6.0),
                    b: Vec3::new(8.0, 0.0, 6.0),
                    thickness: 0.2,
                },
                WallSegment {
                    a: Vec3::new(-4.0, 0.0, -1.0),
                    b: Vec3::new(-4.0, 0.0, 3.0),
                    thickness: 0.2,
                },
            ],
        };
        let mut pos = Vec3::new(0.0, 0.0, 0.0);
        let target = Vec3::new(12.0, 0.0, 0.0);
        let mut vel = Vec3::ZERO;
        let wall_t = 0.12_f32;
        for _ in 0..500 {
            let to_t = (target - pos).normalize_or_zero();
            let heading = (vel * 0.5 + to_t).normalize_or_zero();
            let avoid = compute_obstacle_avoidance(pos, heading, 5, 5.5, &walls);
            let steer = to_t * 1.2 + avoid * 4.5;
            vel = (vel + steer * 0.06).clamp_length_max(2.5);
            pos += vel * 0.06;
            for w in &walls.walls {
                let d = dist_point_segment_xz(pos, w.a, w.b);
                assert!(
                    d >= wall_t,
                    "too close to wall segment: pos {pos:?} dist {d}"
                );
            }
        }
    }
}
