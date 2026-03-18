# User Stories -- 14.3 Threading & Async

| ID         | Persona                 | Features             | Requirements         |
|------------|-------------------------|----------------------|----------------------|
| US-14.3.1  | game developer (P-15)   |                      |                      |
| US-14.3.2  | engine developer (P-26) |                      |                      |
| US-14.3.3  | game developer (P-15)   |                      |                      |
| US-14.3.4  | engine developer (P-26) |                      |                      |
| US-14.3.5  | engine developer (P-26) |                      |                      |
| US-14.3.6  | player (P-23)           |                      |                      |
| US-14.3.7  | engine developer (P-26) |                      |                      |
| US-14.3.8  | engine tester (P-27)    |                      |                      |
| US-14.3.9  | engine tester (P-27)    |                      |                      |
| US-14.3.10 | engine tester (P-27)    |                      |                      |
| US-14.3.11 | engine developer (P-26) |                      |                      |
| US-14.3.12 | game developer (P-15)   |                      |                      |
| US-14.3.13 | engine developer (P-26) |                      |                      |
| US-14.3.14 | game developer (P-15)   |                      |                      |
| US-14.3.15 | game developer (P-15)   |                      |                      |
| US-14.3.16 | engine developer (P-26) |                      |                      |
| US-14.3.17 | game developer (P-15)   |                      |                      |
| US-14.3.19 | engine developer (P-26) |                      |                      |
| US-14.3.20 | game developer (P-15)   |                      |                      |
| US-14.3.21 | engine tester (P-27)    |                      |                      |
| US-14.3.22 | engine tester (P-27)    |                      |                      |
| US-14.3.23 | engine developer (P-26) | F-14.3.14, F-14.3.15 | R-14.3.14, R-14.3.15 |
| US-14.3.24 | game developer (P-15)   | F-14.3.14            | R-14.3.14            |
| US-14.3.25 | QA tester (P-19)        | F-14.3.15            | R-14.3.15            |
| US-14.3.26 | engine developer (P-26) | F-14.3.15            | R-14.3.15            |
| US-14.3.27 | engine tester (P-27)    | F-14.3.15            | R-14.3.15            |
| US-14.3.28 | game developer (P-15)   | F-14.3.14            | R-14.3.14            |
| US-14.3.29 | engine developer (P-26) | F-14.3.16            | R-14.3.16            |
| US-14.3.30 | game developer (P-15)   | F-14.3.16            | R-14.3.16            |
| US-14.3.31 | engine tester (P-27)    | F-14.3.16            | R-14.3.16            |
| US-14.3.32 | engine developer (P-26) | F-14.3.17            | R-14.3.17            |
| US-14.3.33 | QA tester (P-19)        | F-14.3.17            | R-14.3.17            |
| US-14.3.34 | game developer (P-15)   | F-14.3.17, F-14.3.8  | R-14.3.17            |

1. **US-14.3.1** — a work-stealing thread pool sized to performance cores with per-thread local
   queues, so that CPU-parallel work (physics, culling, asset decompression) is distributed evenly
   across all available cores
2. **US-14.3.2** — thread affinity and OS-level priority control to pin the main and render threads
   to performance cores and background I/O to efficiency cores, so that game-critical work is never
   preempted by background tasks on hybrid CPU architectures
3. **US-14.3.3** — a DAG-based job system with typed data dependencies, fan-out, fan-in,
   continuations, and nested sub-graphs, so that frame work (physics substeps, rendering prep) is
   parallelized automatically based on declared dependencies
4. **US-14.3.4** — lightweight fibers with platform-specific primitives (CreateFiber on Windows, GCD
   dispatch queues on macOS, assembly on Linux) that can suspend mid-execution and resume on any
   worker thread, so that pathfinding, procedural generation, and other long-running jobs yield
   without wasting a thread
5. **US-14.3.5** — platform async I/O completions (IOCP, GCD, io_uring) dispatched as job-system
   continuations, so that file reads, network operations, and GPU readbacks integrate with the
   unified task scheduler without blocking any worker thread
6. **US-14.3.6** — the engine to fully utilize my CPU during large-scale content (raids, guild
   sieges) without frame time spikes, so that gameplay stays smooth even with hundreds of entities
   updating simultaneously
7. **US-14.3.7** — automatic detection of performance and efficiency cores via cpuid (x86) and
   IOKit/sysctl (Apple Silicon) at startup, so that thread pool sizing and affinity are configured
   correctly on hybrid architectures without manual tuning
