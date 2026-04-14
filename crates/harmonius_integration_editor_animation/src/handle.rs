//! Generational [`Handle`] values for indirect references without `Arc`.

use std::marker::PhantomData;

/// Stable indirect reference into a generational slot table.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
    marker: PhantomData<*const T>,
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Handle<T> {
    /// Constructs a handle from raw parts (tests and deserialization seams).
    #[must_use]
    pub const fn from_raw_parts(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            marker: PhantomData,
        }
    }

    /// Slot index inside the owning table.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Generation counter for the slot.
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}
