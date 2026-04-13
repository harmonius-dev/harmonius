# Editor ↔ Core Runtime Integration Test Cases

All tests are CI-runnable in a headless editor harness that boots both an `EditorWorld` and a
`GameWorld` in the same process, drives them via synthetic inputs, and asserts state.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-9.1.1.1 | Editor edit does not touch GameWorld | IR-9.1.1 |
| TC-IR-9.1.1.2 | Game tick does not touch EditorWorld | IR-9.1.1 |
| TC-IR-9.1.1.3 | Selection lives in EditorWorld only | IR-9.1.1 |
| TC-IR-9.1.2.1 | SpawnEntity mutation applied | IR-9.1.2 |
| TC-IR-9.1.2.2 | DespawnEntity mutation applied | IR-9.1.2 |
| TC-IR-9.1.2.3 | InsertComponent mutation applied | IR-9.1.2 |
| TC-IR-9.1.2.4 | UpdateComponent mutation applied | IR-9.1.2 |
| TC-IR-9.1.3.1 | Bridge drain happens before Phase 1 | IR-9.1.3 |
| TC-IR-9.1.3.2 | Phase 1..8 runs after drain | IR-9.1.3 |
| TC-IR-9.1.3.3 | Snapshot arrives to editor | IR-9.1.3 |
| TC-IR-9.1.4.1 | PIE start clones EditorWorld | IR-9.1.4 |
| TC-IR-9.1.4.2 | PIE play does not mutate EditorWorld | IR-9.1.4 |
| TC-IR-9.1.4.3 | PIE stop drops GameWorld | IR-9.1.4 |
| TC-IR-9.1.5.1 | Hot reload during edit mode | IR-9.1.5 |
| TC-IR-9.1.5.2 | Hot reload during PIE | IR-9.1.5 |
| TC-IR-9.1.5.3 | StateMigrationFn runs on both worlds | IR-9.1.5 |
| TC-IR-9.1.6.1 | Undo reverses mutations | IR-9.1.6 |
| TC-IR-9.1.6.2 | Redo replays mutations | IR-9.1.6 |
| TC-IR-9.1.6.3 | Undo does not affect GameWorld | IR-9.1.6 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-9.1.2.N1 | CH-22 backpressure (FM-1) | IR-9.1.2 |
| TC-IR-9.1.2.N2 | Mutation id collision (FM-2) | IR-9.1.2 |
| TC-IR-9.1.3.N1 | Snapshot stale one frame (FM-3) | IR-9.1.3 |
| TC-IR-9.1.5.N1 | Hot reload migration failure (FM-4) | IR-9.1.5 |
| TC-IR-9.1.4.N1 | PIE clone failure (FM-5) | IR-9.1.4 |
| TC-IR-9.1.1.N1 | GameWorld despawns selected (FM-6) | IR-9.1.1 |
| TC-IR-9.1.6.N1 | Undo stack overflow drops oldest (FM-7) | IR-9.1.6 |

### Test case details

1. **TC-IR-9.1.1.1** -- Input: editor places a cube in `EditorWorld` without flushing bridge.
   Expected: `GameWorld` has no matching entity.
2. **TC-IR-9.1.1.2** -- Input: game ticks 60 frames without editor activity. Expected:
   `EditorWorld.inner` is bit-identical before and after.
3. **TC-IR-9.1.1.3** -- Input: user selects entity in editor. Expected:
   `EditorWorld.selection.contains(entity)`; `GameWorld` has no selection set.
4. **TC-IR-9.1.2.1** -- Input: `EditorMutationKind::SpawnEntity`. Expected: after bridge drain,
   `GameWorld.inner` has the new entity with matching initial components.
5. **TC-IR-9.1.2.2** -- Input: `EditorMutationKind::DespawnEntity { id: 42 }`. Expected:
   `GameWorld.inner.entities.contains(42) == false` after drain.
6. **TC-IR-9.1.2.3** -- Input: `EditorMutationKind::InsertComponent { entity, data }`. Expected:
   `GameWorld.inner.get::<T>(entity)` returns Some.
7. **TC-IR-9.1.2.4** -- Input: `UpdateComponent { entity, data }`. Expected: updated component
   matches.
8. **TC-IR-9.1.3.1** -- Input: frame with staged mutations. Expected: bridge drain occurs before
   Phase 1 runs (assert via Phase 1 system observing the mutations already applied).
9. **TC-IR-9.1.3.2** -- Input: one frame. Expected: Phases 1..8 run in order after drain.
10. **TC-IR-9.1.3.3** -- Input: one frame. Expected: editor reads a `GameStateSnapshot` with
    `frame_index == GameTime.tick`.
11. **TC-IR-9.1.4.1** -- Input: click Play with 100 entities in `EditorWorld`. Expected: `GameWorld`
    has 100 matching entities after clone.
12. **TC-IR-9.1.4.2** -- Input: PIE runs 60 frames mutating entities. Expected: `EditorWorld`
    unchanged.
13. **TC-IR-9.1.4.3** -- Input: click Stop. Expected: `GameWorld` dropped; editor still has original
    `EditorWorld`.
14. **TC-IR-9.1.5.1** -- Input: hot-reload trigger during edit mode. Expected: barrier enters, dylib
    reloads, both worlds migrated; frame resumes.
15. **TC-IR-9.1.5.2** -- Input: hot-reload during PIE. Expected: barrier pauses both worlds,
    migrates both, resumes PIE.
16. **TC-IR-9.1.5.3** -- Input: `StateMigrationFn` registered for a component. Expected: called once
    for `EditorWorld` and once for `GameWorld`.
17. **TC-IR-9.1.6.1** -- Input: three spawn mutations then two undos. Expected: one entity remains
    in `EditorWorld`.
18. **TC-IR-9.1.6.2** -- Input: undo × 2 then redo × 1. Expected: two entities remain.
19. **TC-IR-9.1.6.3** -- Input: undo while in edit mode. Expected: `GameWorld` unaffected (PIE not
    running).
20. **TC-IR-9.1.2.N1** -- Input: burst 300 mutations against `CH-22` cap=256. Expected: editor
    worker parks briefly; mutations eventually all delivered; `FM-1` counter increments.
21. **TC-IR-9.1.2.N2** -- Input: two mutations with same `mutation_id`. Expected: last-write wins;
    `FM-2` counter increments.
22. **TC-IR-9.1.3.N1** -- Input: editor reads snapshot while game loop blocked. Expected: previous
    snapshot re-read; `FM-3` counter increments.
23. **TC-IR-9.1.5.N1** -- Input: migration function panics. Expected: revert to previous dylib;
    error banner raised in editor; `FM-4` counter increments.
24. **TC-IR-9.1.4.N1** -- Input: PIE clone with insufficient memory. Expected: PIE start aborts;
    user sees banner; `FM-5` counter increments.
25. **TC-IR-9.1.1.N1** -- Input: PIE game loop despawns an entity currently selected in the editor.
    Expected: selection cleared; `FM-6` counter increments; no panic.
26. **TC-IR-9.1.6.N1** -- Input: 10,000 mutations with stack cap=1024. Expected: oldest 8976 entries
    dropped; `FM-7` counter increments.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-9.1.2.B1 | Drain 256 mutations | < 0.1 ms | IR-9.1.2 |
| TC-IR-9.1.3.B1 | Snapshot 10k entities | < 0.3 ms | IR-9.1.3 |
| TC-IR-9.1.4.B1 | Clone 10k entities to GameWorld | < 2.0 ms | IR-9.1.4 |
| TC-IR-9.1.6.B1 | Undo 64-step history | < 0.2 ms | IR-9.1.6 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
