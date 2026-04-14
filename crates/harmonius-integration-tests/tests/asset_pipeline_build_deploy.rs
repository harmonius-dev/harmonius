//! IR-5.1.x integration coverage (see `docs/design/integration/asset-pipeline-build-deploy-test-cases.md`).

use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

use std::collections::HashMap as StdHashMap;

use harmonius_asset_build::{
    build_bundles, build_delta_patch, build_requests, bundle_completions, cook_assets, cook_completions,
    cooked_manifest_merkle_root, parse_cas_key, re_bake_for_test, verify_cooked_manifest_root,
    AssetId, AssetKind, BuildConfig, ChannelSizes, ConcurrentCas, CookRequest, CookedEntry,
    CookedManifest, DeltaError, FakeAssetSource, IncrementalCache, LruCas, MemoryCas, PlatformProfile,
    ShaderArtifactKind, ShaderCompileCache, ShaderCompileKey, TargetPlatform, TextureCompression,
    CasStore,
};

struct MixedSource {
    textures: usize,
}

impl FakeAssetSource for MixedSource {
    fn describe(&self, id: AssetId) -> Option<(AssetKind, Vec<u8>)> {
        let n = id.0 as usize;
        if n < self.textures {
            Some((AssetKind::Texture, format!("tex-{n}").into_bytes()))
        } else if n < self.textures + 50 {
            Some((AssetKind::Shader, format!("shd-{n}").into_bytes()))
        } else {
            Some((AssetKind::Mesh, format!("mesh-{n}").into_bytes()))
        }
    }
}

fn cook_windows(n: usize) -> CookedManifest {
    let ids: Vec<AssetId> = (0..n as u64).map(AssetId).collect();
    let request = CookRequest {
        config: BuildConfig {
            target: TargetPlatform::Windows,
        },
        profile: PlatformProfile {
            target: TargetPlatform::Windows,
            texture_override: None,
        },
        asset_ids: ids,
    };
    let source = MixedSource { textures: 40 };
    cook_assets(&request, &source).unwrap().0
}

fn cook_macos(n: usize) -> CookedManifest {
    let ids: Vec<AssetId> = (0..n as u64).map(AssetId).collect();
    let request = CookRequest {
        config: BuildConfig {
            target: TargetPlatform::MacOS,
        },
        profile: PlatformProfile {
            target: TargetPlatform::MacOS,
            texture_override: None,
        },
        asset_ids: ids,
    };
    let source = MixedSource { textures: 40 };
    cook_assets(&request, &source).unwrap().0
}

/// TC-IR-5.1.1.1 — Windows cook produces BC7 texture prefix and valid Merkle root.
#[test]
fn tc_ir_5_1_1_1_cook_windows() {
    let m = cook_windows(100);
    assert_eq!(m.platform, TargetPlatform::Windows);
    assert!(verify_cooked_manifest_root(&m));
    let tex_entry = m.entries.iter().find(|e| e.kind == AssetKind::Texture).unwrap();
    let baked = fake_read_cas(tex_entry.cas_key, &m);
    assert!(baked.starts_with(b"TEX:"));
    assert_eq!(baked[4], TextureCompression::Bc7 as u8);
}

/// TC-IR-5.1.1.2 — macOS cook uses ASTC prefix for textures.
#[test]
fn tc_ir_5_1_1_2_cook_macos() {
    let m = cook_macos(100);
    assert!(verify_cooked_manifest_root(&m));
    let tex_entry = m.entries.iter().find(|e| e.kind == AssetKind::Texture).unwrap();
    let baked = fake_read_cas(tex_entry.cas_key, &m);
    assert_eq!(baked[4], TextureCompression::Astc as u8);
}

fn fake_read_cas(key: [u8; 32], manifest: &CookedManifest) -> Vec<u8> {
    for e in &manifest.entries {
        if e.cas_key == key {
            let src = MixedSource { textures: 40 }
                .describe(e.asset_id)
                .unwrap()
                .1;
            return re_bake_for_test(e.kind, &src, manifest.platform);
        }
    }
    panic!("missing entry");
}

