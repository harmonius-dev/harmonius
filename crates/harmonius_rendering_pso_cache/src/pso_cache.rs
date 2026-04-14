use std::path::{Path, PathBuf};

use crate::cache_error::{CacheError, PsoError};
use crate::descriptor_layout::DescriptorLayout;
use crate::device::{DeviceFingerprint, GpuDevice};
use crate::disk_index::DiskIndex;
use crate::pso_key::PsoKey;
use crate::sorted_vec_map::SortedVecMap;

/// Default disk LRU cap (design default 512 MiB until [`PsoCache::set_caps`] overrides it).
const DEFAULT_DISK_CAP_BYTES: u64 = 512 * 1024 * 1024;

/// Opaque GPU handle placeholder tracked by tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PsoHandle(pub u32);

/// Minimal pipeline description inputs used for [`PsoKey`] derivation.
///
/// Vertex layout, render-target formats, blend state, and other fixed-function
/// inputs must be folded into `shader_hash` and/or `signature_hash` by the
/// embedding compiler so equal keys imply equal GPU pipeline identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PipelineDesc {
    /// Shader bytecode digest (must cover all stage bytecode inputs).
    pub shader_hash: crate::content_hash::ContentHash,
    /// Root signature / pipeline-layout digest (must cover bind layout inputs).
    pub signature_hash: crate::content_hash::ContentHash,
}

/// Resident PSO entry stored in memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsoEntry {
    /// GPU handle returned to callers.
    pub handle: PsoHandle,
    /// Serialized bytecode snapshot.
    pub bytecode: Vec<u8>,
    /// Descriptor layout inferred once.
    pub desc_layout: DescriptorLayout,
    /// Last successful use tick.
    pub last_used_tick: u64,
    /// Approximate resident size for GC accounting.
    pub size_bytes: u32,
}

/// Compiles or revives PSOs for [`PsoCache`].
pub trait PsoCompiler {
    /// Compiles a fresh PSO for `desc`.
    fn compile(&mut self, desc: &PipelineDesc) -> Result<PsoEntry, PsoError>;

    /// Revives a PSO from a serialized blob.
    fn revive(&mut self, blob: &[u8], desc: &PipelineDesc) -> Result<PsoEntry, PsoError>;

    /// Serializes a resident entry for disk storage.
    fn serialize(&self, entry: &PsoEntry) -> Vec<u8>;
}

/// Two-tier PSO cache (memory + disk) owned by the render thread contract.
#[derive(Debug)]
pub struct PsoCache {
    root: PathBuf,
    format_version: u32,
    memory: SortedVecMap<PsoKey, PsoEntry>,
    disk: DiskIndex,
    device_fp: DeviceFingerprint,
    tick: u64,
    memory_cap_bytes: u64,
    disk_cap_bytes: u64,
    deferred_remove: Vec<PsoKey>,
}

impl PsoCache {
    /// Opens a cache rooted at `root` for `device`.
    pub fn new(root: &Path, device: &GpuDevice) -> Result<Self, CacheError> {
        let fp = device.fingerprint();
        let format_version = 3;
        let disk = DiskIndex::open(root, format_version, &fp)?;
        Ok(Self {
            root: root.to_path_buf(),
            format_version,
            memory: SortedVecMap::new(),
            disk,
            device_fp: fp,
            tick: 0,
            memory_cap_bytes: u64::MAX,
            disk_cap_bytes: DEFAULT_DISK_CAP_BYTES,
            deferred_remove: Vec::new(),
        })
    }

    /// Configures LRU caps used by [`PsoCache::gc`].
    pub fn set_caps(&mut self, memory_cap: u64, disk_cap: u64) {
        self.memory_cap_bytes = memory_cap;
        self.disk_cap_bytes = disk_cap;
    }

    fn make_key(&self, desc: &PipelineDesc) -> PsoKey {
        PsoKey {
            shader_hash: desc.shader_hash,
            signature_hash: desc.signature_hash,
            device_id: crate::device::DeviceId {
                vendor: self.device_fp.vendor_id,
                device: self.device_fp.device_id,
            },
            driver_version: self.device_fp.driver_version,
            format_version: self.format_version,
        }
    }

