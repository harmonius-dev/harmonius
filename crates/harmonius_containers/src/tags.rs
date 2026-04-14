//! Deterministic tag sets for constraints and sockets.

use std::collections::BTreeSet;

/// Sorted tag set for deterministic comparisons and hashing.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TagSet(pub BTreeSet<String>);

impl TagSet {
    /// Builds a tag set from unsorted inputs (stored sorted).
    #[must_use]
    pub fn new(tags: impl IntoIterator<Item = String>) -> Self {
        Self(tags.into_iter().collect())
    }

    /// Returns true when every required tag is present.
    #[must_use]
    pub fn contains_all(&self, required: &[String]) -> bool {
        required.iter().all(|tag| self.0.contains(tag))
    }

    /// Converts to a vector for error payloads.
    #[must_use]
    pub fn to_vec(&self) -> Vec<String> {
        self.0.iter().cloned().collect()
    }
}

impl FromIterator<String> for TagSet {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
