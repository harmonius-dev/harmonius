//! Deterministic meshlet asset layout and CPU-side [`MeshletBuilder`].
//!
//! Implements the Harmonius `MeshletAsset` pipeline described in `docs/design/rendering/meshlets.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]

mod asset;
mod builder;
mod gpu_layout;
mod mesh_input;
mod topology;

pub use asset::{AssetId, BufferView, LodGroup, Meshlet, MeshletAsset};
pub use builder::{BuildError, MeshletBuilder};
pub use gpu_layout::{meshlet_asset_gpu_layout_valid, vertex_stride_bytes};
pub use mesh_input::NormalizedMesh;
