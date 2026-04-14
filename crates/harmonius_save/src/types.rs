//! Core save-system value types.

use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;

/// Stable schema version triple carried in headers and metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

/// Save slot index (1-based paths use zero-padded formatting in the design).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SlotId(pub u32);

/// Logical save partition used by serializers (design: codegen expands variants).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SaveContext {
    Character,
    World,
    Instance,
    Settings,
}

/// I/O scheduling hint for the save pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IoPriority {
    Background,
    Normal,
    Critical,
}

/// How manual save was initiated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SaveType {
    Manual,
    Autosave,
    Quicksave,
    Checkpoint,
    CloudSync,
}

/// Opaque platform id placeholder until platform crate lands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlatformId(pub u32);

/// Opaque thumbnail handle placeholder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThumbnailAsset(pub u64);

/// Localized string id placeholder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalizedStringId(pub u32);

/// String table id placeholder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringId(pub u32);

/// Loose-typed custom field value placeholder.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    U64(u64),
    I64(i64),
    F32(f32),
    Str(String),
}

/// Per-slot metadata stored in `slot_NN.meta`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveSlotMeta {
    pub id: SlotId,
    pub name: String,
    pub character_name: String,
    pub level: u32,
    pub character_class: Option<u32>,
    pub character_appearance: Option<ThumbnailAsset>,
    pub party_members: SmallVec<[LocalizedStringId; 4]>,
    pub playtime_seconds: u64,
    pub zone_name: String,
    pub completion_percentage: f32,
    pub chapter: Option<LocalizedStringId>,
    pub difficulty: Option<u32>,
    pub quest_summary: SmallVec<[LocalizedStringId; 4]>,
    pub deaths: u32,
    pub currency: u64,
    pub real_world_date: i64,
    pub engine_version: SchemaVersion,
    pub platform: PlatformId,
    pub save_type: SaveType,
    pub is_autosave: bool,
    pub schema_version: SchemaVersion,
    pub content_hash: [u8; 32],
    pub thumbnail: ThumbnailAsset,
    pub custom_fields: SmallVec<[(StringId, Value); 8]>,
}

impl SaveSlotMeta {
    /// Deterministic fixture builder for tests (TC-13.3.4.1).
    pub fn fixture(id: SlotId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            character_name: "Hero".to_string(),
            level: 7,
            character_class: Some(2),
            character_appearance: Some(ThumbnailAsset(9)),
            party_members: SmallVec::new(),
            playtime_seconds: 3600,
            zone_name: "TestZone".to_string(),
            completion_percentage: 42.5,
            chapter: Some(LocalizedStringId(11)),
            difficulty: Some(1),
            quest_summary: SmallVec::new(),
            deaths: 3,
            currency: 1200,
            real_world_date: 1_700_000_000,
            engine_version: SchemaVersion {
                major: 0,
                minor: 2,
                patch: 0,
            },
            platform: PlatformId(1),
            save_type: SaveType::Manual,
            is_autosave: false,
            schema_version: SchemaVersion {
                major: 1,
                minor: 0,
                patch: 0,
            },
            content_hash: [0u8; 32],
            thumbnail: ThumbnailAsset(99),
            custom_fields: SmallVec::new(),
        }
    }
}

/// Compression mode for the save pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionMode {
    Lz4,
    Zstd { level: i32 },
    None,
}

/// Key material source (design: platform keystore integration later).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeySource {
    /// Test-only fixed key path.
    FixedKey,
    PlatformKeystore,
    HardwareBound,
}

/// Encryption configuration for [`crate::pipeline::SavePipeline`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub key_source: KeySource,
}

/// Top-level save tuning (design `SaveConfig`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveConfig {
    pub max_slots: u32,
    pub autosave_rotation: u32,
    pub autosave_interval_secs: u32,
    pub local_compression: CompressionMode,
    pub save_dir: String,
}

/// High-level save/load lifecycle events (design `SaveEvent`).
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum SaveEvent {
    /// A save operation began for `slot`.
    SaveStarted { slot: SlotId },
    /// A save operation completed with fresh metadata.
    SaveComplete { slot: SlotId, meta: SaveSlotMeta },
    /// A save operation failed.
    SaveFailed {
        slot: SlotId,
        /// Serialized error for lightweight event comparisons in tests.
        detail: String,
    },
    /// Autosave timer fired for `slot`.
    AutosaveTriggered { slot: SlotId },
}
