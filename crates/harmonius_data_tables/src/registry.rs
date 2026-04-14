//! Dense registry of all loaded [`DataTable`] instances.

use crate::ids::{AssetHandle, RowRef, TableId};
use crate::row::Row;
use crate::table::DataTable;
use std::collections::HashSet;

/// Central dense registry indexed by [`TableId`].
#[derive(Clone, Debug, Default)]
pub struct TableRegistry {
    tables: Vec<Option<DataTable>>,
    /// Asset handles considered valid for `AssetRef` validation.
    pub known_assets: HashSet<AssetHandle>,
}

impl TableRegistry {
    /// Returns the number of table slots (including empty slots).
    pub fn slot_count(&self) -> usize {
        self.tables.len()
    }

    /// Counts non-empty registered tables.
    pub fn table_count(&self) -> usize {
        self.tables.iter().filter(|t| t.is_some()).count()
    }

    /// Borrows a table by id.
    pub fn get(&self, id: TableId) -> Option<&DataTable> {
        self.tables
            .get(id.0 as usize)
            .and_then(|slot| slot.as_ref())
    }

    /// Registers a table, expanding the backing store as needed.
    pub fn insert(&mut self, id: TableId, table: DataTable) {
        let idx = id.0 as usize;
        if idx >= self.tables.len() {
            self.tables.resize_with(idx + 1, || None);
        }
        self.tables[idx] = Some(table);
    }

    /// Swaps a table, returning the previous instance (for rollback stacks).
    pub fn swap(&mut self, id: TableId, new_table: DataTable) -> Option<DataTable> {
        let idx = id.0 as usize;
        if idx >= self.tables.len() {
            self.tables.resize_with(idx + 1, || None);
        }
        self.tables[idx].replace(new_table)
    }

    /// Removes a table slot entirely.
    pub fn remove(&mut self, id: TableId) -> Option<DataTable> {
        let idx = id.0 as usize;
        let t = self.tables.get_mut(idx)?;
        t.take()
    }

    /// Resolves a [`RowRef`] to a row in the target table (zero-copy `&Row`).
    pub fn resolve_foreign_key(&self, row_ref: &RowRef) -> Option<&Row> {
        self.get(row_ref.table)?.get(row_ref.row)
    }
}
