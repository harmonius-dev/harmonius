//! Reticle snap scan within an aim cone.

use crate::types::{Entity, ReticleSnap, Vec2, Vec3};

/// Aimable entity candidate for reticle snapping.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AimableTarget {
    /// Entity id.
    pub entity: Entity,
    /// World-space position used for cone and distance tests.
    pub world_position: Vec3,
}

/// Picks the nearest aimable target inside the cone around `ray_dir`.
pub fn compute_reticle_snap(
    ray_origin: Vec3,
    ray_dir: Vec3,
    targets: &[AimableTarget],
    max_range: f32,
    min_cone_cos: f32,
) -> ReticleSnap {
    let mut best_t = max_range;
    let mut best: Option<Entity> = None;
    for t in targets {
        let v = t.world_position - ray_origin;
        let dist = v.dot(ray_dir);
        if dist <= 0.0 || dist > max_range {
            continue;
        }
        let to = {
            let len = v.dot(v).sqrt();
            if len <= 1e-8 {
                continue;
            }
            v * (1.0 / len)
        };
        let c = ray_dir.dot(to);
        if c < min_cone_cos {
            continue;
        }
        if dist < best_t {
            best_t = dist;
            best = Some(t.entity);
        }
    }
    let confidence = best
        .map(|_| (1.0 - best_t / max_range).clamp(0.0, 1.0))
        .unwrap_or(0.0);
    ReticleSnap {
        target: best,
        offset: Vec2 { x: 0.0, y: 0.0 },
        confidence,
    }
}
