//! Selection change notifications for editor observers.

use smallvec::SmallVec;

use crate::types::{EditorWorldId, EntityRef};

/// Event emitted after a successful selection mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionChanged {
    /// Entities added to the selection in this step.
    pub added: SmallVec<[EntityRef; 4]>,
    /// Entities removed from the selection in this step.
    pub removed: SmallVec<[EntityRef; 4]>,
    /// Monotonic selection revision after the mutation.
    pub revision: u64,
    /// Editor world that produced the change.
    pub world: EditorWorldId,
}

impl SelectionChanged {
    /// Builds a change record for a pure add of one entity.
    pub(crate) fn add_one(world: EditorWorldId, revision: u64, entity: EntityRef) -> Self {
        let mut added = SmallVec::<[EntityRef; 4]>::new();
        added.push(entity);
        Self {
            added,
            removed: SmallVec::new(),
            revision,
            world,
        }
    }

    /// Builds a change record for a pure remove of one entity.
    pub(crate) fn remove_one(world: EditorWorldId, revision: u64, entity: EntityRef) -> Self {
        let mut removed = SmallVec::<[EntityRef; 4]>::new();
        removed.push(entity);
        Self {
            added: SmallVec::new(),
            removed,
            revision,
            world,
        }
    }

    /// Builds a change record replacing the previous entity set.
    pub(crate) fn replace(
        world: EditorWorldId,
        revision: u64,
        before: &[EntityRef],
        after: &[EntityRef],
    ) -> Self {
        let mut removed = SmallVec::<[EntityRef; 4]>::new();
        for entity in before {
            if after.binary_search(entity).is_err() {
                removed.push(*entity);
            }
        }
        let mut added = SmallVec::<[EntityRef; 4]>::new();
        for entity in after {
            if before.binary_search(entity).is_err() {
                added.push(*entity);
            }
        }
        Self {
            added,
            removed,
            revision,
            world,
        }
    }

    /// Cleared selection removes every previously selected entity.
    pub(crate) fn cleared(world: EditorWorldId, revision: u64, before: &[EntityRef]) -> Self {
        Self {
            added: SmallVec::new(),
            removed: SmallVec::from_slice(before),
            revision,
            world,
        }
    }
}
