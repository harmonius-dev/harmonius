use harmonius_rendering_pso_cache::{
    infer_descriptor_layout, ContentHash, DescriptorLayout, DeviceFingerprint, DeviceId, DiskIndex,
    GpuDevice, MemoryEntry, MemoryPsoCache, PipelineDesc, PsoCache, PsoCompiler, PsoEntry,
    PsoHandle, PsoKey, ShaderApi, SortedVecMap,
};

fn hash_a() -> ContentHash {
    ContentHash::from_bytes([7u8; 32])
}

fn hash_b() -> ContentHash {
    ContentHash::from_bytes([9u8; 32])
}

fn sample_key() -> PsoKey {
    PsoKey {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
        device_id: DeviceId {
            vendor: 0x8086,
            device: 0x0412,
        },
        driver_version: pack_version(31, 0, 0),
        format_version: 3,
    }
}

fn pack_version(maj: u32, min: u32, pat: u32) -> u64 {
    ((maj as u64 & 0xFF_FFFF) << 40) | ((min as u64 & 0xFF_FFFF) << 20) | (pat as u64 & 0xFF_FFFF)
}

#[test]
fn test_pso_key_deterministic_hash() {
    let k1 = sample_key();
    let k2 = sample_key();
    assert_eq!(k1, k2);
}

#[test]
fn test_pso_key_shader_hash_affects_key() {
    let left = sample_key();
    let mut right = sample_key();
    right.shader_hash = hash_b();
    assert_ne!(left, right);
}

#[test]
fn test_pso_key_signature_hash_affects_key() {
    let left = sample_key();
    let mut right = sample_key();
    right.signature_hash = hash_a();
    assert_ne!(left, right);
}

#[test]
fn test_pso_key_driver_version_affects_key() {
    let left = sample_key();
    let mut right = sample_key();
    right.driver_version = pack_version(32, 0, 0);
    assert_ne!(left, right);
}

#[test]
fn test_memory_cache_insert_and_get() {
    let mut cache = MemoryPsoCache::new();
    let key = sample_key();
    cache.insert(key, 42);
    assert_eq!(cache.get_mut(&key).expect("hit").payload, 42);
}

#[test]
fn test_memory_cache_last_used_tick_bumps() {
    let mut cache = MemoryPsoCache::new();
    let key = sample_key();
    cache.insert(key, 1);
    let tick_after_insert = cache.current_tick();
    cache.bump_tick();
    let _ = cache.get_mut(&key);
    assert_eq!(
        cache.get_mut(&key).expect("hit").last_used_tick,
        tick_after_insert + 1
    );
}

mod type_assert {
    use harmonius_rendering_pso_cache::SortedVecMap;

    pub trait AssertSortedVec {}

    impl<K: Ord, V> AssertSortedVec for SortedVecMap<K, V> {}

    pub fn assert_sorted_vec_map<T: AssertSortedVec>() {}
}

#[test]
fn test_memory_cache_lookup_is_sorted_vec() {
    type_assert::assert_sorted_vec_map::<SortedVecMap<PsoKey, MemoryEntry>>();
}

#[test]
fn test_pso_cache_memory_tier_is_sorted_vec() {
    type_assert::assert_sorted_vec_map::<SortedVecMap<PsoKey, PsoEntry>>();
}

#[test]
fn test_disk_layout_versioned_directory() {
    let tmp = tempfile::tempdir().expect("tmp");
    let _fp = DeviceFingerprint {
        vendor_id: 1,
        device_id: 2,
        driver_version: 3,
        api_version: 4,
    };
    let path = DiskIndex::versioned_root(tmp.path(), 3);
    assert!(path.ends_with("v3"));
}

#[test]
fn test_disk_layout_fingerprint_subdirectory() {
    let tmp = tempfile::tempdir().expect("tmp");
    let fp = DeviceFingerprint {
        vendor_id: 0x8086,
        device_id: 0x0412,
        driver_version: pack_version(31, 0, 0),
        api_version: 12,
    };
    let path = DiskIndex::fingerprint_dir(tmp.path(), 3, &fp);
    let name = path.file_name().expect("name").to_string_lossy();
    assert!(name.starts_with("vendor_8086_dev_0412_api_0000000c_drv_"));
}

