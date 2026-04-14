//! Typed buffer pool used for zero-copy style handoff across the I/O boundary.

use crate::primitives::Handle;

/// Marker type for [`Handle`] values referencing [`IoBufferPool`] slots.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IoBufferSlot;

/// Opaque buffer view allocated from an [`IoBufferPool`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IoBuffer {
    pub(crate) handle: Handle<IoBufferSlot>,
    pub(crate) len: u32,
    pub(crate) capacity: u32,
}

impl IoBuffer {
    /// Returns the logical byte length currently exposed to callers.
    #[must_use]
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Returns the backing capacity rounded up to the owning bucket size.
    #[must_use]
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns `true` when `len == 0`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the owning pool slot index (stable across reuse within a generation).
    #[must_use]
    pub fn pool_slot_index(&self) -> u32 {
        self.handle.index()
    }

    pub(crate) fn new(handle: Handle<IoBufferSlot>, len: u32, capacity: u32) -> Self {
        Self {
            handle,
            len,
            capacity,
        }
    }
}

#[derive(Debug)]
struct PoolSlot {
    capacity: usize,
    generation: u32,
    bytes: Vec<u8>,
    in_use: bool,
}

/// Bucketed buffer pool with deterministic reuse semantics.
#[derive(Debug)]
pub struct IoBufferPool {
    slots: Vec<PoolSlot>,
    free_lists: [Vec<u32>; Self::BUCKET_COUNT],
    allocation_count: usize,
}

impl IoBufferPool {
    const BUCKET_CAPS: [usize; 5] = [64, 512, 4096, 65536, 1 << 20];
    const BUCKET_COUNT: usize = Self::BUCKET_CAPS.len();

    /// Constructs an empty pool.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_lists: Default::default(),
            allocation_count: 0,
        }
    }

    /// Returns how many backing allocations have ever been created (excluding reuse from free lists).
    #[must_use]
    pub fn allocation_count(&self) -> usize {
        self.allocation_count
    }

    /// Returns the number of buffers still checked out from the pool.
    #[must_use]
    pub fn leaked_buffer_count(&self) -> usize {
        self.slots.iter().filter(|s| s.in_use).count()
    }

    fn bucket_index_for(min_bytes: usize) -> usize {
        Self::BUCKET_CAPS
            .iter()
            .position(|cap| *cap >= min_bytes)
            .unwrap_or(Self::BUCKET_COUNT - 1)
    }

    fn bucket_capacity(min_bytes: usize) -> usize {
        Self::BUCKET_CAPS[Self::bucket_index_for(min_bytes)]
    }

    /// Acquires a buffer with capacity at least `size` bytes.
    pub fn acquire(&mut self, size: usize) -> IoBuffer {
        let cap = Self::bucket_capacity(size);
        let bucket = Self::bucket_index_for(size);
        if let Some(idx) = self.free_lists[bucket].pop() {
            let slot = &mut self.slots[idx as usize];
            debug_assert!(!slot.in_use);
            slot.in_use = true;
            slot.bytes.resize(slot.capacity, 0);
            return IoBuffer::new(Handle::new(idx, slot.generation), 0, slot.capacity as u32);
        }

        self.allocation_count += 1;
        let idx = self.slots.len() as u32;
        self.slots.push(PoolSlot {
            capacity: cap,
            generation: 0,
            bytes: vec![0; cap],
            in_use: true,
        });
        IoBuffer::new(Handle::new(idx, 0), 0, cap as u32)
    }

    /// Releases a buffer back to the pool for reuse.
    pub fn release(&mut self, buf: IoBuffer) {
        let idx = buf.handle.index() as usize;
        let slot = &mut self.slots[idx];
        debug_assert_eq!(slot.generation, buf.handle.generation());
        debug_assert!(slot.in_use);
        slot.in_use = false;
        slot.generation = slot.generation.wrapping_add(1);
        let bucket = Self::bucket_index_for(slot.capacity);
        self.free_lists[bucket].push(buf.handle.index());
    }

    /// Returns shared access to the currently valid prefix of `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf` is stale or not owned by this pool.
    #[must_use]
    pub fn as_slice(&self, buf: &IoBuffer) -> &[u8] {
        let idx = buf.handle.index() as usize;
        let slot = &self.slots[idx];
        assert_eq!(slot.generation, buf.handle.generation());
        assert!(slot.in_use);
        &slot.bytes[..buf.len as usize]
    }

    /// Returns exclusive mutable access to the full backing storage up to `buf.capacity()`.
    ///
    /// # Panics
    ///
    /// Panics when `buf` is stale or not owned by this pool.
    pub fn as_mut_capacity_slice(&mut self, buf: &mut IoBuffer) -> &mut [u8] {
        let idx = buf.handle.index() as usize;
        let slot = &mut self.slots[idx];
        assert_eq!(slot.generation, buf.handle.generation());
        assert!(slot.in_use);
        &mut slot.bytes[..buf.capacity as usize]
    }

    /// Copies `data` into the start of `buf` storage and sets `buf.len` to `data.len()`.
    pub fn write_all(&mut self, buf: &mut IoBuffer, data: &[u8]) {
        let slice = self.as_mut_capacity_slice(buf);
        assert!(data.len() <= slice.len(), "write exceeds IoBuffer capacity");
        let len = data.len() as u32;
        slice[..data.len()].copy_from_slice(data);
        buf.len = len;
    }

    /// Returns which bucket index `size` maps to (for tests asserting bucket routing).
    #[must_use]
    pub fn bucket_index_for_size(size: usize) -> usize {
        Self::bucket_index_for(size)
    }
}

impl Default for IoBufferPool {
    fn default() -> Self {
        Self::new()
    }
}
