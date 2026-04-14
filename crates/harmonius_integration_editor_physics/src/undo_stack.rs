//! Minimal undo stack for editor command integration tests.

use std::fmt;

use crate::command::{CommandError, EditorCommand};
use crate::world::World;

/// Linear undo with optional redo cursor (TC-IR-5.4.1.4).
#[derive(Default)]
pub struct UndoStack {
    done: Vec<Box<dyn EditorCommand>>,
    undone: Vec<Box<dyn EditorCommand>>,
}

impl fmt::Debug for UndoStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UndoStack")
            .field("done_len", &self.done.len())
            .field("undone_len", &self.undone.len())
            .finish()
    }
}

impl UndoStack {
    /// Creates an empty stack.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes `cmd`, stores it on success.
    pub fn execute(
        &mut self,
        world: &mut World,
        mut cmd: Box<dyn EditorCommand>,
    ) -> Result<(), CommandError> {
        cmd.execute(world)?;
        self.done.push(cmd);
        self.undone.clear();
        Ok(())
    }

    /// Pops the last successful command and calls `undo` (TC-IR-5.4.N6 when empty: `Ok`).
    pub fn undo(&mut self, world: &mut World) -> Result<(), CommandError> {
        let Some(mut cmd) = self.done.pop() else {
            return Ok(());
        };
        cmd.undo(world)?;
        self.undone.push(cmd);
        Ok(())
    }

    /// Re-applies the most recently undone command.
    pub fn redo(&mut self, world: &mut World) -> Result<(), CommandError> {
        let Some(mut cmd) = self.undone.pop() else {
            return Ok(());
        };
        cmd.execute(world)?;
        self.done.push(cmd);
        Ok(())
    }
}
