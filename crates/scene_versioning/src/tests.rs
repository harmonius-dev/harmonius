use crate::{
    apply_property_resolution, diff_scene, encode_list, encode_transform, merge_scene_three_way,
    record_with_components, ComponentBlob, ComponentChange, ComponentDescriptorTable, EntityRecord,
    FieldPath, FieldPathSegment, PropertyResolution, SceneDiff, SceneDocument, SceneMergeDriver,
    StableEntityId, COMPONENT_LIST, COMPONENT_TRANSFORM, ENTITY_LEVEL_CONFLICT_COMPONENT,
};

fn scene(root: u64, entities: Vec<EntityRecord>) -> SceneDocument {
    SceneDocument {
        version: 1,
        root: StableEntityId(root),
        entities,
    }
}

#[test]
fn test_diff_empty_scenes() {
    let base = scene(0, Vec::new());
    let other = scene(0, Vec::new());
    let diff = diff_scene(&base, &other);
    assert_eq!(diff, SceneDiff::default());
}

#[test]
fn test_diff_adds_entity() {
    let base = scene(0, Vec::new());
    let record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let other = scene(0, vec![record.clone()]);
    let diff = diff_scene(&base, &other);
    assert_eq!(diff.removed.len(), 0);
    assert_eq!(diff.modified.len(), 0);
    assert_eq!(diff.added.len(), 1);
    assert_eq!(diff.added[0].record, record);
}

#[test]
fn test_diff_removes_entity() {
    let record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let base = scene(0, vec![record]);
    let other = scene(0, Vec::new());
    let diff = diff_scene(&base, &other);
    assert_eq!(diff.added.len(), 0);
    assert_eq!(diff.modified.len(), 0);
    assert_eq!(diff.removed, vec![StableEntityId(1)]);
}

#[test]
fn test_diff_modifies_component_field() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let other_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((5.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let base = scene(0, vec![base_record]);
    let other = scene(0, vec![other_record]);
    let diff = diff_scene(&base, &other);
    assert_eq!(diff.added.len(), 0);
    assert_eq!(diff.removed.len(), 0);
    assert_eq!(diff.modified.len(), 1);
    let modify = &diff.modified[0];
    assert_eq!(modify.id, StableEntityId(1));
    assert_eq!(modify.component_changes.len(), 1);
    let expected_path = FieldPath(vec![
        FieldPathSegment {
            field: "translation".to_string(),
            index: None,
        },
        FieldPathSegment {
            field: "x".to_string(),
            index: None,
        },
    ]);
    match &modify.component_changes[0] {
        ComponentChange::FieldChange {
            component,
            path,
            before,
            after,
        } => {
            assert_eq!(*component, COMPONENT_TRANSFORM);
            assert_eq!(path, &expected_path);
            assert_eq!(before.len(), 4);
            assert_eq!(after.len(), 4);
            assert_ne!(before, after);
        }
        other => panic!("unexpected change {other:?}"),
    }
}

#[test]
fn test_diff_reparent_entity() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let other_record = record_with_components(
        1,
        Some(2),
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let base = scene(2, vec![base_record]);
    let other = scene(2, vec![other_record]);
    let diff = diff_scene(&base, &other);
    assert_eq!(diff.modified.len(), 1);
    let modify = &diff.modified[0];
    assert_eq!(modify.parent_change, Some((None, Some(StableEntityId(2)))));
    assert!(modify.component_changes.is_empty());
}

#[test]
fn test_diff_insert_new_component() {
    let base_record = record_with_components(1, None, 0, Vec::new());
    let other_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let base = scene(0, vec![base_record]);
    let other = scene(0, vec![other_record]);
    let diff = diff_scene(&base, &other);
    assert_eq!(diff.modified.len(), 1);
    let modify = &diff.modified[0];
    match &modify.component_changes[0] {
        ComponentChange::Inserted { component, value } => {
            assert_eq!(*component, COMPONENT_TRANSFORM);
            assert_eq!(value.len(), 40);
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn test_diff_remove_existing_component() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let other_record = record_with_components(1, None, 0, Vec::new());
    let base = scene(0, vec![base_record]);
    let other = scene(0, vec![other_record]);
    let diff = diff_scene(&base, &other);
    assert_eq!(diff.modified.len(), 1);
    let modify = &diff.modified[0];
    match &modify.component_changes[0] {
        ComponentChange::Removed { component, old } => {
            assert_eq!(*component, COMPONENT_TRANSFORM);
            assert_eq!(old.len(), 40);
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn test_diff_vector_field_change_fallback() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_LIST,
            payload: encode_list(&[1, 2, 3]),
        }],
    );
    let other_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_LIST,
            payload: encode_list(&[1, 2]),
        }],
    );
    let base = scene(0, vec![base_record]);
    let other = scene(0, vec![other_record]);
    let diff = diff_scene(&base, &other);
    let modify = &diff.modified[0];
    assert_eq!(modify.component_changes.len(), 2);
    assert!(matches!(
        modify.component_changes[0],
        ComponentChange::Removed { .. }
    ));
    assert!(matches!(
        modify.component_changes[1],
        ComponentChange::Inserted { .. }
    ));
}

