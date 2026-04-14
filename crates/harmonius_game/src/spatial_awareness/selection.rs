//! Player-facing selection queries routed through the shared spatial index.

use glam::{Vec2, Vec3};

use super::sense::Entity;
use super::spatial_index::SpatialIndex;

/// Selection query variants for editor and runtime picking.
#[derive(Clone, Debug, PartialEq)]
pub enum SelectionQuery {
    /// Nearest hit along a ray.
    Raycast {
        /// Ray origin in world space.
        origin: Vec3,
        /// Ray direction (need not be normalized).
        direction: Vec3,
        /// Maximum cast distance.
        max_distance: f32,
    },
    /// Axis-aligned box selection.
    BoxSelect {
        /// Minimum corner of the selection volume.
        min: Vec3,
        /// Maximum corner of the selection volume.
        max: Vec3,
    },
    /// Spherical brush selection.
    SphereSelect {
        /// Sphere center in world space.
        center: Vec3,
        /// Sphere radius in world units.
        radius: f32,
    },
    /// Nearest `count` entities inside `radius`.
    NearestN {
        /// Search origin.
        center: Vec3,
        /// Search radius.
        radius: f32,
        /// Maximum number of hits to return.
        count: u32,
    },
    /// Two-dimensional box selection on the XZ plane.
    BoxSelect2D {
        /// Minimum corner in XZ space (Y ignored for inclusion tests).
        min: Vec2,
        /// Maximum corner in XZ space.
        max: Vec2,
    },
    /// Two-dimensional circular selection on the XZ plane.
    CircleSelect2D {
        /// Circle center projected onto XZ.
        center: Vec2,
        /// Circle radius on the XZ plane.
        radius: f32,
    },
}

/// One hit reported by [`execute_selection`].
#[derive(Clone, Debug, PartialEq)]
pub struct SelectionResult {
    /// Entity that satisfied the query.
    pub entity: Entity,
    /// Representative hit position (entity center for volume queries).
    pub hit_point: Vec3,
    /// Distance from the query reference point.
    pub distance: f32,
}

/// Executes `query` against `spatial_index`, retaining only entities accepted by `filter`.
#[must_use]
pub fn execute_selection(
    query: &SelectionQuery,
    spatial_index: &SpatialIndex,
    filter: impl Fn(Entity) -> bool,
) -> Vec<SelectionResult> {
    match query {
        SelectionQuery::Raycast {
            origin,
            direction,
            max_distance,
        } => {
            let hits = spatial_index.raycast(*origin, *direction, *max_distance);
            let mut results = Vec::new();
            for (entity, distance) in hits {
                if !filter(entity) {
                    continue;
                }
                let position = spatial_index
                    .entities()
                    .iter()
                    .find(|pair| pair.0 == entity)
                    .map(|pair| pair.1)
                    .unwrap_or(*origin);
                results.push(SelectionResult {
                    entity,
                    hit_point: position,
                    distance,
                });
                break;
            }
            results
        }
        SelectionQuery::BoxSelect { min, max } => {
            let hits = spatial_index.query_box(*min, *max, true);
            build_hits(spatial_index, hits, *min, filter)
        }
        SelectionQuery::SphereSelect { center, radius } => {
            let hits = spatial_index.query_sphere(*center, *radius, true);
            build_hits(spatial_index, hits, *center, filter)
        }
        SelectionQuery::NearestN {
            center,
            radius,
            count,
        } => {
            let mut hits = spatial_index.query_sphere(*center, *radius, true);
            hits.truncate(*count as usize);
            build_hits(spatial_index, hits, *center, filter)
        }
        SelectionQuery::BoxSelect2D { min, max } => {
            let min3 = Vec3::new(min.x, f32::NEG_INFINITY, min.y);
            let max3 = Vec3::new(max.x, f32::INFINITY, max.y);
            let hits = spatial_index.query_box(min3, max3, true);
            build_hits(
                spatial_index,
                hits,
                Vec3::new((min.x + max.x) * 0.5, 0.0, (min.y + max.y) * 0.5),
                filter,
            )
        }
        SelectionQuery::CircleSelect2D { center, radius } => {
            let center3 = Vec3::new(center.x, 0.0, center.y);
            let hits = spatial_index.query_sphere(center3, *radius, true);
            let mut filtered = Vec::new();
            for entity in hits {
                if !filter(entity) {
                    continue;
                }
                let position = spatial_index
                    .entities()
                    .iter()
                    .find(|pair| pair.0 == entity)
                    .map(|pair| pair.1)
                    .unwrap_or(center3);
                let flat = Vec2::new(position.x, position.z);
                if (flat - *center).length() > *radius {
                    continue;
                }
                let distance = (position - center3).length();
                filtered.push(SelectionResult {
                    entity,
                    hit_point: position,
                    distance,
                });
            }
            filtered.sort_by(|left, right| {
                left.distance
                    .total_cmp(&right.distance)
                    .then_with(|| left.entity.cmp(&right.entity))
            });
            filtered
        }
    }
}

