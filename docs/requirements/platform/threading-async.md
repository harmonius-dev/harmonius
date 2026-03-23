# R-14.3 — Threading & Async Requirements

## Thread Pool

| ID       | Derived From                                           |
|----------|--------------------------------------------------------|
| R-14.3.1 | [F-14.3.1](../../features/platform/threading-async.md) |
| R-14.3.2 | [F-14.3.2](../../features/platform/threading-async.md) |

1. **R-14.3.1** — The engine **SHALL** provide a fixed-size thread pool with per-thread local queues
   and work-stealing for load balancing, defaulting the worker count to the number of performance
   cores and excluding efficiency cores on hybrid architectures.
   - **Rationale:** A work-stealing pool ensures even load distribution across cores without
     centralized contention, and excluding efficiency cores prevents latency-sensitive work from
     running on slow cores in hybrid CPU designs.
   - **Verification:** Unit test: enqueue 10,000 independent jobs and assert all complete without
     deadlock or data races (run under ThreadSanitizer). Measure throughput and assert work-stealing
     achieves at least 80% CPU utilization across worker threads. On a hybrid-architecture CPU,
     assert the default worker count equals the performance core count, not the total core count.
2. **R-14.3.2** — The engine **SHALL** pin threads to specific cores and set OS-level priority
   classes, running the game loop thread and render submission thread at elevated priority on
   performance cores and background threads at low priority on efficiency cores.
   - **Rationale:** Pinning latency-sensitive threads to performance cores and relegating background
     work to efficiency cores prevents contention during frame-critical work such as physics and
     rendering.
   - **Verification:** Integration test: on each platform, start the engine and query the OS for the
     game loop thread's affinity mask and priority. Assert the game loop thread is pinned to a
     performance core and runs at elevated priority. Start a background I/O thread and assert it is
     pinned to an efficiency core (on hybrid architectures) with low priority.

## Job System

| ID       | Derived From                                           |
|----------|--------------------------------------------------------|
| R-14.3.3 | [F-14.3.3](../../features/platform/threading-async.md) |
| R-14.3.4 | [F-14.3.4](../../features/platform/threading-async.md) |

1. **R-14.3.3** — The engine **SHALL** express parallel work as a directed acyclic graph of jobs
   with typed data dependencies, topologically sort the graph, dispatch ready jobs to the thread
   pool, and support fan-out, fan-in, continuations, and nested sub-graphs.
   - **Rationale:** A DAG-based job system makes data dependencies explicit and enables the
     scheduler to maximize parallelism while guaranteeing correct ordering, which is essential for
     complex frame work involving physics, culling, and rendering.
   - **Verification:** Unit test: construct a DAG with fan-out (1 job spawning 4), fan-in (4 jobs
     joining into 1), and a continuation chain; assert all jobs execute in topological order and the
     final result is correct. Construct a nested sub-graph and assert inner jobs complete before the
     parent continuation runs. Run under ThreadSanitizer and assert no data races.
2. **R-14.3.4** — The engine **SHALL** provide lightweight fibers that can suspend mid-execution and
   resume on any worker thread, using platform-specific primitives, with a default stack size of 64
   KiB per fiber and guard pages to detect stack overflow.
   - **Rationale:** Fibers allow long-running jobs such as pathfinding and procedural generation to
     yield cooperatively without blocking a worker thread, improving thread pool utilization.
   - **Verification:** Unit test: create a fiber that increments a counter, suspends, resumes on a
     different worker thread, and increments again; assert the final counter value is correct and
     the resume occurred on a different OS thread. Allocate a fiber with a 64 KiB stack and trigger
     a stack overflow; assert the guard page causes a controlled fault rather than silent
     corruption. On macOS, verify that fibers use GCD dispatch queues and blocks for suspension and
     resumption: assert that `dispatch_suspend` and `dispatch_resume` are invoked on the fiber's
     serial queue, and that no `ucontext` or custom assembly context switching is used. Verify GCD
     dispatch queue access through C++ wrappers via `cxx.rs` functions correctly under
     ThreadSanitizer.

