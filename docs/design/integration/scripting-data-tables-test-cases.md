# Scripting ↔ Data Tables Integration Test Cases

All tests below are CI-runnable under `cargo test -p harmonius_scripting_data_tables_integration`.
Negative tests exercise every Failure Mode documented in the design. Benchmarks gate the performance
budget section of the companion design.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-2.9.1.1 | Formula column stored as logic graph | IR-2.9.1 |
| TC-IR-2.9.1.2 | Formula reads same-row columns | IR-2.9.1 |
| TC-IR-2.9.2.1 | Formula graph compiles to Rust fn | IR-2.9.2 |
| TC-IR-2.9.2.2 | Static output type emitted | IR-2.9.2 |
| TC-IR-2.9.2.3 | No type-erased Value anywhere | IR-2.9.2 |
| TC-IR-2.9.3.1 | Formula reads FK row | IR-2.9.3 |
| TC-IR-2.9.3.2 | Formula resolves FK chain | IR-2.9.3 |
| TC-IR-2.9.3.3 | Row field access is plain read | IR-2.9.3 |
| TC-IR-2.9.4.1 | Logic graph reads table | IR-2.9.4 |
| TC-IR-2.9.4.2 | Logic graph reads via entity binding | IR-2.9.4 |
| TC-IR-2.9.5.1 | Table reload re-evaluates formulas | IR-2.9.5 |
| TC-IR-2.9.5.2 | Formula .dylib reload re-bakes tables | IR-2.9.5 |
| TC-IR-2.9.5.3 | `TableReloaded` entity event fires | IR-2.9.5 |
| TC-IR-2.9.5.4 | .dylib loaded on main thread only | IR-2.9.5 |
| TC-IR-2.9.5.5 | Snapshot swap is atomic | IR-2.9.5 |
| TC-IR-2.9.6.1 | Cycle detected at bake | IR-2.9.6 |
| TC-IR-2.9.6.2 | Type mismatch rejected | IR-2.9.6 |
| TC-IR-2.9.6.3 | Sandbox rejects unsafe | IR-2.9.6 |

Inputs and expected outputs:

1. **TC-IR-2.9.1.1** -- Input: `damage` formula column authored in editor. Expected:
   `DirectedGraph<NodePayload, EdgePayload>` stored on the column definition asset.
2. **TC-IR-2.9.1.2** -- Input: formula summing `base_dmg` and `bonus` columns on the same row.
   Expected: computed cell equals the sum of both column values.
3. **TC-IR-2.9.2.1** -- Input: authored formula graph asset. Expected: monomorphized
   `formula_<table>_<col>` symbol resolvable in the middleman .dylib `FormulaFnTable`.
4. **TC-IR-2.9.2.2** -- Input: formula with `f32` output and an `f32` target column. Expected:
   direct `fn(&WeaponsRow, &TableRegistry) -> f32` invocation, no `Value` wrapper.
5. **TC-IR-2.9.2.3** -- Input: one `i32` formula and one `f32` formula in the same .dylib. Expected:
   two distinct, concretely-typed symbols; neither routes through a type-erased `Value`.
6. **TC-IR-2.9.3.1** -- Input: formula with foreign-key column referencing the `materials` table.
   Expected: formula body reads `material.weight` via codegen'd accessor.
7. **TC-IR-2.9.3.2** -- Input: formula chasing FK → FK (weapon → material → origin). Expected: two
   levels of `resolve_foreign_key()` succeed and return the final concrete row.
8. **TC-IR-2.9.3.3** -- Input: codegen'd `WeaponsRow` struct. Expected: no branches, no hash
   lookups, no dynamic type checks in the emitted field-access code.
9. **TC-IR-2.9.4.1** -- Input: logic graph containing a `TableLookupNode`. Expected: node expands to
   a direct codegen'd accessor call returning a concrete `&<Table>Row`.
10. **TC-IR-2.9.4.2** -- Input: logic graph with `RowSource::EntityBinding` driven by a spawned
    entity carrying a `DatabaseRow` component. Expected: row data pulled from the component's
    `(table, row)` pair.
11. **TC-IR-2.9.5.1** -- Input: modify `base_dmg`, reload the `weapons` table. Expected: dependent
    formula cells are recomputed in the new snapshot before the swap.
12. **TC-IR-2.9.5.2** -- Input: publish a new formula .dylib. Expected: affected table cells are
    recomputed for every row in the next pre-frame window.
