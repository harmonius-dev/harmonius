# Scripting ↔ ECS Integration Test Cases

All tests are CI-runnable under `cargo test --package harmonius-integration-tests` and do not
require graphical or platform-exclusive resources. Negative tests assert explicit error variants
rather than panics.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-2.8.1.1 | Graph reads own component | IR-2.8.1 |
| TC-IR-2.8.1.2 | Graph reads other entity | IR-2.8.1 |
| TC-IR-2.8.1.3 | Graph reads missing component | IR-2.8.1 |
| TC-IR-2.8.2.1 | Graph writes component | IR-2.8.2 |
| TC-IR-2.8.2.2 | Multiple writes same component | IR-2.8.2 |
| TC-IR-2.8.2.3 | No mid-system flush visible | IR-2.8.2 |
| TC-IR-2.8.3.1 | Graph spawns entity | IR-2.8.3 |
| TC-IR-2.8.3.2 | Graph despawns entity | IR-2.8.3 |
| TC-IR-2.8.3.3 | Command segment overflow halts | IR-2.8.3 |
| TC-IR-2.8.3.4 | Overflow is deterministic | IR-2.8.3 |
| TC-IR-2.8.4.1 | Parallel graph execution | IR-2.8.4 |
| TC-IR-2.8.4.2 | Conflicting graphs serialize | IR-2.8.4 |
| TC-IR-2.8.4.3 | Fixed-step phase binding | IR-2.8.4 |
| TC-IR-2.8.5.1 | Access sets computed | IR-2.8.5 |
| TC-IR-2.8.5.2 | Access sets refreshed on reload | IR-2.8.5 |
| TC-IR-2.8.5.3 | rkyv roundtrip on descriptor | IR-2.8.5 |
| TC-IR-2.8.6.1 | Entity var reads component | IR-2.8.6 |
| TC-IR-2.8.6.2 | Entity var writes component | IR-2.8.6 |
| TC-IR-2.8.6.3 | Multi-component var read | IR-2.8.6 |
| TC-IR-2.8.6.4 | Multi-component var write | IR-2.8.6 |

1. **TC-IR-2.8.1.1** -- Input: entity with `Health(100)`. Expected: `ctx.read::<Health>()` returns
   `Some(&Health(100))`.
2. **TC-IR-2.8.1.2** -- Input: target entity with `Armor(50)`. Expected: `ctx.read_entity::<Armor>`
   returns `Some(&Armor(50))`.
3. **TC-IR-2.8.1.3** -- Input: entity without `Armor`. Expected: `ctx.read::<Armor>()` returns
   `None`; graph branch takes the null path.
4. **TC-IR-2.8.2.1** -- Input: `ctx.write(e, Damage(10))`. Expected: after command flush the
   `Damage(10)` component is present on `e`.
5. **TC-IR-2.8.2.2** -- Input: two writes to `Health` in one frame. Expected: last write wins in
   worker-id stable order.
6. **TC-IR-2.8.2.3** -- Input: observe world state from a second system running mid-`par_iter`.
   Expected: none of the queued writes are visible until the sync point.
7. **TC-IR-2.8.3.1** -- Input: `ctx.spawn().insert(T)`. Expected: new entity exists after the flush
   with component `T`.
8. **TC-IR-2.8.3.2** -- Input: `ctx.commands.despawn(e)`. Expected: `e` is removed after the flush.
9. **TC-IR-2.8.3.3** -- Input: queue more commands than the `SmallVec` + arena budget allows.
   Expected: the segment returns `GraphError::CommandBufferFull`; no commands from that segment are
   applied.
10. **TC-IR-2.8.3.4** -- Input: replay the overflowing graph with identical world state. Expected:
    error triggers on the same frame and op slot every time.
11. **TC-IR-2.8.4.1** -- Input: 100 `GraphInstance` entities with disjoint access sets. Expected:
    observed parallel work count equals the worker count for the phase.
12. **TC-IR-2.8.4.2** -- Input: two programs that both write `Velocity`. Expected: the scheduler
    runs them in two serial buckets rather than in parallel.
13. **TC-IR-2.8.4.3** -- Input: a simulation graph asset registered in Phase 6. Expected: the
    scheduler refuses the registration with a `PhaseMismatch` error.
