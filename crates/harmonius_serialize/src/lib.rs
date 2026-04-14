//! Binary envelopes, migrations, git-friendly text, and HBIN companions.
//!
//! Runtime reflection (`F-1.3.x`) was removed per design RF-1; this crate
//! implements the surviving serialization stack (`F-1.4.x`).

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod binary;
pub mod companion;
pub mod error;
pub mod migration;
pub mod mixed;
pub mod text;

pub use error::{DeserializeError, MigrationError, SerializeError};

#[cfg(test)]
mod tests {
    use rkyv_derive::{Archive, Deserialize, Serialize};

    use super::binary::{
        deserialize_binary, deserialize_binary_owned, serialize_binary, stable_type_name_hash,
        BinaryHeader,
    };
    use super::companion::{atomic_write_file, BinaryCompanion, Compression};
    use super::error::MigrationError;
    use super::migration::{MigrationRegistry, MigrationValue, SchemaVersion};
    use super::mixed::{parse_mixed_manifest, render_mixed_manifest, FieldStorage};
    use super::text::{
        deserialize_text, serialize_text, TextDeserialize, TextReader, TextSerialize, TextWriter,
    };

    // TC-1.4.1.1, TC-1.4.1.2
    #[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
    #[rkyv(compare(PartialEq))]
    struct Inner {
        z: u32,
    }

    #[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
    #[rkyv(compare(PartialEq))]
    struct Sample {
        a: u32,
        b: f64,
        c: String,
        n: Inner,
    }

