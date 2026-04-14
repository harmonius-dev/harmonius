//! Fixed slab pool for Opus frame buffers on the audio thread.

/// Handle into [`OpusFramePool`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpusFrameHandle(pub u8);

/// Pool-allocated Opus frame buffers (64 × 256 bytes).
#[derive(Debug)]
pub struct OpusFramePool {
    slab: [[u8; 256]; 64],
    free: [u8; 64],
    free_top: u8,
    /// Counts forced reuse when the pool is exhausted.
    pub frame_pool_overruns: u32,
}

impl OpusFramePool {
    /// Allocates an empty pool with all frames on the free stack.
    #[must_use]
    pub fn new() -> Self {
        let mut free = [0u8; 64];
        for i in 0u8..64 {
            free[i as usize] = i;
        }
        Self {
            slab: [[0u8; 256]; 64],
            free,
            free_top: 64,
            frame_pool_overruns: 0,
        }
    }

    /// Acquires a writable frame buffer, or `None` when the pool is exhausted.
    pub fn acquire(&mut self) -> Option<OpusFrameHandle> {
        if self.free_top == 0 {
            return None;
        }
        self.free_top -= 1;
        let idx = self.free[self.free_top as usize];
        Some(OpusFrameHandle(idx))
    }

    /// Returns a frame to the pool.
    pub fn release(&mut self, handle: OpusFrameHandle) {
        if self.free_top as usize >= 64 {
            return;
        }
        self.free[self.free_top as usize] = handle.0;
        self.free_top += 1;
    }

    /// Mutable view of the frame bytes for `handle`.
    pub fn frame_mut(&mut self, handle: OpusFrameHandle) -> Option<&mut [u8; 256]> {
        self.slab.get_mut(handle.0 as usize)
    }

    /// Records a pool overrun after the caller forcibly recycled a live frame.
    pub fn record_overrun(&mut self) {
        self.frame_pool_overruns = self.frame_pool_overruns.saturating_add(1);
    }
}

impl Default for OpusFramePool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-4.3.2.5 — 65th acquire fails until the caller recycles the oldest handle.
    #[test]
    fn tc_ir_4_3_2_5_frame_pool_overrun() {
        let mut pool = OpusFramePool::new();
        let mut handles = Vec::new();
        for _ in 0..64 {
            handles.push(pool.acquire().expect("slot"));
        }
        assert!(pool.acquire().is_none());
        let oldest = handles.remove(0);
        pool.release(oldest);
        pool.record_overrun();
        assert!(pool.acquire().is_some());
        assert!(pool.frame_pool_overruns >= 1);
    }
}
