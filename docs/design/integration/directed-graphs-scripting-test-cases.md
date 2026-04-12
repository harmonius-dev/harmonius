# Directed Graphs ↔ Scripting Integration Test Cases

All test cases below are unit or integration tests that run in CI via
`cargo test -p harmonius_scripting --features integration` (or the equivalent workspace target).
Benchmarks run via `cargo bench` under `criterion`. No test requires a GPU, display, or external
service.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.7.1.1 | Logic graph stores as DG | 5-node visual graph | DG with 5 nodes | IR-2.7.1 |
| TC-IR-2.7.1.2 | Edge payloads carry types | f32 output -> f32 input | data_type=F32 | IR-2.7.1 |
| TC-IR-2.7.1.3 | Edge type mismatch rejected | f32 out -> i32 in | EdgeTypeMismatch | IR-2.7.1 |
| TC-IR-2.7.2.1 | Topo sort sets eval order | A->B->C chain | Codegen: A,B,C | IR-2.7.2 |
| TC-IR-2.7.2.2 | Diamond topo sort | A->B,A->C,B->D,C->D | D last | IR-2.7.2 |
| TC-IR-2.7.3.1 | Conditional edge compiles | ConditionExpr on edge | if-branch emitted | IR-2.7.3 |
| TC-IR-2.7.3.2 | Static condition elided | Always-true condition | No branch emitted | IR-2.7.3 |
| TC-IR-2.7.4.1 | Traversal maps to step | current_node=NodeId(3) | current_step=3 | IR-2.7.4 |
| TC-IR-2.7.4.2 | Not-started sentinel used | current_node=None | step=GRAPH_..NOT_.. | IR-2.7.4 |
| TC-IR-2.7.4.3 | Hot-reload reset | Version mismatch | State reset, restart | IR-2.7.4 |
| TC-IR-2.7.5.1 | Cycle detected | A->B->C->A | CycleDetected | IR-2.7.5 |
| TC-IR-2.7.5.2 | Self-loop rejected | A->A edge | SelfLoop(A) | IR-2.7.5 |
| TC-IR-2.7.5.3 | Missing node rejected | Ref NodeId(99), 5 nodes | NodeNotFound(99) | IR-2.7.5 |
| TC-IR-2.7.6.1 | Ordered children preserved | Siblings [C,A,B] | Codegen order C,A,B | IR-2.7.6 |
| TC-IR-2.7.6.2 | Reorder updates codegen | Reorder to [A,B,C] | Codegen order A,B,C | IR-2.7.6 |

Test case details:

1. **TC-IR-2.7.1.1** -- Construct a 5-node `LogicGraph`, verify `DirectedGraph::node_count() == 5`
   and every `NodePayload.op` round-trips through rkyv `Archive`/`Deserialize`.
2. **TC-IR-2.7.1.2** -- Build an edge between an `f32` output pin and an `f32` input pin; assert
   `EdgePayload.data_type == ScriptTypeId::F32` after validation.
3. **TC-IR-2.7.1.3** -- Negative: build an edge between an `f32` output and an `i32` input. Expect
   `compile()` to return `Err(GraphError::EdgeTypeMismatch { edge, source: F32, target: I32 })`.
   This exercises the compiler's type-check pass end-to-end.
4. **TC-IR-2.7.2.1** -- Build a `A -> B -> C` chain; assert the compiler emits statements in
   `[A, B, C]` order and the generated Rust source parses.
5. **TC-IR-2.7.2.2** -- Diamond graph `A -> {B, C} -> D`; assert `D` is the last statement in every
   valid topological order and the generated code evaluates `B` and `C` before `D`.
6. **TC-IR-2.7.3.1** -- A `ConditionalGraph` edge carrying a runtime `ConditionExpr` guard must
   lower to an `if` in the emitted Rust source. Assert the `syn::parse_file` of the output contains
   exactly one `ExprIf` over the gated edge.
7. **TC-IR-2.7.3.2** -- Static `ConditionExpr::Const(true)` must be fully eliminated by the
   compile-time condition resolver. Assert the emitted source contains zero `ExprIf` nodes for that
   edge.
8. **TC-IR-2.7.4.1** -- Call
   `traversal_to_step(&GraphTraversalState { current_node: Some(NodeId(3)), .. })` and assert the
   return is `3`.
