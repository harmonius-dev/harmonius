//! Deterministic translation-memory fuzzy scoring.

/// Normalized `[0,1]` similarity score between `source` and `candidate`.
pub fn fuzzy_similarity(source: &str, candidate: &str) -> f32 {
    if source == candidate {
        return 1.0;
    }
    let dist = levenshtein(source, candidate) as f32;
    let denom = source.len().max(candidate.len()).max(1) as f32;
    1.0_f32 - (dist / denom).min(1.0)
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();
    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr = vec![0usize; m + 1];
    for i in 1..=n {
        curr[0] = i;
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (curr[j - 1] + 1)
                .min(prev[j] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.13.1.3 — fuzzy match between singular and plural forms is strong.
    #[test]
    fn tc_15_13_1_3_tm_fuzzy_match_accuracy() {
        let score = fuzzy_similarity("Save file", "Save files");
        assert!(score >= 0.85, "score={score}");
    }
}
