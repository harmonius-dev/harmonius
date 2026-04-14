use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

/// Sorted association list backed by a `Vec` (O(log n) lookup, no `HashMap` on hot paths).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SortedVecMap<K: Ord, V> {
    entries: Vec<(K, V)>,
}

impl<K: Ord, V> SortedVecMap<K, V> {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Number of stored pairs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clears all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Binary-search lookup by key.
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.position(key).map(|idx| &self.entries[idx].1)
    }

    /// Mutable lookup by key.
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.position(key).map(|idx| &mut self.entries[idx].1)
    }

    /// Inserts or replaces a value for `key`.
    pub fn insert(&mut self, key: K, value: V) {
        match self.position(&key) {
            Some(idx) => self.entries[idx].1 = value,
            None => {
                let idx = self.lower_bound(&key);
                self.entries.insert(idx, (key, value));
            }
        }
    }

    /// Removes a key when present.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.position(key).map(|idx| self.entries.remove(idx).1)
    }

    /// Iterator over keys.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(k, _)| k)
    }

    /// Iterator over values.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }

    /// Iterator over key/value pairs in key order.
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.entries.iter()
    }

    fn position<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.entries
            .binary_search_by(|probe| probe.0.borrow().cmp(key))
            .ok()
    }

    fn lower_bound<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut lo = 0usize;
        let mut hi = self.entries.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.entries[mid].0.borrow().cmp(key) {
                Ordering::Less => lo = mid + 1,
                Ordering::Equal => return mid,
                Ordering::Greater => hi = mid,
            }
        }
        lo
    }
}

impl<K: Ord, V> Index<usize> for SortedVecMap<K, V> {
    type Output = (K, V);

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<K: Ord, V> IndexMut<usize> for SortedVecMap<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}
