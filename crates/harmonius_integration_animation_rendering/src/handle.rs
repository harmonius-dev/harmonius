//! Generational [`Handle`] for GPU resources.

use core::marker::PhantomData;

/// Marker type for GPU buffer resources owned by the resource manager arena.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GpuBuffer {}

/// Copyable generational handle into a typed arena (not reference-counted).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    /// Constructs a handle from raw index and generation.
    #[must_use]
    pub const fn from_raw(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            _marker: PhantomData,
        }
    }

    /// Arena index component.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Generation counter component.
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}
