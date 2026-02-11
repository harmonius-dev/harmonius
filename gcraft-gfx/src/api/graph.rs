use derive_more::{Display, From};
use smallvec::SmallVec;

use crate::{BufferHandle, Format, ImageHandle, PipelineHandle};

// ── Graph-local handles ─────────────────────────────────────────────

/// A handle to a virtual resource declared within a [`RenderGraph`].
///
/// Virtual resources are resolved to concrete allocations during compilation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualResourceId(u32);

/// A handle to a pass that has been added to a [`RenderGraph`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassId(u32);

// ── Resource access ─────────────────────────────────────────────────

/// How a render pass accesses a resource through a slot.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceAccess {
    /// The pass only reads from the resource.
    Read,
    /// The pass only writes to the resource.
    Write,
    /// The pass both reads from and writes to the resource.
    ReadWrite,
}

/// Describes a single resource slot exposed by a [`RenderPassNode`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlotDescriptor {
    /// The kind of resource this slot accepts.
    pub slot_type: ResourceSlotType,
    /// How the pass accesses the resource through this slot.
    pub access: ResourceAccess,
}

// ── Resource slot types ─────────────────────────────────────────────

/// The type of a resource slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub enum ResourceSlotType {
    Image,
    Buffer,
}

// ── Render pass trait ───────────────────────────────────────────────

/// A node in the render graph that represents a single render pass.
///
/// Implementors declare their resource slots (with types and access modes).
/// The [`RenderGraph`] builder wires these slots to virtual resources.
pub trait RenderPassNode {
    /// Declares this pass's resource slots with their types and access modes.
    ///
    /// Slot indices correspond to positions in the returned [`Vec`].
    fn slots(&self) -> Vec<SlotDescriptor>;
}

// ── Resource descriptors ────────────────────────────────────────────

/// A descriptor for a resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceDescriptor {
    Image {
        width: u32,
        height: u32,
        depth: u32,
        format: Format,
    },
    Buffer {
        size: u32,
        format: Format,
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

// ── Draw commands ───────────────────────────────────────────────────

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

// ── Graph internals ─────────────────────────────────────────────────

/// How a virtual resource was introduced into the graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum VirtualResource {
    /// A transient resource created and destroyed within the frame.
    Transient { descriptor: ResourceDescriptor },
    /// An externally-owned image imported into the graph.
    ImportedImage,
    /// An externally-owned buffer imported into the graph.
    ImportedBuffer,
}

impl VirtualResource {
    fn slot_type(&self) -> ResourceSlotType {
        match self {
            VirtualResource::Transient { descriptor } => match descriptor {
                ResourceDescriptor::Image { .. } => ResourceSlotType::Image,
                ResourceDescriptor::Buffer { .. } => ResourceSlotType::Buffer,
            },
            VirtualResource::ImportedImage => ResourceSlotType::Image,
            VirtualResource::ImportedBuffer => ResourceSlotType::Buffer,
        }
    }
}

/// Binds a pass's slot to a virtual resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SlotBinding {
    pass: PassId,
    slot: u32,
    resource: VirtualResourceId,
}

/// Storage for a pass added to the graph.
struct PassEntry {
    node: Box<dyn RenderPassNode>,
}

// ── RenderGraph ─────────────────────────────────────────────────────

/// A multi-stage rendering operation that is compiled into an efficient execution plan.
///
/// Use the builder methods to declare resources, add passes, and bind
/// pass slots to resources.
pub struct RenderGraph {
    passes: Vec<PassEntry>,
    resources: Vec<VirtualResource>,
    bindings: Vec<SlotBinding>,
}

impl RenderGraph {
    /// Create a new, empty render graph.
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            resources: Vec::new(),
            bindings: Vec::new(),
        }
    }

    /// Declare a transient resource that lives only for the current frame.
    ///
    /// Returns a [`VirtualResourceId`] used to bind this resource to pass slots.
    pub fn create_transient(&mut self, descriptor: ResourceDescriptor) -> VirtualResourceId {
        let id = VirtualResourceId(self.resources.len() as u32);
        self.resources.push(VirtualResource::Transient { descriptor });
        id
    }

    /// Import an externally-owned image into the graph (e.g. a swapchain image).
    ///
    /// Returns a [`VirtualResourceId`] used to bind this resource to pass slots.
    pub fn import_image(&mut self) -> VirtualResourceId {
        let id = VirtualResourceId(self.resources.len() as u32);
        self.resources.push(VirtualResource::ImportedImage);
        id
    }

    /// Import an externally-owned buffer into the graph.
    ///
    /// Returns a [`VirtualResourceId`] used to bind this resource to pass slots.
    pub fn import_buffer(&mut self) -> VirtualResourceId {
        let id = VirtualResourceId(self.resources.len() as u32);
        self.resources.push(VirtualResource::ImportedBuffer);
        id
    }

    /// Add a render pass node to the graph.
    ///
    /// Returns a [`PassId`] used to bind the pass's slots to virtual resources.
    pub fn add_pass(&mut self, node: impl RenderPassNode + 'static) -> PassId {
        let id = PassId(self.passes.len() as u32);
        self.passes.push(PassEntry {
            node: Box::new(node),
        });
        id
    }

    /// Bind a pass's slot to a virtual resource.
    ///
    /// `slot` is the index into the [`Vec`] returned by [`RenderPassNode::slots()`].
    /// The resource type must match the slot's declared [`ResourceSlotType`].
    pub fn bind(
        &mut self,
        pass: PassId,
        slot: u32,
        resource: VirtualResourceId,
    ) -> Result<(), RenderGraphError> {
        let entry = self
            .passes
            .get(pass.0 as usize)
            .ok_or(RenderGraphError::InvalidPassId(pass))?;

        let virt = self
            .resources
            .get(resource.0 as usize)
            .ok_or(RenderGraphError::InvalidResourceId(resource))?;

        let slots = entry.node.slots();
        let slot_desc = slots
            .get(slot as usize)
            .ok_or(RenderGraphError::SlotIndexOutOfBounds {
                pass,
                slot,
                slot_count: slots.len() as u32,
            })?;

        let resource_type = virt.slot_type();
        if slot_desc.slot_type != resource_type {
            return Err(RenderGraphError::TypeMismatch {
                pass,
                slot,
                expected: slot_desc.slot_type,
                actual: resource_type,
            });
        }

        self.bindings.push(SlotBinding {
            pass,
            slot,
            resource,
        });
        Ok(())
    }

    /// Returns the number of passes in the graph.
    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }

    /// Returns the number of virtual resources in the graph.
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RenderGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderGraph")
            .field("passes", &self.passes.len())
            .field("resources", &self.resources.len())
            .field("bindings", &self.bindings.len())
            .finish()
    }
}

// ── Errors ──────────────────────────────────────────────────────────

/// An error that can occur when constructing or validating a render graph.
#[derive(Debug, Display)]
pub enum RenderGraphError {
    #[display("invalid pass id: {_0:?}")]
    InvalidPassId(PassId),
    #[display("invalid resource id: {_0:?}")]
    InvalidResourceId(VirtualResourceId),
    #[display("slot index {slot} out of bounds for pass {pass:?} (has {slot_count} slots)")]
    SlotIndexOutOfBounds {
        pass: PassId,
        slot: u32,
        slot_count: u32,
    },
    #[display("type mismatch on pass {pass:?} slot {slot}: expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        pass: PassId,
        slot: u32,
        expected: ResourceSlotType,
        actual: ResourceSlotType,
    },
}

impl std::error::Error for RenderGraphError {}
