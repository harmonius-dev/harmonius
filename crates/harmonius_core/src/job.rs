//! Minimal scoped job API used by transform propagation.
//!
//! The full engine job system will replace this stub; propagation only
//! requires structured fork/join call sites (`TC-1.2.4` integration path).

/// Executes closures sequentially while preserving the scoped API surface.
pub fn scope<F>(f: F)
where
    F: for<'a> FnOnce(&'a Scope),
{
    let scope = Scope;
    f(&scope);
}

/// Handle passed to [`scope`] callbacks.
pub struct Scope;

impl Scope {
    /// Spawns work that is executed immediately on this stub implementation.
    pub fn spawn<R>(&self, job: impl FnOnce() -> R + Send) -> R {
        job()
    }
}
