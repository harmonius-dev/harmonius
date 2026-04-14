//! Cross-table join helpers.

use crate::filter::row_matches;
use crate::filter::FilterExpr;
use crate::ids::ColumnId;
use crate::row::Row;
use crate::table::DataTable;
use crate::value::Value;
use bumpalo::Bump;

/// One matched pair from a join query.
#[derive(Clone, Copy, Debug)]
pub struct JoinRow<'a> {
    /// Left-side row.
    pub left: &'a Row,
    /// Right-side row resolved via FK.
    pub right: &'a Row,
}

/// Joins `left` to `right` on `fk_col` containing [`Value::ForeignKey`] cells.
pub fn join_query<'a>(
    left: &'a DataTable,
    fk_col: ColumnId,
    right: &'a DataTable,
    filter: Option<&FilterExpr>,
    arena: &'a Bump,
) -> &'a [JoinRow<'a>] {
    let mut pairs = Vec::new();
    for lrow in left.rows() {
        if let Some(f) = filter {
            if !row_matches(lrow, f) {
                continue;
            }
        }
        let cell = &lrow.values[fk_col.0 as usize];
        let Value::ForeignKey(rr) = cell else {
            continue;
        };
        if rr.table != right.schema().table_id {
            continue;
        }
        let Some(rrow) = right.get(rr.row) else {
            continue;
        };
        pairs.push(JoinRow {
            left: lrow,
            right: rrow,
        });
    }
    arena.alloc_slice_fill_with(pairs.len(), |i| pairs[i])
}
