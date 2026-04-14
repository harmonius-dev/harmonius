//! Build-time / CI coverage summaries over bundles and metrics.

use crate::bundle::VariantBundle;
use crate::metrics::UsageMetrics;
use crate::permutation::PermutationKey;

/// Simple aggregate for tooling and tests (`TC-2.3.10.7`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoverageReport {
    /// Entries in the bundle index.
    pub indexed_variants: usize,
    /// Sum of recorded bundle hits.
    pub hit_total: u64,
    /// Recent misses copied from metrics (FIFO order preserved).
    pub miss_samples: Vec<PermutationKey>,
}

/// Builds a [`CoverageReport`] from a bundle and resolver metrics snapshot.
#[must_use]
pub fn coverage_report(bundle: &VariantBundle, metrics: &UsageMetrics) -> CoverageReport {
    let hit_total: u64 = metrics.hit_counts().values().sum();
    CoverageReport {
        indexed_variants: bundle.variant_count(),
        hit_total,
        miss_samples: metrics.miss_log().iter().copied().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::{ShaderApi, VariantBundleWriter};
    use crate::permutation::{LodLevel, RenderPath, ShaderFeatures, ShadingModel};
    use tempfile::tempdir;

    fn k(i: u32) -> crate::permutation::PermutationKey {
        crate::permutation::PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures { bits: i },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        }
    }

    #[test]
    fn test_coverage_report_counts_variants() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("c.pak");
        let mut w = VariantBundleWriter::new(ShaderApi::Vulkan);
        w.push_variant(k(0), vec![1], 0);
        w.push_variant(k(1), vec![2], 0);
        w.write_to_path(&p).unwrap();
        let b = VariantBundle::open_mmap(&p).unwrap();
        let m = UsageMetrics::new();
        let r = coverage_report(&b, &m);
        assert_eq!(r.indexed_variants, 2);
    }

    #[test]
    fn test_coverage_report_reports_miss() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("c2.pak");
        let mut w = VariantBundleWriter::new(ShaderApi::Metal);
        w.push_variant(k(0), vec![1], 0);
        w.write_to_path(&p).unwrap();
        let b = VariantBundle::open_mmap(&p).unwrap();
        let mut m = UsageMetrics::new();
        m.record_miss(k(9));
        let r = coverage_report(&b, &m);
        assert!(r.miss_samples.contains(&k(9)));
    }
}