fn build_hits(
    spatial_index: &SpatialIndex,
    hits: Vec<Entity>,
    reference: Vec3,
    filter: impl Fn(Entity) -> bool,
) -> Vec<SelectionResult> {
    let mut results = Vec::new();
    for entity in hits {
        if !filter(entity) {
            continue;
        }
        let position = spatial_index
            .entities()
            .iter()
            .find(|pair| pair.0 == entity)
            .map(|pair| pair.1)
            .unwrap_or(reference);
        let distance = (position - reference).length();
        results.push(SelectionResult {
            entity,
            hit_point: position,
            distance,
        });
    }
    results.sort_by(|left, right| {
        left.distance
            .total_cmp(&right.distance)
            .then_with(|| left.entity.cmp(&right.entity))
    });
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial_awareness::spatial_index::SpatialTraversalMode;

    fn populated_index() -> SpatialIndex {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        index.insert(Entity(1), Vec3::new(0.0, 0.0, 2.0));
        index.insert(Entity(2), Vec3::new(0.0, 0.0, 4.0));
        index.insert(Entity(3), Vec3::new(0.0, 0.0, 6.0));
        index
    }

    /// TC-13.11.1.1 — raycast returns nearest entity first.
    #[test]
    fn test_selection_raycast_nearest() {
        let index = populated_index();
        let query = SelectionQuery::Raycast {
            origin: Vec3::new(0.0, 0.0, -1.0),
            direction: Vec3::Z,
            max_distance: 20.0,
        };
        let hits = execute_selection(&query, &index, |_| true);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity, Entity(1));
    }

    /// TC-13.11.1.2 — box select captures all interior entities.
    #[test]
    fn test_selection_box_all_inside() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        for idx in 1..=5 {
            index.insert(
                Entity(idx),
                Vec3::new(idx as f32, idx as f32, idx as f32),
            );
        }
        index.insert(Entity(6), Vec3::new(20.0, 20.0, 20.0));
        index.insert(Entity(7), Vec3::new(20.0, 20.0, 20.0));
        index.insert(Entity(8), Vec3::new(20.0, 20.0, 20.0));
        let query = SelectionQuery::BoxSelect {
            min: Vec3::splat(0.0),
            max: Vec3::splat(6.0),
        };
        let hits = execute_selection(&query, &index, |_| true);
        assert_eq!(hits.len(), 5);
    }

    /// TC-13.11.1.3 — sphere select obeys radius.
    #[test]
    fn test_selection_sphere_radius() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        index.insert(Entity(1), Vec3::new(0.0, 0.0, 5.0));
        index.insert(Entity(2), Vec3::new(0.0, 0.0, 15.0));
        let query = SelectionQuery::SphereSelect {
            center: Vec3::ZERO,
            radius: 10.0,
        };
        let hits = execute_selection(&query, &index, |_| true);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].entity, Entity(1));
    }

    /// TC-13.11.1.4 — nearest-N truncation.
    #[test]
    fn test_selection_nearest_n_count() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        for idx in 1..=10 {
            index.insert(Entity(idx), Vec3::new(0.0, 0.0, idx as f32));
        }
        let query = SelectionQuery::NearestN {
            center: Vec3::ZERO,
            radius: 50.0,
            count: 3,
        };
        let hits = execute_selection(&query, &index, |_| true);
        assert_eq!(hits.len(), 3);
    }

    /// TC-13.11.1.5 — filter predicate removes entities.
    #[test]
    fn test_selection_filter_excludes() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh3d);
        for idx in 1..=5 {
            index.insert(Entity(idx), Vec3::new(0.0, 0.0, idx as f32));
        }
        let query = SelectionQuery::SphereSelect {
            center: Vec3::ZERO,
            radius: 10.0,
        };
        let hits = execute_selection(&query, &index, |entity| entity.0 % 2 == 1);
        assert_eq!(hits.len(), 3);
    }

    /// TC-13.11.1.6 — results sorted by ascending distance.
    #[test]
    fn test_selection_sorted_by_distance() {
        let index = populated_index();
        let query = SelectionQuery::SphereSelect {
            center: Vec3::ZERO,
            radius: 20.0,
        };
        let hits = execute_selection(&query, &index, |_| true);
        let mut sorted = hits.clone();
        sorted.sort_by(|left, right| {
            left.distance
                .total_cmp(&right.distance)
                .then_with(|| left.entity.cmp(&right.entity))
        });
        assert_eq!(hits, sorted);
    }

    /// TC-13.11.2.1 — 2D box selection on XZ plane.
    #[test]
    fn test_selection_boxselect2d() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh2d);
        index.insert(Entity(1), Vec3::new(1.0, 0.0, 1.0));
        index.insert(Entity(2), Vec3::new(2.0, 0.0, -1.0));
        index.insert(Entity(3), Vec3::new(-2.0, 0.0, 2.0));
        index.insert(Entity(4), Vec3::new(8.0, 0.0, 8.0));
        index.insert(Entity(5), Vec3::new(-8.0, 0.0, -8.0));
        let query = SelectionQuery::BoxSelect2D {
            min: Vec2::new(-5.0, -5.0),
            max: Vec2::new(5.0, 5.0),
        };
        let hits = execute_selection(&query, &index, |_| true);
        assert_eq!(hits.len(), 3);
    }

    /// TC-13.11.2.2 — circle select on XZ plane.
    #[test]
    fn test_selection_circleselect2d() {
        let mut index = SpatialIndex::new(SpatialTraversalMode::Bvh2d);
        for idx in 1..=5 {
            index.insert(Entity(idx), Vec3::new(idx as f32 * 3.0, 0.0, 0.0));
        }
        let query = SelectionQuery::CircleSelect2D {
            center: Vec2::ZERO,
            radius: 10.0,
        };
        let hits = execute_selection(&query, &index, |_| true);
        assert_eq!(hits.len(), 3);
    }
}
