//! Localization load and runtime errors.

/// Failure modes for table loads and locale activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocError {
    /// Serialized bytes failed `rkyv` validation.
    InvalidArchive,
    /// Requested locale is not present in the registry.
    NotLoaded,
}
