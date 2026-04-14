//! RON import for authoring-time table payloads.

use crate::ids::{AssetHandle, ColumnId, Entity, EnumSchemaId, RowId, RowRef, TableId};
use crate::row::Row;
use crate::schema::{ColumnConstraint, ColumnDef, ColumnType, CustomRule, TableSchema};
use crate::table::{DataTable, DataTableError};
use crate::value::{LocalizedPack, Value};
use serde::Deserialize;
use smol_str::SmolStr;
use std::collections::BTreeMap;

/// Parse failure for [`import_table_from_ron`].
#[derive(Clone, Debug, thiserror::Error)]
pub enum ImportError {
    /// RON syntax or schema mismatch.
    #[error("ron parse: {0}")]
    Ron(String),
    /// Table construction failed after parse.
    #[error("table: {0}")]
    Table(#[from] DataTableError),
}

#[derive(Debug, Deserialize)]
struct RonColumn {
    id: u16,
    name: String,
    col_type: String,
    nullable: bool,
    indexed: bool,
    index_kind: Option<String>,
    fk_target: Option<u32>,
    constraints: Option<Vec<RonConstraint>>,
    custom_rule: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RonConstraint {
    kind: String,
    min: Option<f64>,
    max: Option<f64>,
    max_len: Option<usize>,
}

/// Loose cell encoding for RON tables (positionally aligned to columns).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RonCell {
    /// JSON `null`.
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    /// RON `String("...")` cell.
    #[serde(rename = "String")]
    RString(String),
    Text(String),
    ForeignKey {
        /// Referenced table id.
        table: u32,
        /// Referenced row id.
        row: u64,
    },
    Enum {
        /// Enum schema id.
        schema_id: u32,
        /// Variant index.
        variant: u32,
    },
    Asset(u64),
    Entity(u64),
    StringList(Vec<i64>),
    Localized {
        /// Default string.
        base: String,
        /// Optional overlays map.
        overlays: Option<BTreeMap<String, String>>,
    },
}

#[derive(Debug, Deserialize)]
struct RonRow {
    id: u64,
    parent: Option<u64>,
    values: Vec<RonCell>,
}

#[derive(Debug, Deserialize)]
struct RonTable {
    table_id: u32,
    name: String,
    primary_key: u16,
    columns: Vec<RonColumn>,
    rows: Vec<RonRow>,
    version: Option<u64>,
}

fn parse_col_type(s: &str) -> Option<ColumnType> {
    Some(match s {
        "bool" => ColumnType::Bool,
        "i32" => ColumnType::I32,
        "i64" => ColumnType::I64,
        "f32" => ColumnType::F32,
        "f64" => ColumnType::F64,
        "string" => ColumnType::String,
        "enum" => ColumnType::Enum,
        "foreign_key" => ColumnType::ForeignKey,
        "asset_ref" => ColumnType::AssetRef,
        "entity_ref" => ColumnType::EntityRef,
        "array" => ColumnType::Array,
        "localized_string" => ColumnType::LocalizedString,
        _ => return None,
    })
}

fn parse_index_kind(s: &str) -> Option<crate::index::IndexType> {
    Some(match s {
        "hash" => crate::index::IndexType::Hash,
        "btree" => crate::index::IndexType::BTree,
        _ => return None,
    })
}

fn cell_to_value(col: &ColumnDef, cell: &RonCell) -> Result<Value, ImportError> {
    Ok(match (col.col_type, cell) {
        (_, RonCell::Null) => Value::Null,
        (ColumnType::Bool, RonCell::Bool(b)) => Value::Bool(*b),
        (ColumnType::I32, RonCell::Int(n)) => Value::I32(*n as i32),
        (ColumnType::I64, RonCell::Int(n)) => Value::I64(*n),
        (ColumnType::F32, RonCell::Float(f)) => Value::F32(*f as f32),
        (ColumnType::F32, RonCell::Int(n)) => Value::F32(*n as f32),
        (ColumnType::F64, RonCell::Float(f)) => Value::F64(*f),
        (ColumnType::String, RonCell::Text(s)) | (ColumnType::String, RonCell::RString(s)) => {
            Value::String(SmolStr::new(s))
        }
        (ColumnType::Enum, RonCell::Enum { schema_id, variant }) => Value::Enum {
            schema_id: EnumSchemaId(*schema_id),
            variant: *variant,
        },
        (ColumnType::ForeignKey, RonCell::ForeignKey { table, row }) => Value::ForeignKey(RowRef {
            table: TableId(*table),
            row: RowId(*row),
        }),
        (ColumnType::AssetRef, RonCell::Asset(h)) => Value::AssetRef(AssetHandle(*h)),
        (ColumnType::EntityRef, RonCell::Entity(e)) => Value::EntityRef(Entity(*e)),
        (ColumnType::Array, RonCell::StringList(items)) => {
            Value::Array(items.iter().map(|i| Value::I32(*i as i32)).collect())
        }
        (ColumnType::LocalizedString, RonCell::Localized { base, overlays }) => {
            let mut m = BTreeMap::new();
            if let Some(o) = overlays {
                for (k, v) in o {
                    m.insert(SmolStr::new(k), SmolStr::new(v));
                }
            }
            Value::Localized(LocalizedPack {
                base: SmolStr::new(base),
                overlays: m,
            })
        }
        _ => {
            return Err(ImportError::Ron(format!(
                "cell mismatch for column {}",
                col.name
            )));
        }
    })
}

/// Parses a full table document from a RON string and constructs a [`DataTable`].
pub fn import_table_from_ron(ron: &str) -> Result<DataTable, ImportError> {
    let parsed: RonTable = ron::from_str(ron).map_err(|e| ImportError::Ron(e.to_string()))?;
    let mut cols = Vec::new();
    for c in parsed.columns {
        let col_type = parse_col_type(&c.col_type)
            .ok_or_else(|| ImportError::Ron(format!("bad col_type {}", c.col_type)))?;
        let index_kind = match (&c.indexed, &c.index_kind) {
            (true, Some(s)) => {
                Some(parse_index_kind(s).ok_or_else(|| ImportError::Ron("bad index_kind".into()))?)
            }
            _ => None,
        };
        let mut constraints = smallvec::SmallVec::new();
        if let Some(list) = c.constraints {
            for x in list {
                match x.kind.as_str() {
                    "range" => {
                        constraints.push(ColumnConstraint::Range {
                            min: x.min.ok_or_else(|| ImportError::Ron("range min".into()))?,
                            max: x.max.ok_or_else(|| ImportError::Ron("range max".into()))?,
                        });
                    }
                    "max_length" => {
                        constraints.push(ColumnConstraint::MaxLength(
                            x.max_len
                                .ok_or_else(|| ImportError::Ron("max_len".into()))?,
                        ));
                    }
                    _ => {}
                }
            }
        }
        let custom_rule = c.custom_rule.as_deref().map(|s| match s {
            "must_be_even" => CustomRule::MustBeEven,
            _ => CustomRule::MustBeEven,
        });
        cols.push(ColumnDef {
            id: ColumnId(c.id),
            name: SmolStr::new(&c.name),
            col_type,
            nullable: c.nullable,
            indexed: c.indexed,
            index_kind,
            constraints,
            custom_rule,
            fk_target_table: c.fk_target.map(TableId),
        });
    }
    let schema = TableSchema {
        table_id: TableId(parsed.table_id),
        name: SmolStr::new(&parsed.name),
        columns: cols,
        primary_key: ColumnId(parsed.primary_key),
    };
    let mut rows = Vec::new();
    for (ri, r) in parsed.rows.into_iter().enumerate() {
        if r.values.len() != schema.columns.len() {
            return Err(ImportError::Ron(format!(
                "row {ri} value count {}",
                r.values.len()
            )));
        }
        let mut values = Vec::with_capacity(schema.columns.len());
        for (col, cell) in schema.columns.iter().zip(r.values.iter()) {
            values.push(cell_to_value(col, cell)?);
        }
        rows.push(Row {
            id: RowId(r.id),
            parent: r.parent.map(RowId),
            values,
        });
    }
    DataTable::try_new(schema, rows, parsed.version.unwrap_or(1)).map_err(ImportError::from)
}
