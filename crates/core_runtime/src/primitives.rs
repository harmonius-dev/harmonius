//! Small deterministic containers used by the I/O dispatcher.

use std::marker::PhantomData;

/// Opaque generational handle into a typed pool slot.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    pub(crate) fn new(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            _marker: PhantomData,
        }
    }

    /// Slot index inside the owning pool.
    #[must_use]
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Generation counter used to detect stale handles after reuse.
    #[must_use]
    pub fn generation(&self) -> u32 {
        self.generation
    }
}

/// Insert-ordered map keyed by `K` that keeps entries sorted by key for deterministic iteration.
#[derive(Debug)]
pub struct SortedVecMap<K: Ord + Copy, V> {
    entries: Vec<(K, V)>,
}

impl<K: Ord + Copy, V> Default for SortedVecMap<K, V> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl<K: Ord + Copy, V> SortedVecMap<K, V> {
    /// Returns the number of stored entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when no entries are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Borrows the value for `key`, if present.
    #[must_use]
    pub fn get(&self, key: K) -> Option<&V> {
        let idx = self.position(key)?;
        Some(&self.entries[idx].1)
    }

    /// Inserts or replaces `key` while preserving sorted order.
    pub fn insert(&mut self, key: K, value: V) {
        match self.entries.binary_search_by_key(&key, |&(k, _)| k) {
            Ok(idx) => self.entries[idx].1 = value,
            Err(idx) => self.entries.insert(idx, (key, value)),
        }
    }

    /// Removes `key` and returns the stored value, if any.
    pub fn remove(&mut self, key: K) -> Option<V> {
        let idx = self.position(key)?;
        Some(self.entries.remove(idx).1)
    }

    /// Iterator over keys in ascending sorted order.
    pub fn keys(&self) -> impl Iterator<Item = K> + '_ {
        self.entries.iter().map(|&(k, _)| k)
    }

    fn position(&self, key: K) -> Option<usize> {
        self.entries.binary_search_by_key(&key, |&(k, _)| k).ok()
    }
}

impl<K: Ord + Copy, V> SortedVecMap<K, V> {
    /// Returns `true` if keys are strictly ascending (used by tests for determinism checks).
    #[must_use]
    pub fn keys_are_sorted(&self) -> bool {
        self.entries.windows(2).all(|w| w[0].0 < w[1].0)
    }
}
