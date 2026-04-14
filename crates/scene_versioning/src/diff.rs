//! Structural diff between two in-memory scene documents.

use std::collections::BTreeMap;
use std::ops::Range;

use crate::types::{
    ArchetypeId, Blob, ComponentBlob, ComponentChange, ComponentTypeId, EntityAdd, EntityModify,
    EntityRecord, FieldPath, FieldPathSegment, SceneDiff, SceneDocument, StableEntityId,
    COMPONENT_LIST, COMPONENT_TRANSFORM,
};

/// Computes a deterministic structural diff from `base` to `other`.
#[must_use]
pub fn diff_scene(base: &SceneDocument, other: &SceneDocument) -> SceneDiff {
    let base_map = index_entities(base);
    let other_map = index_entities(other);

    let mut diff = SceneDiff::default();

    for (&id, other_record) in &other_map {
        if !base_map.contains_key(&id) {
            diff.added.push(EntityAdd {
                record: (*other_record).clone(),
            });
        }
    }

    for &id in base_map.keys() {
        if !other_map.contains_key(&id) {
            diff.removed.push(id);
        }
    }

    for (&id, base_record) in &base_map {
        let Some(other_record) = other_map.get(&id) else {
            continue;
        };

        if let Some(modify) = diff_entity(id, base_record, other_record) {
            diff.modified.push(modify);
        }
    }

    diff.removed.sort();
    diff.modified.sort_by_key(|m| m.id);
    diff
}

fn index_entities(doc: &SceneDocument) -> BTreeMap<StableEntityId, &EntityRecord> {
    doc.entities.iter().map(|row| (row.id, row)).collect()
}

fn component_map(record: &EntityRecord) -> BTreeMap<ComponentTypeId, &Blob> {
    let mut map = BTreeMap::new();
    for blob in &record.components {
        map.insert(blob.component, &blob.payload);
    }
    map
}

fn diff_entity(
    id: StableEntityId,
    base: &EntityRecord,
    other: &EntityRecord,
) -> Option<EntityModify> {
    let mut component_changes = Vec::new();

    let parent_change = if base.parent == other.parent {
        None
    } else {
        Some((base.parent, other.parent))
    };

    let base_components = component_map(base);
    let other_components = component_map(other);

    for (&ty, base_payload) in &base_components {
        match other_components.get(&ty) {
            None => component_changes.push(ComponentChange::Removed {
                component: ty,
                old: (*base_payload).clone(),
            }),
            Some(other_payload) => {
                if base_payload == other_payload {
                    continue;
                }
                component_changes.extend(diff_component(ty, base_payload, other_payload));
            }
        }
    }

    for (&ty, other_payload) in &other_components {
        if base_components.contains_key(&ty) {
            continue;
        }
        component_changes.push(ComponentChange::Inserted {
            component: ty,
            value: (*other_payload).clone(),
        });
    }

    if parent_change.is_none() && component_changes.is_empty() {
        return None;
    }

    Some(EntityModify {
        id,
        parent_change,
        component_changes,
    })
}

fn diff_component(ty: ComponentTypeId, base: &[u8], other: &[u8]) -> Vec<ComponentChange> {
    if ty == COMPONENT_TRANSFORM {
        diff_transform(base, other)
    } else if ty == COMPONENT_LIST {
        diff_list(base, other)
    } else {
        diff_opaque(ty, base, other)
    }
}

fn diff_opaque(ty: ComponentTypeId, base: &[u8], other: &[u8]) -> Vec<ComponentChange> {
    vec![
        ComponentChange::Removed {
            component: ty,
            old: base.to_vec(),
        },
        ComponentChange::Inserted {
            component: ty,
            value: other.to_vec(),
        },
    ]
}

fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    let slice = bytes.get(0..4)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn diff_list(base: &[u8], other: &[u8]) -> Vec<ComponentChange> {
    let base_len = read_u32_le(base).unwrap_or(0);
    let other_len = read_u32_le(other).unwrap_or(0);
    if base_len != other_len {
        return diff_opaque(COMPONENT_LIST, base, other);
    }
    if base == other {
        return Vec::new();
    }
    diff_opaque(COMPONENT_LIST, base, other)
}

