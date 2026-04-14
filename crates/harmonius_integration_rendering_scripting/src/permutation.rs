//! Sorted permutation table lookup (IR-3.5.3).

use crate::shader_types::{cmp_permutation_key, PermutationKey};

/// Builds a sorted `(key, user_index)` table for binary search (IR-3.5.3).
#[must_use]
pub fn permutation_table_from_keys(keys: &[(PermutationKey, u32)]) -> Vec<(PermutationKey, u32)> {
    let mut out: Vec<(PermutationKey, u32)> = keys.to_vec();
    out.sort_by(|a, b| cmp_permutation_key(&a.0, &b.0));
    out
}

/// Returns the position of `needle` in a sorted `table`, or `None`.
#[must_use]
pub fn permutation_lookup_index(
    table: &[(PermutationKey, u32)],
    needle: &PermutationKey,
) -> Option<usize> {
    table
        .binary_search_by(|probe| cmp_permutation_key(&probe.0, needle))
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shader_types::{RenderPath, ShaderFeatures, ShadingModel};

    #[test]
    fn tc_ir_3_5_3_u1_permutation_fingerprint_stable() {
        let k = PermutationKey {
            shading_model: ShadingModel::DefaultLit,
            features: ShaderFeatures::NORMAL_MAP,
            render_path: RenderPath::ForwardPlus,
        };
        let v = k.stable_fingerprint();
        for _ in 0..1000 {
            assert_eq!(k.stable_fingerprint(), v);
        }
    }

    #[test]
    fn tc_ir_3_5_3_u2_all_nine_feature_bits() {
        let f = ShaderFeatures::ALL_NAMED_BITS;
        assert_eq!(f.0, 0x1FF);
    }

    #[test]
    fn tc_ir_3_5_3_u3_sorted_table_binary_search_64_variants() {
        let mut keys: Vec<(PermutationKey, u32)> = Vec::new();
        for i in 0u32..64 {
            keys.push((
                PermutationKey {
                    shading_model: ShadingModel::DefaultLit,
                    features: ShaderFeatures(i),
                    render_path: RenderPath::ForwardPlus,
                },
                i,
            ));
        }
        let table = permutation_table_from_keys(&keys);
        for (i, row) in table.iter().enumerate().take(63) {
            assert!(cmp_permutation_key(&row.0, &table[i + 1].0).is_le());
        }
        let needle = PermutationKey {
            shading_model: ShadingModel::DefaultLit,
            features: ShaderFeatures(37),
            render_path: RenderPath::ForwardPlus,
        };
        let idx = permutation_lookup_index(&table, &needle).expect("found");
        assert_eq!(table[idx].1, 37);
    }
}
