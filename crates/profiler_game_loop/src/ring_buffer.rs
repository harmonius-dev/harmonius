use crate::arena::FrameArena;
use crate::cpu_event::CpuEvent;

/// Per-thread buffer storing pending scopes and completed [`CpuEvent`] values.
#[derive(Debug)]
pub struct ProfileRingBuffer {
    max_completed: usize,
    completed: Vec<CpuEvent>,
    pending: Vec<CpuEvent>,
    dropped: bool,
}

impl ProfileRingBuffer {
    /// Creates a buffer that stores at most `max_completed` finished events.
    #[must_use]
    pub fn with_capacity(max_completed: usize) -> Self {
        Self {
            max_completed,
            completed: Vec::new(),
            pending: Vec::new(),
            dropped: false,
        }
    }

    /// Returns `true` if any completed event was dropped due to overflow.
    #[must_use]
    pub fn events_dropped(&self) -> bool {
        self.dropped
    }

    /// Number of completed events currently stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.completed.len()
    }

    /// Returns `true` when there are no completed events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.completed.is_empty()
    }

    /// Pushes a pending (incomplete) event onto the per-thread stack.
    pub fn push_pending(&mut self, event: CpuEvent) {
        self.pending.push(event);
    }

    /// Completes the newest pending event, returning `false` if none is pending.
    pub fn complete_pending(&mut self, end_tsc: u64) -> bool {
        let Some(mut top) = self.pending.pop() else {
            return false;
        };
        if end_tsc < top.begin_tsc {
            top.end_tsc = top.begin_tsc;
            top.flags |= CpuEvent::FLAG_NON_MONOTONIC;
        } else {
            top.end_tsc = end_tsc;
        }
        self.push_completed(top)
    }

    /// Returns the newest pending begin timestamp, if any.
    #[must_use]
    pub fn pending_begin(&self) -> Option<u64> {
        self.pending.last().map(|e| e.begin_tsc)
    }

    /// Pushes a completed event, returning `false` when the ring is full.
    pub fn push_completed(&mut self, event: CpuEvent) -> bool {
        if self.completed.len() >= self.max_completed {
            self.dropped = true;
            return false;
        }
        self.completed.push(event);
        true
    }

    /// Copies completed events into `dst` without clearing this buffer.
    pub fn copy_completed_to(&self, dst: &mut [CpuEvent]) -> usize {
        let n = self.completed.len().min(dst.len());
        dst[..n].copy_from_slice(&self.completed[..n]);
        n
    }

    /// Clears completed events after they were copied out.
    pub fn clear_completed(&mut self) {
        self.completed.clear();
    }

    /// Clears the latched overflow flag after it was folded into [`crate::FrameStats`].
    pub fn clear_dropped_flag(&mut self) {
        self.dropped = false;
    }

    /// Drains completed events into the arena without growing the arena backing store.
    pub fn drain_into<'a>(&mut self, arena: &'a mut FrameArena) -> Option<&'a mut [CpuEvent]> {
        let n = self.completed.len();
        if n == 0 {
            return Some(&mut []);
        }
        let slice = arena.alloc_slice(n)?;
        slice.copy_from_slice(&self.completed);
        self.completed.clear();
        Some(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_6_4_u1_drain_into_copies_without_heap_growth_on_ring() {
        let mut ring = ProfileRingBuffer::with_capacity(1024);
        ring.completed.reserve_exact(1024);
        for i in 0..1024 {
            let ev = CpuEvent {
                begin_tsc: i,
                end_tsc: i + 1,
                thread_id: 1,
                zone_name_hash: 2,
                depth: 0,
                flags: 0,
            };
            assert!(ring.push_completed(ev));
        }
        let cap = 1024 * core::mem::size_of::<CpuEvent>() + 256;
        let mut arena = FrameArena::with_capacity(cap);
        let drained = ring.drain_into(&mut arena).expect("drain");
        assert_eq!(drained.len(), 1024);
        assert_eq!(ring.len(), 0);
    }
}
