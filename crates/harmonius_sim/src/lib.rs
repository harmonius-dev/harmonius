//! Typed 2D cell grids, voxel volumes, and GPU sync helpers for Harmonius simulation.
#![cfg_attr(not(test), forbid(unsafe_code))]
#![deny(clippy::all)]
#![deny(missing_docs)]

pub mod aoi;
pub mod cell_grid;
pub mod coord;
pub mod flow_field;
pub mod gpu_dispatch;
pub mod gpu_sync;
pub mod hierarchical;
pub mod sdf;
pub mod voxel_volume;

pub use aoi::EntitySet;
pub use cell_grid::{
    Arena, CellGrid, FloodFillResult, GridQueryShape, LineOfSightResult, NeighborPattern,
    PropagationKernel, UniformGrid,
};
pub use coord::{CellCoord, ChunkCoord, VoxelCoord};
pub use flow_field::compute_flow_field;
pub use gpu_dispatch::{propagation_dispatch_decision, GpuPropagationPath, StagingUploadStats};
pub use gpu_sync::{DirtyRegion, GpuGridSync};
pub use hierarchical::HierarchicalGrid;
pub use sdf::{recompute_sdf_dirty_region, SdfDirtyRegion};
pub use voxel_volume::{PaletteVoxelChunk, VoxelChunk, VoxelHit, VoxelVolume};
