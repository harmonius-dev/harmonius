//! Bindless material parameter indexing (R-2.10.5, TC-2.10.5.1).

/// Result of planning draws that share one bindless descriptor set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindlessMaterialDrawPlan {
    /// Number of root/binding descriptor set changes while encoding draws (bindless path: `0`).
    pub descriptor_set_switches: u32,
    /// Per-draw indices into the bindless parameter buffer (one slot per draw).
    pub param_buffer_indices: Vec<u32>,
}

/// Builds a bindless submission plan: one shared descriptor set, distinct param slots.
///
/// `material_ids` holds one material id per draw; returned indices are draw order slots.
#[must_use]
pub fn plan_bindless_material_draws(material_ids: &[u32]) -> BindlessMaterialDrawPlan {
    let param_buffer_indices: Vec<u32> = (0..material_ids.len() as u32).collect();
    BindlessMaterialDrawPlan {
        descriptor_set_switches: 0,
        param_buffer_indices,
    }
}

#[cfg(test)]
mod tests {
    use super::plan_bindless_material_draws;

    /// TC-2.10.5.1 — many draws share one descriptor set; each draw gets a distinct param index.
    #[test]
    fn test_bindless_param_buffer() {
        let material_ids: Vec<u32> = (0..1000).collect();
        let plan = plan_bindless_material_draws(&material_ids);
        assert_eq!(plan.descriptor_set_switches, 0);
        assert_eq!(plan.param_buffer_indices.len(), 1000);
        for (draw, &idx) in plan.param_buffer_indices.iter().enumerate() {
            assert_eq!(idx, draw as u32);
        }
    }
}
