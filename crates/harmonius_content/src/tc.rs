//! Companion tests for `docs/design/content-pipeline/asset-pipeline-test-cases.md`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use png::{BitDepth, ColorType, Encoder};

use crate::asset_binary::{AssetReader, AssetWriter, ASSET_MAGIC, FORMAT_VERSION};
use crate::audio_flac::parse_flac_loop_comments;
use crate::audio_wav::parse_wav_with_cue;
use crate::auto_resolve::{auto_resolve_lww, auto_resolve_union_tags};
use crate::batch_import::{simple_meta, BatchImportHandle, ImportEntry};
use crate::bundle::{BundleDecodeStats, BundleReader, BundleWriter};
use crate::cas::{ContentAddressableStore, StoreResult};
use crate::data_table::{edit_data_table_cell, structural_diff_table_cells, DataTable};
use crate::dep_graph::DependencyGraph;
use crate::editor_sync::{EditorMessage, EditorSync, MaterialRuntime};
use crate::file_watch::{AssetChange, DebouncedWatcher, FileEvent};
use crate::handle_table::{HandleTable, PendingSwap, SwapScheduler};
use crate::headless_batch::run_headless_batch_twice;
use crate::logic_graph::{hot_reload_logic_graph, LogicGraphInstance, LogicGraphSpec, LogicReloadEvent};
use crate::material_mapper::{translate_gltf_pbr_material, ImportedMaterial};
use crate::metadata::{AssetMetadata, AssetType, MetadataStore, SearchFilter};
use crate::native_format::{build_native_v1, build_native_v1_wrong_hash, import_native_asset};
use crate::pipeline::{run_pipeline, PipelineError};
use crate::progress::ProgressTracker;
use crate::shader_reload::ShaderReloader;
use crate::structural_diff::{diff_mesh_assets, DiffResult, MeshAssetSummary};
use crate::subasset::{partial_subasset_reimport, CompositeAsset, SubassetEdit};
use crate::texture_decode::{decode_exr_linear, decode_png_srgb, write_exr_linear_fixture, ColorSpace};
use crate::three_way_merge::{three_way_merge_disjoint_graphs, MergeResult};
use crate::ui_reload::{reload_ui_panel_style, UiDocument};
use crate::validation::{import_with_optional_tags_validation, validate_native_version, ValidationWarning};
use crate::version_store::VersionStore;
use crate::visual_inspector::{visual_inspector_fields, WidgetKind};
use crate::{
    AssetId, ContentHash, ImportCoordinator, ImportError, ImportResult, ImportSettings,
    TextureCompression,
};

fn png_4x4_red() -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut enc = Encoder::new(&mut buf, 4, 4);
        enc.set_color(ColorType::Rgba);
        enc.set_depth(BitDepth::Eight);
        let mut w = enc.write_header().unwrap();
        let data: Vec<u8> = [255u8, 0, 0, 255].iter().cycle().take(64).copied().collect();
        w.write_image_data(&data).unwrap();
    }
    buf
}

#[test]
fn test_native_format_magic_valid() {
    let body = vec![1u8; 16];
    let bytes = build_native_v1(&body);
    let r = import_native_asset(&bytes, Path::new("a.har")).unwrap();
    assert_eq!(r.content_hash, ContentHash::from_data(&body));
}

#[test]
fn test_native_format_hash_mismatch() {
    let body = vec![2u8; 32];
    let bytes = build_native_v1_wrong_hash(&body);
    let e = import_native_asset(&bytes, Path::new("x.har")).unwrap_err();
    assert!(matches!(e, ImportError::HashMismatch { .. }));
}

#[test]
fn test_native_format_dedup() {
    let dir = tempfile::tempdir().unwrap();
    let mut cas = ContentAddressableStore::new(dir.path().to_path_buf());
    let body = vec![3u8; 4096];
    let h = ContentHash::from_data(&body);
    assert_eq!(cas.store(h, &body).unwrap(), StoreResult::Written);
    assert_eq!(cas.store(h, &body).unwrap(), StoreResult::Deduplicated);
    assert!(cas.exists(h));
    assert_eq!(cas.blob_file_count().unwrap(), 1);
}

