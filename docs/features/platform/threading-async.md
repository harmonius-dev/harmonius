# 14.3 — Threading & Async

## Thread Pool

| ID       | Feature                      | Requirements |
|----------|------------------------------|--------------|
| F-14.3.1 | Work-Stealing Thread Pool    | R-14.3.1     |
| F-14.3.2 | Thread Affinity and Priority | R-14.3.2     |

1. **F-14.3.1** — A fixed-size thread pool with per-thread local queues and work-stealing for load
   balancing. Worker count defaults to the number of performance cores, excluding efficiency cores
   on hybrid architectures. This is the primary execution backend for all CPU-parallel work in the
   engine, from physics broadphase to culling to asset decompression.
   - **Platform:** Windows uses `CreateThread` with `SetThreadAffinityMask` for core pinning; macOS
     uses `pthread_create` with QoS classes (`QOS_CLASS_USER_INTERACTIVE` for game threads,
     `QOS_CLASS_UTILITY` for background work); Linux uses `pthread_create` with
     `pthread_setaffinity_np`. Hybrid core detection uses `cpuid` (x86) or IOKit/sysctl (Apple
     Silicon).
2. **F-14.3.2** — Pin threads to specific cores and set OS-level priority classes. The game loop
   thread and render submission thread run at elevated priority on performance cores. Background I/O
   and telemetry threads run at low priority on efficiency cores to avoid contention during MMO raid
   encounters with hundreds of entities.
   - **Deps:** F-14.3.1
   - **Platform:** Windows uses `SetThreadAffinityMask` and `SetThreadPriority`; macOS uses
     `pthread_set_qos_class_self_np` (preferred over raw priority on Apple platforms); Linux uses
     `pthread_setaffinity_np` and `sched_setscheduler`. Apple Silicon distinguishes P-cores and
     E-cores via `hw.nperflevels` sysctl.

## Job System

| ID       | Feature                              | Requirements |
|----------|--------------------------------------|--------------|
| F-14.3.3 | Task Graph Job System                | R-14.3.3     |
| F-14.3.4 | Fiber and Stackful Coroutine Support | R-14.3.4     |

1. **F-14.3.3** — Express parallel work as a directed acyclic graph of jobs with typed data
   dependencies. The scheduler topologically sorts the graph, dispatches ready jobs to the thread
   pool, and propagates completions. Supports fan-out, fan-in, continuations, and nested sub-graphs
   for hierarchical decomposition of frame work (e.g., physics substeps within the main frame
   graph).
   - **Deps:** F-14.3.1
2. **F-14.3.4** — Lightweight fibers that can suspend mid-execution and resume on any worker thread,
   enabling long-running jobs (pathfinding, procedural generation) to yield without blocking a
   thread. Fibers use platform-specific primitives for minimal overhead.
   - **Deps:** F-14.3.1
   - **Platform:** Windows uses `CreateFiber`/`SwitchToFiber` via COM wrappers. macOS uses GCD
     dispatch queues and blocks: each fiber is modeled as a suspended block on a serial dispatch
     queue, with `dispatch_suspend`/`dispatch_resume` controlling execution. GCD dispatch queues are
     accessed through `objc2`. This avoids deprecated `ucontext` and fragile custom assembly on
     Apple platforms. Linux uses inline assembly for lightweight context switches with explicit
     stack allocation. Stack sizes default to 64 KiB per fiber with guard pages on all platforms.

## Async Runtime

| ID        | Feature                                       | Requirements       |
|-----------|-----------------------------------------------|--------------------|
| F-14.3.5  | Platform Async I/O Integration                | R-14.3.5           |
| F-14.3.6  | Tokio Runtime with Controlled Frame Poll Point | R-14.3.5           |
| F-14.3.7  | Async/Await for All Asynchronous Abstractions | R-14.3.5, R-14.3.3 |
| F-14.3.8  | Scoped Task Execution                         | R-14.3.1, R-14.3.3 |
| F-14.3.9  | Async Synchronization Primitives              | R-14.3.5           |
| F-14.3.10 | Event Handler System                          | R-14.3.3, R-14.3.5 |
| F-14.3.12 | GCD Controlled Dispatch Drain on macOS        | R-14.3.5           |
| F-14.3.13 | Wait for Next Frame as Async Operation        | R-14.3.5, R-14.3.4 |

1. **F-14.3.5** — Bridge the engine's async task system with the Tokio `current_thread` runtime so
   that file reads, network operations, and GPU readbacks complete without blocking worker threads.
   Completion events are dispatched as jobs in the task graph, maintaining a unified scheduling
   model.
   - **Deps:** F-14.3.3, F-1.8.1
   - **Platform:** Tokio handles platform I/O internally (IOCP on Windows, kqueue on macOS, epoll on
     Linux). The engine uses `tokio::fs` for file I/O and `tokio::net` for network I/O.
