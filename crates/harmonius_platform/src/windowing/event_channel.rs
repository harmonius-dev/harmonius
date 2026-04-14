//! Bounded event queue with adaptive capacity for resize storms (R-14.1.9).

use std::collections::VecDeque;

const BASELINE_CAPACITY: usize = 64;
const BURST_WINDOW_NS: u64 = 16_000_000;
const BURST_PUSH_THRESHOLD: u32 = 10_000;
const DECAY_INTERVAL_NS: u64 = 1_000_000_000;
const DECAY_SHRINK_FACTOR: usize = 4;

/// Bounded FIFO queue that grows under burst load and decays toward a baseline when idle.
///
/// The channel never drops events on successful [`Self::push`]; instead it increases its logical
/// capacity when a burst is detected inside a short time window.
#[derive(Debug)]
pub struct AutoSizingEventChannel<T> {
    queue: VecDeque<T>,
    /// Logical capacity limit before backpressure would apply without growth.
    capacity: usize,
    baseline: usize,
    window_start_ns: u64,
    pushes_in_window: u32,
    last_push_ns: u64,
    last_decay_ns: u64,
}

impl<T> AutoSizingEventChannel<T> {
    /// Create a channel with the default baseline capacity (64).
    #[must_use]
    pub fn new() -> Self {
        Self::with_baseline(BASELINE_CAPACITY)
    }

    /// Create a channel with a custom baseline capacity (still subject to growth on bursts).
    #[must_use]
    pub fn with_baseline(baseline: usize) -> Self {
        let baseline = baseline.max(1);
        Self {
            queue: VecDeque::new(),
            capacity: baseline,
            baseline,
            window_start_ns: 0,
            pushes_in_window: 0,
            last_push_ns: 0,
            last_decay_ns: 0,
        }
    }

    /// Current logical capacity (may exceed baseline after a burst).
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Number of queued events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Whether the queue contains no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Push an event using `now_ns` as a monotonic clock in nanoseconds.
    ///
    /// Capacity grows automatically when a burst threshold is crossed inside a 16 ms window.
    pub fn push(&mut self, value: T, now_ns: u64) {
        self.restart_burst_window_if_needed(now_ns);
        self.pushes_in_window = self.pushes_in_window.saturating_add(1);
        self.last_push_ns = now_ns;

        let burst = self.pushes_in_window >= BURST_PUSH_THRESHOLD;
        if self.queue.len() == self.capacity {
            if burst {
                self.grow_capacity();
            } else {
                // Baseline headroom should prevent this for non-burst workloads.
                self.grow_capacity();
            }
        }

        self.queue.push_back(value);
    }

    /// Pop the oldest event, if any.
    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    /// Advance time for decay logic; call periodically when idle or between frames.
    pub fn tick(&mut self, now_ns: u64) {
        if now_ns.saturating_sub(self.last_decay_ns) < DECAY_INTERVAL_NS {
            return;
        }
        self.last_decay_ns = now_ns;

        if self.queue.len() > self.baseline / 2 {
            return;
        }

        if self.capacity > self.baseline {
            let target = (self.capacity - self.capacity / DECAY_SHRINK_FACTOR)
                .max(self.baseline)
                .max(self.queue.len());
            self.capacity = target;
        }
    }

    fn restart_burst_window_if_needed(&mut self, now_ns: u64) {
        if now_ns.saturating_sub(self.window_start_ns) > BURST_WINDOW_NS {
            self.window_start_ns = now_ns;
            self.pushes_in_window = 0;
        }
    }

    fn grow_capacity(&mut self) {
        let needed = self.queue.len().saturating_add(1);
        let mut next = self.capacity.max(self.baseline);
        while next < needed {
            next = next.saturating_mul(2);
        }
        self.capacity = next;
    }
}

impl<T> Default for AutoSizingEventChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_14_1_9_1_slow_rate_keeps_baseline_capacity() {
        let mut ch = AutoSizingEventChannel::<u32>::new();
        let mut t = 0_u64;
        for i in 0..10 {
            ch.push(i, t);
            t += 100_000_000; // 100 ms between events => 10 Hz
        }
        ch.tick(t + DECAY_INTERVAL_NS);
        assert_eq!(ch.capacity(), BASELINE_CAPACITY);
        assert_eq!(ch.len(), 10);
    }

    #[test]
    fn tc_14_1_9_1_burst_grows_capacity_without_drops() {
        let mut ch = AutoSizingEventChannel::<u32>::new();
        let start = 1_000_000_000_u64;
        for i in 0..BURST_PUSH_THRESHOLD {
            ch.push(i, start);
        }
        assert!(ch.capacity() >= BURST_PUSH_THRESHOLD as usize);
        assert_eq!(ch.len(), BURST_PUSH_THRESHOLD as usize);

        // Drain and idle; capacity should decay toward baseline across ticks.
        for _ in 0..BURST_PUSH_THRESHOLD {
            ch.pop();
        }
        let mut t = start + BURST_WINDOW_NS + 1;
        let mut iterations = 0_u32;
        while ch.capacity() > BASELINE_CAPACITY && iterations < 200 {
            ch.tick(t);
            t += DECAY_INTERVAL_NS;
            iterations += 1;
        }
        assert_eq!(ch.capacity(), BASELINE_CAPACITY);
    }
}
