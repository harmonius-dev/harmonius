//! Fixed-size block pool allocator.

use super::tag::AllocationTag;

/// Configuration for [`PoolAllocator`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PoolConfig {
    /// Initial number of blocks.
    pub initial_count: u32,
    /// Maximum number of blocks.
    pub max_count: u32,
    /// Tag for accounting.
    pub tag: AllocationTag,
}

/// Opaque index of an allocated pool block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PoolSlot {
    index: u32,
}

impl PoolSlot {
    /// Returns the dense block index.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }
}

/// Fixed-size block pool with O(1) alloc and free.
pub struct PoolAllocator<T> {
    slots: Vec<Option<T>>,
    free: Vec<u32>,
    max_count: u32,
    /// Tag carried for diagnostics and tests.
    pub tag: AllocationTag,
}

impl<T> PoolAllocator<T> {
    /// Creates a pool with `config` capacity.
    #[must_use]
    pub fn new(config: PoolConfig) -> Self {
        let n = config.initial_count.min(config.max_count) as usize;
        let slots: Vec<Option<T>> = (0..n).map(|_| None).collect();
        let free: Vec<u32> = (0..n as u32).rev().collect();
        Self {
            slots,
            free,
            max_count: config.max_count,
            tag: config.tag,
        }
    }

    /// Allocates one block storing `value`.
    pub fn alloc(&mut self, value: T) -> Option<PoolSlot> {
        if let Some(idx) = self.free.pop() {
            self.slots[idx as usize] = Some(value);
            return Some(PoolSlot { index: idx });
        }
        if self.slots.len() >= self.max_count as usize {
            return None;
        }
        let idx = u32::try_from(self.slots.len()).expect("pool index");
        self.slots.push(Some(value));
        Some(PoolSlot { index: idx })
    }

    /// Borrows `slot`.
    #[must_use]
    pub fn get(&self, slot: PoolSlot) -> Option<&T> {
        self.slots.get(slot.index as usize)?.as_ref()
    }

    /// Mutably borrows `slot`.
    pub fn get_mut(&mut self, slot: PoolSlot) -> Option<&mut T> {
        self.slots.get_mut(slot.index as usize)?.as_mut()
    }

    /// Returns `slot` to the free list.
    pub fn dealloc(&mut self, slot: PoolSlot) {
        let i = slot.index as usize;
        if i < self.slots.len() && self.slots[i].is_some() {
            self.slots[i] = None;
            self.free.push(slot.index);
        }
    }

    /// Number of live blocks.
    #[must_use]
    pub fn used_count(&self) -> u32 {
        self.slots.iter().filter(|s| s.is_some()).count() as u32
    }

    /// Maximum block capacity.
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        self.max_count
    }

    /// Total reserved slots (including free list holes).
    #[must_use]
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    /// Total bytes reserved for block storage (including `Option` discriminants).
    #[must_use]
    pub fn total_memory_bytes(&self) -> usize {
        self.slots.len() * std::mem::size_of::<Option<T>>()
    }
}
