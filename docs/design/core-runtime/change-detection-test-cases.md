# Change Detection Test Cases

Companion test cases for [change-detection.md](change-detection.md).

## Unit Tests

### TC-1.13.1.1 Tick Next Increment

| # | Requirement |
|---|-------------|
| 1 | F-1.13.1    |

1. **#1** — `Tick(5).next()`
   - **Expected:** `Tick(6)`

### TC-1.13.1.2 Tick Next Wraparound

| # | Requirement |
|---|-------------|
| 1 | F-1.13.1    |
| 2 | F-1.13.7    |

1. **#1** — `Tick(u32::MAX).next()`
   - **Expected:** `Tick(0)` (wrapping)
2. **#2** — Arithmetic unchanged from wraparound
   - **Expected:** `next()` never panics

### TC-1.13.1.3 Tick Is Older Than Modular

| # | Requirement |
|---|-------------|
| 1 | F-1.13.1    |

1. **#1** — `Tick(3).is_older_than(Tick(5))`
   - **Expected:** `true`
2. **#2** — `Tick(5).is_older_than(Tick(3))`
   - **Expected:** `false`
3. **#3** — `Tick(0).is_older_than(Tick(u32::MAX / 2 - 1))`
   - **Expected:** `true` (small forward distance)

### TC-1.13.2.1 ChangeTick Is Changed Within Window

| # | Requirement |
|---|-------------|
| 1 | F-1.13.2    |
| 2 | F-1.13.3    |

1. **#1** — `ChangeTick { last_changed: Tick(5), last_added: Tick(1) }`, `last_run=Tick(4)`,
   `current=Tick(5)`
   - **Expected:** `is_changed() == true`
2. **#2** — Same ChangeTick, `last_run=Tick(5)`, `current=Tick(5)`
   - **Expected:** `is_changed() == false` (strictly greater than last_run)

### TC-1.13.2.2 ChangeTick Is Changed Outside Window

| # | Requirement |
|---|-------------|
| 1 | F-1.13.3    |

1. **#1** — `last_changed=Tick(2)`, `last_run=Tick(4)`, `current=Tick(5)`
   - **Expected:** `is_changed() == false` (older than last_run)

### TC-1.13.4.1 ChangeTick Is Added Only On Add

| # | Requirement |
|---|-------------|
| 1 | F-1.13.4    |

1. **#1** — Add component at Tick(5), mutate at Tick(6); check `is_added` with
   `last_run=Tick(4), current=Tick(6)`
   - **Expected:** `is_added() == true` (last_added Tick(5) is in window)
2. **#2** — Same component, `last_run=Tick(5), current=Tick(6)`
   - **Expected:** `is_added() == false` (add already observed)

### TC-1.13.5.1 Advance Frame Increments Once

| # | Requirement |
|---|-------------|
| 1 | F-1.13.5    |

1. **#1** — `WorldTickCounter::new()`; `counter.current() == Tick(0)`
   - **Expected:** `Tick(0)`
2. **#2** — Call `advance_frame()` once
   - **Expected:** `counter.current() == Tick(1)`

### TC-1.13.5.2 Advance Frame Exactly Once Per Frame

| # | Requirement |
|---|-------------|
| 1 | F-1.13.5    |

1. **#1** — Run 10 frames of the game loop with several systems writing components; call
   `advance_frame` at Phase 8 on each frame
   - **Expected:** `counter.current() == Tick(10)`
2. **#2** — Within one frame, read `current()` from multiple systems
   - **Expected:** All reads return the same pre-increment tick

### TC-1.13.6.1 System Dispatch Captures Last Run

| # | Requirement |
|---|-------------|
| 1 | F-1.13.6    |

1. **#1** — `SystemMeta::dispatch(Tick(7))`; later read `system.last_run`
   - **Expected:** `Tick(7)`
2. **#2** — After the system body completes, the scheduler must not overwrite `last_run` until the
   next dispatch of that system
   - **Expected:** `last_run` remains stable for the rest of the frame

### TC-1.13.7.1 Column Compaction Clamps Stale Tick

