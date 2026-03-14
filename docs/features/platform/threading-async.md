# 14.3 — Threading & Async

## Thread Pool

### F-14.3.1 Work-Stealing Thread Pool

A fixed-size thread pool with per-thread local queues and work-stealing for load balancing.
Worker count defaults to the number of performance cores, excluding efficiency cores on hybrid
architectures. This is the primary execution backend for all CPU-parallel work in the engine,
from physics broadphase to culling to asset decompression.

- **Requirements:** R-14.3.1
- **Dependencies:** None
- **Platform notes:** Windows uses `CreateThread` with `SetThreadAffinityMask` for core
  pinning; macOS uses `pthread_create` with QoS classes (`QOS_CLASS_USER_INTERACTIVE` for
  game threads, `QOS_CLASS_UTILITY` for background work); Linux uses `pthread_create` with
  `pthread_setaffinity_np`. Hybrid core detection uses `cpuid` (x86) or IOKit/sysctl (Apple
  Silicon).

### F-14.3.2 Thread Affinity and Priority

Pin threads to specific cores and set OS-level priority classes. The main thread and render
submission thread run at elevated priority on performance cores. Background I/O and telemetry
threads run at low priority on efficiency cores to avoid contention during MMO raid encounters
with hundreds of entities.

- **Requirements:** R-14.3.2
- **Dependencies:** F-14.3.1
- **Platform notes:** Windows uses `SetThreadAffinityMask` and `SetThreadPriority`; macOS
  uses `pthread_set_qos_class_self_np` (preferred over raw priority on Apple platforms);
  Linux uses `pthread_setaffinity_np` and `sched_setscheduler`. Apple Silicon distinguishes
  P-cores and E-cores via `hw.nperflevels` sysctl.

## Job System

### F-14.3.3 Task Graph Job System

Express parallel work as a directed acyclic graph of jobs with typed data dependencies. The
scheduler topologically sorts the graph, dispatches ready jobs to the thread pool, and
propagates completions. Supports fan-out, fan-in, continuations, and nested sub-graphs for
hierarchical decomposition of frame work (e.g., physics substeps within the main frame graph).

- **Requirements:** R-14.3.3
- **Dependencies:** F-14.3.1
- **Platform notes:** None

### F-14.3.4 Fiber and Stackful Coroutine Support

Lightweight fibers that can suspend mid-execution and resume on any worker thread, enabling
long-running jobs (pathfinding, procedural generation) to yield without blocking a thread.
Fibers use platform-specific context-switching primitives for minimal overhead.

- **Requirements:** R-14.3.4
- **Dependencies:** F-14.3.1
- **Platform notes:** Windows uses `CreateFiber`/`SwitchToFiber` via COM wrappers; macOS
  and Linux use `ucontext` (deprecated on macOS but functional) or inline assembly for
  `setjmp`/`longjmp`-style context switches with explicit stack allocation. Stack sizes
  default to 64 KiB per fiber with guard pages.

## Async Runtime

### F-14.3.5 Platform Async I/O Integration

Bridge the engine's async task system with platform-native async I/O primitives so that file
reads, network operations, and GPU readbacks complete without blocking worker threads. Completion
events are dispatched as jobs in the task graph, maintaining a unified scheduling model.

- **Requirements:** R-14.3.5
- **Dependencies:** F-14.3.3
- **Platform notes:** Windows uses I/O completion ports (`CreateIoCompletionPort`,
  `GetQueuedCompletionStatusEx`); macOS uses Grand Central Dispatch
  (`dispatch_io_create`, `dispatch_io_read`); Linux uses `io_uring`
  (`io_uring_submit`, `io_uring_wait_cqe_nr`). Each backend wraps completions into
  job-system continuations via a thin platform adapter layer exposed through C FFI.
