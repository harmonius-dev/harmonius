//! Hot reload bookkeeping (in-memory swap + journal events).

use crate::diagnostics::ValidationError;
use crate::ids::TableId;
use crate::registry::TableRegistry;
use crate::table::DataTable;
use crate::validation::validate_table;

/// Emitted after a successful swap.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableReloaded {
    /// Table id that changed.
    pub table: TableId,
    /// Previous version number.
    pub old_version: u64,
    /// New version number.
    pub new_version: u64,
}

/// Emitted when reload validation fails.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationFailed {
    /// Table id that failed validation.
    pub table: TableId,
    /// Collected errors.
    pub errors: Vec<ValidationError>,
}

/// Hot reload journal entry.
#[derive(Clone, Debug, PartialEq)]
pub enum HotReloadEvent {
    /// Successful reload.
    Reloaded(TableReloaded),
    /// Validation blocked reload.
    Failed(ValidationFailed),
}

/// Rollback stack entry for one table.
#[derive(Clone, Debug)]
struct Stash {
    table: DataTable,
}

/// Tracks prior versions for rollback testing.
#[derive(Clone, Debug, Default)]
pub struct HotReloadStack {
    stacks: Vec<Vec<Stash>>,
}

impl HotReloadStack {
    /// Ensures storage exists for `id`.
    fn ensure(&mut self, id: TableId) {
        let idx = id.0 as usize;
        if idx >= self.stacks.len() {
            self.stacks.resize_with(idx + 1, Vec::new);
        }
    }

    /// Validates and swaps `new_table` into `registry`, journaling outcomes.
    pub fn try_reload(
        &mut self,
        registry: &mut TableRegistry,
        id: TableId,
        new_table: DataTable,
        journal: &mut Vec<HotReloadEvent>,
    ) -> Result<(), ValidationFailed> {
        let errors = validate_table(&new_table, registry);
        if !errors.is_empty() {
            let failed = ValidationFailed {
                table: id,
                errors: errors.clone(),
            };
            journal.push(HotReloadEvent::Failed(failed.clone()));
            return Err(failed);
        }
        let old_version = registry.get(id).map(DataTable::version).unwrap_or(0);
        let new_version = new_table.version();
        let prev = registry.swap(id, new_table);
        self.ensure(id);
        let idx = id.0 as usize;
        if let Some(p) = prev {
            self.stacks[idx].push(Stash { table: p });
        }
        journal.push(HotReloadEvent::Reloaded(TableReloaded {
            table: id,
            old_version,
            new_version,
        }));
        Ok(())
    }

    /// Restores the previous stashed table version, if any.
    pub fn rollback(
        &mut self,
        registry: &mut TableRegistry,
        id: TableId,
    ) -> Result<(), HotReloadError> {
        let idx = id.0 as usize;
        let Some(stack) = self.stacks.get_mut(idx) else {
            return Err(HotReloadError::NoPreviousVersion);
        };
        let Some(prev) = stack.pop() else {
            return Err(HotReloadError::NoPreviousVersion);
        };
        registry.swap(id, prev.table);
        Ok(())
    }
}

/// Rollback failures.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HotReloadError {
    /// No previous table is stashed.
    #[error("no previous version")]
    NoPreviousVersion,
    /// Registry did not contain the table id.
    #[error("table not found")]
    TableNotFound,
}
