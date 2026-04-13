# Undo / Redo -- Test Cases

Companion to [undo-redo.md](undo-redo.md).

Test case IDs use `TC-15.1.3.Z.N` format.

## Unit Tests

| ID             | Name                                     | Req        |
|----------------|------------------------------------------|------------|
| TC-15.1.3.1.1  | `test_command_apply_revert_roundtrip`    | R-15.1.3.1 |
| TC-15.1.3.1.2  | `test_component_set_reverts_value`       | R-15.1.3.1 |
| TC-15.1.3.1.3  | `test_component_insert_reverts_remove`   | R-15.1.3.1 |
| TC-15.1.3.1.4  | `test_entity_spawn_reverts_despawn`      | R-15.1.3.1 |
| TC-15.1.3.1.5  | `test_entity_despawn_reverts_spawn`      | R-15.1.3.1 |
| TC-15.1.3.1.6  | `test_hierarchy_reparent_roundtrip`      | R-15.1.3.1 |
| TC-15.1.3.1.7  | `test_coalesce_continuous_typing`        | R-15.1.3.1 |
| TC-15.1.3.1.8  | `test_coalesce_slider_drag`              | R-15.1.3.1 |
| TC-15.1.3.1.9  | `test_memory_bytes_accounting`           | R-15.1.3.1 |
| TC-15.1.3.2.1  | `test_stack_push_and_undo`               | R-15.1.3.2 |
| TC-15.1.3.2.2  | `test_stack_undo_then_redo`              | R-15.1.3.2 |
| TC-15.1.3.2.3  | `test_stack_push_after_undo_clears_redo` | R-15.1.3.2 |
| TC-15.1.3.2.4  | `test_stack_clear`                       | R-15.1.3.2 |
| TC-15.1.3.3.1  | `test_transaction_single_undo_step`      | R-15.1.3.3 |
| TC-15.1.3.3.2  | `test_transaction_rollback_on_panic`     | R-15.1.3.3 |
| TC-15.1.3.3.3  | `test_transaction_label_preserved`      | R-15.1.3.3 |
| TC-15.1.3.4.1  | `test_budget_evicts_oldest`              | R-15.1.3.4 |
| TC-15.1.3.4.2  | `test_budget_spill_to_disk`              | R-15.1.3.4 |
| TC-15.1.3.4.3  | `test_budget_tracks_total_bytes`         | R-15.1.3.4 |
| TC-15.1.3.7.1  | `test_undo_restores_before_selection`    | R-15.1.3.7 |
| TC-15.1.3.7.2  | `test_redo_restores_after_selection`     | R-15.1.3.7 |

1. **TC-15.1.3.1.1** `test_command_apply_revert_roundtrip` -- Apply a command; revert; assert world
   equal to pre-apply state.
2. **TC-15.1.3.1.2** `test_component_set_reverts_value` -- Change a component's value; revert; old
   value restored.
3. **TC-15.1.3.1.3** `test_component_insert_reverts_remove` -- Insert component; revert; component
   gone.
4. **TC-15.1.3.1.4** `test_entity_spawn_reverts_despawn` -- Spawn entity; revert; entity gone.
5. **TC-15.1.3.1.5** `test_entity_despawn_reverts_spawn` -- Despawn entity captures snapshot; revert
   restores it.
6. **TC-15.1.3.1.6** `test_hierarchy_reparent_roundtrip` -- Reparent and revert.
7. **TC-15.1.3.1.7** `test_coalesce_continuous_typing` -- Typing in a text field coalesces into one
   undo step within 500 ms.
8. **TC-15.1.3.1.8** `test_coalesce_slider_drag` -- Slider drag coalesces per drag gesture.
9. **TC-15.1.3.1.9** `test_memory_bytes_accounting` -- `memory_bytes()` matches rkyv archive size.
10. **TC-15.1.3.2.1** `test_stack_push_and_undo` -- Push command, undo, world reverts.
11. **TC-15.1.3.2.2** `test_stack_undo_then_redo` -- Undo then redo returns world to after-apply
    state.