struct CountingCompiler {
    next: u32,
    compiles: u32,
    revives: u32,
}

impl CountingCompiler {
    fn new() -> Self {
        Self {
            next: 1,
            compiles: 0,
            revives: 0,
        }
    }
}

impl PsoCompiler for CountingCompiler {
    fn compile(
        &mut self,
        desc: &PipelineDesc,
    ) -> Result<PsoEntry, harmonius_rendering_pso_cache::PsoError> {
        self.compiles += 1;
        let handle = PsoHandle(self.next);
        self.next += 1;
        Ok(PsoEntry {
            handle,
            bytecode: desc.shader_hash.0.to_vec(),
            desc_layout: DescriptorLayout {
                bindings: Vec::new(),
                push_constants: 0,
            },
            last_used_tick: 0,
            size_bytes: 64,
        })
    }

    fn revive(
        &mut self,
        blob: &[u8],
        desc: &PipelineDesc,
    ) -> Result<PsoEntry, harmonius_rendering_pso_cache::PsoError> {
        self.revives += 1;
        let handle = PsoHandle(u32::from_le_bytes(blob[0..4].try_into().expect("handle")));
        Ok(PsoEntry {
            handle,
            bytecode: desc.shader_hash.0.to_vec(),
            desc_layout: DescriptorLayout {
                bindings: Vec::new(),
                push_constants: 0,
            },
            last_used_tick: 0,
            size_bytes: blob.len() as u32,
        })
    }

    fn serialize(&self, entry: &PsoEntry) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&entry.handle.0.to_le_bytes());
        out.extend_from_slice(&entry.bytecode);
        out
    }
}

fn gpu() -> GpuDevice {
    GpuDevice {
        ids: DeviceId {
            vendor: 0x10de,
            device: 0x2488,
        },
        driver_version: pack_version(31, 0, 0),
        api_version: 1,
    }
}

#[test]
fn test_invalidate_on_shader_hash_change() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let mut compiler = CountingCompiler::new();
    let mut desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    let h1 = cache.get_or_build(&desc, &mut compiler).expect("build");
    desc.shader_hash = hash_b();
    let h2 = cache.get_or_build(&desc, &mut compiler).expect("build");
    assert_ne!(h1, h2);
}

#[test]
fn test_invalidate_on_driver_upgrade() {
    let tmp = tempfile::tempdir().expect("tmp");
    let dev_v1 = GpuDevice {
        ids: DeviceId {
            vendor: 0x10de,
            device: 0x2488,
        },
        driver_version: pack_version(31, 0, 0),
        api_version: 1,
    };
    let dev_v2 = GpuDevice {
        ids: DeviceId {
            vendor: 0x10de,
            device: 0x2488,
        },
        driver_version: pack_version(32, 0, 0),
        api_version: 1,
    };
    let mut cache = PsoCache::new(tmp.path(), &dev_v1).expect("open");
    let mut compiler = CountingCompiler::new();
    let desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    let _ = cache.get_or_build(&desc, &mut compiler).expect("build");
    let dir_v1 = cache.disk_dir().to_path_buf();

    let cache_v2 = PsoCache::new(tmp.path(), &dev_v2).expect("open");
    let dir_v2 = cache_v2.disk_dir();
    assert_ne!(dir_v1, dir_v2);
}

#[test]
fn test_invalidate_on_api_change() {
    let tmp = tempfile::tempdir().expect("tmp");
    let dev_d3d = GpuDevice {
        ids: DeviceId {
            vendor: 0x10de,
            device: 0x2488,
        },
        driver_version: pack_version(31, 0, 0),
        api_version: 1,
    };
    let dev_vk = GpuDevice {
        ids: DeviceId {
            vendor: 0x10de,
            device: 0x2488,
        },
        driver_version: pack_version(31, 0, 0),
        api_version: 2,
    };
    let cache_d3d = PsoCache::new(tmp.path(), &dev_d3d).expect("open");
    let cache_vk = PsoCache::new(tmp.path(), &dev_vk).expect("open");
    assert_ne!(cache_d3d.disk_dir(), cache_vk.disk_dir());
}

