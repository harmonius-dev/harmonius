# Platform Threading Test Cases

Companion test cases for [threading.md](threading.md).

## Unit Tests

### TC-14.3.1.1 Job System Par Iter 10K Jobs

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |
| 2 | R-14.3.1    |

1. **#1** — Execute 10,000 jobs via `job_system::par_iter()`
   - **Expected:** All 10,000 complete with correct results
2. **#2** — Run under ThreadSanitizer
   - **Expected:** Zero data races detected

### TC-14.3.1.2 Worker Count Matches Perf Cores

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** — Configure job system thread pool on hybrid CPU
   - **Expected:** Worker count matches performance core count

### TC-14.3.3.1 Graph Fan Out Fan In

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |

1. **#1** — Graph: 1 root -> 4 parallel tasks -> 1 join, each increments shared atomic
   - **Expected:** Join task sees atomic value = 4

### TC-14.3.3.2 Graph Nested Subgraph

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |

1. **#1** — Parent graph with subgraph node, continuation after subgraph
   - **Expected:** Continuation runs only after subgraph completes

### TC-14.3.3.3 Graph Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |

1. **#1** — Graph with A -> B -> C -> A cycle
   - **Expected:** `build()` returns `Err(TaskGraphError::CycleDetected)`

### TC-14.3.3.4 Graph Empty

| # | Requirement |
|---|-------------|
| 1 | R-14.3.3    |

1. **#1** — `TaskGraphBuilder::new().build()` with no nodes
   - **Expected:** Returns `Err(TaskGraphError::EmptyGraph)`

### TC-14.3.5.1 Compio File Read

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — Read test file via compio on main thread
   - **Expected:** Correct data returned, game loop thread not blocked

### TC-14.3.5.2 IO Completions At Frame Boundary

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |
| 2 | R-14.3.5    |

1. **#1** — Submit I/O via compio, check before frame drain
   - **Expected:** Result not yet available to game loop
2. **#2** — Drain channel at frame boundary
   - **Expected:** Result arrives and contains correct data

### TC-14.3.5.3 Channel Message Delivery

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — Main thread sends 100 messages via channel
   - **Expected:** Game loop thread receives all 100 in order, no blocking

## Integration Tests

### TC-14.3.2.I1 Affinity Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-14.3.2    |
| 2 | R-14.3.2    |

1. **#1** — Query game loop thread affinity
   - **Expected:** Pinned to performance core
2. **#2** — Spawn low-priority job system task, query affinity
   - **Expected:** Pinned to efficiency core (if available)

### TC-14.3.5.I1 Compio Read 10 MB

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — Read 10 MB file via compio on main thread
   - **Expected:** Game loop not blocked, data integrity verified via channel delivery

### TC-14.3.1.I1 Utilization Imbalance

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** — Imbalanced graph (1 heavy + 7 light tasks)
   - **Expected:** Job system worker utilization >= 80%

### TC-14.3.5.I2 Three Thread IO Flow

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — Main thread reads file via compio, sends result to game loop via channel, game loop
   sends render data to render thread
   - **Expected:** Data flows through all 3 threads correctly with no blocking

<!-- THIN: design section lacks detail -->
### TC-14.3.10.I1 Platform IO Poll 100 Completions

| # | Requirement |
|---|-------------|
| 1 | US-14.3.10  |

1. **#1** — Issue 100 read completions via platform I/O backend; poll once
   - **Expected:** All 100 completions delivered in a single poll, total time < 50 us
2. **#2** — Verify per-completion delivery callback executed exactly once
   - **Expected:** Callback counter == 100, no duplicates

<!-- THIN: design section lacks detail -->
### TC-14.3.11.I1 Job Dispatch End To End

| # | Requirement |
|---|-------------|
| 1 | US-14.3.11  |

1. **#1** — From a game system, dispatch a job that writes to a result component
   - **Expected:** Result component populated within next frame, no data races
2. **#2** — Repeat 1000 dispatches, measure mean per-dispatch wall time
   - **Expected:** Mean < 1 us amortized

<!-- THIN: design section lacks detail -->
### TC-14.3.20.I1 Small Stack Tasks For Loading

| # | Requirement |
|---|-------------|
| 1 | US-14.3.20  |

1. **#1** — Spawn 10,000 loading tasks with 64 KiB stacks
   - **Expected:** All tasks complete; total stack memory ≈ 640 MiB, not 80 GiB
2. **#2** — Spawn deep-recursion task with default 8 MiB stack
   - **Expected:** Task uses standard stack and completes without overflow

<!-- THIN: design section lacks detail -->
### TC-14.6.11.I1 IO Throughput Disk Bandwidth

| # | Requirement |
|---|-------------|
| 1 | US-14.6.11  |

1. **#1** — Sequentially read a 1 GiB file via platform I/O on a known SSD
   - **Expected:** Effective throughput >= 80% of measured raw disk bandwidth
2. **#2** — Repeat with 64 concurrent 16 MiB reads
   - **Expected:** Aggregate throughput maintains >= 80% raw

## Benchmarks

### TC-14.3.1.B1 Job System Dispatch Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dispatch 100K no-op jobs via job system | Per-dispatch | < 1 us | R-14.3.1 |

### TC-14.3.1.B2 Job System Work Stealing Utilization

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10K heterogeneous jobs across job system workers | Avg utilization | >= 80% | R-14.3.1 |

### TC-14.3.5.B1 Compio Poll 100 Completions

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Submit 100 reads via compio, drain completions | Poll duration | < 50 us | R-14.3.5 |

### TC-14.3.5.B2 Channel Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 producers sending to game loop channel | Per-message latency | < 1 us | R-14.3.5 |

### TC-14.3.5.B3 Compio IO Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sequential read 1 GB via compio | Throughput | >= 80% raw disk | R-14.3.5 |
