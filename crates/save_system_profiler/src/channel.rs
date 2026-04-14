//! Bounded CH-24-style queue with DropOldest overflow policy (FM-1).

use std::collections::VecDeque;

use crate::types::ProfileMessage;

/// MPSC-shaped profile bus with fixed capacity and DropOldest semantics.
#[derive(Debug)]
pub struct SaveProfileChannel {
    cap: usize,
    queue: VecDeque<ProfileMessage>,
    /// Count of dropped oldest entries when the channel was full (FM-1).
    pub fm1_channel_drops: u64,
    /// When false, [`Self::send`] is a no-op (FM-4).
    pub profiler_enabled: bool,
}

impl SaveProfileChannel {
    /// Builds a CH-24 channel with capacity 32 and DropOldest overflow semantics.
    pub fn ch24() -> Self {
        Self::new(32)
    }

    /// Builds a channel with the given capacity and profiler emission enabled.
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            queue: VecDeque::new(),
            fm1_channel_drops: 0,
            profiler_enabled: true,
        }
    }

    /// Enqueues a message, dropping the oldest if at capacity.
    pub fn send(&mut self, msg: ProfileMessage) {
        if !self.profiler_enabled {
            return;
        }
        while self.queue.len() >= self.cap {
            self.queue.pop_front();
            self.fm1_channel_drops += 1;
        }
        self.queue.push_back(msg);
    }

    /// Removes and returns the next message, if any.
    pub fn try_recv(&mut self) -> Option<ProfileMessage> {
        self.queue.pop_front()
    }

    /// Current queued depth (for tests).
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns true when no messages are queued.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
