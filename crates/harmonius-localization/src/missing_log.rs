//! Deduplicated missing-translation log.

use crate::{LocaleId, LocalizedStringId};

/// One `(locale, id)` pair recorded when falling back to the source table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingEntry {
    /// Locale that was missing the string.
    pub locale: LocaleId,
    /// Missing id.
    pub id: LocalizedStringId,
}

/// Dedupes missing `(active locale, id)` pairs for diagnostics.
#[derive(Clone, Debug, Default)]
pub struct MissingLog {
    seen: Vec<(LocaleId, LocalizedStringId)>,
}

impl MissingLog {
    /// Creates an empty log.
    #[must_use]
    pub fn new() -> Self {
        Self { seen: Vec::new() }
    }

    /// Records a missing translation if this pair was not seen before.
    pub fn note(&mut self, locale: LocaleId, id: LocalizedStringId) {
        if !self
            .seen
            .iter()
            .any(|&(loc, sid)| loc == locale && sid == id)
        {
            self.seen.push((locale, id));
        }
    }

    /// Drains all entries and clears the log.
    pub fn drain(&mut self) -> Vec<MissingEntry> {
        let out: Vec<MissingEntry> = self
            .seen
            .iter()
            .map(|&(locale, id)| MissingEntry { locale, id })
            .collect();
        self.seen.clear();
        out
    }
}
