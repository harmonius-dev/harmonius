//! Core identifiers and sub-object references used by the selection model.

/// Stable identifier for an editor world instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EditorWorldId(pub u64);

/// Opaque entity reference used by editor selection APIs.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntityRef(pub u32);

/// Kind of editable sub-object element.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubObjectKind {
    /// Bone influence joint selection.
    BoneJoint,
    /// Mesh edge selection.
    Edge,
    /// Mesh face (triangle) selection.
    Face,
    /// Mesh vertex selection.
    Vertex,
}

/// Reference to a concrete sub-object on an entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubObjectElement {
    /// Owning entity.
    pub entity: EntityRef,
    /// Sub-object category.
    pub kind: SubObjectKind,
    /// Index within the entity's mesh or skeleton data.
    pub index: u32,
}
