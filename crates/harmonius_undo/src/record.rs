//! Command records stored on the undo stack.

use crate::command::EditorCommand;
use crate::ids::{CommandId, TxId, UserId};
use crate::selection::SelectionSnapshot;

/// Fully captured command with selection and collaboration metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandRecord {
    /// Stable identifier for this record.
    pub id: CommandId,
    /// Transaction grouping identifier, if any.
    pub tx_id: Option<TxId>,
    /// Authoring collaborator.
    pub author: UserId,
    /// Command payload.
    pub command: EditorCommand,
    /// Selection prior to apply.
    pub before_selection: SelectionSnapshot,
    /// Selection after apply.
    pub after_selection: SelectionSnapshot,
    /// Cached byte accounting for budgeting.
    pub memory_bytes: u32,
    /// Monotonic tick when the command was committed.
    pub committed_at: u64,
}

impl CommandRecord {
    /// Builds a new record with cached memory accounting.
    #[must_use]
    pub fn new(
        id: CommandId,
        tx_id: Option<TxId>,
        author: UserId,
        command: EditorCommand,
        before_selection: SelectionSnapshot,
        after_selection: SelectionSnapshot,
        committed_at: u64,
    ) -> Self {
        let memory_bytes = command.memory_bytes();
        Self {
            id,
            tx_id,
            author,
            command,
            before_selection,
            after_selection,
            memory_bytes,
            committed_at,
        }
    }
}
