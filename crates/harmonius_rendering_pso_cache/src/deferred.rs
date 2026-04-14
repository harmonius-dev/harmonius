/// Marks logical frame boundaries for deferred GPU resource drops.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameBarrier {
    frame: u64,
}

impl FrameBarrier {
    /// Creates a barrier tracker starting at frame zero.
    #[must_use]
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    /// Advances to the next frame.
    pub fn advance(&mut self) {
        self.frame = self.frame.saturating_add(1);
    }

    /// Current frame index.
    #[must_use]
    pub fn frame(&self) -> u64 {
        self.frame
    }
}
