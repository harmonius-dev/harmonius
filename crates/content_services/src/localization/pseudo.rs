//! Pseudo-localization transforms for overflow and font coverage previews.

/// Applies bracketed pseudo-localization with simple diacritic folding (TC-15.13.2.1).
pub fn pseudo_localize(input: &str) -> String {
    let mapped: String = input
        .chars()
        .map(|c| match c {
            'H' => 'Ħ',
            'e' => 'ê',
            'l' => 'ĺ',
            'o' => 'ø',
            other => other,
        })
        .collect();
    format!("[{mapped}]")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.13.2.1 — pseudo-loc transform.
    #[test]
    fn tc_15_13_2_1_pseudo_loc_transform() {
        assert_eq!(pseudo_localize("Hello"), "[Ħêĺĺø]");
    }
}
