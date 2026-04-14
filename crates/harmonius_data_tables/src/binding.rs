//! ECS binding surface (minimal synchronous helpers for tests).

use crate::ids::{ColumnId, RowId, TableId};
use crate::registry::TableRegistry;
use crate::value::Value;
use smallvec::SmallVec;
use smol_str::SmolStr;
use std::collections::BTreeMap;

/// Entity → database row binding (engine-facing component shape).
#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseRow {
    /// Referenced table.
    pub table: TableId,
    /// Referenced primary key.
    pub row: RowId,
    /// Optional subset of columns to bind (empty = all columns).
    pub bound_columns: SmallVec<[ColumnId; 8]>,
    /// Sorted overrides `(column, value)`.
    pub overrides: SmallVec<[(ColumnId, Value); 4]>,
}

/// Resolved column snapshot for binding tests (name → value).
pub type BindingSnapshot = BTreeMap<SmolStr, Value>;

/// Resolves the effective values for a [`DatabaseRow`] without touching ECS.
pub fn binding_snapshot(registry: &TableRegistry, db: &DatabaseRow) -> Option<BindingSnapshot> {
    let table = registry.get(db.table)?;
    table.get(db.row)?;
    let schema = table.schema();
    let mut map = BindingSnapshot::new();
    for col in &schema.columns {
        if !db.bound_columns.is_empty() && !db.bound_columns.contains(&col.id) {
            continue;
        }
        if let Some((_, v)) = db.overrides.iter().find(|(cid, _)| *cid == col.id) {
            map.insert(col.name.clone(), v.clone());
            continue;
        }
        let v = crate::table::resolve_inherited(table, db.row, col.id)?.clone();
        map.insert(col.name.clone(), v);
    }
    Some(map)
}

/// Example 2D transform populated from `x`/`y` columns.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transform2D {
    /// X position.
    pub x: f32,
    /// Y position.
    pub y: f32,
}

/// Populates [`Transform2D`] from `x`/`y` [`Value::F32`] cells.
pub fn populate_transform2d(snapshot: &BindingSnapshot, out: &mut Transform2D) {
    if let Some(Value::F32(x)) = snapshot.get("x") {
        out.x = *x;
    }
    if let Some(Value::F32(y)) = snapshot.get("y") {
        out.y = *y;
    }
}
