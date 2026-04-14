//! Entity identifiers for propagation bookkeeping.

/// Opaque entity identifier (index space for integration tests).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(pub u32);
