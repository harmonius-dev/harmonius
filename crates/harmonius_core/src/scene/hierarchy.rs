//! Parent/children derived views and hierarchy events.

use smallvec::SmallVec;

use super::Entity;

/// Points from a child entity to its parent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Parent {
    /// Parent entity handle.
    pub entity: Entity,
}

/// Ordered list of child entities stored on the parent.
#[derive(Clone, Debug, PartialEq)]
pub struct Children {
    /// Child entities in insertion order.
    pub entities: SmallVec<[Entity; 8]>,
}

impl Children {
    /// Creates an empty child list.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entities: SmallVec::new(),
        }
    }

    /// Number of children.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns `true` when no children are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Iterates child handles.
    pub fn iter(&self) -> std::slice::Iter<'_, Entity> {
        self.entities.iter()
    }

    /// Returns `true` when `entity` is a child.
    #[must_use]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::new()
    }
}

/// Events emitted when hierarchy edges change.
#[derive(Clone, Debug, PartialEq)]
pub enum HierarchyEvent {
    /// A child was attached to a parent.
    ChildAdded {
        /// Child entity.
        child: Entity,
        /// Parent entity.
        parent: Entity,
    },
    /// A child was detached from a parent.
    ChildRemoved {
        /// Child entity.
        child: Entity,
        /// Former parent entity.
        old_parent: Entity,
    },
    /// A child moved between parents.
    ChildMoved {
        /// Child entity.
        child: Entity,
        /// Former parent entity.
        old_parent: Entity,
        /// New parent entity.
        new_parent: Entity,
    },
}

/// Policy for children when a parent is despawned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrphanPolicy {
    /// Recursively despawn descendants.
    Cascade,
    /// Reparent children to the world root.
    Reparent,
}
