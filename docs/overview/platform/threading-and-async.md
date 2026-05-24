# Threading and Async

Parallelism: worker threads, task scheduling, and async/await.

## What it covers

- Work-stealing thread pool sized to performance cores with thread affinity and OS priorities.
- DAG-based job system: typed dependencies, fan-out/fan-in, continuations, nested graphs.
- Lightweight fibers (64 KiB stacks) that suspend and resume on any worker.
- Async/await everywhere: `current_thread` runtime polled once per frame, not callback-based.
- Scoped task execution borrowing non-`'static` data with compile-time leak prevention.
- Async synchronization primitives: `AsyncMutex`, `AsyncRwLock`, `AsyncBarrier`.
- Data-parallel operations: `parallel_for`, `parallel_map`, `parallel_reduce`.
- Compiled frame schedule reused across frames after validation.

## Concepts

### Thread Pool and Pinning

A fixed-size thread pool matches the number of performance cores (excluding efficiency cores).
Threads are pinned to specific cores and assigned OS priorities: game-loop threads run at high
priority, render threads at high priority, background workers at normal. No dynamic thread spawning;
all work is scheduled through the task graph.

### Task Graph and Scheduling

A task graph declares a DAG of jobs with typed read/write dependencies on resources (entities,
components, assets). The engine compiles this into a schedule, detecting cycles and access conflicts
at compile time. At runtime, the scheduler respects dependencies and parallelizes independent jobs.

### Async Integration

The engine integrates Tokio's `current_thread` runtime, which runs inside the task graph. Async
operations never block the main thread or render thread; completions arrive as job continuations at
the next frame's poll point. Scoped tasks allow async code to borrow non-`'static` data (stack
variables) safely.

## How it fits

- See [simulation-loop](../core-runtime/simulation-loop.md) for when tasks are scheduled during the
  frame.
- See [os-services](./os-services.md) for platform-async I/O integration.
- See [data-and-io](../core-runtime/data-and-io.md) for async file operations.
