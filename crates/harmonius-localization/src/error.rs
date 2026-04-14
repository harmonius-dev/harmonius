//! Localization load and runtime errors.

use core::fmt;

/// Failure modes for table loads and locale activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocError {
    /// Serialized bytes failed `rkyv` validation.
    InvalidArchive,
    /// Requested locale is not present in the registry.
    NotLoaded,
}

impl fmt::Display for LocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArchive => write!(f, "invalid localization archive"),
            Self::NotLoaded => write!(f, "locale not loaded"),
        }
    }
}

impl std::error::Error for LocError {}
