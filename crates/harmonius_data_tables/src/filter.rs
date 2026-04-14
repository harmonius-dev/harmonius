//! Row filter expressions evaluated against materialized rows.

use crate::ids::ColumnId;
use crate::row::Row;
use crate::value::Value;
use smol_str::SmolStr;

/// Predicate applied to a single column.
#[derive(Clone, Debug, PartialEq)]
pub enum ColumnPredicate {
    /// Equality against a concrete value.
    Equals(Value),
    /// Inequality against a concrete value.
    NotEquals(Value),
    /// Less-than ordering when supported.
    LessThan(Value),
    /// Less-or-equal ordering when supported.
    LessOrEqual(Value),
    /// Greater-than ordering when supported.
    GreaterThan(Value),
    /// Greater-or-equal ordering when supported.
    GreaterOrEqual(Value),
    /// Inclusive range on ordered scalars.
    Range {
        /// Minimum bound (inclusive).
        min: Value,
        /// Maximum bound (inclusive).
        max: Value,
    },
    /// Substring match for strings.
    Contains(SmolStr),
    /// Matches only null cells.
    IsNull,
    /// Matches only non-null cells.
    IsNotNull,
}

/// Composable boolean filter tree.
#[derive(Clone, Debug, PartialEq)]
pub enum FilterExpr {
    /// Leaf predicate on one column.
    Column {
        /// Target column.
        col: ColumnId,
        /// Predicate to apply.
        predicate: ColumnPredicate,
    },
    /// Logical AND of child expressions.
    And(Vec<FilterExpr>),
    /// Logical OR of child expressions.
    Or(Vec<FilterExpr>),
    /// Logical NOT of a child expression.
    Not(Box<FilterExpr>),
}

fn cmp_values(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (Value::I32(x), Value::I32(y)) => x.partial_cmp(y),
        (Value::Bool(x), Value::Bool(y)) => x.partial_cmp(y),
        (Value::String(x), Value::String(y)) => Some(x.cmp(y)),
        _ => None,
    }
}

fn eval_predicate(row: &Row, col: ColumnId, pred: &ColumnPredicate) -> bool {
    let cell = &row.values[col.0 as usize];
    match pred {
        ColumnPredicate::Equals(v) => cell == v,
        ColumnPredicate::NotEquals(v) => cell != v,
        ColumnPredicate::LessThan(v) => cmp_values(cell, v) == Some(std::cmp::Ordering::Less),
        ColumnPredicate::LessOrEqual(v) => matches!(
            cmp_values(cell, v),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ),
        ColumnPredicate::GreaterThan(v) => cmp_values(cell, v) == Some(std::cmp::Ordering::Greater),
        ColumnPredicate::GreaterOrEqual(v) => matches!(
            cmp_values(cell, v),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        ),
        ColumnPredicate::Range { min, max } => {
            let Some(lo) = cmp_values(cell, min) else {
                return false;
            };
            let Some(hi) = cmp_values(cell, max) else {
                return false;
            };
            (lo == std::cmp::Ordering::Greater || lo == std::cmp::Ordering::Equal)
                && (hi == std::cmp::Ordering::Less || hi == std::cmp::Ordering::Equal)
        }
        ColumnPredicate::Contains(s) => {
            if let Value::String(cur) = cell {
                cur.contains(s.as_str())
            } else {
                false
            }
        }
        ColumnPredicate::IsNull => cell.is_null(),
        ColumnPredicate::IsNotNull => !cell.is_null(),
    }
}

/// Returns `true` when `row` satisfies `expr`.
pub fn row_matches(row: &Row, expr: &FilterExpr) -> bool {
    match expr {
        FilterExpr::Column { col, predicate } => eval_predicate(row, *col, predicate),
        FilterExpr::And(parts) => parts.iter().all(|p| row_matches(row, p)),
        FilterExpr::Or(parts) => parts.iter().any(|p| row_matches(row, p)),
        FilterExpr::Not(inner) => !row_matches(row, inner),
    }
}
