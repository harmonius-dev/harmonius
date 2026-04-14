//! L1/L2 cache behaviors: eviction, GC, backend selection, metrics (R-15.11.*).

use std::collections::{HashMap, VecDeque};

/// L1 disk cache with LRU eviction (TC-15.11.5.1).
#[derive(Debug)]
pub(crate) struct LruL1Cache {
    cap: usize,
    order: VecDeque<u64>,
    entries: HashMap<u64, Vec<u8>>,
}

impl LruL1Cache {
    /// Creates an LRU cache with a maximum entry count.
    #[must_use]
    pub(crate) fn new(cap: usize) -> Self {
        Self {
            cap: cap.max(1),
            order: VecDeque::new(),
            entries: HashMap::new(),
        }
    }

    /// Inserts a key; evicts LRU entries when over capacity.
    pub(crate) fn put(&mut self, key: u64, value: Vec<u8>) {
        if self.entries.contains_key(&key) {
            self.remove_key_from_order(key);
        }
        self.entries.insert(key, value);
        self.order.push_back(key);
        while self.entries.len() > self.cap {
            if let Some(victim) = self.order.pop_front() {
                self.entries.remove(&victim);
            }
        }
    }

    /// Gets a key and refreshes LRU order.
    #[must_use]
    pub(crate) fn get(&mut self, key: u64) -> Option<Vec<u8>> {
        if !self.entries.contains_key(&key) {
            return None;
        }
        self.remove_key_from_order(key);
        self.order.push_back(key);
        self.entries.get(&key).cloned()
    }

    fn remove_key_from_order(&mut self, key: u64) {
        if let Some(pos) = self.order.iter().position(|k| *k == key) {
            self.order.remove(pos);
        }
    }
}

/// GC retention decision (TC-15.11.5.2).
#[must_use]
pub(crate) fn gc_should_retain(last_access_days_ago: u32, retain_days: u32) -> bool {
    last_access_days_ago <= retain_days
}

/// Remote object-store backend for L2 (TC-15.11.6.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CacheBackendKind {
    /// Garage S3-compatible backend.
    Garage,
    /// TiKV metadata / small object backend.
    Tikv,
}

/// Selects backend from a textual feature flag.
#[must_use]
pub(crate) fn cache_backend_from_flag(flag: &str) -> Option<CacheBackendKind> {
    match flag {
        "garage" => Some(CacheBackendKind::Garage),
        "tikv" => Some(CacheBackendKind::Tikv),
        _ => None,
    }
}

/// Hit/miss counters for cache telemetry (TC-15.11.8.1).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct CacheMetrics {
    /// L1 hits.
    pub hits: u64,
    /// L1 misses.
    pub misses: u64,
}

impl CacheMetrics {
    /// Increments hit counter.
    pub(crate) fn record_hit(&mut self) {
        self.hits = self.hits.saturating_add(1);
    }

    /// Increments miss counter.
    pub(crate) fn record_miss(&mut self) {
        self.misses = self.misses.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CacheBackendKind, CacheMetrics, LruL1Cache, cache_backend_from_flag, gc_should_retain,
    };

    /// TC-15.11.5.1 — L1 LRU eviction (R-15.11.5).
    #[test]
    fn tc_15_11_5_1_lru_eviction() {
        let mut c = LruL1Cache::new(2);
        c.put(1, vec![10]);
        c.put(2, vec![20]);
        c.put(3, vec![30]);
        assert!(c.get(1).is_none());
        assert_eq!(c.get(2), Some(vec![20]));
    }

    /// TC-15.11.5.2 — GC retention policy (R-15.11.5).
    #[test]
    fn tc_15_11_5_2_gc_retention() {
        assert!(gc_should_retain(2, 7));
        assert!(!gc_should_retain(10, 7));
    }

    /// TC-15.11.6.1 — Cache backend switch (Garage/TiKV) (R-15.11.6).
    #[test]
    fn tc_15_11_6_1_backend_switch() {
        assert_eq!(
            cache_backend_from_flag("garage"),
            Some(CacheBackendKind::Garage)
        );
        assert_eq!(
            cache_backend_from_flag("tikv"),
            Some(CacheBackendKind::Tikv)
        );
    }

    /// TC-15.11.8.1 — Cache hit/miss metric counter (R-15.11.8).
    #[test]
    fn tc_15_11_8_1_metrics() {
        let mut m = CacheMetrics::default();
        m.record_hit();
        m.record_miss();
        assert_eq!(m.hits, 1);
        assert_eq!(m.misses, 1);
    }
}
