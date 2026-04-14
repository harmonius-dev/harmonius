//! Deterministic key–value storage backed by a sorted `Vec`.

/// Sorted vector of `(K, V)` pairs with `O(log n)` lookup.
#[derive(Debug, Clone)]
pub struct SortedVecMap<K, V> {
    pairs: Vec<(K, V)>,
}

impl<K: Ord, V> Default for SortedVecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> SortedVecMap<K, V> {
    /// Builds an empty map.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Inserts or replaces `value` for `key`, returning the previous value when present.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.pairs.binary_search_by_key(&&key, |(k, _)| k) {
            Ok(i) => Some(std::mem::replace(&mut self.pairs[i].1, value)),
            Err(i) => {
                self.pairs.insert(i, (key, value));
                None
            }
        }
    }

    /// Borrows the value for `key`, if any.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.pairs
            .binary_search_by_key(&key, |(k, _)| k)
            .ok()
            .map(|i| &self.pairs[i].1)
    }

    /// Removes `key`, returning the stored value when present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.pairs.binary_search_by_key(&key, |(k, _)| k) {
            Ok(i) => Some(self.pairs.remove(i).1),
            Err(_) => None,
        }
    }

    /// Number of stored pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Returns `true` when empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
