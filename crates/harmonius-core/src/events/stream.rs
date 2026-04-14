//! Ring-buffer-backed [`PersistentStream`] and [`StreamCursor`].

use std::marker::PhantomData;

use super::channel::Event;

/// Ring-buffer-backed stream retaining events across frames.
#[derive(Debug)]
pub struct PersistentStream<T: Event> {
    ring: Vec<Option<T>>,
    capacity: u32,
    write_head: u64,
}

impl<T: Event> PersistentStream<T> {
    /// Creates a stream with the given ring capacity.
    pub fn new(capacity: u32) -> Self {
        let cap = capacity.max(1) as usize;
        Self {
            ring: (0..cap).map(|_| None).collect(),
            capacity: capacity.max(1),
            write_head: 0,
        }
    }

    /// Monotonic write sequence head.
    pub fn write_head(&self) -> u64 {
        self.write_head
    }

    /// Appends an event, overwriting the oldest slot when full.
    pub fn push(&mut self, event: T) {
        let cap = self.capacity as usize;
        let slot = (self.write_head as usize) % cap;
        self.ring[slot] = Some(event);
        self.write_head = self.write_head.saturating_add(1);
    }

    /// Cursor positioned at the current write head (reads nothing until new events arrive).
    pub fn cursor(&self) -> StreamCursor<T> {
        StreamCursor {
            read_head: self.write_head,
            lost: false,
            _marker: PhantomData,
        }
    }

    /// Cursor positioned to read from the oldest retained slot at this moment.
    pub fn cursor_from_oldest(&self) -> StreamCursor<T> {
        let oldest = self.write_head.saturating_sub(self.capacity as u64);
        self.cursor_at(oldest)
    }

    /// Cursor positioned at an explicit monotonic read head (multi-cursor / replay tests).
    pub fn cursor_at(&self, read_head: u64) -> StreamCursor<T> {
        StreamCursor {
            read_head,
            lost: false,
            _marker: PhantomData,
        }
    }
}

/// Independent read cursor into a [`PersistentStream`].
#[derive(Debug)]
pub struct StreamCursor<T: Event> {
    read_head: u64,
    lost: bool,
    _marker: PhantomData<T>,
}

impl<T: Event> StreamCursor<T> {
    /// Reads all events in `(read_head, write_head]` and advances the cursor.
    pub fn read<'a>(&mut self, stream: &'a PersistentStream<T>) -> Vec<&'a T> {
        let cap = stream.capacity as usize;
        let mut out = Vec::new();
        let mut seq = self.read_head;
        while seq < stream.write_head {
            if stream.write_head.saturating_sub(self.read_head) > u64::from(stream.capacity) {
                self.lost = true;
            }
            let slot = (seq as usize) % cap;
            if let Some(ref e) = stream.ring[slot] {
                out.push(e);
            }
            seq = seq.saturating_add(1);
        }
        self.read_head = stream.write_head;
        out
    }

    /// Number of unread events based on monotonic sequence distance.
    pub fn unread_count(&self, stream: &PersistentStream<T>) -> u32 {
        stream
            .write_head
            .saturating_sub(self.read_head)
            .min(u64::from(u32::MAX)) as u32
    }

    /// True if the cursor fell behind the ring and events were lost.
    pub fn has_overflowed(&self, stream: &PersistentStream<T>) -> bool {
        self.lost || stream.write_head.saturating_sub(self.read_head) > u64::from(stream.capacity)
    }
}

/// Platform-style capacity presets (metadata for tests and hosts).
pub struct StreamConfig {
    /// Maximum events per channel.
    pub capacity: u32,
    /// Maximum concurrent channels.
    pub max_channels: u32,
}

impl StreamConfig {
    /// Mobile preset.
    pub fn mobile() -> Self {
        Self {
            capacity: 4096,
            max_channels: 64,
        }
    }

    /// Nintendo Switch preset.
    pub fn switch() -> Self {
        Self {
            capacity: 8192,
            max_channels: 128,
        }
    }

    /// Desktop preset.
    pub fn desktop() -> Self {
        Self {
            capacity: 32768,
            max_channels: 1024,
        }
    }
}
