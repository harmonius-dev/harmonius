//! I/O error taxonomy surfaced on the `IoResponse::Failed` path.

/// Recoverable I/O failure modes surfaced to subsystems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IoError {
    /// Path or object was not found.
    NotFound {
        /// Human-readable path or identifier for diagnostics.
        path: String,
    },
}
