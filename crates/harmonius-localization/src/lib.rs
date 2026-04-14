//! Localization runtime: `LocalizationService`, baked `LocalizationTable` archives, and locale
//! switching primitives. See `docs/design/game-framework/localization.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod error;
mod missing_log;
mod registry;
mod search;
mod service;

/// Returned by [`LocalizationService::lookup`] when no table row resolves the id.
pub const MISSING_TRANSLATION: &str = "[missing]";

pub use error::LocError;
pub use missing_log::{MissingEntry, MissingLog};
pub use service::{LocaleChanged, LocaleSwitch, LocalizationService, SwitchReason};

pub use crate::types::{
    KeyEntry, LocaleId, LocalizationTable, LocalizedStringId, PluralCategory, PluralForm,
    PluralRule,
};

mod types {
    use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize};

    /// Stable string identifier assigned at bake time.
    #[derive(
        Archive,
        Serialize,
        RkyvDeserialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[archive(check_bytes)]
    #[archive_attr(derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
    pub struct LocalizedStringId(pub u32);

    /// Fixed-width BCP-47 tag (UTF-8, zero padded).
    #[derive(
        Archive,
        Serialize,
        RkyvDeserialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[archive(check_bytes)]
    #[archive_attr(derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd))]
    pub struct LocaleId(pub [u8; 16]);

    impl LocaleId {
        /// Packs a BCP-47 tag into 16 UTF-8 bytes (silently truncates when longer).
        #[must_use]
        pub fn from_bcp47(s: &str) -> Self {
            let mut buf = [0u8; 16];
            let b = s.as_bytes();
            let n = b.len().min(16);
            buf[..n].copy_from_slice(&b[..n]);
            Self(buf)
        }

        /// Returns the tag as UTF-8 (trimmed at the first zero byte).
        ///
        /// Invalid UTF-8 in the stored bytes becomes `""`.
        #[must_use]
        pub fn as_str(&self) -> &str {
            let end = self.0.iter().position(|&x| x == 0).unwrap_or(16);
            core::str::from_utf8(&self.0[..end]).unwrap_or("")
        }
    }

    /// CLDR-style plural category for rule selection.
    #[derive(
        Archive,
        Serialize,
        RkyvDeserialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        PartialOrd,
        Ord,
    )]
    #[archive(check_bytes)]
    #[archive_attr(derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord))]
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

    /// One plural variant mapping a category to a UTF-8 span in `LocalizationTable::strings`.
    #[derive(Archive, Serialize, RkyvDeserialize, Clone, Debug, Eq, PartialEq)]
    #[archive(check_bytes)]
    #[archive_attr(derive(Debug, Eq, PartialEq))]
    pub struct PluralRule {
        /// Selected category for this rule.
        pub category: PluralCategory,
        /// Byte offset in `strings`.
        pub offset: u32,
        /// Byte length in `strings`.
        pub length: u32,
    }

    /// Singular string or plural rule set stored in the baked table.
    #[derive(Archive, Serialize, RkyvDeserialize, Clone, Debug, Eq, PartialEq)]
    #[archive(check_bytes)]
    #[archive_attr(derive(Debug, Eq, PartialEq))]
    pub enum PluralForm {
        /// One span (`KeyEntry::{offset,length}`) is used.
        Singular,
        /// Multiple spans keyed by plural category.
        Plural(Vec<PluralRule>),
    }

    /// One logical string row in a baked table.
    #[derive(Archive, Serialize, RkyvDeserialize, Clone, Debug, Eq, PartialEq)]
    #[archive(check_bytes)]
    #[archive_attr(derive(Debug, Eq, PartialEq))]
    pub struct KeyEntry {
        /// Stable id.
        pub id: LocalizedStringId,
        /// Primary UTF-8 span in `strings` (singular or default plural).
        pub offset: u32,
        /// Byte length in `strings`.
        pub length: u32,
        /// Plural metadata for `lookup_plural`.
        pub plural_form: PluralForm,
    }

    /// Baked localization table for one locale (rkyv archive payload).
    #[derive(Archive, Serialize, RkyvDeserialize, Clone, Debug, Eq, PartialEq)]
    #[archive(check_bytes)]
    #[archive_attr(derive(Debug, Eq, PartialEq))]
    pub struct LocalizationTable {
        /// Locale of this table.
        pub locale: LocaleId,
        /// Source locale used for fallback when this table is active.
        pub source_locale: LocaleId,
        /// Sorted by `LocalizedStringId` ascending (bake invariant).
        pub keys: Vec<KeyEntry>,
        /// UTF-8 blob referenced by `KeyEntry` spans.
        pub strings: String,
        /// Monotonic bake version for hot reload.
        pub version: u32,
    }
}

