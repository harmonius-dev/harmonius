//! Deduplicated missing-translation log.

use std::collections::HashSet;

use tracing::warn;

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
    seen: HashSet<(LocaleId, LocalizedStringId)>,
}

impl MissingLog {
    /// Creates an empty log.
    #[must_use]
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    /// Records a missing translation if this pair was not seen before.
    ///
    /// Emits one `tracing::warn!` the first time each `(locale, id)` is recorded.
    pub fn note(&mut self, locale: LocaleId, id: LocalizedStringId) {
        if self.seen.insert((locale, id)) {
            warn!(
                target: "harmonius_localization",
                id = id.0,
                ?locale,
                "missing translation"
            );
        }
    }

    /// Drains all entries and clears the log.
    pub fn drain(&mut self) -> Vec<MissingEntry> {
        let out: Vec<MissingEntry> = self
            .seen
            .drain()
            .map(|(locale, id)| MissingEntry { locale, id })
            .collect();
        out
    }
}
