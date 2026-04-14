//! Immutable typed [`DataTable`] storage.

use crate::filter::{row_matches, FilterExpr};
use crate::ids::{ColumnId, RowId};
use crate::index::BuiltIndex;
use crate::row::Row;
use crate::schema::TableSchema;
use crate::value::{LocalizedPack, Value};
use bumpalo::Bump;
use smol_str::SmolStr;
use std::collections::HashSet;

/// Typed, immutable table: schema + sorted rows + optional secondary indices.
#[derive(Clone, Debug)]
pub struct DataTable {
    schema: TableSchema,
    rows: Vec<Row>,
    indices: Vec<BuiltIndex>,
    version: u64,
}

/// Errors returned while constructing a [`DataTable`].
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DataTableError {
    /// Duplicate primary key detected while sorting / validating uniqueness.
    #[error("duplicate row id {0:?}")]
    DuplicateRowId(RowId),
}

impl DataTable {
    /// Constructs a table with sorted rows and built indices.
    pub fn try_new(
        schema: TableSchema,
        mut rows: Vec<Row>,
        version: u64,
    ) -> Result<Self, DataTableError> {
        rows.sort_by_key(|r| r.id);
        for w in rows.windows(2) {
            if w[0].id == w[1].id {
                return Err(DataTableError::DuplicateRowId(w[0].id));
            }
        }
        let indices = BuiltIndex::build_all(&schema, &rows);
        Ok(Self {
            schema,
            rows,
            indices,
            version,
        })
    }

    /// Primary-key lookup: `O(log n)` on sorted ids.
    pub fn get(&self, id: RowId) -> Option<&Row> {
        let idx = self.rows.binary_search_by_key(&id, |r| r.id).ok()?;
        Some(&self.rows[idx])
    }

    /// Row count.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Monotonic table version for hot reload.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Borrow the schema.
    pub fn schema(&self) -> &TableSchema {
        &self.schema
    }

    /// All rows in sorted primary-key order.
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// Returns `true` when `row` is a strict descendant of `ancestor` in the prototype chain.
    pub fn is_descendant_of(&self, row: RowId, ancestor: RowId) -> bool {
        let mut cur = self.get(row).and_then(|r| r.parent);
        let limit = self.row_count().saturating_add(1);
        let mut steps = 0usize;
        while let Some(rid) = cur {
            steps += 1;
            if steps > limit {
                return false;
            }
            if rid == ancestor {
                return true;
            }
            cur = self.get(rid).and_then(|r| r.parent);
        }
        false
    }

    /// Resolves inheritance then optional locale overlay for localized columns.
    pub fn get_resolved(&self, id: RowId, col: ColumnId, locale: Option<&str>) -> Option<Value> {
        let v = resolve_inherited(self, id, col)?;
        Some(localize_value(v, locale))
    }

    /// Filter rows using `expr`; results are stored in `arena`.
    pub fn query<'a>(&'a self, filter: &FilterExpr, arena: &'a Bump) -> &'a [&'a Row] {
        let matched: Vec<&Row> = self
            .rows
            .iter()
            .filter(|r| row_matches(r, filter))
            .collect();
        arena.alloc_slice_fill_with(matched.len(), |i| matched[i])
    }

    /// Range query on a btree-backed indexed column; falls back to brute force.
    pub fn range<'a>(
        &'a self,
        col: ColumnId,
        min: &Value,
        max: &Value,
        arena: &'a Bump,
    ) -> &'a [&'a Row] {
        if let (Value::I32(lo), Value::I32(hi)) = (min, max) {
            for ix in &self.indices {
                if ix.column() == col {
                    let ids = ix.range_i32(*lo, *hi);
                    let matched: Vec<&Row> = ids.iter().filter_map(|id| self.get(*id)).collect();
                    return arena.alloc_slice_fill_with(matched.len(), |i| matched[i]);
                }
            }
        }
        let matched: Vec<&Row> = self
            .rows
            .iter()
            .filter(|r| {
                let v = &r.values[col.0 as usize];
                match (min, max, v) {
                    (Value::I32(lo), Value::I32(hi), Value::I32(x)) => x >= lo && x <= hi,
                    _ => false,
                }
            })
            .collect();
        arena.alloc_slice_fill_with(matched.len(), |i| matched[i])
    }

    /// Looks up row ids via a hash index on `col` when present.
    pub fn index_lookup(&self, col: ColumnId, value: &Value) -> Option<Vec<RowId>> {
        for ix in &self.indices {
            if ix.column() == col {
                return ix.lookup(value);
            }
        }
        None
    }
}

/// Resolves a column through the prototype chain until a non-null cell is found.
pub fn resolve_inherited(table: &DataTable, row: RowId, col: ColumnId) -> Option<&Value> {
    let mut current = Some(row);
    let mut guard = 0usize;
    let limit = table.row_count().saturating_add(1);
    while let Some(rid) = current {
        guard += 1;
        if guard > limit {
            return None;
        }
        let row = table.get(rid)?;
        let v = &row.values[col.0 as usize];
        if !v.is_null() {
            return Some(v);
        }
        current = row.parent;
    }
    None
}

/// Detects a cycle reachable from `start` following `parent` links.
pub fn detect_cycle(table: &DataTable, start: RowId) -> Option<Vec<RowId>> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut cur = Some(start);
    let limit = table.row_count().saturating_add(1);
    let mut steps = 0usize;
    while let Some(rid) = cur {
        steps += 1;
        if steps > limit {
            return None;
        }
        if !visited.insert(rid) {
            let pos = stack.iter().position(|&x| x == rid)?;
            return Some(stack[pos..].to_vec());
        }
        stack.push(rid);
        let row = table.get(rid)?;
        cur = row.parent;
    }
    None
}

fn localize_value(v: &Value, locale: Option<&str>) -> Value {
    match v {
        Value::Localized(p) => {
            let Some(loc) = locale else {
                return Value::String(p.base.clone());
            };
            Value::String(resolve_locale_string(p, loc))
        }
        _ => v.clone(),
    }
}

fn resolve_locale_string(pack: &LocalizedPack, active: &str) -> SmolStr {
    if let Some(v) = pack.overlays.get(active) {
        return v.clone();
    }
    if let Some((lang, _)) = active.split_once('-') {
        if let Some(v) = pack.overlays.get(lang) {
            return v.clone();
        }
    }
    pack.base.clone()
}