| # | Requirement |
|---|-------------|
| 1 | F-1.13.7    |

1. **#1** — Column with chunks at `last_changed=Tick(10)`, `Tick(100)`, `Tick(1 << 25)`; compact
   with `max_tick=Tick(1 << 25)`, `window=1 << 24`
   - **Expected:** First two ticks clamped to `Tick((1 << 25) - (1 << 24))`; third unchanged

### TC-1.13.7.2 Maybe Compact Skip Below Threshold

| # | Requirement |
|---|-------------|
| 1 | F-1.13.7    |

1. **#1** — `WorldTickCounter::new()`; 100 calls to `advance_frame`; then `maybe_compact`
   - **Expected:** `last_compacted == Tick(0)` (threshold not reached)

### TC-1.13.7.3 Maybe Compact Runs At Threshold

| # | Requirement |
|---|-------------|
| 1 | F-1.13.7    |

1. **#1** — Advance `current` to `Tick(1 << 30)`, call `maybe_compact`
   - **Expected:** `last_compacted` updated to the new tick; all stored ticks clamped

## Integration Tests

### TC-1.13.5.3 Two Systems Same Frame See Consistent Tick

| # | Requirement |
|---|-------------|
| 1 | F-1.13.5    |
| 2 | F-1.13.6    |

1. **#1** — System A and System B both run in Phase 3 of the same frame; both read
   `world.tick_counter().current()`
   - **Expected:** Both reads return the same `Tick` value

### TC-1.13.3.1 Changed Filter End-To-End

| # | Requirement |
|---|-------------|
| 1 | F-1.13.3    |
| 2 | F-1.13.5    |

1. **#1** — System A writes to `Position` on 100 entities; System B runs next with
   `Changed<Position>` filter
   - **Expected:** Query yields exactly 100 entities
2. **#2** — Next frame: no writes; System B runs again
   - **Expected:** Query yields 0 entities

### TC-1.13.4.2 Added Filter Fires Only On Spawn

| # | Requirement |
|---|-------------|
| 1 | F-1.13.4    |

1. **#1** — Spawn 50 entities with `Health`; system with `Added<Health>` runs next frame
   - **Expected:** Yields 50 entities
2. **#2** — Next frame, mutate `Health` on the same entities but add none
   - **Expected:** `Added<Health>` yields 0 entities

### TC-1.13.8.1 Observer Dispatch Uses Same Tick Stream

| # | Requirement |
|---|-------------|
| 1 | F-1.13.8    |
| 2 | F-1.13.3    |

1. **#1** — Observer watching `Position` fires on mutation; during the same frame a
   `Changed<Position>` query runs
   - **Expected:** Both observer and query agree on the set of mutated entities

### TC-1.13.7.4 Wraparound Rescue Preserves Correctness

| # | Requirement |
|---|-------------|
| 1 | F-1.13.7    |
| 2 | F-1.13.3    |

1. **#1** — Seed world at `current=Tick(1 << 30 - 5)`; run 20 frames; verify compaction runs and
   `Changed<T>` queries still return correct results after compaction
   - **Expected:** Queries match a reference tick-stable baseline
2. **#2** — After compaction, no stored tick has `wraparound_distance > COMPACTION_WINDOW`
   - **Expected:** All stored ticks within the window

## Benchmarks

### TC-1.13.3.2 Changed Filter 1M Entities

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M entities, 1 mutated, `Changed<Position>` | Time | < 1 ms | R-1.13.3a |
| 2 | 1M entities, 50% mutated, `Changed<Position>` | Time | < 3 ms | R-1.13.3a |

### TC-1.13.5.4 Advance Frame Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `advance_frame` (no compaction) | Latency | < 50 ns | R-1.13.5a |

### TC-1.13.7.5 Compaction Pass Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 columns, 100 chunks each, full compaction | Time | < 10 ms | R-1.13.7a |

### TC-1.13.2.3 ChangeTick Compare Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `is_changed` invoked 10M times, scalar path | Time | < 50 ms | R-1.13.2a |
| 2 | `is_changed` on 1,024 contiguous chunks via SIMD | Time | < 2 us | R-1.13.2a |