#[test]
fn test_png_srgb_decode() {
    let png = png_4x4_red();
    let d = decode_png_srgb(&png).unwrap();
    assert_eq!(d.color_space, ColorSpace::Srgb);
    let p = d.pixels[0];
    assert!((p[0] - 1.0).abs() < 1e-3);
    assert!((p[1]).abs() < 1e-3);
    assert!((p[2]).abs() < 1e-3);
    assert!((p[3] - 1.0).abs() < 1e-3);
}

#[test]
fn test_exr_linear_decode() {
    let exr = write_exr_linear_fixture(2.5, 1.0, 0.0).unwrap();
    let d = decode_exr_linear(&exr).unwrap();
    assert_eq!(d.color_space, ColorSpace::Linear);
    assert_eq!(d.pixels[0], [2.5, 1.0, 0.0, 1.0]);
}

#[test]
fn test_wav_metadata_extract() {
    let mut wav = Vec::new();
    wav.extend_from_slice(b"RIFF");
    let data_size = 4 + 8 + 16 + 8 + 4 + 4;
    wav.extend_from_slice(&(data_size as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&48_000u32.to_le_bytes());
    wav.extend_from_slice(&4u32.to_le_bytes());
    wav.extend_from_slice(&4u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&0u32.to_le_bytes());
    wav.extend_from_slice(b"cue ");
    wav.extend_from_slice(&28u32.to_le_bytes());
    wav.extend_from_slice(&1u32.to_le_bytes());
    wav.extend_from_slice(&[0u8; 20]);
    wav.extend_from_slice(&1024u32.to_le_bytes());
    let m = parse_wav_with_cue(&wav).unwrap();
    assert_eq!(m.sample_rate, 48_000);
    assert_eq!(m.channels, 2);
    assert_eq!(m.bit_depth, 16);
    assert_eq!(m.cue_markers, vec![1024]);
}

#[test]
fn test_flac_loop_points() {
    let mut b = Vec::new();
    b.extend_from_slice(b"fLaC");
    let vendor = b"vi";
    let mut vc = Vec::new();
    vc.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    vc.extend_from_slice(vendor);
    vc.extend_from_slice(&(b"LOOPSTART=2048".len() as u32).to_le_bytes());
    vc.extend_from_slice(b"LOOPSTART=2048");
    vc.extend_from_slice(&(b"LOOPEND=4096".len() as u32).to_le_bytes());
    vc.extend_from_slice(b"LOOPEND=4096");
    let block_len = vc.len();
    let last = 0x80u8 | 4u8;
    b.push(last);
    b.extend_from_slice(&(block_len as u32).to_le_bytes()[0..3]);
    b.extend_from_slice(&vc);
    let (ls, le) = parse_flac_loop_comments(&b).unwrap();
    assert_eq!(ls, Some(2048));
    assert_eq!(le, Some(4096));
}

#[test]
fn test_validation_error_offset() {
    let mut bytes = build_native_v1(&[9u8; 8]);
    bytes[4..8].copy_from_slice(&99u32.to_le_bytes());
    let e = import_native_asset(&bytes, Path::new("asset.har")).unwrap_err();
    match e {
        ImportError::ValidationFailed { offset, path, .. } => {
            assert_eq!(offset, 4);
            assert!(path.ends_with("asset.har"));
        }
        _ => panic!("wrong err {e:?}"),
    }
    let r = validate_native_version(&bytes, Path::new("asset.har"));
    assert!(r.is_err());
}

#[test]
fn test_validation_warning_optional() {
    let bytes = build_native_v1(&[7u8; 12]);
    let (out, w) =
        import_with_optional_tags_validation(&bytes, Path::new("a.har"), false).unwrap();
    assert_eq!(out.content_hash, ContentHash::from_data(&bytes[40..]));
    assert!(w.contains(&ValidationWarning::MissingOptional { field: "tags" }));
}

#[test]
fn test_batch_progress_reporting() {
    let mut p = ProgressTracker::new();
    for i in 0..10 {
        p.on_item_completed(i);
    }
    let ev: Vec<_> = p.poll().into_iter().map(|e| e).collect();
    assert_eq!(ev.len(), 10);
    assert_eq!(p.completed(), 10);
}

#[test]
fn test_batch_cancel_rollback() {
    let dir = tempfile::tempdir().unwrap();
    let mut cas = ContentAddressableStore::new(dir.path().join("cas"));
    let mut meta = MetadataStore::new();
    let mut h = BatchImportHandle::new(
        (0..100)
            .map(|i| ImportEntry {
                path: Path::new("p").join(format!("{i}.txt")),
                bytes: vec![i as u8; 16],
            })
            .collect(),
    );
    for i in 0..50 {
        assert!(!h.is_cancelled());
        h.tick(&mut meta, |e| {
            let hash = ContentHash::from_data(&e.bytes);
            let _ = cas.store(hash, &e.bytes);
            simple_meta(i + 1, e.path.clone(), &e.bytes)
        });
    }
    h.cancel();
    h.run_all(&mut meta, |_| unreachable!());
    assert_eq!(meta.len(), 50);
    let mut referenced = HashSet::new();
    for id in meta.all_ids() {
        referenced.insert(meta.get(id).unwrap().content_hash);
    }
    for i in 50..100 {
        let b = vec![i as u8; 16];
        let hash = ContentHash::from_data(&b);
        let _ = cas.store(hash, &b);
    }
    let gc = cas.gc(&referenced).unwrap();
    assert!(gc.orphans_removed >= 50);
}

#[test]
fn test_cas_store_dedup() {
    let dir = tempfile::tempdir().unwrap();
    let mut cas = ContentAddressableStore::new(dir.path().to_path_buf());
    let data = vec![b'x'; 1024];
    let h = ContentHash::from_data(&data);
    assert_eq!(cas.store(h, &data).unwrap(), StoreResult::Written);
    assert_eq!(cas.store(h, &data).unwrap(), StoreResult::Deduplicated);
    assert_eq!(cas.blob_file_count().unwrap(), 1);
}

#[test]
fn test_cas_load_by_hash() {
    let dir = tempfile::tempdir().unwrap();
    let mut cas = ContentAddressableStore::new(dir.path().to_path_buf());
    let blob: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    let h = ContentHash::from_data(&blob);
    cas.store(h, &blob).unwrap();
    assert_eq!(cas.load(h).unwrap(), Some(blob));
}

#[test]
fn test_metadata_put_get() {
    let mut m = MetadataStore::new();
    let meta = AssetMetadata {
        asset_id: AssetId(1),
        content_hash: ContentHash::from_data(b"x"),
        source_path: "a".into(),
        import_settings: ImportSettings::Native,
        tags: vec![],
        asset_type: AssetType::Other,
        version: 3,
        name: "".into(),
        thumbnail_hash: None,
    };
    m.put(AssetId(1), meta.clone());
    assert_eq!(m.get(AssetId(1)).unwrap().version, 3);
}

#[test]
fn test_cache_hit_skips_processing() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = ImportCoordinator::new(dir.path().join("cas"));
    let body = vec![5u8; 20];
    let bytes = build_native_v1(&body);
    let p = Path::new("f.har");
    let s = ImportSettings::Native;
    let r1 = c.import_native_bytes(p, &bytes, s.clone()).unwrap();
    let n1 = c.processor_invocations;
    let r2 = c.import_native_bytes(p, &bytes, s).unwrap();
    assert_eq!(n1, 1);
    assert_eq!(c.processor_invocations, 1);
    assert!(matches!(r2, ImportResult::CacheHit { .. }));
    assert!(matches!(r1, ImportResult::Imported { .. }));
}

