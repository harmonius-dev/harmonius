//! Slot-based handle table without raw pointers (opaque `usize` tokens).

/// Pending swap at frame boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingSwap {
    /// Slot index.
    pub index: u32,
    /// New opaque pointer token.
    pub new_ptr: usize,
    /// Byte size (test metadata).
    pub new_size: usize,
}

/// Generational slot table.
#[derive(Debug, Default)]
pub struct HandleTable {
    generations: Vec<u32>,
    ptrs: Vec<usize>,
    sizes: Vec<usize>,
}

impl HandleTable {
    /// New empty table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate slot with `ptr` token and size.
    pub fn allocate(&mut self, ptr: usize, data_size: usize) -> (u32, u32) {
        let index = self.generations.len() as u32;
        self.generations.push(1);
        self.ptrs.push(ptr);
        self.sizes.push(data_size);
        (index, 1)
    }

    /// Swap pointer; returns previous (ptr, size).
    pub fn swap_ptr(
        &mut self,
        index: u32,
        new_ptr: usize,
        new_size: usize,
    ) -> Option<(usize, usize)> {
        let i = index as usize;
        let old = (*self.ptrs.get(i)?, *self.sizes.get(i)?);
        *self.ptrs.get_mut(i)? = new_ptr;
        *self.sizes.get_mut(i)? = new_size;
        Some(old)
    }

    /// Resolve if generation matches.
    pub fn resolve(&self, index: u32, generation: u32) -> Option<usize> {
        let i = index as usize;
        if *self.generations.get(i)? != generation {
            return None;
        }
        Some(*self.ptrs.get(i)?)
    }
}

/// Schedules swaps and tracks retired tokens until fence.
#[derive(Debug, Default)]
pub struct SwapScheduler {
    pending: Vec<PendingSwap>,
    retired: Vec<(usize, usize)>,
}

impl SwapScheduler {
    /// New empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueue swap.
    pub fn schedule(&mut self, swap: PendingSwap) {
        self.pending.push(swap);
    }

    /// Apply swaps to `table`; returns count applied.
    pub fn apply_pending_swaps(&mut self, handle_table: &mut HandleTable) -> u32 {
        let mut n = 0u32;
        for s in std::mem::take(&mut self.pending) {
            if let Some(old) = handle_table.swap_ptr(s.index, s.new_ptr, s.new_size) {
                self.retired.push(old);
                n += 1;
            }
        }
        n
    }

    /// Drain retired entries after GPU fence (test no-op).
    pub fn retire_old_assets(&mut self, _completed_fence: u64) {
        self.retired.clear();
    }

    /// Borrow retired (for assertions before clear).
    pub fn retired(&self) -> &[(usize, usize)] {
        &self.retired
    }
}
