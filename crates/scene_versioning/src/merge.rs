//! Three-way merge and interactive conflict helpers.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use crate::diff::{diff_scene, transform_field_specs};
use crate::types::{
    Blob, ComponentChange, ComponentDescriptorTable, ComponentTypeId, Conflict, EntityRecord,
    FieldPath, MergeResult, SceneDiff, SceneDocument, StableEntityId, COMPONENT_TRANSFORM,
    ENTITY_LEVEL_CONFLICT_COMPONENT,
};

/// Side selection when applying a recorded [`Conflict`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropertyResolution {
    /// Keep the our-branch value for the contested field.
    Ours,
    /// Keep the their-branch value for the contested field.
    Theirs,
}

/// Merge-time failures surfaced to callers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MergeError {
    /// Placeholder for future schema validation failures.
    SchemaMismatch,
}

/// Driver object mirroring the design surface area.
#[derive(Clone, Copy, Debug)]
pub struct SceneMergeDriver {
    _descriptors: ComponentDescriptorTable,
}

impl SceneMergeDriver {
    /// Constructs a merge driver with the supplied descriptor table.
    #[must_use]
    pub const fn new(descriptors: ComponentDescriptorTable) -> Self {
        Self {
            _descriptors: descriptors,
        }
    }

    /// Performs a structural three-way merge.
    pub fn merge(
        &self,
        base: &SceneDocument,
        ours: &SceneDocument,
        theirs: &SceneDocument,
    ) -> Result<MergeResult, MergeError> {
        merge_scene_three_way(base, ours, theirs)
    }
}

/// Convenience wrapper around [`SceneMergeDriver::merge`].
pub fn merge_scene_three_way(
    base: &SceneDocument,
    ours: &SceneDocument,
    theirs: &SceneDocument,
) -> Result<MergeResult, MergeError> {
    let diff_o = diff_scene(base, ours);
    let diff_t = diff_scene(base, theirs);

    let ours_fields = gather_field_patches(&diff_o);
    let theirs_fields = gather_field_patches(&diff_t);

    let ours_rem: BTreeSet<StableEntityId> = diff_o.removed.iter().copied().collect();
    let theirs_rem: BTreeSet<StableEntityId> = diff_t.removed.iter().copied().collect();

    let ours_adds = gather_adds(&diff_o);
    let theirs_adds = gather_adds(&diff_t);

    let mut conflicts = Vec::new();
    let mut entity_conflicts = BTreeSet::new();
    let mut merged = base.clone();
    let mut applied_ours = 0_u32;
    let mut applied_theirs = 0_u32;

    // Entity-level removal conflicts must be evaluated before mutating `merged`.
    for &id in &union_ids(&ours_rem, &theirs_rem) {
        let ours_r = ours_rem.contains(&id);
        let theirs_r = theirs_rem.contains(&id);
        if ours_r && theirs_r {
            remove_entity(&mut merged, id);
            continue;
        }
        if ours_r && entity_modified(&diff_t, id) {
            conflicts.push(entity_level_conflict(id));
            entity_conflicts.insert(id);
            continue;
        }
        if theirs_r && entity_modified(&diff_o, id) {
            conflicts.push(entity_level_conflict(id));
            entity_conflicts.insert(id);
            continue;
        }
        if ours_r {
            remove_entity(&mut merged, id);
        }
        if theirs_r {
            remove_entity(&mut merged, id);
        }
    }

    // Disjoint entity additions.
    for (&id, record) in &ours_adds {
        if merged.entities.iter().any(|row| row.id == id) {
            continue;
        }
        if theirs_adds.contains_key(&id) {
            // Extremely defensive: allocator collisions should be impossible; treat as conflict.
            conflicts.push(entity_level_conflict(id));
            continue;
        }
        merged.entities.push(record.clone());
        applied_ours += 1;
    }

    for (&id, record) in &theirs_adds {
        if merged.entities.iter().any(|row| row.id == id) {
            continue;
        }
        merged.entities.push(record.clone());
        applied_theirs += 1;
    }

    let all_field_keys = ours_fields
        .keys()
        .chain(theirs_fields.keys())
        .cloned()
        .collect::<BTreeSet<_>>();

    for key in all_field_keys {
        if entity_conflicts.contains(&key.0) {
            continue;
        }
        let ours_patch = ours_fields.get(&key);
        let theirs_patch = theirs_fields.get(&key);

        let Some((base_before, ours_after)) = ours_patch else {
            if let Some((base_before, theirs_after)) = theirs_patch {
                if theirs_after != base_before {
                    patch_field(&mut merged, &key, theirs_after)?;
                    applied_theirs += 1;
                }
            }
            continue;
        };

        let Some((base_before_theirs, theirs_after)) = theirs_patch else {
            if ours_after != base_before {
                patch_field(&mut merged, &key, ours_after)?;
                applied_ours += 1;
            }
            continue;
        };

        debug_assert_eq!(base_before, base_before_theirs);

        if ours_after == theirs_after {
            if ours_after != base_before {
                patch_field(&mut merged, &key, ours_after)?;
                applied_ours += 1;
                applied_theirs += 1;
            }
            continue;
        }

        if ours_after == base_before {
            patch_field(&mut merged, &key, theirs_after)?;
            applied_theirs += 1;
            continue;
        }

        if theirs_after == base_before {
            patch_field(&mut merged, &key, ours_after)?;
            applied_ours += 1;
            continue;
        }

        conflicts.push(Conflict {
            entity: key.0,
            component: key.1,
            field: key.2.clone(),
            base: base_before.clone(),
            ours: ours_after.clone(),
            theirs: theirs_after.clone(),
        });
    }

    normalize_document(&mut merged);

    Ok(MergeResult {
        merged,
        conflicts,
        applied_from_ours: applied_ours,
        applied_from_theirs: applied_theirs,
    })
}

