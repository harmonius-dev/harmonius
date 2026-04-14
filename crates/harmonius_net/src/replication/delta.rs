//! Field-level delta encoding for replicated components.

/// Twenty-field stand-in for codegen output in tests.
#[derive(Clone, Debug, PartialEq)]
pub struct Component20Fields {
    /// Fields 0..19.
    pub f: [i32; 20],
}

/// Returns `(field_indices, values)` for changed fields only.
pub fn delta_encode_component20(
    before: &Component20Fields,
    after: &Component20Fields,
) -> (Vec<usize>, Vec<i32>) {
    let mut idx = Vec::new();
    let mut vals = Vec::new();
    for i in 0..20 {
        if before.f[i] != after.f[i] {
            idx.push(i);
            vals.push(after.f[i]);
        }
    }
    (idx, vals)
}

/// Full-state wire size for tests (index + value per field).
pub fn full_state_bytes() -> usize {
    20 * (1 + 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.2.1.1 — single-field mutation produces a minimal delta.
    #[test]
    fn test_delta_only_changed_field() {
        let mut a = Component20Fields { f: [0; 20] };
        let b = a.clone();
        a.f[7] = 99;
        let (ix, v) = delta_encode_component20(&b, &a);
        assert_eq!(ix, vec![7]);
        assert_eq!(v, vec![99]);
        let delta_bytes = ix.len() * (1 + 4);
        assert!(delta_bytes < full_state_bytes() * 2 / 10);
    }
}
