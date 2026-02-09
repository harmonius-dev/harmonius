//! Vulkan render graph and graphics utilities for gcraft.
//!
//! This crate provides a declarative render graph builder ([`RenderGraph`]),
//! compilation with automatic barrier insertion ([`CompiledGraph`]), device
//! and queue abstraction ([`DeviceContext`]), frame synchronization
//! ([`FrameSync`]), and transient resource pooling ([`ResourcePool`]).

pub mod device;
pub mod graph;
pub mod command;
pub mod resource;
pub mod sync;

// ---------------------------------------------------------------------------
// Public API re-exports
// ---------------------------------------------------------------------------

pub use device::{DeviceContext, QueueHandle, QueueType};
pub use graph::{CompiledGraph, RenderGraph};
pub use graph::resource::{
    BufferDesc, BufferHandle, ConditionFlag, DrawSlot, GraphError, ImageDesc, ImageHandle,
    SubresourceRange,
};
pub use graph::pass::{PassBuilder, RecordedCommand};
pub use command::batch::{Batchable, DrawParams, fill_opaque, fill_transparent};
pub use command::indirect::{collapse_to_indirect, DrawIndexedIndirectCommand, IndirectBatch};
pub use resource::pool::ResourcePool;
pub use sync::FrameSync;

#[cfg(test)]
pub mod test_utils;
