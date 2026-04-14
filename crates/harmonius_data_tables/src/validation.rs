//! Cross-table validation helpers.

use crate::diagnostics::{ValidationError, ValidationSeverity};
use crate::ids::TableId;
use crate::registry::TableRegistry;
use crate::row::Row;
use crate::schema::{ColumnType, TableSchema};
use crate::table::DataTable;
use crate::value::Value;
use smol_str::SmolStr;

/// Validates one table: schema rows, FK targets, and asset refs.
pub fn validate_table(table: &DataTable, registry: &TableRegistry) -> Vec<ValidationError> {
    let mut out = Vec::new();
    let schema = table.schema();
    for row in table.rows() {
        if let Err(mut e) = schema.validate_row(row) {
            out.append(&mut e);
        }
        validate_row_cross(schema, row, registry, &mut out);
    }
    out
}

/// Validates every registered table.
pub fn validate_all(registry: &TableRegistry) -> Vec<ValidationError> {
    let mut out = Vec::new();
    for tid in 0..registry.slot_count() {
        let id = TableId(tid as u32);
        let Some(t) = registry.get(id) else {
            continue;
        };
        out.extend(validate_table(t, registry));
    }
    out
}

fn validate_row_cross(
    schema: &TableSchema,
    row: &Row,
    registry: &TableRegistry,
    out: &mut Vec<ValidationError>,
) {
    for col in &schema.columns {
        let cell = &row.values[col.id.0 as usize];
        if cell.is_null() {
            continue;
        }
        if col.col_type == ColumnType::ForeignKey {
            if let Value::ForeignKey(rr) = cell {
                if let Some(expected) = col.fk_target_table {
                    if expected != rr.table {
                        out.push(ValidationError {
                            table: schema.table_id,
                            row: row.id,
                            column: col.id,
                            message: SmolStr::new("fk target table mismatch"),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
                if registry.resolve_foreign_key(rr).is_none() {
                    out.push(ValidationError {
                        table: schema.table_id,
                        row: row.id,
                        column: col.id,
                        message: SmolStr::new("broken foreign key"),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
        }
        if let Value::AssetRef(h) = cell {
            if !registry.known_assets.contains(h) {
                out.push(ValidationError {
                    table: schema.table_id,
                    row: row.id,
                    column: col.id,
                    message: SmolStr::new("missing asset"),
                    severity: ValidationSeverity::Warning,
                });
            }
        }
    }
}