#[test]
fn test_cache_miss_on_settings_change() {
    let dir = tempfile::tempdir().unwrap();
    let mut c = ImportCoordinator::new(dir.path().join("cas"));
    let body = vec![6u8; 20];
    let bytes = build_native_v1(&body);
    let p = Path::new("t.tex");
    let _ = c
        .import_native_bytes(
            p,
            &bytes,
            ImportSettings::Texture {
                compression: TextureCompression::Bc7,
                generate_mips: true,
            },
        )
        .unwrap();
    let _ = c
        .import_native_bytes(
            p,
            &bytes,
            ImportSettings::Texture {
                compression: TextureCompression::Astc,
                generate_mips: true,
            },
        )
        .unwrap();
    assert_eq!(c.processor_invocations, 2);
}

#[test]
fn test_dep_invalidation_transitive() {
    let mut g = DependencyGraph::new();
    let a = AssetId(1);
    let b = AssetId(2);
    let c = AssetId(3);
    g.add_dependency(a, b);
    g.add_dependency(b, c);
    let v = g.invalidate(&[c]);
    assert_eq!(v, vec![a, b, c]);
}

#[test]
fn test_search_full_text() {
    let mut m = MetadataStore::new();
    for i in 0..100 {
        let name = if i < 5 {
            format!("iron_sword_{i}")
        } else {
            format!("other_{i}")
        };
        m.put(
            AssetId(i),
            AssetMetadata {
                asset_id: AssetId(i),
                content_hash: ContentHash::from_data(&[i as u8; 4]),
                source_path: Path::new("x").to_path_buf(),
                import_settings: ImportSettings::Native,
                tags: vec![],
                asset_type: AssetType::Other,
                version: 1,
                name,
                thumbnail_hash: None,
            },
        );
    }
    let ids = m.query(&SearchFilter::Text("sword".into()));
    assert_eq!(ids.len(), 5);
}

