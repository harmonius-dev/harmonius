# R-14.3 — Threading & Async Requirements

## Thread Pool

### R-14.3.1 Work-Stealing Thread Pool

The engine **SHALL** provide a fixed-size thread pool with per-thread local queues and work-stealing
for load balancing, defaulting the worker count to the number of performance cores and excluding
efficiency cores on hybrid architectures.

- **Derived from:** [F-14.3.1](../../features/platform/threading-async.md)
- **Rationale:** A work-stealing pool ensures even load distribution across cores without
  centralized contention, and excluding efficiency cores prevents latency-sensitive work from
  running on slow cores in hybrid CPU designs.
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
worker thread, using platform-specific primitives, with a default stack size of 64 KiB per fiber and
guard pages to detect stack overflow.

- **Derived from:** [F-14.3.4](../../features/platform/threading-async.md)
- **Rationale:** Fibers allow long-running jobs such as pathfinding and procedural generation to
  yield cooperatively without blocking a worker thread, improving thread pool utilization.
- **Verification:** Unit test: create a fiber that increments a counter, suspends, resumes on a
  different worker thread, and increments again; assert the final counter value is correct and the
  resume occurred on a different OS thread. Allocate a fiber with a 64 KiB stack and trigger a stack
  overflow; assert the guard page causes a controlled fault rather than silent corruption. On macOS,
  verify that fibers use GCD dispatch queues and blocks for suspension and resumption: assert that
  `dispatch_suspend` and `dispatch_resume` are invoked on the fiber's serial queue, and that no
  `ucontext` or custom assembly context switching is used. Verify GCD dispatch queue access through
  C++ wrappers via `cxx.rs` functions correctly under ThreadSanitizer.

## Async Runtime

### R-14.3.5 Platform Async I/O Integration

The engine **SHALL** bridge the task graph job system with platform-native async I/O (IOCP on
Windows, GCD on macOS, io_uring on Linux) so that file reads, network operations, and GPU readbacks
complete without blocking worker threads, with completions dispatched as job-system continuations.

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** Blocking a worker thread on I/O wastes a core and can stall the frame if the thread
  pool is saturated; integrating async I/O completions as jobs maintains a unified scheduling model
  with no idle threads.
- **Verification:** Integration test: on each platform, submit an async file read for a 10 MB file
  and assert the completion fires as a job-system continuation without any worker thread blocking on
  the read. Measure thread pool utilization during the I/O operation and assert no worker thread is
  parked waiting for I/O. Verify the read data matches the file contents.

### R-14.3.6 IoReactor Controlled Poll Point

The engine **SHALL** provide an I/O reactor that processes completions only when explicitly polled
by the game loop. Completions **SHALL NOT** be delivered asynchronously by the OS. The poll
operation **SHALL** be non-blocking (zero timeout).

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** Controlled polling gives the game loop deterministic timing over when I/O
  completions wake async tasks, preventing mid-frame interference from OS-scheduled callbacks and
  enabling the engine to batch completion processing at optimal points in the frame.
- **Verification:** Unit test: submit 10 async reads, advance the OS to completion (mock backend),
  and assert no futures are woken until `reactor.poll()` is called. After `poll()`, assert all 10
  futures resolve. Call `poll()` with no pending completions and assert it returns immediately
  (measure < 1 us wall time). Run under ThreadSanitizer to verify no data races between submission
  and poll.

### R-14.3.7 Async/Await for All Asynchronous Abstractions

All asynchronous operations (I/O, GPU synchronization, long waits, frame-boundary yields) **SHALL**
use Rust's `async`/`await`. Callbacks **SHALL NOT** be used. Synchronous blocking **SHALL** only be
permitted for sub-microsecond critical sections.

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** A single async/await model eliminates callback inversion, enables composable
  concurrency via standard Future combinators, and ensures the compiler generates state machines
  with zero heap allocation, whereas callbacks fragment control flow and prevent structured
  concurrency.
- **Verification:** Static analysis: scan the codebase for callback registration patterns (closures
  passed to I/O or timer APIs) and assert none exist outside platform backend internals. Unit test:
  perform an async file read, an async GPU fence wait, and an async frame-boundary yield, confirming
  all three use `.await` and no callback closures are registered by user code. Verify that no worker
  thread blocks for more than 1 us during any async operation (measured via instrumentation).

### R-14.3.8 Scoped Task Execution

The thread pool **SHALL** support scoped execution where spawned tasks may borrow data from the
calling scope without requiring `'static` lifetimes, with all tasks joined before the scope exits.

