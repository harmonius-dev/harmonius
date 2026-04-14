#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Structural diff and three-way merge for Harmonius scene documents.
//!
//! This crate implements the algorithms described in `docs/design/tools/scene-versioning.md`.
//! Serialization (`rkyv`) and codegen field walkers are intentionally out of scope for the
//! bootstrap implementation; [`COMPONENT_TRANSFORM`] uses a fixed binary layout for tests.

mod diff;
mod merge;
mod types;

pub use diff::diff_scene;
pub use diff::{encode_list, encode_transform, record_with_components};
pub use merge::{
    apply_property_resolution, merge_scene_three_way, MergeError, PropertyResolution,
    SceneMergeDriver,
};
pub use types::{
    Blob, ComponentBlob, ComponentChange, ComponentDescriptorTable, ComponentTypeId, Conflict,
    EntityAdd, EntityModify, EntityRecord, FieldPath, FieldPathSegment, MergeResult, SceneDiff,
    SceneDocument, StableEntityId, COMPONENT_LIST, COMPONENT_TRANSFORM,
    ENTITY_LEVEL_CONFLICT_COMPONENT,
};

#[cfg(test)]
mod tests;
