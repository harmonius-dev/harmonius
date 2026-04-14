//! Physics ↔ world geometry integration contracts.
//!
//! This crate materializes the data contracts from
//! `docs/design/integration/physics-geometry.md` as executable Rust types and pure helpers so CI can
//! enforce the integration surface before the full engine wiring lands.
//!
//! # Scope notes
//!
//! - [`ColliderShape`] is a bootstrap subset of the design enum until additional producers land.
//! - Persistent `rkyv` derives on mesh and terrain payloads are tracked for a follow-up once the
//!   workspace policy for integration crates is fixed; this pass focuses on types and pure helpers.
//! - `glam` is listed in `docs/design/constraints.md` core deps; the integrating PR description
//!   records maintainer approval for new workspace dependencies.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

mod collider_shape;
mod collision_layers;
mod heightfield;
mod materials;
mod terrain_collider;
mod terrain_tile;
mod types;
mod voxel_mesh;

pub use collider_shape::{ColliderShape, TriMeshData};
pub use collision_layers::CollisionLayers;
pub use heightfield::{
    heightfield_collider_from_tile, heightfield_collider_from_tile_with_scale,
    validate_heightfield_scale, HeightfieldBuildError, HeightfieldCollider,
};
pub use materials::{DEFAULT_PHYSICS_MATERIAL_FRICTION, DEFAULT_PHYSICS_MATERIAL_RESTITUTION};
pub use terrain_collider::TerrainCollider;
pub use terrain_tile::TerrainTileSample;
pub use types::{BvhHandle, ChunkCoord, PhysicsMaterialHandle, TerrainHole};
pub use voxel_mesh::VoxelCollisionMesh;
