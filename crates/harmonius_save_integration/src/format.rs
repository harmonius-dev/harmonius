//! Save file envelope: 8-byte LE header length, rkyv [`SaveFileHeader`](crate::types::SaveFileHeader),
//! then sealed payload tail (IR-5.10.2).

use rkyv::api::high::access;
use rkyv::rancor;
use rkyv::Archive;

use crate::error::{LoadError, SaveError};
use crate::migration::MigrationRegistry;
use crate::pipeline::{
    compress_payload, crc32c, decompress_payload, decrypt_payload, encrypt_payload,
};
use crate::types::{EntitySnapshot, SaveEnvelopeParams, SaveFileHeader, SAVE_MAGIC};

/// Parameters that vary per decode (current schema + optional AES key).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SaveDecodeParams<'a> {
    /// Engine schema expected after migrations complete.
    pub current_schema: crate::types::SchemaVersion,
    /// Optional AES-256-GCM key when the header requests encryption.
    pub key: Option<&'a [u8; 32]>,
}

/// Encodes a save file envelope for `entities`.
pub fn encode_save_file(
    entities: &Vec<EntitySnapshot>,
    params: &SaveEnvelopeParams,
    key: Option<&[u8; 32]>,
) -> Result<Vec<u8>, SaveError> {
    let raw =
        rkyv::to_bytes::<rancor::Error>(entities).map_err(|e| SaveError::SerializationFailed {
            type_hash: 0,
            detail: format!("entity vec: {e:?}"),
        })?;
    let compressed = compress_payload(raw.as_slice(), params.compression)?;
    let sealed = encrypt_payload(&compressed, params.encryption, key)?;
    let checksum = crc32c(&sealed);
    let header = SaveFileHeader {
        magic: SAVE_MAGIC,
        schema_version: params.schema_version,
        component_versions: params.component_versions.clone(),
        compression: params.compression,
        encryption: params.encryption,
        checksum,
        created_at_unix: params.created_at_unix,
        dimension: params.dimension,
    };
    let header_bytes =
        rkyv::to_bytes::<rancor::Error>(&header).map_err(|e| SaveError::SerializationFailed {
            type_hash: 0,
            detail: format!("header: {e:?}"),
        })?;
    let header_len =
        u64::try_from(header_bytes.len()).map_err(|_| SaveError::SerializationFailed {
            type_hash: 0,
            detail: "header length overflow".into(),
        })?;
    let mut out = Vec::new();
    out.extend_from_slice(&header_len.to_le_bytes());
    out.extend_from_slice(header_bytes.as_slice());
    out.extend_from_slice(&sealed);
    Ok(out)
}

/// Decodes a save file envelope from an mmap-friendly byte slice.
pub fn decode_save_file(
    data: &[u8],
    decode: &SaveDecodeParams<'_>,
    migration: &MigrationRegistry,
) -> Result<Vec<EntitySnapshot>, LoadError> {
    if data.len() < 8 {
        return Err(LoadError::InvalidHeader);
    }
    let header_len_u64 = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let header_len = usize::try_from(header_len_u64).map_err(|_| LoadError::InvalidHeader)?;
    let payload_start = 8usize
        .checked_add(header_len)
        .ok_or(LoadError::InvalidHeader)?;
    if payload_start > data.len() {
        return Err(LoadError::InvalidHeader);
    }
    let header_slice = &data[8..payload_start];
    let header_archived =
        access::<<SaveFileHeader as Archive>::Archived, rancor::Error>(header_slice)
            .map_err(|_| LoadError::InvalidHeader)?;
    let header: SaveFileHeader =
        rkyv::deserialize::<SaveFileHeader, rancor::Error>(header_archived)
            .map_err(|_| LoadError::InvalidHeader)?;
    if header.magic != SAVE_MAGIC {
        return Err(LoadError::InvalidHeader);
    }
    let sealed = &data[payload_start..];
    if crc32c(sealed) != header.checksum {
        return Err(LoadError::ChecksumMismatch);
    }
    let decrypted = decrypt_payload(sealed, header.encryption, decode.key)?;
    let decompressed = decompress_payload(&decrypted, header.compression)?;
    let migrated = if migration.needs_migration(header.schema_version, decode.current_schema) {
        migration.migrate_all(decompressed)?
    } else {
        decompressed
    };
    let archived_entities = access::<<Vec<EntitySnapshot> as Archive>::Archived, rancor::Error>(
        &migrated,
    )
    .map_err(|_| LoadError::DeserializationFailed {
        detail: "payload validation".into(),
    })?;
    rkyv::deserialize::<Vec<EntitySnapshot>, rancor::Error>(archived_entities).map_err(|e| {
        LoadError::DeserializationFailed {
            detail: format!("{e:?}"),
        }
    })
}
