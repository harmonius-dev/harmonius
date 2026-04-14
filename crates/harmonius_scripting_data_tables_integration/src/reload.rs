//! Pre-frame reload ordering and event dispatch stubs.

use crate::types::{FormulaDylibReloaded, FormulaIdRange, RowRange, TableId, TableReloaded};

/// One step in the ordered reload pipeline (FM-7).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReloadStep {
    /// Loaded or attempted `.dylib` install on the main thread.
    DylibLoad,
    /// Re-baked formula cells against the active `.dylib` + staging snapshot.
    TableRebake,
    /// Swapped the registry snapshot pointer.
    SnapshotSwap,
    /// Emitted `FormulaDylibReloaded` after a successful `.dylib` publish.
    DispatchFormulaDylibReloaded,
    /// Emitted `TableReloaded` after a successful snapshot swap.
    DispatchTableReloaded,
}

/// Ordered record of integration events emitted during a reload window.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreFrameReloadLog {
    /// Strictly ordered steps for assertions.
    pub steps: Vec<ReloadStep>,
    /// Captured `TableReloaded` after a successful swap.
    pub table_reloaded: Option<TableReloaded>,
    /// Captured `FormulaDylibReloaded` after a successful `.dylib` swap.
    pub formula_dylib_reloaded: Option<FormulaDylibReloaded>,
}

impl PreFrameReloadLog {
    /// Starts an empty log for one pre-frame window.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            table_reloaded: None,
            formula_dylib_reloaded: None,
        }
    }
}

impl Default for PreFrameReloadLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Runs the strict pre-frame ordering required by IR-2.9.5 / FM-7.
///
/// Order: load `.dylib` → re-bake tables → swap registry snapshot → dispatch events.
pub fn run_pre_frame_reload(
    dylib_ok: bool,
    table_id: TableId,
    registry_version: u32,
    affected_rows: RowRange,
    formula_version: u32,
    affected_formulas: FormulaIdRange,
) -> PreFrameReloadLog {
    let mut log = PreFrameReloadLog::new();
    log.steps.push(ReloadStep::DylibLoad);
    if dylib_ok {
        log.steps.push(ReloadStep::TableRebake);
        log.steps.push(ReloadStep::SnapshotSwap);
        log.formula_dylib_reloaded = Some(FormulaDylibReloaded {
            version: formula_version,
            affected_formulas,
        });
        log.steps.push(ReloadStep::DispatchFormulaDylibReloaded);
        log.table_reloaded = Some(TableReloaded {
            table_id,
            registry_version,
            affected_rows,
        });
        log.steps.push(ReloadStep::DispatchTableReloaded);
    } else {
        log.steps.push(ReloadStep::TableRebake);
        log.steps.push(ReloadStep::SnapshotSwap);
        log.table_reloaded = Some(TableReloaded {
            table_id,
            registry_version,
            affected_rows,
        });
        log.steps.push(ReloadStep::DispatchTableReloaded);
    }
    log
}
