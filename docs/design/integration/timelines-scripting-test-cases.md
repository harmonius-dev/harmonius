# Timelines ↔ Scripting Integration Test Cases

All tests below are pure Rust integration tests runnable in CI via `cargo test` with no external
services. Benchmarks use `criterion` on the same harness and are runnable via `cargo bench`.
Negative cases are called out explicitly.

## Integration Tests

Short-key table (IDs and short names only; details are in the numbered list below).

| ID | Short name | Req |
|----|-----------|-----|
| TC-IR-4.9.1.1 | Keyframe fires graph entry | IR-4.9.1 |
| TC-IR-4.9.1.2 | Multiple KFs fire in order | IR-4.9.1 |
| TC-IR-4.9.1.3 | Missing entry point skips (neg) | IR-4.9.1 |
| TC-IR-4.9.2.1 | Graph calls play() | IR-4.9.2 |
| TC-IR-4.9.2.2 | Graph calls pause() | IR-4.9.2 |
| TC-IR-4.9.2.3 | Graph calls seek() | IR-4.9.2 |
| TC-IR-4.9.2.4 | Graph calls stop() | IR-4.9.2 |
| TC-IR-4.9.2.5 | stop() vs pause() (neg) | IR-4.9.2 |
| TC-IR-4.9.3.1 | Wait cont. suspends on complete | IR-4.9.3 |
| TC-IR-4.9.3.2 | Wait cont. resumes on complete | IR-4.9.3 |
| TC-IR-4.9.3.3 | Wait timeout fires (neg) | IR-4.9.3 |
| TC-IR-4.9.3.4 | Default timeout applied | IR-4.9.3 |
| TC-IR-4.9.3.5 | No coroutines in API (neg) | IR-4.9.3 |
| TC-IR-4.9.4.1 | F32 track drives variable | IR-4.9.4 |
| TC-IR-4.9.4.2 | Bool track drives variable | IR-4.9.4 |
| TC-IR-4.9.4.3 | Vec3 track drives variable | IR-4.9.4 |
| TC-IR-4.9.4.4 | Slot type mismatch (neg) | IR-4.9.4 |
| TC-IR-4.9.5.1 | Branch seeks to choice A | IR-4.9.5 |
| TC-IR-4.9.5.2 | Branch seeks to choice B | IR-4.9.5 |
| TC-IR-4.9.5.3 | Default branch on error (neg) | IR-4.9.5 |
| TC-IR-4.9.5.4 | UI prompt spawned via command buffer | IR-4.9.5 |
| TC-IR-4.9.5.5 | ChoiceMade MPSC resumes graph | IR-4.9.5 |
| TC-IR-4.9.6.1 | Graph emits seek event | IR-4.9.6 |
| TC-IR-4.9.6.2 | Seek clamped to duration (neg) | IR-4.9.6 |
| TC-IR-4.9.6.3 | TimelineSeek MPSC full (neg) | IR-4.9.6 |
| TC-IR-4.9.7.1 | Hot-reload preserves variables | FM-7 |
| TC-IR-4.9.7.2 | Hot-reload drops stale slots (neg) | FM-7 |

Test details (input / expected):

1. **TC-IR-4.9.1.1** — Input: an Entity-valued keyframe crosses the playhead for a graph entity.
   Expected: `GraphExecutionSystem` invokes the `on_timeline_event` entry fn once in the same tick.
2. **TC-IR-4.9.1.2** — Input: three keyframes on the same track resolve in one advance step.
   Expected: three `TimelineEventKind::KeyframeCrossed` events are produced in `KeyframeId` order.
3. **TC-IR-4.9.1.3** — Negative. Input: a graph without `on_timeline_event` receives a keyframe
   event. Expected: one warning is logged with the `GraphId`, no panic, no queued event.
4. **TC-IR-4.9.2.1** — Input: a graph node invokes `PlaybackState::play`. Expected:
   `playing == true`, cursor unchanged.
5. **TC-IR-4.9.2.2** — Input: a graph node invokes `PlaybackState::pause`. Expected:
   `playing == false`, cursor unchanged.
6. **TC-IR-4.9.2.3** — Input: graph node calls `PlaybackState::seek(TickCount(180))`. Expected:
   `current_tick == TickCount(180)`, `playing` unchanged.
7. **TC-IR-4.9.2.4** — Input: a graph node invokes `PlaybackState::stop`. Expected:
   `playing == false` AND `current_tick == TickCount(0)`.
8. **TC-IR-4.9.2.5** — Negative. Input: sequence `play, seek(Tick(120)), pause, play, stop`.
   Expected: after `pause` the cursor is `Tick(120)`; after `stop` the cursor is `Tick(0)`.
9. **TC-IR-4.9.3.1** — Input: a graph arms `WaitCondition::TimelineComplete`. Expected:
   `GraphStateMachine::current_step` is frozen while the timeline is still running.
