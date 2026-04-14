//! World-space ray pick against CPU fixtures and CH-27 bounded queue.

use std::collections::VecDeque;

use crate::fallback::FallbackMetrics;
use crate::types::{Entity, Vec2, Vec3, WorldPickRequest, WorldPickResult};

/// Maximum queued pick requests (CH-27 capacity).
pub const UI_PICK_CHANNEL_CAP: usize = 8;

/// Normalized ray in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    /// Ray origin.
    pub origin: Vec3,
    /// Unit direction.
    pub dir: Vec3,
}

/// Axis-aligned pick proxy with a collision layer bit.
#[derive(Clone, Debug, PartialEq)]
pub struct PickBody {
    /// Owning entity returned on hit.
    pub entity: Entity,
    /// Minimum corner.
    pub min: Vec3,
    /// Maximum corner.
    pub max: Vec3,
    /// Layer bitmask for this body.
    pub collision_layer: u32,
}

/// In-memory physics pick scene (physics-private BVH stand-in).
#[derive(Clone, Debug, Default)]
pub struct PhysicsPickScene {
    /// When false, every pick returns `entity=None` (FM-2).
    pub bvh_ready: bool,
    /// Pick proxies (linear scan for CI fixtures).
    pub bodies: Vec<PickBody>,
}

/// Camera basis used to build a pick ray from cursor NDC.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PickCameraRig {
    /// Eye position.
    pub origin: Vec3,
    /// Normalized forward (−Z view axis in camera space mapped to world).
    pub forward: Vec3,
    /// Normalized camera right.
    pub right: Vec3,
    /// Normalized camera up.
    pub up: Vec3,
}

impl PickCameraRig {
    /// Standard test rig: eye at origin, looks down world −Z, Y up, X right.
    pub fn standard_origin() -> Self {
        Self {
            origin: Vec3::ZERO,
            forward: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            right: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        }
    }

    /// Builds a world pick ray from NDC using a symmetric perspective frustum.
    pub fn ray_from_ndc(&self, ndc: Vec2, tan_half_fov_y: f32, aspect: f32) -> Ray {
        let w = self.right * (ndc.x * tan_half_fov_y * aspect)
            + self.up * (ndc.y * tan_half_fov_y)
            + self.forward;
        Ray {
            origin: self.origin,
            dir: w.normalize(),
        }
    }
}

/// Bounded MPSC-style queue for `WorldPickRequest` (CH-27, DropOldest).
#[derive(Clone, Debug, Default)]
pub struct UiPickChannel {
    q: VecDeque<WorldPickRequest>,
}

impl UiPickChannel {
    /// Creates an empty channel.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueues a request, dropping the oldest entries when full.
    pub fn push(&mut self, req: WorldPickRequest, metrics: &mut FallbackMetrics) {
        while self.q.len() >= UI_PICK_CHANNEL_CAP {
            self.q.pop_front();
            metrics.fm4_ch27_drop = metrics.fm4_ch27_drop.saturating_add(1);
        }
        self.q.push_back(req);
    }

    /// Removes all pending requests (newest-first drain is not required; FIFO for tests).
    pub fn drain_fifo(&mut self) -> Vec<WorldPickRequest> {
        self.q.drain(..).collect()
    }
}

fn ndc_in_window(ndc: Vec2) -> bool {
    ndc.x >= -1.0 && ndc.x <= 1.0 && ndc.y >= -1.0 && ndc.y <= 1.0
}

fn ray_aabb(origin: Vec3, dir: Vec3, min: Vec3, max: Vec3, t_clip: f32) -> Option<f32> {
    let mut t0 = 0.0_f32;
    let mut t1 = t_clip;
    let o = [origin.x, origin.y, origin.z];
    let d = [dir.x, dir.y, dir.z];
    let mn = [min.x, min.y, min.z];
    let mx = [max.x, max.y, max.z];
    for i in 0..3 {
        if d[i].abs() < 1e-8 {
            if o[i] < mn[i] || o[i] > mx[i] {
                return None;
            }
            continue;
        }
        let inv = 1.0 / d[i];
        let mut ta = (mn[i] - o[i]) * inv;
        let mut tb = (mx[i] - o[i]) * inv;
        if ta > tb {
            std::mem::swap(&mut ta, &mut tb);
        }
        t0 = t0.max(ta);
        t1 = t1.min(tb);
        if t0 > t1 {
            return None;
        }
    }
    if t0.is_finite() && t0 <= t1 {
        Some(t0)
    } else {
        None
    }
}

fn hit_normal(origin: Vec3, dir: Vec3, t: f32, min: Vec3, max: Vec3) -> Vec3 {
    let p = origin + dir * t;
    let eps = 1e-3;
    if (p.x - min.x).abs() < eps {
        return Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if (p.x - max.x).abs() < eps {
        return Vec3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if (p.y - min.y).abs() < eps {
        return Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
    }
    if (p.y - max.y).abs() < eps {
        return Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };
    }
    if (p.z - min.z).abs() < eps {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
    }
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    }
}

/// Resolves one pick request against `scene`.
pub fn resolve_world_pick(
    scene: &PhysicsPickScene,
    rig: &PickCameraRig,
    req: WorldPickRequest,
    tan_half_fov_y: f32,
    aspect: f32,
    metrics: &mut FallbackMetrics,
) -> WorldPickResult {
    if !ndc_in_window(req.cursor_ndc) {
        metrics.fm1_cursor_outside = metrics.fm1_cursor_outside.saturating_add(1);
        return WorldPickResult {
            request_id: req.request_id,
            entity: None,
            world_position: Vec3::ZERO,
            distance: 0.0,
            normal: Vec3::ZERO,
        };
    }
    if !scene.bvh_ready {
        metrics.fm2_bvh_not_built = metrics.fm2_bvh_not_built.saturating_add(1);
        return WorldPickResult {
            request_id: req.request_id,
            entity: None,
            world_position: Vec3::ZERO,
            distance: 0.0,
            normal: Vec3::ZERO,
        };
    }
    let ray = rig.ray_from_ndc(req.cursor_ndc, tan_half_fov_y, aspect);
    let mut best_t = req.max_distance;
    let mut best: Option<(Entity, Vec3, Vec3)> = None;
    for body in &scene.bodies {
        if body.collision_layer & req.ray_mask.0 == 0 {
            continue;
        }
        if let Some(t) = ray_aabb(ray.origin, ray.dir, body.min, body.max, req.max_distance) {
            if t < best_t {
                best_t = t;
                let p = ray.origin + ray.dir * t;
                let n = hit_normal(ray.origin, ray.dir, t, body.min, body.max);
                best = Some((body.entity, p, n));
            }
        }
    }
    if let Some((e, p, n)) = best {
        WorldPickResult {
            request_id: req.request_id,
            entity: Some(e),
            world_position: p,
            distance: best_t,
            normal: n,
        }
    } else {
        WorldPickResult {
            request_id: req.request_id,
            entity: None,
            world_position: Vec3::ZERO,
            distance: 0.0,
            normal: Vec3::ZERO,
        }
    }
}

/// Drains CH-27 and resolves each request in FIFO order.
pub fn resolve_pick_through_channel(
    channel: &mut UiPickChannel,
    scene: &PhysicsPickScene,
    rig: &PickCameraRig,
    tan_half_fov_y: f32,
    aspect: f32,
    metrics: &mut FallbackMetrics,
) -> Vec<WorldPickResult> {
    let pending = channel.drain_fifo();
    pending
        .into_iter()
        .map(|r| resolve_world_pick(scene, rig, r, tan_half_fov_y, aspect, metrics))
        .collect()
}
