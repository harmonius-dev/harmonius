//! Least-significant-digit radix sort for `u32` keys (`TC-11.1.4.5`).

/// Stable LSD radix sort: reorders `indices` so `keys[idx]` is non-decreasing.
pub fn radix_sort_indices_by_u32_keys(keys: &[u32], indices: &mut [usize]) {
    let mut aux_idx = vec![0usize; indices.len()];
    let mut counts = [0usize; 256];
    for shift in (0..32).step_by(8) {
        counts.fill(0);
        for &i in indices.iter() {
            let b = ((keys[i] >> shift) & 0xFF) as usize;
            counts[b] += 1;
        }
        for c in 1..256 {
            counts[c] += counts[c - 1];
        }
        for &i in indices.iter().rev() {
            let b = ((keys[i] >> shift) & 0xFF) as usize;
            counts[b] -= 1;
            let pos = counts[b];
            aux_idx[pos] = i;
        }
        indices.copy_from_slice(&aux_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.4.5` — sorted alive list by distance key (back-to-front uses descending depth).
    #[test]
    fn tc_11_1_4_5_radix_sort_correctness() {
        let keys = [
            100u32, 50, 200, 10, 75, 300, 0, 99, 150, 42, 88, 120, 33, 77, 199, 1, 255, 128, 64, 32,
        ];
        let n = keys.len();
        let mut idx: Vec<usize> = (0..n).collect();
        radix_sort_indices_by_u32_keys(&keys, &mut idx);
        for w in idx.windows(2) {
            assert!(keys[w[0]] <= keys[w[1]]);
        }
    }
}