#[cfg(test)]
mod tests {
    use rkyv::Deserialize as RkyvDeserializeTrait;
    use tracing_test::traced_test;

    use super::*;
    use crate::search::{binary_search_call_count, reset_binary_search_call_count};

    fn en() -> LocaleId {
        LocaleId::from_bcp47("en")
    }

    fn ja() -> LocaleId {
        LocaleId::from_bcp47("ja")
    }

    fn fr() -> LocaleId {
        LocaleId::from_bcp47("fr")
    }

    fn archive(table: &LocalizationTable) -> Vec<u8> {
        rkyv::to_bytes::<_, 256>(table).unwrap().into()
    }

    fn simple_table(locale: LocaleId, source: LocaleId, id: u32, text: &str) -> LocalizationTable {
        let strings = text.to_string();
        LocalizationTable {
            locale,
            source_locale: source,
            keys: vec![KeyEntry {
                id: LocalizedStringId(id),
                offset: 0,
                length: strings.len() as u32,
                plural_form: PluralForm::Singular,
            }],
            strings,
            version: 1,
        }
    }

    /// **TC-10.1.9.1** `test_lookup_returns_active_locale_string`
    #[test]
    fn test_lookup_returns_active_locale_string() {
        let en_t = simple_table(en(), en(), 1, "hello");
        let ja_t = simple_table(ja(), en(), 1, "konnichiwa");
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        svc.set_active(ja()).unwrap();
        assert_eq!(svc.lookup(LocalizedStringId(1)), "konnichiwa");
    }

