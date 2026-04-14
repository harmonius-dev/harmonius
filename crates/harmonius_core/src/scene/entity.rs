//! Generational entity identifiers.

/// Stable handle to an entity row in [`super::World`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    /// Sentinel value that never matches a live entity.
    pub const INVALID: Self = Self {
        index: u32::MAX,
        generation: u32::MAX,
    };

    pub(crate) fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Dense table index for component storage.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Generation counter used to detect stale handles after despawn.
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}
