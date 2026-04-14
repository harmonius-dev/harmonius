//! Primary selection storage and mutation API.

use smallvec::SmallVec;

use crate::events::SelectionChanged;
use crate::types::{EditorWorldId, EntityRef, SubObjectElement};

/// Immutable snapshot of [`SelectionState`] for undo integration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSnapshot {
    /// Selected entities in stable sorted order.
    pub entities: SmallVec<[EntityRef; 16]>,
    /// Primary entity for gizmo and inspector focus.
    pub primary: Option<EntityRef>,
    /// Selected sub-objects.
    pub sub_objects: SmallVec<[SubObjectElement; 16]>,
}

/// Per-world selection set with deterministic ordering.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelectionState {
    entities: SmallVec<[EntityRef; 16]>,
    primary: Option<EntityRef>,
    revision: u64,
    sub_objects: SmallVec<[SubObjectElement; 16]>,
}

impl SelectionState {
    /// Inserts `entity` in sorted order when absent.
    ///
    /// Returns `true` when the selection changed.
    pub fn add(&mut self, entity: EntityRef) -> bool {
        match self.entities.binary_search(&entity) {
            Ok(_) => false,
            Err(idx) => {
                self.entities.insert(idx, entity);
                if self.primary.is_none() {
                    self.primary = Some(entity);
                }
                self.revision += 1;
                true
            }
        }
    }

    /// Adds `entity` and returns a [`SelectionChanged`] notification when the revision advances.
    pub fn add_notify(
        &mut self,
        world: EditorWorldId,
        entity: EntityRef,
    ) -> Option<SelectionChanged> {
        if !self.add(entity) {
            return None;
        }
        let revision = self.revision;
        Some(SelectionChanged::add_one(world, revision, entity))
    }

    /// Removes `entity` when present.
    ///
    /// Returns `true` when the selection changed.
    pub fn remove(&mut self, entity: EntityRef) -> bool {
        match self.entities.binary_search(&entity) {
            Ok(idx) => {
                self.entities.remove(idx);
                self.sub_objects.retain(|so| so.entity != entity);
                if self.primary == Some(entity) {
                    self.primary = self.entities.first().copied();
                }
                self.revision += 1;
                true
            }
            Err(_) => false,
        }
    }

    /// Removes `entity` and emits [`SelectionChanged`] when successful.
    pub fn remove_notify(
        &mut self,
        world: EditorWorldId,
        entity: EntityRef,
    ) -> Option<SelectionChanged> {
        if !self.remove(entity) {
            return None;
        }
        let revision = self.revision;
        Some(SelectionChanged::remove_one(world, revision, entity))
    }

    /// Clears entities, sub-objects, and primary selection.
    pub fn clear(&mut self) -> bool {
        if self.entities.is_empty() && self.sub_objects.is_empty() && self.primary.is_none() {
            return false;
        }
        self.entities.clear();
        self.sub_objects.clear();
        self.primary = None;
        self.revision += 1;
        true
    }

    /// Clears the selection and emits [`SelectionChanged`] with every removed entity.
    pub fn clear_notify(&mut self, world: EditorWorldId) -> Option<SelectionChanged> {
        if self.entities.is_empty() {
            return None;
        }
        let before = self.entities.clone();
        self.entities.clear();
        self.sub_objects.clear();
        self.primary = None;
        self.revision += 1;
        let revision = self.revision;
        Some(SelectionChanged::cleared(
            world,
            revision,
            before.as_slice(),
        ))
    }

    /// Adds `entity` when absent or removes it when present.
    ///
    /// Returns `true` when the selection changed.
    pub fn toggle(&mut self, entity: EntityRef) -> bool {
        match self.entities.binary_search(&entity) {
            Ok(_) => self.remove(entity),
            Err(_) => self.add(entity),
        }
    }

    /// Replaces the entity selection with `entities` (sub-objects cleared).
    pub fn replace(&mut self, entities: impl IntoIterator<Item = EntityRef>) {
        self.entities.clear();
        self.sub_objects.clear();
        self.primary = None;
        for entity in entities {
            let _ = self.add(entity);
        }
    }

    /// Replaces selection and emits a consolidated diff against the prior entity list.
    pub fn replace_notify(
        &mut self,
        world: EditorWorldId,
        entities: impl IntoIterator<Item = EntityRef>,
    ) -> SelectionChanged {
        let before = self.entities.clone();
        self.entities.clear();
        self.sub_objects.clear();
        self.primary = None;
        for entity in entities {
            let _ = self.add(entity);
        }
        let after = self.entities.clone();
        let revision = self.revision;
        SelectionChanged::replace(world, revision, before.as_slice(), after.as_slice())
    }