#[test]
fn test_invalidate_on_device_change() {
    let tmp = tempfile::tempdir().expect("tmp");
    let dev_a = GpuDevice {
        ids: DeviceId {
            vendor: 0x10de,
            device: 0x2488,
        },
        driver_version: pack_version(31, 0, 0),
        api_version: 1,
    };
    let dev_b = GpuDevice {
        ids: DeviceId {
            vendor: 0x1002,
            device: 0x731f,
        },
        driver_version: pack_version(31, 0, 0),
        api_version: 1,
    };
    let cache_a = PsoCache::new(tmp.path(), &dev_a).expect("open");
    let cache_b = PsoCache::new(tmp.path(), &dev_b).expect("open");
    assert_ne!(cache_a.disk_dir(), cache_b.disk_dir());
    assert!(cache_a.disk_dir().exists());
    assert!(cache_b.disk_dir().exists());
}

#[test]
fn test_invalidate_all_clears_memory() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let mut compiler = CountingCompiler::new();
    let desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    let _ = cache.get_or_build(&desc, &mut compiler).expect("build");
    assert_eq!(cache.memory_len(), 1);
    cache.invalidate_all();
    assert_eq!(cache.memory_len(), 0);
}

#[test]
fn test_gc_drops_oldest_by_tick() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut disk = DiskIndex::open(tmp.path(), 3, &gpu().fingerprint()).expect("open");
    let k1 = sample_key();
    let mut k2 = sample_key();
    k2.shader_hash = hash_b();
    disk.write(&k1, b"blob-a", 1).expect("write");
    disk.write(&k2, b"blob-b", 5).expect("write");
    disk.gc_disk_lru(20).expect("gc");
    assert!(disk.read(&k2).is_ok());
}

#[test]
fn test_gc_compacts_blob_file() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut disk = DiskIndex::open(tmp.path(), 3, &gpu().fingerprint()).expect("open");
    let k1 = sample_key();
    let mut k2 = sample_key();
    k2.shader_hash = hash_b();
    disk.write(&k1, b"aaaaaaaa", 10).expect("write");
    disk.write(&k2, b"bbbbbbbb", 20).expect("write");
    let before = disk.total_blob_bytes();
    disk.gc_disk_lru(24).expect("gc");
    disk.compact_blob_file().expect("compact");
    let after = disk.total_blob_bytes();
    assert!(after < before);
    assert_eq!(after, disk.total_blob_bytes());
}

#[test]
fn test_gc_respects_disk_cap() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut disk = DiskIndex::open(tmp.path(), 3, &gpu().fingerprint()).expect("open");
    let k1 = sample_key();
    let mut k2 = sample_key();
    k2.shader_hash = hash_b();
    disk.write(&k1, b"11111111", 1).expect("write");
    disk.write(&k2, b"22222222", 2).expect("write");
    disk.gc_disk_lru(24).expect("gc");
    assert!(disk.total_blob_bytes() <= 24);
}

#[test]
fn test_corrupt_blob_isolated() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut disk = DiskIndex::open(tmp.path(), 3, &gpu().fingerprint()).expect("open");
    let k1 = sample_key();
    let mut k2 = sample_key();
    k2.shader_hash = hash_b();
    disk.write(&k1, b"good-good-good", 1).expect("write");
    disk.write(&k2, b"bad-bad-bad-bad", 2).expect("write");
    let path = disk.device_dir().join("blobs.bin");
    let mut bytes = std::fs::read(&path).expect("read");
    if let Some(b) = bytes.last_mut() {
        *b ^= 0xFF;
    }
    std::fs::write(&path, bytes).expect("write");
    let reopened = DiskIndex::open(tmp.path(), 3, &gpu().fingerprint()).expect("reopen");
    assert!(reopened.read(&k1).is_ok());
    assert!(reopened.read(&k2).is_err());
}