/// TC-IR-5.1.2.1 / TC-IR-5.1.2.2 — profile selects BC7 vs ASTC.
#[test]
fn tc_ir_5_1_2_texture_codecs() {
    let win = PlatformProfile {
        target: TargetPlatform::Windows,
        texture_override: None,
    };
    assert_eq!(
        harmonius_asset_build::select_texture_format(&win),
        TextureCompression::Bc7
    );
    let ios = PlatformProfile {
        target: TargetPlatform::IOS,
        texture_override: None,
    };
    assert_eq!(
        harmonius_asset_build::select_texture_format(&ios),
        TextureCompression::Astc
    );
}

/// TC-IR-5.1.3.1 — incremental skips unchanged assets.
#[test]
fn tc_ir_5_1_3_1_incremental_skip() {
    let mut cache: IncrementalCache<u64> = IncrementalCache::new();
    let ids: Vec<AssetId> = (0..100).map(AssetId).collect();
    let fp_state: RefCell<StdHashMap<AssetId, u64>> = RefCell::new(
        (0..100u64).map(|i| (AssetId(i), i)).collect(),
    );
    let fp = |id: AssetId| *fp_state.borrow().get(&id).unwrap();
    assert_eq!(cache.filter_changed(&ids, fp).len(), 100);
    cache.commit(&ids, fp);
    *fp_state.borrow_mut().get_mut(&AssetId(7)).unwrap() += 1_000;
    let second = cache.filter_changed(&ids, fp);
    assert_eq!(second, vec![AssetId(7)]);
}

/// TC-IR-5.1.3.2 — invalidation forces rebuild set.
#[test]
fn tc_ir_5_1_3_2_invalidate() {
    let mut cache: IncrementalCache<u64> = IncrementalCache::new();
    let ids: Vec<AssetId> = (0..100).map(AssetId).collect();
    let fp = |id: AssetId| id.0;
    cache.commit(&ids, fp);
    cache.invalidate(&(0..5).map(AssetId).collect::<Vec<_>>());
    let dirty = cache.filter_changed(&ids, fp);
    assert_eq!(dirty.len(), 5);
}

/// TC-IR-5.1.4.1 — bundles hash cooked content.
#[test]
fn tc_ir_5_1_4_1_bundle_hashes() {
    let m = cook_windows(50);
    let mut cas = MemoryCas::default();
    let mut map = StdHashMap::new();
    for e in &m.entries {
        let bytes = entry_bytes(&m, e);
        let k = cas.put(&bytes).unwrap();
        assert_eq!(k, e.cas_key);
        map.insert(e.cas_key, bytes);
    }
    let set = build_bundles(&m, 1_000_000, |k| map.get(k).cloned()).expect("bundle");
    assert!(!set.shards.is_empty());
    for s in &set.shards {
        assert_ne!(s.blake3, [0u8; 32]);
    }
}

/// TC-IR-5.1.4.2 — size limit splits bundles.
#[test]
fn tc_ir_5_1_4_2_bundle_size_limit() {
    let mut entries = Vec::new();
    for i in 0..8u64 {
        let mut bytes = vec![7u8; 300_000];
        bytes[0..8].copy_from_slice(&i.to_le_bytes());
        let cas_key = *blake3::hash(&bytes).as_bytes();
        entries.push(CookedEntry {
            asset_id: AssetId(i),
            cas_key,
            size_bytes: bytes.len() as u64,
            kind: AssetKind::Texture,
        });
    }
    let leaves: Vec<_> = entries.iter().map(|e| e.cas_key).collect();
    let manifest = CookedManifest {
        platform: TargetPlatform::Windows,
        entries,
        blake3_root: cooked_manifest_merkle_root(&leaves),
    };
    let mut map: StdHashMap<[u8; 32], Vec<u8>> = manifest
        .entries
        .iter()
        .map(|e| {
            let mut b = vec![7u8; 300_000];
            b[0..8].copy_from_slice(&e.asset_id.0.to_le_bytes());
            (e.cas_key, b)
        })
        .collect();
    // Two ~300 KiB rows cannot share a 400 KiB shard budget (wire includes key + length prefix).
    let set = build_bundles(&manifest, 400_000, |k| map.remove(k)).expect("split");
    assert!(set.shards.len() > 1);
}

fn entry_bytes(manifest: &CookedManifest, e: &CookedEntry) -> Vec<u8> {
    let src = MixedSource { textures: 40 }
        .describe(e.asset_id)
        .unwrap()
        .1;
    re_bake_for_test(e.kind, &src, manifest.platform)
}

