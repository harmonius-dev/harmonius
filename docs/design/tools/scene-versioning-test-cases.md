# Scene Versioning -- Test Cases

Companion to [scene-versioning.md](scene-versioning.md).

Test case IDs use `TC-15.10.Z.N` and `TC-12.7.Z.N` format.

## Unit Tests

| ID              | Name                                         | Req        |
|-----------------|----------------------------------------------|------------|
| TC-12.7.4.1     | `test_diff_empty_scenes`                     | R-12.7.4   |
| TC-12.7.4.2     | `test_diff_adds_entity`                      | R-12.7.4   |
| TC-12.7.4.3     | `test_diff_removes_entity`                   | R-12.7.4   |
| TC-12.7.4.4     | `test_diff_modifies_component_field`         | R-12.7.4   |
| TC-12.7.4.5     | `test_diff_reparent_entity`                  | R-12.7.4   |
| TC-12.7.4.6     | `test_diff_insert_new_component`             | R-12.7.4   |
| TC-12.7.4.7     | `test_diff_remove_existing_component`        | R-12.7.4   |
| TC-12.7.4.8     | `test_diff_vector_field_change_fallback`     | R-12.7.4   |
| TC-15.8.13.1    | `test_entity_level_changeset_keys`           | R-15.8.13  |
| TC-15.8.13.2    | `test_field_path_nested_struct`              | R-15.8.13  |
| TC-15.10.3.1    | `test_merge_no_conflict_apply_both`          | R-15.10.3  |
| TC-15.10.3.2    | `test_merge_both_sides_identical_edit`       | R-15.10.3  |
| TC-15.10.3.3    | `test_merge_ours_modifies_theirs_removes`    | R-15.10.3  |
| TC-15.10.3.4    | `test_merge_theirs_modifies_ours_removes`    | R-15.10.3  |
| TC-15.10.3.5    | `test_merge_add_disjoint_entities`           | R-15.10.3  |
| TC-15.10.9.1    | `test_field_level_conflict_emitted`          | R-15.10.9  |
| TC-15.10.9.2    | `test_per_property_resolution_accepts_ours`  | R-15.10.9  |
| TC-15.10.9.3    | `test_per_property_resolution_accepts_theirs`| R-15.10.9  |
| TC-15.10.9.4    | `test_non_overlapping_fields_merge_cleanly`  | R-15.10.9  |

1. **TC-12.7.4.1** `test_diff_empty_scenes` -- Empty base and other produce empty `SceneDiff`.
2. **TC-12.7.4.2** `test_diff_adds_entity` -- One new entity in `other` appears in `added`.
3. **TC-12.7.4.3** `test_diff_removes_entity` -- Entity in base absent from `other` appears in
   `removed`.
4. **TC-12.7.4.4** `test_diff_modifies_component_field` -- A single `Transform.translation.x` change
   produces one `FieldChange` with correct `FieldPath` and blobs.
5. **TC-12.7.4.5** `test_diff_reparent_entity` -- Reparent yields `parent_change = Some((old, new))`
   and no component change.
6. **TC-12.7.4.6** `test_diff_insert_new_component` -- Adding a component to an existing entity
   emits `ComponentChange::Inserted`.
7. **TC-12.7.4.7** `test_diff_remove_existing_component` -- Removing a component emits
   `ComponentChange::Removed` with the old blob.
8. **TC-12.7.4.8** `test_diff_vector_field_change_fallback` -- Length change in `Vec<T>` field emits
   a whole-component replacement, not a field diff.
9. **TC-15.8.13.1** `test_entity_level_changeset_keys` -- Diff index keyed by `StableEntityId` in
   sorted order; no duplicates.
10. **TC-15.8.13.2** `test_field_path_nested_struct` -- `Transform.rotation.xyz.y` resolves to a
    three-segment `FieldPath`.
11. **TC-15.10.3.1** `test_merge_no_conflict_apply_both` -- Ours edits entity A, theirs edits entity
    B; merged scene has both edits.
12. **TC-15.10.3.2** `test_merge_both_sides_identical_edit` -- Same field, same value on both sides;
    applied once, no conflict.
13. **TC-15.10.3.3** `test_merge_ours_modifies_theirs_removes` -- Entity-level conflict emitted.
14. **TC-15.10.3.4** `test_merge_theirs_modifies_ours_removes` -- Entity-level conflict emitted.
15. **TC-15.10.3.5** `test_merge_add_disjoint_entities` -- Each side adds a unique new entity; both
    present in merged scene.
16. **TC-15.10.9.1** `test_field_level_conflict_emitted` -- Both sides change same field to
    different values; one `Conflict` with base/ours/theirs blobs.
17. **TC-15.10.9.2** `test_per_property_resolution_accepts_ours` -- Resolver picks `ours`; merged
    field equals ours blob.
18. **TC-15.10.9.3** `test_per_property_resolution_accepts_theirs` -- Resolver picks `theirs`;
    merged field equals theirs blob.
19. **TC-15.10.9.4** `test_non_overlapping_fields_merge_cleanly` -- Ours edits `Transform.x`, theirs
    edits `Transform.y`; both applied with zero conflicts.

## Integration Tests

| ID              | Name                                             | Req         |
|-----------------|--------------------------------------------------|-------------|
| TC-15.10.3.I1   | `test_git_merge_driver_clean_merge`              | R-15.10.3   |
| TC-15.10.3.I2   | `test_git_merge_driver_conflict_exit_code`       | R-15.10.3   |
| TC-15.10.10.I1  | `test_conflict_panel_round_trip`                 | R-15.10.10  |
| TC-12.7.4.I1    | `test_diff_merge_golden_fixture`                 | R-12.7.4    |
| TC-12.7.4.I2    | `test_stable_entity_id_survives_round_trip`      | R-12.7.4    |

1. **TC-15.10.3.I1** `test_git_merge_driver_clean_merge` -- Invoke the driver under Git with
   disjoint edits; Git reports clean merge; file contents equal expected.
2. **TC-15.10.3.I2** `test_git_merge_driver_conflict_exit_code` -- Overlapping edits; driver exits
   1; `.harmonius/conflicts/<path>.pending` exists.
3. **TC-15.10.10.I1** `test_conflict_panel_round_trip` -- Pending conflicts load into panel;
   resolutions applied; final scene committed clean.
4. **TC-12.7.4.I1** `test_diff_merge_golden_fixture` -- Fixed golden fixture produces same merged
   output across platforms.
5. **TC-12.7.4.I2** `test_stable_entity_id_survives_round_trip` -- Save and re-load scene; entity
   IDs preserved bit-identically.

## Benchmarks

| ID              | Name                                       | Target       |
|-----------------|--------------------------------------------|--------------|
| TC-12.7.4.B1    | `bench_diff_10k_entities`                  | < 50 ms      |
| TC-12.7.4.B2    | `bench_diff_single_field_change`           | < 0.1 ms     |
| TC-15.10.3.B1   | `bench_merge_10k_entities_no_conflict`     | < 150 ms     |
| TC-15.10.3.B2   | `bench_merge_10k_entities_1pct_conflict`   | < 200 ms     |

1. **TC-12.7.4.B1** -- Diff two scenes of 10k entities. < 50 ms.
2. **TC-12.7.4.B2** -- Diff with a single field change between scenes. < 0.1 ms.
3. **TC-15.10.3.B1** -- Merge a 10k entity scene with disjoint edits on both sides. < 150 ms.
4. **TC-15.10.3.B2** -- Merge a 10k entity scene with 1% of fields in conflict. < 200 ms.
