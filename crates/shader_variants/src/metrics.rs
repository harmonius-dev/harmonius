//! Runtime usage metrics (hits, bounded miss log).

use std::collections::{BTreeMap, VecDeque};

use crate::permutation::PermutationKey;

const MISS_RING_CAP: usize = 1024;

/// Hit counts and bounded miss ring for shader resolver telemetry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UsageMetrics {
    hit_counts: BTreeMap<PermutationKey, u64>,
    miss_log: VecDeque<PermutationKey>,
}

impl UsageMetrics {
    /// Creates empty metrics.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a bundle hit.
    pub fn record_hit(&mut self, key: PermutationKey) {
        *self.hit_counts.entry(key).or_insert(0) += 1;
    }

    /// Records a bundle miss (pushes into bounded ring).
    pub fn record_miss(&mut self, key: PermutationKey) {
        if self.miss_log.len() == MISS_RING_CAP {
            self.miss_log.pop_front();
        }
        self.miss_log.push_back(key);
    }

    /// Read-only hit totals.
    #[must_use]
    pub fn hit_counts(&self) -> &BTreeMap<PermutationKey, u64> {
        &self.hit_counts
    }

    /// Ordered miss history (oldest first).
    #[must_use]
    pub fn miss_log(&self) -> &VecDeque<PermutationKey> {
        &self.miss_log
    }
}

/// Builds the next precompile list: declared keys that appeared in the miss log (unique, stable).
#[must_use]
pub fn next_precompile_from_metrics(
    metrics: &UsageMetrics,
    declared: &[PermutationKey],
) -> Vec<PermutationKey> {
    let decl: std::collections::BTreeSet<_> = declared.iter().copied().collect();
    let mut out: Vec<PermutationKey> = metrics
        .miss_log
        .iter()
        .copied()
        .filter(|k| decl.contains(k))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permutation::{LodLevel, RenderPath, ShaderFeatures, ShadingModel};

    fn key(bits: u32) -> PermutationKey {
        PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures { bits },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        }
    }

    #[test]
    fn test_metrics_miss_log_bounded() {
        let mut m = UsageMetrics::new();
        for i in 0..2000u32 {
            m.record_miss(key(i));
        }
        assert_eq!(m.miss_log.len(), MISS_RING_CAP);
    }

    #[test]
    fn test_metrics_feeds_next_precompile() {
        let declared = vec![key(1), key(2), key(3)];
        let mut m = UsageMetrics::new();
        m.record_miss(key(2));
        m.record_miss(key(99)); // not declared — ignored
        let next = next_precompile_from_metrics(&m, &declared);
        assert_eq!(next, vec![key(2)]);
    }
}
