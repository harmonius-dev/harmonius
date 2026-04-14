//! Cross-cutting diagnostic types.

use crate::ids::{ColumnId, RowId, TableId};
use smol_str::SmolStr;

/// Severity for [`ValidationError`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationSeverity {
    /// Hard failure: table must not load.
    Error,
    /// Soft failure: load may proceed with telemetry.
    Warning,
}

/// A single validation diagnostic for a specific cell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    /// Table containing the offending row.
    pub table: TableId,
    /// Row primary key.
    pub row: RowId,
    /// Column id when applicable.
    pub column: ColumnId,
    /// Human-readable message (short, stable prefix for tests).
    pub message: SmolStr,
    /// Whether this should block load.
    pub severity: ValidationSeverity,
}
