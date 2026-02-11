use derive_more::{Display, Error, From};
use smallvec::SmallVec;

use crate::{BufferHandle, ImageHandle, PipelineHandle};

/// Describes a multi-stage rendering operation that is compiled into an efficient execution plan.
#[derive(Debug, Default, Clone)]
pub struct RenderGraph {}

// An error that can occur when constructing and validating a render graph.
#[derive(Debug, Display, Error, From)]
pub enum RenderGraphError {}

/// The type of a resource slot.
#[derive(Debug, Clone, From)]
pub enum ResourceSlotType {
    Image,
    Buffer,
}

/// A node in the render graph that represents a single pass.
pub trait RenderPassNode {
    fn slots(&self) -> Vec<ResourceSlotType>;
}

/// A descriptor for a resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceDescriptor {
    Image {
        width: u32,
        height: u32,
        depth: u32,
        format: crate::Format,
    },
    Buffer {
        size: u32,
        format: crate::Format,
    },
}

/// A slab of memory allocated for a resource from a buffer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferSlab {
    buffer: BufferHandle,
    offset: u64,
    size: u32,
    stride: u32,
}

/// A slot containing a buffer
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ResourceSlot {
    slot: u32,
    descriptor: ResourceDescriptor,
}

/// A single platform-independent draw command issued by a render pass.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DrawCommand {
    BindPipeline(PipelineHandle),
    BindBuffer(ResourceSlot, BufferSlab),
    BindImage(ResourceSlot, ImageHandle),
    Draw,
    DrawIndexed {
        index_slab: BufferSlab,
    },
    DrawIndirect {
        command_slab: BufferSlab,
        draw_slab: BufferSlab,
    },
    DrawIndexedIndirect {
        command_slab: BufferSlab,
        draw_slab: BufferSlab,
    },
}

/// A bundle of draw commands issued by a render pass, used for sorting and batching draw calls.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DrawCommandBundle {
    commands: SmallVec<[DrawCommand; 8]>,
}
