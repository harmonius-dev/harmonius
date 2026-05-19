//! Barrier batching and merging (stub).

/// Barrier batch submitted before command buffer execution.
pub struct BarrierBatch {
    _private: (),
}

impl BarrierBatch {
    /// Create an empty barrier batch.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for BarrierBatch {
    fn default() -> Self {
        Self::new()
    }
}
