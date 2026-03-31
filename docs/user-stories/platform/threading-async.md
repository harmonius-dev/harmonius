# User Stories -- 14.3 Threading & Async

| ID         | Persona                 |
|------------|-------------------------|
| US-14.3.1  | player (P-23)           |
| US-14.3.2  | game developer (P-15)   |
| US-14.3.3  | game developer (P-15)   |
| US-14.3.4  | game developer (P-15)   |
| US-14.3.5  | game developer (P-15)   |
| US-14.3.6  | game developer (P-15)   |
| US-14.3.7  | game developer (P-15)   |
| US-14.3.8  | game developer (P-15)   |
| US-14.3.9  | game developer (P-15)   |
| US-14.3.10 | game developer (P-15)   |
| US-14.3.11 | game developer (P-15)   |
| US-14.3.12 | engine developer (P-26) |
| US-14.3.13 | engine developer (P-26) |
| US-14.3.14 | engine developer (P-26) |
| US-14.3.15 | engine developer (P-26) |
| US-14.3.16 | engine developer (P-26) |
| US-14.3.17 | engine developer (P-26) |
| US-14.3.18 | engine developer (P-26) |
| US-14.3.19 | engine developer (P-26) |
| US-14.3.20 | engine tester (P-27)    |
| US-14.3.21 | engine tester (P-27)    |
| US-14.3.22 | engine tester (P-27)    |
| US-14.3.23 | engine tester (P-27)    |
| US-14.3.24 | engine tester (P-27)    |
| US-14.3.25 | engine tester (P-27)    |
| US-14.3.26 | QA tester (P-19)        |

## Player Experience

1. **US-14.3.1** — **As a** player (P-23), **I want** the engine to fully utilize my CPU during
   large-scale content without frame time spikes, **so that** gameplay stays smooth with hundreds of
   entities updating simultaneously.

## Thread Pool and Jobs

2. **US-14.3.2** — **As a** game developer (P-15), **I want** a work-stealing thread pool sized to
   performance cores, **so that** CPU-parallel work is distributed evenly across all available
   cores.
3. **US-14.3.3** — **As a** game developer (P-15), **I want** a DAG-based job system with typed data
   dependencies, fan-out, fan-in, and nested sub-graphs, **so that** frame work is parallelized
   automatically based on declared dependencies.
4. **US-14.3.4** — **As a** game developer (P-15), **I want** configurable fiber stack sizes per
   workload category (default 64 KiB with guard pages), **so that** long-running jobs like
   pathfinding can yield without wasting a thread.

## Async Runtime

5. **US-14.3.5** — **As a** game developer (P-15), **I want** all asynchronous operations (file I/O,
   GPU sync, network) to use `async`/`await`, **so that** I can write linear async code without
   callback pyramids.
6. **US-14.3.6** — **As a** game developer (P-15), **I want** to spawn parallel tasks that borrow
   from the current stack frame without `Arc` or `'static`, **so that** per-frame parallel work is
   ergonomic and zero-overhead.
7. **US-14.3.7** — **As a** game developer (P-15), **I want** async mutex and rwlock primitives that
   yield via `.await` when contended, **so that** contended locks never block a worker thread.
8. **US-14.3.8** — **As a** game developer (P-15), **I want** to register both sync and async event
   handlers, **so that** event responses requiring I/O do not block the dispatch thread.
9. **US-14.3.9** — **As a** game developer (P-15), **I want** to `await` a `next_frame` operation
   from within an async task, **so that** long procedural generation or loading work can yield at
   frame boundaries.

## Game Loop Graph

10. **US-14.3.10** — **As a** game developer (P-15), **I want** to define a custom game loop graph
    with game-specific phases that integrate with the engine's built-in phases, **so that** I can
    customize frame structure without forking the engine.
11. **US-14.3.11** — **As a** game developer (P-15), **I want** to bind new world state, input
    events, and delta time to the compiled frame each frame without triggering graph recompilation,
    **so that** frame setup is fast and predictable.

## Engine Developer -- Platform and Internals

12. **US-14.3.12** — **As an** engine developer (P-26), **I want** thread affinity and OS-level
    priority control to pin main and render threads to performance cores and background I/O to
    efficiency cores, **so that** game-critical work is never preempted by background tasks.
13. **US-14.3.13** — **As an** engine developer (P-26), **I want** platform async I/O completions
    (IOCP, GCD, io_uring) dispatched as job-system continuations, **so that** I/O integrates with
    the unified task scheduler without blocking workers.
14. **US-14.3.14** — **As an** engine developer (P-26), **I want** automatic detection of
    performance and efficiency cores at startup, **so that** thread pool sizing and affinity are
    configured correctly on hybrid architectures.
15. **US-14.3.15** — **As an** engine developer (P-26), **I want** I/O completions processed only at
    a defined reactor poll point in the game loop, **so that** I/O events never cause mid-frame
    latency spikes.
16. **US-14.3.16** — **As an** engine developer (P-26), **I want** GCD `dispatch_io` callbacks
    drained at a controlled point in the frame loop on macOS, **so that** macOS I/O completion
    timing is deterministic and does not interrupt frame work.
17. **US-14.3.17** — **As an** engine developer (P-26), **I want** a safe game loop graph that
    validates access patterns and dependencies at compile time, **so that** scheduling bugs (data
    races, deadlocks) are caught before the game runs.
18. **US-14.3.18** — **As an** engine developer (P-26), **I want** the compiled frame to be reusable
    across frames without recompilation when only per-frame data changes, **so that** compilation
    cost is amortized across thousands of frames.
19. **US-14.3.19** — **As an** engine developer (P-26), **I want** all public game loop graph and
    task graph APIs to be safe Rust with no `unsafe` in user-facing types, **so that** memory bugs
    are impossible when composing frame schedules.

## Engine Tester -- Validation

20. **US-14.3.20** — **As an** engine tester (P-27), **I want** benchmarks that submit imbalanced
    job graphs and measure per-thread utilization, **so that** work-stealing achieves at least 80 %
    CPU utilization.
21. **US-14.3.21** — **As an** engine tester (P-27), **I want** tests that create, suspend, and
    resume fibers on all platforms and verify stack integrity, **so that** fiber regressions are
    caught in CI.
22. **US-14.3.22** — **As an** engine tester (P-27), **I want** stress tests that issue concurrent
    file reads and network operations under full CPU load and verify no worker thread blocks,
    **so that** async I/O never causes thread starvation.
23. **US-14.3.23** — **As an** engine tester (P-27), **I want** tests that verify no I/O completion
    wakes a future between `poll()` calls, **so that** the controlled poll model is validated in CI.
24. **US-14.3.24** — **As an** engine tester (P-27), **I want** benchmarks that measure async mutex
    contention overhead, **so that** async locks are validated as non-blocking.
25. **US-14.3.25** — **As an** engine tester (P-27), **I want** test cases for every `GraphError`
    variant (cycle, access conflict, missing dependency), **so that** the graph compiler has full
    validation coverage.
26. **US-14.3.26** — **As a** QA tester (P-19), **I want** compile-time graph validation to produce
    structured error messages identifying the exact problem, **so that** invalid configurations fail
    with actionable diagnostics.
