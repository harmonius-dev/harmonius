//! Atomic registry snapshot switching without `Arc` or `HashMap`.

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::registry::TableRegistry;

/// Double-buffered registry snapshots with a single atomic index flip.
#[derive(Debug)]
pub struct AtomicRegistrySwitch {
    slots: [UnsafeCell<TableRegistry>; 2],
    active: AtomicUsize,
}

// SAFETY: Writers synchronize on `swap_after_rebuild` (main thread only in this stub). Readers
// load `active` with `Acquire` and then immutably borrow the chosen slot for the duration of their
// logical snapshot read. Each slot is written only while it is inactive.
unsafe impl Sync for AtomicRegistrySwitch {}

impl AtomicRegistrySwitch {
    /// Builds a switcher with both slots holding identical initial data.
    pub fn new(initial: TableRegistry) -> Self {
        let clone = initial.clone();
        Self {
            slots: [UnsafeCell::new(initial), UnsafeCell::new(clone)],
            active: AtomicUsize::new(0),
        }
    }

    /// Returns an immutable view of the active snapshot.
    pub fn read_snapshot(&self) -> &TableRegistry {
        let idx = self.active.load(Ordering::Acquire) % 2;
        // SAFETY: `idx` is always 0 or 1; no concurrent mutation of the active slot by writers.
        unsafe { &*self.slots[idx].get() }
    }

    /// Writes `updated` into `inactive_idx` and publishes it as active.
    pub fn swap_after_rebuild(&self, inactive_idx: usize, updated: TableRegistry) {
        let idx = inactive_idx % 2;
        // SAFETY: Main-thread bake owns the inactive slot exclusively before the flip.
        unsafe {
            *self.slots[idx].get() = updated;
        }
        self.active.store(idx, Ordering::Release);
    }
}
