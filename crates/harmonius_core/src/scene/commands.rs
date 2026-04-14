//! Deferred hierarchy command buffer.

use super::Entity;

/// Deferred hierarchy operations applied at [`super::World::flush_hierarchy_commands`].
#[derive(Clone, Debug, PartialEq)]
pub enum HierarchyCommand {
    /// Sets `child` parent to `parent`, moving between parents when needed.
    SetParent {
        /// Child entity.
        child: Entity,
        /// Parent entity.
        parent: Entity,
    },
    /// Removes the parent from `child`, making it a root entity.
    RemoveParent {
        /// Child entity.
        child: Entity,
    },
    /// Recursively despawns `root` and its descendants.
    DespawnRecursive {
        /// Root entity.
        root: Entity,
    },
    /// Despawns `root` while reparenting children to the world root.
    DespawnOrphaning {
        /// Root entity.
        root: Entity,
    },
}

/// Queue of hierarchy commands.
#[derive(Debug, Default)]
pub struct HierarchyCommandBuffer {
    commands: Vec<HierarchyCommand>,
}

impl HierarchyCommandBuffer {
    /// Creates an empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Queues [`HierarchyCommand::SetParent`].
    pub fn set_parent(&mut self, child: Entity, parent: Entity) {
        self.commands
            .push(HierarchyCommand::SetParent { child, parent });
    }

    /// Queues [`HierarchyCommand::RemoveParent`].
    pub fn remove_parent(&mut self, child: Entity) {
        self.commands.push(HierarchyCommand::RemoveParent { child });
    }

    /// Queues [`HierarchyCommand::DespawnRecursive`].
    pub fn despawn_recursive(&mut self, root: Entity) {
        self.commands
            .push(HierarchyCommand::DespawnRecursive { root });
    }

    /// Queues [`HierarchyCommand::DespawnOrphaning`].
    pub fn despawn_orphaning(&mut self, root: Entity) {
        self.commands
            .push(HierarchyCommand::DespawnOrphaning { root });
    }

    pub(crate) fn drain(&mut self) -> std::vec::Drain<'_, HierarchyCommand> {
        self.commands.drain(..)
    }
}
