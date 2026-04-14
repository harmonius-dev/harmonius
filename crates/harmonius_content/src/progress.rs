//! Batch import progress events.

/// One progress step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportProgressEvent {
    /// One entry finished successfully.
    ItemCompleted {
        /// Monotonic index.
        index: usize,
    },
}

/// Tracks completed count for batch imports.
#[derive(Debug, Default)]
pub struct ProgressTracker {
    completed: u32,
    events: Vec<ImportProgressEvent>,
}

impl ProgressTracker {
    /// Zero progress.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one completion.
    pub fn on_item_completed(&mut self, index: usize) {
        self.completed += 1;
        self.events
            .push(ImportProgressEvent::ItemCompleted { index });
    }

    /// Number finished.
    pub fn completed(&self) -> u32 {
        self.completed
    }

    /// Drain queued events (FIFO).
    pub fn poll(&mut self) -> Vec<ImportProgressEvent> {
        std::mem::take(&mut self.events)
    }
}
