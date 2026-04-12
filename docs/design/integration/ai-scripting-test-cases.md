# AI Behavior ↔ Scripting Integration Test Cases

All tests below are runnable in CI; none require a GPU, network, or interactive visual editor. Tests
that exercise the graph compiler drive it via its library entry point with source fixtures checked
into the repository. Tests that exercise the middleman `.dylib` build a minimal fixture crate via
`cargo` in a temporary target directory.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-2.4.1.1 | BT graph compiles | IR-2.4.1 |
| TC-IR-2.4.1.2 | BT graph runs as system | IR-2.4.1 |
| TC-IR-2.4.1.3 | BT graph native dispatch | IR-2.4.1 |
| TC-IR-2.4.2.1 | Utility curve codegen | IR-2.4.2 |
| TC-IR-2.4.2.2 | Custom consideration | IR-2.4.2 |
| TC-IR-2.4.2.3 | Score fn is pure | IR-2.4.2 |
| TC-IR-2.4.3.1 | BT leaf invokes graph | IR-2.4.3 |
| TC-IR-2.4.3.2 | BT leaf returns failure | IR-2.4.3 |
| TC-IR-2.4.3.3 | Registry bypass | IR-2.4.3 |
| TC-IR-2.4.4.1 | GOAP executes graph | IR-2.4.4 |
| TC-IR-2.4.4.2 | GOAP multi-step plan | IR-2.4.4 |
| TC-IR-2.4.4.3 | GOAP no entry_name | IR-2.4.4 |
| TC-IR-2.4.5.1 | Hot reload preserves | IR-2.4.5 |
| TC-IR-2.4.5.2 | Hot reload version bump | IR-2.4.5 |
| TC-IR-2.4.5.3 | Reload at phase boundary | IR-2.4.5 |
| TC-IR-2.4.6.1 | State machine suspends | IR-2.4.6 |
| TC-IR-2.4.6.2 | State machine resumes | IR-2.4.6 |
| TC-IR-2.4.6.3 | No coroutine in build | IR-2.4.6 |
| TC-IR-2.4.BB.1 | BB→Vars sync | IR-2.4.3 |
| TC-IR-2.4.BB.2 | Vars→BB sync | IR-2.4.3 |
| TC-IR-2.4.BB.3 | Bridge no HashMap walk | IR-2.4.3 |
| TC-IR-2.4.ADPT.1 | Step-to-status mapping | IR-2.4.3 |
| TC-IR-2.4.OWN.1 | Shared `FnPtrTable` | IR-2.4.1 |

### Inputs and expected outputs

1. **TC-IR-2.4.1.1** -- Input: visual BT asset. Expected: compiled `GraphProgram` with an `on_tick`
   entry point listed in `GraphProgram::entry_points`.
2. **TC-IR-2.4.1.2** -- Input: entity with a `GraphInstance` component referencing the compiled
   program. Expected: the BT is evaluated exactly once per frame by the AI phase system.
3. **TC-IR-2.4.1.3** -- Input: compiled middleman `.dylib`. Expected: dispatch path is a direct
   fn-pointer call with no bytecode interpreter loop on any profiled frame.
4. **TC-IR-2.4.2.1** -- Input: linear curve graph. Expected: emitted Rust source contains a
   `fn score(&GraphInstanceState, &ExecutionContext) -> f32` compiled into the `.dylib`.
5. **TC-IR-2.4.2.2** -- Input: consideration graph that reads an ECS component. Expected: score
   returned matches the component value from the fixture.
6. **TC-IR-2.4.2.3** -- Input: `UtilityScoreFn` type alias. Expected: takes `&GraphInstanceState`
   (immutable), not `&mut`.
7. **TC-IR-2.4.3.1** -- Input: BT with a leaf whose `fn_idx = 0`. Expected: the `GraphFn` at index 0
   is invoked and the leaf resolves to `NodeStatus::Success`.
8. **TC-IR-2.4.3.2** -- Input: `GraphFn` that returns `StepResult::Error(_)`. Expected: leaf
   resolves to `NodeStatus::Failure`.
9. **TC-IR-2.4.3.3** -- Input: codegen'd leaf dispatch. Expected: zero `HashMap::get` calls are
   recorded during 1000 ticks (registry bypass).
10. **TC-IR-2.4.4.1** -- Input: GOAP plan step with `fn_idx = 2`. Expected: the corresponding
    `GraphFn` runs and mutates the fixture component.
11. **TC-IR-2.4.4.2** -- Input: 3-step GOAP plan. Expected: all three `GraphFn`s are invoked in plan
    order, once each.
12. **TC-IR-2.4.4.3** -- Input: `GoapGraphAction` instance. Expected: the struct contains only
    `fn_idx` (no `entry_name` field).
13. **TC-IR-2.4.5.1** -- Input: modify a graph source and reload the `.dylib`. Expected: the
    existing `GraphInstance::variables` survive the reload with matching slot contents.
14. **TC-IR-2.4.5.2** -- Input: new `.dylib` version. Expected: `GraphInstance::needs_reload()`
    returns `true` for every stale instance.
15. **TC-IR-2.4.5.3** -- Input: reload request fired during Phase 4. Expected: the reload is
    deferred to the next Phase 1 boundary and then applied.
16. **TC-IR-2.4.6.1** -- Input: patrol graph with 3 waypoints. Expected: the graph suspends with
    `StepResult::Yield(NextFrame)` between each waypoint.
17. **TC-IR-2.4.6.2** -- Input: suspended `GraphInstance` on frame N. Expected: frame N+1 resumes
    the function at the saved `resume_variant` via `match` dispatch.
