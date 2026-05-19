//! GPU memory sub-allocation (stub).

/// Unified GPU heap allocator (not yet implemented).
pub struct MemoryAllocator {
    _private: (),
}

impl MemoryAllocator {
    /// Create a new allocator.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for MemoryAllocator {
    fn default() -> Self {
        Self::new()
    }
}
