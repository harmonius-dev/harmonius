//! Storage-layer error types.

/// Preference persistence failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefsError {
    /// Underlying filesystem error.
    IoError(crate::filesystem::FsError),
    /// TOML parse failure.
    ParseError {
        /// Source line when known.
        line: u32,
        /// Parser message.
        message: String,
    },
    /// Cloud sync hook failed with a diagnostic string.
    Cloud(String),
}
