/// Generational entity handle used for grid ownership checks.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity {
    /// Stable row index in an ECS table.
    pub index: u32,
    /// Generation incremented on slot reuse.
    pub generation: u32,
}

impl Entity {
    /// Creates a new entity handle.
    #[must_use]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}
