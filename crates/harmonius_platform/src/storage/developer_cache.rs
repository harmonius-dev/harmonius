//! Developer-facing content-addressed cache (`R-14.5.10`).

use std::collections::HashMap;

use crate::filesystem::Blake3Hash;

/// Developer cache partition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DevCacheCategory {
    /// Compiled asset bytes.
    CompiledAsset,
}

/// Hit tier for a [`DeveloperCache::lookup`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheHitTier {
    /// Local disk tier hit.
    Local,
    /// Missed all tiers.
    Miss,
}

/// BLAKE3 content hash alias.
pub type ContentHash = Blake3Hash;

/// Three-tier dev cache (local RAM only in this stub).
#[derive(Debug, Default)]
pub struct DeveloperCache {
    local: HashMap<(ContentHash, DevCacheCategory), Vec<u8>>,
}

impl DeveloperCache {
    /// Creates an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Hashes arbitrary bytes (BLAKE3).
    pub fn hash(data: &[u8]) -> ContentHash {
        Blake3Hash(*blake3::hash(data).as_bytes())
    }

    /// Stores bytes under `hash`.
    pub fn store(&mut self, hash: &ContentHash, category: DevCacheCategory, data: &[u8]) {
        self.local.insert((*hash, category), data.into());
    }

    /// Looks up cached bytes.
    pub fn lookup(
        &self,
        hash: &ContentHash,
        category: DevCacheCategory,
    ) -> (CacheHitTier, Option<Vec<u8>>) {
        match self.local.get(&(*hash, category)) {
            Some(v) => (CacheHitTier::Local, Some(v.clone())),
            None => (CacheHitTier::Miss, None),
        }
    }
}
