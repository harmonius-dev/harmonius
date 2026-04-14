//! Generational handles for assets and playback slots.

use std::marker::PhantomData;

/// Stable reference into an [`crate::assets::AssetStore`].
///
/// Uses a `fn() -> T` marker so `T` never needs to be `Copy`.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Handle<T> {
    /// Slot index.
    pub index: u32,
    /// Generation counter for stale detection.
    pub generation: u32,
    marker: PhantomData<fn() -> T>,
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Handle<T> {
    /// Builds a new typed handle.
    #[must_use]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            marker: PhantomData,
        }
    }
}
