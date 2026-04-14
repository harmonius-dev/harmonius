//! Deterministic import cache keys.

use crate::{ContentHash, ImportSettings};

/// Cache key = BLAKE3(source hash bytes || settings discriminator).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CacheKey(pub [u8; 32]);

impl CacheKey {
    /// Build from raw source bytes and settings.
    pub fn from_source_and_settings(source: &[u8], settings: &ImportSettings) -> Self {
        let src_hash = ContentHash::from_data(source);
        let mut payload = Vec::new();
        payload.extend_from_slice(&src_hash.bytes);
        match settings {
            ImportSettings::Native => payload.push(0),
            ImportSettings::Texture {
                compression,
                generate_mips,
            } => {
                payload.push(1);
                payload.push(match compression {
                    crate::TextureCompression::Bc7 => 0,
                    crate::TextureCompression::Astc => 1,
                });
                payload.push(u8::from(*generate_mips));
            }
        }
        Self(*blake3::hash(&payload).as_bytes())
    }
}

/// LRU-free map: insert/lookup/invalidate by key.
#[derive(Debug, Default)]
pub struct ImportCache {
    map: std::collections::HashMap<CacheKey, ContentHash>,
}

impl ImportCache {
    /// Lookup cached artifact key.
    pub fn lookup(&self, key: &CacheKey) -> Option<ContentHash> {
        self.map.get(key).copied()
    }

    /// Insert mapping from cache key to artifact hash.
    pub fn insert(&mut self, key: CacheKey, artifact: ContentHash) {
        self.map.insert(key, artifact);
    }

    /// Remove one key.
    pub fn invalidate(&mut self, key: &CacheKey) -> bool {
        self.map.remove(key).is_some()
    }
}
