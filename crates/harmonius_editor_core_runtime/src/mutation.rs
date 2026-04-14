//! `EditorMutation` messages carried across the `EventBridge`.

/// Tag for editor-originated edits applied to the runtime world after the bridge drains.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditorMutationKind {
    /// Spawns a runtime entity with the given id (minimal stand-in).
    SpawnEntity {
        /// New entity id.
        id: crate::world::EntityId,
    },
    /// Removes an entity from the runtime world.
    DespawnEntity {
        /// Target entity id.
        id: crate::world::EntityId,
    },
    /// Attaches bytes for a logical component type id.
    InsertComponent {
        /// Target entity.
        entity: crate::world::EntityId,
        /// Stable component type id for tests.
        component_id: u32,
        /// Serialized payload (test harness bytes, not rkyv yet).
        bytes: Vec<u8>,
    },
    /// Overwrites component bytes for an existing component slot.
    UpdateComponent {
        /// Target entity.
        entity: crate::world::EntityId,
        /// Stable component type id for tests.
        component_id: u32,
        /// Serialized payload (test harness bytes, not rkyv yet).
        bytes: Vec<u8>,
    },
    /// Drops a component slot from an entity without despawning the entity.
    RemoveComponent {
        /// Target entity.
        entity: crate::world::EntityId,
        /// Stable component type id for tests.
        component_id: u32,
    },
    /// Placeholder for resource writes (integration stub).
    SetResource {
        /// Resource key for tests.
        key: u32,
        /// Opaque payload.
        value: Vec<u8>,
    },
    /// Placeholder scene stack push.
    PushScene {
        /// Scene id for tests.
        scene: u32,
    },
    /// Placeholder scene stack pop.
    PopScene,
}

/// One staged editor edit addressed to the runtime world.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditorMutation {
    /// Stable id used for last-write-wins dedupe tests (`FM-2`).
    pub mutation_id: u64,
    /// Work performed on the runtime world after bridging.
    pub kind: EditorMutationKind,
}