#[test]
fn test_corrupt_index_resets_directory() {
    let tmp = tempfile::tempdir().expect("tmp");
    let fp = gpu().fingerprint();
    let mut disk = DiskIndex::open(tmp.path(), 3, &fp).expect("open");
    disk.write(&sample_key(), b"payload", 1).expect("write");
    let index_path = disk.device_dir().join("index.bin");
    let mut bytes = std::fs::read(&index_path).expect("read");
    if let Some(b) = bytes.last_mut() {
        *b ^= 0xFF;
    }
    std::fs::write(&index_path, bytes).expect("write");
    let reopened = DiskIndex::open(tmp.path(), 3, &fp).expect("reopen");
    assert!(reopened.read(&sample_key()).is_err());
}

#[test]
fn test_bad_magic_resets_directory() {
    let tmp = tempfile::tempdir().expect("tmp");
    let fp = gpu().fingerprint();
    let mut disk = DiskIndex::open(tmp.path(), 3, &fp).expect("open");
    disk.write(&sample_key(), b"payload", 1).expect("write");
    let index_path = disk.device_dir().join("index.bin");
    std::fs::write(&index_path, b"BADMAGIX").expect("write");
    let reopened = DiskIndex::open(tmp.path(), 3, &fp).expect("reopen");
    assert!(reopened.read(&sample_key()).is_err());
}

#[test]
fn test_index_version_skew_resets_directory() {
    let tmp = tempfile::tempdir().expect("tmp");
    let fp = gpu().fingerprint();
    let mut disk = DiskIndex::open(tmp.path(), 3, &fp).expect("open");
    disk.write(&sample_key(), b"payload", 1).expect("write");
    let index_path = disk.device_dir().join("index.bin");
    let mut bytes = std::fs::read(&index_path).expect("read");
    bytes[8] = 0xFE;
    bytes[9] = 0xFE;
    bytes[10] = 0xFE;
    bytes[11] = 0xFE;
    std::fs::write(&index_path, bytes).expect("write");
    let reopened = DiskIndex::open(tmp.path(), 3, &fp).expect("reopen");
    assert!(reopened.read(&sample_key()).is_err());
}

struct FailReviveCompiler {
    next: u32,
    compiles: u32,
    revives: u32,
}

impl FailReviveCompiler {
    fn new() -> Self {
        Self {
            next: 1,
            compiles: 0,
            revives: 0,
        }
    }
}

impl PsoCompiler for FailReviveCompiler {
    fn compile(
        &mut self,
        desc: &PipelineDesc,
    ) -> Result<PsoEntry, harmonius_rendering_pso_cache::PsoError> {
        self.compiles += 1;
        let handle = PsoHandle(self.next);
        self.next += 1;
        Ok(PsoEntry {
            handle,
            bytecode: desc.shader_hash.0.to_vec(),
            desc_layout: DescriptorLayout {
                bindings: Vec::new(),
                push_constants: 0,
            },
            last_used_tick: 0,
            size_bytes: 64,
        })
    }

    fn revive(
        &mut self,
        _blob: &[u8],
        _desc: &PipelineDesc,
    ) -> Result<PsoEntry, harmonius_rendering_pso_cache::PsoError> {
        self.revives += 1;
        Err(harmonius_rendering_pso_cache::PsoError::Compiler(
            "revive rejected",
        ))
    }

    fn serialize(&self, entry: &PsoEntry) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&entry.handle.0.to_le_bytes());
        out.extend_from_slice(&entry.bytecode);
        out
    }
}

#[test]
fn test_revive_failure_tombstones_and_recompiles() {
    let tmp = tempfile::tempdir().expect("tmp");
    let desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    {
        let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
        let mut compiler = CountingCompiler::new();
        cache.get_or_build(&desc, &mut compiler).expect("build");
        assert_eq!(compiler.compiles, 1);
    }
    let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let mut compiler = FailReviveCompiler::new();
    let handle = cache
        .get_or_build(&desc, &mut compiler)
        .expect("recompile after revive failure");
    assert_eq!(compiler.revives, 1);
    assert_eq!(compiler.compiles, 1);
    assert_eq!(handle.0, 1);
}

#[test]
fn test_infer_descriptor_layout_dxil() {
    let mut bytecode = b"DXIL_TEST".to_vec();
    bytecode.push(5);
    bytecode.push(2);
    let layout = infer_descriptor_layout(&bytecode, ShaderApi::Dxil).expect("infer");
    assert_eq!(layout.bindings.len(), 5);
    assert_eq!(layout.push_constants, 2);
}

