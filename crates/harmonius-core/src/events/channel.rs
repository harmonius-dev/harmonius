//! Double-buffered [`EventChannel`] and frame-boundary swapping.

/// Marker trait for types stored in [`EventChannel`].
pub trait Event: Send + Sync + Clone + 'static {}

impl<T> Event for T where T: Send + Sync + Clone + 'static {}

/// Double-buffered channel: writers append to the back buffer; readers observe the front buffer.
#[derive(Debug)]
pub struct EventChannel<T: Event> {
    front: Vec<T>,
    back: Vec<T>,
    frame_tick: u64,
    flood_threshold: u32,
    flood_diagnostic_fired: bool,
}

impl<T: Event> EventChannel<T> {
    /// Constructs a channel with the default flood threshold (50_000 events).
    pub fn new() -> Self {
        Self::with_flood_threshold(50_000)
    }

    /// Constructs a channel with an explicit flood diagnostic threshold.
    pub fn with_flood_threshold(flood_threshold: u32) -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
            frame_tick: 0,
            flood_threshold,
            flood_diagnostic_fired: false,
        }
    }

    /// Swaps front and back buffers and clears the new back buffer.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
        self.back.clear();
        self.frame_tick = self.frame_tick.saturating_add(1);
    }

    /// Number of events readable on the front buffer.
    pub fn front_len(&self) -> usize {
        self.front.len()
    }

    /// Number of events writable on the back buffer.
    pub fn back_len(&self) -> usize {
        self.back.len()
    }

    /// Frame tick after the last swap.
    pub fn frame_tick(&self) -> u64 {
        self.frame_tick
    }

    /// Whether the flood diagnostic has fired since construction.
    pub fn flood_diagnostic_fired(&self) -> bool {
        self.flood_diagnostic_fired
    }

    /// Resets the flood diagnostic latch (test hook).
    pub fn reset_flood_diagnostic(&mut self) {
        self.flood_diagnostic_fired = false;
    }

    /// Appends a single event to the back buffer.
    pub fn send(&mut self, event: T) {
        if self.back.len() as u32 >= self.flood_threshold {
            self.flood_diagnostic_fired = true;
        }
        self.back.push(event);
    }

    /// Appends many events to the back buffer.
    pub fn send_batch(&mut self, events: impl IntoIterator<Item = T>) {
        for event in events {
            self.send(event);
        }
    }

    /// Iterates events readable from the front buffer.
    pub fn read_front(&self) -> impl Iterator<Item = &T> {
        self.front.iter()
    }

    /// Mutable access to the back buffer for advanced writers.
    pub fn back_mut(&mut self) -> &mut Vec<T> {
        &mut self.back
    }

    /// Shared access to the front buffer for readers.
    pub fn front(&self) -> &[T] {
        &self.front
    }
}

impl<T: Event> Default for EventChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}
