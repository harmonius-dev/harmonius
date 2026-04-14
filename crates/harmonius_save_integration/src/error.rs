//! Save and load errors for the save ↔ serialization integration surface.

use thiserror::Error;

/// Errors returned while building a save file or serializing components.
#[derive(Debug, Error)]
pub enum SaveError {
    /// rkyv serialization failed for a Saveable component.
    #[error("serialization failed for type_hash {type_hash}: {detail}")]
    SerializationFailed {
        /// Stable type hash of the failing component.
        type_hash: u64,
        /// Human-readable detail from the serializer.
        detail: String,
    },
    /// Arena budget was exceeded after bounded growth attempts.
    #[error("arena overflow after bounded growth")]
    ArenaOverflow,
    /// Compression failed.
    #[error("compression failed: {0}")]
    CompressionFailed(String),
    /// Encryption failed.
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),
}

/// Errors returned while parsing or applying a save file.
#[derive(Debug, Error)]
pub enum LoadError {
    /// The on-disk header prefix or header archive was invalid.
    #[error("invalid save header")]
    InvalidHeader,
    /// CRC-32C of the sealed payload did not match the header.
    #[error("checksum mismatch")]
    ChecksumMismatch,
    /// Decryption failed (wrong key or corrupt ciphertext).
    #[error("decryption failed: {0}")]
    DecryptionFailed(String),
    /// Decompression failed.
    #[error("decompression failed: {0}")]
    DecompressionFailed(String),
    /// rkyv validation or deserialize failed for the payload.
    #[error("deserialization failed: {detail}")]
    DeserializationFailed {
        /// Human-readable detail.
        detail: String,
    },
    /// A migration step failed.
    #[error("migration failed at step {step_index} for type_hash {type_hash}: {detail}")]
    MigrationFailed {
        /// Index of the failing migration step.
        step_index: u32,
        /// Type hash associated with the step.
        type_hash: u64,
        /// Human-readable detail.
        detail: String,
    },
}
