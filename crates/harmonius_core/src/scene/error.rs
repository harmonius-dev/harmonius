//! Scene and hierarchy error types.

/// Errors returned by scene and hierarchy operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SceneError {
    /// Parent pointers form a cycle.
    CyclicHierarchy,
    /// More than one BVH resource was registered.
    DuplicateBvh,
    /// JSON (de)serialization failed.
    Serialization,
    /// A hierarchy command would introduce a cycle.
    HierarchyCycle,
    /// Entity handle is stale or invalid.
    StaleEntity,
}