#[test]
fn test_search_facet_filter() {
    let mut m = MetadataStore::new();
    for i in 0u64..50 {
        let is_tex = i < 10;
        let has_hero = matches!(i, 1 | 3 | 5 | 7);
        let mut tags = vec![];
        if has_hero {
            tags.push("hero".into());
        }
        m.put(
            AssetId(i),
            AssetMetadata {
                asset_id: AssetId(i),
                content_hash: ContentHash::from_data(&i.to_le_bytes()),
                source_path: Path::new("x").to_path_buf(),
                import_settings: ImportSettings::Native,
                tags,
                asset_type: if is_tex {
                    AssetType::Texture
                } else {
                    AssetType::Mesh
                },
                version: 1,
                name: format!("n{i}"),
                thumbnail_hash: None,
            },
        );
    }
    let ids = m.query(&SearchFilter::Facet {
        asset_type: AssetType::Texture,
        tag: "hero".into(),
    });
    assert_eq!(ids.len(), 4);
}

#[test]
fn test_thumbnail_async_generated() {
    let dir = tempfile::tempdir().unwrap();
    let mut cas = ContentAddressableStore::new(dir.path().join("cas"));
    let (th, _) = crate::thumbnail::generate_thumbnail_for_mesh_import("cube");
    let bytes = th.bytes.to_vec();
    cas.store(th, &bytes).unwrap();
    let meta = AssetMetadata {
        asset_id: AssetId(1),
        content_hash: ContentHash::from_data(b"m"),
        source_path: PathBuf::from("m"),
        import_settings: ImportSettings::Native,
        tags: vec![],
        asset_type: AssetType::Mesh,
        version: 1,
        name: "cube".into(),
        thumbnail_hash: Some(th),
    };
    assert!(meta.thumbnail_hash.is_some());
    assert!(cas.exists(th));
}

#[test]
fn test_version_history_restore() {
    let mut vs = VersionStore::new();
    let v1 = vec![1u8, 2, 3];
    let v2 = vec![4u8, 5, 6];
    let h1 = ContentHash::from_data(&v1);
    let h2 = ContentHash::from_data(&v2);
    vs.record(7, h1, v1.clone());
    vs.record(7, h2, v2.clone());
    assert_eq!(vs.restore(h1).unwrap(), v1);
}

#[test]
fn test_file_watch_debounce() {
    let mut w = DebouncedWatcher::new(200);
    for t in [0u32, 20, 40, 60, 80] {
        w.ingest(t, FileEvent::Modified("f.png".into()));
    }
    let out = w.poll_changes(300);
    assert_eq!(out.len(), 1);
    assert!(matches!(&out[0], AssetChange::Modified(s) if s == "f.png"));
}

#[test]
fn test_file_watch_dedupe_rapid() {
    let mut w = DebouncedWatcher::new(200);
    w.ingest(0, FileEvent::Modified("a.png".into()));
    w.ingest(10, FileEvent::Renamed("a.png".into(), "b.png".into()));
    let out = w.poll_changes(250);
    assert_eq!(out.len(), 1);
    assert_eq!(
        out[0],
        AssetChange::Renamed {
            from: "a.png".into(),
            to: "b.png".into()
        }
    );
}

