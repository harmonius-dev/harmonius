//! Save system ↔ rkyv serialization integration (IR-5.10.x).
//!
//! This crate implements the integration envelope from
//! `docs/design/integration/save-system-serialization.md`: separate rkyv
//! archives for [`SaveFileHeader`](types::SaveFileHeader) and
//! `Vec<`[`EntitySnapshot`](types::EntitySnapshot)`>`, plus CRC-32C over the
//! sealed payload tail, optional LZ4/Zstd compression, and optional AES-256-GCM.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod arena;
mod error;
mod format;
mod migration;
mod pipeline;
mod types;

pub use arena::SaveArena;
pub use error::{LoadError, SaveError};
pub use format::{decode_save_file, encode_save_file, SaveDecodeParams};
pub use migration::{MigrationRegistry, MigrationStep};
pub use pipeline::{compress_payload, crc32c, decompress_payload, decrypt_payload, encrypt_payload};
pub use types::{
    deserialize_component, serialize_component, ComponentSnapshot, ComponentVersion,
    CompressionKind, EncryptionKind, EntitySnapshot, Saveable, SaveDirty, SaveEnvelopeParams,
    SaveFileHeader, SchemaVersion, WorldDimension, SAVE_MAGIC,
};

#[cfg(test)]
mod tests {
    use rkyv::api::high::access;
    use rkyv_derive::{Archive, Deserialize, Serialize};

    use super::arena::SaveArena;
    use super::compress_payload;
    use super::error::LoadError;
    use super::format::{decode_save_file, encode_save_file, SaveDecodeParams};
    use super::migration::{MigrationRegistry, MigrationStep};
    use super::types::{
        deserialize_component, serialize_component, CompressionKind, EncryptionKind, EntitySnapshot,
        Saveable, SaveEnvelopeParams, SchemaVersion, WorldDimension,
    };

    const TEST_KEY: [u8; 32] = [7u8; 32];
    const WRONG_KEY: [u8; 32] = [9u8; 32];
    const TRANSFORM_TYPE_HASH: u64 = 0x5452_414E_5346_524D;

    #[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[rkyv(compare(PartialEq), derive(Debug, PartialEq))]
    struct Transform {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Saveable for Transform {
        const TYPE_HASH: u64 = TRANSFORM_TYPE_HASH;
        const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
    }

