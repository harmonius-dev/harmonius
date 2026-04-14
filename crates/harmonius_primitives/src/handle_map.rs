//! Dense map keyed by [`crate::Handle`].

use crate::handle::Handle;
use std::vec::Vec;

struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// Dense map of `T` keyed by [`Handle<T>`] with O(1) insert/remove/get.
pub struct HandleMap<T> {
    entries: Vec<Slot<T>>,
    free: Vec<u32>,
    /// Monotonic insert counter retained for diagnostics and forward-compat with the design doc.
    generation_counter: u32,
}

impl<T: 'static> Default for HandleMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> core::fmt::Debug for HandleMap<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let len = self.entries.len().saturating_sub(self.free.len());
        f.debug_struct("HandleMap")
            .field("len", &len)
            .field("entries_len", &self.entries.len())
            .field("free_len", &self.free.len())
            .finish_non_exhaustive()
    }
}

impl<T: 'static> HandleMap<T> {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty map with reserved capacity for `n` live entries.
    #[must_use]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            entries: Vec::with_capacity(n),
            free: Vec::new(),
            generation_counter: 0,
        }
    }

    /// Inserts `value` and returns a fresh [`Handle`].
    pub fn insert(&mut self, value: T) -> Handle<T> {
        self.generation_counter = self.generation_counter.wrapping_add(1);
        if let Some(i) = self.free.pop() {
            let slot = &mut self.entries[i as usize];
            debug_assert!(slot.value.is_none());
            slot.value = Some(value);
            Handle {
                index: i,
                generation: slot.generation,
                _marker: core::marker::PhantomData,
            }
        } else {
            let i = u32::try_from(self.entries.len()).expect("HandleMap capacity overflow");
            self.entries.push(Slot {
                generation: 0,
                value: Some(value),
            });
            Handle {
                index: i,
                generation: 0,
                _marker: core::marker::PhantomData,
            }
        }
    }

    /// Removes `handle`'s value when the generation matches.
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        let slot = self.entries.get_mut(handle.index as usize)?;
        if slot.generation != handle.generation {
            return None;
        }
        let value = slot.value.take()?;
        self.free.push(handle.index);
        slot.generation = slot.generation.wrapping_add(1);
        Some(value)
    }

    /// Borrows `handle`'s value when the generation matches.
    #[must_use]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        let slot = self.entries.get(handle.index as usize)?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_ref()
    }

    /// Mutably borrows `handle`'s value when the generation matches.
    #[must_use]
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let slot = self.entries.get_mut(handle.index as usize)?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_mut()
    }

    /// Returns the number of live entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len().saturating_sub(self.free.len())
    }

    /// Returns `true` when there are no live entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates `(handle, value)` pairs for live entries in index order.
    pub fn iter(&self) -> impl Iterator<Item = (Handle<T>, &T)> + '_ {
        self.entries.iter().enumerate().filter_map(|(i, slot)| {
            let value = slot.value.as_ref()?;
            let handle = Handle {
                index: u32::try_from(i).ok()?,
                generation: slot.generation,
                _marker: core::marker::PhantomData,
            };
            Some((handle, value))
        })
    }

    /// Test hook: forces the generation stored for `index` (must be occupied or reserved).
    #[doc(hidden)]
    pub fn set_slot_generation_for_test(&mut self, index: u32, generation: u32) {
        if let Some(slot) = self.entries.get_mut(index as usize) {
            slot.generation = generation;
        }
    }
}
