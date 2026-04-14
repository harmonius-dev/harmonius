//! Helpers for GPU dispatch sizing (`TC-11.1.1.8`).

/// Returns the number of one-dimensional compute thread groups for `alive` threads when each group
/// has `threads_per_group` threads (rounded up, minimum 1 when `alive > 0`).
///
/// Matches `ceil(alive / threads_per_group)` for non-zero `alive`.
#[must_use]
pub fn compute_thread_group_count_u32(alive: u32, threads_per_group: u32) -> u32 {
    if alive == 0 {
        return 0;
    }
    let g = threads_per_group.max(1);
    alive.div_ceil(g)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.1.8` — 500 alive particles, group size 256 → 2 groups.
    #[test]
    fn tc_11_1_1_8_indirect_dispatch_args() {
        assert_eq!(compute_thread_group_count_u32(500, 256), 2);
        assert_eq!(compute_thread_group_count_u32(256, 256), 1);
        assert_eq!(compute_thread_group_count_u32(257, 256), 2);
        assert_eq!(compute_thread_group_count_u32(0, 256), 0);
    }
}
