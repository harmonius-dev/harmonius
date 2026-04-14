//! Core save file types, markers, and component (de)serialization helpers.

use rkyv::api::high::access;
use rkyv::bytecheck::CheckBytes;
use rkyv::rancor;
use rkyv::ser::allocator::ArenaHandle;
use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Serialize};

use crate::error::{LoadError, SaveError};

/// On-disk magic for Harmonius save envelopes (`SAV1`).
pub const SAVE_MAGIC: [u8; 4] = *b"SAV1";

/// Semantic schema version carried in headers and snapshots.
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, Ord, PartialEq, PartialOrd))]
pub struct SchemaVersion {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
    /// Patch version.
    pub patch: u16,
}

impl SchemaVersion {
    /// Builds a schema version from triple parts.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Compression applied to the serialized entity snapshot archive.
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub enum CompressionKind {
    /// No compression.
    None,
    /// LZ4 block compression with size prefix (lz4_flex).
    Lz4,
    /// Zstandard compression.
    Zstd,
}

/// Encryption applied after compression.
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub enum EncryptionKind {
    /// No encryption.
    None,
    /// AES-256-GCM with a random 12-byte nonce prepended to ciphertext.
    Aes256Gcm,
    /// ChaCha20-Poly1305 (reserved; not implemented in this slice).
    ChaCha20Poly1305,
}

/// World dimension tag stored in the save header.
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub enum WorldDimension {
    /// Pure 2D simulation data.
    D2,
    /// 2D with depth sorting.
    D2_5,
    /// Full 3D simulation data.
    D3,
}

/// Per-component schema version table entry in the save header.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct ComponentVersion {
    /// Stable component type hash.
    pub type_hash: u64,
    /// Component schema version.
    pub version: SchemaVersion,
}

/// Mutable inputs for [`crate::format::encode_save_file`] besides the entity list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SaveEnvelopeParams {
    /// Schema version written into the header.
    pub schema_version: SchemaVersion,
    /// Optional per-component version table.
    pub component_versions: Vec<ComponentVersion>,
    /// Compression applied to the entity snapshot archive.
    pub compression: CompressionKind,
    /// Encryption applied after compression.
    pub encryption: EncryptionKind,
    /// World dimension tag stored in the header.
    pub dimension: WorldDimension,
    /// Creation timestamp in unix seconds (tests may use 0).
    pub created_at_unix: u64,
}

/// Header archive stored immediately after the 8-byte little-endian length prefix.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct SaveFileHeader {
    /// File magic (`SAVE_MAGIC`).
    pub magic: [u8; 4],
    /// Schema version for the save file as a whole.
    pub schema_version: SchemaVersion,
    /// Per-component schema versions.
    pub component_versions: Vec<ComponentVersion>,
    /// Compression applied to the sealed payload tail.
    pub compression: CompressionKind,
    /// Encryption applied to the compressed bytes.
    pub encryption: EncryptionKind,
    /// CRC-32C (Castagnoli) over the sealed payload tail.
    pub checksum: u32,
    /// Unix timestamp in seconds (best-effort; tests may use 0).
    pub created_at_unix: u64,
    /// Declared world dimension for the save.
    pub dimension: WorldDimension,
}

/// One serialized Saveable component payload.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct ComponentSnapshot {
    /// Stable type hash for dispatch.
    pub type_hash: u64,
    /// Schema version stamped at serialization time.
    pub schema_version: SchemaVersion,
    /// Raw rkyv bytes for the component value.
    pub data: Vec<u8>,
}

/// One entity worth of Saveable snapshots.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct EntitySnapshot {
    /// Stable entity id used for remap on load.
    pub stable_id: u64,
    /// Serialized Saveable components.
    pub components: Vec<ComponentSnapshot>,
}

/// Marks an entity as dirty for incremental saves (integration design).
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct SaveDirty {
    /// Last simulation tick that dirtied this entity.
    pub dirty_tick: u64,
}

/// Trait implemented by every persisted component type (codegen template in full engine).
pub trait Saveable: Archive + Send + Sync + Sized + 'static {
    /// Stable type hash used in snapshots and migrations.
    const TYPE_HASH: u64;
    /// Schema version for this component type.
    const SCHEMA_VERSION: SchemaVersion;
}

/// Serializes a Saveable component into a [`ComponentSnapshot`].
pub fn serialize_component<T>(component: &T) -> Result<ComponentSnapshot, SaveError>
where
    T: Saveable
        + for<'a> Serialize<
            rkyv::api::high::HighSerializer<AlignedVec, ArenaHandle<'a>, rancor::Error>,
        >,
{
    let bytes =
        rkyv::to_bytes::<rancor::Error>(component).map_err(|e| SaveError::SerializationFailed {
            type_hash: T::TYPE_HASH,
            detail: format!("{e:?}"),
        })?;
    Ok(ComponentSnapshot {
        type_hash: T::TYPE_HASH,
        schema_version: T::SCHEMA_VERSION,
        data: bytes.into_vec(),
    })
}

/// Deserializes a [`ComponentSnapshot`] back into `T`.
pub fn deserialize_component<T>(snapshot: &ComponentSnapshot) -> Result<T, LoadError>
where
    T: Saveable,
    T::Archived: for<'a> CheckBytes<rkyv::api::high::HighValidator<'a, rancor::Error>>,
    T::Archived: Deserialize<T, rkyv::api::high::HighDeserializer<rancor::Error>>,
{
    if snapshot.type_hash != T::TYPE_HASH {
        return Err(LoadError::DeserializationFailed {
            detail: "type_hash mismatch".into(),
        });
    }
    let archived = access::<T::Archived, rancor::Error>(&snapshot.data).map_err(|_| {
        LoadError::DeserializationFailed {
            detail: "archive validation failed".into(),
        }
    })?;
    rkyv::deserialize::<T, rancor::Error>(archived).map_err(|e| LoadError::DeserializationFailed {
        detail: format!("{e:?}"),
    })
}
