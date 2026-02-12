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

/// How a render pass accesses a resource.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceAccess {
    /// The pass only reads from the resource.
    Read,
    /// The pass only writes to the resource.
    Write,
    /// The pass both reads from and writes to the resource.
    ReadWrite,
}

// ── Resource slot types ─────────────────────────────────────────────

/// The type of a resource slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub enum ResourceSlotType {
    Image,
    Buffer,
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

/// A resource binding within a pass: which resource and how it's accessed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PassSlot {
    resource: VirtualResourceId,
    access: ResourceAccess,
}

/// A purely declarative pass: just its ordered resource bindings.
#[derive(Debug, Clone, Default)]
struct PassData {
    slots: Vec<PassSlot>,
}

// ── RenderGraph ─────────────────────────────────────────────────────

/// A multi-stage rendering operation that is compiled into an efficient execution plan.
///
/// # Example
///
/// ```ignore
/// let mut graph = RenderGraph::new();
///
/// let albedo = graph.create_transient(albedo_desc);
/// let depth  = graph.create_transient(depth_desc);
/// let output = graph.import_image();
///
/// let gbuffer = graph.add_pass()
///     .write(albedo)
///     .write(depth)
///     .build();
///
/// let lighting = graph.add_pass()
///     .read(albedo)
///     .read(depth)
///     .write(output)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct RenderGraph {
    passes: Vec<PassData>,
    resources: Vec<VirtualResource>,
}

impl RenderGraph {
    /// Create a new, empty render graph.
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            resources: Vec::new(),
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

    /// Begin declaring a new render pass.
    ///
    /// Use the returned [`PassBuilder`] to declare which resources the pass
    /// reads and writes, then call [`.build()`](PassBuilder::build) to finalize.
    pub fn add_pass(&mut self) -> PassBuilder<'_> {
        let id = PassId(self.passes.len() as u32);
        self.passes.push(PassData::default());
        PassBuilder { graph: self, pass: id }
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

// ── PassBuilder ─────────────────────────────────────────────────────

/// Builder for declaring a render pass's resource bindings.
///
/// Obtained from [`RenderGraph::add_pass`]. Chain `.read()`, `.write()`,
/// and `.read_write()` calls to declare resource access, then call
/// `.build()` to finalize the pass.
pub struct PassBuilder<'a> {
    graph: &'a mut RenderGraph,
    pass: PassId,
}

impl<'a> PassBuilder<'a> {
    /// Declare that this pass reads from `resource`.
    pub fn read(self, resource: VirtualResourceId) -> Self {
        self.push_slot(resource, ResourceAccess::Read)
    }

    /// Declare that this pass writes to `resource`.
    pub fn write(self, resource: VirtualResourceId) -> Self {
        self.push_slot(resource, ResourceAccess::Write)
    }

    /// Declare that this pass both reads from and writes to `resource`.
    pub fn read_write(self, resource: VirtualResourceId) -> Self {
        self.push_slot(resource, ResourceAccess::ReadWrite)
    }

    /// Finalize the pass and return its [`PassId`].
    pub fn build(self) -> PassId {
        self.pass
    }

    fn push_slot(self, resource: VirtualResourceId, access: ResourceAccess) -> Self {
        self.graph.passes[self.pass.0 as usize]
            .slots
            .push(PassSlot { resource, access });
        self
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
}

impl std::error::Error for RenderGraphError {}
