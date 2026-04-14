//! Static function-pointer tables backing `ReplicationCondition::Custom`.

use crate::ids::{ConnectionId, Entity};
use crate::wire::ReplicationFilterId;

/// Function pointer type for custom replication filters.
pub type ReplicationFilterFn<W> = fn(&W, Entity, ConnectionId) -> bool;

/// Codegen-shaped static table of replication filter functions.
pub struct ReplicationFilterTable<W: 'static + ?Sized> {
    entries: &'static [ReplicationFilterFn<W>],
}

impl<W: 'static + ?Sized> ReplicationFilterTable<W> {
    /// Wraps a static slice of filter functions (mirrors generated `FILTERS` table).
    #[must_use]
    pub fn new(entries: &'static [ReplicationFilterFn<W>]) -> Self {
        Self { entries }
    }

    /// Returns the function pointer at `id` when in range.
    #[must_use]
    pub fn get(&self, id: ReplicationFilterId) -> Option<ReplicationFilterFn<W>> {
        self.entries.get(id.0 as usize).copied()
    }

    /// Count of registered filters (mirrors generated `filter_count()` symbol).
    #[must_use]
    pub fn filter_count(&self) -> usize {
        self.entries.len()
    }
}
