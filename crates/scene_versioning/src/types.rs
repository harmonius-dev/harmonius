//! Wire-level scene document and diff shapes.

/// Raw component payload bytes.
pub type Blob = Vec<u8>;

/// Stable entity identifier persisted in scene archives.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StableEntityId(pub u64);

/// Archetype discriminator carried on each entity record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArchetypeId(pub u32);

/// Component type identifier from engine codegen tables.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentTypeId(pub u32);

/// Sentinel component id marking entity-level merge conflicts.
pub const ENTITY_LEVEL_CONFLICT_COMPONENT: ComponentTypeId = ComponentTypeId(u32::MAX);

/// Built-in transform component used by unit tests (`TC-12.7.4.4`, `TC-15.8.13.2`, …).
pub const COMPONENT_TRANSFORM: ComponentTypeId = ComponentTypeId(1);

/// Synthetic list component used for vector-length fallback tests (`TC-12.7.4.8`).
pub const COMPONENT_LIST: ComponentTypeId = ComponentTypeId(2);

/// One segment in a [`FieldPath`].
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FieldPathSegment {
    /// Field or property name.
    pub field: String,
    /// Optional index when traversing homogeneous sequences.
    pub index: Option<u32>,
}

/// Hierarchical field location inside a component payload.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FieldPath(pub Vec<FieldPathSegment>);

/// Serialized component blob attached to an entity record.
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentBlob {
    /// Component type this payload encodes.
    pub component: ComponentTypeId,
    /// Encoded component bytes.
    pub payload: Blob,
}

/// One row inside a [`SceneDocument`].
#[derive(Clone, Debug, PartialEq)]
pub struct EntityRecord {
    /// Stable id for diff and merge.
    pub id: StableEntityId,
    /// Parent link in the entity hierarchy.
    pub parent: Option<StableEntityId>,
    /// Archetype discriminator.
    pub archetype: ArchetypeId,
    /// Attached components sorted by [`ComponentTypeId`] for deterministic diff output.
    pub components: Vec<ComponentBlob>,
}

/// Versioned scene archive (`SceneDocument` in the design doc).
#[derive(Clone, Debug, PartialEq)]
pub struct SceneDocument {
    /// Archive format version.
    pub version: u32,
    /// Scene root entity id.
    pub root: StableEntityId,
    /// All entities in the scene.
    pub entities: Vec<EntityRecord>,
}

/// New entity introduced relative to the base scene.
#[derive(Clone, Debug, PartialEq)]
pub struct EntityAdd {
    /// Full entity record as it should appear in the target scene.
    pub record: EntityRecord,
}

/// Summary of structural edits between a base and target scene.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SceneDiff {
    /// Entities present in `other` but not `base`.
    pub added: Vec<EntityAdd>,
    /// Stable ids removed in `other` relative to `base`.
    pub removed: Vec<StableEntityId>,
    /// Entities present in both scenes with any detected change.
    pub modified: Vec<EntityModify>,
}

/// Component- or hierarchy-level edits for one stable entity id.
#[derive(Clone, Debug, PartialEq)]
pub struct EntityModify {
    /// Target entity id.
    pub id: StableEntityId,
    /// Parent pointer change, if any.
    pub parent_change: Option<(Option<StableEntityId>, Option<StableEntityId>)>,
    /// Component-level edits in deterministic order.
    pub component_changes: Vec<ComponentChange>,
}

/// Component-scoped edit applied to a single entity.
#[derive(Clone, Debug, PartialEq)]
pub enum ComponentChange {
    /// New component attachment.
    Inserted {
        /// Inserted component type.
        component: ComponentTypeId,
        /// Inserted payload.
        value: Blob,
    },
    /// Removed component attachment.
    Removed {
        /// Removed component type.
        component: ComponentTypeId,
        /// Payload removed from the base scene.
        old: Blob,
    },
    /// Field-level edit inside an existing component.
    FieldChange {
        /// Component type owning the field.
        component: ComponentTypeId,
        /// Field path inside the payload.
        path: FieldPath,
        /// Field value in the base scene.
        before: Blob,
        /// Field value in the `other` scene.
        after: Blob,
    },
}

/// Descriptor table hook from the design; currently a zero-sized placeholder for API shape.
#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentDescriptorTable;

/// Successful output of [`crate::SceneMergeDriver::merge`].
#[derive(Clone, Debug, PartialEq)]
pub struct MergeResult {
    /// Best-effort merged scene (conflicting fields remain at base values).
    pub merged: SceneDocument,
    /// Conflicts that require explicit [`crate::PropertyResolution`].
    pub conflicts: Vec<Conflict>,
    /// Count of auto-applied hunks sourced from our branch.
    pub applied_from_ours: u32,
    /// Count of auto-applied hunks sourced from their branch.
    pub applied_from_theirs: u32,
}

/// Recorded three-way field conflict awaiting UI or headless resolution.
#[derive(Clone, Debug, PartialEq)]
pub struct Conflict {
    /// Entity hosting the conflict.
    pub entity: StableEntityId,
    /// Component under contention, or [`ENTITY_LEVEL_CONFLICT_COMPONENT`] for entity-level cases.
    pub component: ComponentTypeId,
    /// Field under contention (empty for entity-level conflicts).
    pub field: FieldPath,
    /// Base side payload slice for the contested region.
    pub base: Blob,
    /// Our side payload slice.
    pub ours: Blob,
    /// Their side payload slice.
    pub theirs: Blob,
}