2. **F-14.3.6** — The game loop owns a Tokio `current_thread` runtime that drives async I/O and
   drains completions only when explicitly polled via `runtime.poll()`. The OS never fires callbacks
   asynchronously. The engine decides exactly when completions are harvested, giving deterministic
   control over when async tasks resume. `poll()` is non-blocking — it drains ready events and
   returns immediately. Multiple poll points per frame are supported to reduce I/O response latency
   below one full frame interval.
   - **Deps:** F-14.3.5, F-14.3.1
   - **Platform:** All platforms: Tokio `current_thread` runtime is polled via `poll()` which drives
     the I/O event loop for one non-blocking iteration.
3. **F-14.3.7** — All I/O operations, GPU synchronization, long waits, and frame-boundary yields use
   Rust's `async`/`await`. No callbacks. Futures yield at `.await` points, freeing worker threads to
   execute other tasks. Synchronous blocking is only permitted for sub-microsecond critical sections
   where the cost of async overhead would exceed the wait time.
   - **Deps:** F-14.3.5, F-14.3.6
   - **Platform:** None. Rust's `async`/`await` compiles to state machines with no platform-specific
     behavior. The platform-specific layer is in the Tokio runtime that wakes futures.
4. **F-14.3.8** — The thread pool supports scoped execution modeled after `std::thread::scope`,
   where spawned tasks may borrow data from the calling scope without requiring `'static` lifetimes
   or `Arc` wrapping. All scoped tasks are joined before the scope exits, guaranteeing that borrowed
   references remain valid. This eliminates unnecessary heap allocation and reference counting for
   per-frame parallel work such as physics stepping and frustum culling.
   - **Deps:** F-14.3.1
   - **Platform:** None. Scoped execution is a lifetime-based Rust abstraction enforced at compile
     time with no platform-specific code.
5. **F-14.3.9** — `AsyncMutex`, `AsyncRwLock`, and `AsyncBarrier` that yield via `.await` instead of
   blocking the calling thread. Waiters suspend as async tasks and are re-enqueued on the thread
   pool when the lock becomes available. Synchronous `try_lock` methods are provided for
   sub-microsecond critical sections where contention is known to be rare. Any wait that could
   exceed a few microseconds must use the async variant to avoid blocking a worker thread.
   - **Deps:** F-14.3.1, F-14.3.7
   - **Platform:** None. Built on atomic operations and Rust `Waker` infrastructure. No OS-level
     synchronization primitives are used.
6. **F-14.3.10** — Both synchronous and asynchronous event handlers for the engine's event dispatch
   system. Synchronous handlers run inline during dispatch and are restricted to cheap, non-blocking
   operations. Asynchronous handlers are spawned as async tasks on the thread pool, allowing
   event-driven code to perform I/O or long operations without blocking the dispatch thread. A
   unified `EventDispatcher` manages subscriptions for both handler types.
   - **Deps:** F-14.3.1, F-14.3.7
   - **Platform:** None. Event dispatch is platform-agnostic. Async handlers use the same thread
     pool and Tokio runtime as all other async tasks.
7. **F-14.3.12** — On macOS, GCD is used exclusively for fiber scheduling (F-14.3.4) and Metal
   command buffer completion handlers — NOT for file or network I/O, which Tokio handles (F-14.3.5).
   Fiber dispatch blocks and Metal completion callbacks are routed to a dedicated serial dispatch
   queue. At the frame poll point, the engine drains this queue synchronously via `dispatch_sync`,
   executing all pending callbacks on the calling thread in a controlled manner. This ensures the
   engine controls exactly when GCD callbacks execute, preventing asynchronous callback firing that
   would violate the deterministic poll model.
   - **Deps:** F-14.3.6, F-14.3.4
   - **Platform:** macOS only. Uses `dispatch_queue_create` for the serial queue and `dispatch_sync`
     at the poll point to drain fiber resumption and Metal completion handlers. GCD/Dispatch is
     accessed through `objc2`. No direct core pinning; thread scheduling is delegated to macOS via
     QoS classes.
8. **F-14.3.13** — Coroutines and async tasks that need to spread work across multiple frames call
   `runtime.next_frame().await`. This yields the task; the runtime resumes it at the next frame's
   poll point. This replaces manual state tracking and frame counters with a natural async
   suspension point, enabling long-running operations like procedural generation, streaming, and
   multi-frame asset processing to be written as sequential async code.
   - **Deps:** F-14.3.6, F-14.3.7
   - **Platform:** None. Frame-boundary yield is managed entirely by the runtime's internal waker
     list. The runtime wakes all `next_frame` waiters at the start of each `poll()` call before
     draining platform completions.

## Game Loop Graph

| ID        | Feature                     | Requirements |
|-----------|-----------------------------|--------------|
| F-14.3.14 | Game Loop Graph             | R-14.3.14    |
| F-14.3.15 | Game Loop Compilation       | R-14.3.15    |
| F-14.3.16 | Per-Frame Parameter Binding | R-14.3.16    |
| F-14.3.17 | Safe Graph Execution        | R-14.3.17    |