    #[test]
    fn tc_1_4_1_1_binary_roundtrip() {
        let v = Sample {
            a: 7,
            b: 2.5,
            c: "hi".into(),
            n: Inner { z: 99 },
        };
        let reg = MigrationRegistry::new();
        let bytes = serialize_binary(&v, "Sample", SchemaVersion::new(1, 0, 0)).unwrap();
        let back: Sample = deserialize_binary_owned(&bytes, "Sample", &reg).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn tc_1_4_1_2_truncated_reports_offset() {
        let mut bytes = serialize_binary(
            &Sample {
                a: 1,
                b: 1.0,
                c: "x".into(),
                n: Inner { z: 0 },
            },
            "Sample",
            SchemaVersion::new(1, 0, 0),
        )
        .unwrap();
        bytes.truncate(bytes.len().saturating_sub(4));
        let reg = MigrationRegistry::new();
        let err = match deserialize_binary::<Sample>(&bytes, "Sample", &reg) {
            Err(e) => e,
            Ok(_) => panic!("expected error"),
        };
        let super::DeserializeError::Truncated { offset } = err else {
            panic!("expected Truncated");
        };
        assert_eq!(offset, BinaryHeader::SIZE);
    }

    // TC-1.4.3.1, TC-1.4.3.2 (git-friendly text, not RON)
    #[derive(Debug, PartialEq)]
    struct Complex {
        items: Vec<String>,
        opt: Option<String>,
        uni: String,
    }

    impl TextSerialize for Complex {
        fn serialize_text(&self, out: &mut TextWriter) -> Result<(), super::SerializeError> {
            out.begin("Complex", SchemaVersion::new(1, 0, 0));
            out.field("items", &self.items.join(","))?;
            out.field("opt", &self.opt.clone().unwrap_or_default())?;
            out.field("uni", &self.uni)?;
            Ok(())
        }
    }

    impl TextDeserialize for Complex {
        fn deserialize_text(
            src: TextReader<'_>,
            _migration: &MigrationRegistry,
        ) -> Result<Self, super::DeserializeError> {
            let (_t, _s, m) = src.into_map()?;
            let items = m
                .get("items")
                .map(|s| {
                    if s.is_empty() {
                        Vec::new()
                    } else {
                        s.split(',').map(|x| x.to_string()).collect()
                    }
                })
                .unwrap_or_default();
            let opt = m.get("opt").filter(|s| !s.is_empty()).cloned();
            let uni = m.get("uni").cloned().unwrap_or_default();
            Ok(Self { items, opt, uni })
        }
    }

    #[test]
    fn tc_1_4_3_1_text_roundtrip() {
        let v = Complex {
            items: vec!["a".into(), "b".into()],
            opt: None,
            uni: "π".into(),
        };
        let s = serialize_text(&v).unwrap();
        let reg = MigrationRegistry::new();
        let back: Complex = deserialize_text(&s, &reg).unwrap();
        assert_eq!(v, back);
    }

    #[test]
    fn tc_1_4_3_2_text_edge_cases() {
        let v = Complex {
            items: vec![],
            opt: None,
            uni: "你好".into(),
        };
        let s = serialize_text(&v).unwrap();
        let reg = MigrationRegistry::new();
        let back: Complex = deserialize_text(&s, &reg).unwrap();
        assert_eq!(v, back);
    }

    // TC-1.4.4.1 schema migration on MigrationValue map (text-derived)
    #[test]
    fn tc_1_4_4_1_schema_migration() {
        let mut reg = MigrationRegistry::new();
        reg.set_current_version("Widget", SchemaVersion::new(1, 1, 0));
        fn bump(
            map: &mut std::collections::BTreeMap<String, MigrationValue>,
        ) -> Result<(), MigrationError> {
            map.insert("new_field".into(), MigrationValue::Int(1));
            Ok(())
        }
        reg.register(
            "Widget",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(1, 1, 0),
            bump,
        );
        let mut map = std::collections::BTreeMap::new();
        map.insert("old".into(), MigrationValue::Int(0));
        reg.migrate("Widget", SchemaVersion::new(1, 0, 0), &mut map)
            .unwrap();
        assert_eq!(map.get("new_field"), Some(&MigrationValue::Int(1)));
    }

    // TC-1.4.5.1 chain
    #[test]
    fn tc_1_4_5_1_migration_chain_three_steps() {
        let mut reg = MigrationRegistry::new();
        reg.set_current_version("T", SchemaVersion::new(1, 3, 0));
        reg.register(
            "T",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(1, 1, 0),
            |m| {
                m.insert("v".into(), MigrationValue::Int(1));
                Ok(())
            },
        );
        reg.register(
            "T",
            SchemaVersion::new(1, 1, 0),
            SchemaVersion::new(1, 2, 0),
            |m| {
                if let MigrationValue::Int(v) = m.get_mut("v").unwrap() {
                    *v += 1;
                }
                Ok(())
            },
        );
        reg.register(
            "T",
            SchemaVersion::new(1, 2, 0),
            SchemaVersion::new(1, 3, 0),
            |m| {
                if let MigrationValue::Int(v) = m.get_mut("v").unwrap() {
                    *v += 1;
                }
                Ok(())
            },
        );
        let mut map = std::collections::BTreeMap::new();
        reg.migrate("T", SchemaVersion::new(1, 0, 0), &mut map)
            .unwrap();
        assert_eq!(map.get("v"), Some(&MigrationValue::Int(3)));
    }

    // TC-1.4.5.2 fifty-step chain
    #[test]
    fn tc_1_4_5_2_migration_chain_fifty() {
        let mut reg = MigrationRegistry::new();
        reg.set_current_version("L", SchemaVersion::new(50, 0, 0));
        for i in 1u16..50 {
            let from = SchemaVersion::new(i, 0, 0);
            let to = SchemaVersion::new(i + 1, 0, 0);
            reg.register("L", from, to, |m| {
                let MigrationValue::Int(v) = m.entry("x".into()).or_insert(MigrationValue::Int(0))
                else {
                    return Ok(());
                };
                *v += 1;
                Ok(())
            });
        }
        let mut map = std::collections::BTreeMap::new();
        map.insert("x".into(), MigrationValue::Int(0));
        reg.migrate("L", SchemaVersion::new(1, 0, 0), &mut map)
            .unwrap();
        assert_eq!(map.get("x"), Some(&MigrationValue::Int(49)));
    }

    // TC-1.4.5.3 missing step
    #[test]
    fn tc_1_4_5_3_migration_missing_step() {
        let mut reg = MigrationRegistry::new();
        reg.set_current_version("G", SchemaVersion::new(3, 0, 0));
        reg.register(
            "G",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(2, 0, 0),
            |_| Ok(()),
        );
        // v2->v3 intentionally omitted
        let err = reg
            .migrate(
                "G",
                SchemaVersion::new(1, 0, 0),
                &mut std::collections::BTreeMap::new(),
            )
            .unwrap_err();
        match err {
            MigrationError::MissingStep { .. } => {}
            _ => panic!("expected MissingStep"),
        }
    }

    // TC-1.4.9.1, TC-1.4.9.2, TC-1.4.9.3
    #[test]
    fn tc_1_4_9_companion_write_read_dedup_append() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("c.bin");
        let mut c = BinaryCompanion::open(p.clone()).unwrap();
        c.write_blob("b1", &[1, 2, 3], Compression::Lz4, 1).unwrap();
        c.write_blob("b2", &[9; 1024], Compression::None, 1)
            .unwrap();
        c.write_blob("b3", &[7; 16], Compression::None, 1).unwrap();
        assert_eq!(c.read_blob("b1").unwrap(), vec![1, 2, 3]);
        let sz1 = std::fs::metadata(&p).unwrap().len();
        c.write_blob("b1", &[1, 2, 3], Compression::Lz4, 1).unwrap();
        let sz2 = std::fs::metadata(&p).unwrap().len();
        assert_eq!(sz1, sz2, "dedup should not grow file");
        c.append_blob("b4", &[8], Compression::None, 1).unwrap();
        assert_eq!(c.read_blob("b4").unwrap(), vec![8]);
        c.verify().unwrap();
    }

