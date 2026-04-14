//! Behavior-tree leaf that reads a table column into the cache.

use crate::blackboard::BlackboardKey;
use crate::cache::{AiTableCache, CachedValue, CachedValueKind, ColumnError};
use crate::ids::ColumnId;
use crate::table::{DatabaseRow, TableRegistry, Value};

/// BT leaf that reads an NPC parameter column from the entity's `DatabaseRow`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BtTableLookup {
    /// Column to read.
    pub column: ColumnId,
    /// Blackboard key receiving the value (written by the BT driver).
    pub target_key: BlackboardKey,
}

impl BtTableLookup {
    /// Resolves a column value using `DatabaseRow` + read-through cache.
    pub fn lookup(
        &self,
        registry: &TableRegistry,
        db_row: &DatabaseRow,
        cache: &mut AiTableCache,
    ) -> Result<CachedValue, ColumnError> {
        if cache.bound_table != db_row.table {
            cache.bound_table = db_row.table;
            cache.entries.clear();
            cache.cleared = true;
            cache.version = 0;
        }

        let table = registry
            .get(db_row.table)
            .ok_or(ColumnError::MissingTable(db_row.table))?;

        if table.column_index(self.column).is_none() {
            return Err(ColumnError::MissingColumn(db_row.table, self.column));
        }

        if table.get(db_row.row).is_none() {
            return Err(ColumnError::MissingRow(db_row.table, db_row.row));
        }

        if cache.version != table.version() {
            cache.version = table.version();
            cache.entries.clear();
            cache.cleared = true;
        }

        if let Ok(idx) = cache.find_entry(self.column) {
            if !cache.cleared {
                return Ok(cache.entries[idx].1.clone());
            }
        }

        let resolved = table.get_resolved(db_row.row, self.column).or_else(|| {
            table
                .column_index(self.column)
                .and_then(|ci| table.columns().get(ci))
                .and_then(|c| c.default.clone())
        });

        let resolved = match resolved {
            Some(v) => db_row.apply_overrides(self.column, Some(v)),
            None => db_row.apply_overrides(self.column, None),
        };

        let Some(cell) = resolved else {
            return Err(ColumnError::MissingRow(db_row.table, db_row.row));
        };

        let cached = match &cell {
            Value::Float(f) => CachedValue::Float(*f),
            Value::Int(i) => CachedValue::Float(*i as f32),
            Value::Bool(_) => {
                return Err(ColumnError::ColumnTypeMismatch {
                    table: db_row.table,
                    column: self.column,
                    expected: CachedValueKind::Float,
                    actual: CachedValueKind::Bool,
                });
            }
            Value::Null => {
                return Err(ColumnError::MissingRow(db_row.table, db_row.row));
            }
            Value::String(_) => {
                return Err(ColumnError::ColumnTypeMismatch {
                    table: db_row.table,
                    column: self.column,
                    expected: CachedValueKind::Float,
                    actual: CachedValueKind::String,
                });
            }
        };

        cache.insert_sorted(self.column, cached.clone());
        cache.version = table.version();
        Ok(cached)
    }
}
