//! I/O error taxonomy surfaced on the `IoResponse::Failed` path.

use std::io::ErrorKind;

/// Recoverable I/O failure modes surfaced to subsystems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IoError {
    /// Path or object was not found.
    NotFound {
        /// Human-readable path or identifier for diagnostics.
        path: String,
    },
    /// Caller lacked permission for the operation.
    PermissionDenied {
        /// Human-readable path or identifier for diagnostics.
        path: String,
    },
    /// Storage device reported no space left.
    StorageFull {
        /// Human-readable path or identifier for diagnostics.
        path: String,
    },
    /// Operation was interrupted before completion.
    Interrupted {
        /// Human-readable path or identifier for diagnostics.
        path: String,
    },
    /// OS error kinds not mapped to a dedicated variant yet.
    Other {
        /// Human-readable path or identifier for diagnostics.
        path: String,
        /// `std::io::ErrorKind` from the originating error.
        kind: ErrorKind,
    },
}

/// Maps a [`std::io::Error`] from filesystem calls into [`IoError`].
#[must_use]
pub fn map_std_io_error(path: String, err: std::io::Error) -> IoError {
    match err.kind() {
        ErrorKind::NotFound => IoError::NotFound { path },
        ErrorKind::PermissionDenied => IoError::PermissionDenied { path },
        ErrorKind::StorageFull => IoError::StorageFull { path },
        ErrorKind::Interrupted => IoError::Interrupted { path },
        kind => IoError::Other { path, kind },
    }
}
