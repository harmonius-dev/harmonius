//! Instance compaction into indirect draw batches by material (R-2.3.7).

/// One contiguous indirect draw after sorting instances by `material_id`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaterialIndirectBatch {
    /// Shared material for this draw call.
    pub material_id: u16,
    /// Number of instances referencing `material_id`.
    pub instance_count: u32,
}

/// Sorts `instance_materials` and returns one batch per distinct material id.
///
/// The sum of `instance_count` equals `instance_materials.len()`.
#[must_use]
pub fn indirect_batches_by_material(instance_materials: &[u16]) -> Vec<MaterialIndirectBatch> {
    if instance_materials.is_empty() {
        return Vec::new();
    }
    let mut mats = instance_materials.to_vec();
    mats.sort_unstable();
    let mut out = Vec::new();
    let mut i = 0_usize;
    while i < mats.len() {
        let m = mats[i];
        let mut j = i + 1;
        while j < mats.len() && mats[j] == m {
            j += 1;
        }
        out.push(MaterialIndirectBatch {
            material_id: m,
            instance_count: (j - i) as u32,
        });
        i = j;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::indirect_batches_by_material;

    /// TC-2.3.7.1 — ten thousand instances across five materials compact to five batches.
    #[test]
    fn test_indirect_compact_by_material() {
        let mut instance_materials = Vec::with_capacity(10_000);
        for i in 0_u32..10_000 {
            instance_materials.push((i % 5) as u16);
        }
        let batches = indirect_batches_by_material(&instance_materials);
        assert_eq!(batches.len(), 5);
        let total: u32 = batches.iter().map(|b| b.instance_count).sum();
        assert_eq!(total, 10_000);
        for b in &batches {
            assert_eq!(b.instance_count, 2000);
        }
    }
}
