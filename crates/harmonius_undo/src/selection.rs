//! Selection snapshots coupled to undo steps.
#![allow(missing_docs)] // rkyv-generated archived companion types omit doc comments.

use rkyv::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;

/// Entity identifier used by editor selection bookkeeping.
#[derive(
    Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct EntityRef(pub u64);

/// Captures editor selection before and after a command executes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSnapshot {
    /// Primary selected entities.
    pub entities: SmallVec<[EntityRef; 8]>,
}

impl SelectionSnapshot {
    /// Creates an empty selection snapshot.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            entities: SmallVec::new(),
        }
    }

    /// Creates a snapshot containing a single entity.
    #[must_use]
    pub fn single(entity: EntityRef) -> Self {
        let mut entities = SmallVec::new();
        entities.push(entity);
        Self { entities }
    }
}
