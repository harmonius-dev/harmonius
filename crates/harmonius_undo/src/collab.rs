//! Collaborative undo bookkeeping.

use std::collections::BTreeMap;

use crate::ids::UserId;
use crate::record::CommandRecord;
use crate::stack::{UndoError, UndoStack};
use crate::world::TestWorld;

/// Conflicts surfaced when collaborators compete for overlapping undo steps.
#[derive(Debug, Eq, PartialEq)]
pub enum UndoConflict {
    /// Two collaborators targeted overlapping entities for simultaneous undo.
    TouchesOverlap,
}

/// Errors returned by collaborative undo helpers.
#[derive(Debug)]
pub enum CollabError {
    /// Underlying single-user undo failure.
    Undo(UndoError),
    /// Collaborative conflict prevented the operation.
    Conflict(UndoConflict),
    /// The requested collaborator is not registered.
    UnknownUser,
}

impl From<UndoError> for CollabError {
    fn from(value: UndoError) -> Self {
        Self::Undo(value)
    }
}

/// Coordinates separate per-user undo stacks against a shared world.
#[derive(Debug)]
pub struct CollabSession {
    /// Shared editor world authoritative for this harness.
    pub world: TestWorld,
    /// Per-user undo stacks.
    pub stacks: BTreeMap<UserId, UndoStack>,
}

impl Default for CollabSession {
    fn default() -> Self {
        Self::new()
    }
}

impl CollabSession {
    /// Creates an empty collaborative harness.
    #[must_use]
    pub fn new() -> Self {
        Self {
            world: TestWorld::default(),
            stacks: BTreeMap::new(),
        }
    }

    /// Registers a stack for a collaborator.
    pub fn insert_stack(&mut self, user: UserId, stack: UndoStack) {
        self.stacks.insert(user, stack);
    }

    /// Applies and records a command for `user` against the shared world.
    pub fn push_for_user(
        &mut self,
        user: UserId,
        record: CommandRecord,
    ) -> Result<(), CollabError> {
        let stack = self.stacks.get_mut(&user).ok_or(CollabError::UnknownUser)?;
        stack.push_record(&mut self.world, record)?;
        Ok(())
    }

    /// Undoes for `user`, rejecting when another collaborator has a conflicting next undo.
    pub fn undo_for(&mut self, user: UserId) -> Result<(), CollabError> {
        let my_touches = {
            let stack = self.stacks.get(&user).ok_or(CollabError::UnknownUser)?;
            stack
                .peek_next_undo_touches()
                .ok_or(CollabError::Undo(UndoError::NothingToUndo))?
        };

        for (other_user, stack) in &self.stacks {
            if *other_user == user {
                continue;
            }
            if let Some(their) = stack.peek_next_undo_touches() {
                if UndoStack::touches_overlap(&my_touches, &their) {
                    return Err(CollabError::Conflict(UndoConflict::TouchesOverlap));
                }
            }
        }

        let stack = self.stacks.get_mut(&user).ok_or(CollabError::UnknownUser)?;
        stack.undo(&mut self.world)?;
        Ok(())
    }
}
