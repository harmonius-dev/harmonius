use std::collections::VecDeque;

/// Fixed-capacity MPSC-style buffer that drops the oldest event when full (IR-5.9.3 FB2).
#[derive(Debug)]
pub struct EventBridge<T> {
    cap: usize,
    queue: VecDeque<T>,
}

impl<T> EventBridge<T> {
    /// Creates a bridge with the given maximum retained events (design: `1024` for transfers).
    #[must_use]
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            queue: VecDeque::new(),
        }
    }

    /// Pushes `event`, dropping the oldest item when at capacity. Returns the dropped value, if any.
    pub fn push_drop_oldest(&mut self, event: T) -> Option<T> {
        let mut dropped = None;
        if self.queue.len() == self.cap {
            dropped = self.queue.pop_front();
        }
        self.queue.push_back(event);
        dropped
    }

    /// Number of events currently held.
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns `true` when no events are queued.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Iterator from oldest to newest.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.queue.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_5_9_3_fb2_event_bridge_channel_full_drops_oldest() {
        let cap = 1024;
        let mut bridge: EventBridge<u32> = EventBridge::new(cap);
        for i in 0..2000 {
            bridge.push_drop_oldest(i);
        }
        assert_eq!(bridge.len(), cap);
        let oldest = *bridge.iter().next().expect("non-empty");
        assert_eq!(oldest, 2000_u32 - u32::try_from(cap).expect("cap fits u32"));
    }
}
