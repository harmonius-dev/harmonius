//! Window and display operation errors.

use crate::windowing::{DisplayId, WindowMode};

/// Errors from window operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WindowError {
    /// The platform backend failed to create the window.
    CreationFailed {
        /// Human-readable failure detail.
        message: String,
    },
    /// The requested display was not found.
    DisplayNotFound(DisplayId),
    /// The requested fullscreen mode is not supported.
    FullscreenNotSupported(WindowMode),
    /// Platform-specific error with OS error code.
    Platform {
        /// OS or driver error code.
        code: i32,
    },
}
