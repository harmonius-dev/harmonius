//! Secondary indices for exact and range queries.

use crate::ids::{ColumnId, RowId};
use crate::row::Row;
use crate::schema::{ColumnType, TableSchema};
use crate::value::Value;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap};

/// Index algorithm for a secondary index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexType {
    /// Hash map exact lookup.
    Hash,
    /// Ordered map range lookup.
    BTree,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum BuiltIndex {
    /// Integer hash index.
    HashI32 {
        /// Indexed column.
        column: ColumnId,
        /// Value → matching row ids (sorted, deduped).
        map: HashMap<i32, Vec<RowId>>,
    },
    /// Integer btree index.
    BTreeI32 {
        /// Indexed column.
        column: ColumnId,
        /// Value → matching row ids.
        map: BTreeMap<i32, Vec<RowId>>,
    },
    /// String hash index.
    HashStr {
        /// Indexed column.
        column: ColumnId,
        /// Value → matching row ids.
        map: HashMap<SmolStr, Vec<RowId>>,
    },
}

impl BuiltIndex {
    /// Builds all declared indices for `schema` over `rows`.
    pub fn build_all(schema: &TableSchema, rows: &[Row]) -> Vec<BuiltIndex> {
        let mut out = Vec::new();
        for col in &schema.columns {
            if !col.indexed {
                continue;
            }
            let Some(kind) = col.index_kind else {
                continue;
            };
            match (col.col_type, kind) {
                (ColumnType::I32, IndexType::Hash) => {
                    let mut map: HashMap<i32, Vec<RowId>> = HashMap::new();
                    for row in rows {
                        if let Value::I32(v) = &row.values[col.id.0 as usize] {
                            map.entry(*v).or_default().push(row.id);
                        }
                    }
                    for ids in map.values_mut() {
                        ids.sort_unstable();
                        ids.dedup();
                    }
                    out.push(BuiltIndex::HashI32 {
                        column: col.id,
                        map,
                    });
                }
                (ColumnType::I32, IndexType::BTree) => {
                    let mut map: BTreeMap<i32, Vec<RowId>> = BTreeMap::new();
                    for row in rows {
                        if let Value::I32(v) = &row.values[col.id.0 as usize] {
                            map.entry(*v).or_default().push(row.id);
                        }
                    }
                    for ids in map.values_mut() {
                        ids.sort_unstable();
                        ids.dedup();
                    }
                    out.push(BuiltIndex::BTreeI32 {
                        column: col.id,
                        map,
                    });
                }
                (ColumnType::String, IndexType::Hash) => {
                    let mut map: HashMap<SmolStr, Vec<RowId>> = HashMap::new();
                    for row in rows {
                        if let Value::String(s) = &row.values[col.id.0 as usize] {
                            map.entry(s.clone()).or_default().push(row.id);
                        }
                    }
                    for ids in map.values_mut() {
                        ids.sort_unstable();
                        ids.dedup();
                    }
                    out.push(BuiltIndex::HashStr {
                        column: col.id,
                        map,
                    });
                }
                _ => {}
            }
        }
        out
    }

    /// Exact lookup for hash-style indices.
    pub fn lookup(&self, value: &Value) -> Option<Vec<RowId>> {
        match self {
            BuiltIndex::HashI32 { map, .. } => {
                let Value::I32(v) = value else {
                    return None;
                };
                map.get(v).cloned()
            }
            BuiltIndex::HashStr { map, .. } => {
                let Value::String(s) = value else {
                    return None;
                };
                map.get(s).cloned()
            }
            BuiltIndex::BTreeI32 { .. } => None,
        }
    }

    /// Range query inclusive on btree i32 indices.
    pub fn range_i32(&self, min: i32, max: i32) -> Vec<RowId> {
        let BuiltIndex::BTreeI32 { map, .. } = self else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for (_k, ids) in map.range(min..=max) {
            out.extend_from_slice(ids.as_slice());
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// Column id for this built index.
    pub fn column(&self) -> ColumnId {
        match self {
            BuiltIndex::HashI32 { column, .. }
            | BuiltIndex::BTreeI32 { column, .. }
            | BuiltIndex::HashStr { column, .. } => *column,
        }
    }
}
