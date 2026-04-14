//! Per-thread bump arena (minimal stand-in for `ThreadArena`).

/// Bump allocator backed by a fixed `Vec` for integration tests.
#[derive(Debug)]
pub struct ThreadArena {
    buf: Vec<u8>,
    pos: usize,
}

impl ThreadArena {
    /// Arena with `capacity` bytes of backing storage.
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0u8; capacity],
            pos: 0,
        }
    }

    /// Reset bump pointer (frame boundary).
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    /// Bump-allocate `size` bytes, or `None` if exhausted.
    pub fn alloc(&mut self, size: usize) -> Option<&mut [u8]> {
        let next = self.pos.checked_add(size)?;
        if next > self.buf.len() {
            return None;
        }
        let start = self.pos;
        self.pos = next;
        Some(&mut self.buf[start..next])
    }
}

impl Default for ThreadArena {
    fn default() -> Self {
        Self::new(4096)
    }
}