    /// Captures a snapshot suitable for undo stacks.
    pub fn snapshot(&self) -> SelectionSnapshot {
        SelectionSnapshot {
            entities: self.entities.clone(),
            primary: self.primary,
            sub_objects: self.sub_objects.clone(),
        }
    }

    /// Restores a prior snapshot, bumping the revision exactly once.
    pub fn restore(&mut self, snap: SelectionSnapshot) {
        self.entities = snap.entities;
        self.sub_objects = snap.sub_objects;
        self.primary = snap.primary;
        self.revision += 1;
    }

    /// Monotonically increasing revision counter for viewport synchronization.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Sorted entity selection (read-only).
    pub fn entities(&self) -> &[EntityRef] {
        self.entities.as_slice()
    }

    /// Primary entity, if any.
    pub fn primary(&self) -> Option<EntityRef> {
        self.primary
    }

    /// Returns `true` when `entity` is selected.
    pub fn contains(&self, entity: EntityRef) -> bool {
        self.entities.binary_search(&entity).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(id: u32) -> EntityRef {
        EntityRef(id)
    }

    #[test]
    fn test_selection_add_new_entity() {
        let mut s = SelectionState::default();
        assert!(s.add(e(1)));
        assert!(s.contains(e(1)));
    }

    #[test]
    fn test_selection_add_duplicate_noop() {
        let mut s = SelectionState::default();
        assert!(s.add(e(1)));
        let rev = s.revision();
        assert!(!s.add(e(1)));
        assert_eq!(s.revision(), rev);
    }

    #[test]
    fn test_selection_remove() {
        let mut s = SelectionState::default();
        assert!(s.add(e(1)));
        assert!(s.remove(e(1)));
        assert!(!s.contains(e(1)));
    }

    #[test]
    fn test_selection_clear() {
        let mut s = SelectionState::default();
        assert!(s.add(e(1)));
        assert!(s.clear());
        assert!(s.entities().is_empty());
        assert!(s.primary().is_none());
    }

    #[test]
    fn test_selection_toggle_add_remove() {
        let mut s = SelectionState::default();
        assert!(s.toggle(e(2)));
        assert!(s.contains(e(2)));
        assert!(s.toggle(e(2)));
        assert!(!s.contains(e(2)));
    }

    #[test]
    fn test_selection_replace() {
        let mut s = SelectionState::default();
        assert!(s.add(e(1)));
        s.replace([e(3), e(4), e(5)]);
        assert!(!s.contains(e(1)));
        assert!(s.contains(e(3)));
        assert!(s.contains(e(4)));
        assert!(s.contains(e(5)));
    }

    #[test]
    fn test_selection_primary_first_added() {
        let mut s = SelectionState::default();
        assert!(s.add(e(10)));
        assert_eq!(s.primary(), Some(e(10)));
        assert!(s.add(e(11)));
        assert_eq!(s.primary(), Some(e(10)));
    }

    #[test]
    fn test_selection_snapshot_roundtrip() {
        let mut s = SelectionState::default();
        assert!(s.add(e(7)));
        let snap = s.snapshot();
        s.clear();
        s.restore(snap);
        assert!(s.contains(e(7)));
    }

    #[test]
    fn test_selection_revision_bumps() {
        let mut s = SelectionState::default();
        let r0 = s.revision();
        assert!(s.add(e(1)));
        assert!(s.revision() > r0);
        let r1 = s.revision();
        assert!(s.remove(e(1)));
        assert!(s.revision() > r1);
    }

    #[test]
    fn test_selection_sorted_vec_on_insert() {
        let mut s = SelectionState::default();
        assert!(s.add(e(3)));
        assert!(s.add(e(1)));
        assert!(s.add(e(2)));
        assert_eq!(s.entities(), &[e(1), e(2), e(3)]);
    }

    #[test]
    fn test_selection_changed_emitted_on_add() {
        let mut s = SelectionState::default();
        let world = EditorWorldId(99);
        let ev = s.add_notify(world, e(5)).expect("event");
        assert_eq!(ev.added.as_slice(), &[e(5)]);
        assert!(ev.removed.is_empty());
    }

    #[test]
    fn test_selection_changed_emitted_on_remove() {
        let mut s = SelectionState::default();
        let world = EditorWorldId(1);
        assert!(s.add(e(9)));
        let ev = s.remove_notify(world, e(9)).expect("event");
        assert!(ev.added.is_empty());
        assert_eq!(ev.removed.as_slice(), &[e(9)]);
    }

    #[test]
    fn test_selection_changed_added_removed() {
        let mut s = SelectionState::default();
        let world = EditorWorldId(2);
        assert!(s.add(e(1)));
        assert!(s.add(e(2)));
        let ev = s.replace_notify(world, [e(2), e(3)]);
        assert!(ev.added.contains(&e(3)));
        assert!(ev.removed.contains(&e(1)));
    }
}
