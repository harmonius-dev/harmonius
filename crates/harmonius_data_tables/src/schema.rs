//! Table schema definitions and per-row validation.

use crate::diagnostics::{ValidationError, ValidationSeverity};
use crate::ids::{ColumnId, RowId, TableId};
use crate::row::Row;
use crate::value::Value;
use smol_str::SmolStr;
use std::collections::BTreeMap;

/// Supported logical column types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColumnType {
    Bool,
    I32,
    I64,
    F32,
    F64,
    String,
    Enum,
    ForeignKey,
    AssetRef,
    EntityRef,
    Array,
    /// Locale-aware string with overlays.
    LocalizedString,
}

/// Optional per-column constraints evaluated at load time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnConstraint {
    /// Inclusive floating range used for `F32` / `F64` cells.
    Range {
        /// Minimum bound (inclusive).
        min: f64,
        /// Maximum bound (inclusive).
        max: f64,
    },
    /// Maximum UTF-8 byte length for `String` / localized base strings.
    MaxLength(usize),
}

/// Optional custom validation hook identified by tag (no runtime reflection).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CustomRule {
    /// Integer must be even (used by tests for custom-rule paths).
    MustBeEven,
}

/// Column metadata within a [`TableSchema`].
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnDef {
    /// Stable column id (dense index).
    pub id: ColumnId,
    /// Column name for diagnostics and tooling.
    pub name: SmolStr,
    /// Logical storage type.
    pub col_type: ColumnType,
    /// When `true`, [`Value::Null`] is accepted for this column.
    pub nullable: bool,
    /// When `true`, a secondary index may be built for this column.
    pub indexed: bool,
    /// Secondary index kind when `indexed` is `true`.
    pub index_kind: Option<crate::index::IndexType>,
    /// Optional load-time constraints.
    pub constraints: smallvec::SmallVec<[ColumnConstraint; 2]>,
    /// Optional custom rule evaluated after type checks.
    pub custom_rule: Option<CustomRule>,
    /// When `col_type` is `ForeignKey`, references this table id.
    pub fk_target_table: Option<TableId>,
}

/// Immutable schema for a single table.
#[derive(Clone, Debug, PartialEq)]
pub struct TableSchema {
    /// Owning table id.
    pub table_id: TableId,
    /// Human-readable table name.
    pub name: SmolStr,
    /// Column definitions in column-id order.
    pub columns: Vec<ColumnDef>,
    /// Column id used as the primary key (must be unique across rows).
    pub primary_key: ColumnId,
}

impl TableSchema {
    /// Returns the number of columns in this schema.
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Lookup column metadata by id.
    pub fn column(&self, id: ColumnId) -> Option<&ColumnDef> {
        self.columns.iter().find(|c| c.id == id)
    }

    /// Lookup column metadata by stable name.
    pub fn column_by_name(&self, name: &str) -> Option<&ColumnDef> {
        self.columns.iter().find(|c| c.name == name)
    }

    /// Returns a map from column name to id for tooling.
    pub fn column_index_by_name(&self) -> BTreeMap<SmolStr, ColumnId> {
        self.columns
            .iter()
            .map(|c| (c.name.clone(), c.id))
            .collect()
    }

    /// Validates a single row against this schema (types, constraints, custom rules).
    pub fn validate_row(&self, row: &Row) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if row.values.len() != self.columns.len() {
            errors.push(ValidationError {
                table: self.table_id,
                row: row.id,
                column: ColumnId(0),
                message: SmolStr::new("row value count mismatch"),
                severity: ValidationSeverity::Error,
            });
            return Err(errors);
        }
        for col in &self.columns {
            let cell = &row.values[col.id.0 as usize];
            if let Err(mut e) = validate_cell(self.table_id, row.id, col, cell) {
                errors.append(&mut e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn validate_cell(
    table: TableId,
    row: RowId,
    col: &ColumnDef,
    value: &Value,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if value.is_null() {
        if col.nullable {
            return Ok(());
        }
        errors.push(ValidationError {
            table,
            row,
            column: col.id,
            message: SmolStr::new("null in non-nullable column"),
            severity: ValidationSeverity::Error,
        });
        return Err(errors);
    }
    let ok_type = matches!(
        (col.col_type, value),
        (ColumnType::Bool, Value::Bool(_))
            | (ColumnType::I32, Value::I32(_))
            | (ColumnType::I64, Value::I64(_))
            | (ColumnType::F32, Value::F32(_))
            | (ColumnType::F64, Value::F64(_))
            | (ColumnType::String, Value::String(_))
            | (ColumnType::Enum, Value::Enum { .. })
            | (ColumnType::ForeignKey, Value::ForeignKey(_))
            | (ColumnType::AssetRef, Value::AssetRef(_))
            | (ColumnType::EntityRef, Value::EntityRef(_))
            | (ColumnType::Array, Value::Array(_))
            | (ColumnType::LocalizedString, Value::Localized(_))
    );
    if !ok_type {
        errors.push(ValidationError {
            table,
            row,
            column: col.id,
            message: SmolStr::new("type mismatch for column"),
            severity: ValidationSeverity::Error,
        });
    }
    for c in &col.constraints {
        match c {
            ColumnConstraint::Range { min, max } => match value {
                Value::F32(v) => {
                    let x = f64::from(*v);
                    if x < *min || x > *max {
                        errors.push(ValidationError {
                            table,
                            row,
                            column: col.id,
                            message: SmolStr::new("range violation"),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
                Value::F64(v) => {
                    if *v < *min || *v > *max {
                        errors.push(ValidationError {
                            table,
                            row,
                            column: col.id,
                            message: SmolStr::new("range violation"),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
                Value::I32(v) => {
                    let x = f64::from(*v);
                    if x < *min || x > *max {
                        errors.push(ValidationError {
                            table,
                            row,
                            column: col.id,
                            message: SmolStr::new("range violation"),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
                _ => {}
            },
            ColumnConstraint::MaxLength(max) => match value {
                Value::String(s) => {
                    if s.len() > *max {
                        errors.push(ValidationError {
                            table,
                            row,
                            column: col.id,
                            message: SmolStr::new("max length exceeded"),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
                Value::Localized(p) => {
                    if p.base.len() > *max {
                        errors.push(ValidationError {
                            table,
                            row,
                            column: col.id,
                            message: SmolStr::new("max length exceeded"),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
                _ => {}
            },
        }
    }
    if let Some(CustomRule::MustBeEven) = col.custom_rule {
        if let Value::I32(v) = value {
            if v % 2 != 0 {
                errors.push(ValidationError {
                    table,
                    row,
                    column: col.id,
                    message: SmolStr::new("custom rule: must be even"),
                    severity: ValidationSeverity::Error,
                });
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