/// Applies a single interactive resolution to an in-memory scene.
pub fn apply_property_resolution(
    document: &mut SceneDocument,
    conflict: &Conflict,
    resolution: PropertyResolution,
) -> Result<(), MergeError> {
    if conflict.component == ENTITY_LEVEL_CONFLICT_COMPONENT {
        return Err(MergeError::SchemaMismatch);
    }

    let chosen = match resolution {
        PropertyResolution::Ours => &conflict.ours,
        PropertyResolution::Theirs => &conflict.theirs,
    };

    let key = FieldKey(conflict.entity, conflict.component, conflict.field.clone());
    patch_field(document, &key, chosen)?;
    normalize_document(document);
    Ok(())
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct FieldKey(StableEntityId, ComponentTypeId, FieldPath);

type FieldPatchMap = BTreeMap<FieldKey, (Blob, Blob)>;

fn gather_field_patches(diff: &SceneDiff) -> FieldPatchMap {
    let mut map = FieldPatchMap::new();
    for modify in &diff.modified {
        for change in &modify.component_changes {
            if let ComponentChange::FieldChange {
                component,
                path,
                before,
                after,
            } = change
            {
                map.insert(
                    FieldKey(modify.id, *component, path.clone()),
                    (before.clone(), after.clone()),
                );
            }
        }
    }
    map
}

fn gather_adds(diff: &SceneDiff) -> BTreeMap<StableEntityId, EntityRecord> {
    diff.added
        .iter()
        .map(|add| (add.record.id, add.record.clone()))
        .collect()
}

fn entity_modified(diff: &SceneDiff, id: StableEntityId) -> bool {
    diff.modified.iter().any(|row| row.id == id)
}

fn union_ids(
    a: &BTreeSet<StableEntityId>,
    b: &BTreeSet<StableEntityId>,
) -> BTreeSet<StableEntityId> {
    a.union(b).copied().collect()
}

fn entity_level_conflict(id: StableEntityId) -> Conflict {
    Conflict {
        entity: id,
        component: ENTITY_LEVEL_CONFLICT_COMPONENT,
        field: FieldPath(Vec::new()),
        base: Vec::new(),
        ours: Vec::new(),
        theirs: Vec::new(),
    }
}

fn remove_entity(document: &mut SceneDocument, id: StableEntityId) {
    document.entities.retain(|row| row.id != id);
}

fn patch_field(
    document: &mut SceneDocument,
    key: &FieldKey,
    value: &[u8],
) -> Result<(), MergeError> {
    let FieldKey(entity, component, path) = key;
    let Some(record) = document.entities.iter_mut().find(|row| row.id == *entity) else {
        return Err(MergeError::SchemaMismatch);
    };

    if *component != COMPONENT_TRANSFORM {
        return Err(MergeError::SchemaMismatch);
    }

    let Some(blob) = record
        .components
        .iter_mut()
        .find(|blob| blob.component == *component)
    else {
        return Err(MergeError::SchemaMismatch);
    };

    let range = transform_field_byte_range(path).ok_or(MergeError::SchemaMismatch)?;
    if blob.payload.len() < range.end || value.len() != range.len() {
        return Err(MergeError::SchemaMismatch);
    }
    blob.payload[range.clone()].copy_from_slice(value);
    Ok(())
}

fn transform_field_byte_range(path: &FieldPath) -> Option<Range<usize>> {
    transform_field_specs()
        .into_iter()
        .find(|(candidate, _)| candidate == path)
        .map(|(_, range)| range)
}

fn normalize_document(document: &mut SceneDocument) {
    document.entities.sort_by_key(|row| row.id);
    for row in &mut document.entities {
        row.components.sort_by_key(|component| component.component);
    }
}
