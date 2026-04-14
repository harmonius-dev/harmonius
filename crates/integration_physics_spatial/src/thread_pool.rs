//! Minimal worker pool facade used by batch spatial queries.

use std::thread::Scope;

/// Physics-adjacent worker thread pool handle (`std::thread::scope` wrapper).
#[derive(Debug, Default)]
pub struct ThreadPool;

impl ThreadPool {
    /// Creates a new pool handle (thread count reserved for future tuning).
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Runs `f` with a scoped spawner backed by the standard library scope API.
    pub fn scope<'env, F, R>(&'env self, f: F) -> R
    where
        F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> R,
    {
        std::thread::scope(f)
    }
}
