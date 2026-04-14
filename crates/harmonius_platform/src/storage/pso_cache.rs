//! Pipeline state object cache (`R-14.5.11`).

use std::collections::HashMap;

/// GPU + driver identity for cache namespacing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GpuDriverKey {
    /// PCI vendor id.
    pub vendor: u32,
    /// PCI device id.
    pub device: u32,
    /// Driver version string.
    pub driver: String,
}

/// Stores compiled PSO blobs per GPU driver key.
#[derive(Debug)]
pub struct PsoCacheStore {
    key: GpuDriverKey,
    entries: HashMap<Vec<u8>, Vec<u8>>,
}

impl PsoCacheStore {
    /// Opens a store scoped to `key`.
    pub fn new(key: GpuDriverKey) -> Self {
        Self {
            key,
            entries: HashMap::new(),
        }
    }

    /// Inserts a blob under `key_blob`.
    pub fn store(&mut self, key_blob: &[u8], blob: &[u8]) {
        self.entries.insert(key_blob.into(), blob.into());
    }

    /// Fetches a blob if present.
    pub fn get(&self, key_blob: &[u8]) -> Option<Vec<u8>> {
        self.entries.get(key_blob).cloned()
    }

    /// Clears all entries, returning how many were removed.
    pub fn invalidate_all(&mut self) -> u32 {
        let n = self.entries.len() as u32;
        self.entries.clear();
        n
    }

    /// Imports entries from another store only when GPU keys match.
    pub fn load_all_from_store(&mut self, other: &PsoCacheStore) {
        if self.key != other.key {
            return;
        }
        self.entries.clone_from(&other.entries);
    }
}
