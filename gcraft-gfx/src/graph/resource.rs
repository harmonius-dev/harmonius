//! Resource type definitions for the render graph.
//!
//! This module defines the handles, descriptors, and metadata types used to
//! track resources (images, buffers, condition flags, draw slots) within a
//! [`RenderGraph`]. Every resource is identified by a lightweight, `Copy`
//! handle that carries a version number for explicit dataflow tracking and a
//! generation tag that ties the handle to the graph instance that created it.

use ash::vk;

// ---------------------------------------------------------------------------
// Versioned handles
// ---------------------------------------------------------------------------

/// Lightweight handle to an image resource managed by the render graph.
///
/// Each write to the resource increments [`version`](Self::version), producing
/// a new handle that encodes the dataflow edge.  The [`generation`](Self::generation)
/// field must match the owning graph's generation (invariant S7).
///
/// Handles are `Copy` but cannot be constructed outside the crate because all
/// fields are `pub(crate)`.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ImageHandle {
    /// Index into the graph's image resource table.
    pub(crate) index: u16,
    /// Monotonically increasing write version.
    pub(crate) version: u16,
    /// Generation of the `RenderGraph` that created this handle.
    pub(crate) generation: u32,
}

/// Lightweight handle to a buffer resource managed by the render graph.
///
/// Semantics are identical to [`ImageHandle`] — see its documentation for
/// details on versioning and generation tracking.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct BufferHandle {
    /// Index into the graph's buffer resource table.
    pub(crate) index: u16,
    /// Monotonically increasing write version.
    pub(crate) version: u16,
    /// Generation of the `RenderGraph` that created this handle.
    pub(crate) generation: u32,
}

// ---------------------------------------------------------------------------
// Resource descriptors
// ---------------------------------------------------------------------------

/// Description of an image resource to be created by the render graph.
///
/// This mirrors the subset of `VkImageCreateInfo` fields that the graph needs
/// in order to allocate (or reuse) a transient image.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImageDesc {
    /// Texel format.
    pub format: vk::Format,
    /// Width × height × depth.
    pub extent: vk::Extent3D,
    /// Number of mip levels.
    pub mip_levels: u32,
    /// Number of array layers.
    pub array_layers: u32,
    /// Multisampling sample count.
    pub samples: vk::SampleCountFlags,
    /// Combined usage flags required by every pass that touches this image.
    pub usage: vk::ImageUsageFlags,
}

impl Default for ImageDesc {
    fn default() -> Self {
        Self {
            format: vk::Format::R8G8B8A8_UNORM,
            extent: vk::Extent3D {
                width: 1,
                height: 1,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        }
    }
}

/// Description of a buffer resource to be created by the render graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BufferDesc {
    /// Size in bytes.
    pub size: vk::DeviceSize,
    /// Combined usage flags required by every pass that touches this buffer.
    pub usage: vk::BufferUsageFlags,
}

// ---------------------------------------------------------------------------
// Subresource range
// ---------------------------------------------------------------------------

/// A mip-level / array-layer subresource range within an image.
///
/// Used for per-mip or per-layer barrier tracking.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct SubresourceRange {
    /// First mip level in the range.
    pub base_mip_level: u32,
    /// Number of mip levels in the range.
    pub mip_count: u32,
    /// First array layer in the range.
    pub base_array_layer: u32,
    /// Number of array layers in the range.
    pub layer_count: u32,
}

impl SubresourceRange {
    /// Returns a range covering every mip level and every array layer
    /// described by `desc`.
    pub fn full(desc: &ImageDesc) -> Self {
        Self {
            base_mip_level: 0,
            mip_count: desc.mip_levels,
            base_array_layer: 0,
            layer_count: desc.array_layers,
        }
    }
}

impl From<SubresourceRange> for vk::ImageSubresourceRange {
    fn from(range: SubresourceRange) -> Self {
        Self {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: range.base_mip_level,
            level_count: range.mip_count,
            base_array_layer: range.base_array_layer,
            layer_count: range.layer_count,
        }
    }
}

// ---------------------------------------------------------------------------
// Condition flags & draw slots
// ---------------------------------------------------------------------------

/// Handle to a boolean condition flag evaluated at graph compile time.
///
/// Condition flags allow passes to be conditionally enabled or disabled
/// without rebuilding the graph structure.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ConditionFlag {
    /// Index into the graph's condition flag table.
    pub(crate) index: u16,
    /// Generation of the `RenderGraph` that created this flag.
    pub(crate) generation: u32,
}

/// Handle to a deferred draw-command slot within a render pass.
///
/// Draw slots let the application record indirect or multi-draw commands
/// into a pass after graph construction.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct DrawSlot {
    /// Index of the owning pass.
    pub(crate) pass_index: u16,
    /// Slot index within that pass.
    pub(crate) slot_index: u16,
    /// Generation of the `RenderGraph` that created this slot.
    pub(crate) generation: u32,
}

