//! Bounded arena growth used by the save serializer (IR-5.10.1.5).

use crate::error::SaveError;

/// Byte budget helper for save serialization retries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SaveArena {
    capacity_bytes: usize,
}

impl SaveArena {
    /// Creates an arena with the given initial capacity in bytes.
    pub fn new(capacity_bytes: usize) -> Self {
        Self { capacity_bytes }
    }

    /// Current capacity in bytes.
    pub fn capacity_bytes(&self) -> usize {
        self.capacity_bytes
    }

    /// Reserves `required` bytes for the next serialization attempt.
    ///
    /// Returns [`SaveError::ArenaOverflow`] when `required` exceeds the current
    /// capacity. Callers double the capacity (bounded by policy) and retry.
    pub fn try_reserve(&self, required: usize) -> Result<(), SaveError> {
        if required > self.capacity_bytes {
            return Err(SaveError::ArenaOverflow);
        }
        Ok(())
    }

    /// Grows the arena to `next_capacity` bytes.
    pub fn grow_to(&mut self, next_capacity: usize) {
        self.capacity_bytes = next_capacity;
    }
}
