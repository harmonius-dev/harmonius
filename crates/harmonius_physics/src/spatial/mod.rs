//! Spatial queries: rays, sweeps, overlaps, closest points, batching.

mod batch;
mod filter;
mod intersect;
mod scene;
mod shape;

pub use batch::{run_batch, run_rays_individually, SpatialQueryDescriptor, SpatialQueryResult};
pub use filter::{CollisionLayerMask, ComponentTagMask, PredicateFn, QueryFilter};
pub use scene::{
    BvhScene, ClosestPointResult, ColliderEntry, OverlapResult, RayHit, ShapeHit,
};
pub use shape::{ColliderShape, ColliderTransform};

/// Alias matching legacy export name.
pub type ShapeCastHit = ShapeHit;

#[cfg(test)]
mod tests;