#[test]
fn test_entity_level_changeset_keys() {
    let mut records = Vec::new();
    for id in [3_u64, 1, 2] {
        records.push(record_with_components(
            id,
            None,
            0,
            vec![ComponentBlob {
                component: COMPONENT_TRANSFORM,
                payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
            }],
        ));
    }
    let base = scene(0, records.clone());
    let mut others = records;
    others[0].components[0].payload =
        encode_transform((1.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    others[1].components[0].payload =
        encode_transform((2.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    others[2].components[0].payload =
        encode_transform((3.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let other = scene(0, others);
    let diff = diff_scene(&base, &other);
    let ids: Vec<StableEntityId> = diff.modified.iter().map(|row| row.id).collect();
    assert_eq!(
        ids,
        vec![StableEntityId(1), StableEntityId(2), StableEntityId(3)]
    );
}

#[test]
fn test_field_path_nested_struct() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let other_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.5, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let base = scene(0, vec![base_record]);
    let other = scene(0, vec![other_record]);
    let diff = diff_scene(&base, &other);
    let modify = &diff.modified[0];
    let expected_path = FieldPath(vec![
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
    ]);
    let field_change = modify
        .component_changes
        .iter()
        .find_map(|change| match change {
            ComponentChange::FieldChange { path, .. } if path == &expected_path => Some(path),
            _ => None,
        });
    assert!(field_change.is_some());
}

#[test]
fn test_merge_no_conflict_apply_both() {
    let base_a = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let base_b = record_with_components(
        2,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let base = scene(0, vec![base_a.clone(), base_b.clone()]);

    let mut ours_a = base_a.clone();
    ours_a.components[0].payload =
        encode_transform((4.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let ours = scene(0, vec![ours_a, base_b.clone()]);

    let mut theirs_b = base_b.clone();
    theirs_b.components[0].payload =
        encode_transform((0.0, 5.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let theirs = scene(0, vec![base_a.clone(), theirs_b]);

    let merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    assert!(merged.conflicts.is_empty());
    let entity_a = merged
        .merged
        .entities
        .iter()
        .find(|row| row.id == StableEntityId(1))
        .expect("entity a");
    let entity_b = merged
        .merged
        .entities
        .iter()
        .find(|row| row.id == StableEntityId(2))
        .expect("entity b");
    assert_eq!(
        entity_a.components[0].payload,
        encode_transform((4.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0))
    );
    assert_eq!(
        entity_b.components[0].payload,
        encode_transform((0.0, 5.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0))
    );
    assert_eq!(merged.applied_from_ours, 1);
    assert_eq!(merged.applied_from_theirs, 1);
}

#[test]
fn test_merge_both_sides_identical_edit() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let updated_payload = encode_transform((9.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let mut ours_record = base_record.clone();
    ours_record.components[0].payload = updated_payload.clone();
    let mut theirs_record = base_record.clone();
    theirs_record.components[0].payload = updated_payload.clone();

    let base = scene(0, vec![base_record]);
    let ours = scene(0, vec![ours_record]);
    let theirs = scene(0, vec![theirs_record]);

    let merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    assert!(merged.conflicts.is_empty());
    assert_eq!(
        merged.merged.entities[0].components[0].payload,
        updated_payload
    );
    assert_eq!(merged.applied_from_ours, 1);
    assert_eq!(merged.applied_from_theirs, 1);
}

#[test]
fn test_merge_ours_modifies_theirs_removes() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let mut ours_record = base_record.clone();
    ours_record.components[0].payload =
        encode_transform((2.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let base = scene(0, vec![base_record]);
    let ours = scene(0, vec![ours_record]);
    let theirs = scene(0, Vec::new());

    let merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    assert_eq!(merged.conflicts.len(), 1);
    assert_eq!(
        merged.conflicts[0].component,
        ENTITY_LEVEL_CONFLICT_COMPONENT
    );
    assert_eq!(merged.merged.entities.len(), 1);
}

#[test]
fn test_merge_theirs_modifies_ours_removes() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let mut theirs_record = base_record.clone();
    theirs_record.components[0].payload =
        encode_transform((0.0, 3.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let base = scene(0, vec![base_record]);
    let ours = scene(0, Vec::new());
    let theirs = scene(0, vec![theirs_record]);

    let merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    assert_eq!(merged.conflicts.len(), 1);
    assert_eq!(
        merged.conflicts[0].component,
        ENTITY_LEVEL_CONFLICT_COMPONENT
    );
    assert_eq!(merged.merged.entities.len(), 1);
}

#[test]
fn test_merge_add_disjoint_entities() {
    let base = scene(0, Vec::new());
    let record_a = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((1.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let record_b = record_with_components(
        2,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 2.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let ours = scene(0, vec![record_a.clone()]);
    let theirs = scene(0, vec![record_b.clone()]);
    let merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    assert!(merged.conflicts.is_empty());
    assert_eq!(merged.merged.entities.len(), 2);
    assert_eq!(merged.applied_from_ours, 1);
    assert_eq!(merged.applied_from_theirs, 1);
}

#[test]
fn test_field_level_conflict_emitted() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let mut ours_record = base_record.clone();
    ours_record.components[0].payload =
        encode_transform((1.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let mut theirs_record = base_record.clone();
    theirs_record.components[0].payload =
        encode_transform((2.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let base = scene(0, vec![base_record]);
    let ours = scene(0, vec![ours_record]);
    let theirs = scene(0, vec![theirs_record]);

    let merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    assert_eq!(merged.conflicts.len(), 1);
    let conflict = &merged.conflicts[0];
    assert_eq!(conflict.entity, StableEntityId(1));
    assert_eq!(conflict.component, COMPONENT_TRANSFORM);
    assert_ne!(conflict.ours, conflict.theirs);
}

#[test]
fn test_per_property_resolution_accepts_ours() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let mut ours_record = base_record.clone();
    ours_record.components[0].payload =
        encode_transform((7.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let mut theirs_record = base_record.clone();
    theirs_record.components[0].payload =
        encode_transform((8.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let base = scene(0, vec![base_record]);
    let ours = scene(0, vec![ours_record]);
    let theirs = scene(0, vec![theirs_record]);

    let mut merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    let conflict = merged.conflicts[0].clone();
    apply_property_resolution(&mut merged.merged, &conflict, PropertyResolution::Ours)
        .expect("apply ours");
    let payload = &merged.merged.entities[0].components[0].payload;
    let expected = encode_transform((7.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    assert_eq!(payload, &expected);
}

#[test]
fn test_per_property_resolution_accepts_theirs() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let mut ours_record = base_record.clone();
    ours_record.components[0].payload =
        encode_transform((7.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let mut theirs_record = base_record.clone();
    theirs_record.components[0].payload =
        encode_transform((8.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let base = scene(0, vec![base_record]);
    let ours = scene(0, vec![ours_record]);
    let theirs = scene(0, vec![theirs_record]);

    let mut merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    let conflict = merged.conflicts[0].clone();
    apply_property_resolution(&mut merged.merged, &conflict, PropertyResolution::Theirs)
        .expect("apply theirs");
    let payload = &merged.merged.entities[0].components[0].payload;
    let expected = encode_transform((8.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    assert_eq!(payload, &expected);
}

#[test]
fn test_non_overlapping_fields_merge_cleanly() {
    let base_record = record_with_components(
        1,
        None,
        0,
        vec![ComponentBlob {
            component: COMPONENT_TRANSFORM,
            payload: encode_transform((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0)),
        }],
    );
    let mut ours_record = base_record.clone();
    ours_record.components[0].payload =
        encode_transform((4.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let mut theirs_record = base_record.clone();
    theirs_record.components[0].payload =
        encode_transform((0.0, 6.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    let base = scene(0, vec![base_record]);
    let ours = scene(0, vec![ours_record]);
    let theirs = scene(0, vec![theirs_record]);

    let merged = merge_scene_three_way(&base, &ours, &theirs).expect("merge");
    assert!(merged.conflicts.is_empty());
    let expected = encode_transform((4.0, 6.0, 0.0), (0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0));
    assert_eq!(merged.merged.entities[0].components[0].payload, expected);
}

#[test]
fn scene_merge_driver_matches_free_function() {
    let base = scene(0, Vec::new());
    let driver = SceneMergeDriver::new(ComponentDescriptorTable::default());
    let ours = scene(0, Vec::new());
    let theirs = scene(0, Vec::new());
    assert_eq!(
        driver.merge(&base, &ours, &theirs).expect("driver"),
        merge_scene_three_way(&base, &ours, &theirs).expect("free fn")
    );
}