9. **TC-IR-2.7.4.2** -- Call `traversal_to_step(&GraphTraversalState { current_node: None, .. })`
   and assert the return is `GRAPH_STEP_NOT_STARTED` (`u32::MAX`). Protects against the historic bug
   of silently resuming at NodeId(0).
10. **TC-IR-2.7.4.3** -- Hot-reload recovery (negative + recovery path): a) Compile program v1,
    start a `GraphInstance`, advance two steps. b) Recompile program v2; swap the Scripting
    `GraphProgram` Arc with a new version. c) Step the instance; assert first step returns
    `StepOutcome::Error(GraphError::StaleProgram { expected: 1, found: 2 })`. d)
    `GraphExecutionSystem` resets `GraphTraversalState` (`current_node = None`,
    `started_at = current_tick`) and clears `GraphStateMachine::current_step` to
    `GRAPH_STEP_NOT_STARTED`. e) Step again; assert the instance executes from the entry node with
    no leaked state.
11. **TC-IR-2.7.5.1** -- Build `A -> B -> C -> A`. Assert `DirectedGraph::validate()` returns
    `Err(GraphError::CycleDetected(CycleError { path: [A, B, C, A] }))`.
12. **TC-IR-2.7.5.2** -- Build a single self-edge `A -> A`. Assert `validate()` returns
    `Err(GraphError::SelfLoop(A))`.
13. **TC-IR-2.7.5.3** -- Negative: an `EdgePayload` references `NodeId(99)` in a 5-node graph.
    Assert `compile()` returns `Err(GraphError::NodeNotFound(NodeId(99)))`.
14. **TC-IR-2.7.6.1** -- An `OrderedGraph` with siblings `[C, A, B]`. Assert the compiler emits
    statements in that exact order.
15. **TC-IR-2.7.6.2** -- Mutate the ordering to `[A, B, C]` and recompile. Assert the emitted source
    reflects the new order, proving the compiler reads the ordering each time.

## Benchmarks

| ID | Benchmark | Phase | Target | Req |
|----|-----------|-------|--------|-----|
| TC-IR-2.7.2.B1 | Topo sort 1000-node DAG | Compile | < 1 ms | IR-2.7.2 |
| TC-IR-2.7.3.B1a | Lower 500 conditional edges | Compile | < 5 ms | IR-2.7.3 |
| TC-IR-2.7.3.B1b | Run 500 compiled cond branches | Runtime | < 0.1 ms | IR-2.7.3 |
| TC-IR-2.7.5.B1 | Validate 1000-node DAG | Compile | < 2 ms | IR-2.7.5 |
| TC-IR-2.7.4.B1 | 1000 traversal-state updates | Runtime | < 0.1 ms | IR-2.7.4 |

Benchmark details:

1. **TC-IR-2.7.2.B1** -- `criterion` bench: build a 1000-node DAG with random topology, time
   `DirectedGraph::topological_sort()`. Target < 1 ms on an M-class laptop. Compile-time only.
2. **TC-IR-2.7.3.B1a** -- Compile-time benchmark. Measures the graph compiler lowering 500
   `ConditionalGraph` edges into Rust `if`/`match` statements. Target < 5 ms per compile pass. This
   is the pass that previously was ambiguously labelled "500 conditional edge evals."
3. **TC-IR-2.7.3.B1b** -- Runtime benchmark. Builds a `GraphProgram` containing 500 pre-compiled
   conditional branches, loads it through the middleman `.dylib` (or inlines when shipping), then
   measures end-to-end native execution (no rustc on the hot path). Target < 0.1 ms for the full
   500-branch chain. Confirms that compiled branches run as native machine code.
4. **TC-IR-2.7.5.B1** -- `criterion` bench: validate a 1000-node DAG. Exercises cycle detection and
   edge consistency checks. Target < 2 ms. Compile-time only.
5. **TC-IR-2.7.4.B1** -- Runtime bench: apply 1000 `traversal_to_step` + `step_graph` pure-fn
   updates in a tight loop. No Arc contention, no async, no syscalls. Target < 0.1 ms total.

All benchmarks register under `harmonius_scripting/benches/directed_graphs_scripting.rs` and run in
the same CI pipeline as the integration tests (`cargo bench --no-run` in CI, full runs on the perf
runner).