14. **TC-IR-2.8.5.1** -- Input: graph that reads `A` and writes `B`. Expected:
    `GraphAccessDescriptor` has `reads={A}` and `writes={B}`.
15. **TC-IR-2.8.5.2** -- Input: reload `.dylib` with a changed access set. Expected: `GraphProgram`
    access descriptor is replaced atomically at the next phase boundary.
16. **TC-IR-2.8.5.3** -- Input: `GraphAccessDescriptor` round-tripped through rkyv
    `Archive`/`Deserialize`. Expected: resulting value equals the original.
17. **TC-IR-2.8.6.1** -- Input: entity variable named `"health"`. Expected: codegen'd read path
    calls `ctx.world.get::<Health>(self)`.
18. **TC-IR-2.8.6.2** -- Input: set entity variable `"speed"`. Expected: `Speed` component updated
    after the sync point.
19. **TC-IR-2.8.6.3** -- Input: variable `"pose"` mapped to `Transform` plus `AnimState`. Expected:
    both components are read into the graph's local state in one logical step.
20. **TC-IR-2.8.6.4** -- Input: set variable `"pose"`. Expected: both `Transform` and `AnimState`
    are written atomically through the command segment.

## Negative and Error Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-2.8.3.N1 | Overflow does NOT flush early | IR-2.8.2, IR-2.8.3 |
| TC-IR-2.8.3.N2 | Overflow logs diagnostic | IR-2.8.3 |
| TC-IR-2.8.3.N3 | Retry resumes from prior variant | IR-2.8.3 |
| TC-IR-2.8.5.N1 | Hot-reload layout mismatch rejected | IR-2.8.5 |
| TC-IR-2.8.5.N2 | Hot-reload layout equal accepted | IR-2.8.5 |
| TC-IR-2.8.6.N1 | Missing component for var | IR-2.8.6 |
| TC-IR-2.8.6.N2 | Multi-component partial presence | IR-2.8.6 |
| TC-IR-2.8.4.N1 | Instruction budget exhausted | IR-2.8.4 |

1. **TC-IR-2.8.3.N1** -- Input: queue commands past the segment capacity while an observer system
   inspects the world. Expected: world state is unchanged until the normal sync point, and the
   overflowing segment is discarded rather than flushed early.
2. **TC-IR-2.8.3.N2** -- Input: capture the engine log sink during an overflow. Expected: exactly
   one `command_buffer_full` event with the offending program id and worker id.
3. **TC-IR-2.8.3.N3** -- Input: advance one frame after an overflow. Expected: the graph resumes
   from its prior `ResumeVariant::AfterYield(n)` rather than restarting.
4. **TC-IR-2.8.5.N1** -- Input: reload a `.dylib` whose `GraphInstanceState` layout hash differs.
   Expected: the reload is rejected, the prior `.dylib` stays loaded, and the editor receives a
   structured error.
5. **TC-IR-2.8.5.N2** -- Input: reload a `.dylib` with the same layout hash. Expected: function
   pointers swap at the next phase boundary with no observable state change.
6. **TC-IR-2.8.6.N1** -- Input: read variable `"armor"` on an entity lacking `Armor`. Expected: the
   read returns `None`; a write is logged and dropped.
7. **TC-IR-2.8.6.N2** -- Input: write variable `"pose"` on an entity that has `Transform` but not
   `AnimState`. Expected: the write is rejected atomically -- neither component is modified.
8. **TC-IR-2.8.4.N1** -- Input: a graph that loops without yielding. Expected: after
   `instruction_budget` back-edges the step returns `StepResult::Error(BudgetExhausted)`.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.8.1.B1 | 10 000 component reads | < 0.1 ms | IR-2.8.1 |
| TC-IR-2.8.2.B1 | 5 000 deferred writes | < 0.5 ms | IR-2.8.2 |
| TC-IR-2.8.3.B1 | 1 000 entity spawns | < 1 ms | IR-2.8.3 |
| TC-IR-2.8.3.B2 | 16-worker segment merge | < 0.2 ms | IR-2.8.3 |
| TC-IR-2.8.4.B1 | 1 000 active graphs | < 4 ms | IR-2.8.4 |
| TC-IR-2.8.5.B1 | Access-set conflict check | < 10 us | IR-2.8.5 |
