//! CPU-side GPU state tracking (stub).

/// Pipeline and descriptor state cache (not yet implemented).
pub struct StateTracker {
    _private: (),
}

impl StateTracker {
    /// Create an empty state tracker.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for StateTracker {
    fn default() -> Self {
        Self::new()
    }
}
