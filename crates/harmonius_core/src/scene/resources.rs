//! Engine resources related to spatial queries.

use glam::Vec3;

use super::Entity;

/// Shared BVH resource used for gameplay spatial queries.
#[derive(Debug, Default)]
pub struct BvhResource {
    /// Counts how many AABB queries executed (tests use this as a probe).
    pub query_count: u64,
}

impl BvhResource {
    /// Records a query against the BVH.
    pub fn query_aabb(&mut self, _min: Vec3, _max: Vec3) -> &[Entity] {
        self.query_count = self.query_count.saturating_add(1);
        &[]
    }
}

/// Resource bag attached to [`super::World`].
#[derive(Debug, Default)]
pub struct Resources {
    /// Optional shared BVH instance (at most one).
    pub bvh: Option<BvhResource>,
}

/// Abstraction for spatial queries (allows instrumented fakes in tests).
pub trait SpatialQueryBackend {
    /// Executes an axis-aligned bounding box query.
    fn query_aabb(&mut self, min: Vec3, max: Vec3) -> &[Entity];
}

impl SpatialQueryBackend for BvhResource {
    fn query_aabb(&mut self, min: Vec3, max: Vec3) -> &[Entity] {
        BvhResource::query_aabb(self, min, max)
    }
}
