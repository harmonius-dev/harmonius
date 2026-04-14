use std::cmp::Ordering;

use glam::{Vec3, Vec4};

use super::aabb::{Aabb, FrustumTest, Sphere};
use super::bvh::BvhIndex;
use super::entity::Entity;
use super::grid::UniformGrid;
use super::layers::SpatialLayerMask;

/// One hit result from a spatial query.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialHit {
    /// Entity that was hit.
    pub entity: Entity,
    /// Hit position in world space.
    pub point: Vec3,
    /// Surface normal (zero for overlap queries).
    pub normal: Vec3,
    /// Distance along the ray or penetration metric for overlaps.
    pub distance: f32,
}

/// Sorting preference for spatial hits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuerySort {
    /// Closest hits first.
    Nearest,
    /// Unspecified order (fastest).
    Unsorted,
}

/// Configuration for spatial queries.
#[derive(Clone, Debug)]
pub struct QueryConfig {
    /// Layer mask filter.
    pub layer_mask: SpatialLayerMask,
    /// Maximum hits (`0` means unlimited).
    pub max_results: u32,
    /// Sorting mode.
    pub sort: QuerySort,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            layer_mask: SpatialLayerMask::ALL,
            max_results: 0,
            sort: QuerySort::Nearest,
        }
    }
}

/// Supported primitive query shapes.
#[derive(Clone, Debug)]
pub enum QueryShape {
    /// Ray cast in world space.
    Ray {
        /// Ray origin.
        origin: Vec3,
        /// Ray direction (need not be normalized).
        direction: Vec3,
        /// Maximum travel distance along the ray.
        max_distance: f32,
    },
    /// Axis-aligned overlap query.
    AabbOverlap(Aabb),
    /// Spherical overlap query.
    SphereOverlap(Sphere),
    /// Frustum described by six inward-facing planes.
    Frustum([Vec4; 6]),
    /// k-nearest neighbors around a point.
    KNearest {
        /// Query origin.
        origin: Vec3,
        /// Number of neighbors to return.
        k: u32,
        /// Maximum search radius.
        max_radius: f32,
    },
}

/// Unified spatial query surface implemented by [`QueryEngine`].
pub trait SpatialQuery {
    /// Executes a generic query.
    fn query(&self, shape: &QueryShape, config: &QueryConfig) -> Vec<SpatialHit>;

    /// Ray cast convenience wrapper.
    fn ray_cast(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        config: &QueryConfig,
    ) -> Vec<SpatialHit> {
        self.query(
            &QueryShape::Ray {
                origin,
                direction,
                max_distance,
            },
            config,
        )
    }

    /// Returns the closest hit along a ray, if any.
    fn ray_cast_nearest(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        layer_mask: SpatialLayerMask,
    ) -> Option<SpatialHit>;

    /// Returns true if any hit exists along the ray.
    fn ray_test(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        layer_mask: SpatialLayerMask,
    ) -> bool;

    /// Axis-aligned overlap query.
    fn overlap_aabb(&self, aabb: &Aabb, config: &QueryConfig) -> Vec<SpatialHit>;

    /// Spherical overlap query.
    fn overlap_sphere(&self, center: Vec3, radius: f32, config: &QueryConfig) -> Vec<SpatialHit>;

    /// Frustum query returning entities whose bounds overlap the frustum.
    fn frustum_query(&self, planes: &[Vec4; 6], layer_mask: SpatialLayerMask) -> Vec<Entity>;

    /// k-nearest neighbors query.
    fn k_nearest(
        &self,
        origin: Vec3,
        k: u32,
        max_radius: f32,
        config: &QueryConfig,
    ) -> Vec<SpatialHit>;
}

/// Concrete [`SpatialQuery`] implementation over a [`BvhIndex`] and optional [`UniformGrid`].
pub struct QueryEngine<'a> {
    bvh: &'a BvhIndex,
    #[allow(dead_code)]
    grid: Option<&'a UniformGrid>,
}

impl<'a> QueryEngine<'a> {
    /// Creates a query engine bound to the provided indices.
    #[must_use]
    pub fn new(bvh: &'a BvhIndex, grid: Option<&'a UniformGrid>) -> Self {
        Self { bvh, grid }
    }

    fn collect_ray_hits(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        config: &QueryConfig,
    ) -> Vec<SpatialHit> {
        let dir_len = direction.length();
        if dir_len <= f32::EPSILON {
            return Vec::new();
        }
        let dir = direction / dir_len;
        let mut hits = Vec::new();
        self.bvh
            .traverse_ray(origin, dir, max_distance, config.layer_mask, |leaf, t| {
                hits.push(SpatialHit {
                    entity: leaf.entity,
                    point: origin + dir * t,
                    normal: Vec3::ZERO,
                    distance: t,
                });
                true
            });
        finalize_hits(hits, config)
    }
}

