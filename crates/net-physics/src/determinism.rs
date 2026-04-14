//! Deterministic ordering helpers for cross-platform physics replay.

/// Stable body identifier `(generation, index)` used before island solve.
pub type SolverBodyKey = (u32, u32);

/// Returns `keys` sorted lexicographically for fixed island iteration order.
#[must_use]
pub fn sort_solver_body_keys(mut keys: Vec<SolverBodyKey>) -> Vec<SolverBodyKey> {
    keys.sort();
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_4_5_4_1_solver_key_order_is_stable() {
        let keys = vec![(2, 1), (1, 1), (1, 0), (2, 0)];
        assert_eq!(
            sort_solver_body_keys(keys),
            vec![(1, 0), (1, 1), (2, 0), (2, 1)]
        );
    }
}
