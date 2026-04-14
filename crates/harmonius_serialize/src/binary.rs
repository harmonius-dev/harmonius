//! HSER binary envelope plus rkyv payload (RF-2).

use rkyv::api::high::access;
use rkyv::bytecheck::CheckBytes;
use rkyv::rancor;
use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Serialize};

use crate::error::{DeserializeError, SerializeError};
use crate::migration::{MigrationRegistry, SchemaVersion};

const MAGIC: [u8; 4] = *b"HSER";

/// Fixed-size little-endian header before every rkyv archive.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinaryHeader {
    /// Magic bytes `HSER`.
    pub magic: [u8; 4],
    /// Header format version (currently 1).
    pub header_version: u8,
    /// Reserved.
    pub _pad0: [u8; 3],
    /// Stable hash of the logical type name.
    pub type_id_hash: u64,
    /// Embedded schema version.
    pub schema_major: u16,
    /// Minor schema component.
    pub schema_minor: u16,
    /// Patch schema component.
    pub schema_patch: u16,
    /// Padding to 8-byte boundary.
    pub _pad1: u16,
    /// Byte length of the rkyv payload following this header.
    pub payload_len: u64,
}

impl BinaryHeader {
    /// Current header wire version.
    pub const HEADER_VERSION: u8 = 1;

    /// Wire size in bytes (multiple of 8).
    pub const SIZE: usize = 32;

    fn from_parts(type_id_hash: u64, version: SchemaVersion, payload_len: u64) -> Self {
        Self {
            magic: MAGIC,
            header_version: Self::HEADER_VERSION,
            _pad0: [0; 3],
            type_id_hash,
            schema_major: version.major,
            schema_minor: version.minor,
            schema_patch: version.patch,
            _pad1: 0,
            payload_len,
        }
    }

    fn schema_version(&self) -> SchemaVersion {
        SchemaVersion::new(self.schema_major, self.schema_minor, self.schema_patch)
    }

    /// Writes this header as 32 little-endian bytes.
    pub fn write_bytes(&self) -> [u8; Self::SIZE] {
        let mut b = [0u8; Self::SIZE];
        let mut o = 0;
        b[o..o + 4].copy_from_slice(&self.magic);
        o += 4;
        b[o] = self.header_version;
        o += 1;
        b[o..o + 3].copy_from_slice(&self._pad0);
        o += 3;
        b[o..o + 8].copy_from_slice(&self.type_id_hash.to_le_bytes());
        o += 8;
        b[o..o + 2].copy_from_slice(&self.schema_major.to_le_bytes());
        o += 2;
        b[o..o + 2].copy_from_slice(&self.schema_minor.to_le_bytes());
        o += 2;
        b[o..o + 2].copy_from_slice(&self.schema_patch.to_le_bytes());
        o += 2;
        b[o..o + 2].copy_from_slice(&self._pad1.to_le_bytes());
        o += 2;
        b[o..o + 8].copy_from_slice(&self.payload_len.to_le_bytes());
        b
    }

    /// Parses a header from the start of `bytes`.
    pub fn read_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        if bytes.len() < Self::SIZE {
            return Err(DeserializeError::Truncated { offset: 0 });
        }
        let mut o = 0;
        let magic: [u8; 4] = bytes[o..o + 4]
            .try_into()
            .map_err(|_| DeserializeError::BadMagic { offset: 0 })?;
        o += 4;
        let header_version = bytes[o];
        o += 1;
        let _pad0: [u8; 3] = bytes[o..o + 3].try_into().unwrap();
        o += 3;
        let type_id_hash = u64::from_le_bytes(bytes[o..o + 8].try_into().unwrap());
        o += 8;
        let schema_major = u16::from_le_bytes(bytes[o..o + 2].try_into().unwrap());
        o += 2;
        let schema_minor = u16::from_le_bytes(bytes[o..o + 2].try_into().unwrap());
        o += 2;
        let schema_patch = u16::from_le_bytes(bytes[o..o + 2].try_into().unwrap());
        o += 2;
        let _pad1 = u16::from_le_bytes(bytes[o..o + 2].try_into().unwrap());
        o += 2;
        let payload_len = u64::from_le_bytes(bytes[o..o + 8].try_into().unwrap());
        Ok(Self {
            magic,
            header_version,
            _pad0,
            type_id_hash,
            schema_major,
            schema_minor,
            schema_patch,
            _pad1,
            payload_len,
        })
    }
}

/// Stable 64-bit FNV-1a of `type_name` (portable across hosts).
pub fn stable_type_name_hash(type_name: &str) -> u64 {
    const OFFSET: u64 = 14695981039346656037;
    const PRIME: u64 = 1099511628211;
    let mut h = OFFSET;
    for &b in type_name.as_bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(PRIME);
    }
    h
}

