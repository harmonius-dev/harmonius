# Profiler ↔ Game Loop Integration Test Cases

## Upstream Requirements Trace

Each integration requirement below corresponds to upstream profiler and game-loop requirements.
Regressions in any test case below also constitute regressions against the listed upstream IDs.

| IR-ID    | Upstream R-IDs          |
|----------|-------------------------|
| IR-5.6.1 | R-15.5.1, R-5.6.1       |
| IR-5.6.2 | R-15.5.1, R-5.6.1       |
| IR-5.6.3 | R-15.5.1, R-5.6.1       |
| IR-5.6.4 | R-15.5.1, R-5.6.1       |
| IR-5.6.5 | R-15.5.1, R-5.6.6       |
| IR-5.6.6 | R-15.5.1, R-5.6.3       |
| IR-5.6.7 | R-15.5.6, R-5.6.1       |

## Unit Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.6.1.U1 | `CpuScopeGuard::new` emits begin event | IR-5.6.1 |
| TC-IR-5.6.1.U2 | `CpuScopeGuard::drop` emits end event | IR-5.6.1 |
| TC-IR-5.6.3.U1 | `SpikeRing::push` wraps at capacity 32 | IR-5.6.3 |
| TC-IR-5.6.3.U2 | `detect_spikes` returns borrowed `SpikeRingView` | IR-5.6.3 |
| TC-IR-5.6.4.U1 | `ProfileRingBuffer::drain_into` is zero-alloc | IR-5.6.4 |
| TC-IR-5.6.4.U2 | `FrameArena::reset` clears arena between frames | IR-5.6.4 |
| TC-IR-5.6.7.U1 | `LatestFrameCapture` resource replaced each frame | IR-5.6.7 |

1. **TC-IR-5.6.1.U1** -- Construct a `CpuScopeGuard`, inspect the thread ring buffer, confirm one
   partial event with a valid `begin_tsc` is present.
2. **TC-IR-5.6.1.U2** -- Drop a `CpuScopeGuard`, confirm the same event now has a valid `end_tsc`
   greater than its `begin_tsc`.
3. **TC-IR-5.6.3.U1** -- Push 40 `SpikeEntry` values into a `SpikeRing` of capacity 32. Verify the
   oldest 8 were overwritten and the newest 32 are retained in insertion order.
4. **TC-IR-5.6.3.U2** -- Call `detect_spikes` on a `FrameCapture` allocated inside a `FrameArena`.
   Verify the returned `SpikeRingView` lives inside the arena and performs no heap allocations
   (measured via a counting allocator).
5. **TC-IR-5.6.4.U1** -- Pre-fill a ring buffer with 1024 events, call `drain_into(&mut arena)`
   under a counting allocator, assert `allocations == 0`.
6. **TC-IR-5.6.4.U2** -- Fill an arena, call `reset`, confirm `remaining == capacity` and the next
   `alloc_slice` returns a slice at offset 0.
7. **TC-IR-5.6.7.U1** -- Run two frames, capture the `Res<LatestFrameCapture>` pointer from each,
   and verify the second frame replaces the first (distinct `frame_number`).

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.6.1.1 | Phase 1 Input timed | IR-5.6.1 |
| TC-IR-5.6.1.2 | All 8 phases timed | IR-5.6.1 |
| TC-IR-5.6.1.3 | Nested system scopes | IR-5.6.1 |
| TC-IR-5.6.1.4 | Phase scope wraps fork-join | IR-5.6.1 |
| TC-IR-5.6.2.1 | Frame budget breakdown | IR-5.6.2 |
| TC-IR-5.6.2.2 | Budget tracks over time | IR-5.6.2 |
| TC-IR-5.6.3.1 | Spike on slow phase | IR-5.6.3 |
| TC-IR-5.6.3.2 | No spike within budget | IR-5.6.3 |
| TC-IR-5.6.3.3 | Spike includes frame number | IR-5.6.3 |
| TC-IR-5.6.4.1 | Ring buffer drained at boundary | IR-5.6.4 |
| TC-IR-5.6.4.2 | Events sorted by timestamp | IR-5.6.4 |
| TC-IR-5.6.4.3 | Drain uses FrameArena, zero alloc | IR-5.6.4 |
| TC-IR-5.6.5.1 | Per-system timing recorded | IR-5.6.5 |
| TC-IR-5.6.5.2 | System timing matches wall clock | IR-5.6.5 |
| TC-IR-5.6.6.1 | Substep profiling | IR-5.6.6 |
| TC-IR-5.6.6.2 | Substep durations sum correctly | IR-5.6.6 |
| TC-IR-5.6.7.1 | StatOverlays reads LatestFrameCapture | IR-5.6.7 |
| TC-IR-5.6.7.2 | FrameStats entity count | IR-5.6.7 |