pub(crate) fn transform_field_specs() -> Vec<(FieldPath, Range<usize>)> {
    vec![
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "translation".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "x".to_string(),
                    index: None,
                },
            ]),
            0..4,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "translation".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "y".to_string(),
                    index: None,
                },
            ]),
            4..8,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "translation".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "z".to_string(),
                    index: None,
                },
            ]),
            8..12,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "rotation".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "xyz".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "x".to_string(),
                    index: None,
                },
            ]),
            12..16,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "rotation".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "xyz".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "y".to_string(),
                    index: None,
                },
            ]),
            16..20,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "rotation".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "xyz".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "z".to_string(),
                    index: None,
                },
            ]),
            20..24,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "rotation".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "w".to_string(),
                    index: None,
                },
            ]),
            24..28,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "scale".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "x".to_string(),
                    index: None,
                },
            ]),
            28..32,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "scale".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "y".to_string(),
                    index: None,
                },
            ]),
            32..36,
        ),
        (
            FieldPath(vec![
                FieldPathSegment {
                    field: "scale".to_string(),
                    index: None,
                },
                FieldPathSegment {
                    field: "z".to_string(),
                    index: None,
                },
            ]),
            36..40,
        ),
    ]
}

fn slice_field(bytes: &[u8], range: &Range<usize>) -> Blob {
    bytes.get(range.clone()).unwrap_or(&[]).to_vec()
}

fn diff_transform(base: &[u8], other: &[u8]) -> Vec<ComponentChange> {
    if base.len() != 40 || other.len() != 40 {
        return diff_opaque(COMPONENT_TRANSFORM, base, other);
    }

    let mut changes = Vec::new();
    for (path, range) in transform_field_specs() {
        let before = slice_field(base, &range);
        let after = slice_field(other, &range);
        if before == after {
            continue;
        }
        changes.push(ComponentChange::FieldChange {
            component: COMPONENT_TRANSFORM,
            path,
            before,
            after,
        });
    }

    if changes.is_empty() {
        return Vec::new();
    }

    changes
}

/// Builds a transform payload with the fixed 40-byte layout used by [`diff_transform`].
#[must_use]
pub fn encode_transform(
    translation: (f32, f32, f32),
    rotation_xyzw: (f32, f32, f32, f32),
    scale: (f32, f32, f32),
) -> Blob {
    let mut out = vec![0_u8; 40];
    let (tx, ty, tz) = translation;
    out[0..4].copy_from_slice(&tx.to_le_bytes());
    out[4..8].copy_from_slice(&ty.to_le_bytes());
    out[8..12].copy_from_slice(&tz.to_le_bytes());
    let (rx, ry, rz, rw) = rotation_xyzw;
    out[12..16].copy_from_slice(&rx.to_le_bytes());
    out[16..20].copy_from_slice(&ry.to_le_bytes());
    out[20..24].copy_from_slice(&rz.to_le_bytes());
    out[24..28].copy_from_slice(&rw.to_le_bytes());
    let (sx, sy, sz) = scale;
    out[28..32].copy_from_slice(&sx.to_le_bytes());
    out[32..36].copy_from_slice(&sy.to_le_bytes());
    out[36..40].copy_from_slice(&sz.to_le_bytes());
    out
}

/// Encodes a synthetic list component: `u32` little-endian length followed by `i32` elements.
#[must_use]
pub fn encode_list(elements: &[i32]) -> Blob {
    let mut out = Vec::new();
    out.extend_from_slice(&(elements.len() as u32).to_le_bytes());
    for value in elements {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

/// Convenience for tests constructing entity rows with sorted components.
#[must_use]
pub fn record_with_components(
    id: u64,
    parent: Option<u64>,
    archetype: u32,
    components: Vec<ComponentBlob>,
) -> EntityRecord {
    let mut sorted = components;
    sorted.sort_by_key(|c| c.component);
    EntityRecord {
        id: StableEntityId(id),
        parent: parent.map(StableEntityId),
        archetype: ArchetypeId(archetype),
        components: sorted,
    }
}
