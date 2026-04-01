# R-14.3 — Threading & Async Requirements

## Thread Pool

1. **R-14.3.1** — The engine **SHALL** provide a fixed-size thread pool with per-thread local queues
   and work-stealing for load balancing, defaulting the worker count to the number of performance
   cores and excluding efficiency cores on hybrid architectures.
   - **Rationale:** Work-stealing ensures even load distribution without centralized contention.
     Excluding efficiency cores prevents latency-sensitive work from running on slow cores.
   - **Verification:** Unit test: enqueue 10,000 jobs and assert all complete without deadlock or
     data races (ThreadSanitizer). Measure throughput and assert at least 80 % CPU utilization. On
     hybrid CPU, assert default worker count equals performance core count.

2. **R-14.3.2** — The engine **SHALL** pin threads to specific cores and set OS-level priority
   classes, running the game loop and render threads at elevated priority on performance cores and
   background threads at low priority on efficiency cores.
   - **Rationale:** Pinning latency-sensitive threads to performance cores and relegating background
     work to efficiency cores prevents contention during frame-critical work.
   - **Verification:** Integration test per platform: start the engine and query the OS for the game
     loop thread's affinity and priority. Assert it is pinned to a performance core. Assert
     background I/O threads run at low priority on efficiency cores (on hybrid architectures).

## Job System

3. **R-14.3.3** — The engine **SHALL** express parallel work as a directed acyclic graph of jobs
   with typed data dependencies, topologically sort the graph, dispatch ready jobs to the thread
   pool, and support fan-out, fan-in, continuations, and nested sub-graphs.
   - **Rationale:** A DAG-based job system makes dependencies explicit and enables the scheduler to
     maximize parallelism while guaranteeing correct ordering.
   - **Verification:** Unit test: construct a DAG with fan-out (1 to 4), fan-in (4 to 1), and nested
     sub-graph; assert topological execution order and correct results. Run under ThreadSanitizer.

4. **R-14.3.4** — The engine **SHALL** provide lightweight fibers that can suspend mid-execution and
   resume on any worker thread, using platform-specific primitives, with a default stack size of 64
   KiB per fiber and guard pages to detect stack overflow.
   - **Rationale:** Fibers allow long-running jobs to yield cooperatively without blocking a worker
     thread, improving thread pool utilization.
   - **Verification:** Unit test: create a fiber, suspend, resume on a different thread, assert
     counter value and thread change. Trigger stack overflow and assert guard page catches it. On
     macOS, assert GCD dispatch queues are used (no `ucontext` or custom assembly).

## Async Runtime

5. **R-14.3.5** — The engine **SHALL** bridge the task graph job system with the Tokio
   `current_thread` runtime so that file reads, network operations, and GPU readbacks complete
   without blocking worker threads, with completions dispatched as job-system continuations.
   - **Rationale:** Blocking a worker thread on I/O wastes a core and can stall the frame.
     Integrating completions as jobs maintains unified scheduling.
   - **Verification:** Integration test: submit a 10 MB Tokio async file read; assert completion
     fires as a job continuation without blocking. Assert no worker thread parks waiting for I/O.

6. **R-14.3.6** — The engine **SHALL** provide a Tokio `current_thread` runtime that processes
   completions only when explicitly polled via `runtime.poll()`. Completions **SHALL NOT** be
   delivered asynchronously by the OS. The poll **SHALL** be non-blocking (zero timeout).
   - **Rationale:** Controlled polling gives the game loop deterministic timing over when
     completions wake async tasks, preventing mid-frame interference from OS-scheduled callbacks.
   - **Verification:** Unit test: submit 10 async reads, advance to completion, assert no futures
     woken until `runtime.poll()`. Assert `poll()` with no pending completions returns in under 1
     us.

7. **R-14.3.7** — All asynchronous operations **SHALL** use Rust's `async`/`await`. Callbacks
   **SHALL NOT** be used. Synchronous blocking **SHALL** only be permitted for sub-microsecond
   critical sections.
   - **Rationale:** A single async/await model eliminates callback inversion and enables composable
     concurrency via standard Future combinators.
   - **Verification:** Static analysis: scan for callback patterns outside platform internals and
     assert none exist. Verify no worker thread blocks for more than 1 us during async operations.

8. **R-14.3.8** — The thread pool **SHALL** support scoped execution where spawned tasks may borrow
   data from the calling scope without `'static` lifetimes, with all tasks joined before the scope
   exits.
   - **Rationale:** Per-frame parallel work needs to borrow world data without `Arc` wrapping.
     Scoped tasks guarantee borrows are valid because the scope blocks until all tasks complete.
   - **Verification:** Unit test: create a local `Vec<u32>`, spawn two scoped tasks reading it,
     assert correct results. Verify no `Arc` wrapping. Assert compile failure when leaking a handle
     outside the scope. Run under ThreadSanitizer and Miri.

