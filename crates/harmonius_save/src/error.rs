//! Save/load/migration error types aligned with the save-system design.

use std::fmt;

/// Low-level I/O failures surfaced through the save stack.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IoError {
    /// Human-readable context for logging.
    pub message: String,
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for IoError {}

/// Errors while writing saves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveError {
    /// Per-entity serialization failure.
    SerializationFailed {
        /// Stable entity id.
        entity: u64,
        /// Component type hash.
        type_hash: u64,
        detail: String,
    },
    IoFailed(IoError),
    EncryptionKeyUnavailable,
    SlotLimitReached {
        max: u32,
    },
    SlotNotFound(crate::types::SlotId),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::SerializationFailed {
                entity,
                type_hash,
                detail,
            } => write!(
                f,
                "serialization failed entity={entity} type_hash={type_hash}: {detail}"
            ),
            SaveError::IoFailed(e) => write!(f, "io failed: {e}"),
            SaveError::EncryptionKeyUnavailable => f.write_str("encryption key unavailable"),
            SaveError::SlotLimitReached { max } => write!(f, "slot limit reached (max {max})"),
            SaveError::SlotNotFound(id) => write!(f, "slot not found: {}", id.0),
        }
    }
}

impl std::error::Error for SaveError {}

/// Errors while reading saves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadError {
    FileNotFound(crate::types::SlotId),
    IoFailed(IoError),
    ChecksumMismatch {
        expected: u32,
        actual: u32,
    },
    DecryptionFailed,
    InvalidHeader,
    ForwardCompatError {
        saved: crate::types::SchemaVersion,
        current: crate::types::SchemaVersion,
    },
    MigrationFailed {
        type_hash: u64,
        from: crate::types::SchemaVersion,
        to: crate::types::SchemaVersion,
        detail: String,
    },
    DeserializationFailed {
        detail: String,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::FileNotFound(id) => write!(f, "save file not found for slot {}", id.0),
            LoadError::IoFailed(e) => write!(f, "io failed: {e}"),
            LoadError::ChecksumMismatch { expected, actual } => {
                write!(f, "checksum mismatch expected={expected} actual={actual}")
            }
            LoadError::DecryptionFailed => f.write_str("decryption failed"),
            LoadError::InvalidHeader => f.write_str("invalid save header"),
            LoadError::ForwardCompatError { saved, current } => write!(
                f,
                "forward compatibility: save {saved:?} newer than engine {current:?}"
            ),
            LoadError::MigrationFailed {
                type_hash,
                from,
                to,
                detail,
            } => write!(
                f,
                "migration failed type_hash={type_hash} {from:?}->{to:?}: {detail}"
            ),
            LoadError::DeserializationFailed { detail } => {
                write!(f, "deserialization failed: {detail}")
            }
        }
    }
}

impl std::error::Error for LoadError {}

/// Migration planning and step failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MigrationError {
    NoPath {
        type_hash: u64,
        from: crate::types::SchemaVersion,
        to: crate::types::SchemaVersion,
    },
    StepFailed {
        type_hash: u64,
        step_from: crate::types::SchemaVersion,
        step_to: crate::types::SchemaVersion,
        detail: String,
    },
    InvalidOrder {
        expected: crate::types::SchemaVersion,
        got: crate::types::SchemaVersion,
    },
    /// Non-fatal: migration completed; surfaced alongside success (TC-13.3.2.5).
    DataLossWarning { type_hash: u64, field: &'static str },
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationError::NoPath {
                type_hash,
                from,
                to,
            } => write!(
                f,
                "no migration path type_hash={type_hash} {from:?} -> {to:?}"
            ),
            MigrationError::StepFailed {
                type_hash,
                step_from,
                step_to,
                detail,
            } => write!(
                f,
                "migration step failed type_hash={type_hash} {step_from:?}->{step_to:?}: {detail}"
            ),
            MigrationError::InvalidOrder { expected, got } => {
                write!(
                    f,
                    "invalid migration order expected={expected:?} got={got:?}"
                )
            }
            MigrationError::DataLossWarning { type_hash, field } => {
                write!(f, "data loss warning type_hash={type_hash} field={field}")
            }
        }
    }
}

impl std::error::Error for MigrationError {}
