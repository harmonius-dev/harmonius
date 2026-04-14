//! Bounded MPSC-style channel used to model `CH-29` locale fan-out in tests.

use crate::types::LocaleId;

/// Locale transition notification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocaleChangeEvent {
    /// Previous locale.
    pub previous: LocaleId,
    /// Next locale.
    pub next: LocaleId,
}

/// Fixed-capacity queue with explicit backpressure (FM-5).
#[derive(Clone, Debug)]
pub struct LocaleChangeChannel {
    cap: usize,
    queue: Vec<LocaleChangeEvent>,
    dropped: u32,
}

impl LocaleChangeChannel {
    /// Creates a channel with capacity `cap` (tests use 16 per design).
    #[must_use]
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            queue: Vec::new(),
            dropped: 0,
        }
    }

    /// Enqueues an event, dropping newest when full (counts `dropped`).
    pub fn send(&mut self, event: LocaleChangeEvent) {
        if self.queue.len() >= self.cap {
            self.dropped += 1;
            return;
        }
        self.queue.push(event);
    }

    /// Drains all pending events.
    pub fn drain(&mut self) -> Vec<LocaleChangeEvent> {
        std::mem::take(&mut self.queue)
    }

    /// Count of dropped sends when over capacity.
    #[must_use]
    pub fn dropped(&self) -> u32 {
        self.dropped
    }
}
