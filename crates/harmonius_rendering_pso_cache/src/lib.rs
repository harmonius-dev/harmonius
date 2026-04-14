//! Pipeline state object cache types and helpers for Harmonius rendering.
//!
//! This crate implements the in-memory and on-disk contracts described in
//! `docs/design/rendering/pipeline-state-cache.md` with deterministic, testable
//! pure-Rust behavior (no GPU backends).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod cache_error;
mod content_hash;
mod deferred;
mod descriptor_layout;
mod device;
mod disk_index;
mod memory_cache;
mod pso_cache;
mod pso_key;
mod sorted_vec_map;

pub use cache_error::{CacheError, PsoError};
pub use content_hash::ContentHash;
pub use deferred::FrameBarrier;
pub use descriptor_layout::{infer_descriptor_layout, DescriptorLayout, ShaderApi, ShaderBinding};
pub use device::{DeviceFingerprint, DeviceId, GpuDevice};
pub use disk_index::DiskIndex;
pub use memory_cache::{MemoryEntry, MemoryPsoCache};
pub use pso_cache::{PipelineDesc, PsoCache, PsoCompiler, PsoEntry, PsoHandle};
pub use pso_key::PsoKey;
pub use sorted_vec_map::SortedVecMap;