    fn bump_tick(&mut self) {
        self.tick = self.tick.saturating_add(1);
    }

    /// Current logical tick used for LRU ordering.
    #[must_use]
    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    /// Queues invalidation to be applied at the next frame barrier.
    pub fn invalidate_deferred(&mut self, key: PsoKey) {
        self.deferred_remove.push(key);
    }

    /// Drains deferred invalidations (call at end-of-frame).
    pub fn end_frame(&mut self) {
        let keys: Vec<PsoKey> = self.deferred_remove.drain(..).collect();
        for key in keys {
            self.invalidate(&key);
        }
    }

    /// Removes a single key from memory (disk tombstones are handled separately).
    pub fn invalidate(&mut self, key: &PsoKey) {
        self.memory.remove(key);
    }

    /// Clears the memory tier (disk is retained).
    pub fn invalidate_all(&mut self) {
        self.memory.clear();
    }

    /// Memory tier size in entries.
    #[must_use]
    pub fn memory_len(&self) -> usize {
        self.memory.len()
    }

    /// Borrow the memory tier for structural tests.
    #[must_use]
    pub fn memory_map(&self) -> &SortedVecMap<PsoKey, PsoEntry> {
        &self.memory
    }

    /// Disk directory used by this cache instance.
    #[must_use]
    pub fn disk_dir(&self) -> &Path {
        self.disk.device_dir()
    }

    /// Root directory passed to [`PsoCache::new`](Self::new).
    #[must_use]
    pub fn cache_root(&self) -> &Path {
        &self.root
    }

    /// Performs LRU trimming on memory and disk tiers.
    pub fn gc(&mut self) -> Result<(), CacheError> {
        while self.memory_bytes() > self.memory_cap_bytes && !self.memory.is_empty() {
            let Some((key, _)) = self
                .memory
                .iter()
                .min_by_key(|(_, entry)| entry.last_used_tick)
                .map(|(key, entry)| (*key, entry))
            else {
                break;
            };
            self.memory.remove(&key);
        }
        self.disk.gc_disk_lru(self.disk_cap_bytes)?;
        Ok(())
    }

    fn memory_bytes(&self) -> u64 {
        self.memory
            .values()
            .map(|entry| u64::from(entry.size_bytes))
            .sum()
    }

    /// Primary lookup / build path.
    pub fn get_or_build<C: PsoCompiler>(
        &mut self,
        desc: &PipelineDesc,
        compiler: &mut C,
    ) -> Result<PsoHandle, PsoError> {
        self.bump_tick();
        let key = self.make_key(desc);

        if let Some(entry) = self.memory.get_mut(&key) {
            entry.last_used_tick = self.tick;
            return Ok(entry.handle);
        }

        match self.disk.read(&key) {
            Ok(blob) => match compiler.revive(&blob, desc) {
                Ok(mut entry) => {
                    entry.last_used_tick = self.tick;
                    let handle = entry.handle;
                    self.memory.insert(key, entry);
                    Ok(handle)
                }
                Err(_err) => {
                    let _ = self.disk.remove_key(&key);
                    self.compile_store(desc, compiler, key)
                }
            },
            Err(CacheError::Missing) => self.compile_store(desc, compiler, key),
            Err(CacheError::Corrupted(_)) => {
                let _ = self.disk.remove_key(&key);
                self.compile_store(desc, compiler, key)
            }
            Err(CacheError::Io(err)) => Err(PsoError::Cache(CacheError::Io(err))),
            Err(other) => Err(PsoError::Cache(other)),
        }
    }

    fn compile_store<C: PsoCompiler>(
        &mut self,
        desc: &PipelineDesc,
        compiler: &mut C,
        key: PsoKey,
    ) -> Result<PsoHandle, PsoError> {
        let mut entry = compiler.compile(desc)?;
        let blob = compiler.serialize(&entry);
        self.disk
            .write(&key, &blob, self.tick)
            .map_err(PsoError::Cache)?;
        entry.last_used_tick = self.tick;
        let handle = entry.handle;
        self.memory.insert(key, entry);
        Ok(handle)
    }

    /// Cleans scratch directories under the device cache path.
    pub fn shutdown_cleanup(&self) -> Result<(), CacheError> {
        self.disk.cleanup_tmp()
    }
}