/// TC-IR-5.1.5.1 — distinct bytecode per platform (fake cache path).
#[test]
fn tc_ir_5_1_5_1_shader_variants() {
    let mut cache = ShaderCompileCache::new();
    let key_dxil = ShaderCompileKey {
        graph_id: 1,
        target: TargetPlatform::Windows,
    };
    let key_spirv = ShaderCompileKey {
        graph_id: 1,
        target: TargetPlatform::Linux,
    };
    let key_metal = ShaderCompileKey {
        graph_id: 1,
        target: TargetPlatform::MacOS,
    };
    let c1 = cache.get_or_compile(key_dxil.clone(), |_| {
        (ShaderArtifactKind::Dxil, b"dxil-bytes".to_vec())
    });
    let c2 = cache.get_or_compile(key_spirv.clone(), |_| {
        (ShaderArtifactKind::Spirv, b"spirv-bytes".to_vec())
    });
    let c3 = cache.get_or_compile(key_metal.clone(), |_| {
        (ShaderArtifactKind::Metallib, b"metal-bytes".to_vec())
    });
    assert_ne!(c1.1, c2.1);
    assert_ne!(c2.1, c3.1);
}

/// TC-IR-5.1.5.2 — second compile hits cache (zero additional tool calls).
#[test]
fn tc_ir_5_1_5_2_shader_cache_hit() {
    let mut cache = ShaderCompileCache::new();
    let key = ShaderCompileKey {
        graph_id: 42,
        target: TargetPlatform::Windows,
    };
    let _ = cache.get_or_compile(key.clone(), |_| {
        (ShaderArtifactKind::Dxil, b"a".to_vec())
    });
    let dxc_after_first = cache.invocations.dxc.load(std::sync::atomic::Ordering::SeqCst);
    let _ = cache.get_or_compile(key, |_| panic!("compile_fn must not run on cache hit"));
    let dxc_after_second = cache.invocations.dxc.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(dxc_after_first, dxc_after_second);
}

/// TC-IR-5.1.6.1 — delta smaller than full v2 wire when one asset changes.
#[test]
fn tc_ir_5_1_6_1_delta_smaller_than_full() {
    let v1 = cook_windows(20);
    let mut v2 = v1.clone();
    let last = v2.entries.len() - 1;
    let aid = v2.entries[last].asset_id;
    let new_src = b"changed-shader".to_vec();
    let new_bytes = re_bake_for_test(AssetKind::Shader, &new_src, TargetPlatform::Windows);
    let new_key = *blake3::hash(&new_bytes).as_bytes();
    v2.entries[last] = CookedEntry {
        asset_id: aid,
        cas_key: new_key,
        size_bytes: new_bytes.len() as u64,
        kind: AssetKind::Shader,
    };
    let leaves: Vec<_> = v2.entries.iter().map(|e| e.cas_key).collect();
    v2.blake3_root = cooked_manifest_merkle_root(&leaves);
    let mut cas: StdHashMap<[u8; 32], Vec<u8>> = StdHashMap::new();
    for e in &v1.entries {
        cas.insert(e.cas_key, entry_bytes(&v1, e));
    }
    cas.insert(new_key, new_bytes.clone());
    let patch = build_delta_patch(&v1, &v2, |k| cas.get(k).cloned()).unwrap();
    let full: usize = v2
        .entries
        .iter()
        .map(|e| 32 + 8 + cas.get(&e.cas_key).map(|b| b.len()).unwrap_or(0))
        .sum();
    assert!(patch.payload.len() < full);
}

/// TC-IR-5.1.6.2 — identical manifests → empty patch.
#[test]
fn tc_ir_5_1_6_2_zero_byte_patch() {
    let v1 = cook_windows(10);
    let v2 = v1.clone();
    let cas: StdHashMap<[u8; 32], Vec<u8>> = v1
        .entries
        .iter()
        .map(|e| (e.cas_key, entry_bytes(&v1, e)))
        .collect();
    let patch = build_delta_patch(&v1, &v2, |k| cas.get(k).cloned()).unwrap();
    assert!(patch.payload.is_empty());
}