## Async Runtime

| ID        | Derived From                                           |
|-----------|--------------------------------------------------------|
| R-14.3.5  | [F-14.3.5](../../features/platform/threading-async.md) |
| R-14.3.6  | [F-14.3.5](../../features/platform/threading-async.md) |
| R-14.3.7  | [F-14.3.5](../../features/platform/threading-async.md) |
| R-14.3.8  | [F-14.3.1](../../features/platform/threading-async.md) |
| R-14.3.9  | [F-14.3.5](../../features/platform/threading-async.md) |
| R-14.3.10 | [F-14.3.5](../../features/platform/threading-async.md) |
| R-14.3.12 | [F-14.3.5](../../features/platform/threading-async.md) |
| R-14.3.13 | [F-14.3.5](../../features/platform/threading-async.md) |

1. **R-14.3.5** — The engine **SHALL** bridge the task graph job system with platform-native async
   I/O (IOCP on Windows, GCD on macOS, io_uring on Linux) so that file reads, network operations,
   and GPU readbacks complete without blocking worker threads, with completions dispatched as
   job-system continuations.
   - **Rationale:** Blocking a worker thread on I/O wastes a core and can stall the frame if the
     thread pool is saturated; integrating async I/O completions as jobs maintains a unified
     scheduling model with no idle threads.
   - **Verification:** Integration test: on each platform, submit an async file read for a 10 MB
     file and assert the completion fires as a job-system continuation without any worker thread
     blocking on the read. Measure thread pool utilization during the I/O operation and assert no
     worker thread is parked waiting for I/O. Verify the read data matches the file contents.
2. **R-14.3.6** — The engine **SHALL** provide an I/O reactor that processes completions only when
   explicitly polled by the game loop. Completions **SHALL NOT** be delivered asynchronously by the
   OS. The poll operation **SHALL** be non-blocking (zero timeout).
   - **Rationale:** Controlled polling gives the game loop deterministic timing over when I/O
     completions wake async tasks, preventing mid-frame interference from OS-scheduled callbacks and
     enabling the engine to batch completion processing at optimal points in the frame.
   - **Verification:** Unit test: submit 10 async reads, advance the OS to completion (mock
     backend), and assert no futures are woken until `reactor.poll()` is called. After `poll()`,
     assert all 10 futures resolve. Call `poll()` with no pending completions and assert it returns
     immediately (measure < 1 us wall time). Run under ThreadSanitizer to verify no data races
     between submission and poll.
3. **R-14.3.7** — All asynchronous operations (I/O, GPU synchronization, long waits, frame-boundary
   yields) **SHALL** use Rust's `async`/`await`. Callbacks **SHALL NOT** be used. Synchronous
   blocking **SHALL** only be permitted for sub-microsecond critical sections.
   - **Rationale:** A single async/await model eliminates callback inversion, enables composable
     concurrency via standard Future combinators, and ensures the compiler generates state machines
     with zero heap allocation, whereas callbacks fragment control flow and prevent structured
     concurrency.
   - **Verification:** Static analysis: scan the codebase for callback registration patterns
     (closures passed to I/O or timer APIs) and assert none exist outside platform backend
     internals. Unit test: perform an async file read, an async GPU fence wait, and an async
     frame-boundary yield, confirming all three use `.await` and no callback closures are registered
     by user code. Verify that no worker thread blocks for more than 1 us during any async operation
     (measured via instrumentation).
4. **R-14.3.8** — The thread pool **SHALL** support scoped execution where spawned tasks may borrow
   data from the calling scope without requiring `'static` lifetimes, with all tasks joined before
   the scope exits.
   - **Rationale:** Frame-local parallel work (physics step, frustum culling) needs to borrow world
     data owned by the caller. Requiring `'static` or `Arc` wrapping adds unnecessary overhead and
     API friction. Scoped tasks guarantee all borrows are valid because the scope blocks until every
     spawned task completes.
   - **Verification:** Unit test: create a local `Vec<u32>` on the stack, spawn two scoped tasks
     that read from it in parallel, and assert correct results after the scope exits. Verify the
     `Vec` is not moved to the heap (no `Arc`). Attempt to compile a test that returns a
     `ScopedJoinHandle` outside the scope and assert it fails with a lifetime error. Run under
     ThreadSanitizer and Miri to confirm no data races or undefined behavior.
