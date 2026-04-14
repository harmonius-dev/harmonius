//! Dense map keyed by [`Handle`](super::Handle).

use super::Handle;

/// Lookup failure for a [`Handle`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandleMapError {
    /// Handle generation does not match the slot.
    GenerationMismatch {
        /// Generation stored in the slot.
        expected: u32,
        /// Generation carried by the handle.
        actual: u32,
    },
}

struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// Map of `T` keyed by stable [`Handle<T>`] values.
pub struct HandleMap<T> {
    entries: Vec<Slot<T>>,
    free: Vec<u32>,
}

impl<T> Default for HandleMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HandleMap<T> {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free: Vec::new(),
        }
    }

    /// Inserts `value` and returns a fresh [`Handle`].
    pub fn insert(&mut self, value: T) -> Handle<T> {
        if let Some(idx) = self.free.pop() {
            let slot = &mut self.entries[idx as usize];
            slot.value = Some(value);
            return Handle {
                index: idx,
                generation: slot.generation,
                _marker: core::marker::PhantomData,
            };
        }
        let idx = u32::try_from(self.entries.len()).expect("handle map capacity");
        self.entries.push(Slot {
            generation: 0,
            value: Some(value),
        });
        Handle {
            index: idx,
            generation: 0,
            _marker: core::marker::PhantomData,
        }
    }

    /// Removes `handle` if it is live, bumping the slot generation.
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        if handle.is_null() {
            return None;
        }
        let slot = self.entries.get_mut(handle.index as usize)?;
        if slot.generation != handle.generation || slot.value.is_none() {
            return None;
        }
        let v = slot.value.take().expect("checked");
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(handle.index);
        Some(v)
    }

    /// Borrows `handle` when it matches the stored generation.
    #[must_use]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.entries.get(handle.index as usize).and_then(|s| {
            if s.generation == handle.generation {
                s.value.as_ref()
            } else {
                None
            }
        })
    }

    /// Validates `handle` against the map without borrowing the value.
    pub fn validate(&self, handle: Handle<T>) -> Result<(), HandleMapError> {
        if handle.is_null() {
            return Err(HandleMapError::GenerationMismatch {
                expected: 0,
                actual: u32::MAX,
            });
        }
        let Some(slot) = self.entries.get(handle.index as usize) else {
            return Err(HandleMapError::GenerationMismatch {
                expected: 0,
                actual: handle.generation,
            });
        };
        if slot.generation != handle.generation {
            return Err(HandleMapError::GenerationMismatch {
                expected: slot.generation,
                actual: handle.generation,
            });
        }
        if slot.value.is_none() {
            return Err(HandleMapError::GenerationMismatch {
                expected: slot.generation,
                actual: handle.generation,
            });
        }
        Ok(())
    }

    /// Number of live entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.iter().filter(|s| s.value.is_some()).count()
    }

    /// Returns `true` when there are no live entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
