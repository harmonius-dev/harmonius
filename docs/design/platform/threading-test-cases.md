# Platform Threading Test Cases

Companion test cases for [threading.md](threading.md).

## Unit Tests

### TC-14.3.1.1 Work Stealing 10K Jobs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue 10,000 jobs via `ThreadPool::spawn` | All 10,000 complete with correct results | R-14.3.1 |
| 2 | Run under ThreadSanitizer | Zero data races detected | R-14.3.1 |

### TC-14.3.1.2 Worker Count Matches Perf Cores

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `ThreadPool::new(ThreadPoolConfig { worker_count: None })` on hybrid CPU | `worker_count()` == `CoreTopology::detect().performance_core_count()` | R-14.3.1 |

### TC-14.3.3.1 Graph Fan Out Fan In

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph: 1 root -> 4 parallel tasks -> 1 join, each increments shared atomic | Join task sees atomic value = 4 | R-14.3.3 |

### TC-14.3.3.2 Graph Nested Subgraph

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent graph with subgraph node, continuation after subgraph | Continuation runs only after subgraph completes | R-14.3.3 |

### TC-14.3.3.3 Graph Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with A -> B -> C -> A cycle | `build()` returns `Err(TaskGraphError::CycleDetected)` | R-14.3.3 |

### TC-14.3.3.4 Graph Empty

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `TaskGraphBuilder::new().build()` with no nodes | Returns `Err(TaskGraphError::EmptyGraph)` | R-14.3.3 |

### TC-14.3.4.1 Fiber Suspend Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fiber calls `yielder.yield_now()` then completes | Fiber resumes on (potentially different) worker, produces correct result | R-14.3.4 |

### TC-14.3.4.2 Fiber Guard Page

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fiber with 64 KiB stack recurses beyond limit | Guard page fault detected (no silent corruption) | R-14.3.4 |

### TC-14.3.5.1 Async Task IO

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Async task `.await`s `IoReactor::read()` on test file | Correct data returned, no worker thread blocks | R-14.3.5 |

### TC-14.3.5.2 Reactor Poll Drains

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit async I/O, check before `poll()` | Future not woken before `poll()` is called | R-14.3.5 |
| 2 | Call `poll()` | Future wakes and returns correct data | R-14.3.5 |

### TC-14.3.5.3 Async Mutex No Block

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 tasks contend on `AsyncMutex`, each holds 1 ms | No worker thread blocks; all tasks complete | R-14.3.5 |

## Integration Tests

### TC-14.3.2.I1 Affinity Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query main thread affinity | Pinned to performance core | R-14.3.2 |
| 2 | Spawn Low-priority task, query affinity | Pinned to efficiency core (if available) | R-14.3.2 |

### TC-14.3.5.I1 Async Read 10 MB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Async read 10 MB file via IoReactor | No worker blocks, data integrity verified | R-14.3.5 |

### TC-14.3.1.I1 Utilization Imbalance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Imbalanced graph (1 heavy + 7 light tasks) | Worker utilization >= 80% | R-14.3.1 |

### TC-14.3.4.I1 Fiber Cross Thread

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fiber suspended on worker N | Resumes on worker M (different worker) | R-14.3.4 |

### TC-14.3.5.I2 GCD Controlled Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | macOS: submit GCD I/O, verify no callback before `poll()` | Callbacks fire only during `poll()` | R-14.3.5 |

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
