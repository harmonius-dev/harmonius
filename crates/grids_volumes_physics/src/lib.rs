//! Grids and volumes integration with physics: shared types and pure helpers for IR-3.10.x.
//!
//! See `docs/design/integration/grids-volumes-physics.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod los;
mod occupancy;
mod types;

pub use los::{propagation_los_check, LosCache, PhysicsQueries, RayHit};
pub use occupancy::OccupancyUpdate;
pub use types::{
    destruction_pattern_tag, CellCoord, DestructionPattern, Entity, VoxelCoord,
    VoxelDestructionRequest,
};
