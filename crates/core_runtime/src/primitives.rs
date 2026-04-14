//! Small stack-friendly containers used by the ConVar registry.

/// Growable list tuned for tiny inline counts used in hot paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmallVec<T, const N: usize> {
    items: Vec<T>,
}

impl<T, const N: usize> SmallVec<T, N> {
    /// Builds an empty vector.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Returns the number of stored elements.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` when no elements are stored.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Appends an element.
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Removes every element without shrinking capacity.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Borrows the backing slice.
    pub fn as_slice(&self) -> &[T] {
        self.items.as_slice()
    }

    /// Iterates immutably over stored elements.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    /// Moves all elements out, leaving the vector empty.
    pub fn drain_all(&mut self) -> Vec<T> {
        std::mem::take(&mut self.items)
    }
}

impl<T, const N: usize> Default for SmallVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> IntoIterator for SmallVec<T, N> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SmallVec<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

/// Key–value map sorted by `Ord` on `K` (backed by a B-tree).
#[derive(Clone, Debug)]
pub struct SortedVecMap<K, V> {
    inner: std::collections::BTreeMap<K, V>,
}

impl<K: Ord, V> SortedVecMap<K, V> {
    /// Builds an empty map.
    pub fn new() -> Self {
        Self {
            inner: std::collections::BTreeMap::new(),
        }
    }

    /// Inserts a value, returning the previous entry when present.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// Borrows the value for `key` when it exists.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    /// Mutably borrows the value for `key` when it exists.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    /// Iterates key–value pairs in sorted key order.
    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, K, V> {
        self.inner.iter()
    }

    /// Iterates mutably in sorted key order.
    pub fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<'_, K, V> {
        self.inner.iter_mut()
    }
}

impl<K: Ord, V> IntoIterator for SortedVecMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K: Ord, V> Default for SortedVecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
