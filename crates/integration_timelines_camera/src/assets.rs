//! Minimal asset store backing timeline handles.

use std::fmt;

use crate::handle::Handle;

enum Slot<T> {
    Occupied {
        gen: u32,
        value: T,
    },
    Vacant {
        _next_gen: u32,
    },
}

/// Slot storage for immutable timeline assets.
pub struct AssetStore<T> {
    slots: Vec<Slot<T>>,
}

impl<T> fmt::Debug for AssetStore<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssetStore")
            .field("slot_count", &self.slots.len())
            .finish()
    }
}

impl<T> Default for AssetStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AssetStore<T> {
    /// Returns an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    /// Inserts `value` at a new slot and returns its [`Handle`].
    pub fn insert(&mut self, value: T) -> Handle<T> {
        let index = u32::try_from(self.slots.len()).expect("asset index fits u32");
        self.slots.push(Slot::Occupied { gen: 0, value });
        Handle::new(index, 0)
    }

    /// Frees a slot so the same index bumps generation (dangling handles).
    pub fn free(&mut self, handle: Handle<T>) {
        let Some(slot) = self.slots.get_mut(handle.index as usize) else {
            return;
        };
        if let Slot::Occupied { gen, value: _ } = slot {
            if *gen != handle.generation {
                return;
            }
            let next_gen = gen.saturating_add(1);
            *slot = Slot::Vacant { _next_gen: next_gen };
        }
    }

    /// Resolves `handle` when the generation still matches.
    #[must_use]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        match self.slots.get(handle.index as usize)? {
            Slot::Occupied { gen, value } if *gen == handle.generation => Some(value),
            _ => None,
        }
    }
}
