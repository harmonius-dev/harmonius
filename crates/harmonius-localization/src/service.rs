//! `LocalizationService` — active locale, fallback, and switching.

use std::borrow::Cow;
use std::cell::RefCell;

use crate::missing_log::MissingLog;
use crate::registry::LocaleRegistry;
use crate::search::{binary_search_entry, slice_str};
use crate::{
    KeyEntry, LocError, LocaleId, LocalizationTable, LocalizedStringId, PluralCategory, PluralForm,
};

/// Reason a locale switch was requested (ECS event payload in the full engine).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwitchReason {
    /// Player changed language in settings.
    PlayerSetting,
    /// Narrative or gameplay requested a locale.
    GameEvent,
    /// Editor preview language.
    EditorPreview,
}

/// Request to change the active locale.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocaleSwitch {
    /// Target BCP-47 locale.
    pub target: LocaleId,
    /// Why the switch occurred.
    pub reason: SwitchReason,
}

/// Emitted after a successful switch so UI can re-resolve strings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocaleChanged {
    /// New active locale.
    pub locale: LocaleId,
}

/// Resolves localized ids against baked tables with source fallback.
#[derive(Debug)]
pub struct LocalizationService {
    registry: LocaleRegistry,
    active: LocaleId,
    source: LocaleId,
    missing_log: RefCell<MissingLog>,
}

impl LocalizationService {
    /// Creates a service with `source` as both source and initial active locale id.
    #[must_use]
    pub fn new(source: LocaleId) -> Self {
        Self {
            registry: LocaleRegistry::new(),
            active: source,
            source,
            missing_log: RefCell::new(MissingLog::new()),
        }
    }

    /// Loads or replaces `locale` from an rkyv archive.
    pub fn load(&mut self, locale: LocaleId, bytes: &[u8]) -> Result<(), LocError> {
        self.registry.load_bytes(locale, bytes)
    }

    /// Removes `locale` from the registry.
    pub fn unload(&mut self, locale: LocaleId) {
        self.registry.remove(locale);
    }

    /// Sets the active locale; returns `LocError::NotLoaded` if the table is absent.
    pub fn set_active(&mut self, locale: LocaleId) -> Result<(), LocError> {
        if self.registry.table(locale).is_none() {
            return Err(LocError::NotLoaded);
        }
        self.active = locale;
        Ok(())
    }

    /// Current active locale id (may be unloaded — lookup still consults source).
    #[must_use]
    pub fn active_locale(&self) -> LocaleId {
        self.active
    }

    /// Applies a locale switch; returns `LocaleChanged` when the active locale updates.
    pub fn apply_locale_switch(
        &mut self,
        switch: LocaleSwitch,
    ) -> Result<Option<LocaleChanged>, LocError> {
        if self.registry.table(switch.target).is_none() {
            return Err(LocError::NotLoaded);
        }
        let prev = self.active;
        self.active = switch.target;
        if prev == switch.target {
            Ok(None)
        } else {
            Ok(Some(LocaleChanged {
                locale: switch.target,
            }))
        }
    }

    /// Resolves `id` for the active locale with source fallback.
    #[must_use]
    pub fn lookup(&self, id: LocalizedStringId) -> Cow<'_, str> {
        self.lookup_inner(id, None)
    }

    /// Resolves plural-sensitive text for count `n` (English plural rules).
    #[must_use]
    pub fn lookup_plural(&self, id: LocalizedStringId, n: u64) -> Cow<'_, str> {
        self.lookup_inner(id, Some(n))
    }

    fn lookup_inner(&self, id: LocalizedStringId, n: Option<u64>) -> Cow<'_, str> {
        let active_loaded = self.registry.table(self.active).is_some();

        if let Some(table) = self.registry.table(self.active) {
            if let Some(text) = Self::resolve_in_table(table, id, n) {
                return Cow::Borrowed(text);
            }
        }

        if let Some(table) = self.registry.table(self.source) {
            if let Some(text) = Self::resolve_in_table(table, id, n) {
                if active_loaded {
                    self.missing_log.borrow_mut().note(self.active, id);
                }
                return Cow::Borrowed(text);
            }
        }

        Cow::Owned(format!("[missing:{}]", id.0))
    }

    fn resolve_in_table(
        table: &LocalizationTable,
        id: LocalizedStringId,
        n: Option<u64>,
    ) -> Option<&str> {
        let idx = binary_search_entry(&table.keys, id)?;
        let entry = &table.keys[idx];
        Self::text_for_entry(table, entry, n)
    }

    fn text_for_entry<'a>(
        table: &'a LocalizationTable,
        entry: &'a KeyEntry,
        n: Option<u64>,
    ) -> Option<&'a str> {
        match &entry.plural_form {
            PluralForm::Singular => slice_str(&table.strings, entry.offset, entry.length),
            PluralForm::Plural(rules) => {
                let n = n.unwrap_or(1);
                let cat = english_plural_category(n);
                let rule = rules.iter().find(|r| r.category == cat)?;
                slice_str(&table.strings, rule.offset, rule.length)
            }
        }
    }

    /// Removes and returns deduped missing entries.
    pub fn drain_missing(&mut self) -> Vec<crate::MissingEntry> {
        let mut log = self.missing_log.borrow_mut();
        log.drain()
    }
}

fn english_plural_category(n: u64) -> PluralCategory {
    if n == 1 {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}
