//! Lock-free SPSC ring buffer for [`CpuEvent`] samples (design: `ProfileRingBuffer`).
//!
//! # Safety contract (`Sync`)
//!
//! A single producer thread may call [`ProfileRingBuffer::push`]. A single consumer thread may
//! call [`ProfileRingBuffer::drain_into`]. Violating this contract is undefined behavior.

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// One CPU zone sample on the profiling timeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuEvent {
    /// Timestamp counter at scope begin.
    pub begin_tsc: u64,
    /// Timestamp counter at scope end.
    pub end_tsc: u64,
    /// OS or engine thread id for swimlanes.
    pub thread_id: u32,
    /// Stable hash of the zone label (see design: codegen zone table).
    pub zone_name_hash: u32,
    /// Nesting depth for flame graph layout.
    pub depth: u16,
}

/// Scratch storage for [`ProfileRingBuffer::drain_into`]; avoids per-frame heap churn in the
/// collector (design: frame arena).
#[derive(Debug, Default)]
pub struct FrameArena {
    events: Vec<CpuEvent>,
}

impl FrameArena {
    /// Empty arena.
    #[must_use]
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Drop drained events without deallocating capacity.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Events accumulated by the last drain (and any manual pushes — none expected).
    #[must_use]
    pub fn as_slice(&self) -> &[CpuEvent] {
        self.events.as_slice()
    }
}

/// Lock-free ring buffer: one producer, one consumer.
pub struct ProfileRingBuffer {
    capacity: u64,
    mask: u32,
    slots: Box<[UnsafeCell<MaybeUninit<CpuEvent>>]>,
    head: AtomicU64,
    tail: AtomicU64,
    dropped: AtomicBool,
}

// SAFETY: `UnsafeCell` slots are accessed only under the SPSC contract documented on the type.
unsafe impl Sync for ProfileRingBuffer {}

impl std::fmt::Debug for ProfileRingBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfileRingBuffer")
            .field("capacity", &self.capacity)
            .field("len", &self.len())
            .field("events_dropped", &self.events_dropped())
            .finish_non_exhaustive()
    }
}

impl ProfileRingBuffer {
    /// Builds a buffer. `capacity` must be a power of two ≥ 2.
    #[must_use]
    pub fn new(capacity: u32) -> Self {
        assert!(capacity >= 2, "capacity must be at least 2");
        assert!(
            capacity.is_power_of_two(),
            "capacity must be a power of two"
        );
        let cap = u64::from(capacity);
        let mask = capacity - 1;
        let slots: Vec<_> = (0..capacity)
            .map(|_| UnsafeCell::new(MaybeUninit::uninit()))
            .collect();
        Self {
            capacity: cap,
            mask,
            slots: slots.into_boxed_slice(),
            head: AtomicU64::new(0),
            tail: AtomicU64::new(0),
            dropped: AtomicBool::new(false),
        }
    }

    /// Push one event from the producer thread. Returns `false` when the buffer is full.
    #[inline]
    pub fn push(&self, event: CpuEvent) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail.wrapping_sub(head) >= self.capacity {
            self.dropped.store(true, Ordering::Release);
            return false;
        }
        let idx = (tail as u32 & self.mask) as usize;
        // SAFETY: `idx` is in bounds. Only the producer writes slot `idx` while `tail` is not yet
        // published; the consumer will not read it until after `tail` advances (Release below).
        unsafe {
            (*self.slots[idx].get()).write(event);
        }
        self.tail.store(tail + 1, Ordering::Release);
        true
    }

    /// Moves all pending events into `arena` and clears the `events_dropped` latch.
    pub fn drain_into(&self, arena: &mut FrameArena) {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            if head == tail {
                break;
            }
            let idx = (head as u32 & self.mask) as usize;
            // SAFETY: the consumer owns `head` until the matching `head` store; the slot was fully
            // written by the producer before `tail` crossed this index.
            let event = unsafe { (*self.slots[idx].get()).assume_init_read() };
            arena.events.push(event);
            self.head.store(head + 1, Ordering::Release);
        }
        self.dropped.store(false, Ordering::Release);
    }

    /// Count of events not yet drained.
    #[must_use]
    pub fn len(&self) -> u32 {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        u32::try_from(tail.wrapping_sub(head).min(self.capacity)).unwrap_or(u32::MAX)
    }

    /// `true` when [`ProfileRingBuffer::len`] is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether the producer overflowed since the last [`ProfileRingBuffer::drain_into`].
    #[must_use]
    pub fn events_dropped(&self) -> bool {
        self.dropped.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(i: u32) -> CpuEvent {
        CpuEvent {
            begin_tsc: u64::from(i),
            end_tsc: u64::from(i) + 1,
            thread_id: 1,
            zone_name_hash: i,
            depth: 0,
        }
    }

    /// TC-15.5.1.1 #1 — Push 10,000 `CpuEvent` entries, drain all.
    #[test]
    fn tc_15_5_1_1_push_ten_thousand_and_drain() {
        let buf = ProfileRingBuffer::new(16_384);
        for i in 0..10_000 {
            assert!(buf.push(sample(i)), "push {i}");
        }
        let mut arena = FrameArena::new();
        buf.drain_into(&mut arena);
        assert_eq!(arena.as_slice().len(), 10_000);
        assert_eq!(buf.len(), 0);
    }

    /// TC-15.5.1.1 #2 — Push 0 events, drain.
    #[test]
    fn tc_15_5_1_1_push_zero_drain() {
        let buf = ProfileRingBuffer::new(256);
        let mut arena = FrameArena::new();
        buf.drain_into(&mut arena);
        assert!(arena.as_slice().is_empty());
    }

    /// TC-15.5.1.2 #1 — Push `capacity + 1` without draining → `events_dropped()`.
    #[test]
    fn tc_15_5_1_2_overflow_sets_dropped() {
        let cap = 16;
        let buf = ProfileRingBuffer::new(cap);
        for i in 0..cap {
            assert!(buf.push(sample(u32::try_from(i).unwrap())));
        }
        assert!(!buf.events_dropped());
        assert!(!buf.push(sample(999)));
        assert!(buf.events_dropped());
    }

    /// TC-15.5.1.2 #2 — Push `capacity - 1` without draining → not dropped.
    #[test]
    fn tc_15_5_1_2_under_capacity_no_drop_flag() {
        let cap = 16;
        let buf = ProfileRingBuffer::new(cap);
        for i in 0..cap - 1 {
            assert!(buf.push(sample(u32::try_from(i).unwrap())));
        }
        assert!(!buf.events_dropped());
    }
}