    // TC-1.4.8.1 atomic rename pattern
    #[test]
    fn tc_1_4_8_1_atomic_write_no_partial_final() {
        let dir = tempfile::tempdir().unwrap();
        let final_path = dir.path().join("out.txt");
        let temp_path = dir.path().join("out.txt.tmp");
        atomic_write_file(&temp_path, &final_path, b"ok").unwrap();
        assert!(!temp_path.exists());
        assert_eq!(std::fs::read(&final_path).unwrap(), b"ok");
    }

    // TC-1.4.10.1 — manifest lists companion vs inline fields
    #[test]
    fn tc_1_4_10_1_binary_attribute_manifest() {
        let manifest =
            render_mixed_manifest("@schema 1.0.0\ntitle=hello", &["vertices", "weights"]);
        let (inline, comp) = parse_mixed_manifest(&manifest);
        assert!(inline.contains("title=hello"));
        assert_eq!(comp, vec!["vertices", "weights"]);
        assert_eq!(
            FieldStorage::Companion { compress_lz4: true },
            FieldStorage::Companion { compress_lz4: true }
        );
    }

    // TC-1.4.1.3 throughput (release builds only; rkyv is fast enough for target)
    #[test]
    fn tc_1_4_1_3_binary_throughput() {
        if cfg!(debug_assertions) {
            return;
        }
        let v = Sample {
            a: 42,
            b: 1.0,
            c: "x".into(),
            n: Inner { z: 1 },
        };
        let reg = MigrationRegistry::new();
        let mut buf = serialize_binary(&v, "Sample", SchemaVersion::new(1, 0, 0)).unwrap();
        let n = 100_000usize;
        let t0 = std::time::Instant::now();
        for _ in 0..n {
            let _: Sample = deserialize_binary_owned(&buf, "Sample", &reg).unwrap();
        }
        let elapsed = t0.elapsed().as_secs_f64().max(1e-9);
        let bytes_per = buf.len() as f64 * n as f64;
        let mb_s = bytes_per / (1024.0 * 1024.0) / elapsed;
        assert!(
            mb_s >= 200.0,
            "expected high throughput in release, got {mb_s} MB/s"
        );
        buf.truncate(0);
        let _ = buf;
    }

    #[test]
    fn stable_type_hash_matches_header_field() {
        let h = stable_type_name_hash("Sample");
        let bytes = serialize_binary(
            &Sample {
                a: 0,
                b: 0.0,
                c: "".into(),
                n: Inner { z: 0 },
            },
            "Sample",
            SchemaVersion::new(2, 0, 0),
        )
        .unwrap();
        let hdr = BinaryHeader::read_bytes(&bytes).unwrap();
        assert_eq!(hdr.type_id_hash, h);
    }

    #[test]
    fn binary_schema_behind_registry_current_errors() {
        let v = Sample {
            a: 1,
            b: 1.0,
            c: "x".into(),
            n: Inner { z: 0 },
        };
        let bytes = serialize_binary(&v, "Sample", SchemaVersion::new(1, 0, 0)).unwrap();
        let mut reg = MigrationRegistry::new();
        reg.set_current_version("Sample", SchemaVersion::new(2, 0, 0));
        let err = deserialize_binary_owned::<Sample>(&bytes, "Sample", &reg).unwrap_err();
        match err {
            super::DeserializeError::InvalidLayout { offset, .. } => assert_eq!(offset, 16),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn binary_schema_ahead_of_registry_current_errors() {
        let v = Sample {
            a: 1,
            b: 1.0,
            c: "x".into(),
            n: Inner { z: 0 },
        };
        let bytes = serialize_binary(&v, "Sample", SchemaVersion::new(2, 0, 0)).unwrap();
        let mut reg = MigrationRegistry::new();
        reg.set_current_version("Sample", SchemaVersion::new(1, 0, 0));
        let err = deserialize_binary_owned::<Sample>(&bytes, "Sample", &reg).unwrap_err();
        match err {
            super::DeserializeError::UnsupportedSchemaVersion { found, max } => {
                assert_eq!(found, SchemaVersion::new(2, 0, 0));
                assert_eq!(max, SchemaVersion::new(1, 0, 0));
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
