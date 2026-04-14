//! Crash signature aggregation (`R-14.4.3`).

use std::collections::HashMap;

/// Top frames used to build a crash signature.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StackSignature(pub Vec<u64>);

/// Counts crashes grouped by [`StackSignature`].
#[derive(Debug, Default)]
pub struct CrashAggregator {
    groups: HashMap<StackSignature, u32>,
}

impl CrashAggregator {
    /// Creates an empty aggregator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one crash with the given top-frame fingerprint.
    pub fn submit(&mut self, top_frames: Vec<u64>) {
        let sig = StackSignature(top_frames);
        *self.groups.entry(sig).or_insert(0) += 1;
    }

    /// Returns the occurrence count for a signature.
    pub fn count(&self, sig: &StackSignature) -> u32 {
        self.groups.get(sig).copied().unwrap_or(0)
    }

    /// Returns the number of distinct signatures tracked.
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}