### Inputs and Expected Outputs

1. **TC-IR-5.6.1.1** -- Run 1 frame with input. Expect a `CpuEvent` whose zone name hash equals
   `hash("Phase1_Input")` in the capture.
2. **TC-IR-5.6.1.2** -- Run 1 frame. Expect exactly 8 phase-level `CpuEvent` entries matching the
   `Phase` enum discriminants (excluding `Custom`).
3. **TC-IR-5.6.1.3** -- Run a phase containing 3 systems. Expect 3 child `CpuEvent` entries whose
   `[begin_tsc, end_tsc]` intervals fall inside the parent phase event's interval.
4. **TC-IR-5.6.1.4** -- Schedule a phase with parallel workers. Verify the phase-level scope begins
   before any worker fires and ends after the last worker returns (fork-join boundary).
5. **TC-IR-5.6.2.1** -- Set a 16.6 ms frame target. Compute per-phase percentages from the capture.
   Expect the sum over all 8 phases to equal 100% (+/- 0.1%).
6. **TC-IR-5.6.2.2** -- Run 100 frames with deterministic workloads. Expect the rolling average
   per-phase duration to stabilize within 1% of the steady-state value.
7. **TC-IR-5.6.3.1** -- Budget Phase 5 Physics at 4 ms, inject a workload that takes 10 ms. Expect
   one `SpikeEntry` in the capture with `phase == PhysicsStep`, `duration_ms ≈ 10.0`,
   `budget_ms == 4.0`.
8. **TC-IR-5.6.3.2** -- All phases under budget. Expect `SpikeRingView.count == 0`.
9. **TC-IR-5.6.3.3** -- Trigger a spike on frame 42. Expect the emitted `SpikeEntry` to have
   `frame_number == 42`.
