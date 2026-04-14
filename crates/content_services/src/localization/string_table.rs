//! Authoring-time string table with per-key locks.

use std::collections::HashMap;

/// Stable string identifier inside a localization bundle.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StringKey(pub String);

/// Lock or merge failures when mutating translations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocaleError {
    /// Key is locked by an external TMS workflow (TC-15.13.1.4).
    Locked,
}

/// Translation lifecycle inside the editor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransUnitState {
    /// Awaiting translator work.
    Initial,
    /// Translator supplied text.
    Translated,
}

/// One logical row in the string table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringEntry {
    /// Default (usually English) source text.
    pub source: String,
    /// Locale id → translated text.
    pub translations: HashMap<String, String>,
    /// Workflow state for export/import tooling.
    pub state: TransUnitState,
    /// External lock flag.
    pub locked: bool,
}

/// Hash map backed authoring model.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StringTableModel {
    rows: HashMap<StringKey, StringEntry>,
}

impl StringTableModel {
    /// Empty model.
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrows all rows for import/export helpers.
    pub fn rows(&self) -> &HashMap<StringKey, StringEntry> {
        &self.rows
    }

    /// Mutable access to rows for merge tooling.
    pub fn rows_mut(&mut self) -> &mut HashMap<StringKey, StringEntry> {
        &mut self.rows
    }

    /// Inserts or replaces a row keyed by `key`.
    pub fn insert_entry(&mut self, key: StringKey, entry: StringEntry) {
        self.rows.insert(key, entry);
    }

    /// Reads a row when present.
    pub fn get(&self, key: &StringKey) -> Option<&StringEntry> {
        self.rows.get(key)
    }

    /// Writes `text` for `locale`, honoring external locks.
    pub fn set_translation(
        &mut self,
        key: &StringKey,
        locale: &str,
        text: String,
    ) -> Result<(), LocaleError> {
        let entry = self.rows.get_mut(key).ok_or(LocaleError::Locked)?;
        if entry.locked {
            return Err(LocaleError::Locked);
        }
        entry.translations.insert(locale.to_string(), text);
        entry.state = TransUnitState::Translated;
        Ok(())
    }

    /// Locks `key` for TMS workflows.
    pub fn lock_key(&mut self, key: &StringKey) {
        if let Some(e) = self.rows.get_mut(key) {
            e.locked = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.13.1.4 — locked keys reject translation edits.
    #[test]
    fn tc_15_13_1_4_string_entry_lock_unlock() {
        let key = StringKey("ui.save".to_string());
        let mut model = StringTableModel::new();
        model.insert_entry(
            key.clone(),
            StringEntry {
                source: "Save".to_string(),
                translations: HashMap::new(),
                state: TransUnitState::Initial,
                locked: false,
            },
        );
        model.lock_key(&key);
        let result = model.set_translation(&key, "en", "Save".to_string());
        assert_eq!(result, Err(LocaleError::Locked));
    }
}
