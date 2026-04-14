//! Compression, encryption, and CRC-32C sealing for save payloads (IR-5.10.6).

use std::io::Cursor;

use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use crc::{Crc, CRC_32_ISCSI};

use crate::error::{LoadError, SaveError};
use crate::types::{CompressionKind, EncryptionKind};

const AES_NONCE_LEN: usize = 12;

/// CRC-32C (Castagnoli) used by the save envelope.
pub fn crc32c(bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&CRC_32_ISCSI);
    crc.checksum(bytes)
}

/// Compresses `input` according to `kind`.
pub fn compress_payload(input: &[u8], kind: CompressionKind) -> Result<Vec<u8>, SaveError> {
    match kind {
        CompressionKind::None => Ok(input.to_vec()),
        CompressionKind::Lz4 => Ok(lz4_flex::compress_prepend_size(input)),
        CompressionKind::Zstd => zstd::encode_all(Cursor::new(input), 3)
            .map_err(|e| SaveError::CompressionFailed(format!("{e:?}"))),
    }
}

/// Decompresses `input` according to `kind`.
pub fn decompress_payload(input: &[u8], kind: CompressionKind) -> Result<Vec<u8>, LoadError> {
    match kind {
        CompressionKind::None => Ok(input.to_vec()),
        CompressionKind::Lz4 => lz4_flex::decompress_size_prepended(input)
            .map_err(|e| LoadError::DecompressionFailed(format!("{e:?}"))),
        CompressionKind::Zstd => zstd::decode_all(Cursor::new(input))
            .map_err(|e| LoadError::DecompressionFailed(format!("{e:?}"))),
    }
}

/// Encrypts `input` according to `kind` using `key` when required.
pub fn encrypt_payload(
    input: &[u8],
    kind: EncryptionKind,
    key: Option<&[u8; 32]>,
) -> Result<Vec<u8>, SaveError> {
    match kind {
        EncryptionKind::None => Ok(input.to_vec()),
        EncryptionKind::Aes256Gcm => {
            let key =
                key.ok_or_else(|| SaveError::EncryptionFailed("missing AES-256-GCM key".into()))?;
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|e| SaveError::EncryptionFailed(format!("{e:?}")))?;
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let mut out = Vec::with_capacity(AES_NONCE_LEN + input.len() + 16);
            out.extend_from_slice(nonce.as_slice());
            let ct = cipher
                .encrypt(&nonce, input)
                .map_err(|e| SaveError::EncryptionFailed(format!("{e:?}")))?;
            out.extend_from_slice(&ct);
            Ok(out)
        }
        EncryptionKind::ChaCha20Poly1305 => Err(SaveError::EncryptionFailed(
            "ChaCha20-Poly1305 is not implemented".into(),
        )),
    }
}

/// Decrypts `input` according to `kind` using `key` when required.
pub fn decrypt_payload(
    input: &[u8],
    kind: EncryptionKind,
    key: Option<&[u8; 32]>,
) -> Result<Vec<u8>, LoadError> {
    match kind {
        EncryptionKind::None => Ok(input.to_vec()),
        EncryptionKind::Aes256Gcm => {
            let key =
                key.ok_or_else(|| LoadError::DecryptionFailed("missing AES-256-GCM key".into()))?;
            if input.len() < AES_NONCE_LEN {
                return Err(LoadError::DecryptionFailed("truncated nonce".into()));
            }
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|e| LoadError::DecryptionFailed(format!("{e:?}")))?;
            let nonce = Nonce::from_slice(&input[..AES_NONCE_LEN]);
            let ct = &input[AES_NONCE_LEN..];
            cipher
                .decrypt(nonce, ct)
                .map_err(|e| LoadError::DecryptionFailed(format!("{e:?}")))
        }
        EncryptionKind::ChaCha20Poly1305 => Err(LoadError::DecryptionFailed(
            "ChaCha20-Poly1305 is not implemented".into(),
        )),
    }
}
