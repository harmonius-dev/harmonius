//! Stable entity identifiers used by container and socket tests.

/// Generational or opaque entity handle (ECS-local in full engine).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(pub u64);

impl Entity {
    /// Constructs an entity from a raw id (tests use small integers).
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Raw numeric id for assertions and maps.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }
}