    /// **TC-10.1.9.2** `test_lookup_missing_key_returns_source`
    #[test]
    fn test_lookup_missing_key_returns_source() {
        let en_t = simple_table(en(), en(), 1, "only-en");
        let ja_t = {
            let mut t = simple_table(ja(), en(), 99, "orphan");
            t.keys.clear();
            t.strings.clear();
            t
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        svc.set_active(ja()).unwrap();
        assert_eq!(svc.lookup(LocalizedStringId(1)), "only-en");
    }

    /// **TC-10.1.9.3** `test_lookup_missing_in_source_returns_placeholder`
    #[traced_test]
    #[test]
    fn test_lookup_missing_in_source_returns_placeholder() {
        let en_t = {
            let mut t = simple_table(en(), en(), 1, "a");
            t.keys.clear();
            t.strings.clear();
            t
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        assert_eq!(svc.lookup(LocalizedStringId(42)).as_ref(), MISSING_TRANSLATION);
        assert_eq!(svc.lookup_or_key(LocalizedStringId(42)).as_ref(), "42");
        assert!(logs_contain("missing translation"));
    }

    /// **TC-10.1.9.4** `test_active_locale_reported`
    #[test]
    fn test_active_locale_reported() {
        let en_t = simple_table(en(), en(), 1, "a");
        let ja_t = simple_table(ja(), en(), 1, "b");
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        svc.set_active(ja()).unwrap();
        assert_eq!(svc.active_locale(), ja());
    }

    /// **TC-10.1.9.5** `test_binary_search_on_sorted_keys`
    #[test]
    fn test_binary_search_on_sorted_keys() {
        let n = 1024u32;
        let mut keys = Vec::new();
        let mut strings = String::new();
        for i in 0..n {
            let id = LocalizedStringId(i + 1);
            let fragment = format!("v{i}");
            let off = strings.len() as u32;
            strings.push_str(&fragment);
            keys.push(KeyEntry {
                id,
                offset: off,
                length: fragment.len() as u32,
                plural_form: PluralForm::Singular,
            });
        }
        let table = LocalizationTable {
            locale: en(),
            source_locale: en(),
            keys,
            strings,
            version: 1,
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&table)).unwrap();
        reset_binary_search_call_count();
        let _ = svc.lookup(LocalizedStringId(n));
        let calls = binary_search_call_count();
        let max_expected = (n as f32).log2().ceil() as u32 + 2;
        assert!(calls <= max_expected, "calls={calls} max={max_expected}");
    }

    /// **TC-10.1.9.6** `test_plural_selection_one`
    #[test]
    fn test_plural_selection_one() {
        let mut strings = String::new();
        strings.push_str("one person");
        let off_one = 0u32;
        let len_one = strings.len() as u32;
        strings.push_str("many people");
        let off_other = len_one;
        let len_other = strings.len() as u32 - off_other;
        let rules = vec![
            PluralRule {
                category: PluralCategory::One,
                offset: off_one,
                length: len_one,
            },
            PluralRule {
                category: PluralCategory::Other,
                offset: off_other,
                length: len_other,
            },
        ];
        let table = LocalizationTable {
            locale: en(),
            source_locale: en(),
            keys: vec![KeyEntry {
                id: LocalizedStringId(1),
                offset: 0,
                length: 0,
                plural_form: PluralForm::Plural(rules),
            }],
            strings,
            version: 1,
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&table)).unwrap();
        assert_eq!(svc.lookup_plural(LocalizedStringId(1), 1), "one person");
    }