10. **TC-IR-5.6.4.1** -- 50 events across 4 worker threads (not counting main/render, per the parent
    profiler's worker-only instrumentation). Expect all 50 events in the final
    `FrameCapture.cpu_events`.
11. **TC-IR-5.6.4.2** -- Mix events from multiple worker threads with interleaved timestamps. Expect
    `FrameCapture.cpu_events` sorted ascending by `begin_tsc`.
12. **TC-IR-5.6.4.3** -- Run `collect_frame` under a counting allocator. Expect `allocations == 0`
    in the drain/sort/publish path.
13. **TC-IR-5.6.5.1** -- 10 ECS systems in Phase 3. Expect 10 `CpuEvent` entries whose zone name
    hashes match the system names.
14. **TC-IR-5.6.5.2** -- Busy-wait a system for 1 ms. Expect the recorded `CpuEvent` duration to be
    within 0.1 ms of 1 ms.
15. **TC-IR-5.6.6.1** -- Run 4 physics substeps in one Phase 5. Expect 4 `CpuEvent` entries whose
    zone name hashes match `"Physics_Substep"`.
16. **TC-IR-5.6.6.2** -- Each substep ~1 ms. Expect the Phase 5 parent event duration within 0.2 ms
    of 4 ms.
17. **TC-IR-5.6.7.1** -- Run 1 frame. Read `Res<LatestFrameCapture>` from `StatOverlays`. Expect
    displayed FPS and frame time derived from `capture.stats`.
18. **TC-IR-5.6.7.2** -- World with 500 entities. Expect
    `LatestFrameCapture.capture.stats.entity_count == 500`.

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.6.1.N1 | Disabled collector emits empty capture | IR-5.6.1 |
| TC-IR-5.6.1.N2 | Runtime toggle off then on resumes on next frame | IR-5.6.1 |
| TC-IR-5.6.3.N1 | Budget unset skips spike check for phase | IR-5.6.3 |
| TC-IR-5.6.3.N2 | False positive filtered by rolling average | IR-5.6.3 |
| TC-IR-5.6.4.N1 | Ring buffer full sets `events_dropped` flag | IR-5.6.4 |
| TC-IR-5.6.4.N2 | Unregistered thread pushes are ignored | IR-5.6.4 |
| TC-IR-5.6.4.N3 | TSC regression clamped to zero duration | IR-5.6.4 |
| TC-IR-5.6.4.N4 | Arena overflow truncates capture | IR-5.6.4 |
| TC-IR-5.6.5.N1 | System panic does not leave open scope | IR-5.6.5 |

1. **TC-IR-5.6.1.N1** -- Call `set_enabled(false)` then run a frame. Expect
   `FrameCapture.cpu_events.is_empty()` and `FrameStats` populated only with `frame_number`.
2. **TC-IR-5.6.1.N2** -- Call `set_enabled(false)`, run 2 frames, call `set_enabled(true)`, run 1
   more frame. Expect the third frame's capture to contain all phase events with no residual events
   from the disabled frames.
3. **TC-IR-5.6.3.N1** -- Leave Phase 4 AI budget at 0.0 ms. Run a frame where Phase 4 takes 20 ms.
   Expect zero spike entries (budget unset -> skip).
4. **TC-IR-5.6.3.N2** -- One-frame hitch in an otherwise stable 100-frame run. Expect the
   rolling-average spike detector to not flag the isolated frame.
5. **TC-IR-5.6.4.N1** -- Push events until `ProfileRingBuffer::push` returns false. Expect the
   buffer's `events_dropped()` flag to become true and no panic or allocation to occur.
6. **TC-IR-5.6.4.N2** -- Construct a `CpuScopeGuard` on a thread that has not called
   `register_thread`. Expect the event to be discarded silently, a one-time warning logged, and the
   capture to be unaffected.
7. **TC-IR-5.6.4.N3** -- Inject an event with `end_tsc < begin_tsc`. Expect the collector to clamp
   the duration to zero and set the event's warning bit.
8. **TC-IR-5.6.4.N4** -- Push enough events to overflow the `FrameArena`. Expect the capture to be
   truncated at the arena boundary with `FrameStats.profiler_truncated == true`; no heap allocation
   occurs.
9. **TC-IR-5.6.5.N1** -- A system panics mid-execution. Expect the enclosing `CpuScopeGuard`'s
   `drop` to still record an end event via stack unwinding, leaving no orphaned scopes in the
   capture.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.6.1.B1 | `CpuScopeGuard` begin+end | < 20 ns | IR-5.6.1 |
| TC-IR-5.6.1.B2 | Disabled-path guard overhead | < 3 ns | IR-5.6.1 |
| TC-IR-5.6.4.B1 | `FrameCollector` drain 1000 events | < 0.1 ms | IR-5.6.4 |
| TC-IR-5.6.4.B2 | `FrameCollector` full frame (drain+sort+spikes) | < 0.2 ms | IR-5.6.4 |
| TC-IR-5.6.4.B3 | Profiler total overhead per frame | < 1% of 16.6 ms | IR-5.6.4 |
| TC-IR-5.6.5.B1 | `EcsSystemTracker` per-system cost | < 50 ns | IR-5.6.5 |

1. **TC-IR-5.6.1.B1** -- Measure `CpuScopeGuard::new` + `drop` under Criterion. Target 20 ns
   including two TSC reads and one SPSC push.
2. **TC-IR-5.6.1.B2** -- Measure guard overhead with `FrameCollector::enabled == false`. Target 3 ns
   -- a single predicted branch in the push path.
3. **TC-IR-5.6.4.B1** -- Pre-fill a ring buffer with 1000 `CpuEvent` entries and time
   `FrameCollector::collect_frame`'s drain-and-sort under Criterion. Target 0.1 ms.
4. **TC-IR-5.6.4.B2** -- Measure the full `collect_frame` including spike detection against a
   populated `PhaseBudgetTable`. Target 0.2 ms.
5. **TC-IR-5.6.4.B3** -- Run a representative frame workload with the profiler enabled and measure
   wall-clock delta vs. disabled. Target less than 1% of a 16.6 ms budget (166 us).
6. **TC-IR-5.6.5.B1** -- Measure `EcsSystemTracker` per-system scope cost. Target 50 ns.

All benchmarks are CI-runnable, require no platform-specific hardware beyond a monotonic timer, and
are gated on the repository's standard Criterion benchmark runner.

## Sub-story and Variant Trace

The upstream design lists the following refined sub-stories and letter-variant stories. Each is
covered by the parent-ID TC rows above; a regression in any parent TC constitutes a regression
against the listed sub-story or variant.

- US-15.5.1.1
- US-15.5.6.1
- US-5.6.1.1
- US-5.6.3.1
- US-5.6.6.1
