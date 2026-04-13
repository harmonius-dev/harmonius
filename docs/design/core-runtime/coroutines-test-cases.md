# Coroutines Test Cases

Companion test cases for [coroutines.md](coroutines.md).

## Unit Tests

### TC-1.14.2.1 Resume Yields Pending

| # | Requirement |
|---|-------------|
| 1 | F-1.14.2    |

1. **#1** — Coroutine whose first `resume` returns `CoroutineState::Pending`, call once
   - **Expected:** Scheduler keeps the entry in `WaitState::Ready`; `resume` not called twice this
     frame

### TC-1.14.2.2 Resume Yields This Frame

| # | Requirement |
|---|-------------|
| 1 | F-1.14.2    |
| 2 | F-1.14.3    |

1. **#1** — Coroutine returns `CoroutineState::YieldedThisFrame`
   - **Expected:** `WaitState::PendingThisFrame` recorded
2. **#2** — Tick again in the same frame
   - **Expected:** `resume` not invoked until the next tick

### TC-1.14.3.1 Yielded Next Frame Ticks

| # | Requirement |
|---|-------------|
| 1 | F-1.14.3    |
| 2 | F-1.14.4    |

1. **#1** — Coroutine returns `CoroutineState::YieldedNextFrame(DelaySpec::Ticks(3))`
   - **Expected:** `WaitState::PendingTicks { remaining: 3 }` stored in `waiting` map
2. **#2** — Tick scheduler 3 more times
   - **Expected:** `remaining` decrements to 0, then state -> `Ready`

### TC-1.14.3.2 Yielded Next Frame Seconds

| # | Requirement |
|---|-------------|
| 1 | F-1.14.3    |

1. **#1** — Coroutine returns `YieldedNextFrame(DelaySpec::Seconds(1.0))`
   - **Expected:** `WaitState::PendingSeconds { remaining_secs: 1.0 }` stored
2. **#2** — Tick 4 times with `delta_time = 0.25`
   - **Expected:** `remaining_secs` reaches 0; state -> `Ready` on the 4th tick

### TC-1.14.3.3 Yielded Next Frame Until Event

| # | Requirement |
|---|-------------|
| 1 | F-1.14.3    |
| 2 | F-1.14.6    |

1. **#1** — Coroutine returns `YieldedNextFrame(DelaySpec::UntilEvent(EventId(42)))`
   - **Expected:** `WaitState::PendingEvent { event: EventId(42) }` recorded
2. **#2** — Tick with `EventSnapshot` containing `EventId(42)`
   - **Expected:** State -> `Ready`; `resume` invoked on next tick

### TC-1.14.3.4 Yielded Next Frame Until Event No Match

| # | Requirement |
|---|-------------|
| 1 | F-1.14.6    |

1. **#1** — Coroutine waits on `EventId(42)`, tick with snapshot containing only `EventId(7)`
   - **Expected:** `WaitState` unchanged; `resume` not invoked

### TC-1.14.2.3 Resume Yields Completed

| # | Requirement |
|---|-------------|
| 1 | F-1.14.2    |
| 2 | F-1.14.4    |

1. **#1** — Coroutine returns `CoroutineState::Completed(())`
   - **Expected:** `WaitState::Done` recorded; handle released from `HandleMap`
2. **#2** — Subsequent `poll(handle)`
   - **Expected:** `Some(PollResult::Done(()))`

### TC-1.14.4.1 Spawn Returns Handle

| # | Requirement |
|---|-------------|
| 1 | F-1.14.4    |
| 2 | F-1.14.4    |

1. **#1** — `scheduler.spawn(task)` on an empty scheduler
   - **Expected:** Returns `CoroutineHandle` with `is_null() == false`
2. **#2** — Spawn 100 coroutines
   - **Expected:** 100 distinct handles; `entries.len() == 100`

### TC-1.14.4.2 Cancel Transitions To Cancelled

| # | Requirement |
|---|-------------|
| 1 | F-1.14.4    |

1. **#1** — Spawn coroutine, `scheduler.cancel(handle)`
   - **Expected:** `WaitState::Cancelled` recorded
2. **#2** — Tick the scheduler
   - **Expected:** `resume` never called on the cancelled coroutine

