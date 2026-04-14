//! Native `HAST` container used by TC-12.1.1.*.

use std::path::Path;

use crate::{AssetId, ContentHash, ImportError};

pub const NATIVE_MAGIC: &[u8; 4] = b"HAST";

/// Header + body import output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeImportOutput {
    /// Assigned asset id (test harness uses monotonic ids).
    pub asset_id: AssetId,
    /// BLAKE3 of the body bytes (after header).
    pub content_hash: ContentHash,
}

fn body_hash(body: &[u8]) -> ContentHash {
    ContentHash::from_data(body)
}

/// Parses `bytes` as native `HAST` v1: magic, version u32 LE, content_hash, body.
pub fn parse_native<'a>(
    bytes: &'a [u8],
    path: &Path,
) -> Result<(u32, ContentHash, &'a [u8]), ImportError> {
    if bytes.len() < 40 {
        return Err(ImportError::InvalidNativeContainer {
            path: path.to_path_buf(),
            reason: "native asset smaller than 40-byte HAST header",
        });
    }
    if &bytes[0..4] != NATIVE_MAGIC {
        return Err(ImportError::InvalidNativeContainer {
            path: path.to_path_buf(),
            reason: "expected HAST magic",
        });
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    let mut expected = [0u8; 32];
    expected.copy_from_slice(&bytes[8..40]);
    let expected = ContentHash { bytes: expected };
    let body = &bytes[40..];
    let actual = body_hash(body);
    if actual != expected {
        return Err(ImportError::HashMismatch { expected, actual });
    }
    Ok((version, expected, body))
}

/// Validates and returns import metadata for a native asset file bytes.
pub fn import_native_asset(bytes: &[u8], path: &Path) -> Result<NativeImportOutput, ImportError> {
    if bytes.len() < 8 {
        return Err(ImportError::InvalidNativeContainer {
            path: path.to_path_buf(),
            reason: "native asset too small for HAST magic and version",
        });
    }
    if &bytes[0..4] != NATIVE_MAGIC {
        return Err(ImportError::InvalidNativeContainer {
            path: path.to_path_buf(),
            reason: "expected HAST magic",
        });
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version != 1 {
        return Err(ImportError::ValidationFailed {
            path: path.to_path_buf(),
            offset: 4,
            suggestion: Some("Set format_version to 1 for supported native assets.".into()),
        });
    }
    let (_v, content_hash, _body) = parse_native(bytes, path)?;
    Ok(NativeImportOutput {
        asset_id: AssetId(1),
        content_hash,
    })
}

/// Builds native bytes: magic, version=1, blake3(body), body.
pub fn build_native_v1(body: &[u8]) -> Vec<u8> {
    let h = body_hash(body);
    let mut v = Vec::with_capacity(40 + body.len());
    v.extend_from_slice(NATIVE_MAGIC);
    v.extend_from_slice(&1u32.to_le_bytes());
    v.extend_from_slice(&h.bytes);
    v.extend_from_slice(body);
    v
}

/// Builds native bytes with an incorrect header hash (integrity failure).
pub fn build_native_v1_wrong_hash(body: &[u8]) -> Vec<u8> {
    let wrong = ContentHash { bytes: [0u8; 32] };
    let mut v = Vec::with_capacity(40 + body.len());
    v.extend_from_slice(NATIVE_MAGIC);
    v.extend_from_slice(&1u32.to_le_bytes());
    v.extend_from_slice(&wrong.bytes);
    v.extend_from_slice(body);
    v
}