12. **TC-15.1.3.2.3** `test_stack_push_after_undo_clears_redo` -- New push discards remaining redo
    stack.
13. **TC-15.1.3.2.4** `test_stack_clear` -- After `clear()`, undo/redo both fail.
14. **TC-15.1.3.3.1** `test_transaction_single_undo_step` -- Three commands in a transaction, one
    undo reverts all three.
15. **TC-15.1.3.3.2** `test_transaction_rollback_on_panic` -- Panic drops transaction; no partial
    state.
16. **TC-15.1.3.3.3** `test_transaction_label_preserved` -- Label appears in history view.
17. **TC-15.1.3.4.1** `test_budget_evicts_oldest` -- Exceed budget; oldest evicted to disk.
18. **TC-15.1.3.4.2** `test_budget_spill_to_disk` -- Disk file contains the spilled record; load
    restores it.
19. **TC-15.1.3.4.3** `test_budget_tracks_total_bytes` -- `current_bytes` equals sum of in-memory
    records.
20. **TC-15.1.3.7.1** `test_undo_restores_before_selection` -- Selection returns to pre-command
    state.
21. **TC-15.1.3.7.2** `test_redo_restores_after_selection` -- Selection returns to post-command
    state.

## Integration Tests

| ID             | Name                                         | Req        |
|----------------|----------------------------------------------|------------|
| TC-15.1.3.5.1  | `test_persistent_history_across_sessions`    | R-15.1.3.5 |
| TC-15.1.3.5.2  | `test_manifest_roundtrip`                    | R-15.1.3.5 |
| TC-15.1.3.5.3  | `test_load_spilled_command_on_undo`          | R-15.1.3.5 |
| TC-15.1.3.8.1  | `test_collab_two_user_independent_undo`      | R-15.1.3.8 |
| TC-15.1.3.8.2  | `test_collab_conflict_detection_touches`     | R-15.1.3.8 |
| TC-15.1.3.8.3  | `test_collab_per_user_cursor`                | R-15.1.3.8 |

1. **TC-15.1.3.5.1** `test_persistent_history_across_sessions` -- Start session, push 10 commands,
   shut down, restart, assert history intact.
2. **TC-15.1.3.5.2** `test_manifest_roundtrip` -- Manifest encode/decode preserves stub order.
3. **TC-15.1.3.5.3** `test_load_spilled_command_on_undo` -- Undo past tail loads from disk.
4. **TC-15.1.3.8.1** `test_collab_two_user_independent_undo` -- User 1 undoes own commands; User 2
   undoes own; interleaved edits remain intact.
5. **TC-15.1.3.8.2** `test_collab_conflict_detection_touches` -- Overlapping touches cause later
   undo to be rejected.
6. **TC-15.1.3.8.3** `test_collab_per_user_cursor` -- Each user's cursor advances only on their own
   commands.

## Benchmarks

| ID              | Name                                  | Target       |
|-----------------|---------------------------------------|--------------|
| TC-15.1.3.6.B1  | `bench_apply_typical_command`         | < 50 ms      |
| TC-15.1.3.6.B2  | `bench_revert_typical_command`        | < 50 ms      |
| TC-15.1.3.6.B3  | `bench_apply_large_entity_spawn_1000` | < 100 ms     |
| TC-15.1.3.6.B4  | `bench_coalesce_5_slider_updates`     | < 1 ms       |
| TC-15.1.3.4.B1  | `bench_spill_to_disk_100_commands`    | < 20 ms      |
| TC-15.1.3.5.B1  | `bench_load_100_commands_from_disk`   | < 30 ms      |

1. **TC-15.1.3.6.B1** -- Apply a typical component-set command. < 50 ms.
2. **TC-15.1.3.6.B2** -- Revert a typical command. < 50 ms.
3. **TC-15.1.3.6.B3** -- Apply a spawn of 1000 entities. < 100 ms.
4. **TC-15.1.3.6.B4** -- Coalesce 5 slider updates. < 1 ms.
5. **TC-15.1.3.4.B1** -- Spill 100 commands to disk. < 20 ms.
6. **TC-15.1.3.5.B1** -- Load 100 commands from disk. < 30 ms.
