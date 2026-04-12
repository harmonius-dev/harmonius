# AI Behavior ↔ Data Tables Integration Test Cases

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.1.3.U1 | Bake-time formula eval | FormulaId col, row inputs | f32 cost baked | IR-2.1.3 |
| TC-IR-2.1.3.U2 | Bake-time formula error | FormulaId missing | Result::Err at bake | IR-2.1.3 |
| TC-IR-2.1.5.U1 | Binding lookup by ColumnId | Sorted binding vec | Binary-search hit | IR-2.1.5 |

1. **TC-IR-2.1.3.U1** -- Pure function test of the codegen'd formula evaluator. Feed a row with
   numeric inputs and a `FormulaId` reference; assert the returned `f32` matches the expected value
   computed by the formula's Rust function.
2. **TC-IR-2.1.3.U2** -- Bake-time verification that an unknown `FormulaId` produces a build error
   via `Result::Err(BakeError::UnknownFormula)` rather than a runtime panic.
3. **TC-IR-2.1.5.U1** -- `BlackboardBindings::find(ColumnId)` performs binary search over its sorted
   vector and returns the matching `BlackboardBinding` index in O(log n) time.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.1.1.1 | BT leaf reads aggro radius | RowRef, col=aggro | BB key = 15.0 | IR-2.1.1 |
| TC-IR-2.1.1.2 | BT leaf missing row | Invalid RowRef | BB key = schema default | IR-2.1.1 |
| TC-IR-2.1.1.N1 | BT leaf bad column type | String col, want f32 | Err, no panic | IR-2.1.1 |
| TC-IR-2.1.2.1 | Utility reads stat col | RowRef + col=strength | Score uses 25.0 | IR-2.1.2 |
| TC-IR-2.1.2.2 | Utility reads null col | Nullable col, null val | Score = 0.0 | IR-2.1.2 |
| TC-IR-2.1.2.N1 | Utility missing table | TableId unregistered | Skipped, logged | IR-2.1.2 |
| TC-IR-2.1.3.1 | GOAP baked cost read | Action row, cost=5.0 | Action cost = 5.0 | IR-2.1.3 |
| TC-IR-2.1.3.2 | GOAP formula baked | Formula col codegen'd | Baked cost correct | IR-2.1.3 |
| TC-IR-2.1.3.3 | GOAP bake pipeline e2e | Table + formula crate | GoapAction.cost=f32 | IR-2.1.3 |
| TC-IR-2.1.3.N1 | GOAP runtime f32 only | GoapAction type | No FormulaId field | IR-2.1.3 |
| TC-IR-2.1.4.1 | Ability RowRef resolves | FK to ability table | Cooldown, range read | IR-2.1.4 |
| TC-IR-2.1.4.2 | Ability FK chain | Row inherits parent | Merged values read | IR-2.1.4 |
| TC-IR-2.1.5.1 | BB binds to column | DatabaseRow + bindings | BB synced on spawn | IR-2.1.5 |
| TC-IR-2.1.5.2 | BB rebinds on reload | TableReloaded event | BB key updated | IR-2.1.5 |
| TC-IR-2.1.5.N1 | BB binding missing col | ColumnId unknown | Default written, logged | IR-2.1.5 |
| TC-IR-2.1.6.1 | Hot reload invalidates | Swap table version+1 | AiTableCache cleared | IR-2.1.6 |
| TC-IR-2.1.6.2 | Reload mid-tick queued | Reload in Phase 4 | Deferred to next frame | IR-2.1.6 |
| TC-IR-2.1.6.N1 | Reload dangling row | Row deleted on reload | Default, logged | IR-2.1.6 |
| TC-DBG.1 | Runtime debug toggle | Toggle ai_data_trace | Extra tracing observed | IR-2.1.1 |

1. **TC-IR-2.1.1.1** -- Real `TableRegistry` with an NPC table; BT tick writes the `aggro_radius`
   column value into the entity's `Blackboard`. No mocks.