#[test]
fn test_atomic_pointer_swap() {
    let mut ht = HandleTable::new();
    let (idx, gen) = ht.allocate(100, 64);
    assert_eq!(ht.resolve(idx, gen), Some(100));
    let mut sch = SwapScheduler::new();
    sch.schedule(PendingSwap {
        index: idx,
        new_ptr: 200,
        new_size: 128,
    });
    sch.apply_pending_swaps(&mut ht);
    assert_eq!(ht.resolve(idx, gen), Some(200));
    assert!(sch.retired().iter().any(|(p, _)| *p == 100));
}

#[test]
fn test_swap_dependents_updated() {
    let mut ht = HandleTable::new();
    let (tex, g) = ht.allocate(10, 4);
    let mats = [tex, tex, tex];
    ht.swap_ptr(tex, 99, 4).unwrap();
    for m in mats {
        assert_eq!(ht.resolve(m, g), Some(99));
    }
}

#[test]
fn test_shader_pso_swap() {
    let mut s = ShaderReloader::new(2);
    s.recompile_ok(0);
    assert_eq!(s.pso_generation[0], 2);
}

#[test]
fn test_shader_error_overlay() {
    let mut s = ShaderReloader::new(1);
    s.recompile_ok(0);
    let before = s.pso_generation[0];
    s.recompile_err(0, "syntax");
    assert!(s.last_overlay.is_some());
    assert_eq!(s.pso_generation[0], before);
}

#[test]
fn test_logic_graph_state_preserve() {
    let old = LogicGraphSpec {
        vars: vec!["x".into()],
    };
    let new = LogicGraphSpec {
        vars: vec!["x".into()],
    };
    let mut inst = LogicGraphInstance {
        vars: HashMap::from([("x".into(), 42)]),
    };
    let defs = HashMap::new();
    let ev = hot_reload_logic_graph(&mut inst, &old, &new, &defs);
    assert_eq!(ev, LogicReloadEvent::StatePreserved);
    assert_eq!(inst.read_var("x"), Some(42));
}

#[test]
fn test_logic_graph_layout_change() {
    let old = LogicGraphSpec {
        vars: vec!["x".into()],
    };
    let new = LogicGraphSpec {
        vars: vec!["x".into(), "y".into()],
    };
    let mut inst = LogicGraphInstance {
        vars: HashMap::from([("x".into(), 42)]),
    };
    let defs = HashMap::from([("x".into(), 0), ("y".into(), 0)]);
    let ev = hot_reload_logic_graph(&mut inst, &old, &new, &defs);
    assert_eq!(ev, LogicReloadEvent::LogicGraphRestart);
    assert_eq!(inst.read_var("x"), Some(0));
}

#[test]
fn test_ui_subtree_rebuild() {
    let mut doc = UiDocument::default_three();
    let wa = doc.panels[0].widget_id;
    let wb0 = doc.panels[1].widget_id;
    let wc = doc.panels[2].widget_id;
    reload_ui_panel_style(&mut doc, 'B');
    assert_eq!(doc.panels[0].widget_id, wa);
    assert_ne!(doc.panels[1].widget_id, wb0);
    assert_eq!(doc.panels[2].widget_id, wc);
}

#[test]
fn test_partial_subasset_reimport() {
    let mut c = CompositeAsset {
        clips: (0..10)
            .map(|i| crate::subasset::Clip {
                index: i,
                data: vec![i as u8; 4],
            })
            .collect(),
    };
    let n = partial_subasset_reimport(&mut c, SubassetEdit { clip_index: 5 });
    assert_eq!(n, 1);
}

