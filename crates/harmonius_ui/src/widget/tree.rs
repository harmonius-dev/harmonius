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
/// Handle is stale, missing, or not a leaf.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemoveLeafError;

#[derive(Debug, Default)]
struct SlotPool {
    free_indices: Vec<u32>,
}

impl SlotPool {
    fn recycle(&mut self, index: u32) {
        self.free_indices.push(index);
    }

    fn pop_free(&mut self) -> Option<u32> {
        self.free_indices.pop()
    }
}

/// Retained widget forest with structural pooling for stable slot reuse.
#[derive(Debug)]
pub struct WidgetTree {
    records: Vec<WidgetRecord>,
    root: WidgetId,
    pool: SlotPool,
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
            pool: SlotPool::default(),
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

        let (_slot_index, child_id) = if let Some(reuse_index) = self.pool.pop_free() {
            let idx = reuse_index as usize;
            let generation = self.records[idx].generation;
            let child_id = Entity::new(reuse_index, generation);
            self.records[idx] = WidgetRecord {
                generation,
                node: WidgetNode {
                    kind,
                    key,
                    dirty: DirtyFlags::empty(),
                },
                parent: Some(parent),
                children: Vec::new(),
            };
            (reuse_index, child_id)
        } else {
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
            (child_index, child_id)
        };

        let parent_slot = &mut self.records[parent_index];
        parent_slot.children.push(child_id);
        parent_slot.node.dirty |= DirtyFlags::CHILDREN;
        Ok(child_id)
    }

    /// Removes a leaf widget, invalidates the handle, and returns its slot index to the free list.
    ///
    /// # Errors
    ///
    /// [`RemoveLeafError`] when `id` is not live, is the distinguished root, or has children.
    pub fn remove_leaf(&mut self, id: WidgetId) -> Result<(), RemoveLeafError> {
        if id == self.root {
            return Err(RemoveLeafError);
        }
        let idx = id.index() as usize;
        let Some(record) = self.records.get(idx) else {
            return Err(RemoveLeafError);
        };
        if !record.matches_id(id) {
            return Err(RemoveLeafError);
        }
        if !record.children.is_empty() {
            return Err(RemoveLeafError);
        }
        let parent_id = record.parent.ok_or(RemoveLeafError)?;

        let parent_index = parent_id.index() as usize;
        let parent_children = &mut self.records[parent_index].children;
        let pos = parent_children
            .iter()
            .position(|&c| c == id)
            .ok_or(RemoveLeafError)?;
        parent_children.remove(pos);
        self.records[parent_index].node.dirty |= DirtyFlags::CHILDREN;

        let slot = &mut self.records[idx];
        slot.generation = slot.generation.saturating_add(1);
        slot.parent = None;
        slot.children.clear();
        slot.node = WidgetNode {
            kind: WidgetKind::Panel,
            key: WidgetKey::Index(0),
            dirty: DirtyFlags::empty(),
        };
        self.pool.recycle(id.index());
        Ok(())
    }

    /// Returns indices currently sitting in the recycle pool (for `TC-10.1.3.1` style checks).
    #[must_use]
    pub fn pool_free_len(&self) -> usize {
        self.pool.free_indices.len()
    }

    /// Borrow the ordered child list for a live widget.
    #[must_use]
    pub fn children(&self, id: WidgetId) -> Option<&[WidgetId]> {
        let idx = id.index() as usize;
        let slot = self.records.get(idx)?;
        if !slot.matches_id(id) {
            return None;
        }
        Some(slot.children.as_slice())
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

    /// `TC-10.1.1.2` — remove leaf, parent dirtied, slot recycled.
    #[test]
    fn tc_10_1_1_2_remove_leaf_recycles_slot() {
        let mut tree = WidgetTree::new_with_root_panel();
        let root = tree.root();
        let child = tree
            .insert_child(root, WidgetKind::Label, WidgetKey::Index(1))
            .expect("insert");

        tree.remove_leaf(child).expect("leaf removal");
        assert!(!tree.contains(child));
        let root_node = tree.get(root).expect("root");
        assert!(root_node.dirty.contains(DirtyFlags::CHILDREN));
        assert_eq!(tree.pool_free_len(), 1);
    }

    /// `TC-10.1.3.1` — pool hands back the same slot indices after bulk release.
    #[test]
    fn tc_10_1_3_1_pool_reuses_released_indices() {
        let mut tree = WidgetTree::new_with_root_panel();
        let root = tree.root();
        let mut handles = Vec::new();
        for i in 0..10_u32 {
            handles.push(
                tree
                    .insert_child(root, WidgetKind::Label, WidgetKey::Index(i))
                    .expect("insert"),
            );
        }
        let indices: std::collections::HashSet<u32> =
            handles.iter().map(|h| h.index()).collect();
        for h in handles {
            tree.remove_leaf(h).expect("remove leaf");
        }
        assert_eq!(tree.pool_free_len(), 10);

        let mut new_indices = std::collections::HashSet::new();
        for i in 0..10_u32 {
            let id = tree
                .insert_child(root, WidgetKind::Button, WidgetKey::Index(100 + i))
                .expect("reinsert");
            new_indices.insert(id.index());
        }
        assert_eq!(indices, new_indices);
        assert_eq!(tree.pool_free_len(), 0);
    }
}
