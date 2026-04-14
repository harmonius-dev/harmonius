//! Per-entity propagation result storage with partitioned `UnsafeCell` slots.
//!
//! # Safety contract
//!
//! The ECS job system must guarantee **at most one concurrent writer per entity index**. Readers may
//! observe tearing if they read while a writer updates the same slot; tests in this crate access
//! slots sequentially after parallel writers finish.

use std::cell::UnsafeCell;

use crate::{Entity, PropagationResult};

/// Per-entity partitioned storage for propagation results.
pub struct PropagationResultStore {
    slots: Vec<UnsafeCell<PropagationResult>>,
}

// SAFETY: Each `UnsafeCell` slot is written by at most one worker for a disjoint entity index, which
// matches the Harmonius job-system contract described in the integration design.
unsafe impl Sync for PropagationResultStore {}

impl PropagationResultStore {
    /// Allocates `capacity` slots initialized to LOS defaults for `Entity(0)` placeholders.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let mut slots = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            slots.push(UnsafeCell::new(PropagationResult::line_of_sight_default(
                Entity(0),
            )));
        }
        Self { slots }
    }

    /// Number of allocated slots.
    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Returns `true` when no slots are allocated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Writes `result` into `index` without synchronization.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    ///
    /// # Safety
    ///
    /// Callers must ensure disjoint indices across concurrent writers.
    pub fn write_slot(&self, index: usize, result: PropagationResult) {
        assert!(
            index < self.slots.len(),
            "PropagationResultStore index out of bounds"
        );
        // SAFETY: `index` is validated and callers uphold single-writer-per-slot.
        unsafe {
            *self.slots[index].get() = result;
        }
    }

    /// Reads `index` without synchronization.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    #[must_use]
    pub fn read_slot(&self, index: usize) -> PropagationResult {
        assert!(
            index < self.slots.len(),
            "PropagationResultStore index out of bounds"
        );
        // SAFETY: `index` is validated; tests read after writers join.
        unsafe { (*self.slots[index].get()).clone() }
    }
}
