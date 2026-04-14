/// Generational handle into the BVH leaf table.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BvhHandle {
    index: u32,
    generation: u32,
}

impl BvhHandle {
    pub(crate) const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Slot index inside the handle table.
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
