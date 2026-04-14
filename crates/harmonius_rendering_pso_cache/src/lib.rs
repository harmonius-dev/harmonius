//! Pipeline state object cache types and helpers for Harmonius rendering.
//!
//! This crate holds the in-memory and on-disk contracts aligned with
//! `docs/design/rendering/pipeline-state-cache.md`, with deterministic,
//! testable pure-Rust behavior (no GPU backends).
//!
//! # On-disk layout (implementation v1)
//!
//! The device partition uses [`DeviceFingerprint::directory_name`], which folds
//! the graphics API token into the subdirectory name so distinct APIs never
//! share a cache root.
//!
//! The blob index file is `index.bin` (magic `HMNSPIDX`, versioned header, CRC
//! over record bodies). A future revision may switch the index to an mmap'd
//! rkyv archive as sketched in the design prose; v1 keeps this framed binary
//! format for self-contained bring-up and corruption recovery tests.

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
