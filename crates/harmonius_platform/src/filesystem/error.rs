//! Filesystem error types (`R-14.6`).

/// Failure from a filesystem operation on the host I/O path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    /// Path does not exist.
    NotFound {
        /// Original path string from the caller.
        path: String,
    },
    /// Permission denied for the path.
    PermissionDenied {
        /// Path that could not be accessed.
        path: String,
    },
    /// Exclusive create failed because the entry already exists.
    AlreadyExists {
        /// Conflicting path.
        path: String,
    },
    /// Directory not empty when removal was requested.
    DirectoryNotEmpty {
        /// Directory path.
        path: String,
    },
    /// Operation was cancelled before completion.
    Cancelled,
    /// Device or quota reported full.
    DeviceFull,
    /// Symlink cycle detected while resolving a path.
    SymlinkLoop {
        /// Path involved in the cycle.
        path: String,
    },
    /// Path length exceeded platform limits.
    PathTooLong {
        /// Overlong path.
        path: String,
    },
    /// Opaque platform error from the host OS.
    Platform {
        /// errno-style code when available.
        code: i32,
        /// Human-readable message.
        message: String,
    },
}
