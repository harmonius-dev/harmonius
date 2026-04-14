//! Precompile list validation against declared variants and caps.

use std::collections::BTreeSet;

use crate::budget::validate_variant_budgets;
use crate::permutation::PermutationKey;

/// Ensures every precompiled key exists in the declared superset.
pub fn validate_precompile_subset(
    declared: &[PermutationKey],
    precompile: &[PermutationKey],
) -> Result<(), PrecompileError> {
    let set: BTreeSet<_> = declared.iter().copied().collect();
    for k in precompile {
        if !set.contains(k) {
            return Err(PrecompileError::NotSubset { missing: *k });
        }
    }
    Ok(())
}

/// Runs [`validate_variant_budgets`] on the union of declared keys (empty material groups).
pub fn validate_post_precompile_caps(
    project_keys: &[PermutationKey],
) -> Result<(), crate::budget::BuildBudgetError> {
    validate_variant_budgets(&[], project_keys)
}

/// Precompile list is not contained in declared variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrecompileError {
    /// A precompile key was not present in `declared`.
    NotSubset {
        /// First offending key.
        missing: PermutationKey,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permutation::{LodLevel, RenderPath, ShaderFeatures, ShadingModel};

    fn k(bits: u32) -> PermutationKey {
        PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures { bits },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        }
    }

    #[test]
    fn test_precompile_list_subset() {
        let declared = vec![k(0), k(1)];
        assert!(validate_precompile_subset(&declared, &[k(0)]).is_ok());
        assert!(validate_precompile_subset(&declared, &[k(2)]).is_err());
    }

    #[test]
    fn test_precompile_respects_caps() {
        let keys: Vec<PermutationKey> = (0..64u32).map(k).collect();
        assert!(validate_post_precompile_caps(&keys).is_ok());
    }
}
