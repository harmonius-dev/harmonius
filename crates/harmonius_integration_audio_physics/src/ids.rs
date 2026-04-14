//! Stable identifiers for entities, assets, and voices.

use core::marker::PhantomData;

/// Opaque entity id from the ECS.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Entity(pub u32);

/// Typed asset handle.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AssetHandle<T> {
    /// Stable asset index for tests and staging.
    pub id: u32,
    _marker: PhantomData<T>,
}

impl<T> AssetHandle<T> {
    /// Constructs a handle from a raw asset index.
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

/// Marker type for [`AssetHandle`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AudioClip;

/// Voice slot identifier allocated by [`VoiceIdAllocator`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VoiceId(pub u32);

/// Allocates monotonic transient [`VoiceId`] values for the bridge hot path.
#[derive(Debug, Default)]
pub struct VoiceIdAllocator {
    next: u32,
}

impl VoiceIdAllocator {
    /// Returns the next transient voice id.
    pub fn transient(&mut self) -> VoiceId {
        let id = self.next;
        self.next = self.next.wrapping_add(1);
        VoiceId(id)
    }
}
