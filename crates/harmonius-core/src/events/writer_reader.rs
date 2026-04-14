//! Type-safe [`EventWriter`] / [`EventReader`] views over
//! [`EventChannel`](super::channel::EventChannel).

use super::channel::{Event, EventChannel};

/// Sends events into an [`EventChannel`](super::channel::EventChannel) during a writer phase.
pub struct EventWriter<'a, T: Event> {
    channel: &'a mut EventChannel<T>,
}

impl<'a, T: Event> EventWriter<'a, T> {
    /// Wraps exclusive access to the channel back buffer.
    pub fn new(channel: &'a mut EventChannel<T>) -> Self {
        Self { channel }
    }

    /// Appends one event to the back buffer.
    pub fn send(&mut self, event: T) {
        self.channel.send(event);
    }

    /// Appends many events to the back buffer.
    pub fn send_batch(&mut self, events: impl IntoIterator<Item = T>) {
        self.channel.send_batch(events);
    }
}

/// Observes events from the front buffer after a swap.
pub struct EventReader<'a, T: Event> {
    channel: &'a EventChannel<T>,
}

impl<'a, T: Event> EventReader<'a, T> {
    /// Wraps shared access to the channel front buffer.
    pub fn new(channel: &'a EventChannel<T>) -> Self {
        Self { channel }
    }

    /// Events readable from the front buffer (slice view).
    pub fn read_front(&self) -> &[T] {
        self.channel.front()
    }

    /// Frame tick after the last swap on the underlying channel.
    pub fn frame_tick(&self) -> u64 {
        self.channel.frame_tick()
    }
}
