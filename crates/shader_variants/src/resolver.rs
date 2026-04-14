//! Runtime resolver: bundle hit path plus on-demand compilation fallback.

use crate::bundle::{VariantBundle, VariantError};
use crate::metrics::UsageMetrics;
use crate::permutation::PermutationKey;

/// Compiles a missing [`PermutationKey`] when the bundle does not contain bytecode.
pub trait OnDemandCompiler {
    /// Produces fresh bytecode for `key` (for example by invoking DXC).
    fn compile(&mut self, key: PermutationKey) -> Result<Vec<u8>, VariantError>;
}

/// Resolves bytecode for a key, preferring the mmap bundle and falling back to `compiler`.
pub struct ShaderResolver<C: OnDemandCompiler> {
    bundle: VariantBundle,
    compiler: C,
    metrics: UsageMetrics,
    scratch: Vec<u8>,
}

impl<C: OnDemandCompiler> ShaderResolver<C> {
    /// Builds a resolver over a mapped bundle and an on-demand compiler implementation.
    #[must_use]
    pub fn new(bundle: VariantBundle, compiler: C) -> Self {
        Self {
            bundle,
            compiler,
            metrics: UsageMetrics::new(),
            scratch: Vec::new(),
        }
    }

    /// Returns bytecode for `key`, updating [`UsageMetrics`].
    pub fn resolve(&mut self, key: PermutationKey) -> Result<&[u8], VariantError> {
        if let Some(rec) = self.bundle.get_record(&key) {
            self.metrics.record_hit(key);
            return Ok(self.bundle.slice(rec));
        }
        self.metrics.record_miss(key);
        self.scratch = self.compiler.compile(key)?;
        Ok(&self.scratch)
    }

    /// Borrow resolver telemetry.
    #[must_use]
    pub fn metrics(&self) -> &UsageMetrics {
        &self.metrics
    }

    /// Mutable metrics access (for coverage helpers).
    pub fn metrics_mut(&mut self) -> &mut UsageMetrics {
        &mut self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::{ShaderApi, VariantBundleWriter};
    use crate::permutation::{LodLevel, RenderPath, ShaderFeatures, ShadingModel};
    use tempfile::tempdir;

    #[derive(Default)]
    struct FakeCompiler {
        out: Vec<u8>,
    }

    impl OnDemandCompiler for FakeCompiler {
        fn compile(&mut self, _: PermutationKey) -> Result<Vec<u8>, VariantError> {
            Ok(self.out.clone())
        }
    }

    fn key(i: u32) -> PermutationKey {
        PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures { bits: i },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        }
    }

    #[test]
    fn test_on_demand_compile_missing_key() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("e.pak");
        let w = VariantBundleWriter::new(ShaderApi::D3D12);
        w.write_to_path(&p).unwrap();
        let b = VariantBundle::open_mmap(&p).unwrap();
        let mut r = ShaderResolver::new(b, FakeCompiler { out: vec![9, 9, 9] });
        let out = r.resolve(key(7)).unwrap();
        assert_eq!(out, &[9, 9, 9]);
    }

    #[test]
    fn test_on_demand_updates_miss_log() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("e2.pak");
        let w = VariantBundleWriter::new(ShaderApi::D3D12);
        w.write_to_path(&p).unwrap();
        let b = VariantBundle::open_mmap(&p).unwrap();
        let mut r = ShaderResolver::new(b, FakeCompiler { out: vec![1] });
        let _ = r.resolve(key(42)).unwrap();
        assert!(r.metrics().miss_log().contains(&key(42)));
    }
}