13. **TC-IR-2.9.5.3** -- Input: successful table hot-reload. Expected: `TableReloaded` entity event
    dispatched on the `TableRegistry` entity with the new registry version.
14. **TC-IR-2.9.5.4** -- Input: worker thread attempts to call the .dylib loader. Expected:
    compile-time rejection (loader API only exposed to the main thread).
15. **TC-IR-2.9.5.5** -- Input: worker reads `&TableRegistry` mid-swap. Expected: worker observes
    either the old or the new snapshot in full, never a partial mix.
16. **TC-IR-2.9.6.1** -- Input: formula A references formula B, formula B references A. Expected:
    bake-time topological sort reports `BakeError::CycleDetected`.
17. **TC-IR-2.9.6.2** -- Input: formula outputs `f32` into an `i32` column. Expected: compile-time
    rejection with `BakeError::TypeMismatch`; no new .dylib produced.
18. **TC-IR-2.9.6.3** -- Input: formula containing an `unsafe` block. Expected:
    `BakeError::SandboxViolation` from the sandbox pass; compile rejected.

## Negative Tests

| ID | Failure mode | Req |
|----|--------------|-----|
| TC-IR-2.9.FM1 | Formula cycle (FM-1) | IR-2.9.6 |
| TC-IR-2.9.FM2 | FK target missing (FM-2) | IR-2.9.3 |
| TC-IR-2.9.FM3 | Type mismatch (FM-3) | IR-2.9.6 |
| TC-IR-2.9.FM4 | Formula compile error (FM-4) | IR-2.9.5 |
| TC-IR-2.9.FM5 | dlopen failure (FM-5) | IR-2.9.5 |
| TC-IR-2.9.FM6 | Unresolved symbol (FM-6) | IR-2.9.5 |
| TC-IR-2.9.FM7 | Reload race (FM-7) | IR-2.9.5 |

Inputs and expected outputs:

1. **TC-IR-2.9.FM1** -- Input: `A.out` reads `B.out` which reads `A.out`. Expected:
   `BakeError::CycleDetected` with the cycle path; no cell written.
2. **TC-IR-2.9.FM2** -- Input: formula references a dangling `RowRef`. Expected: codegen'd default
   used, `BakeOutcome::DefaultUsed`, and an event-log entry recorded.
3. **TC-IR-2.9.FM3** -- Input: `f32` formula bound to an `i32` column. Expected:
   `BakeError::TypeMismatch`; previous .dylib preserved.
4. **TC-IR-2.9.FM4** -- Input: graph that produces a Rust source failing to compile. Expected:
   previous versioned `formula_v{N}.dylib` retained; `FormulaFnTable` version unchanged.
5. **TC-IR-2.9.FM5** -- Input: corrupt `.dylib` file on disk. Expected: `dlopen` returns error;
   active `FormulaFnTable` unchanged; no `FormulaDylibReloaded` dispatched.
6. **TC-IR-2.9.FM6** -- Input: `dlsym` returns null for a `FormulaId`. Expected: slot marked
   `Rejected(UnresolvedSymbol)`; affected column falls back to previous snapshot value.
7. **TC-IR-2.9.FM7** -- Input: table data change and formula .dylib change published in the same
   pre-frame window. Expected: .dylib loaded first, then table re-baked against the new .dylib, then
   snapshot swapped, then events dispatched -- strict order preserved.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.9.2.B1 | 10k formula evaluations | < 1 ms | IR-2.9.2 |
| TC-IR-2.9.2.B2 | Single formula invocation | < 100 ns | IR-2.9.2 |
| TC-IR-2.9.3.B1 | 5k FK resolutions in formulas | < 0.5 ms | IR-2.9.3 |
| TC-IR-2.9.3.B2 | Single FK resolve | < 20 ns | IR-2.9.3 |
| TC-IR-2.9.4.B1 | 1000 table-lookup nodes | < 0.5 ms | IR-2.9.4 |
| TC-IR-2.9.4.B2 | Single table-lookup node | < 10 ns | IR-2.9.4 |
| TC-IR-2.9.5.B1 | Hot reload of 10k-row table | < 500 ms | IR-2.9.5 |
| TC-IR-2.9.5.B2 | .dylib dlopen + symbol resolve | < 50 ms | IR-2.9.5 |