impl SpatialQuery for QueryEngine<'_> {
    fn query(&self, shape: &QueryShape, config: &QueryConfig) -> Vec<SpatialHit> {
        match shape {
            QueryShape::Ray {
                origin,
                direction,
                max_distance,
            } => self.collect_ray_hits(*origin, *direction, *max_distance, config),
            QueryShape::AabbOverlap(aabb) => self.overlap_aabb(aabb, config),
            QueryShape::SphereOverlap(sphere) => {
                self.overlap_sphere(sphere.center, sphere.radius, config)
            }
            QueryShape::Frustum(planes) => self
                .frustum_query(planes, config.layer_mask)
                .into_iter()
                .map(|entity| SpatialHit {
                    entity,
                    point: Vec3::ZERO,
                    normal: Vec3::ZERO,
                    distance: 0.0,
                })
                .collect(),
            QueryShape::KNearest {
                origin,
                k,
                max_radius,
            } => self.k_nearest(*origin, *k, *max_radius, config),
        }
    }

    fn ray_cast_nearest(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        layer_mask: SpatialLayerMask,
    ) -> Option<SpatialHit> {
        let mut best: Option<SpatialHit> = None;
        let dir_len = direction.length();
        if dir_len <= f32::EPSILON {
            return None;
        }
        let dir = direction / dir_len;
        self.bvh
            .traverse_ray(origin, dir, max_distance, layer_mask, |leaf, t| {
                let hit = SpatialHit {
                    entity: leaf.entity,
                    point: origin + dir * t,
                    normal: Vec3::ZERO,
                    distance: t,
                };
                best = Some(match &best {
                    None => hit,
                    Some(prev) => {
                        if hit.distance < prev.distance {
                            hit
                        } else {
                            prev.clone()
                        }
                    }
                });
                true
            });
        best
    }

    fn ray_test(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        layer_mask: SpatialLayerMask,
    ) -> bool {
        self.ray_cast_nearest(origin, direction, max_distance, layer_mask)
            .is_some()
    }

    fn overlap_aabb(&self, aabb: &Aabb, config: &QueryConfig) -> Vec<SpatialHit> {
        let mut hits = Vec::new();
        self.bvh
            .for_each_overlapping_leaf(aabb, config.layer_mask, |leaf| {
                if leaf.aabb.intersects(aabb) {
                    hits.push(SpatialHit {
                        entity: leaf.entity,
                        point: aabb.center(),
                        normal: Vec3::ZERO,
                        distance: leaf.aabb.center().distance(aabb.center()),
                    });
                }
                true
            });
        finalize_hits(hits, config)
    }

    fn overlap_sphere(&self, center: Vec3, radius: f32, config: &QueryConfig) -> Vec<SpatialHit> {
        let loose = Aabb::from_center_extents(center, Vec3::splat(radius));
        let mut hits = Vec::new();
        self.bvh
            .for_each_overlapping_leaf(&loose, config.layer_mask, |leaf| {
                if leaf.aabb.intersects_sphere(center, radius) {
                    hits.push(SpatialHit {
                        entity: leaf.entity,
                        point: center,
                        normal: Vec3::ZERO,
                        distance: leaf.aabb.center().distance(center),
                    });
                }
                true
            });
        finalize_hits(hits, config)
    }

    fn frustum_query(&self, planes: &[Vec4; 6], layer_mask: SpatialLayerMask) -> Vec<Entity> {
        let mut entities = Vec::new();
        let world = loose_frustum_aabb(planes);
        self.bvh
            .for_each_overlapping_leaf(&world, layer_mask, |leaf| {
                match leaf.aabb.intersects_frustum(planes) {
                    FrustumTest::Outside => {}
                    FrustumTest::Intersecting | FrustumTest::Inside => {
                        entities.push(leaf.entity);
                    }
                }
                true
            });
        entities.sort_by_key(|e| e.0);
        entities.dedup();
        entities
    }

    fn k_nearest(
        &self,
        origin: Vec3,
        k: u32,
        max_radius: f32,
        config: &QueryConfig,
    ) -> Vec<SpatialHit> {
        let mut candidates: Vec<SpatialHit> = Vec::new();
        let loose = Aabb::from_center_extents(origin, Vec3::splat(max_radius));
        self.bvh
            .for_each_overlapping_leaf(&loose, config.layer_mask, |leaf| {
                let d = leaf.aabb.center().distance(origin);
                if d <= max_radius {
                    candidates.push(SpatialHit {
                        entity: leaf.entity,
                        point: leaf.aabb.center(),
                        normal: Vec3::ZERO,
                        distance: d,
                    });
                }
                true
            });
        candidates.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(Ordering::Equal)
        });
        candidates.truncate(k as usize);
        finalize_hits(candidates, config)
    }
}

fn finalize_hits(mut hits: Vec<SpatialHit>, config: &QueryConfig) -> Vec<SpatialHit> {
    if config.sort == QuerySort::Nearest {
        hits.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(Ordering::Equal)
        });
    }
    if config.max_results > 0 {
        hits.truncate(config.max_results as usize);
    }
    hits
}

fn loose_frustum_aabb(_planes: &[Vec4; 6]) -> Aabb {
    Aabb::new(Vec3::splat(-1_000_000.0), Vec3::splat(1_000_000.0))
}

/// One entry in a batch query dispatch list.
#[derive(Clone, Debug)]
pub struct BatchQuery {
    /// Query shape to execute.
    pub shape: QueryShape,
    /// Query configuration.
    pub config: QueryConfig,
}

/// Result for a single batch entry.
#[derive(Clone, Debug)]
pub struct BatchQueryResult {
    /// Index in the original batch.
    pub query_index: u32,
    /// Hits for that query.
    pub hits: Vec<SpatialHit>,
}
