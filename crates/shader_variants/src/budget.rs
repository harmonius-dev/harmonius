//! Build-time variant budget enforcement.

use crate::permutation::{PermutationKey, RenderPath, ShadingModel};

/// Hard caps from the shader variant design.
pub const MAX_VARIANTS_PER_MATERIAL_GRAPH: usize = 64;
/// Maximum variants sharing one shading model.
pub const MAX_VARIANTS_PER_SHADING_MODEL: usize = 256;
/// Maximum variants sharing one render path.
pub const MAX_VARIANTS_PER_RENDER_PATH: usize = 1024;
/// Maximum variants in one project bundle.
pub const MAX_VARIANTS_PER_PROJECT: usize = 4096;

/// Which budget rule was violated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BudgetViolationKind {
    /// More than 64 keys in one material graph group.
    PerMaterialGraph,
    /// More than 256 keys under one shading model.
    PerShadingModel,
    /// More than 1024 keys under one render path.
    PerRenderPath,
    /// More than 4096 keys in the project-wide set.
    PerProject,
}

/// Returned when a build-time budget check fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildBudgetError {
    /// Which cap was exceeded.
    pub kind: BudgetViolationKind,
    /// Representative keys proving the violation (truncated to reasonable size).
    pub offending_keys: Vec<PermutationKey>,
}

/// Validates caps for material graph groups and the flat project list.
///
/// `material_groups` lists disjoint variant sets, one per material graph.
pub fn validate_variant_budgets(
    material_groups: &[Vec<PermutationKey>],
    project_keys: &[PermutationKey],
) -> Result<(), BuildBudgetError> {
    for group in material_groups {
        if group.len() > MAX_VARIANTS_PER_MATERIAL_GRAPH {
            return Err(BuildBudgetError {
                kind: BudgetViolationKind::PerMaterialGraph,
                offending_keys: group.iter().copied().take(8).collect(),
            });
        }
    }

    if project_keys.len() > MAX_VARIANTS_PER_PROJECT {
        return Err(BuildBudgetError {
            kind: BudgetViolationKind::PerProject,
            offending_keys: project_keys.iter().copied().take(8).collect(),
        });
    }

    let mut by_shading: std::collections::BTreeMap<ShadingModel, Vec<PermutationKey>> =
        std::collections::BTreeMap::new();
    let mut by_path: std::collections::BTreeMap<RenderPath, Vec<PermutationKey>> =
        std::collections::BTreeMap::new();

    for k in project_keys {
        by_shading.entry(k.shading_model).or_default().push(*k);
        by_path.entry(k.render_path).or_default().push(*k);
    }

    for keys in by_shading.values() {
        if keys.len() > MAX_VARIANTS_PER_SHADING_MODEL {
            return Err(BuildBudgetError {
                kind: BudgetViolationKind::PerShadingModel,
                offending_keys: keys.iter().copied().take(8).collect(),
            });
        }
    }

    for keys in by_path.values() {
        if keys.len() > MAX_VARIANTS_PER_RENDER_PATH {
            return Err(BuildBudgetError {
                kind: BudgetViolationKind::PerRenderPath,
                offending_keys: keys.iter().copied().take(8).collect(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permutation::{LodLevel, ShaderFeatures};

    fn base_key() -> PermutationKey {
        PermutationKey {
            shading_model: ShadingModel::Lit,
            features: ShaderFeatures { bits: 0 },
            render_path: RenderPath::Forward,
            lod: LodLevel::High,
        }
    }

    #[test]
    fn test_budget_per_material_64() {
        let g: Vec<PermutationKey> = (0..65u32)
            .map(|i| PermutationKey {
                features: ShaderFeatures { bits: i },
                ..base_key()
            })
            .collect();
        let err = validate_variant_budgets(std::slice::from_ref(&g), &g).unwrap_err();
        assert_eq!(err.kind, BudgetViolationKind::PerMaterialGraph);
        assert!(!err.offending_keys.is_empty());
    }

    #[test]
    fn test_budget_per_shading_model_256() {
        let keys: Vec<PermutationKey> = (0..257u32)
            .map(|i| PermutationKey {
                features: ShaderFeatures { bits: i },
                ..base_key()
            })
            .collect();
        let err = validate_variant_budgets(&[], &keys).unwrap_err();
        assert_eq!(err.kind, BudgetViolationKind::PerShadingModel);
    }

    #[test]
    fn test_budget_per_project_4096() {
        let keys: Vec<PermutationKey> = (0..4097u32)
            .map(|i| PermutationKey {
                shading_model: ShadingModel::VARIANTS[(i as usize) % ShadingModel::VARIANTS.len()],
                features: ShaderFeatures { bits: i },
                render_path: RenderPath::VARIANTS[(i as usize) % RenderPath::VARIANTS.len()],
                lod: LodLevel::VARIANTS[(i as usize) % LodLevel::VARIANTS.len()],
            })
            .collect();
        let err = validate_variant_budgets(&[], &keys).unwrap_err();
        assert_eq!(err.kind, BudgetViolationKind::PerProject);
    }

    #[test]
    fn test_budget_per_render_path_1024() {
        let keys: Vec<PermutationKey> = (0..1025u32)
            .map(|i| PermutationKey {
                shading_model: ShadingModel::VARIANTS[(i as usize) % ShadingModel::VARIANTS.len()],
                features: ShaderFeatures { bits: i },
                render_path: RenderPath::Forward,
                lod: LodLevel::VARIANTS[(i as usize) % LodLevel::VARIANTS.len()],
            })
            .collect();
        let err = validate_variant_budgets(&[], &keys).unwrap_err();
        assert_eq!(err.kind, BudgetViolationKind::PerRenderPath);
    }

    #[test]
    fn test_over_budget_fails_build() {
        let g: Vec<PermutationKey> = (0..65u32)
            .map(|i| PermutationKey {
                features: ShaderFeatures { bits: i },
                ..base_key()
            })
            .collect();
        let err = validate_variant_budgets(std::slice::from_ref(&g), &[]).unwrap_err();
        assert!(!err.offending_keys.is_empty());
    }
}
