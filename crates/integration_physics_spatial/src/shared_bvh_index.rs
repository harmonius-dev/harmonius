//! Shared gameplay / AI spatial index placeholder (`BvhIndex`).

/// Shared spatial index revision counter (`BvhIndex` stand-in).
///
/// Physics must not mutate this structure when integrating the private `PhysicsBvh`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BvhIndex {
    revision: u64,
}

impl BvhIndex {
    /// Returns the current revision.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// Simulates a consumer-side spatial update (AI/audio/gameplay only).
    pub fn bump(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }
}