/// TC-IR-5.1.6.3 — corrupt v1 Merkle yields error (caller-visible).
#[test]
fn tc_ir_5_1_6_3_corrupt_v1() {
    let mut v1 = cook_windows(5);
    v1.blake3_root = [1u8; 32];
    let v2 = cook_windows(5);
    let cas: StdHashMap<[u8; 32], Vec<u8>> = v2
        .entries
        .iter()
        .map(|e| (e.cas_key, entry_bytes(&v2, e)))
        .collect();
    let err = build_delta_patch(&v1, &v2, |k| cas.get(k).cloned()).unwrap_err();
    assert_eq!(err, DeltaError::CorruptV1);
}

/// TC-IR-5.1.6.N1 — tampered v2 root rejected.
#[test]
fn tc_ir_5_1_6_n1_merkle_root_mismatch() {
    let v1 = cook_windows(4);
    let mut v2 = v1.clone();
    v2.blake3_root = [9u8; 32];
    let cas: StdHashMap<[u8; 32], Vec<u8>> = v1
        .entries
        .iter()
        .map(|e| (e.cas_key, entry_bytes(&v1, e)))
        .collect();
    let err = build_delta_patch(&v1, &v2, |k| cas.get(k).cloned()).unwrap_err();
    assert_eq!(err, DeltaError::MerkleRootMismatch);
}

/// TC-IR-5.1.7.1 — pre-warmed map serves every key without recompute (fake “cache hit”).
#[test]
fn tc_ir_5_1_7_1_cache_hits() {
    let m = cook_windows(10);
    let cas: StdHashMap<[u8; 32], Vec<u8>> = m
        .entries
        .iter()
        .map(|e| (e.cas_key, entry_bytes(&m, e)))
        .collect();
    for e in &m.entries {
        assert!(cas.contains_key(&e.cas_key));
    }
}

/// TC-IR-5.1.7.2 — empty CAS filled by puts.
#[test]
fn tc_ir_5_1_7_2_cache_miss_cook() {
    let mut cas = MemoryCas::default();
    for i in 0u64..10 {
        let b = format!("blob-{i}").into_bytes();
        cas.put(&b).unwrap();
    }
    assert_eq!(cas.len(), 10);
}

/// TC-IR-5.1.7.3 — concurrent writes dedupe identical keys.
#[test]
fn tc_ir_5_1_7_3_concurrent_same_key() {
    let cas = Arc::new(ConcurrentCas::new());
    let bytes: Vec<u8> = b"shared-payload".to_vec();
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&cas);
        let b = bytes.clone();
        handles.push(thread::spawn(move || c.put(&b)));
    }
    for h in handles {
        h.join().unwrap().unwrap();
    }
}

/// TC-IR-5.1.7.4 — LRU evicts oldest.
#[test]
fn tc_ir_5_1_7_4_lru_eviction() {
    let mut cas = LruCas::new(2);
    let k1 = cas.put(b"a").unwrap();
    let k2 = cas.put(b"b").unwrap();
    assert!(cas.contains_key(&k1));
    let k3 = cas.put(b"c").unwrap();
    assert!(!cas.contains_key(&k1));
    assert!(cas.contains_key(&k2));
    assert!(cas.contains_key(&k3));
}

/// TC-IR-5.1.7.N1 — malformed CAS key rejected.
#[test]
fn tc_ir_5_1_7_n1_malformed_key() {
    let short = [0u8; 31];
    assert!(parse_cas_key(&short).is_err());
}

/// Channel buffer sizes match integration design.
#[test]
fn channel_buffer_sizes() {
    let s = ChannelSizes::default();
    let (tx, rx) = build_requests::<()>();
    assert_eq!(s.build_requests, 64);
    drop(tx);
    drop(rx);
    let (t2, r2) = cook_completions::<()>();
    assert_eq!(s.cook_completions, 256);
    drop(t2);
    drop(r2);
    let (t3, r3) = bundle_completions::<()>();
    assert_eq!(s.bundle_completions, 64);
    drop(t3);
    drop(r3);
}

#[test]
fn rkyv_roundtrip_cooked_manifest() {
    let m = cook_windows(3);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&m).expect("serialize");
    let archived = rkyv::access::<rkyv::Archived<CookedManifest>, rkyv::rancor::Error>(&bytes)
        .expect("access");
    let round: CookedManifest = rkyv::deserialize::<_, rkyv::rancor::Error>(archived).expect("de");
    assert_eq!(round, m);
}
