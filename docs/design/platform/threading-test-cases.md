# Platform Threading Test Cases

Companion test cases for [threading.md](threading.md).

## Unit Tests

### TC-14.3.1.1 Work Stealing 10K Jobs

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |
| 2 | R-14.3.1    |

1. **#1** — Enqueue 10,000 jobs via `ThreadPool::spawn`
   - **Expected:** All 10,000 complete with correct results
2. **#2** — Run under ThreadSanitizer
   - **Expected:** Zero data races detected

### TC-14.3.1.2 Worker Count Matches Perf Cores

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** — `ThreadPool::new(ThreadPoolConfig { worker_count: None })` on hybrid CPU
   - **Expected:** `worker_count()` == `CoreTopology::detect().performance_core_count()`

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

### TC-14.3.4.1 Fiber Suspend Resume

| # | Requirement |
|---|-------------|
| 1 | R-14.3.4    |

1. **#1** — Fiber calls `yielder.yield_now()` then completes
   - **Expected:** Fiber resumes on (potentially different) worker, produces correct result

### TC-14.3.4.2 Fiber Guard Page

| # | Requirement |
|---|-------------|
| 1 | R-14.3.4    |

1. **#1** — Fiber with 64 KiB stack recurses beyond limit
   - **Expected:** Guard page fault detected (no silent corruption)

### TC-14.3.5.1 Async Task IO

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — Async task `.await`s `IoReactor::read()` on test file
   - **Expected:** Correct data returned, no worker thread blocks

### TC-14.3.5.2 Reactor Poll Drains

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |
| 2 | R-14.3.5    |

1. **#1** — Submit async I/O, check before `poll()`
   - **Expected:** Future not woken before `poll()` is called
2. **#2** — Call `poll()`
   - **Expected:** Future wakes and returns correct data

### TC-14.3.5.3 Async Mutex No Block

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — 4 tasks contend on `AsyncMutex`, each holds 1 ms
   - **Expected:** No worker thread blocks; all tasks complete

## Integration Tests

### TC-14.3.2.I1 Affinity Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-14.3.2    |
| 2 | R-14.3.2    |

1. **#1** — Query main thread affinity
   - **Expected:** Pinned to performance core
2. **#2** — Spawn Low-priority task, query affinity
   - **Expected:** Pinned to efficiency core (if available)

### TC-14.3.5.I1 Async Read 10 MB

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — Async read 10 MB file via IoReactor
   - **Expected:** No worker blocks, data integrity verified

### TC-14.3.1.I1 Utilization Imbalance

| # | Requirement |
|---|-------------|
| 1 | R-14.3.1    |

1. **#1** — Imbalanced graph (1 heavy + 7 light tasks)
   - **Expected:** Worker utilization >= 80%

### TC-14.3.4.I1 Fiber Cross Thread

| # | Requirement |
|---|-------------|
| 1 | R-14.3.4    |

1. **#1** — Fiber suspended on worker N
   - **Expected:** Resumes on worker M (different worker)

### TC-14.3.5.I2 GCD Controlled Drain

| # | Requirement |
|---|-------------|
| 1 | R-14.3.5    |

1. **#1** — macOS: submit GCD I/O, verify no callback before `poll()`
   - **Expected:** Callbacks fire only during `poll()`

## Benchmarks

### TC-14.3.1.B1 Job Dispatch Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dispatch 100K no-op jobs | Per-dispatch overhead | < 1 us | R-14.3.1 |

### TC-14.3.1.B2 Work Stealing Utilization

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10K heterogeneous jobs across all workers | Average worker utilization | >= 80% | R-14.3.1 |

### TC-14.3.4.B1 Fiber Context Switch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Yield and resume 100K times | Per-switch latency | < 500 ns | R-14.3.4 |

### TC-14.3.5.B1 Reactor Poll 100 Completions

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Submit 100 async reads, drain in single `poll()` | Poll duration | < 50 us | R-14.3.5 |

### TC-14.3.5.B2 Async Mutex Contended Lock

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 tasks contending on AsyncMutex | Per-lock latency (yield, not spin) | < 1 us | R-14.3.5 |

### TC-14.3.5.B3 IO Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Async sequential read 1 GB via IoReactor | Throughput | >= 80% raw disk bandwidth | R-14.3.5 |
