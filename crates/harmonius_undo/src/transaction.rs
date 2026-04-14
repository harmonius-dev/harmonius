//! Transaction grouping for undo steps.

use crate::ids::TxId;
use crate::record::CommandRecord;
use crate::stack::{UndoError, UndoStack};
use crate::world::TestWorld;

/// Marker returned when a transaction commits successfully.
#[derive(Debug)]
pub struct TransactionEnd;

/// RAII helper that batches pushes into one undo step.
pub struct TransactionGuard<'a> {
    stack: &'a mut UndoStack,
    label: &'static str,
    pending: Vec<CommandRecord>,
    tx_id: TxId,
    committed: bool,
}

impl<'a> TransactionGuard<'a> {
    pub(crate) fn new(stack: &'a mut UndoStack, label: &'static str, tx_id: TxId) -> Self {
        Self {
            stack,
            label,
            pending: Vec::new(),
            tx_id,
            committed: false,
        }
    }

    /// Applies `record` to `world` and buffers it for the transaction.
    pub fn push(
        &mut self,
        world: &mut TestWorld,
        mut record: CommandRecord,
    ) -> Result<(), UndoError> {
        record.command.apply(world)?;
        record.tx_id = Some(self.tx_id);
        self.pending.push(record);
        Ok(())
    }

    /// Commits the transaction as a single undo step.
    pub fn commit(mut self) -> Result<TransactionEnd, UndoError> {
        let label = self.label;
        let records = core::mem::take(&mut self.pending);
        self.stack.push_transaction(label, records)?;
        self.committed = true;
        Ok(TransactionEnd)
    }

    /// Reverts every buffered command without committing history.
    pub fn rollback(mut self, world: &mut TestWorld) {
        for record in self.pending.drain(..).rev() {
            let _ = record.command.revert(world);
        }
        self.committed = true;
    }
}
