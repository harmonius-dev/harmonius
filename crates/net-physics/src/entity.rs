use rkyv_derive::{Archive, Deserialize, Serialize};

/// Stable entity identifier used by networking and physics integration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    /// Constructs a new entity handle.
    #[must_use]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Entity index within its generation lane.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Generation counter used to detect stale handles.
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}
