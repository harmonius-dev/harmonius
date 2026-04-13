# Editor Selection Model -- Test Cases

Companion to [selection-model.md](selection-model.md).

Test case IDs use `TC-15.1.4.Z.N` format.

## Unit Tests

| ID             | Name                                     | Req        |
|----------------|------------------------------------------|------------|
| TC-15.1.4.1.1  | `test_selection_add_new_entity`          | R-15.1.4.1 |
| TC-15.1.4.1.2  | `test_selection_add_duplicate_noop`      | R-15.1.4.1 |
| TC-15.1.4.1.3  | `test_selection_remove`                  | R-15.1.4.1 |
| TC-15.1.4.1.4  | `test_selection_clear`                   | R-15.1.4.1 |
| TC-15.1.4.1.5  | `test_selection_toggle_add_remove`       | R-15.1.4.1 |
| TC-15.1.4.1.6  | `test_selection_replace`                 | R-15.1.4.1 |
| TC-15.1.4.1.7  | `test_selection_primary_first_added`     | R-15.1.4.1 |
| TC-15.1.4.1.8  | `test_selection_snapshot_roundtrip`      | R-15.1.4.1 |
| TC-15.1.4.1.9  | `test_selection_revision_bumps`          | R-15.1.4.1 |
| TC-15.1.4.1.10 | `test_selection_sorted_vec_on_insert`    | R-15.1.4.1 |
| TC-15.1.4.2.1  | `test_selectable_pickable_flag`          | R-15.1.4.2 |
| TC-15.1.4.2.2  | `test_selectable_group_redirect`         | R-15.1.4.2 |
| TC-15.1.4.4.1  | `test_subobject_vertex_picked`           | R-15.1.4.4 |
| TC-15.1.4.4.2  | `test_subobject_edge_picked`             | R-15.1.4.4 |
| TC-15.1.4.4.3  | `test_subobject_face_picked`             | R-15.1.4.4 |
| TC-15.1.4.4.4  | `test_subobject_nearest_element`         | R-15.1.4.4 |
| TC-15.1.4.5.1  | `test_marquee_intersect_mode`            | R-15.1.4.5 |
| TC-15.1.4.5.2  | `test_marquee_enclose_mode`              | R-15.1.4.5 |
| TC-15.1.4.6.1  | `test_lasso_inside_selected`             | R-15.1.4.6 |
| TC-15.1.4.6.2  | `test_lasso_outside_rejected`            | R-15.1.4.6 |
| TC-15.1.4.7.1  | `test_aggregate_transform_single`        | R-15.1.4.7 |
| TC-15.1.4.7.2  | `test_aggregate_transform_average`       | R-15.1.4.7 |
| TC-15.1.4.7.3  | `test_aggregate_transform_empty`         | R-15.1.4.7 |
| TC-15.1.4.8.1  | `test_selection_changed_emitted_on_add`  | R-15.1.4.8 |
| TC-15.1.4.8.2  | `test_selection_changed_emitted_on_remove`| R-15.1.4.8|
| TC-15.1.4.8.3  | `test_selection_changed_added_removed`   | R-15.1.4.8 |

1. **TC-15.1.4.1.1** -- `add(e)` returns true and inserts.
2. **TC-15.1.4.1.2** -- Adding same entity returns false; count unchanged.
3. **TC-15.1.4.1.3** -- `remove(e)` removes and bumps revision.
4. **TC-15.1.4.1.4** -- `clear()` empties all collections, clears primary.
5. **TC-15.1.4.1.5** -- `toggle(e)` flips membership.
6. **TC-15.1.4.1.6** -- `replace([a,b,c])` replaces whatever was there.
7. **TC-15.1.4.1.7** -- First added entity becomes primary.
8. **TC-15.1.4.1.8** -- snapshot -> restore produces equal state.
9. **TC-15.1.4.1.9** -- Any mutation increments `revision`.
10. **TC-15.1.4.1.10** -- `entities` is sorted after every insert.
11. **TC-15.1.4.2.1** -- `Selectable { pickable: false }` is not returned by pick.
12. **TC-15.1.4.2.2** -- Clicking a child redirects to the group root.
13. **TC-15.1.4.4.1** -- Ray hits a vertex; returned `SubObjectKind::Vertex` with index.
14. **TC-15.1.4.4.2** -- Ray hits an edge centerline.
15. **TC-15.1.4.4.3** -- Ray hits a face triangle.
16. **TC-15.1.4.4.4** -- With multiple candidates, the nearest is chosen.
17. **TC-15.1.4.5.1** -- Intersect mode selects entities partially inside rect.
18. **TC-15.1.4.5.2** -- Enclose mode only selects entities fully inside rect.
19. **TC-15.1.4.6.1** -- Entity centroid inside lasso is selected.
20. **TC-15.1.4.6.2** -- Entity centroid outside lasso is not selected.
21. **TC-15.1.4.7.1** -- One entity selected; aggregate == its transform.
22. **TC-15.1.4.7.2** -- Multiple entities; aggregate == average translation.
23. **TC-15.1.4.7.3** -- No selection; aggregate == `None`.
24. **TC-15.1.4.8.1** -- `SelectionChanged` event fires on add.
25. **TC-15.1.4.8.2** -- fires on remove.
26. **TC-15.1.4.8.3** -- Event contains added and removed lists.

## Integration Tests

| ID             | Name                                      | Req        |
|----------------|-------------------------------------------|------------|
| TC-15.1.4.3.1  | `test_two_viewport_sync_on_pick`          | R-15.1.4.3 |
| TC-15.1.4.3.2  | `test_four_viewport_sync_on_marquee`      | R-15.1.4.3 |
| TC-15.1.4.4.5  | `test_cpu_raycast_with_scene_bvh`         | R-15.1.4.4 |
| TC-15.1.4.5.3  | `test_marquee_against_1k_entities`        | R-15.1.4.5 |
| TC-15.1.4.7.4  | `test_gizmo_position_from_aggregate`      | R-15.1.4.7 |

1. **TC-15.1.4.3.1** -- Click in viewport 1; both viewports show selection.
2. **TC-15.1.4.3.2** -- Marquee in viewport 1; all four show updated selection.
3. **TC-15.1.4.4.5** -- CPU raycast through the spatial BVH; hit returned.
4. **TC-15.1.4.5.3** -- Marquee over 1000 entities, verify count and time.
5. **TC-15.1.4.7.4** -- Gizmo position matches aggregate transform.

## Benchmarks

| ID              | Name                                     | Target         |
|-----------------|------------------------------------------|----------------|
| TC-15.1.4.4.B1  | `bench_pick_entity_10k`                  | < 1 ms         |
| TC-15.1.4.4.B2  | `bench_pick_subobject_mesh_100k_tri`     | < 2 ms         |
| TC-15.1.4.5.B1  | `bench_marquee_10k_entities`             | < 3 ms         |
| TC-15.1.4.6.B1  | `bench_lasso_polygon_64_vertices`        | < 3 ms         |
| TC-15.1.4.8.B1  | `bench_event_dispatch_100_observers`     | < 0.5 ms       |

1. **TC-15.1.4.4.B1** -- Entity pick in a 10k-entity scene. < 1 ms.
2. **TC-15.1.4.4.B2** -- Sub-object pick on a 100k-triangle mesh. < 2 ms.
3. **TC-15.1.4.5.B1** -- Marquee over 10k entities. < 3 ms.
4. **TC-15.1.4.6.B1** -- Lasso with 64 polygon vertices. < 3 ms.
5. **TC-15.1.4.8.B1** -- Selection event to 100 observers. < 0.5 ms.
