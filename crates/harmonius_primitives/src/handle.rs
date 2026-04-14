//! Type-safe generational handles.

use core::marker::PhantomData;

/// Generational handle parameterized by the resource type.
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
pub struct Handle<T: 'static> {
    /// Slot index.
    pub index: u32,
    /// Generation counter for stale detection.
    pub generation: u32,
    pub(crate) _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> Handle<T> {
    /// Sentinel handle that never refers to live storage.
    pub const NULL: Self = Self {
        index: u32::MAX,
        generation: u32::MAX,
        _marker: PhantomData,
    };

    /// Returns `true` when this handle is the reserved null pattern.
    #[must_use]
    pub fn is_null(self) -> bool {
        self.index == Self::NULL.index && self.generation == Self::NULL.generation
    }
}
