# AI Behavior ↔ Animation Integration Test Cases

All tests are CI-runnable (no GPU, no interactive input). Negative tests verify documented fallback
behavior from the design's Failure Modes table. **Dimensionality:** 3D only; 2D/2.5D out of scope.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.1.1.1 | BT leaf writes speed | BT MoveTo at 3.0 | params.speed == 3.0 | IR-1.1.1 |
| TC-IR-1.1.1.2 | GOAP writes trigger | Attack action | "attack" in triggers | IR-1.1.1 |
| TC-IR-1.1.1.3 | AI writes posture bools | NPC crouches | is_crouching == true | IR-1.1.1 |
| TC-IR-1.1.1.4 | AI writes aim offset | NPC aims up 30 | aim_pitch ~= 30.0 | IR-1.1.1 |
| TC-IR-1.1.2.1 | AI reads remaining | Anim 80% done | remaining_time < 0.2 | IR-1.1.2 |
| TC-IR-1.1.2.2 | AI gates transition | SM transitioning | is_transitioning == true | IR-1.1.2 |
| TC-IR-1.1.2.3 | Same-frame state read | Phase 4 after Phase 6 | current_state matches | IR-1.1.2 |
| TC-IR-1.1.3.1 | Combat plays montage | AI -> Combat | ActiveMontage inserted | IR-1.1.3 |
| TC-IR-1.1.3.2 | Montage removed end | Montage ends | ActiveMontage removed | IR-1.1.3 |
| TC-IR-1.1.4.1 | Nav speed blends | speed 2.5, dir 45 | Blend walk/run | IR-1.1.4 |
| TC-IR-1.1.4.2 | Zero speed idles | speed 0.0 | Idle state selected | IR-1.1.4 |
| TC-IR-1.1.5.1 | 500 agents budget | 500 AI+anim | Total < 2 ms | IR-1.1.5 |

## Negative / Failure Mode Tests

| ID | Test | Input | Expected | FM |
|----|------|-------|----------|----|
| TC-IR-1.1.E1 | Missing AnimationParams | SM, no params comp | Idle, warn logged once | FM-1 |
| TC-IR-1.1.E2 | Invalid trigger ID | Unknown StringId | Consumed, warn logged | FM-2 |
| TC-IR-1.1.E3 | Missing montage asset | Dangling handle | Component removed, error | FM-3 |
| TC-IR-1.1.E4 | Budget exceeded | 1000 agents, 1 ms cap | Deferred, stale params ok | FM-4 |

1. **TC-IR-1.1.E1** -- Spawn an entity with `AnimationStateMachine` but no `AnimationParams`
   component. Run one frame. Assert the entity remains in the idle state, a single `warn` log is
   emitted for the entity, and subsequent frames do not re-emit the warning.
2. **TC-IR-1.1.E2** -- Spawn an entity with a trigger `StringId` that matches no transition
   condition. Run one frame. Assert the trigger is removed from the triggers list, a `warn` log is
   emitted naming the unrecognized trigger, and the state machine continues evaluating remaining
   triggers and parameter-based conditions.
3. **TC-IR-1.1.E3** -- Insert an `ActiveMontage` referencing an unloaded `AssetHandle<MontageDef>`.
   Run one frame. Assert the `ActiveMontage` component is removed, an `error` log is emitted naming
   the asset ID, and the state machine remains in its previous state.
4. **TC-IR-1.1.E4** -- Spawn 1000 AI agents with a 1 ms `FrameBudget`. Run one frame. Assert that
   the number of evaluated agents is less than 1000, remaining agents are deferred to the next
   frame, deferred agents retain their previous `AnimationParams` values, and a `debug` log reports
   the deferred count.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.1.5.B1 | 500 AI+anim eval | < 2 ms | IR-1.1.5 |
| TC-IR-1.1.1.B1 | 1000 param writes | < 0.1 ms | IR-1.1.1 |
| TC-IR-1.1.2.B1 | 1000 anim queries | < 0.05 ms | IR-1.1.2 |
| TC-IR-1.1.5.B2 | Budget time-slicing | < 0.2 ms overhead | IR-1.1.5 |

## Sub-story and Variant Trace

The upstream design lists the following refined sub-stories and letter-variant stories. Each is
covered by the parent-ID TC rows above; a regression in any parent TC constitutes a regression
against the listed sub-story or variant.

- US-9.4.10.3
