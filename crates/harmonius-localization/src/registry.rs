//! In-memory registry of loaded localization tables.

use rkyv::Deserialize;

use crate::{LocError, LocaleId, LocalizationTable};

/// Holds loaded `LocalizationTable` values keyed by `LocaleId`.
#[derive(Clone, Debug, Default)]
pub struct LocaleRegistry {
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
    archived
        .deserialize(&mut rkyv::Infallible)
        .map_err(|_| LocError::InvalidArchive)
}