    /// **TC-10.1.9.7** `test_plural_selection_many`
    #[test]
    fn test_plural_selection_many() {
        let mut strings = String::new();
        strings.push_str("one person");
        let off_one = 0u32;
        let len_one = strings.len() as u32;
        strings.push_str("many people");
        let off_other = len_one;
        let len_other = strings.len() as u32 - off_other;
        let rules = vec![
            PluralRule {
                category: PluralCategory::One,
                offset: off_one,
                length: len_one,
            },
            PluralRule {
                category: PluralCategory::Other,
                offset: off_other,
                length: len_other,
            },
        ];
        let table = LocalizationTable {
            locale: en(),
            source_locale: en(),
            keys: vec![KeyEntry {
                id: LocalizedStringId(1),
                offset: 0,
                length: 0,
                plural_form: PluralForm::Plural(rules),
            }],
            strings,
            version: 1,
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&table)).unwrap();
        assert_eq!(svc.lookup_plural(LocalizedStringId(1), 5), "many people");
    }

    /// **TC-10.1.9.8** `test_missing_log_dedupes_per_pair`
    #[test]
    fn test_missing_log_dedupes_per_pair() {
        let en_t = simple_table(en(), en(), 1, "en");
        let ja_t = {
            let mut t = simple_table(ja(), en(), 99, "x");
            t.keys.clear();
            t.strings.clear();
            t
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        svc.set_active(ja()).unwrap();
        let _ = svc.lookup(LocalizedStringId(1));
        let _ = svc.lookup(LocalizedStringId(1));
        let drained = svc.drain_missing();
        assert_eq!(drained.len(), 1);
    }

    /// **TC-10.1.9.9** `test_missing_log_drain_returns_entries`
    #[test]
    fn test_missing_log_drain_returns_entries() {
        let en_t = simple_table(en(), en(), 1, "en");
        let ja_t = {
            let mut t = simple_table(ja(), en(), 99, "x");
            t.keys.clear();
            t.strings.clear();
            t
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        svc.set_active(ja()).unwrap();
        let _ = svc.lookup(LocalizedStringId(1));
        let mut first = svc.drain_missing();
        assert_eq!(first.len(), 1);
        first.clear();
        let second = svc.drain_missing();
        assert!(second.is_empty());
    }

    /// **TC-15.13.1.1** `test_table_archive_roundtrip`
    #[test]
    fn test_table_archive_roundtrip() {
        let t = simple_table(en(), en(), 7, "roundtrip");
        let bytes = archive(&t);
        let archived = rkyv::check_archived_root::<LocalizationTable>(&bytes).unwrap();
        let back: LocalizationTable =
            RkyvDeserializeTrait::deserialize(archived, &mut rkyv::Infallible).unwrap();
        assert_eq!(t, back);
    }

    /// **TC-15.13.1.2** `test_table_keys_sorted_ascending`
    #[test]
    fn test_table_keys_sorted_ascending() {
        let strings = "ab".to_string();
        let t = LocalizationTable {
            locale: en(),
            source_locale: en(),
            keys: vec![
                KeyEntry {
                    id: LocalizedStringId(1),
                    offset: 0,
                    length: 1,
                    plural_form: PluralForm::Singular,
                },
                KeyEntry {
                    id: LocalizedStringId(2),
                    offset: 1,
                    length: 1,
                    plural_form: PluralForm::Singular,
                },
            ],
            strings,
            version: 1,
        };
        let ids: Vec<u32> = t.keys.iter().map(|k| k.id.0).collect();
        for w in ids.windows(2) {
            assert!(w[0] < w[1]);
        }
    }

    /// **TC-15.13.1.3** `test_table_key_entries_unique`
    #[test]
    fn test_table_key_entries_unique() {
        let t = simple_table(en(), en(), 1, "a");
        let mut seen = std::collections::HashSet::new();
        for k in &t.keys {
            assert!(seen.insert(k.id.0));
        }
    }

    /// **TC-15.13.1.4** `test_bake_deterministic_across_platforms`
    #[test]
    fn test_bake_deterministic_across_platforms() {
        let t = simple_table(en(), en(), 3, "deterministic");
        let a = archive(&t);
        let b = archive(&t);
        assert_eq!(a, b);
    }

    /// **TC-15.13.2.1** `test_locale_switch_event_changes_active`
    #[test]
    fn test_locale_switch_event_changes_active() {
        let en_t = simple_table(en(), en(), 1, "a");
        let ja_t = simple_table(ja(), en(), 1, "b");
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        let sw = LocaleSwitch {
            target: ja(),
            reason: SwitchReason::PlayerSetting,
        };
        svc.apply_locale_switch(sw).unwrap();
        assert_eq!(svc.active_locale(), ja());
    }

    /// **TC-15.13.2.2** `test_locale_switch_emits_locale_changed`
    #[test]
    fn test_locale_switch_emits_locale_changed() {
        let en_t = simple_table(en(), en(), 1, "a");
        let ja_t = simple_table(ja(), en(), 1, "b");
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        let note = svc
            .apply_locale_switch(LocaleSwitch {
                target: ja(),
                reason: SwitchReason::GameEvent,
            })
            .unwrap();
        assert_eq!(note, Some(LocaleChanged { locale: ja() }));
    }

    /// **TC-15.13.2.3** `test_locale_switch_unknown_locale_errors`
    #[test]
    fn test_locale_switch_unknown_locale_errors() {
        let en_t = simple_table(en(), en(), 1, "a");
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        let err = svc
            .apply_locale_switch(LocaleSwitch {
                target: fr(),
                reason: SwitchReason::PlayerSetting,
            })
            .unwrap_err();
        assert_eq!(err, LocError::NotLoaded);
    }

    /// **TC-15.13.3.1** `test_fallback_to_source_logs_warning_once`
    #[traced_test]
    #[test]
    fn test_fallback_to_source_logs_warning_once() {
        let en_t = simple_table(en(), en(), 1, "src");
        let ja_t = {
            let mut t = simple_table(ja(), en(), 99, "x");
            t.keys.clear();
            t.strings.clear();
            t
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        svc.set_active(ja()).unwrap();
        let _ = svc.lookup(LocalizedStringId(1));
        let _ = svc.lookup(LocalizedStringId(1));
        assert_eq!(svc.drain_missing().len(), 1);
        assert!(logs_contain("missing translation"));
    }

    /// **TC-15.13.3.2** `test_fallback_returns_correct_source_text`
    #[test]
    fn test_fallback_returns_correct_source_text() {
        let en_t = simple_table(en(), en(), 1, "source-text");
        let ja_t = {
            let mut t = simple_table(ja(), en(), 99, "x");
            t.keys.clear();
            t.strings.clear();
            t
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        svc.set_active(ja()).unwrap();
        assert_eq!(svc.lookup(LocalizedStringId(1)), "source-text");
    }

    /// **TC-15.13.3.3** `test_placeholder_when_neither_has_key`
    #[traced_test]
    #[test]
    fn test_placeholder_when_neither_has_key() {
        let empty = LocalizationTable {
            locale: en(),
            source_locale: en(),
            keys: vec![],
            strings: String::new(),
            version: 1,
        };
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&empty)).unwrap();
        svc.set_active(en()).unwrap();
        assert_eq!(svc.lookup(LocalizedStringId(9)).as_ref(), MISSING_TRANSLATION);
        assert_eq!(svc.lookup_or_key(LocalizedStringId(9)).as_ref(), "9");
        assert!(logs_contain("missing translation"));
    }

    #[test]
    fn test_load_rejects_unsorted_key_ids() {
        let strings = "ab".to_string();
        let t = LocalizationTable {
            locale: en(),
            source_locale: en(),
            keys: vec![
                KeyEntry {
                    id: LocalizedStringId(2),
                    offset: 0,
                    length: 1,
                    plural_form: PluralForm::Singular,
                },
                KeyEntry {
                    id: LocalizedStringId(1),
                    offset: 1,
                    length: 1,
                    plural_form: PluralForm::Singular,
                },
            ],
            strings,
            version: 1,
        };
        let mut svc = LocalizationService::new(en());
        assert_eq!(
            svc.load(en(), archive(&t)).unwrap_err(),
            LocError::InvalidArchive
        );
    }

    #[test]
    fn test_load_rejects_duplicate_key_ids() {
        let strings = "aa".to_string();
        let t = LocalizationTable {
            locale: en(),
            source_locale: en(),
            keys: vec![
                KeyEntry {
                    id: LocalizedStringId(1),
                    offset: 0,
                    length: 1,
                    plural_form: PluralForm::Singular,
                },
                KeyEntry {
                    id: LocalizedStringId(1),
                    offset: 1,
                    length: 1,
                    plural_form: PluralForm::Singular,
                },
            ],
            strings,
            version: 1,
        };
        let mut svc = LocalizationService::new(en());
        assert_eq!(
            svc.load(en(), archive(&t)).unwrap_err(),
            LocError::InvalidArchive
        );
    }

    #[test]
    fn test_handle_locale_switch_alias_matches_apply() {
        let en_t = simple_table(en(), en(), 1, "a");
        let ja_t = simple_table(ja(), en(), 1, "b");
        let mut svc = LocalizationService::new(en());
        svc.load(en(), archive(&en_t)).unwrap();
        svc.load(ja(), archive(&ja_t)).unwrap();
        let a = svc
            .apply_locale_switch(LocaleSwitch {
                target: ja(),
                reason: SwitchReason::PlayerSetting,
            })
            .unwrap();
        let _ = svc
            .apply_locale_switch(LocaleSwitch {
                target: en(),
                reason: SwitchReason::PlayerSetting,
            })
            .unwrap();
        let b = svc
            .handle_locale_switch(LocaleSwitch {
                target: ja(),
                reason: SwitchReason::PlayerSetting,
            })
            .unwrap();
        assert_eq!(a, b);
    }
}
