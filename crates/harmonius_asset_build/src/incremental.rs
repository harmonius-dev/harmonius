//! Incremental cache between editor and build (IR-5.1.3).

use std::collections::HashMap;
use std::hash::Hash;

use crate::types::AssetId;

/// Tracks last-known source fingerprints per asset for incremental cooks.
#[derive(Clone, Debug, Default)]
pub struct IncrementalCache<F: Copy + Eq + Hash> {
    fingerprints: HashMap<AssetId, F>,
}

impl<F: Copy + Eq + Hash> IncrementalCache<F> {
    /// Creates an empty cache.
    pub fn new() -> Self {
        Self {
            fingerprints: HashMap::new(),
        }
    }

    /// Returns asset IDs whose fingerprint changed or are unknown.
    pub fn filter_changed(&self, ids: &[AssetId], fp: impl Fn(AssetId) -> F) -> Vec<AssetId> {
        ids.iter()
            .copied()
            .filter(|id| self.fingerprints.get(id).copied() != Some(fp(*id)))
            .collect()
    }

    /// Records fingerprints after a successful cook.
    pub fn commit(&mut self, ids: &[AssetId], fp: impl Fn(AssetId) -> F) {
        for id in ids {
            self.fingerprints.insert(*id, fp(*id));
        }
    }

    /// Removes cached rows for `ids` (forces rebuild next cook).
    pub fn invalidate(&mut self, ids: &[AssetId]) {
        for id in ids {
            self.fingerprints.remove(id);
        }
    }
}
