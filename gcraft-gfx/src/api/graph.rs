use derive_more::{Display, From};

use crate::{Format, PipelineHandle};

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

// ── Pass dispatch ───────────────────────────────────────────────────

/// How a pass dispatches GPU work.
///
/// Mesh shader variants map to `vkCmdDrawMeshTasksEXT` and friends.
/// Compute variants map to `vkCmdDispatch` and friends.
/// Indirect command/count buffers are automatically tracked as read
/// dependencies when set through the [`PassBuilder`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PassDispatch {
    /// No dispatch specified.
    #[default]
    None,
    /// Dispatch mesh shader work groups directly.
    MeshDirect {
        group_count: [u32; 3],
    },
    /// Draw via mesh shaders from an indirect command buffer.
    ///
    /// Each entry in the buffer is a `DrawMeshTasksIndirectCommandEXT`
    /// (groupCountX, groupCountY, groupCountZ).
    MeshIndirect {
        /// Buffer of `DrawMeshTasksIndirectCommandEXT` entries.
        commands: VirtualResourceId,
        /// Maximum number of draws the GPU may issue.
        max_draw_count: u32,
    },
    /// Draw via mesh shaders with a GPU-driven draw count.
    MeshIndirectCount {
        /// Buffer of `DrawMeshTasksIndirectCommandEXT` entries.
        commands: VirtualResourceId,
        /// Buffer containing the actual draw count (`u32`).
        count: VirtualResourceId,
        /// Maximum number of draws the GPU may issue.
        max_draw_count: u32,
    },
    /// Dispatch compute work groups directly.
    Compute {
        group_count: [u32; 3],
    },
    /// Dispatch compute work groups from an indirect command buffer.
    ComputeIndirect {
        /// Buffer of `DispatchIndirectCommand` entries.
        commands: VirtualResourceId,
    },
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
    /// A buffer filled from CPU data before graph execution (host → device transfer).
    Upload { size: u32 },
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
            VirtualResource::Upload { .. } => ResourceSlotType::Buffer,
        }
    }
}

/// A resource binding within a pass: which resource and how it's accessed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PassSlot {
    resource: VirtualResourceId,
    access: ResourceAccess,
}

/// A purely declarative pass: resource bindings, pipeline, and dispatch.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct PassData {
    slots: Vec<PassSlot>,
    pipeline: Option<PipelineHandle>,
    dispatch: PassDispatch,
}

// ── RenderGraph ─────────────────────────────────────────────────────

/// A multi-stage rendering operation that is compiled into an efficient execution plan.
///
/// # Example
///
/// ```ignore
/// let mut graph = RenderGraph::new();
///
/// // CPU → GPU: scene object list uploaded each frame
/// let scene_objects = graph.upload(max_objects * OBJECT_STRIDE);
///
/// // Transient GPU resources
/// let draw_cmds  = graph.create_transient(indirect_buf_desc);
/// let draw_count = graph.create_transient(count_buf_desc);
/// let albedo     = graph.create_transient(albedo_desc);
/// let depth      = graph.create_transient(depth_desc);
/// let output     = graph.import_image(); // swapchain
///
/// // GPU culling — reads scene objects, fills indirect draw buffer
/// let _cull = graph.add_pass()
///     .read(scene_objects)
///     .write(draw_cmds)
///     .write(draw_count)
///     .pipeline(cull_pipeline)
///     .compute([num_groups, 1, 1])
///     .build();
///
/// // G-buffer — mesh shader indirect with GPU-driven count
/// let _gbuffer = graph.add_pass()
///     .write(albedo)
///     .write(depth)
///     .pipeline(gbuffer_pipeline)
///     .mesh_indirect_count(draw_cmds, draw_count, MAX_DRAWS)
///     .build();
///
/// // Lighting — fullscreen mesh shader triangle
/// let _lighting = graph.add_pass()
///     .read(albedo)
///     .read(depth)
///     .write(output)
///     .pipeline(lighting_pipeline)
///     .mesh_direct([1, 1, 1])
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    /// Declare a buffer that will be filled from CPU data at execution time.
    ///
    /// The executor handles the host-to-device transfer before any pass that
    /// reads from this resource. Use this for per-frame scene data (object
    /// lists, transforms, etc.) that originates on the CPU.
    pub fn upload(&mut self, size: u32) -> VirtualResourceId {
        let id = VirtualResourceId(self.resources.len() as u32);
        self.resources.push(VirtualResource::Upload { size });
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

/// Builder for declaring a render pass's resource bindings and dispatch.
///
/// Obtained from [`RenderGraph::add_pass`]. Chain `.read()`, `.write()`,
/// `.read_write()` to declare resource access, set a `.pipeline()` and
/// dispatch method, then call `.build()` to finalize.
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

    /// Set the pipeline for this pass.
    pub fn pipeline(self, pipeline: PipelineHandle) -> Self {
        self.graph.passes[self.pass.0 as usize].pipeline = Some(pipeline);
        self
    }

    /// Dispatch mesh shader work groups directly.
    pub fn mesh_direct(self, group_count: [u32; 3]) -> Self {
        self.set_dispatch(PassDispatch::MeshDirect { group_count })
    }

    /// Draw via mesh shaders from an indirect command buffer.
    ///
    /// Implicitly adds a read dependency on `commands`.
    pub fn mesh_indirect(self, commands: VirtualResourceId, max_draw_count: u32) -> Self {
        self.push_slot(commands, ResourceAccess::Read)
            .set_dispatch(PassDispatch::MeshIndirect {
                commands,
                max_draw_count,
            })
    }

    /// Draw via mesh shaders with a GPU-driven draw count.
    ///
    /// Implicitly adds read dependencies on `commands` and `count`.
    pub fn mesh_indirect_count(
        self,
        commands: VirtualResourceId,
        count: VirtualResourceId,
        max_draw_count: u32,
    ) -> Self {
        self.push_slot(commands, ResourceAccess::Read)
            .push_slot(count, ResourceAccess::Read)
            .set_dispatch(PassDispatch::MeshIndirectCount {
                commands,
                count,
                max_draw_count,
            })
    }

    /// Dispatch compute work groups directly.
    pub fn compute(self, group_count: [u32; 3]) -> Self {
        self.set_dispatch(PassDispatch::Compute { group_count })
    }

    /// Dispatch compute work groups from an indirect command buffer.
    ///
    /// Implicitly adds a read dependency on `commands`.
    pub fn compute_indirect(self, commands: VirtualResourceId) -> Self {
        self.push_slot(commands, ResourceAccess::Read)
            .set_dispatch(PassDispatch::ComputeIndirect { commands })
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

    fn set_dispatch(self, dispatch: PassDispatch) -> Self {
        self.graph.passes[self.pass.0 as usize].dispatch = dispatch;
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