// ---------------------------------------------------------------------------
// Graph errors
// ---------------------------------------------------------------------------

/// Errors produced during render graph construction, compilation, or
/// validation.
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    /// A pass attempted to read a resource version that was never written.
    #[error("pass `{pass}` reads resource `{resource}` at version {version} which was never written")]
    ReadOfUnwrittenResource {
        pass: &'static str,
        resource: String,
        version: u16,
    },

    /// A resource was written but is never read and is not marked as an output.
    #[error("resource `{resource}` is written but never read and is not an output")]
    OrphanWrite { resource: String },

    /// Two passes write to the same resource without an explicit ordering.
    #[error("resource `{resource}` has conflicting writes in passes `{pass_a}` and `{pass_b}` with no ordering")]
    UnorderedWrites {
        resource: String,
        pass_a: &'static str,
        pass_b: &'static str,
    },

    /// A dependency cycle was detected among passes.
    #[error("dependency cycle detected involving passes: {cycle:?}")]
    DependencyCycle { cycle: Vec<&'static str> },

    /// A handle's generation does not match the current graph generation.
    #[error("handle generation mismatch: handle is from generation {handle_gen}, graph is at {graph_gen}")]
    StaleHandle { handle_gen: u32, graph_gen: u32 },

    /// No graphics-capable queue was available.
    #[error("no graphics queue available")]
    NoGraphicsQueue,

    /// An async-compute pass was requested but no dedicated compute queue exists.
    #[error("async compute pass `{pass}` requested but no dedicated compute queue available")]
    NoComputeQueue { pass: &'static str },

    /// A draw slot in a pass was not filled before compilation.
    #[error("draw slot in pass `{pass}` (index {slot_index}) was not filled before compile")]
    UnfilledDrawSlot {
        pass: &'static str,
        slot_index: u16,
    },

    /// A command inside a pass references a buffer that was not declared in
    /// the pass's resource accesses.
    #[error("pass `{pass}` command references buffer `{resource}` not declared in resource accesses")]
    UndeclaredCommandResource {
        pass: &'static str,
        resource: String,
    },

    /// A command is not supported on the queue type assigned to its pass.
    ///
    /// The `queue` field is a pre-formatted `String` to avoid coupling this
    /// module to `graph::pass::QueueType`.
    #[error("pass `{pass}` on {queue:?} queue contains `{command}` which is not supported on that queue")]
    CommandQueueMismatch {
        pass: &'static str,
        queue: String,
        command: &'static str,
    },

    /// A resource is used in a way that requires a usage flag not present in
    /// its descriptor.
    #[error("resource `{resource}` used as {usage} but descriptor lacks required usage flag")]
    MissingUsageFlags {
        resource: String,
        usage: &'static str,
    },

    /// A pass writes the same resource more than once.
    #[error("pass `{pass}` writes resource `{resource}` more than once")]
    DuplicateWrite {
        pass: &'static str,
        resource: String,
    },
}

// ---------------------------------------------------------------------------
// Internal resource metadata
// ---------------------------------------------------------------------------

/// Whether a resource is transient (graph-managed lifetime) or imported
/// (externally owned, just tracked for barriers).
#[derive(Clone, Debug)]
pub(crate) enum ResourceKind {
    /// Allocated and freed by the graph's resource pool.
    Transient,
    /// Owned externally; the graph only inserts barriers.
    Imported,
}

/// Common bookkeeping for any resource tracked by the graph.
#[derive(Clone, Debug)]
pub(crate) struct ResourceInfo {
    /// Human-readable name for debug messages and error reporting.
    pub name: &'static str,
    /// Whether the resource is transient or imported.
    pub kind: ResourceKind,
    /// Latest write version (starts at 0 for the initial declaration).
    pub current_version: u16,
    /// Optional subresource range for per-mip/per-layer views.
    pub subresource: Option<SubresourceRange>,
    /// Index of the parent resource when this entry represents a sub-resource
    /// view rather than a top-level resource.
    pub parent_index: Option<u16>,
}

/// Internal metadata for an image resource.
#[derive(Clone, Debug)]
pub(crate) struct ImageInfo {
    /// Image descriptor (format, extent, usage, etc.).
    pub desc: ImageDesc,
    /// Common resource bookkeeping.
    pub resource: ResourceInfo,
}

/// Internal metadata for a buffer resource.
#[derive(Clone, Debug)]
pub(crate) struct BufferInfo {
    /// Buffer descriptor (size, usage).
    pub desc: BufferDesc,
    /// Common resource bookkeeping.
    pub resource: ResourceInfo,
}

/// Internal metadata for a boolean condition flag.
#[derive(Clone, Debug)]
pub(crate) struct FlagInfo {
    /// Human-readable name for debug messages.
    pub name: &'static str,
    /// Current value of the flag.
    pub value: bool,
}
