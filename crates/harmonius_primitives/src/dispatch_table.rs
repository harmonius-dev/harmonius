//! Compact static dispatch keyed by integer IDs.

use std::vec::Vec;

/// Enum-dispatch replacement keyed by a compact `u32` id.
#[derive(Debug)]
pub struct DispatchTable<F> {
    entries: Vec<Option<F>>,
}

impl<F> Default for DispatchTable<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F> DispatchTable<F> {
    /// Creates an empty table.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Registers `f` at `id`, replacing any previous entry.
    pub fn register(&mut self, id: u32, f: F) {
        let idx = id as usize;
        while self.entries.len() <= idx {
            self.entries.push(None);
        }
        self.entries[idx] = Some(f);
    }

    /// Borrows the entry registered at `id`.
    #[must_use]
    pub fn get(&self, id: u32) -> Option<&F> {
        self.entries.get(id as usize)?.as_ref()
    }
}
