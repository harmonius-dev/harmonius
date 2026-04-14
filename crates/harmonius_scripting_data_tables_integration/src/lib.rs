//! Scripting ↔ data tables integration contracts (IR-2.9.x).
//!
//! See `docs/design/integration/scripting-data-tables.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod bake;
pub mod graph;
pub mod loader;
pub mod registry;
pub mod reload;
pub mod snapshot;
pub mod types;

pub use bake::{
    detect_formula_dependency_cycle, sandbox_check_rust_source, validate_formula_output_type,
};
pub use graph::DirectedGraph;
pub use loader::{MainThreadDylibLoader, MainThreadToken};
pub use registry::{DatabaseRow, ForeignRow, MaterialsRow, TableRegistry, WeaponsRow};
pub use reload::{PreFrameReloadLog, ReloadStep, run_pre_frame_reload};
pub use snapshot::AtomicRegistrySwitch;
pub use types::{
    BakeError, BakeOutcome, ColumnId, ColumnType, DylibHandle, FormulaDylibReloaded, FormulaFnSlot,
    FormulaFnTable, FormulaId, FormulaIdRange, RowId, RowRange, RowRef, RowSource, TableId,
    TableLookupNode, TableReloaded,
};

/// Monomorphized formula function signature shared by codegen and tests.
pub type FormulaFn<Row, T> = fn(&Row, &registry::TableRegistry) -> T;
