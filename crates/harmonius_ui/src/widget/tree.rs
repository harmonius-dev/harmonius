//! Retained widget hierarchy and structural edits (`R-10.1.1`, `TC-10.1.1.1`).
//!
//! Owns widget slots, parent/child links, and structural dirty propagation.

use crate::widget::id::Entity;
use crate::widget::node::{DirtyFlags, WidgetKey, WidgetKind, WidgetNode};
use crate::widget::WidgetId;

/// Parent slot is missing or stale.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnknownParentError;

#[derive(Debug)]
struct WidgetRecord {
    generation: u32,
    node: WidgetNode,
    parent: Option<WidgetId>,
    children: Vec<WidgetId>,
}

impl WidgetRecord {
    fn matches_id(&self, id: WidgetId) -> bool {
        self.generation == id.generation()
    }
}

/// Retained widget forest with a single distinguished root used by screen-space UI.
#[derive(Debug)]
pub struct WidgetTree {
    records: Vec<WidgetRecord>,
    root: WidgetId,
}

impl WidgetTree {
    /// Builds a tree containing only a root [`WidgetKind::Panel`] with no children — the starting
    /// point for `TC-10.1.1.1` (“empty” structural child list).
    #[must_use]
    pub fn new_with_root_panel() -> Self {
        let root = Entity::new(0, 0);
        let root_node = WidgetNode {
            kind: WidgetKind::Panel,
            key: WidgetKey::Index(0),
            dirty: DirtyFlags::empty(),
        };
        Self {
            records: vec![WidgetRecord {
                generation: 0,
                node: root_node,
                parent: None,
                children: Vec::new(),
            }],
            root,
        }
    }

    /// Root widget created by [`Self::new_with_root_panel`].
    #[must_use]
    pub fn root(&self) -> WidgetId {
        self.root
    }

    /// Returns `true` when `id` refers to a live slot with a matching generation.
    #[must_use]
    pub fn contains(&self, id: WidgetId) -> bool {
        let idx = id.index() as usize;
        self.records
            .get(idx)
            .is_some_and(|slot| slot.matches_id(id))
    }

    /// Borrows a widget node when the handle is still live.
    #[must_use]
    pub fn get(&self, id: WidgetId) -> Option<&WidgetNode> {
        let idx = id.index() as usize;
        self.records
            .get(idx)
            .filter(|slot| slot.matches_id(id))
            .map(|slot| &slot.node)
    }

    /// Parent link for a live widget, if any (`None` for the distinguished root).
    #[must_use]
    pub fn parent(&self, id: WidgetId) -> Option<WidgetId> {
        let idx = id.index() as usize;
        let slot = self.records.get(idx)?;
        if !slot.matches_id(id) {
            return None;
        }
        slot.parent
    }

    /// Appends a child under `parent`, marking the parent with [`DirtyFlags::CHILDREN`].
    ///
    /// # Errors
    ///
    /// Returns [`UnknownParentError`] when `parent` is not a live widget in this tree.
    pub fn insert_child(
        &mut self,
        parent: WidgetId,
        kind: WidgetKind,
        key: WidgetKey,
    ) -> Result<WidgetId, UnknownParentError> {
        let parent_index = parent.index() as usize;
        let Some(parent_record) = self.records.get(parent_index) else {
            return Err(UnknownParentError);
        };
        if !parent_record.matches_id(parent) {
            return Err(UnknownParentError);
        }

        let child_index = self.records.len() as u32;
        let child_id = Entity::new(child_index, 0);
        self.records.push(WidgetRecord {
            generation: 0,
            node: WidgetNode {
                kind,
                key,
                dirty: DirtyFlags::empty(),
            },
            parent: Some(parent),
            children: Vec::new(),
        });

        let parent_slot = &mut self.records[parent_index];
        parent_slot.children.push(child_id);
        parent_slot.node.dirty |= DirtyFlags::CHILDREN;
        Ok(child_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-10.1.1.1` — insert into an empty child list under the root panel.
    #[test]
    fn tc_10_1_1_1_insert_sets_parent_children_dirty() {
        let mut tree = WidgetTree::new_with_root_panel();
        let root = tree.root();
        let child = tree
            .insert_child(root, WidgetKind::Label, WidgetKey::Index(1))
            .expect("root is valid parent");

        assert!(tree.contains(child));
        let root_node = tree.get(root).expect("root node");
        assert!(root_node.dirty.contains(DirtyFlags::CHILDREN));
    }
}