9. **R-14.3.9** — The engine **SHALL** provide `AsyncMutex`, `AsyncRwLock`, and `AsyncBarrier` that
   yield via `.await` when contended. A non-blocking `try_lock` **SHALL** be available for
   sub-microsecond critical sections.
   - **Rationale:** Standard synchronous locks block the worker thread when contended. Async
     primitives yield the task back to the pool.
   - **Verification:** Unit test: 8 async tasks contending on `AsyncMutex`; assert no deadlock and
     no worker blocks over 1 us. Test `AsyncRwLock` with 4 concurrent readers and 1 writer. Test
     `AsyncBarrier` with N tasks.

10. **R-14.3.10** — The engine **SHALL** support both synchronous and asynchronous event handlers.
    Sync handlers **SHALL** run inline. Async handlers **SHALL** be spawned as async tasks on the
    thread pool.
    - **Rationale:** Cheap handlers benefit from inline dispatch; expensive handlers must not block
      the dispatch thread.
    - **Verification:** Unit test: register sync handler incrementing `AtomicU32`, assert inline
      execution. Register async handler with mock I/O `.await`, assert it runs on thread pool
      worker. Run under ThreadSanitizer.

11. **R-14.3.12** — On macOS, GCD dispatch blocks for fiber scheduling and Metal GPU command buffer
    completion handlers **SHALL** be routed to a serial dispatch queue drained synchronously at the
    game loop poll point. The engine **SHALL** control when GCD callbacks execute.
    - **Rationale:** GCD normally delivers callbacks on arbitrary threads. Routing to a serial queue
      and draining at the poll point ensures deterministic processing aligned with the game loop.
    - **Verification:** Integration test (macOS): submit fiber resume and Metal command buffer
      completion via GCD, assert no callbacks fire until the poll point. Assert all callbacks
      execute within the poll call stack. Assert serial (not concurrent) execution.

12. **R-14.3.13** — The engine **SHALL** provide `next_frame` as an async operation that yields the
    task until the next frame's reactor poll point.
    - **Rationale:** Workloads spread across frames (LOD generation, background streaming) benefit
      from natural sequential code with `.await` suspension points instead of manual state.
    - **Verification:** Unit test: spawn task calling `next_frame().await` three times, incrementing
      `AtomicU32` each time. Assert counter equals 0 before first poll, 1 after first, 2 after
      second, 3 after third.

## Game Loop Graph

13. **R-14.3.14** — The engine **SHALL** define the game loop as a `GameLoopGraph` DAG that compiles
    into a `TaskGraph` for execution on the thread pool, where each node declares typed read/write
    access to shared frame resources.
    - **Rationale:** A declarative DAG makes phase dependencies explicit and enables the scheduler
      to maximize parallelism while guaranteeing correct ordering.
    - **Verification:** Unit test: construct a graph with four phases and data dependencies. Compile
      and assert topological order. Run under ThreadSanitizer.

14. **R-14.3.15** — The compiler **SHALL** detect access conflicts, cycles, and missing dependencies
    at compile time, returning `Result<CompiledFrame, GraphError>` with structured error variants.
    Invalid graphs **SHALL NOT** be executable.
    - **Rationale:** Catching scheduling errors at compile time eliminates runtime data races and
      deadlocks that are difficult to reproduce.
    - **Verification:** Unit test: construct a graph with a cycle and assert `GraphError::Cycle`.
      Construct concurrent writes and assert `GraphError::AccessConflict`. Construct missing
      dependency and assert `GraphError::MissingDependency`. Construct valid graph and assert
      `Ok(CompiledFrame)`.

15. **R-14.3.16** — A compiled frame **SHALL** be reusable across frames; per-frame data binding
    **SHALL NOT** trigger recompilation. The topology **SHALL** remain unchanged when only per-frame
    parameters change.
    - **Rationale:** Recompiling every frame wastes CPU. Separating topology from data amortizes
      compilation across thousands of frames.
    - **Verification:** Unit test: compile once, execute across 1,000 frames with different data;
      assert compilation occurs exactly once. Assert per-frame binding overhead below 1 microsecond.

16. **R-14.3.17** — All public task graph and game loop APIs **SHALL** be safe Rust with no `unsafe`
    in user-facing types. Internal `unsafe` **SHALL** be confined to the thread pool executor and
    platform FFI.
    - **Rationale:** Memory safety bugs in scheduling code cause data races that are extremely
      difficult to diagnose. A safe API prevents undefined behavior through the scheduling
      interface.
    - **Verification:** Static analysis: scan public types for `unsafe` and assert none exist. Run
      full test suite under Miri. Assert compile failure when leaking a scoped borrow outside the
      frame.