    fn sample_entities() -> Vec<EntitySnapshot> {
        let comp = serialize_component(&Transform {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
        .unwrap();
        vec![EntitySnapshot {
            stable_id: 1,
            components: vec![comp],
        }]
    }

    fn base_params() -> SaveEnvelopeParams {
        SaveEnvelopeParams {
            schema_version: SchemaVersion::new(1, 2, 0),
            component_versions: Vec::new(),
            compression: CompressionKind::None,
            encryption: EncryptionKind::None,
            dimension: WorldDimension::D3,
            created_at_unix: 0,
        }
    }

    /// TC-IR-5.10.1.1 — Serialize single Saveable component.
    #[test]
    fn tc_ir_5_10_1_1_single_saveable_component() {
        let snap = serialize_component(&Transform {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
        .unwrap();
        assert_eq!(snap.type_hash, TRANSFORM_TYPE_HASH);
        assert_eq!(snap.schema_version, SchemaVersion::new(1, 0, 0));
        assert!(!snap.data.is_empty());
    }

    /// TC-IR-5.10.1.4 — Round-trip serialize/deserialize for entity snapshots.
    #[test]
    fn tc_ir_5_10_1_4_entity_snapshot_roundtrip() {
        let entities = sample_entities();
        let params = base_params();
        let bytes = encode_save_file(&entities, &params, None).unwrap();
        let decode = SaveDecodeParams {
            current_schema: params.schema_version,
            key: None,
        };
        let reg = MigrationRegistry::new();
        let back = decode_save_file(&bytes, &decode, &reg).unwrap();
        assert_eq!(back.len(), entities.len());
        let t: Transform = deserialize_component(&back[0].components[0]).unwrap();
        assert_eq!(
            t,
            Transform {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
    }

    /// TC-IR-5.10.2.1 — Zero-copy header validation on a mapped slice.
    #[test]
    fn tc_ir_5_10_2_1_zero_copy_header_slice() {
        let entities = sample_entities();
        let params = base_params();
        let bytes = encode_save_file(&entities, &params, None).unwrap();
        let header_len = usize::try_from(u64::from_le_bytes(bytes[0..8].try_into().unwrap()))
            .unwrap();
        let header_slice = &bytes[8..8 + header_len];
        let _ = access::<<super::SaveFileHeader as rkyv::Archive>::Archived, rkyv::rancor::Error>(
            header_slice,
        )
        .unwrap();
    }

    /// TC-IR-5.10.2.2 — CRC-32C matches on load.
    #[test]
    fn tc_ir_5_10_2_2_crc_ok() {
        let entities = sample_entities();
        let params = base_params();
        let bytes = encode_save_file(&entities, &params, None).unwrap();
        let decode = SaveDecodeParams {
            current_schema: params.schema_version,
            key: None,
        };
        let reg = MigrationRegistry::new();
        decode_save_file(&bytes, &decode, &reg).unwrap();
    }

    /// TC-IR-5.10.2.3 — Flipped bit in sealed payload fails checksum.
    #[test]
    fn tc_ir_5_10_2_3_checksum_mismatch() {
        let entities = sample_entities();
        let params = base_params();
        let mut bytes = encode_save_file(&entities, &params, None).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 1;
        let decode = SaveDecodeParams {
            current_schema: params.schema_version,
            key: None,
        };
        let reg = MigrationRegistry::new();
        let err = decode_save_file(&bytes, &decode, &reg).unwrap_err();
        assert!(matches!(err, LoadError::ChecksumMismatch));
    }

    /// TC-IR-5.10.2.5 — Oversized header length prefix.
    #[test]
    fn tc_ir_5_10_2_5_invalid_header_length() {
        let mut bytes = vec![0xFFu8; 16];
        bytes[0..8].copy_from_slice(&u64::MAX.to_le_bytes());
        let decode = SaveDecodeParams {
            current_schema: SchemaVersion::new(1, 0, 0),
            key: None,
        };
        let reg = MigrationRegistry::new();
        let err = decode_save_file(&bytes, &decode, &reg).unwrap_err();
        assert!(matches!(err, LoadError::InvalidHeader));
    }

    /// TC-IR-5.10.3.1 — Schema version stored in header matches params.
    #[test]
    fn tc_ir_5_10_3_1_schema_version_in_header() {
        let entities = sample_entities();
        let mut params = base_params();
        params.schema_version = SchemaVersion::new(1, 2, 0);
        let bytes = encode_save_file(&entities, &params, None).unwrap();
        let header_len = usize::try_from(u64::from_le_bytes(bytes[0..8].try_into().unwrap()))
            .unwrap();
        let header_slice = &bytes[8..8 + header_len];
        let archived =
            access::<<super::SaveFileHeader as rkyv::Archive>::Archived, rkyv::rancor::Error>(
                header_slice,
            )
            .unwrap();
        let header: super::SaveFileHeader =
            rkyv::deserialize::<super::SaveFileHeader, rkyv::rancor::Error>(archived).unwrap();
        assert_eq!(header.schema_version, SchemaVersion::new(1, 2, 0));
    }

    /// TC-IR-5.10.3.2 — Two migration steps run in registration order.
    #[test]
    fn tc_ir_5_10_3_2_migration_chain_order() {
        let entities = sample_entities();
        let mut params = base_params();
        params.schema_version = SchemaVersion::new(1, 0, 0);
        let raw = rkyv::to_bytes::<rkyv::rancor::Error>(&entities).unwrap();
        let mut reg = MigrationRegistry::new();
        reg.register(MigrationStep {
            type_hash: 0,
            from: SchemaVersion::new(1, 0, 0),
            to: SchemaVersion::new(2, 0, 0),
            apply: |b| Ok([b, b"v1"].concat()),
        });
        reg.register(MigrationStep {
            type_hash: 0,
            from: SchemaVersion::new(2, 0, 0),
            to: SchemaVersion::new(3, 0, 0),
            apply: |b| Ok([b, b"v2"].concat()),
        });
        let migrated = reg.migrate_all(raw.to_vec()).unwrap();
        assert!(migrated.ends_with(b"v1v2"));
    }

    /// TC-IR-5.10.3.5 — Migration failure surfaces [`LoadError::MigrationFailed`].
    #[test]
    fn tc_ir_5_10_3_5_migration_failure_propagates() {
        let entities = sample_entities();
        let mut params = base_params();
        params.schema_version = SchemaVersion::new(1, 0, 0);
        let mut reg = MigrationRegistry::new();
        reg.register(MigrationStep {
            type_hash: 1,
            from: SchemaVersion::new(1, 0, 0),
            to: SchemaVersion::new(2, 0, 0),
            apply: |_| Err("boom"),
        });
        let bytes = encode_save_file(&entities, &params, None).unwrap();
        let decode = SaveDecodeParams {
            current_schema: SchemaVersion::new(3, 0, 0),
            key: None,
        };
        let err = decode_save_file(&bytes, &decode, &reg).unwrap_err();
        assert!(matches!(err, LoadError::MigrationFailed { .. }));
    }

    /// TC-IR-5.10.1.5 — Arena overflow with bounded growth retries.
    #[test]
    fn tc_ir_5_10_1_5_arena_overflow_retries() {
        let mut arena = SaveArena::new(256 * 1024);
        let required = 700 * 1024;
        assert!(arena.try_reserve(required).is_err());
        arena.grow_to(512 * 1024);
        assert!(arena.try_reserve(required).is_err());
        arena.grow_to(1024 * 1024);
        arena.try_reserve(required).unwrap();
    }

    /// TC-IR-5.10.6.1 — LZ4 reduces size for compressible payload.
    #[test]
    fn tc_ir_5_10_6_1_lz4_smaller_than_none() {
        let payload = vec![0u8; 4096];
        let none = compress_payload(&payload, CompressionKind::None).unwrap();
        let lz4 = compress_payload(&payload, CompressionKind::Lz4).unwrap();
        assert!(lz4.len() < none.len());
    }

    /// TC-IR-5.10.6.2 — Zstd improves ratio vs LZ4 on repetitive data.
    #[test]
    fn tc_ir_5_10_6_2_zstd_smaller_than_lz4() {
        let payload = vec![0xABu8; 16 * 1024];
        let lz4 = compress_payload(&payload, CompressionKind::Lz4).unwrap();
        let zstd = compress_payload(&payload, CompressionKind::Zstd).unwrap();
        assert!(zstd.len() < lz4.len());
    }

    /// TC-IR-5.10.6.3 — AES-256-GCM round-trip on the sealed payload path.
    #[test]
    fn tc_ir_5_10_6_3_aes_gcm_roundtrip() {
        let entities = sample_entities();
        let mut params = base_params();
        params.encryption = EncryptionKind::Aes256Gcm;
        let bytes = encode_save_file(&entities, &params, Some(&TEST_KEY)).unwrap();
        let decode = SaveDecodeParams {
            current_schema: params.schema_version,
            key: Some(&TEST_KEY),
        };
        let reg = MigrationRegistry::new();
        let back = decode_save_file(&bytes, &decode, &reg).unwrap();
        assert_eq!(back.len(), entities.len());
    }

    /// TC-IR-5.10.6.4 — Wrong key fails decryption.
    #[test]
    fn tc_ir_5_10_6_4_wrong_key_fails() {
        let entities = sample_entities();
        let mut params = base_params();
        params.encryption = EncryptionKind::Aes256Gcm;
        let bytes = encode_save_file(&entities, &params, Some(&TEST_KEY)).unwrap();
        let decode = SaveDecodeParams {
            current_schema: params.schema_version,
            key: Some(&WRONG_KEY),
        };
        let reg = MigrationRegistry::new();
        let err = decode_save_file(&bytes, &decode, &reg).unwrap_err();
        assert!(matches!(err, LoadError::DecryptionFailed(_)));
    }

    /// TC-IR-5.10.2.4 — Full save/load round-trip for a multi-entity snapshot list.
    #[test]
    fn tc_ir_5_10_2_4_full_roundtrip_many_entities() {
        let mut entities = Vec::new();
        for i in 0..100u64 {
            let comp = serialize_component(&Transform {
                x: i as f32,
                y: (i + 1) as f32,
                z: (i + 2) as f32,
            })
            .unwrap();
            entities.push(EntitySnapshot {
                stable_id: i + 1,
                components: vec![comp],
            });
        }
        let params = base_params();
        let bytes = encode_save_file(&entities, &params, None).unwrap();
        let decode = SaveDecodeParams {
            current_schema: params.schema_version,
            key: None,
        };
        let reg = MigrationRegistry::new();
        let back = decode_save_file(&bytes, &decode, &reg).unwrap();
        assert_eq!(back.len(), 100);
        for (a, b) in back.iter().zip(entities.iter()) {
            assert_eq!(a.stable_id, b.stable_id);
            let ta: Transform = deserialize_component(&a.components[0]).unwrap();
            let tb: Transform = deserialize_component(&b.components[0]).unwrap();
            assert_eq!(ta, tb);
        }
    }

    /// TC-IR-5.10.1.2 — Serialize entity with five components (deterministic order).
    #[test]
    fn tc_ir_5_10_1_2_five_components() {
        #[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        #[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
        struct C1 {
            v: u32,
        }
        #[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        #[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
        struct C2 {
            v: u32,
        }
        #[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        #[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
        struct C3 {
            v: u32,
        }
        #[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        #[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
        struct C4 {
            v: u32,
        }
        #[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        #[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
        struct C5 {
            v: u32,
        }
        impl Saveable for C1 {
            const TYPE_HASH: u64 = 1;
            const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
        }
        impl Saveable for C2 {
            const TYPE_HASH: u64 = 2;
            const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
        }
        impl Saveable for C3 {
            const TYPE_HASH: u64 = 3;
            const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
        }
        impl Saveable for C4 {
            const TYPE_HASH: u64 = 4;
            const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
        }
        impl Saveable for C5 {
            const TYPE_HASH: u64 = 5;
            const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
        }
        let comps = vec![
            serialize_component(&C1 { v: 1 }).unwrap(),
            serialize_component(&C2 { v: 2 }).unwrap(),
            serialize_component(&C3 { v: 3 }).unwrap(),
            serialize_component(&C4 { v: 4 }).unwrap(),
            serialize_component(&C5 { v: 5 }).unwrap(),
        ];
        let ent = EntitySnapshot {
            stable_id: 99,
            components: comps,
        };
        assert_eq!(ent.components.len(), 5);
        let hashes: Vec<u64> = ent.components.iter().map(|c| c.type_hash).collect();
        assert_eq!(hashes, vec![1, 2, 3, 4, 5]);
    }

    /// TC-IR-5.10.1.3 — Non-Saveable components are excluded by construction.
    #[test]
    fn tc_ir_5_10_1_3_only_saveable_in_snapshot_list() {
        #[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        #[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
        struct SaveA {
            v: u32,
        }
        #[derive(Archive, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
        #[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
        struct SaveB {
            v: u32,
        }
        impl Saveable for SaveA {
            const TYPE_HASH: u64 = 10;
            const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
        }
        impl Saveable for SaveB {
            const TYPE_HASH: u64 = 11;
            const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0, 0);
        }
        let comps = vec![
            serialize_component(&SaveA { v: 1 }).unwrap(),
            serialize_component(&SaveB { v: 2 }).unwrap(),
        ];
        assert_eq!(comps.len(), 2);
    }

}