### TC-1.14.4.3 Poll Returns Running

| # | Requirement |
|---|-------------|
| 1 | F-1.14.4    |

1. **#1** — Spawn coroutine that returns `Pending`, call `poll(handle)`
   - **Expected:** `Some(PollResult::Running)`

### TC-1.14.4.4 Poll Stale Handle

| # | Requirement |
|---|-------------|
| 1 | F-1.14.4    |

1. **#1** — Spawn, run to completion, `poll(handle)` long after release
   - **Expected:** `None` — handle generation has advanced

### TC-1.14.1.1 Coroutine Is Not A Future

| # | Requirement |
|---|-------------|
| 1 | F-1.14.1    |

1. **#1** — `trait Coroutine` definition
   - **Expected:** No method returns `impl Future`; no `async fn`; no `.await`; grep for
     `use core::future` returns empty

### TC-1.14.1.2 Boss Phase Example Compiles Without Async

| # | Requirement |
|---|-------------|
| 1 | F-1.14.1    |

1. **#1** — Compile the `BossPhaseCoroutine` example from `coroutines.md`
   - **Expected:** `cargo build` succeeds with `#![forbid(async_fn_in_trait)]` in scope

## Integration Tests

### TC-1.14.5.1 CoroutineSystem Ticks Scheduler Once Per Phase

| # | Requirement |
|---|-------------|
| 1 | F-1.14.5    |

1. **#1** — Register `coroutine_system` in the ECS `PreUpdate` phase, run 10 frames
   - **Expected:** `CoroutineScheduler::tick` invoked exactly 10 times

### TC-1.14.5.2 Boss Phase Multi-Frame Sequence

| # | Requirement |
|---|-------------|
| 1 | F-1.14.2    |
| 2 | F-1.14.3    |
| 3 | F-1.14.5    |

1. **#1** — Spawn `BossPhaseCoroutine`; run the game loop until completion
   - **Expected:** Completes after `Seconds(2.0)` + `Ticks(60)` + 1 this-frame yield elapse
2. **#2** — Handle released after completion
   - **Expected:** `poll(handle)` returns `None` after the release frame
3. **#3** — `EventSnapshot` delivered to coroutine on tick
   - **Expected:** `ctx.events` reflects the current frame's buffered events

### TC-1.14.5.3 Many Coroutines Run To Completion

| # | Requirement |
|---|-------------|
| 1 | F-1.14.4    |
| 2 | F-1.14.5    |

1. **#1** — Spawn 1,000 coroutines that each yield once then complete
   - **Expected:** All 1,000 handles released after 2 scheduler ticks

### TC-1.14.6.1 Event Wakeup Integration

| # | Requirement |
|---|-------------|
| 1 | F-1.14.6    |
| 2 | F-1.14.5    |

1. **#1** — Coroutine yields on `DelaySpec::UntilEvent(GameOver)`; another system emits `GameOver`
   - **Expected:** Coroutine resumes in the same frame the event fires
2. **#2** — Event never fires
   - **Expected:** Coroutine remains in `PendingEvent` indefinitely

### TC-1.14.5.4 Scripting Client Round Trip

| # | Requirement |
|---|-------------|
| 1 | F-1.14.1    |
| 2 | F-15.8.4    |

1. **#1** — Scripting subsystem wraps a logic graph coroutine and spawns it via the
   `CoroutineScheduler` resource
   - **Expected:** Graph state machine advances frame-by-frame without `.await`

## Benchmarks

### TC-1.14.5.5 Tick 10K Ready Coroutines Under 2 ms

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 coroutines all `Ready`, single tick | Time | < 2 ms | R-1.14.5a |

### TC-1.14.4.5 Spawn Cancel Round Trip

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `spawn` then `cancel` | Latency | < 500 ns | R-1.14.4a |

### TC-1.14.3.5 Timer Advancement Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 `PendingTicks` entries, advance 1 tick | Time | < 500 us | R-1.14.3a |
| 2 | 10,000 `PendingSeconds` entries, advance `delta_time` | Time | < 500 us | R-1.14.3a |

### TC-1.14.6.2 Event Wakeup Scan

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1,000 `PendingEvent` entries, 100 events in snapshot | Time | < 1 ms | R-1.14.6a |
