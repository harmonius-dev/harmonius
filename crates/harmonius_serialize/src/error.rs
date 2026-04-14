//! Serialization and I/O error types.

use thiserror::Error;

/// Failure while serializing a value.
#[derive(Debug, Error)]
pub enum SerializeError {
    /// rkyv could not serialize the payload.
    #[error("binary payload error: {0}")]
    BinaryPayload(String),
    /// I/O while persisting data.
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Failure while deserializing or validating bytes.
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// Magic bytes did not match `HSER`.
    #[error("invalid binary magic at offset {offset}")]
    BadMagic {
        /// Byte offset of the header field.
        offset: usize,
    },
    /// Declared payload length exceeds available bytes.
    #[error("truncated binary at offset {offset}")]
    Truncated {
        /// Byte offset where parsing failed.
        offset: usize,
    },
    /// Header or payload layout was inconsistent.
    #[error("invalid binary layout at offset {offset}: {message}")]
    InvalidLayout {
        /// Byte offset of the problem.
        offset: usize,
        /// Human-readable detail.
        message: String,
    },
    /// rkyv validation or access failed.
    #[error("archive access failed at offset {offset}: {message}")]
    ArchiveAccess {
        /// Byte offset into the provided buffer (payload-relative when applicable).
        offset: usize,
        /// Underlying error text.
        message: String,
    },
    /// Text format parse error.
    #[error("text parse error at line {line}: {message}")]
    TextParse {
        /// 1-based line number.
        line: usize,
        /// Detail.
        message: String,
    },
    /// Schema version in the payload is newer than this build understands.
    #[error("unsupported schema version {found:?} (max {max:?})")]
    UnsupportedSchemaVersion {
        /// Version read from the stream.
        found: crate::migration::SchemaVersion,
        /// Highest supported version.
        max: crate::migration::SchemaVersion,
    },
}

/// Migration registry errors.
#[derive(Debug, Error)]
pub enum MigrationError {
    /// No migration path exists between two versions.
    #[error("no migration registered from {from:?} to {to:?} for type {type_name}")]
    MissingStep {
        /// Type logical name.
        type_name: String,
        /// Source version.
        from: crate::migration::SchemaVersion,
        /// Missing destination hop.
        to: crate::migration::SchemaVersion,
    },
    /// Requested type has no declared current version.
    #[error("unknown type {0}")]
    UnknownType(String),
    /// User migration function returned an error.
    #[error("migration function failed: {0}")]
    User(&'static str),
}
