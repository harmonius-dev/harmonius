//! Minimal editor world used by unit tests in this crate.

use std::collections::{BTreeMap, BTreeSet};

use crate::selection::{EntityRef, SelectionSnapshot};

/// Deterministic stand-in for `EditorWorld` in undo tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestWorld {
    /// Monotonic counter used by generic apply/revert tests.
    pub counter: i32,
    /// Component values keyed by entity id.
    pub values: BTreeMap<EntityRef, i32>,
    /// Active entity ids.
    pub entities: BTreeSet<EntityRef>,
    /// Simple text fields keyed by entity id.
    pub text: BTreeMap<EntityRef, String>,
    /// Parent pointers for hierarchy tests.
    pub parents: BTreeMap<EntityRef, Option<EntityRef>>,
    /// Current editor selection.
    pub selection: SelectionSnapshot,
}

impl Default for TestWorld {
    fn default() -> Self {
        Self {
            counter: 0,
            values: BTreeMap::new(),
            entities: BTreeSet::new(),
            text: BTreeMap::new(),
            parents: BTreeMap::new(),
            selection: SelectionSnapshot::empty(),
        }
    }
}

impl TestWorld {
    /// Returns the value for `entity` if present.
    #[must_use]
    pub fn get_value(&self, entity: EntityRef) -> Option<i32> {
        self.values.get(&entity).copied()
    }
}
