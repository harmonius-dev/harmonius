//! Type-safe generational handle.

use core::marker::PhantomData;

/// Generational handle parameterized by the resource type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Handle<T: 'static> {
    /// Dense or sparse slot index.
    pub index: u32,
    /// Generation observed at insert time.
    pub generation: u32,
    #[doc(hidden)]
    pub(crate) _marker: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    /// Sentinel null handle.
    pub const NULL: Self = Self {
        index: u32::MAX,
        generation: u32::MAX,
        _marker: PhantomData,
    };

    /// Returns `true` when this handle is [`Handle::NULL`].
    #[must_use]
    pub const fn is_null(&self) -> bool {
        self.index == u32::MAX && self.generation == u32::MAX
    }

    /// Slot index.
    #[must_use]
    pub const fn index(&self) -> u32 {
        self.index
    }

    /// Generation counter.
    #[must_use]
    pub const fn generation(&self) -> u32 {
        self.generation
    }
}
