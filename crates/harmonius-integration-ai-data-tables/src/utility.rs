//! Utility consideration that reads numeric columns.

use crate::cache::{AiTableCache, CachedValue, CachedValueKind, ColumnError};
use crate::ids::ColumnId;
use crate::table::{DatabaseRow, TableRegistry, Value};

/// Response shaping for utility scores.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseCurve {
    /// Identity (`y = x`).
    Linear,
    /// `y = x * x` sign-preserving.
    Quadratic,
    /// Logistic (placeholder: identity for tests).
    Logistic,
    /// Step at 0.5 (placeholder: identity).
    Step,
    /// Piecewise (placeholder: identity).
    Piecewise,
}

impl ResponseCurve {
    fn apply(self, x: f32) -> f32 {
        match self {
            ResponseCurve::Linear => x,
            ResponseCurve::Quadratic => x * x.abs(), // preserve sign via x * abs(x)
            ResponseCurve::Logistic | ResponseCurve::Step | ResponseCurve::Piecewise => x,
        }
    }
}

/// Utility consideration reading a numeric column via `DatabaseRow`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableColumnConsideration {
    /// Column id.
    pub column: ColumnId,
    /// Curve applied after normalizing to `f32`.
    pub curve: ResponseCurve,
}

impl TableColumnConsideration {
    /// Resolves a numeric score from the bound row.
    pub fn lookup(
        &self,
        registry: &TableRegistry,
        db_row: &DatabaseRow,
        cache: &mut AiTableCache,
    ) -> Result<f32, ColumnError> {
        let Some(table) = registry.get(db_row.table) else {
            return Err(ColumnError::MissingTable(db_row.table));
        };

        if cache.bound_table != db_row.table {
            cache.bound_table = db_row.table;
            cache.entries.clear();
            cache.version = 0;
        }

        if cache.version != table.version() {
            cache.version = table.version();
            cache.entries.clear();
        }

        if let Ok(idx) = cache.find_entry(self.column) {
            if !cache.cleared {
                if let CachedValue::Float(f) = cache.entries[idx].1 {
                    return Ok(self.curve.apply(f));
                }
            }
        }

        if cache.cleared {
            return Err(ColumnError::SnapshotInvalidated(db_row.table));
        }

        if table.column_index(self.column).is_none() {
            return Err(ColumnError::MissingColumn(db_row.table, self.column));
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

        let raw = match resolved {
            None | Some(Value::Null) => 0.0,
            Some(Value::Float(f)) => f,
            Some(Value::Int(i)) => i as f32,
            Some(Value::Bool(_)) => {
                return Err(ColumnError::ColumnTypeMismatch {
                    table: db_row.table,
                    column: self.column,
                    expected: CachedValueKind::Float,
                    actual: CachedValueKind::Bool,
                });
            }
            Some(Value::String(_)) => {
                return Err(ColumnError::ColumnTypeMismatch {
                    table: db_row.table,
                    column: self.column,
                    expected: CachedValueKind::Float,
                    actual: CachedValueKind::String,
                });
            }
        };

        cache.insert_sorted(self.column, CachedValue::Float(raw));
        cache.version = table.version();
        Ok(self.curve.apply(raw))
    }
}
