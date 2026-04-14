//! Draw list ordering helpers (opaque batching, R-2.10.4 / TC-2.10.4.1).

/// One instanced draw before sorting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DrawItem {
    /// Batching key: material instance id bucket.
    pub material_id: u16,
    /// Stable label so tests can prove reordering occurred.
    pub draw_index: u32,
}

/// Sorts draws so identical `material_id` values occupy contiguous runs (opaque batching).
pub fn sort_draws_by_material(draws: &mut [DrawItem]) {
    draws.sort_by_key(|d| d.material_id);
}

/// Returns `true` when each material id occupies a single contiguous slice.
#[must_use]
pub fn material_runs_are_contiguous(draws: &[DrawItem]) -> bool {
    if draws.is_empty() {
        return true;
    }
    let mut i = 0;
    while i < draws.len() {
        let m = draws[i].material_id;
        let mut j = i + 1;
        while j < draws.len() && draws[j].material_id == m {
            j += 1;
        }
        if draws[j..].iter().any(|d| d.material_id == m) {
            return false;
        }
        i = j;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{material_runs_are_contiguous, sort_draws_by_material, DrawItem};

    /// TC-2.10.4.1 — after sorting, each material id forms one contiguous run.
    #[test]
    fn test_draw_list_sort_by_material() {
        let mut draws = vec![
            DrawItem {
                material_id: 2,
                draw_index: 0,
            },
            DrawItem {
                material_id: 0,
                draw_index: 1,
            },
            DrawItem {
                material_id: 2,
                draw_index: 2,
            },
            DrawItem {
                material_id: 1,
                draw_index: 3,
            },
            DrawItem {
                material_id: 0,
                draw_index: 4,
            },
        ];
        sort_draws_by_material(&mut draws);
        assert!(material_runs_are_contiguous(&draws));
        assert_eq!(
            draws
                .iter()
                .map(|d| d.material_id)
                .collect::<Vec<_>>(),
            vec![0, 0, 1, 2, 2]
        );
    }
}
