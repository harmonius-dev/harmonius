//! Content-addressable blob store with deduplication and mark-sweep GC.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ContentHash;

/// Result of storing a blob.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreResult {
    /// A new file was written.
    Written,
    /// Hash already present; no write.
    Deduplicated,
}

/// Statistics from [`ContentAddressableStore::gc`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GcResult {
    /// Blobs removed from disk that were not referenced.
    pub orphans_removed: usize,
}

/// Filesystem-backed CAS using sharded hex paths.
#[derive(Debug)]
pub struct ContentAddressableStore {
    root: PathBuf,
}

impl ContentAddressableStore {
    /// Opens or creates a CAS rooted at `root`.
    pub fn new(root: PathBuf) -> Self {
        let _ = fs::create_dir_all(&root);
        Self { root }
    }

    fn blob_path(&self, hash: ContentHash) -> PathBuf {
        let p = hash.prefix();
        let hex = hash.hex();
        self.root.join(format!("{:02x}", p[0])).join(hex)
    }

    /// Returns whether `hash` is present on disk.
    pub fn exists(&self, hash: ContentHash) -> bool {
        self.blob_path(hash).is_file()
    }

    /// Stores `data` keyed by `hash`.
    pub fn store(&mut self, hash: ContentHash, data: &[u8]) -> std::io::Result<StoreResult> {
        let path = self.blob_path(hash);
        if path.exists() {
            return Ok(StoreResult::Deduplicated);
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, data)?;
        Ok(StoreResult::Written)
    }

    /// Loads bytes for `hash`, if present.
    pub fn load(&self, hash: ContentHash) -> std::io::Result<Option<Vec<u8>>> {
        let path = self.blob_path(hash);
        if !path.is_file() {
            return Ok(None);
        }
        Ok(Some(fs::read(path)?))
    }

    /// Removes blobs under `root` whose hash is not in `referenced`.
    pub fn gc(&mut self, referenced: &HashSet<ContentHash>) -> std::io::Result<GcResult> {
        let mut orphans_removed = 0usize;
        if !self.root.is_dir() {
            return Ok(GcResult { orphans_removed });
        }
        for entry in walk_files(&self.root)? {
            let Some(name) = entry.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.len() != 64 {
                continue;
            }
            let mut bytes = [0u8; 32];
            for (i, chunk) in name.as_bytes().chunks(2).enumerate().take(32) {
                let s = std::str::from_utf8(chunk).ok();
                let Some(s) = s else {
                    continue;
                };
                let Ok(b) = u8::from_str_radix(s, 16) else {
                    continue;
                };
                bytes[i] = b;
            }
            let hash = ContentHash { bytes };
            if !referenced.contains(&hash) {
                fs::remove_file(&entry)?;
                orphans_removed += 1;
            }
        }
        Ok(GcResult { orphans_removed })
    }

    /// Count blob files (leaf objects) under the CAS root.
    pub fn blob_file_count(&self) -> std::io::Result<usize> {
        Ok(walk_files(&self.root)?.len())
    }
}

fn walk_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !root.is_dir() {
        return Ok(out);
    }
    for e1 in fs::read_dir(root)? {
        let e1 = e1?;
        let p1 = e1.path();
        if p1.is_dir() {
            for e2 in fs::read_dir(&p1)? {
                let e2 = e2?;
                let p2 = e2.path();
                if p2.is_file() {
                    out.push(p2);
                }
            }
        }
    }
    Ok(out)
}
