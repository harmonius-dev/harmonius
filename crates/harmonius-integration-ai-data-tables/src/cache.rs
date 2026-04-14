//! `AiTableCache` and column lookup errors.

use rkyv_derive::{Archive, Deserialize, Serialize};

use crate::ids::{ColumnId, RowId, TableId};

/// Discriminant for `CachedValue` used in `ColumnTypeMismatch`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Archive, Serialize, Deserialize)]
pub enum CachedValueKind {
    /// `f32` payload.
    Float,
    /// `i32` payload.
    Int,
    /// `bool` payload.
    Bool,
    /// String payload.
    String,
}

/// Cached scalar or string read from a data table column.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub enum CachedValue {
    /// Floating scalar.
    Float(f32),
    /// Signed integer.
    Int(i32),
    /// Boolean.
    Bool(bool),
    /// Owned UTF-8 string (design uses `Box<str>`; `String` satisfies rkyv derives).
    String(String),
}

impl CachedValue {
    /// Maps to the kind discriminant.
    pub fn kind(&self) -> CachedValueKind {
        match self {
            CachedValue::Float(_) => CachedValueKind::Float,
            CachedValue::Int(_) => CachedValueKind::Int,
            CachedValue::Bool(_) => CachedValueKind::Bool,
            CachedValue::String(_) => CachedValueKind::String,
        }
    }
}

/// Recoverable lookup failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColumnError {
    /// `TableRegistry::get` returned `None`.
    MissingTable(TableId),
    /// Row id not present in the table.
    MissingRow(TableId, RowId),
    /// Column id missing from schema.
    MissingColumn(TableId, ColumnId),
    /// Requested type does not match stored column type.
    ColumnTypeMismatch {
        /// Table id.
        table: TableId,
        /// Column id.
        column: ColumnId,
        /// Requested kind.
        expected: CachedValueKind,
        /// Observed kind.
        actual: CachedValueKind,
    },
    /// Cache cleared by reload; caller should retry next tick.
    SnapshotInvalidated(TableId),
}

/// Per-entity read-through row cache (sorted by `ColumnId`).
#[derive(Clone, Debug, Default, Archive, Serialize, Deserialize)]
pub struct AiTableCache {
    /// Cached column values sorted by `ColumnId`.
    pub entries: Vec<(ColumnId, CachedValue)>,
    /// Last table version observed for `bound_table`.
    pub version: u64,
    /// `true` immediately after invalidation until repopulated.
    pub cleared: bool,
    /// Table this cache tracks (for reload filtering).
    pub bound_table: TableId,
}

impl AiTableCache {
    /// Creates an empty cache pinned to `bound_table`.
    pub fn new(bound_table: TableId) -> Self {
        Self {
            entries: Vec::new(),
            version: 0,
            cleared: true,
            bound_table,
        }
    }

    /// Binary search for `column` in `entries`.
    pub fn find_entry(&self, column: ColumnId) -> Result<usize, usize> {
        self.entries.binary_search_by_key(&column, |(c, _)| *c)
    }

    /// Clears snapshot for reload handling.
    pub fn invalidate(&mut self, new_version: u64) {
        self.entries.clear();
        self.cleared = true;
        self.version = new_version;
    }

    /// Upserts a column value keeping sort order.
    pub fn insert_sorted(&mut self, column: ColumnId, value: CachedValue) {
        match self.entries.binary_search_by_key(&column, |(c, _)| *c) {
            Ok(i) => self.entries[i].1 = value,
            Err(i) => self.entries.insert(i, (column, value)),
        }
        self.cleared = false;
    }
}
