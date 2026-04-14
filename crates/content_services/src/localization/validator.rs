//! Locale validation categories for missing translations.

use std::collections::HashSet;

/// High-level validation bucket returned by `LocaleValidator`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationCategory {
    /// No localized string exists for the requested locale.
    MissingTranslation,
    /// Reserved categories for future authoring checks.
    TextOverflow,
}

/// Validates string table coverage for requested locales.
#[derive(Clone, Debug, Default)]
pub struct LocaleValidator;

impl LocaleValidator {
    /// Returns `MissingTranslation` when `locale` is absent from `available`.
    pub fn validate(
        &self,
        locale: &str,
        available: &HashSet<String>,
    ) -> Result<(), ValidationCategory> {
        if !available.contains(locale) {
            return Err(ValidationCategory::MissingTranslation);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.13.2.3 — validating `ar` against English-only data flags missing work.
    #[test]
    fn tc_15_13_2_3_missing_translation_detect() {
        let mut available = HashSet::new();
        available.insert("en".to_string());
        let validator = LocaleValidator;
        let result = validator.validate("ar", &available);
        assert_eq!(result, Err(ValidationCategory::MissingTranslation));
    }
}