- **Derived from:** [F-14.3.1](../../features/platform/threading-async.md)
- **Rationale:** Frame-local parallel work (physics step, frustum culling) needs to borrow world
  data owned by the caller. Requiring `'static` or `Arc` wrapping adds unnecessary overhead and API
  friction. Scoped tasks guarantee all borrows are valid because the scope blocks until every
  spawned task completes.
- **Verification:** Unit test: create a local `Vec<u32>` on the stack, spawn two scoped tasks that
  read from it in parallel, and assert correct results after the scope exits. Verify the `Vec` is
  not moved to the heap (no `Arc`). Attempt to compile a test that returns a `ScopedJoinHandle`
  outside the scope and assert it fails with a lifetime error. Run under ThreadSanitizer and Miri to
  confirm no data races or undefined behavior.

### R-14.3.9 Async Synchronization Primitives

The engine **SHALL** provide `AsyncMutex`, `AsyncRwLock`, and `AsyncBarrier` that yield via `.await`
when contended. A non-blocking `try_lock` **SHALL** be available for sub-microsecond critical
sections where contention is rare.

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** Standard synchronous locks (e.g., `std::sync::Mutex`) block the calling worker
  thread when contended, reducing thread pool throughput. Async primitives yield the task back to
  the pool, letting the worker execute other ready tasks while waiting, which is critical for
  maintaining frame budget in a game engine.
- **Verification:** Unit test: spawn 8 async tasks contending on a single `AsyncMutex`, assert all
  acquire and release without deadlock, and verify via instrumentation that no worker thread blocks
  for more than 1 us while waiting. Test `try_lock`: acquire the mutex, call `try_lock` from another
  task, and assert it returns `None`. Test `AsyncRwLock`: spawn 4 concurrent readers and assert all
  acquire simultaneously, then spawn a writer and assert it waits until readers release. Test
  `AsyncBarrier`: spawn N tasks that call `wait()`, assert all resume only after the Nth task
  arrives.

### R-14.3.10 Event Handler System

The engine **SHALL** support both synchronous and asynchronous event handlers. Sync handlers
**SHALL** run inline during dispatch. Async handlers **SHALL** be spawned as async tasks on the
thread pool.

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** Cheap handlers (stat updates, flag toggles) benefit from zero-overhead inline
  dispatch, while expensive handlers (asset streaming triggered by events, network notifications)
  must not block the dispatch thread and should run as async tasks that can yield on I/O.
- **Verification:** Unit test: register a sync handler that increments an `AtomicU32` and dispatch
  an event; assert the counter increments before `dispatch()` returns (inline execution). Register
  an async handler that performs a mock I/O `.await` and dispatch an event; assert the handler runs
  on a thread pool worker (not the dispatch thread) and completes after `reactor.poll()`. Register
  both sync and async handlers for the same event type and assert both execute correctly. Run under
  ThreadSanitizer.

### R-14.3.12 GCD Controlled Dispatch Drain

On macOS, GCD `dispatch_io` callbacks **SHALL** be routed to a serial dispatch queue that is drained
synchronously at the reactor poll point. The engine **SHALL** control when Dispatch callbacks
execute.

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** GCD normally delivers `dispatch_io` callbacks on arbitrary threads at arbitrary
  times. Routing them to a dedicated serial queue and draining it synchronously during
  `reactor.poll()` ensures completions are processed at a deterministic point in the frame, matching
  the controlled-poll model used on Windows (IOCP) and Linux (io_uring).
- **Verification:** Integration test (macOS only): submit 10 async reads via `dispatch_io`, wait for
  the OS to complete them, and assert no futures are woken until `reactor.poll()` is called.
  Instrument the serial dispatch queue and assert all callback invocations occur within the `poll()`
  call stack (not on a GCD worker thread). Verify that the queue is a serial queue (not concurrent)
  by asserting callbacks execute sequentially.

### R-14.3.13 Async Frame Boundary Wait

The engine **SHALL** provide an async operation (`next_frame`) that yields the calling task until
the next frame's reactor poll point, enabling coroutine-style work spread across frames.

- **Derived from:** [F-14.3.5](../../features/platform/threading-async.md)
- **Rationale:** Some workloads (progressive LOD generation, background world streaming, multi-frame
  animation blending) need to perform a small amount of work each frame rather than completing in a
  single burst. An async frame-boundary yield lets these tasks use natural sequential code with
  `.await` suspension points instead of manually managing state across frame callbacks.
- **Verification:** Unit test: spawn an async task that calls `next_frame().await` three times,
  incrementing a shared `AtomicU32` each time. Call `reactor.poll()` three times (simulating three
  frames) and assert the counter equals 3 after the third poll. Assert the counter is 0 before the
  first poll, 1 after the first, and 2 after the second (one increment per frame). Verify the task
  does not resume between poll calls.
