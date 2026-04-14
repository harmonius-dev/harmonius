//! Spatial indexing: BVH, uniform grid, and query surfaces.
//!
//! Implements the CPU-side structures described in `docs/design/core-runtime/spatial-index.md`.

mod aabb;
mod bvh;
mod entity;
mod error;
mod grid;
mod handle;
mod layers;
mod query;

pub use aabb::{Aabb, Aabb2D, FrustumTest, Sphere};
pub use bvh::{BvhConfig, BvhIndex};
pub use entity::Entity;
pub use error::SpatialError;
pub use grid::{GridConfig, GridCoord, UniformGrid};
pub use handle::BvhHandle;
pub use layers::SpatialLayerMask;
pub use query::{
    BatchQuery, BatchQueryResult, QueryConfig, QueryEngine, QueryShape, QuerySort, SpatialHit,
    SpatialQuery,
};

#[cfg(test)]
mod tests;