18. **TC-IR-2.4.6.3** -- Input: emitted Rust source for a multi-frame graph. Expected: no `async`,
    no `await`, no coroutine tokens appear in the emitted source.
19. **TC-IR-2.4.BB.1** -- Input: dirty `BlackboardKey`. Expected: after `sync_to_vars`, the matching
    `VariableStore` slot contains the Blackboard value.
20. **TC-IR-2.4.BB.2** -- Input: modified `VariableStore` slot. Expected: after `sync_to_bb`, the
    matching `Blackboard` key contains the new value.
21. **TC-IR-2.4.BB.3** -- Input: 1000 bridge ticks. Expected: zero `HashMap::get` calls are recorded
    by the allocation / call tracker.
22. **TC-IR-2.4.ADPT.1** -- Input: each variant of `StepResult`. Expected: `step_to_status` returns
    the `NodeStatus` listed in the mapping table of the design document.
23. **TC-IR-2.4.OWN.1** -- Input: three AI systems (`bt_tick_system`, `utility_tick_system`,
    `goap_tick_system`). Expected: all three resolve `Res<FnPtrTable>` to the same pointer.

## Negative / Error Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-2.4.NEG.1 | Compile error preserves prior .dylib | IR-2.4.1 |
| TC-IR-2.4.NEG.2 | Invalid node connection | IR-2.4.1 |
| TC-IR-2.4.NEG.3 | Curve with invalid domain | IR-2.4.2 |
| TC-IR-2.4.NEG.4 | `fn_idx` out of range | IR-2.4.3 |
| TC-IR-2.4.NEG.5 | `UtilityScoreTable` OOB | IR-2.4.2 |
| TC-IR-2.4.NEG.6 | Stale `resume_variant` | IR-2.4.5 |
| TC-IR-2.4.NEG.7 | Missing BB mapping key | IR-2.4.3 |
| TC-IR-2.4.NEG.8 | `needs_reload` migration | IR-2.4.5 |
| TC-IR-2.4.NEG.9 | Graph returns Error | IR-2.4.3 |
| TC-IR-2.4.NEG.10 | Cycle in BT asset | IR-2.4.1 |

### Scenarios

1. **TC-IR-2.4.NEG.1** -- Load a valid graph fixture and compile to `.dylib` v1. Then load a fixture
   with a deliberate type error and recompile. Assert: (a) compile returns `Err`, (b)
   `Res<FnPtrTable>` still points at v1, (c) `GraphProgram::version` is unchanged, (d) subsequent
   `BtGraphLeaf` dispatch still succeeds against v1.
2. **TC-IR-2.4.NEG.2** -- Author a graph connecting an `f32` output to a `bool` input. Expected:
   compiler returns `Err(PortTypeMismatch)` and does not emit a `.dylib`.
3. **TC-IR-2.4.NEG.3** -- Author a utility curve whose domain is `[1.0, -1.0]` (inverted). The
   compiler's curve validator rejects it. Assert the error payload names the invalid curve node so
   the editor can highlight it.
4. **TC-IR-2.4.NEG.4** -- Construct a `BtGraphLeaf` with `fn_idx = u32::MAX`. Tick the tree. Assert
   `NodeStatus::Failure` is returned from that leaf and a log entry contains both the entity ID and
   the offending index.
5. **TC-IR-2.4.NEG.5** -- Construct a consideration with a `score_fn_idx` past the end of
   `UtilityScoreTable`. Expected: score of `0.0` assigned, error logged, no crash.
6. **TC-IR-2.4.NEG.6** -- Suspend a graph on `resume_variant = 3`, reload with a new `.dylib` in
   which the state machine has only 2 variants. Assert: migration resets the instance to variant 0
   with cleared `saved_locals`, and the next tick runs from the top of the sequence.
7. **TC-IR-2.4.NEG.7** -- Call `BlackboardBridge::sync_to_vars` with a `BlackboardKey` not present
   in `BbVarMapping`. Expected: key skipped, debug warn logged, no crash.
8. **TC-IR-2.4.NEG.8** -- Reload with a new `.dylib` whose `VariableStore` layout adds a slot.
   Assert: matching slots are migrated, new slot initialized to default.
9. **TC-IR-2.4.NEG.9** -- Graph returns `StepResult::Error(RuntimeError::ComponentNotFound)`.
   Expected: leaf resolves to `NodeStatus::Failure`, BT parent Selector advances to next child.
10. **TC-IR-2.4.NEG.10** -- Feed a BT asset whose leaves form a cycle through macro-node expansion.
    The compiler must reject before codegen and produce a diagnostic naming the cycle.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.4.1.B1 | 1000 BT graph ticks | < 4 ms | IR-2.4.1 |
| TC-IR-2.4.1.B2 | Direct vs registry HashMap | > 4x faster | IR-2.4.1 |
| TC-IR-2.4.2.B1 | 500 utility score calls | < 0.5 ms | IR-2.4.2 |
| TC-IR-2.4.3.B1 | BB↔Vars sync 256 keys | < 50 us | IR-2.4.3 |
| TC-IR-2.4.4.B1 | GOAP 3-step, 100 agents | < 1 ms | IR-2.4.4 |
| TC-IR-2.4.5.B1 | Hot reload turnaround | < 1 s | IR-2.4.5 |
| TC-IR-2.4.6.B1 | 200 state-machine resumes | < 0.2 ms | IR-2.4.6 |

All benchmarks run via `cargo bench` in CI with a fixed-seed fixture workload and assert the
measured mean against the target listed above. A regression > 10% fails the build.
