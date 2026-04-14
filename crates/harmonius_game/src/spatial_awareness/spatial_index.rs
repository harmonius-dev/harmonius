//! Shared spatial index facade used by sense and selection queries.

use glam::Vec3;

use super::sense::Entity;

/// Identifies which traversal path a [`SpatialIndex`] uses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialTraversalMode {
    /// Three-dimensional world queries.
    Bvh3d,
    /// Planar queries mapped onto the XZ plane.
    Bvh2d,
}

/// Deterministic linear spatial index used until the engine BVH lands.
///
/// All query entry points share the same backing storage so sense and selection
/// paths exercise identical candidate sets.
#[derive(Clone, Debug)]
pub struct SpatialIndex {
    /// Declared traversal mode (2D queries assert this in tests).
    pub mode: SpatialTraversalMode,
    entities: Vec<(Entity, Vec3)>,
}

impl SpatialIndex {
    /// Builds an empty index with the requested traversal mode.
    #[must_use]
    pub fn new(mode: SpatialTraversalMode) -> Self {
        Self {
            mode,
            entities: Vec::new(),
        }
    }

    /// Inserts or replaces an entity position.
    pub fn insert(&mut self, entity: Entity, position: Vec3) {
        if let Some(existing) = self.entities.iter_mut().find(|pair| pair.0 == entity) {
            existing.1 = position;
            return;
        }
        self.entities.push((entity, position));
    }

    /// Clears all tracked entities.
    pub fn clear(&mut self) {
        self.entities.clear();
    }

    /// Returns the tracked `(entity, position)` pairs.
    #[must_use]
    pub fn entities(&self) -> &[(Entity, Vec3)] {
        &self.entities
    }

    /// Returns entities whose AABB distance to `center` is within `radius`.
    #[must_use]
    pub fn query_sphere(&self, center: Vec3, radius: f32, sort_by_distance: bool) -> Vec<Entity> {
        let mut hits: Vec<(Entity, f32)> = self
            .entities
            .iter()
            .filter_map(|(entity, position)| {
                let distance = (*position - center).length();
                if distance <= radius {
                    Some((*entity, distance))
                } else {
                    None
                }
            })
            .collect();

        if sort_by_distance {
            hits.sort_by(|left, right| {
                left.1
                    .total_cmp(&right.1)
                    .then_with(|| left.0.cmp(&right.0))
            });
        } else {
            hits.sort_by_key(|pair| pair.0);
        }

        hits.into_iter().map(|pair| pair.0).collect()
    }

    /// Returns entities inside an axis-aligned box.
    #[must_use]
    pub fn query_box(&self, min: Vec3, max: Vec3, sort_by_distance: bool) -> Vec<Entity> {
        let center = (min + max) * 0.5;
        let mut hits: Vec<(Entity, f32)> = self
            .entities
            .iter()
            .filter_map(|(entity, position)| {
                if position.x >= min.x
                    && position.x <= max.x
                    && position.y >= min.y
                    && position.y <= max.y
                    && position.z >= min.z
                    && position.z <= max.z
                {
                    let distance = (*position - center).length();
                    Some((*entity, distance))
                } else {
                    None
                }
            })
            .collect();

        if sort_by_distance {
            hits.sort_by(|left, right| {
                left.1
                    .total_cmp(&right.1)
                    .then_with(|| left.0.cmp(&right.0))
            });
        } else {
            hits.sort_by_key(|pair| pair.0);
        }

        hits.into_iter().map(|pair| pair.0).collect()
    }

    /// Returns the nearest hit along `direction` starting from `origin`.
    #[must_use]
    pub fn raycast(&self, origin: Vec3, direction: Vec3, max_distance: f32) -> Vec<(Entity, f32)> {
        let dir = direction.normalize_or_zero();
        if dir.length_squared() <= f32::EPSILON {
            return Vec::new();
        }

        let mut hits = Vec::new();
        for (entity, position) in &self.entities {
            let offset = *position - origin;
            let projection = offset.dot(dir);
            if projection < 0.0 || projection > max_distance {
                continue;
            }
            let closest = origin + dir * projection;
            let radial = (*position - closest).length();
            if radial > 0.5 {
                // Treat each entity as a small sphere for picking tests.
                continue;
            }
            hits.push((*entity, projection));
        }

        hits.sort_by(|left, right| {
            left.1
                .total_cmp(&right.1)
                .then_with(|| left.0.cmp(&right.0))
        });
        hits
    }

    /// Simplified frustum query backed by the same linear scan as [`Self::query_box`].
    #[must_use]
    pub fn query_frustum(&self, min: Vec3, max: Vec3, sort_by_distance: bool) -> Vec<Entity> {
        self.query_box(min, max, sort_by_distance)
    }
}
