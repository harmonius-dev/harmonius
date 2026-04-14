//! Widget and simulated ECS entity identifiers.

/// Stable handle to a simulated ECS entity used by the UI framework until the engine `Entity`
/// type is wired through from core runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    pub(crate) fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Slot index backing this entity (for tree storage).
    #[must_use]
    pub fn index(self) -> u32 {
        self.index
    }

    /// Generation counter guarding reuse after removal.
    #[must_use]
    pub fn generation(self) -> u32 {
        self.generation
    }
}

/// Widget identity matches the design alias to `Entity`.
pub type WidgetId = Entity;
