//! OS integration error surface (`R-14.2`).

/// Errors from OS-facing helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsError {
    /// Feature not available on this host.
    Unsupported,
    /// User cancelled an interactive flow.
    Cancelled,
    /// Platform-specific failure.
    Platform {
        /// errno-style code when available.
        code: i32,
        /// Message text.
        message: String,
    },
    /// Clipboard format mismatch.
    FormatMismatch,
    /// MIME type rejected by drag/drop filters.
    MimeRejected {
        /// Rejected MIME string.
        mime_type: String,
    },
}
