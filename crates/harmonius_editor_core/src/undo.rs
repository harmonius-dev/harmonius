//! Undo stack and reversible editor commands (`TC-15.1.3.*`).

use std::fmt;

/// Minimal ECS stand-in used by command unit tests.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct World {
    /// Scratch counter mutated by test commands.
    pub counter: i32,
}

/// Failure while executing or rolling back a command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandError {
    /// Command-specific failure string.
    Failed(&'static str),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::Failed(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for CommandError {}

/// Reversible editor command targeting [`World`].
pub trait EditorCommand: Send {
    /// Short human-readable label for UI and logs.
    fn description(&self) -> &str;
    /// Applies the command forward.
    fn execute(&mut self, world: &mut World) -> Result<(), CommandError>;
    /// Reverts [`execute`].
    fn undo(&mut self, world: &mut World) -> Result<(), CommandError>;
    /// Estimated heap footprint for history trimming.
    fn size_bytes(&self) -> usize {
        0
    }
    /// Optional additive replay payload for crash recovery snapshots.
    fn replay_payload(&self) -> Option<i32> {
        None
    }
}

/// Multi-command transaction applied as one undo step.
pub struct Transaction {
    desc: &'static str,
    cmds: Vec<Box<dyn EditorCommand>>,
}

impl Transaction {
    /// Wraps ordered commands that share one undo boundary.
    pub fn new(desc: &'static str, cmds: Vec<Box<dyn EditorCommand>>) -> Self {
        Self { desc, cmds }
    }
}

impl EditorCommand for Transaction {
    fn description(&self) -> &str {
        self.desc
    }

    fn execute(&mut self, world: &mut World) -> Result<(), CommandError> {
        for c in &mut self.cmds {
            c.execute(world)?;
        }
        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CommandError> {
        for c in self.cmds.iter_mut().rev() {
            c.undo(world)?;
        }
        Ok(())
    }

    fn size_bytes(&self) -> usize {
        self.cmds.len() * std::mem::size_of::<usize>()
    }

    fn replay_payload(&self) -> Option<i32> {
        None
    }
}

struct AddCommand(i32);

impl EditorCommand for AddCommand {
    fn description(&self) -> &str {
        "add"
    }

    fn execute(&mut self, world: &mut World) -> Result<(), CommandError> {
        world.counter += self.0;
        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CommandError> {
        world.counter -= self.0;
        Ok(())
    }

    fn replay_payload(&self) -> Option<i32> {
        Some(self.0)
    }
}

/// Linear undo/redo stack.
#[derive(Default)]
pub struct UndoStack {
    undo: Vec<Box<dyn EditorCommand>>,
    redo: Vec<Box<dyn EditorCommand>>,
}

impl UndoStack {
    /// Empty stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes a command and clears the redo branch.
    pub fn execute(
        &mut self,
        mut command: Box<dyn EditorCommand>,
        world: &mut World,
    ) -> Result<(), CommandError> {
        command.execute(world)?;
        self.redo.clear();
        self.undo.push(command);
        Ok(())
    }

    /// Pops one undo step if present.
    pub fn undo(&mut self, world: &mut World) -> Result<bool, CommandError> {
        let Some(mut cmd) = self.undo.pop() else {
            return Ok(false);
        };
        cmd.undo(world)?;
        self.redo.push(cmd);
        Ok(true)
    }

    /// Re-applies one redo step if present.
    pub fn redo(&mut self, world: &mut World) -> Result<bool, CommandError> {
        let Some(mut cmd) = self.redo.pop() else {
            return Ok(false);
        };
        cmd.execute(world)?;
        self.undo.push(cmd);
        Ok(true)
    }

    /// Serializes replayable commands with a [`AddCommand`] payload lane.
    pub fn serialize_history(&self) -> Result<Vec<u8>, CommandError> {
        let mut out = Vec::new();
        let mut payloads = Vec::new();
        for cmd in &self.undo {
            let Some(delta) = cmd.replay_payload() else {
                return Err(CommandError::Failed("non-replayable command in history"));
            };
            payloads.push(delta);
        }
        let n: u32 = payloads
            .len()
            .try_into()
            .map_err(|_| CommandError::Failed("overflow"))?;
        out.extend_from_slice(&n.to_le_bytes());
        for d in payloads {
            out.extend_from_slice(&d.to_le_bytes());
        }
        Ok(out)
    }

    /// Replays serialized additive commands into `world` and returns a fresh stack.
    pub fn load_and_replay(bytes: &[u8], world: &mut World) -> Result<Self, CommandError> {
        let mut stack = UndoStack::new();
        if bytes.len() < 4 {
            return Err(CommandError::Failed("truncated history"));
        }
        let count = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;
        let mut offset = 4usize;
        for _ in 0..count {
            if offset + 4 > bytes.len() {
                return Err(CommandError::Failed("truncated delta"));
            }
            let delta = i32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
            offset += 4;
            let mut cmd: Box<dyn EditorCommand> = Box::new(AddCommand(delta));
            cmd.execute(world)?;
            stack.undo.push(cmd);
        }
        Ok(stack)
    }
}

/// Builds a boxed additive command for tests and examples.
pub fn add_command(delta: i32) -> Box<dyn EditorCommand> {
    Box::new(AddCommand(delta))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_15_1_3_1_undo_single_command() {
        let mut w = World::default();
        let mut s = UndoStack::new();
        s.execute(add_command(5), &mut w).unwrap();
        assert_eq!(w.counter, 5);
        s.undo(&mut w).unwrap();
        assert_eq!(w.counter, 0);
    }

    #[test]
    fn tc_15_1_3_2_redo_single_command() {
        let mut w = World::default();
        let mut s = UndoStack::new();
        s.execute(add_command(2), &mut w).unwrap();
        s.undo(&mut w).unwrap();
        s.redo(&mut w).unwrap();
        assert_eq!(w.counter, 2);
    }

    #[test]
    fn tc_15_1_3_3_transaction_undo() {
        let mut w = World::default();
        let mut s = UndoStack::new();
        let tx = Transaction::new("pair", vec![add_command(1), add_command(2)]);
        s.execute(Box::new(tx), &mut w).unwrap();
        assert_eq!(w.counter, 3);
        s.undo(&mut w).unwrap();
        assert_eq!(w.counter, 0);
    }

    #[test]
    fn tc_15_1_3_4_undo_clears_redo() {
        let mut w = World::default();
        let mut s = UndoStack::new();
        s.execute(add_command(1), &mut w).unwrap();
        s.execute(add_command(2), &mut w).unwrap();
        s.undo(&mut w).unwrap();
        s.execute(add_command(10), &mut w).unwrap();
        assert_eq!(w.counter, 11);
        assert!(!s.redo(&mut w).unwrap());
    }

    #[test]
    fn tc_15_1_3_5_crash_recovery_replay() {
        let mut w = World::default();
        let mut s = UndoStack::new();
        s.execute(add_command(4), &mut w).unwrap();
        s.execute(add_command(7), &mut w).unwrap();
        let bytes = s.serialize_history().unwrap();
        let mut w2 = World::default();
        let s2 = UndoStack::load_and_replay(&bytes, &mut w2).unwrap();
        assert_eq!(w2.counter, 11);
        assert_eq!(s2.undo.len(), 2);
    }
}