#[test]
fn test_editor_runtime_sync() {
    let mut sync = EditorSync::new();
    let mut rt = MaterialRuntime::default();
    sync.send_editor(EditorMessage::SetMaterialParam {
        material: 0,
        param: "albedo".into(),
        value: [1.0, 0.0, 0.0, 1.0],
    });
    sync.poll(&mut rt);
    assert_eq!(rt.albedo, [1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_asset_header_roundtrip() {
    let mut w = AssetWriter::new(42);
    w.add_section("main", vec![1, 2, 3, 4]);
    let bytes = w.build().unwrap();
    let r = AssetReader::from_bytes(&bytes).unwrap();
    assert_eq!(r.header.magic, ASSET_MAGIC);
    assert_eq!(r.header.format_version, FORMAT_VERSION);
    assert_eq!(r.header.asset_type_id, 42);
    r.verify_integrity().unwrap();
}

#[test]
fn test_asset_section_o1_lookup() {
    let mut w = AssetWriter::new(1);
    for i in 0..100 {
        w.add_section(&format!("section_{i}"), vec![i as u8; 8]);
    }
    let bytes = w.build().unwrap();
    let r = AssetReader::from_bytes(&bytes).unwrap();
    let s = r.section("section_50").unwrap();
    assert_eq!(s[0], 50u8);
}

#[test]
fn test_bundle_partial_decompress() {
    let mut bw = BundleWriter::new();
    for i in 0..10 {
        bw.push_entry(vec![i as u8; 32]);
    }
    let blob = bw.finish();
    let br = BundleReader::new(&blob).unwrap();
    let mut stats = BundleDecodeStats::default();
    let out = br.extract(5, &mut stats).unwrap();
    assert_eq!(out[0], 5);
    assert_eq!(stats.chunks_decompressed, 1);
}

#[test]
fn test_structural_diff_mesh() {
    let d = diff_mesh_assets(
        &MeshAssetSummary { vertices: 1000 },
        &MeshAssetSummary { vertices: 1100 },
    );
    assert_eq!(d, DiffResult::Mesh { vertex_delta: 100 });
}

#[test]
fn test_three_way_merge_disjoint() {
    let a = b"nodes:";
    let o = b"nodes:N1";
    let t = b"nodes:N2";
    let m = three_way_merge_disjoint_graphs(a, o, t);
    let MergeResult::Success { merged } = m;
    let s = String::from_utf8(merged).unwrap();
    assert!(s.contains("N1") && s.contains("N2"));
}

#[test]
fn test_auto_resolve_lww() {
    let r = auto_resolve_lww(0.7, 1, 0.9, 2);
    assert!((r.roughness - 0.9).abs() < 1e-6);
    assert_eq!(r.resolutions.len(), 1);
}

#[test]
fn test_auto_resolve_union() {
    let r = auto_resolve_union_tags(
        &[String::from("a")],
        &[String::from("a"), String::from("b")],
        &[String::from("a"), String::from("c")],
    );
    assert_eq!(r.tags, vec!["a", "b", "c"]);
}

#[test]
fn test_dep_graph_pipeline_cycle() {
    let a = AssetId(1);
    let b = AssetId(2);
    let e = run_pipeline(&[(a, b), (b, a)]).unwrap_err();
    match e {
        PipelineError::DependencyCycle { path } => {
            assert!(path.contains(&a) && path.contains(&b));
        }
    }
}

#[test]
fn test_material_mapper_gltf_pbr() {
    let im = ImportedMaterial {
        has_base_color: true,
        has_metallic_roughness: true,
        has_emissive: false,
    };
    let fallback = [0.1, 0.2, 0.3, 1.0];
    let (d, w) = translate_gltf_pbr_material(&im, fallback);
    assert!(d.base_color && d.mr);
    assert_eq!(d.emissive, fallback);
    assert!(w.is_empty());
}

#[test]
fn test_headless_batch_identical() {
    let (a, b) = run_headless_batch_twice(50);
    assert_eq!(a, b);
}

#[test]
fn test_data_table_editor_cell_edit() {
    let mut rows = BTreeMap::new();
    let mut r42 = BTreeMap::new();
    r42.insert("damage".into(), "10".into());
    rows.insert(42, r42);
    let t = DataTable { rows };
    let saved = edit_data_table_cell(&t, 42, "damage", "55");
    assert!(saved.contains("55"));
    let t2 = DataTable::parse_hat(&saved);
    let n = structural_diff_table_cells(&t, &t2);
    assert_eq!(n, 1);
}

#[test]
fn test_visual_inspector_fields() {
    let rows = visual_inspector_fields();
    assert_eq!(rows[0].widget, WidgetKind::TextField);
    assert_eq!(rows[1].widget, WidgetKind::IntegerSpinner);
    assert_eq!(rows[2].widget, WidgetKind::ColorPicker);
}
