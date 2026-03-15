# R-14.3 — Threading & Async Requirements

## Thread Pool

### R-14.3.1 Work-Stealing Thread Pool

The engine **SHALL** provide a fixed-size thread pool with per-thread local queues and
work-stealing for load balancing, defaulting the worker count to the number of performance cores
and excluding efficiency cores on hybrid architectures.

- **Derived from:** [F-14.3.1](../../features/platform/threading-async.md)
- **Rationale:** A work-stealing pool ensures even load distribution across cores without centralized
  contention, and excluding efficiency cores prevents latency-sensitive work from running on slow
  cores in hybrid CPU designs.
- **Verification:** Unit test: enqueue 10,000 independent jobs and assert all complete without
  deadlock or data races (run under ThreadSanitizer). Measure throughput and assert work-stealing
  achieves at least 80% CPU utilization across worker threads. On a hybrid-architecture CPU, assert
  the default worker count equals the performance core count, not the total core count.

### R-14.3.2 Thread Affinity and Priority

The engine **SHALL** pin threads to specific cores and set OS-level priority classes, running the
main thread and render submission thread at elevated priority on performance cores and background
threads at low priority on efficiency cores.

- **Derived from:** [F-14.3.2](../../features/platform/threading-async.md)
- **Rationale:** Pinning latency-sensitive threads to performance cores and relegating background
  work to efficiency cores prevents contention during frame-critical work such as physics and
  rendering.
- **Verification:** Integration test: on each platform, start the engine and query the OS for the
  main thread's affinity mask and priority. Assert the main thread is pinned to a performance core
  and runs at elevated priority. Start a background I/O thread and assert it is pinned to an
  efficiency core (on hybrid architectures) with low priority.

## Job System

### R-14.3.3 Task Graph Job System

The engine **SHALL** express parallel work as a directed acyclic graph of jobs with typed data
dependencies, topologically sort the graph, dispatch ready jobs to the thread pool, and support
fan-out, fan-in, continuations, and nested sub-graphs.

- **Derived from:** [F-14.3.3](../../features/platform/threading-async.md)
- **Rationale:** A DAG-based job system makes data dependencies explicit and enables the scheduler
  to maximize parallelism while guaranteeing correct ordering, which is essential for complex frame
  work involving physics, culling, and rendering.
- **Verification:** Unit test: construct a DAG with fan-out (1 job spawning 4), fan-in (4 jobs
  joining into 1), and a continuation chain; assert all jobs execute in topological order and the
  final result is correct. Construct a nested sub-graph and assert inner jobs complete before the
  parent continuation runs. Run under ThreadSanitizer and assert no data races.

### R-14.3.4 Fiber and Stackful Coroutine Support

The engine **SHALL** provide lightweight fibers that can suspend mid-execution and resume on any
worker thread, using platform-specific context-switching primitives, with a default stack size of
64 KiB per fiber and guard pages to detect stack overflow.

- **Derived from:** [F-14.3.4](../../features/platform/threading-async.md)
- **Rationale:** Fibers allow long-running jobs such as pathfinding and procedural generation to
  yield cooperatively without blocking a worker thread, improving thread pool utilization.
- **Verification:** Unit test: create a fiber that increments a counter, suspends, resumes on a
  different worker thread, and increments again; assert the final counter value is correct and the
  resume occurred on a different OS thread. Allocate a fiber with a 64 KiB stack and trigger a
  stack overflow; assert the guard page causes a controlled fault rather than silent corruption.

## Async Runtime

### R-14.3.5 Platform Async I/O Integration

The engine **SHALL** bridge the task graph job system with platform-native async I/O (IOCP on
Windows, GCD on macOS, io_uring on Linux) so that file reads, network operations, and GPU
readbacks complete without blocking worker threads, with completions dispatched as job-system
continuations.

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** Blocking a worker thread on I/O wastes a core and can stall the frame if the
  thread pool is saturated; integrating async I/O completions as jobs maintains a unified scheduling
  model with no idle threads.
- **Verification:** Integration test: on each platform, submit an async file read for a 10 MB file
  and assert the completion fires as a job-system continuation without any worker thread blocking on
  the read. Measure thread pool utilization during the I/O operation and assert no worker thread is
  parked waiting for I/O. Verify the read data matches the file contents.
