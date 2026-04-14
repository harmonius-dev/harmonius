//! Simple deterministic budget enforcement.

/// Tracks reserved units against a hard ceiling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BudgetAllocator {
    ceiling: usize,
    used: usize,
}

impl BudgetAllocator {
    /// Creates a budget allocator with the given ceiling.
    #[must_use]
    pub fn new(ceiling: usize) -> Self {
        Self { ceiling, used: 0 }
    }

    /// Attempts to reserve `n` units.
    pub fn reserve(&mut self, n: usize) -> Option<()> {
        let next = self.used.checked_add(n)?;
        if next > self.ceiling {
            return None;
        }
        self.used = next;
        Some(())
    }

    /// Releases `n` previously reserved units.
    pub fn release(&mut self, n: usize) {
        self.used = self.used.saturating_sub(n);
    }

    /// Returns remaining capacity before the ceiling.
    #[must_use]
    pub fn available(&self) -> usize {
        self.ceiling.saturating_sub(self.used)
    }
}
