//! LRU-ish player asset cache (`R-14.5.9`).

use std::collections::{HashMap, VecDeque};

/// Cache partition for accounting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CacheCategory {
    /// Downloaded asset bundle bytes.
    AssetBundle,
    /// Miscellaneous cached blobs.
    Misc,
}

/// Byte accounting per [`CacheCategory`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CacheStats {
    /// Total stored bytes.
    pub total_bytes: u64,
    /// Per-category totals.
    pub per_category: HashMap<CacheCategory, u64>,
}

/// Simple LRU player cache with a byte budget.
#[derive(Debug)]
pub struct PlayerCache {
    max_bytes: u64,
    order: VecDeque<String>,
    entries: HashMap<String, (CacheCategory, Vec<u8>)>,
}

impl PlayerCache {
    /// Creates a cache with the given byte cap.
    pub fn new(max_bytes: u64) -> Self {
        Self {
            max_bytes,
            order: VecDeque::new(),
            entries: HashMap::new(),
        }
    }

    /// Inserts or replaces an entry.
    pub fn put(&mut self, key: &str, category: CacheCategory, data: &[u8]) {
        if self.entries.remove(key).is_some() {
            self.order.retain(|k| k != key);
        }
        self.entries
            .insert(key.into(), (category, data.into()));
        self.order.push_back(key.into());
    }

    /// Fetches a cached blob.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.entries.get(key).map(|(_, v)| v.clone())
    }

    /// Evicts oldest entries until under budget.
    pub fn evict_to_budget(&mut self) {
        while self.total_bytes() > self.max_bytes {
            if let Some(k) = self.order.pop_front() {
                self.entries.remove(&k);
            } else {
                break;
            }
        }
    }

    /// Returns accounting snapshot.
    pub fn stats(&self) -> CacheStats {
        let mut per_category: HashMap<CacheCategory, u64> = HashMap::new();
        let mut total = 0u64;
        for (cat, data) in self.entries.values() {
            total += data.len() as u64;
            *per_category.entry(*cat).or_insert(0) += data.len() as u64;
        }
        CacheStats {
            total_bytes: total,
            per_category,
        }
    }

    fn total_bytes(&self) -> u64 {
        self.entries.values().map(|(_, d)| d.len() as u64).sum()
    }
}
