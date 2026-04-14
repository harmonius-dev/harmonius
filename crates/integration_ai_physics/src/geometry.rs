//! Minimal analytic geometry for deterministic tests (rays, planes, axis-aligned boxes).

use glam::Vec3;

/// Axis-aligned bounding box used by jump-arc and LOS tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisAlignedBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl AxisAlignedBox {
    /// Returns a new box with component-wise min/max ordering corrected.
    #[must_use]
    pub fn new(mut min: Vec3, mut max: Vec3) -> Self {
        let a = min.min(max);
        let b = min.max(max);
        min = a;
        max = b;
        Self { min, max }
    }

    /// Ray segment `origin + delta * t` for `t in [0,1]` against this AABB (slab method).
    #[must_use]
    pub fn ray_segment_hit_t(&self, origin: Vec3, delta: Vec3) -> Option<f32> {
        let inv = Vec3::new(safe_inv(delta.x), safe_inv(delta.y), safe_inv(delta.z));
        let t1 = (self.min - origin) * inv;
        let t2 = (self.max - origin) * inv;
        let tmin = t1.min(t2);
        let tmax = t1.max(t2);
        let t_enter = tmin.max_element();
        let t_exit = tmax.min_element();
        if t_enter > t_exit || t_exit < 0.0 {
            return None;
        }
        let t_hit = if t_enter >= 0.0 { t_enter } else { t_exit };
        if t_hit > 1.0 {
            return None;
        }
        Some(t_hit)
    }
}

fn safe_inv(v: f32) -> f32 {
    if v.abs() < f32::EPSILON {
        1.0 / f32::EPSILON.copysign(v.signum()).max(f32::EPSILON)
    } else {
        1.0 / v
    }
}

/// Infinite ground plane `dot(point, normal) = d` with outward/upward `normal`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    /// Builds a plane from a unit normal and a point on the plane.
    #[must_use]
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        let n = normal.normalize_or_zero();
        let d = point.dot(n);
        Self { normal: n, d }
    }

    /// Ray `origin + dir * t` for `t in [0, max_t]` intersection, returns smallest `t >= 0`.
    #[must_use]
    pub fn ray_hit_t(&self, origin: Vec3, dir: Vec3, max_t: f32) -> Option<f32> {
        let denom = dir.dot(self.normal);
        if denom.abs() < f32::EPSILON {
            return None;
        }
        let t = (self.d - origin.dot(self.normal)) / denom;
        if t < 0.0 || t > max_t {
            return None;
        }
        Some(t)
    }
}
