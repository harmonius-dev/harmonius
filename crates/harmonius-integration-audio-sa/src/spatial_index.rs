//! Shared spatial index used for occlusion rays (axis-aligned surfaces).

use glam::Vec3;

use crate::entity::Entity;

/// Axis-aligned box representing an occluder surface in the shared index.
#[derive(Debug, Clone, Copy)]
pub struct AxisAlignedSurface {
    /// Owning entity for material lookup.
    pub entity: Entity,
    /// Minimum corner.
    pub min: Vec3,
    /// Maximum corner.
    pub max: Vec3,
}

/// Closest surface hit along a ray segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceHit {
    /// Entity owning the struck surface.
    pub entity: Entity,
    /// Parametric distance along the ray from `origin`.
    pub distance: f32,
    /// Outward-facing hit normal (axis-aligned).
    pub normal: Vec3,
}

/// Shared spatial index for occlusion rays: linear sweep over axis-aligned surfaces.
///
/// This is a deterministic placeholder until a BVH-backed shared index is wired from the runtime.
#[derive(Debug, Clone)]
pub struct SharedSpatialIndex {
    surfaces: Vec<AxisAlignedSurface>,
    ready: bool,
}

impl SharedSpatialIndex {
    /// Creates an index in the requested readiness state.
    #[must_use]
    pub fn new(surfaces: Vec<AxisAlignedSurface>, ready: bool) -> Self {
        Self { surfaces, ready }
    }

    /// Returns whether the index is safe for tracing.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.ready
    }

    /// Marks the index ready (for example after BVH build completes).
    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    /// Returns the closest hit along \[`origin`, `origin + dir * max_t`\] with `dir` unit length.
    #[must_use]
    pub fn raycast_segment(&self, origin: Vec3, dir: Vec3, max_t: f32) -> Option<SurfaceHit> {
        if !self.ready {
            return None;
        }
        let inv_dir = Vec3::new(
            inv_dir_component(dir.x),
            inv_dir_component(dir.y),
            inv_dir_component(dir.z),
        );
        let mut best: Option<SurfaceHit> = None;
        for surface in &self.surfaces {
            if let Some(t_hit) = ray_aabb(origin, inv_dir, surface.min, surface.max, max_t) {
                let better = match best {
                    None => true,
                    Some(prev) => t_hit < prev.distance,
                };
                if better {
                    best = Some(SurfaceHit {
                        entity: surface.entity,
                        distance: t_hit,
                        normal: aabb_exit_normal(surface.min, surface.max, origin, dir, t_hit),
                    });
                }
            }
        }
        best
    }
}

fn inv_dir_component(d: f32) -> f32 {
    if d.abs() < 1e-8 {
        1.0e8 * d.signum()
    } else {
        1.0 / d
    }
}

fn ray_aabb(origin: Vec3, inv_dir: Vec3, bmin: Vec3, bmax: Vec3, max_ray_t: f32) -> Option<f32> {
    let mut tmin = 0.0_f32;
    let mut tmax = max_ray_t;
    for axis in 0..3 {
        let o = origin[axis];
        let invd = inv_dir[axis];
        let mut t0 = (bmin[axis] - o) * invd;
        let mut t1 = (bmax[axis] - o) * invd;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        tmin = tmin.max(t0);
        tmax = tmax.min(t1);
        if tmin > tmax {
            return None;
        }
    }
    if tmax < 0.0 {
        return None;
    }
    let t_hit = if tmin >= 0.0 { tmin } else { tmax };
    if !(t_hit >= 0.0 && t_hit <= max_ray_t) {
        return None;
    }
    Some(t_hit)
}

fn aabb_exit_normal(bmin: Vec3, bmax: Vec3, origin: Vec3, dir: Vec3, t_hit: f32) -> Vec3 {
    let p = origin + dir * t_hit;
    let eps = 1e-3;
    if (p.x - bmin.x).abs() < eps {
        return Vec3::new(-1.0, 0.0, 0.0);
    }
    if (p.x - bmax.x).abs() < eps {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    if (p.y - bmin.y).abs() < eps {
        return Vec3::new(0.0, -1.0, 0.0);
    }
    if (p.y - bmax.y).abs() < eps {
        return Vec3::new(0.0, 1.0, 0.0);
    }
    if (p.z - bmin.z).abs() < eps {
        return Vec3::new(0.0, 0.0, -1.0);
    }
    Vec3::new(0.0, 0.0, 1.0)
}
