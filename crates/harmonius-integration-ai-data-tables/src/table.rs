//! Minimal `DataTable` / `TableRegistry` / `DatabaseRow` for integration tests.

use smallvec::SmallVec;

use crate::ids::{ColumnId, RowId, TableId};

/// Typed cell payload stored in a row.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// 32-bit float.
    Float(f32),
    /// 32-bit signed integer.
    Int(i32),
    /// Boolean flag.
    Bool(bool),
    /// UTF-8 string.
    String(String),
    /// SQL-style null.
    Null,
}

/// Single row in a `DataTable`.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    /// Primary key.
    pub id: RowId,
    /// Optional prototype row for inheritance tests (IR-2.1.4).
    pub parent: Option<RowId>,
    /// Column-major values aligned to the owning table's schema order.
    pub values: Vec<Value>,
}

/// Column metadata: id, type hint, optional schema default for fallbacks.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnSchema {
    /// Column id.
    pub id: ColumnId,
    /// Default used when lookups fail or cells are null.
    pub default: Option<Value>,
}

/// Immutable table snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTable {
    columns: Vec<ColumnSchema>,
    rows: Vec<Row>,
    version: u64,
}

impl DataTable {
    /// Builds a table with sorted rows by `RowId` and a column schema list.
    pub fn new(columns: Vec<ColumnSchema>, mut rows: Vec<Row>, version: u64) -> Self {
        rows.sort_by_key(|r| r.id);
        Self {
            columns,
            rows,
            version,
        }
    }

    /// Schema column count.
    pub fn column_index(&self, col: ColumnId) -> Option<usize> {
        self.columns.iter().position(|c| c.id == col)
    }

    /// Primary-key lookup.
    pub fn get(&self, id: RowId) -> Option<&Row> {
        self.rows
            .binary_search_by_key(&id, |r| r.id)
            .ok()
            .map(|i| &self.rows[i])
    }

    /// Table version (bumped on hot reload).
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Column schema slice.
    pub fn columns(&self) -> &[ColumnSchema] {
        &self.columns
    }

    /// Resolved value for `(row, col)` with single-hop parent inheritance for nulls.
    pub fn get_resolved(&self, row: RowId, col: ColumnId) -> Option<Value> {
        let idx = self.column_index(col)?;
        let mut current = row;
        loop {
            let r = self.get(current)?;
            match r.values.get(idx).cloned() {
                Some(Value::Null) | None => {
                    if let Some(p) = r.parent {
                        current = p;
                        continue;
                    }
                    return self.columns[idx].default.clone();
                }
                Some(v) => return Some(v),
            }
        }
    }
}

/// Registry of loaded tables (dense `Vec` indexed by `TableId`).
#[derive(Clone, Debug, Default)]
pub struct TableRegistry {
    tables: Vec<Option<DataTable>>,
}

impl TableRegistry {
    /// O(1) lookup.
    pub fn get(&self, id: TableId) -> Option<&DataTable> {
        self.tables.get(id.0 as usize).and_then(|t| t.as_ref())
    }

    /// Insert or replace a slot, growing the vector as needed.
    pub fn insert(&mut self, id: TableId, table: DataTable) {
        let idx = id.0 as usize;
        if idx >= self.tables.len() {
            self.tables.resize_with(idx + 1, || None);
        }
        self.tables[idx] = Some(table);
    }

    /// Hot-reload swap returning the previous table, if any.
    pub fn swap(&mut self, id: TableId, new_table: DataTable) -> Option<DataTable> {
        let idx = id.0 as usize;
        if idx >= self.tables.len() {
            self.tables.resize_with(idx + 1, || None);
        }
        self.tables[idx].replace(new_table)
    }

    /// Foreign-key style resolve: `row_ref` points into `self`.
    pub fn resolve_foreign_key(&self, table: TableId, row: RowId) -> Option<&Row> {
        self.get(table)?.get(row)
    }
}

/// ECS component binding an entity to a data row.
#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseRow {
    /// Owning table.
    pub table: TableId,
    /// Primary key row.
    pub row: RowId,
    /// Optional column subset (empty = all schema columns participate).
    pub bound_columns: SmallVec<[ColumnId; 8]>,
    /// Per-column overrides sorted by `ColumnId`.
    pub overrides: SmallVec<[(ColumnId, Value); 4]>,
}

impl DatabaseRow {
    /// Returns true when `self.table` matches `table`.
    pub fn references_table(&self, table: TableId) -> bool {
        self.table == table
    }
}

impl DatabaseRow {
    /// Merge override into resolved column read.
    pub fn apply_overrides(&self, col: ColumnId, base: Option<Value>) -> Option<Value> {
        if let Ok(i) = self.overrides.binary_search_by_key(&col, |(c, _)| *c) {
            return Some(self.overrides[i].1.clone());
        }
        base
    }
}
