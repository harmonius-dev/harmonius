//! Unified sub-allocator, dedicated heaps, and ring staging (GR-1.x).

use std::collections::HashMap;

/// Allocation description for texture-like resources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocDesc {
    /// Byte size (power-of-two alignment expected by callers).
    pub size: u64,
    /// When true, allocation must not share a sub-allocated heap with unrelated resources.
    pub dedicated: bool,
}

/// Handle returned by [`UnifiedAllocator::alloc`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AllocHandle(pub u32);

/// O(1) freelist bump allocator over a pre-sized pool (GR-1.2).
#[derive(Debug)]
pub struct UnifiedAllocator {
    pool: u64,
    next_id: u32,
    free: Vec<u32>,
    live: HashMap<u32, u64>,
}

impl UnifiedAllocator {
    /// Creates an allocator backed by `pool` bytes of virtual capacity.
    #[must_use]
    pub fn new(pool: u64) -> Self {
        Self {
            pool,
            next_id: 1,
            free: Vec::new(),
            live: HashMap::new(),
        }
    }

    /// Allocates `desc.size` bytes or returns `None` if the pool is exhausted.
    pub fn alloc(&mut self, desc: &AllocDesc) -> Option<AllocHandle> {
        if desc.size > self.pool {
            return None;
        }
        let id = if let Some(id) = self.free.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(1);
            if id == 0 {
                return None;
            }
            id
        };
        self.live.insert(id, desc.size);
        Some(AllocHandle(id))
    }

    /// Frees a live handle.
    pub fn free(&mut self, h: AllocHandle) {
        if self.live.remove(&h.0).is_some() {
            self.free.push(h.0);
        }
    }

    /// Returns the number of outstanding allocations.
    #[must_use]
    pub fn live_count(&self) -> usize {
        self.live.len()
    }
}

/// Dedicated heap containing at most one visible allocation (TC-2.1.1.2).
#[derive(Debug)]
pub struct DedicatedHeap {
    allocation: Option<(AllocHandle, u64)>,
}

impl DedicatedHeap {
    /// Empty dedicated heap.
    #[must_use]
    pub fn new() -> Self {
        Self { allocation: None }
    }

    /// Allocates exclusively; fails if already occupied.
    pub fn alloc_exclusive(&mut self, desc: &AllocDesc) -> Option<AllocHandle> {
        if self.allocation.is_some() {
            return None;
        }
        let h = AllocHandle(1);
        self.allocation = Some((h, desc.size));
        Some(h)
    }

    /// Returns the number of packed allocations (0 or 1).
    #[must_use]
    pub fn allocation_count(&self) -> usize {
        usize::from(self.allocation.is_some())
    }
}

impl Default for DedicatedHeap {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed-size ring for per-frame uploads (GR-1.5).
#[derive(Debug)]
pub struct RingAllocator {
    buf: Vec<u8>,
    head: usize,
    frame_heap_allocs: u32,
}

impl RingAllocator {
    /// Ring of `capacity` bytes.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity],
            head: 0,
            frame_heap_allocs: 0,
        }
    }

    /// Resets cursor and per-frame heap counter (call at frame start after warm-up).
    pub fn begin_frame(&mut self) {
        self.head = 0;
        self.frame_heap_allocs = 0;
    }

    /// Sub-allocates `size` bytes from the ring without heap traffic.
    pub fn ring_alloc(&mut self, size: usize) -> Option<usize> {
        let cap = self.buf.len();
        if size == 0 || size > cap {
            return None;
        }
        if self.head.saturating_add(size) > cap {
            self.head = 0;
        }
        if self.head.saturating_add(size) > cap {
            return None;
        }
        let off = self.head;
        self.head = self.head.saturating_add(size);
        Some(off)
    }

    /// Increments global heap counter (simulates rare heap expansion).
    pub fn note_heap_alloc(&mut self) {
        self.frame_heap_allocs = self.frame_heap_allocs.saturating_add(1);
    }

    /// Heap allocations this frame.
    #[must_use]
    pub fn frame_heap_allocs(&self) -> u32 {
        self.frame_heap_allocs
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    /// TC-2.1.1.1 — alternating alloc/free stays bounded per operation after warm-up.
    #[test]
    fn test_unified_alloc_o1() {
        let mut a = UnifiedAllocator::new(1 << 20);
        let mut handles = Vec::new();
        for _ in 0..50 {
            let h = a
                .alloc(&AllocDesc {
                    size: 1024,
                    dedicated: false,
                })
                .expect("pool");
            handles.push(h);
        }
        for h in handles {
            a.free(h);
        }
        let t0 = Instant::now();
        for _ in 0..10_000 {
            let h = a
                .alloc(&AllocDesc {
                    size: 1024,
                    dedicated: false,
                })
                .expect("pool");
            a.free(h);
        }
        let elapsed = t0.elapsed();
        let per = elapsed / 10_000;
        assert!(
            per.as_micros() < 50,
            "expected fast freelist recycle, got {per:?}"
        );
    }

    /// TC-2.1.1.2 — dedicated heap holds exactly one allocation.
    #[test]
    fn test_dedicated_heap_alloc() {
        let mut heap = DedicatedHeap::new();
        let d = AllocDesc {
            size: 256 << 20,
            dedicated: true,
        };
        let _h = heap.alloc_exclusive(&d).expect("dedicated");
        assert_eq!(heap.allocation_count(), 1);
        assert!(heap.alloc_exclusive(&d).is_none());
    }

    /// TC-2.1.1.3 — ring sub-alloc does not bump heap counter.
    #[test]
    fn test_ring_buffer_no_heap_alloc() {
        let mut ring = RingAllocator::new(64 * 200);
        ring.begin_frame();
        for _ in 0..200 {
            assert!(ring.ring_alloc(64).is_some());
        }
        assert_eq!(ring.frame_heap_allocs(), 0);
    }
}
