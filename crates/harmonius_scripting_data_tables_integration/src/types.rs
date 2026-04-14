//! Shared integration types for scripting and data tables (IR-2.9.x).

use rkyv::{Archive, Deserialize, Serialize};

/// Dense index into `FormulaFnTable` slots.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
pub struct FormulaId(pub u32);

/// Opaque dynamic library handle (resolved on the main thread only).
#[derive(Debug)]
pub struct DylibHandle {
    /// Version label carried for diagnostics and rollback tests.
    pub version: u32,
}

/// Column schema discriminator for bake-time checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColumnType {
    /// 32-bit float column.
    F32,
    /// 32-bit signed integer column.
    I32,
    /// Formula column referencing a compiled slot.
    Formula(FormulaId),
}

/// One resolved formula slot (runtime-only; not archived).
#[derive(Debug)]
pub struct FormulaFnSlot {
    /// Index matching `ColumnType::Formula(id)`.
    pub formula_id: FormulaId,
    /// Resolved symbol address; typed at each call site.
    pub fn_addr: *const (),
    /// Output column type for schema matching at bake time.
    pub output_type: ColumnType,
}

/// Table of codegen'd formula functions loaded from a middleman `.dylib`.
#[derive(Debug)]
pub struct FormulaFnTable {
    /// Dense slots indexed by `FormulaId.0`.
    pub fns: Vec<FormulaFnSlot>,
    /// Loaded middleman dynamic library for this table generation.
    pub dylib: DylibHandle,
    /// Monotonic version bumped on every successful reload.
    pub version: u32,
}

/// Stable table identifier (dense `u32`).
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
pub struct TableId(pub u32);

/// Stable column identifier (dense `u32`).
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
pub struct ColumnId(pub u32);

/// Dense row index inside a table.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
pub struct RowId(pub u32);

/// Inclusive row range used for reload scoping.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct RowRange {
    /// First affected row (inclusive).
    pub start: RowId,
    /// One past the last affected row (exclusive).
    pub end: RowId,
}

/// Dense `FormulaId` span used after `.dylib` reload.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct FormulaIdRange {
    /// First affected formula (inclusive).
    pub start: FormulaId,
    /// One past the last affected formula (exclusive).
    pub end: FormulaId,
}

/// Foreign-key style row pointer between tables.
#[derive(
    Archive,
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct RowRef {
    /// Target table id.
    pub table_id: TableId,
    /// Target row id.
    pub row_id: RowId,
}

/// Logic graph node that reads a data table cell.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TableLookupNode {
    /// Target table baked into codegen.
    pub table_id: TableId,
    /// Target column baked into codegen.
    pub column_id: ColumnId,
    /// How the row is resolved at runtime.
    pub row_source: RowSource,
}

/// Row resolution strategy for `TableLookupNode`.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RowSource {
    /// Static row reference baked at compile time.
    Static(RowRef),
    /// Read `(table, row)` from a `DatabaseRow` ECS component.
    EntityBinding,
}

/// Dispatched after a table hot-reload completes.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TableReloaded {
    /// Table whose data was replaced.
    pub table_id: TableId,
    /// New registry snapshot version.
    pub registry_version: u32,
    /// Rows that require formula re-bake.
    pub affected_rows: RowRange,
}

/// Dispatched after a new formula `.dylib` is installed.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FormulaDylibReloaded {
    /// New `FormulaFnTable` version.
    pub version: u32,
    /// Formula ids whose addresses changed.
    pub affected_formulas: FormulaIdRange,
}

/// Per-row bake result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BakeOutcome {
    /// Cell computed and written.
    Ok,
    /// Foreign key missing; default used.
    DefaultUsed,
    /// Bake rejected for the cell.
    Rejected(BakeError),
}

/// Bake-time failure reasons.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BakeError {
    /// Cycle in cross-row references.
    CycleDetected,
    /// Output type does not match column schema.
    TypeMismatch,
    /// `FormulaId` missing from the active `.dylib`.
    UnresolvedSymbol,
    /// Sandbox violation in authored Rust source.
    SandboxViolation,
}