2. **TC-IR-2.1.1.N1** -- Negative: the bound column is a `String` but `BtTableLookup` expects `f32`.
   `lookup()` returns `Err(ColumnError::ColumnTypeMismatch { .. })`; system logs and writes the
   column's schema default. No panic, no frame abort.
3. **TC-IR-2.1.2.N1** -- Negative: a referenced `TableId` is not yet registered.
   `TableRegistry::get()` returns `None`; the consideration is skipped this tick and retried next
   tick.
4. **TC-IR-2.1.3.3** -- End-to-end bake pipeline: the data-tables bake crate consumes a table with a
   `FormulaId` cost column, invokes the codegen'd formula function, and emits a baked `GoapAction`
   with an `f32` cost. Runtime loads the baked artifact and executes GOAP planning.
5. **TC-IR-2.1.3.N1** -- Structural test: `std::any::type_name::<GoapAction>` + reflection-free
   compile-time assertion that `GoapAction::cost: f32` (not `FormulaId`). Guards against accidental
   runtime indirection being reintroduced.
6. **TC-IR-2.1.5.N1** -- Negative: a binding references a `ColumnId` that no longer exists after a
   schema edit; `bind_entity` writes the schema default (or `BlackboardValue::None`) and logs once.
7. **TC-IR-2.1.6.N1** -- Negative: a reloaded table drops a row still referenced by a live entity's
   `DatabaseRow`. `rebind_on_reload` writes defaults and logs once per entity.
8. **TC-DBG.1** -- Runtime debug toggle: flip the `ai_data_trace` flag (ECS resource, runtime
   mutable); observe increased trace emission from `BlackboardTableBindingSystem` without
   recompilation. The toggle is never `cfg`-gated.

All tests run in CI via `cargo test -p harmonius-integration-ai-data-tables`. Every test
instantiates a real `TableRegistry`, real `Blackboard`, and real ECS `World` -- no mocking.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.1.1.B1 | 1000 BT leaf lookups | < 0.5 ms | IR-2.1.1 |
| TC-IR-2.1.2.B1 | 500 utility considerations | < 0.3 ms | IR-2.1.2 |
| TC-IR-2.1.3.B1 | GOAP baked cost read (per op) | < 100 ns | IR-2.1.3 |
| TC-IR-2.1.3.B2 | Bake 10k formula rows | < 200 ms | IR-2.1.3 |
| TC-IR-2.1.4.B1 | 200 ability RowRef resolves | < 0.1 ms | IR-2.1.4 |
| TC-IR-2.1.5.B1 | Blackboard-to-column sync (per op) | < 500 ns | IR-2.1.5 |
| TC-IR-2.1.5.B2 | 10k BB rebinds on reload | < 5.0 ms | IR-2.1.5 |
| TC-IR-2.1.6.B1 | Hot reload 10k-row table | < 500 ms | IR-2.1.6 |

1. **TC-IR-2.1.3.B1** -- Runtime hot path: per-operation read of `GoapAction::cost` (already `f32`,
   no indirection) during GOAP planning. Criterion reports time per read; the target is strictly
   under **100 ns** per read. Verifies no per-read formula evaluation at runtime.
2. **TC-IR-2.1.3.B2** -- Bake-time cost of evaluating 10k `FormulaId` cells and writing baked `f32`
   costs into the output artifact. Runs in the content-pipeline stage, not at game runtime.
3. **TC-IR-2.1.5.B1** -- Per-operation blackboard-to-column sync: single
   `BlackboardTableBindingSystem::bind_entity` call for one binding against a warm `AiTableCache`.
   Criterion reports time per sync; the target is strictly under **500 ns** per sync.
4. **TC-IR-2.1.5.B2** -- Hot-reload rebind path: 10k entities with bindings against a reloaded
   table; measure `rebind_on_reload` wall time.

All benchmarks are Criterion-based and run in CI on the `bench` profile. Targets are advisory
budgets, enforced by `harmonius-integration-perf` regression checks.
