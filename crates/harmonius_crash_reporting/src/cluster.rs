//! Cluster upsert bookkeeping (`R-14.4.3`).

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use smallvec::SmallVec;

use crate::fingerprint::StackFingerprint;
use crate::symbols::BuildId;

/// Identifier returned after ingestion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ClusterId(pub u64);

/// Aggregated crash cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cluster {
    /// Stable identifier.
    pub id: ClusterId,
    /// Fingerprint grouping key.
    pub fingerprint: StackFingerprint,
    /// Number of ingested dumps in this cluster.
    pub count: u64,
    /// First observation (unix millis).
    pub first_seen: u64,
    /// Last observation (unix millis).
    pub last_seen: u64,
    /// Builds observed in this cluster.
    pub affected_builds: SmallVec<[BuildId; 8]>,
}

/// In-memory cluster store for unit tests.
#[derive(Debug, Default)]
pub struct ClusterStore {
    next_id: u64,
    map: HashMap<StackFingerprint, Cluster>,
}

impl ClusterStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Upserts a cluster for `fingerprint`, optionally recording `build`.
    pub fn upsert(&mut self, fingerprint: StackFingerprint, build: Option<BuildId>) -> ClusterId {
        let now = unix_millis();
        if let Some(existing) = self.map.get_mut(&fingerprint) {
            existing.count = existing.count.saturating_add(1);
            existing.last_seen = now;
            if let Some(b) = build {
                if !existing.affected_builds.iter().any(|x| x == &b)
                    && existing.affected_builds.len() < 8
                {
                    existing.affected_builds.push(b);
                }
            }
            return existing.id;
        }
        self.next_id = self.next_id.saturating_add(1);
        let id = ClusterId(self.next_id);
        let mut affected_builds = SmallVec::new();
        if let Some(b) = build {
            affected_builds.push(b);
        }
        self.map.insert(
            fingerprint,
            Cluster {
                id,
                fingerprint,
                count: 1,
                first_seen: now,
                last_seen: now,
                affected_builds,
            },
        );
        id
    }

    /// Returns the cluster for a fingerprint when present.
    #[must_use]
    pub fn get(&self, fingerprint: &StackFingerprint) -> Option<&Cluster> {
        self.map.get(fingerprint)
    }
}

fn unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_upsert_increments_count() {
        let mut store = ClusterStore::new();
        let fp = StackFingerprint([7u8; 32]);
        let id1 = store.upsert(fp, None);
        let id2 = store.upsert(fp, None);
        assert_eq!(id1, id2);
        assert_eq!(store.get(&fp).unwrap().count, 2);
    }
}
