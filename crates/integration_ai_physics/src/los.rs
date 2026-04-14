//! IR-2.5.6 line-of-sight queries against simplified occluder geometry.
//!
//! Perception pipeline wiring lives in `docs/design/integration/ai-spatial-awareness.md`; this
//! crate only holds the LOS query contract exercised in tests.

use glam::Vec3;

use crate::geometry::AxisAlignedBox;
use crate::types::Entity;

/// Line-of-sight query between two points in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineOfSightQuery {
    pub observer: Vec3,
    pub target: Vec3,
}

/// Result classification for LOS tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineOfSightResult {
    /// No occluder intersects the open segment `(observer, target)`.
    Visible,
    /// An occluder blocks the segment before reaching the target.
    Occluded {
        /// First blocking entity along the ray.
        blocker: Entity,
    },
}

/// Scene containing axis-aligned occluders.
#[derive(Clone, Debug, Default)]
pub struct LineOfSightScene {
    pub occluders: Vec<(Entity, AxisAlignedBox)>,
}

impl LineOfSightScene {
    /// Evaluates LOS along the straight segment to `target`.
    #[must_use]
    pub fn query(&self, query: LineOfSightQuery) -> LineOfSightResult {
        let delta = query.target - query.observer;
        let dist = delta.length();
        if dist <= f32::EPSILON {
            return LineOfSightResult::Visible;
        }

        let dir = delta / dist;
        let mut best_t = f32::MAX;
        let mut best_entity = Entity::NONE;

        for (entity, aabb) in &self.occluders {
            if let Some(t_hit) = aabb.ray_segment_hit_t(query.observer, dir * dist) {
                if (0.0..=1.0).contains(&t_hit) && t_hit < best_t {
                    best_t = t_hit;
                    best_entity = *entity;
                }
            }
        }

        if best_entity == Entity::NONE {
            LineOfSightResult::Visible
        } else {
            LineOfSightResult::Occluded {
                blocker: best_entity,
            }
        }
    }
}
