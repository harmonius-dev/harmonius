use crate::pso_key::PsoKey;
use crate::sorted_vec_map::SortedVecMap;

/// Minimal resident entry used by [`MemoryPsoCache`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryEntry {
    /// Last successful lookup tick.
    pub last_used_tick: u64,
    /// Opaque payload mirrored in tests.
    pub payload: u32,
}

/// In-memory PSO table backed by [`SortedVecMap`].
#[derive(Clone, Debug)]
pub struct MemoryPsoCache {
    map: SortedVecMap<PsoKey, MemoryEntry>,
    tick: u64,
}

impl Default for MemoryPsoCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPsoCache {
    /// Creates an empty cache at tick zero.
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: SortedVecMap::new(),
            tick: 0,
        }
    }

    /// Inserts `payload` for `key` stamped with the current tick.
    pub fn insert(&mut self, key: PsoKey, payload: u32) {
        let tick = self.tick;
        self.map.insert(
            key,
            MemoryEntry {
                last_used_tick: tick,
                payload,
            },
        );
    }

    /// Successful lookups bump [`MemoryEntry::last_used_tick`] to the current tick.
    #[must_use]
    pub fn get_mut(&mut self, key: &PsoKey) -> Option<&mut MemoryEntry> {
        let tick = self.tick;
        let entry = self.map.get_mut(key)?;
        entry.last_used_tick = tick;
        Some(entry)
    }

    /// Advances the logical clock.
    pub fn bump_tick(&mut self) {
        self.tick = self.tick.saturating_add(1);
    }

    /// Current tick.
    #[must_use]
    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    /// Borrow the underlying sorted map for structural tests.
    #[must_use]
    pub fn map(&self) -> &SortedVecMap<PsoKey, MemoryEntry> {
        &self.map
    }

    /// Clears all entries.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` when no entries are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