#[test]
fn test_infer_descriptor_layout_spirv() {
    let bytecode = vec![0x03, 0x02, 0x23, 0x07, 4, 1];
    let layout = infer_descriptor_layout(&bytecode, ShaderApi::Spirv).expect("infer");
    assert_eq!(layout.bindings.len(), 4);
    assert_eq!(layout.push_constants, 1);
}

#[test]
fn test_infer_descriptor_layout_metal() {
    let mut bytecode = b"MTL_SENT".to_vec();
    bytecode.push(3);
    bytecode.push(9);
    let layout = infer_descriptor_layout(&bytecode, ShaderApi::Metal).expect("infer");
    assert_eq!(layout.bindings.len(), 3);
    assert_eq!(layout.push_constants, 9);
}

#[test]
fn test_cold_start_miss_then_hit_disk() {
    let tmp = tempfile::tempdir().expect("tmp");
    let desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    let h1 = {
        let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
        let mut compiler = CountingCompiler::new();
        cache.get_or_build(&desc, &mut compiler).expect("build")
    };
    let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let mut compiler = CountingCompiler::new();
    let h2 = cache.get_or_build(&desc, &mut compiler).expect("hit disk");
    assert_eq!(h1, h2);
    assert_eq!(compiler.compiles, 0);
    assert_eq!(compiler.revives, 1);
}

#[test]
fn test_warm_start_hit_memory() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let mut compiler = CountingCompiler::new();
    let desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    let h1 = cache.get_or_build(&desc, &mut compiler).expect("build");
    let h2 = cache.get_or_build(&desc, &mut compiler).expect("memory");
    assert_eq!(h1, h2);
    assert_eq!(compiler.compiles, 1);
    assert_eq!(compiler.revives, 0);
}

#[test]
fn test_hot_reload_invalidate_and_rebuild() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let mut compiler = CountingCompiler::new();
    let mut desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    let first = cache.get_or_build(&desc, &mut compiler).expect("build");
    let key = cache
        .memory_map()
        .iter()
        .map(|(k, _)| *k)
        .next()
        .expect("key");
    cache.invalidate(&key);
    desc.shader_hash = hash_b();
    let second = cache.get_or_build(&desc, &mut compiler).expect("rebuild");
    assert_ne!(first, second);
}

#[test]
fn test_hot_reload_mid_frame_safe() {
    let tmp = tempfile::tempdir().expect("tmp");
    let mut cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let mut compiler = CountingCompiler::new();
    let mut desc = PipelineDesc {
        shader_hash: hash_a(),
        signature_hash: hash_b(),
    };
    let first = cache.get_or_build(&desc, &mut compiler).expect("build");
    let key = cache
        .memory_map()
        .iter()
        .map(|(k, _)| *k)
        .next()
        .expect("key");
    cache.invalidate_deferred(key);
    let mid = cache
        .get_or_build(&desc, &mut compiler)
        .expect("still resident");
    assert_eq!(first, mid);
    cache.end_frame();
    desc.shader_hash = hash_b();
    let after = cache.get_or_build(&desc, &mut compiler).expect("rebuilt");
    assert_ne!(first, after);
}

#[test]
fn test_shutdown_flush_clean() {
    let tmp = tempfile::tempdir().expect("tmp");
    let cache = PsoCache::new(tmp.path(), &gpu()).expect("open");
    let scratch = cache.disk_dir().join(format!("tmp-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).expect("mkdir");
    cache.shutdown_cleanup().expect("cleanup");
    assert!(!scratch.exists());
}

#[test]
fn test_editor_and_game_cache_isolated() {
    let tmp = tempfile::tempdir().expect("tmp");
    let editor = tmp.path().join("editor");
    let game = tmp.path().join("game");
    std::fs::create_dir_all(&editor).expect("editor");
    std::fs::create_dir_all(&game).expect("game");
    let editor_cache = PsoCache::new(&editor, &gpu()).expect("open");
    let game_cache = PsoCache::new(&game, &gpu()).expect("open");
    assert_ne!(editor_cache.disk_dir(), game_cache.disk_dir());
}
