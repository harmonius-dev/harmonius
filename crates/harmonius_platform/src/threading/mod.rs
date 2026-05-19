//! Job system and task graph (bootstrap stubs).

/// Work-stealing thread pool (not yet implemented).
pub struct ThreadPool;

impl ThreadPool {
    /// Placeholder constructor.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self::new()
    }
}

/// DAG task graph (not yet implemented).
pub struct TaskGraph;

impl TaskGraph {
    /// Placeholder constructor.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for TaskGraph {
    fn default() -> Self {
        Self::new()
    }
}