1. **F-14.3.14** — A declarative DAG (`GameLoopGraph`) defining one frame's phase structure: input,
   simulation, rendering, audio, and any game-specific phases. Each node declares typed read/write
   access to shared frame resources. The graph is compiled into a `TaskGraph` that the work-stealing
   thread pool executes each frame. This unifies all per-frame scheduling under a single graph
   representation, replacing ad-hoc phase ordering with explicit, validated dependencies.
   - **Deps:** F-14.3.3
   - **Platform:** None. The game loop graph is a Rust-side abstraction with no platform-specific
     behavior.
2. **F-14.3.15** — Safe compile-time validation of the `GameLoopGraph` before execution. The
   compiler detects access conflicts (concurrent mutable access to the same resource), cycles in the
   dependency graph, and missing dependencies. Validation returns
   `Result<CompiledFrame, GraphError>` with structured error variants for each failure mode. Invalid
   graphs cannot be executed.
   - **Deps:** F-14.3.14, F-14.3.3
   - **Platform:** None. Validation is pure Rust logic with no platform-specific code.
3. **F-14.3.16** — A compiled `CompiledFrame` is reused across frames without recompilation.
   Per-frame data (world state, input events, delta time, render targets) is bound to the compiled
   frame via a typed parameter interface before each execution. Binding updates per-frame data slots
   without modifying the compiled topology, barriers, or scheduling decisions.
   - **Deps:** F-14.3.15
   - **Platform:** None. Parameter binding is a Rust-side data operation.
4. **F-14.3.17** — All public task graph and game loop graph APIs are safe Rust. No `unsafe` blocks
   appear in user-facing types (`GameLoopGraph`, `CompiledFrame`, `TaskNode`, `TaskGraph`). Internal
   `unsafe` is permitted only in the thread pool executor and platform FFI, behind safe
   abstractions. Scoped borrows (F-14.3.8) ensure frame data references remain valid for the
   duration of graph execution without requiring `'static` or `Arc`.
   - **Deps:** F-14.3.14, F-14.3.15, F-14.3.8
   - **Platform:** None. Safety is enforced by the Rust type system at compile time.

## Data Parallelism

| ID        | Feature                                          |
|-----------|--------------------------------------------------|
| F-14.3.18 | Data-Parallel Slice Operations (for, map, reduce)|

1. **F-14.3.18** — A suite of data-parallel primitives built on the work-stealing thread pool:
   `parallel_for`, `parallel_map`, `parallel_reduce`, `parallel_for_zip`, `parallel_for_indexed`.
   All primitives operate on mutable and immutable slices within scoped execution, use recursive
   bisection with auto-tuned chunk sizes based on element count and core count, and integrate with
   the task graph so nested parallelism composes correctly. Used by physics broadphase, frustum
   culling, particle updates, animation sampling, and AI perception.
   - **Deps:** F-14.3.1, F-14.3.8
   - **Platform:** None. Built on the work-stealing pool.

## I/O Infrastructure

| ID        | Feature                            |
|-----------|------------------------------------|
| F-14.3.19 | Pre-Allocated Aligned I/O Buffer Pool |
| F-14.3.20 | GPU Direct I/O                     |

1. **F-14.3.19** — A `BufferPool` of pre-allocated, aligned `BufferSlot` objects for zero-copy I/O
   operations. Buffer slots are acquired and released without allocation, supporting direct I/O
   alignment requirements (typically 4 KiB or 512 B) on all platforms. Used by file reads, network
   packet staging, and asset streaming to avoid per-operation allocation.
   - **Deps:** F-14.3.5
   - **Platform:** Alignment matches platform direct I/O requirements: Windows 4 KiB for unbuffered
     I/O, Linux 512 B or filesystem block size, macOS 4 KiB.
2. **F-14.3.20** — Direct disk-to-GPU DMA for asset loading, bypassing the CPU staging path. Uses
   DirectStorage on Windows, Metal I/O (`MTLIOCommandQueue`) on Apple platforms, and a staging
   buffer fallback on Linux (BAR memory or compute-queue upload). Integrated with the platform I/O
   driver so asset loads go directly to GPU resources without CPU copies.
   - **Deps:** F-14.3.5, F-14.3.19
   - **Platform:** Windows DirectStorage 1.2+, Apple Metal I/O on macOS 13+, iOS 16+, tvOS 16+.
     Linux falls back to the staging buffer path.

## Work Graph Emulation

| ID        | Feature                            |
|-----------|------------------------------------|
| F-14.3.21 | CPU-Side GPU Work Graph Emulation  |

1. **F-14.3.21** — Emulate GPU work graphs (D3D12 work graphs) CPU-side on platforms without native
   hardware support by expanding work graph nodes into indirect dispatch chains within the task
   graph. This lets the same work graph authoring model apply to Metal, Vulkan without work graph
   extensions, and older D3D12 drivers, producing equivalent (though lower-bandwidth) execution.
   - **Deps:** F-14.3.3, F-14.3.14
   - **Platform:** All platforms without native work graph support.
