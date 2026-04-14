//! In-memory registry of loaded localization tables.
//!
//! Loads validate archives then deserialize into owned tables. A future mmap /
//! zero-copy path can retain `Archived<LocalizationTable>` once buffer alignment
//! and editor pipeline integration are wired.

use rkyv::Deserialize;

use crate::{LocError, LocaleId, LocalizationTable};

/// Holds loaded `LocalizationTable` values keyed by `LocaleId`.
#[derive(Clone, Debug, Default)]
pub(crate) struct LocaleRegistry {
    loaded: Vec<(LocaleId, LocalizationTable)>,
}

impl LocaleRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self { loaded: Vec::new() }
    }

    /// Inserts or replaces a table for `locale`.
    pub fn insert(&mut self, locale: LocaleId, table: LocalizationTable) {
        if let Some(i) = self.loaded.iter().position(|(l, _)| *l == locale) {
            self.loaded[i] = (locale, table);
        } else {
            self.loaded.push((locale, table));
        }
    }

    /// Removes a locale if present.
    pub fn remove(&mut self, locale: LocaleId) {
        if let Some(i) = self.loaded.iter().position(|(l, _)| *l == locale) {
            self.loaded.swap_remove(i);
        }
    }

    /// Borrows a table for `locale` when loaded.
    #[must_use]
    pub fn table(&self, locale: LocaleId) -> Option<&LocalizationTable> {
        self.loaded
            .iter()
            .find_map(|(l, t)| (*l == locale).then_some(t))
    }

    /// Decodes an rkyv archive into a table and registers it.
    pub fn load_bytes(&mut self, locale: LocaleId, bytes: &[u8]) -> Result<(), LocError> {
        let table = decode_table(bytes)?;
        self.insert(locale, table);
        Ok(())
    }
}

fn decode_table(bytes: &[u8]) -> Result<LocalizationTable, LocError> {
    let archived = rkyv::check_archived_root::<LocalizationTable>(bytes)
        .map_err(|_| LocError::InvalidArchive)?;
    let table: LocalizationTable = archived
        .deserialize(&mut rkyv::Infallible)
        .map_err(|_| LocError::InvalidArchive)?;
    validate_key_order(&table)?;
    Ok(table)
}

fn validate_key_order(table: &LocalizationTable) -> Result<(), LocError> {
    for w in table.keys.windows(2) {
        if w[0].id.0 >= w[1].id.0 {
            return Err(LocError::InvalidArchive);
        }
    }
    Ok(())
}