5. **R-14.3.9** — The engine **SHALL** provide `AsyncMutex`, `AsyncRwLock`, and `AsyncBarrier` that
   yield via `.await` when contended. A non-blocking `try_lock` **SHALL** be available for
   sub-microsecond critical sections where contention is rare.
   - **Rationale:** Standard synchronous locks (e.g., `std::sync::Mutex`) block the calling worker
     thread when contended, reducing thread pool throughput. Async primitives yield the task back to
     the pool, letting the worker execute other ready tasks while waiting, which is critical for
     maintaining frame budget in a game engine.
   - **Verification:** Unit test: spawn 8 async tasks contending on a single `AsyncMutex`, assert
     all acquire and release without deadlock, and verify via instrumentation that no worker thread
     blocks for more than 1 us while waiting. Test `try_lock`: acquire the mutex, call `try_lock`
     from another task, and assert it returns `None`. Test `AsyncRwLock`: spawn 4 concurrent readers
     and assert all acquire simultaneously, then spawn a writer and assert it waits until readers
     release. Test `AsyncBarrier`: spawn N tasks that call `wait()`, assert all resume only after
     the Nth task arrives.
6. **R-14.3.10** — The engine **SHALL** support both synchronous and asynchronous event handlers.
   Sync handlers **SHALL** run inline during dispatch. Async handlers **SHALL** be spawned as async
   tasks on the thread pool.
   - **Rationale:** Cheap handlers (stat updates, flag toggles) benefit from zero-overhead inline
     dispatch, while expensive handlers (asset streaming triggered by events, network notifications)
     must not block the dispatch thread and should run as async tasks that can yield on I/O.
   - **Verification:** Unit test: register a sync handler that increments an `AtomicU32` and
     dispatch an event; assert the counter increments before `dispatch()` returns (inline
     execution). Register an async handler that performs a mock I/O `.await` and dispatch an event;
     assert the handler runs on a thread pool worker (not the dispatch thread) and completes after
     `reactor.poll()`. Register both sync and async handlers for the same event type and assert both
     execute correctly. Run under ThreadSanitizer.
7. **R-14.3.12** — On macOS, GCD `dispatch_io` callbacks **SHALL** be routed to a serial dispatch
   queue that is drained synchronously at the reactor poll point. The engine **SHALL** control when
   Dispatch callbacks execute.
   - **Rationale:** GCD normally delivers `dispatch_io` callbacks on arbitrary threads at arbitrary
     times. Routing them to a dedicated serial queue and draining it synchronously during
     `reactor.poll()` ensures completions are processed at a deterministic point in the frame,
     matching the controlled-poll model used on Windows (IOCP) and Linux (io_uring).
   - **Verification:** Integration test (macOS only): submit 10 async reads via `dispatch_io`, wait
     for the OS to complete them, and assert no futures are woken until `reactor.poll()` is called.
     Instrument the serial dispatch queue and assert all callback invocations occur within the
     `poll()` call stack (not on a GCD worker thread). Verify that the queue is a serial queue (not
     concurrent) by asserting callbacks execute sequentially.
8. **R-14.3.13** — The engine **SHALL** provide an async operation (`next_frame`) that yields the
   calling task until the next frame's reactor poll point, enabling coroutine-style work spread
   across frames.
   - **Rationale:** Some workloads (progressive LOD generation, background world streaming,
     multi-frame animation blending) need to perform a small amount of work each frame rather than
     completing in a single burst. An async frame-boundary yield lets these tasks use natural
     sequential code with `.await` suspension points instead of manually managing state across frame
     callbacks.
   - **Verification:** Unit test: spawn an async task that calls `next_frame().await` three times,
     incrementing a shared `AtomicU32` each time. Call `reactor.poll()` three times (simulating
     three frames) and assert the counter equals 3 after the third poll. Assert the counter is 0
     before the first poll, 1 after the first, and 2 after the second (one increment per frame).
     Verify the task does not resume between poll calls.

