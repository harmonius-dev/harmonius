//! Deterministic ordered map (binary search on sorted `Vec`) for SC-2-style harness data.

/// Sorted key-value map backed by a `Vec` (no `HashMap` on integration hot paths).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SortedVecMap<K: Ord, V> {
    inner: Vec<(K, V)>,
}

impl<K: Ord, V> SortedVecMap<K, V> {
    /// Borrows the value for `key`, if present.
    pub fn get(&self, key: &K) -> Option<&V> {
        let i = self.inner.binary_search_by(|(k, _)| k.cmp(key)).ok()?;
        Some(&self.inner[i].1)
    }

    /// Mutable borrow for `key`, if present.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let i = self.inner.binary_search_by(|(k, _)| k.cmp(key)).ok()?;
        Some(&mut self.inner[i].1)
    }

    /// Inserts `default` when missing, then returns a mutable reference to the value.
    pub fn entry_or_default(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        match self.inner.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => &mut self.inner[i].1,
            Err(i) => {
                self.inner.insert(i, (key, V::default()));
                &mut self.inner[i].1
            }
        }
    }

    /// Removes and returns the value for `key`, if present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let i = self.inner.binary_search_by(|(k, _)| k.cmp(key)).ok()?;
        Some(self.inner.remove(i).1)
    }
}
