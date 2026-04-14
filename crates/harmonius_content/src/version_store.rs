//! Version history keyed by content hash.

use std::collections::HashMap;

use crate::ContentHash;

/// Stores immutable versions of an asset's bytes by hash.
#[derive(Debug, Default)]
pub struct VersionStore {
    /// `asset_id.0` -> ordered list of `(hash, bytes)`.
    versions: HashMap<u64, Vec<(ContentHash, Vec<u8>)>>,
}

impl VersionStore {
    /// New empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new version.
    pub fn record(&mut self, asset_key: u64, hash: ContentHash, bytes: Vec<u8>) {
        self.versions.entry(asset_key).or_default().push((hash, bytes));
    }

    /// List all versions for an asset.
    pub fn list(&self, asset_key: u64) -> Vec<ContentHash> {
        self.versions
            .get(&asset_key)
            .map(|v| v.iter().map(|(h, _)| *h).collect())
            .unwrap_or_default()
    }

    /// Restore bytes by exact hash.
    pub fn restore(&self, hash: ContentHash) -> Result<Vec<u8>, &'static str> {
        for chain in self.versions.values() {
            for (h, b) in chain {
                if *h == hash {
                    return Ok(b.clone());
                }
            }
        }
        Err("missing hash")
    }
}