## Game Loop Graph

| ID        | Derived From                                            |
|-----------|---------------------------------------------------------|
| R-14.3.14 | [F-14.3.14](../../features/platform/threading-async.md) |
| R-14.3.15 | [F-14.3.15](../../features/platform/threading-async.md) |
| R-14.3.16 | [F-14.3.16](../../features/platform/threading-async.md) |
| R-14.3.17 | [F-14.3.17](../../features/platform/threading-async.md) |

1. **R-14.3.14** — The engine **SHALL** define the game loop as a `GameLoopGraph` DAG that compiles
   into a `TaskGraph` for execution on the work-stealing thread pool, where each node declares typed
   read/write access to shared frame resources.
   - **Rationale:** A declarative DAG representation makes frame phase dependencies explicit and
     enables the scheduler to maximize parallelism while guaranteeing correct ordering across input,
     simulation, rendering, and audio phases.
   - **Verification:** Unit test: construct a `GameLoopGraph` with four phases (input, simulate,
     render, audio) and explicit data dependencies. Compile to a `TaskGraph` and assert the
     topological order respects all declared dependencies. Run under ThreadSanitizer and assert no
     data races during parallel execution.
2. **R-14.3.15** — The compiler **SHALL** detect access conflicts, cycles, and missing dependencies
   at compile time, returning `Result<CompiledFrame, GraphError>` with structured error variants for
   each failure mode. Invalid graphs **SHALL NOT** be executable.
   - **Rationale:** Catching scheduling errors at compile time eliminates an entire class of runtime
     data races and deadlocks that are difficult to reproduce and diagnose in a multi-threaded game
     loop.
   - **Verification:** Unit test: construct a graph with a cycle and assert compilation returns
     `GraphError::Cycle`. Construct a graph with two nodes writing the same resource concurrently
     and assert `GraphError::AccessConflict`. Construct a graph referencing an undeclared dependency
     and assert `GraphError::MissingDependency`. Construct a valid graph and assert compilation
     returns `Ok(CompiledFrame)`.
3. **R-14.3.16** — A compiled frame **SHALL** be reusable across frames; per-frame data binding
   **SHALL NOT** trigger recompilation. The compiled topology, barrier schedule, and task ordering
   **SHALL** remain unchanged when only per-frame parameters (world state, input events, delta time)
   are updated.
   - **Rationale:** Recompiling the frame graph every frame would waste CPU time rediscovering a
     topology that rarely changes. Separating topology from per-frame data allows the scheduler to
     amortize compilation cost across thousands of frames.
   - **Verification:** Unit test: compile a `GameLoopGraph` once, then execute it across 1,000
     simulated frames with different per-frame data each time. Assert that compilation occurs
     exactly once. Instrument the compiled frame to detect topology mutations and assert none occur
     during per-frame binding. Measure per-frame binding overhead and assert it is below 1
     microsecond.
4. **R-14.3.17** — All public task graph and game loop APIs **SHALL** be safe Rust with no `unsafe`
   blocks in user-facing types. Internal `unsafe` **SHALL** be confined to the thread pool executor
   and platform FFI, behind safe abstraction boundaries.
   - **Rationale:** Memory safety bugs in scheduling code cause data races that are extremely
     difficult to diagnose. A safe public API ensures that users of the game loop graph cannot
     introduce undefined behavior through the scheduling interface.
   - **Verification:** Static analysis: scan all public types and methods in the `game_loop` and
     `task_graph` modules for `unsafe` blocks and assert none exist in public API surface. Run the
     full test suite under Miri to verify absence of undefined behavior. Compile a test that
     attempts to leak a scoped borrow outside the frame scope and assert it fails with a lifetime
     error.
