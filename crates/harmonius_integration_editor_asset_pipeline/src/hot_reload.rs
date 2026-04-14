//! Hot reload barrier and request channel stubs for IR-9.2.3 and FM-4 / FM-5.
//!
//! `HotReloadBarrier` records a single park cycle per successful swap. A full dual-park protocol
//! between editor and PIE threads is deferred until the editor viewport wires this crate into the
//! runtime (see `editor-asset-pipeline.md`).

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

    /// Enqueues `id` when below capacity; otherwise counts FM-5 and returns without enqueueing.
    ///
    /// Callers model the main thread draining via [`Self::drain_one`] before retrying bursts.
    pub fn enqueue_with_cooperative_drain(&mut self, id: u32) {
        if self.queue.len() >= self.cap {
            self.fm5_backpressure_events += 1;
            return;
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

/// Main → editor hot reload completion queue (`HotReloadResultCh`, CH-21).
#[derive(Clone, Debug, Default)]
pub struct HotReloadResultChannel {
    queue: VecDeque<HotReloadSwapOk>,
}

/// Minimal completion token after a runtime swap attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HotReloadSwapOk {
    pub asset_index: u32,
}

impl HotReloadResultChannel {
    /// Pushes a completion record (bounded only by memory in this harness stub).
    pub fn push_ok(&mut self, asset_index: u32) {
        self.queue.push_back(HotReloadSwapOk { asset_index });
    }

    /// Pops the oldest completion, if any.
    pub fn pop(&mut self) -> Option<HotReloadSwapOk> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
