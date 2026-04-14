//! Scratch bump arena reset between saves (TC-13.3.1.5).

/// Minimal bump allocator backing store used by serializers until engine arenas land.
#[derive(Debug, Default)]
pub struct Arena {
    buf: Vec<u8>,
}

impl Arena {
    /// Returns a new empty arena.
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Current allocated bytes (high-water since last reset).
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// True when no scratch bytes are retained.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Append bytes and return a stable slice index range as `(start, len)`.
    pub fn push_bytes(&mut self, data: &[u8]) -> (usize, usize) {
        let start = self.buf.len();
        self.buf.extend_from_slice(data);
        (start, data.len())
    }

    /// View previously pushed bytes.
    pub fn get_range(&self, start: usize, len: usize) -> Option<&[u8]> {
        self.buf.get(start..start + len)
    }

    /// Drop scratch allocations between saves.
    pub fn reset(&mut self) {
        self.buf.clear();
    }
}
