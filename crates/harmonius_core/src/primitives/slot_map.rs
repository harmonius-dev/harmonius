//! Dense [`Vec`]-backed storage with stable generational [`Handle`](super::Handle) keys.

use super::Handle;

/// Lookup failure for a [`SlotMap`] handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SlotMapError {
    /// Handle generation does not match the slot.
    GenerationMismatch {
        /// Generation stored in the slot.
        expected: u32,
        /// Generation carried by the handle.
        actual: u32,
    },
}

struct SlotMeta {
    generation: u32,
    /// Index in `dense` when occupied.
    dense: Option<usize>,
}

/// Dense slot map with O(1) insert, remove, and lookup.
pub struct SlotMap<T> {
    dense: Vec<T>,
    slots: Vec<SlotMeta>,
    free_slots: Vec<u32>,
}

impl<T> Default for SlotMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SlotMap<T> {
    /// Creates an empty slot map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            slots: Vec::new(),
            free_slots: Vec::new(),
        }
    }

    /// Inserts `value` and returns a [`Handle`].
    pub fn insert(&mut self, value: T) -> Handle<T> {
        if let Some(idx) = self.free_slots.pop() {
            let m = &mut self.slots[idx as usize];
            let dense_i = self.dense.len();
            m.dense = Some(dense_i);
            self.dense.push(value);
            return Handle {
                index: idx,
                generation: m.generation,
                _marker: core::marker::PhantomData,
            };
        }
        let idx = u32::try_from(self.slots.len()).expect("slot map capacity");
        let dense_i = self.dense.len();
        self.slots.push(SlotMeta {
            generation: 0,
            dense: Some(dense_i),
        });
        self.dense.push(value);
        Handle {
            index: idx,
            generation: 0,
            _marker: core::marker::PhantomData,
        }
    }

    /// Removes `handle` when live.
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        if handle.is_null() {
            return None;
        }
        let m = self.slots.get_mut(handle.index as usize)?;
        if m.generation != handle.generation || m.dense.is_none() {
            return None;
        }
        let dense_idx = m.dense.take().expect("dense");
        m.generation = m.generation.wrapping_add(1);
        self.free_slots.push(handle.index);
        let last = self.dense.len() - 1;
        let removed = if dense_idx == last {
            self.dense.pop().expect("elem")
        } else {
            let swapped = self.dense.swap_remove(dense_idx);
            let moved_slot = self
                .slots
                .iter_mut()
                .find(|s| s.dense == Some(last))
                .expect("moved slot");
            moved_slot.dense = Some(dense_idx);
            swapped
        };
        Some(removed)
    }

    /// Borrows the value behind `handle`.
    pub fn get(&self, handle: Handle<T>) -> Result<&T, SlotMapError> {
        if handle.is_null() {
            return Err(SlotMapError::GenerationMismatch {
                expected: 0,
                actual: u32::MAX,
            });
        }
        let Some(m) = self.slots.get(handle.index as usize) else {
            return Err(SlotMapError::GenerationMismatch {
                expected: 0,
                actual: handle.generation,
            });
        };
        let Some(di) = m.dense else {
            return Err(SlotMapError::GenerationMismatch {
                expected: m.generation,
                actual: handle.generation,
            });
        };
        if m.generation != handle.generation {
            return Err(SlotMapError::GenerationMismatch {
                expected: m.generation,
                actual: handle.generation,
            });
        }
        Ok(&self.dense[di])
    }

    /// Contiguous slice of live values in dense order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.dense
    }

    /// Number of live values (equals `as_slice().len()`).
    #[must_use]
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    /// Returns `true` when empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }
}