/// Serializes `value` with an HSER header and rkyv payload.
pub fn serialize_binary<T>(
    value: &T,
    type_name: &str,
    version: SchemaVersion,
) -> Result<Vec<u8>, SerializeError>
where
    T: for<'a> Serialize<
        rkyv::api::high::HighSerializer<
            AlignedVec,
            rkyv::ser::allocator::ArenaHandle<'a>,
            rancor::Error,
        >,
    >,
{
    let payload = rkyv::to_bytes::<rancor::Error>(value)
        .map_err(|e| SerializeError::BinaryPayload(format!("{e:?}")))?;
    let type_id_hash = stable_type_name_hash(type_name);
    let payload_len = u64::try_from(payload.len())
        .map_err(|_| SerializeError::BinaryPayload("payload length overflow".into()))?;
    let header = BinaryHeader::from_parts(type_id_hash, version, payload_len);
    let mut out = Vec::with_capacity(BinaryHeader::SIZE + payload.len());
    out.extend_from_slice(&header.write_bytes());
    out.extend_from_slice(payload.as_slice());
    Ok(out)
}

/// Zero-copy access to an archived `T` inside an HSER blob.
///
/// When [`MigrationRegistry::current_version`] is set for `type_name`, the header schema must match
/// that version. Automatic migration of rkyv payloads is not supported; use the text or
/// [`MigrationValue`](crate::migration::MigrationValue) migration path for upgrades.
pub fn deserialize_binary<'a, T>(
    data: &'a [u8],
    type_name: &str,
    migration: &MigrationRegistry,
) -> Result<&'a T::Archived, DeserializeError>
where
    T: Archive,
    T::Archived: for<'b> CheckBytes<rkyv::api::high::HighValidator<'b, rancor::Error>>,
{
    let header = BinaryHeader::read_bytes(data)?;
    if header.magic != MAGIC {
        return Err(DeserializeError::BadMagic { offset: 0 });
    }
    let expected_hash = stable_type_name_hash(type_name);
    if header.type_id_hash != expected_hash {
        return Err(DeserializeError::InvalidLayout {
            offset: 8,
            message: format!(
                "type_id_hash mismatch (expected {expected_hash}, got {})",
                header.type_id_hash
            ),
        });
    }
    let stored = header.schema_version();
    if let Some(current) = migration.current_version(type_name) {
        if stored != current {
            if stored > current {
                return Err(DeserializeError::UnsupportedSchemaVersion {
                    found: stored,
                    max: current,
                });
            }
            return Err(DeserializeError::InvalidLayout {
                offset: 16,
                message: format!(
                    "stored schema {stored:?} is behind declared current {current:?}; \
                     rkyv binary payloads cannot be auto-migrated in this crate"
                ),
            });
        }
    }
    let start = BinaryHeader::SIZE;
    let len = usize::try_from(header.payload_len).map_err(|_| DeserializeError::InvalidLayout {
        offset: 24,
        message: "payload_len does not fit usize".into(),
    })?;
    let end = start
        .checked_add(len)
        .ok_or_else(|| DeserializeError::InvalidLayout {
            offset: 24,
            message: "payload end overflow".into(),
        })?;
    if end > data.len() {
        return Err(DeserializeError::Truncated { offset: start });
    }
    let payload = &data[start..end];
    access::<T::Archived, rancor::Error>(payload).map_err(|e| DeserializeError::ArchiveAccess {
        offset: start,
        message: format!("{e:?}"),
    })
}

/// Access a bare rkyv root without an HSER header (mmap-friendly helper).
pub fn access_archived<'a, T>(data: &'a [u8]) -> Result<&'a T::Archived, DeserializeError>
where
    T: Archive,
    T::Archived: for<'b> CheckBytes<rkyv::api::high::HighValidator<'b, rancor::Error>>,
{
    access::<T::Archived, rancor::Error>(data).map_err(|e| DeserializeError::ArchiveAccess {
        offset: 0,
        message: format!("{e:?}"),
    })
}

/// Deserializes an owned `T` after validating the HSER envelope.
pub fn deserialize_binary_owned<T>(
    data: &[u8],
    type_name: &str,
    migration: &MigrationRegistry,
) -> Result<T, DeserializeError>
where
    T: Archive,
    T::Archived: for<'b> CheckBytes<rkyv::api::high::HighValidator<'b, rancor::Error>>,
    T::Archived: Deserialize<T, rkyv::api::high::HighDeserializer<rancor::Error>>,
{
    let archived = deserialize_binary::<T>(data, type_name, migration)?;
    rkyv::deserialize::<T, rancor::Error>(archived).map_err(|e| DeserializeError::ArchiveAccess {
        offset: 0,
        message: format!("{e:?}"),
    })
}
