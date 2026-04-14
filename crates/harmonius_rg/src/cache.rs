//! Compile cache keyed by topology hash.

use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

use crate::builder::RenderGraph;

/// Tracks topology signatures across compiles (TC-2.2.10.x).
#[derive(Debug, Default)]
pub struct GraphCompileCache {
    seen: BTreeSet<u64>,
    /// New topologies after the first stored signature.
    pub topology_changes: u32,
    /// Compiles whose topology signature matched an existing entry.
    pub topology_reuse_hits: u32,
}

impl GraphCompileCache {
    /// Stable hash of pass topology for cache indexing.
    pub fn topology_signature(&self, graph: &RenderGraph) -> u64 {
        let mut h = DefaultHasher::new();
        graph.passes.len().hash(&mut h);
        for (i, p) in graph.passes.iter().enumerate() {
            i.hash(&mut h);
            p.name.hash(&mut h);
            p.reads.hash(&mut h);
            p.writes.hash(&mut h);
            p.requires.hash(&mut h);
            p.fallback.hash(&mut h);
            p.queue.hash(&mut h);
        }
        h.finish()
    }

    /// Updates hit / change counters for this compile.
    pub fn observe_topology(&mut self, sig: u64) {
        if self.seen.contains(&sig) {
            self.topology_reuse_hits = self.topology_reuse_hits.saturating_add(1);
            return;
        }
        if !self.seen.is_empty() {
            self.topology_changes = self.topology_changes.saturating_add(1);
        }
        self.seen.insert(sig);
    }

    /// `(topology_changes, topology_reuse_hits, distinct_topology_entries)`.
    #[must_use]
    pub fn stats(&self) -> (u32, u32, usize) {
        (
            self.topology_changes,
            self.topology_reuse_hits,
            self.seen.len(),
        )
    }
}
