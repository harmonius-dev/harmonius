//! Row storage for typed tables.

use crate::ids::RowId;
use crate::value::Value;

/// A single logical row: primary key, optional prototype parent, and cell values.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    /// Primary key.
    pub id: RowId,
    /// Optional prototype parent for inheritance.
    pub parent: Option<RowId>,
    /// Dense column values aligned to [`crate::schema::TableSchema::columns`].
    pub values: Vec<Value>,
}
