//! Untyped generational index shared by multiple containers.

/// Untyped generational index; [`crate::Handle`] is a newtype wrapper over this pattern.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    rkyv_derive::Archive,
    rkyv_derive::Deserialize,
    rkyv_derive::Serialize,
)]
pub struct GenerationalIndex {
    /// Slot index.
    pub index: u32,
    /// Generation counter for stale detection.
    pub generation: u32,
}

impl GenerationalIndex {
    /// Returns `true` when this index matches the reserved null pattern.
    #[must_use]
    pub fn is_null(self) -> bool {
        self.index == u32::MAX && self.generation == u32::MAX
    }
}
