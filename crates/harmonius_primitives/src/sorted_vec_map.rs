//! Deterministic ordered map backed by a sorted vector.

use std::vec::Vec;

/// Sorted vector of `(K, V)` pairs with deterministic iteration order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SortedVecMap<K: Ord, V> {
    pairs: Vec<(K, V)>,
}

impl<K: Ord, V> Default for SortedVecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> SortedVecMap<K, V> {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Inserts `key` mapping to `value`, returning the previous value when present.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.pairs.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(idx) => Some(core::mem::replace(&mut self.pairs[idx].1, value)),
            Err(idx) => {
                self.pairs.insert(idx, (key, value));
                None
            }
        }
    }

    /// Borrows the value for `key`, if present.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.pairs
            .binary_search_by(|(k, _)| k.cmp(key))
            .ok()
            .map(|idx| &self.pairs[idx].1)
    }

    /// Removes `key`, returning the stored value when present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.pairs.binary_search_by(|(k, _)| k.cmp(key)).ok()?;
        Some(self.pairs.remove(idx).1)
    }

    /// Iterates `(key, value)` pairs in ascending key order.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.pairs.iter().map(|(k, v)| (k, v))
    }

    /// Returns the number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Returns `true` when empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
