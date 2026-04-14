//! CPU-side contracts for rendering ↔ grids/volumes integration.
//!
//! See `docs/design/integration/rendering-grids-volumes.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod dirty_region_set;
pub mod fog;
pub mod grid_sync;
pub mod handles;
pub mod pass;
pub mod sdf;
pub mod types;

pub use dirty_region_set::DirtyRegionSet;
pub use fog::{fog_cell_to_r8, FogCellState};
pub use grid_sync::{GpuGridSync, GridUploadCommand, GridUploadQueue};
pub use handles::{DeviceTextureArena, GpuTextureHandle};
pub use pass::{
    register_fog_overlay_pass, register_sdf_shadow_pass, register_voxel_gi_pass, FogGpuTexture,
    GridPassRegistration, RenderCommandBuffer, RenderGraphBuilder, VolumeGpuTexture,
};
pub use sdf::{clamp_sdf_cell, reject_nan_sdf_cell, SdfSampleRejected};
pub use types::{
    GridAssetHandle, GridDimension, GridFormat, GridTransform, GridUploadPriority,
    GridUploadStatus, SdfClampMode,
};
