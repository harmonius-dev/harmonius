//! Dense component bag used by reload draining.

use crate::cache::AiTableCache;
use crate::ids::TableId;
use crate::world::EntityId;

/// Vec-backed component store (tests + integration harness).
#[derive(Clone, Debug, Default)]
pub struct ComponentStore<T> {
    entries: Vec<(EntityId, T)>,
}

impl<T> ComponentStore<T> {
    /// Iterates all pairs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&EntityId, &mut T)> {
        self.entries.iter_mut().map(|(e, t)| (&*e, t))
    }

    /// Upserts a component for `entity`.
    pub fn insert(&mut self, entity: EntityId, value: T) {
        if let Some((_, slot)) = self.entries.iter_mut().find(|(e, _)| *e == entity) {
            *slot = value;
        } else {
            self.entries.push((entity, value));
        }
    }

    /// Borrows a component for `entity`.
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.entries
            .iter_mut()
            .find_map(|(e, t)| if *e == entity { Some(t) } else { None })
    }
}

impl ComponentStore<AiTableCache> {
    /// Clears caches for every entity bound to `table` when the table version advances.
    pub fn invalidate_table(&mut self, table: TableId, new_version: u64) {
        for (_, cache) in self.iter_mut() {
            if cache.bound_table == table && cache.version < new_version {
                cache.invalidate(new_version);
            }
        }
    }
}
