//! Minimal ICU MessageFormat validation and English plural selection for deterministic tests.

/// Failure to treat a pattern as well-formed ICU MessageFormat (bounded structural checks only).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IcuPatternError {
    /// Unbalanced `{` / `}` delimiters.
    UnbalancedDelimiters,
    /// A `plural` block was expected for this validator slice.
    MissingPluralKeyword,
}

/// CLDR-style plural categories needed for editor previews (English-only in this slice).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluralCategory {
    /// CLDR `zero`.
    Zero,
    /// CLDR `one`.
    One,
    /// CLDR `two`.
    Two,
    /// CLDR `few`.
    Few,
    /// CLDR `many`.
    Many,
    /// CLDR `other`.
    Other,
}

/// Validates brace balance and presence of a `plural` selector block (TC-15.13.1.1).
pub fn validate_icu_pattern(pattern: &str) -> Result<(), IcuPatternError> {
    let mut depth = 0i32;
    for ch in pattern.chars() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth < 0 {
                    return Err(IcuPatternError::UnbalancedDelimiters);
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err(IcuPatternError::UnbalancedDelimiters);
    }
    if !pattern.contains("plural") {
        return Err(IcuPatternError::MissingPluralKeyword);
    }
    Ok(())
}

/// English plural category selection used by the string table editor (TC-15.13.1.2).
pub fn plural_category(locale: &str, n: i64) -> PluralCategory {
    match locale {
        // English CLDR cardinal rule: `one` only for exactly 1.
        "en" => {
            if n == 1 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }
        _ => PluralCategory::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.13.1.1 — ICU pattern validation.
    #[test]
    fn tc_15_13_1_1_icu_pattern_validation() {
        let pattern = "{count, plural, one{# item} other{# items}}";
        assert_eq!(validate_icu_pattern(pattern), Ok(()));
    }

    /// TC-15.13.1.2 — plural category selection for English.
    #[test]
    fn tc_15_13_1_2_plural_category_en_one() {
        assert_eq!(plural_category("en", 1), PluralCategory::One);
    }
}