10. **TC-IR-4.9.3.2** — Input: the awaited timeline emits `TimelineComplete`. Expected: the state
    machine advances its `current_step` on the following tick and no further.
11. **TC-IR-4.9.3.3** — Negative. Input: the awaited timeline never completes, `timeout` set to 10
    ticks. Expected: after 10 ticks the step returns
    `StepOutcome::Error(RuntimeError::WaitTimeout)`.
12. **TC-IR-4.9.3.4** — Input: `WaitCondition::TimelineComplete { timeout: None }`. Expected:
    `DEFAULT_WAIT_TIMEOUT` (`TickCount(600)`) is used and a timeout fires at tick 600.
13. **TC-IR-4.9.3.5** — Negative. Input: grep for `async`, `await`, `coroutine`, `Coroutine`.
    Expected: zero matches inside `harmonius_scripting` and this integration crate.
14. **TC-IR-4.9.4.1** — Input: a `TrackValue::F32` track sampled at `0.5`. Expected: the bound
    `VariableStore` slot reads `0.5_f32` after the tick.
15. **TC-IR-4.9.4.2** — Input: a `TrackValue::Bool` track sampled at `true`. Expected: the bound
    slot reads `true`.
16. **TC-IR-4.9.4.3** — Input: a `TrackValue::Vec3` track sampled at `(1.0, 2.0, 3.0)`. Expected:
    the bound slot reads the same `Vec3`.
17. **TC-IR-4.9.4.4** — Negative. Input: attempt to bind a `TrackValue::Bool` track to an `F32`
    slot. Expected: `BindError::TypeMismatch { track, slot, expected, actual }` is returned; zero
    samples are written to the slot.
18. **TC-IR-4.9.5.1** — Input: the branch condition evaluates to "A". Expected: `PlaybackState`
    seeks to `tick_A`.
19. **TC-IR-4.9.5.2** — Input: the branch condition evaluates to "B". Expected: `PlaybackState`
    seeks to `tick_B`.
20. **TC-IR-4.9.5.3** — Negative. Input: the condition returns `StepOutcome::Error`. Expected: the
    default (first topologically-ordered) branch is taken and the error is logged.
21. **TC-IR-4.9.5.4** — Input: a graph node branches at a cutscene keyframe. Expected: a
    `ChoicePromptEntity` is visible to UI queries after the command-apply frame boundary.
22. **TC-IR-4.9.5.5** — Input: UI writes `ChoiceMadeEvent(B)` into the MPSC channel. Expected: the
    graph drains the MPSC on its next step and calls `PlaybackState::seek(tick_B)`.
23. **TC-IR-4.9.6.1** — Input: a graph emits `TimelineSeekEvent { target_tick: TickCount(300) }`.
    Expected: `current_tick == TickCount(300)` after the next advance.
24. **TC-IR-4.9.6.2** — Negative. Input: `target_tick = TickCount(99999)`, duration is
    `TickCount(600)`. Expected: `current_tick == TickCount(600)` (clamped).
25. **TC-IR-4.9.6.3** — Negative. Input: 257 seek events in one tick against a channel of capacity
    256. Expected: the 257th `try_send` returns `TrySendError::Full(event)` and logs a warning.
26. **TC-IR-4.9.7.1** — Input: the middleman `.dylib` is hot-reloaded mid-cutscene. Expected:
    `VariableStore` entries whose `SlotId` survives the new layout are preserved; the graph's
    `current_step` resets to `GRAPH_STEP_NOT_STARTED`.
27. **TC-IR-4.9.7.2** — Negative. Input: hot-reload removes a `SlotId` that was in use. Expected:
    the dropped entry is logged once per entity, no panic, and execution resumes from the entry
    node.

## Benchmarks

Benchmarks run under `criterion` in CI at 60 Hz simulation budget.

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.9.1.B1 | 100 graph events dispatched per tick | < 0.5 ms | IR-4.9.1 |
| TC-IR-4.9.2.B1 | 1000 play/pause/stop/seek calls | < 0.2 ms | IR-4.9.2 |
| TC-IR-4.9.3.B1 | 50 wait-continuation resume checks | < 0.1 ms | IR-4.9.3 |
| TC-IR-4.9.4.B1 | 32 track-to-variable bindings sampled | < 0.05 ms | IR-4.9.4 |
| TC-IR-4.9.5.B1 | 16 branching cutscene choice steps | < 0.3 ms | IR-4.9.5 |
| TC-IR-4.9.6.B1 | Drain a 256-event TimelineSeek MPSC | < 0.1 ms | IR-4.9.6 |

## Notes

- All `TrackId`, `SlotId`, `TickCount`, `EventId` values in tests use the newtype wrappers; raw
  integers are rejected at compile time.
- Tests that exercise hot-reload link against a stub middleman `.dylib` built by the test harness.
- Negative tests are explicitly marked `(neg)` and assert error paths, clamping, warning logs, or
  grep-style API invariants (e.g., "no coroutines anywhere").
