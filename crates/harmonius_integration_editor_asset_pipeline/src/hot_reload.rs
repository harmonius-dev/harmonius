//! Hot reload barrier and request channel stubs for IR-9.2.3 and FM-4 / FM-5.

use std::collections::VecDeque;

/// Coordinates parking editor + game loop before swapping baked assets (IR-9.2.3).
#[derive(Clone, Debug, Default)]
pub struct HotReloadBarrier {
    /// Increments once per successful coordinated swap (both sides parked).
    pub park_cycles: u64,
    swap_allowed: bool,
}

impl HotReloadBarrier {
    /// Creates a barrier that completes swaps normally.
    pub fn new() -> Self {
        Self {
            park_cycles: 0,
            swap_allowed: true,
        }
    }

    /// Barrier that always times out (FM-4).
    pub fn always_times_out() -> Self {
        Self {
            park_cycles: 0,
            swap_allowed: false,
        }
    }

    /// Runs `swap` after recording a park cycle, unless this barrier is configured to time out.
    pub fn try_swap(&mut self, swap: impl FnOnce()) -> bool {
        if !self.swap_allowed {
            return false;
        }
        self.park_cycles += 1;
        swap();
        true
    }
}

/// Bounded hot reload request queue (CH-20 cap semantics, FM-5).
#[derive(Clone, Debug)]
pub struct HotReloadReqChannel {
    cap: usize,
    queue: VecDeque<u32>,
    /// Increments each time an enqueue had to wait while the channel was at capacity.
    pub fm5_backpressure_events: u64,
}

impl HotReloadReqChannel {
    /// Builds a channel with the documented CH-20 capacity of 16.
    pub fn with_cap(cap: usize) -> Self {
        Self {
            cap,
            queue: VecDeque::new(),
            fm5_backpressure_events: 0,
        }
    }

    /// Enqueues `id`, simulating main-thread drain whenever the channel is already at capacity.
    pub fn enqueue_with_cooperative_drain(&mut self, id: u32) {
        while self.queue.len() >= self.cap {
            self.fm5_backpressure_events += 1;
            self.queue.pop_front();
        }
        self.queue.push_back(id);
    }

    /// Drains one slot (main thread draining CH-20).
    pub fn drain_one(&mut self) -> Option<u32> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
