//! Core render graph identifiers and enumerations.

/// Stable index for a pass in a [`crate::RenderGraph`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PassHandle(pub u32);

/// Stable index for a resource node.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId(pub u32);

/// Optional hardware capability gates on a pass.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Capability {
    /// Ray tracing pipelines.
    RayTracing,
}

/// Preferred execution queue for a pass.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QueueAffinity {
    /// Graphics queue.
    Graphics,
    /// Async compute queue.
    AsyncCompute,
    /// Copy queue.
    Transfer,
    /// Let compiler pick.
    Any,
}

/// Pass scheduling priority for budget culling.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PassPriority {
    /// Culled first when over budget.
    Optional,
    /// Low importance.
    Low,
    /// Default.
    Normal,
    /// High importance.
    High,
    /// Never culled by budget.
    Required,
}

/// Lifetime class for aliasing analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceLifetime {
    /// Exists only between first and last pass reference in a frame.
    Transient {
        /// First pass index in declaration order touching the resource.
        first_pass: u32,
        /// Last pass index in declaration order touching the resource.
        last_pass: u32,
        /// Size in bytes for VRAM accounting.
        size: u64,
    },
}