8. **US-14.3.8** — benchmarks that submit imbalanced job graphs (some branches heavy, some light)
   and measure per-thread utilization, so that I can verify work-stealing achieves at least 80% CPU
   utilization under realistic workloads
9. **US-14.3.9** — automated tests that create, suspend, and resume fibers on all platforms
   (CreateFiber on Windows, GCD dispatch queues on macOS, assembly on Linux) and verify stack
   integrity, so that fiber implementation regressions are caught in CI
10. **US-14.3.10** — stress tests that issue concurrent file reads and network operations under full
    CPU load and verify no worker thread blocks for I/O, so that I can confirm the async I/O
    integration never causes thread starvation
11. **US-14.3.11** — microbenchmarks that measure per-job dispatch and completion overhead in the
    task graph scheduler, so that scheduling cost does not exceed a fixed budget (target < 1
    microsecond per job dispatch) on all platforms
12. **US-14.3.12** — configurable fiber stack sizes per workload category (default 64 KiB with guard
    pages), so that pathfinding and procedural generation fibers have sufficient stack space without
    wasting memory for simpler jobs
13. **US-14.3.13** — I/O completions processed only at a defined reactor poll point in the game
    loop, so that I/O events never cause mid-frame latency spikes from unexpected callback execution
14. **US-14.3.14** — all asynchronous operations (file I/O, GPU sync, network) to use async/await,
    so that I can write linear async code without callback pyramids or manual future polling
15. **US-14.3.15** — to spawn parallel tasks that borrow from the current stack frame without Arc or
    'static, so that per-frame parallel work (physics + culling) is ergonomic and zero-overhead
16. **US-14.3.16** — async mutex and rwlock primitives that yield via .await when contended, so that
    contended locks never block a worker thread even for 1 ms
17. **US-14.3.17** — to register both sync and async event handlers, so that event responses that
    require I/O or long computation don't block the dispatch thread
18. **US-14.3.19** — GCD dispatch_io callbacks drained at a controlled point in the frame loop, so
    that macOS I/O completion timing is deterministic and does not interrupt frame work
19. **US-14.3.20** — to await a "next frame" operation from within an async task, so that long
    procedural generation or loading work can yield at frame boundaries without blocking
20. **US-14.3.21** — tests that verify no I/O completion wakes a future between poll() calls, so
    that the controlled poll model is validated in CI
21. **US-14.3.22** — benchmarks that measure async mutex contention overhead and verify it stays
    under 1 us, so that async locks are validated as non-blocking
22. **US-14.3.23** — a safe game loop graph that validates access patterns and dependencies at
    compile time, so that scheduling bugs (data races, deadlocks, missing dependencies) are caught
    before the game runs
23. **US-14.3.24** — to define a custom game loop graph with game-specific phases (e.g., turn
    resolution, AI planning) that integrates with the engine's built-in phases, so that I can
    customize frame structure for my game without forking the engine or modifying engine source code
24. **US-14.3.25** — compile-time graph validation to produce structured error messages identifying
    the exact cycle, access conflict, or missing dependency, so that invalid configurations fail
    immediately with actionable diagnostics
25. **US-14.3.26** — the game loop compiler to reject graphs where two phases write to the same
    resource concurrently, so that data races are impossible by construction
26. **US-14.3.27** — test cases for every `GraphError` variant (cycle, access conflict, missing
    dependency), so that the compiler's validation logic has full coverage in CI
27. **US-14.3.28** — to add custom phases (e.g., card resolution, replay capture) as new nodes in
    the game loop graph with explicit dependencies on existing phases, so that custom logic executes
    at the right point in the frame without ad-hoc ordering hacks
28. **US-14.3.29** — the compiled frame to be reusable across frames without recompilation when only
    per-frame data changes, so that compilation cost is amortized and per-frame overhead stays below
    1 microsecond
29. **US-14.3.30** — to bind new world state, input events, and delta time to the compiled frame
    each frame without triggering graph recompilation, so that frame setup is fast and predictable
30. **US-14.3.31** — tests that instrument the compiled frame and assert no topology mutations occur
    during per-frame parameter binding, so that the compile-once-run-many invariant is verified in
    CI
31. **US-14.3.32** — all public game loop graph and task graph APIs to be safe Rust with no `unsafe`
    in user-facing types, so that memory bugs are impossible when composing frame schedules
32. **US-14.3.33** — static analysis that scans public task graph types for `unsafe` blocks, so that
    the safe API guarantee is enforced automatically in CI
33. **US-14.3.34** — graph nodes to borrow frame data via scoped lifetimes without `Arc` or
    `'static`, so that per-frame parallel work is ergonomic and zero-overhead
