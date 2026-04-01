# Platform Threading Design

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are defined in
> [features/platform/](../../features/), [requirements/platform/](../../requirements/), and
> [user-stories/platform/](../../user-stories/). The table below traces design elements to those
> definitions.

| Feature  | Requirement |
|----------|-------------|
| F-14.3.1 | R-14.3.1    |
| F-14.3.2 | R-14.3.2    |
| F-14.3.3 | R-14.3.3    |
| F-14.3.4 | R-14.3.4    |
| F-14.3.5 | R-14.3.5    |

1. **F-14.3.1** — Work-stealing thread pool sized to performance cores
2. **F-14.3.2** — Thread affinity and OS-level priority classes
3. **F-14.3.3** — DAG-based task graph with fan-out, fan-in, continuations, nested sub-graphs
4. **F-14.3.4** — Lightweight fibers with 64 KiB stacks and guard pages
5. **F-14.3.5** — Platform async I/O bridge dispatching completions as task graph continuations

## Overview

The threading subsystem is the execution backbone of the Harmonius engine. It provides a
work-stealing thread pool, a DAG-based task graph, a Tokio `current_thread` I/O runtime, lightweight
fibers, and async-aware synchronization primitives. All asynchronous abstractions -- I/O, GPU
synchronization, long waits, and frame-boundary yields -- use Rust's `async`/`await`. No callbacks.
Synchronous blocking is only permitted for sub-microsecond critical sections.

The game loop runs on a **dedicated game loop thread** and owns a Tokio `current_thread` runtime.
Each frame, the loop calls `runtime.poll()` at frame boundaries to drain ready I/O completions and
wake async tasks. `poll()` is non-blocking -- it processes whatever is ready and returns
immediately. Tokio handles all platform I/O (epoll on Linux, kqueue on macOS, IOCP on Windows)
through a single unified abstraction. `block_on()` is reserved for operations that must complete
before proceeding (GPU present, task graph execution). `poll()` and `block_on()` are the only points
where I/O futures make progress.

On platforms where the OS owns thread 0 (iOS, Android), the game loop thread is a dedicated thread
separate from the OS main thread. The OS main thread handles platform UI events (UIKit, Activity
lifecycle) and forwards them to the game loop thread via a lock-free SPSC queue. On desktop
platforms (macOS, Windows, Linux), the game loop thread is typically thread 0. This distinction is
transparent to engine code — the Tokio `current_thread` runtime is thread-local and only accessible
from the game loop thread.

The thread pool supports scoped execution (like `std::thread::scope`) so tasks can borrow from the
calling frame without `'static` or `Arc` overhead. All async I/O tasks use `'static` futures with
`Arc` for shared state. Fibers remain available for deep-call-stack workloads (recursive procedural
generation, pathfinding) that cannot be expressed as async state machines. On macOS, fibers use
async/await coroutines -- yield points become `.await` points. Event handlers support both
synchronous and asynchronous variants. All platform-specific code is selected via `cfg` attributes
-- no trait objects, no dynamic dispatch.

## Architecture

### Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_platform::threading"
        CT[CoreTopology]
        TP[ThreadPool]
        TG[TaskGraph]
        TK["Tokio current_thread"]
        FB[FiberPool]
        BP[BufferPool]
        SY[Async Sync Primitives]
    end

    subgraph "platform::windows"
        WT[Win32 Threads]
        WF[CreateFiber]
    end

    subgraph "platform::macos"
        MT[pthread + QoS]
        MF[Async/Await Coroutines]
    end

    subgraph "platform::linux"
        LT[pthread + affinity]
        LF[Assembly Context Switch]
    end

    CT --> TP
    TP --> TG
    TP --> FB
    TK --> BP
    TP --> TK

    TP -.->|"cfg(windows)"| WT
    TP -.->|"cfg(macos)"| MT
    TP -.->|"cfg(linux)"| LT
    FB -.->|"cfg(windows)"| WF
    FB -.->|"cfg(macos)"| MF
    FB -.->|"cfg(linux)"| LF
```

```text
harmonius_platform/
├── threading/
│   ├── topology.rs      # CoreTopology detection
│   ├── pool.rs          # ThreadPool, Worker, Scope,
│   │                    # work-stealing
│   ├── task.rs          # ErasedTask, inline closure
│   │                    # storage
│   ├── graph.rs         # TaskGraphBuilder, TaskGraph
│   ├── io.rs            # Tokio current_thread wrapper
│   ├── fiber.rs         # FiberPool, FiberYielder
│   ├── sync.rs          # AsyncMutex, AsyncRwLock,
│   │                    # AsyncBarrier
│   ├── event.rs         # EventHandler,
│   │                    # AsyncEventHandler
│   └── buffer.rs        # BufferPool, BufferSlot
└── platform/
    ├── windows/
    │   ├── threads.rs   # CreateThread,
    │   │                # SetThreadAffinityMask
    │   └── fibers.rs    # CreateFiber,
    │                    # SwitchToFiber
    ├── macos/
    │   ├── threads.rs   # pthread_create,
    │   │                # QoS classes
    │   └── fibers.rs    # async/await coroutines
    └── linux/
        ├── threads.rs   # pthread_create,
        │                # pthread_setaffinity_np
        └── fibers.rs    # setjmp/longjmp assembly
                         # context switch
```

### Frame Loop and Tokio Poll Point

```mermaid
sequenceDiagram
    participant ML as Game Loop Thread
    participant TK as Tokio Runtime
    participant ECS as ECS Scheduler
    participant TP as ThreadPool
    participant W as Workers
    participant GPU as GPU

    loop Every Frame
        ML->>TK: poll()
        Note over TK: Drains ready I/O, wakes futures
        TK->>TP: re-enqueue woken tasks

        ML->>ECS: build_frame_graph()
        ECS->>TP: execute_graph(graph)
        TP->>W: dispatch ready tasks
        W->>W: execute systems in parallel
        Note over W: Async tasks yield at .await
        W-->>TP: all tasks complete

        ML->>TK: poll()
        ML->>GPU: submit_commands()
        ML->>GPU: present().await
        Note over GPU: Async GPU sync, no CPU spin
    end
```

### Async I/O Completion Flow

```mermaid
sequenceDiagram
    participant T as Async Task
    participant TK as Tokio Runtime
    participant OS as Kernel

    T->>TK: tokio::fs::read(path).await
    TK->>OS: submit async read
    Note over T: Future yields, worker runs other tasks

    Note over OS: Kernel completes read

    Note over TK: poll() drives reactor
    TK->>OS: poll readiness (epoll/kqueue/IOCP)
    OS-->>TK: completion event
    TK->>TK: wake the waiting Future
    Note over T: Task resumes from .await
```

### Core Data Structures

```mermaid
classDiagram
    class TokioRuntime {
        -runtime tokio..runtime..Runtime
        -buffer_pool BufferPool
        +new() TokioRuntime
        +poll()
        +block_on~F~(future) F..Output
        +spawn~F~(future) JoinHandle
    }

    class ThreadPool {
        -workers Vec~Worker~
        -injector Injector~ErasedTask~
        -topology CoreTopology
        +new(config) ThreadPool
        +spawn~F,R~(f) JoinHandle~R~
        +scope(f) R
        +execute_graph(graph) TaskGraphHandle
    }

    class TaskGraphBuilder {
        -nodes Vec~TaskNodeDesc~
        -edges Vec~Edge~
        +add_task(name, f) TaskNodeId
        +add_async_task(name, f) TaskNodeId
        +add_dependency(dep, dependent)
        +add_subgraph(name, sub) TaskNodeId
        +build() Result~TaskGraph~
    }

    class TaskGraph {
        -nodes Vec~TaskNode~
        -sorted_order Vec~TaskNodeId~
        -root_count u32
    }

    class TaskNode {
        -task ErasedTask
        -remaining_deps AtomicU32
        -dependents SmallVec~TaskNodeId 4~
    }

    class FiberPool {
        -fibers Vec~Fiber~
        -free_stack AtomicU32
        +new(config) FiberPool
        +spawn~F~(f) Result~FiberHandle~
    }

    class AsyncMutex~T~ {
        -inner UnsafeCell~T~
        -waiters WaitQueue
        +new(value) AsyncMutex
        +lock() Future~AsyncMutexGuard~
        +try_lock() Option~AsyncMutexGuard~
    }

    ThreadPool --> CoreTopology
    ThreadPool *-- Worker
    ThreadPool --> FiberPool
    ThreadPool --> TokioRuntime
    TaskGraphBuilder ..> TaskGraph : builds
    TaskGraph *-- TaskNode
    TokioRuntime --> BufferPool
```

### Work-Stealing Algorithm

Each worker maintains a local LIFO deque. The worker loop proceeds as:

1. **Try local** — pop from own deque (LIFO preserves cache locality).
2. **Try steal** — steal from a randomly-chosen victim's deque (FIFO end for fairness).
3. **Try global** — pop from the shared injector queue (external submissions).
4. **Park** — no work available; sleep on a condvar until woken by a submission.

When a task graph node completes, it atomically decrements the `remaining_deps` counter of each
dependent. If a dependent reaches zero, it is pushed onto the completing worker's local deque (hot
path) or the injector queue.

## API Design

### Core Topology

```rust
/// Identifies a logical CPU core.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct CoreId(pub u32);

/// CPU core topology distinguishing performance
/// and efficiency cores.
pub struct CoreTopology {
    pub performance_cores: Vec<CoreId>,
    pub efficiency_cores: Vec<CoreId>,
}

impl CoreTopology {
    /// Detect the core topology of the current CPU.
    /// On non-hybrid CPUs all cores are classified
    /// as performance.
    pub fn detect() -> Self;

    pub fn performance_core_count(&self) -> u32;
    pub fn efficiency_core_count(&self) -> u32;
    pub fn total_core_count(&self) -> u32;
}
```

### Thread Pool

```rust
pub struct ThreadPoolConfig {
    /// Number of worker threads. Defaults to
    /// performance core count.
    pub worker_count: Option<u32>,
    /// Name prefix for worker threads (debugger
    /// identification).
    pub name_prefix: &'static str,
}

/// Thread priority levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreadPriority {
    /// Main thread, render submission. Pinned to
    /// performance cores.
    High,
    /// General worker threads.
    Normal,
    /// Background I/O, telemetry. Pinned to
    /// efficiency cores.
    Low,
}

/// A work-stealing thread pool.
pub struct ThreadPool { /* ... */ }

impl ThreadPool {
    pub fn new(config: ThreadPoolConfig) -> Self;

    /// Spawn a one-off task with 'static lifetime.
    pub fn spawn<F, R>(&self, f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;

    /// Scoped execution: tasks within the closure
    /// may borrow from the calling scope. All tasks
    /// are joined before `scope` returns. Modeled
    /// after `std::thread::scope`.
    pub fn scope<'scope, F, R>(&self, f: F) -> R
    where
        F: for<'s> FnOnce(
            &'s Scope<'scope>,
        ) -> R + Send;

    /// Submit a task graph for parallel execution.
    /// Returns a future that resolves when all
    /// tasks in the graph complete.
    pub fn execute_graph(
        &self,
        graph: TaskGraph,
    ) -> impl Future<Output = ()> + Send;

    pub fn worker_count(&self) -> u32;
    pub fn topology(&self) -> &CoreTopology;
}

/// A scope for spawning tasks that borrow from
/// the parent frame.
pub struct Scope<'scope> { /* ... */ }

impl<'scope> Scope<'scope> {
    /// Spawn a scoped task that may borrow data
    /// with lifetime 'scope. Scoped tasks are
    /// synchronous only -- async I/O tasks use
    /// `'static` futures with `Arc` via the Tokio
    /// runtime instead.
    pub fn spawn<F, R>(
        &self,
        f: F,
    ) -> ScopedJoinHandle<'scope, R>
    where
        F: FnOnce() -> R + Send + 'scope,
        R: Send + 'scope;
}

/// Handle to a spawned task's result.
pub struct JoinHandle<R> { /* ... */ }

impl<R> JoinHandle<R> {
    pub fn is_complete(&self) -> bool;
    pub fn join(self) -> R;
}

/// Handle to a scoped task's result. Automatically
/// joined when the parent Scope exits.
pub struct ScopedJoinHandle<'scope, R> {
    /* ... */
}

impl<'scope, R> ScopedJoinHandle<'scope, R> {
    pub fn is_complete(&self) -> bool;
    pub fn join(self) -> R;
}
```

### Data-Level Parallelism

The threading subsystem provides rayon-style data-parallel primitives integrated into the custom
work-stealing pool. These compose with the async executor and scoped borrowing — no external
runtime.

**Design.** All data-parallel operations use recursive splitting (fork-join) within a `Scope`. The
caller picks a chunk size or lets the system auto-tune based on core count and item count. Work is
split recursively until chunks reach the threshold, then processed sequentially per chunk. This
minimizes task spawn overhead while saturating all workers.

```rust
/// Configuration for data-parallel operations.
pub struct ParallelConfig {
    /// Minimum items per chunk before sequential
    /// fallback. Defaults to 1024.
    pub min_chunk_size: u32,
    /// Maximum splits (log2 of parallelism).
    /// Defaults to 2 * worker_count.
    pub max_splits: u32,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            min_chunk_size: 1024,
            max_splits: 0, // 0 = auto
        }
    }
}

impl<'scope> Scope<'scope> {
    /// Parallel for-each over a mutable slice.
    /// Splits `data` into chunks and processes
    /// each chunk on a worker thread. Borrows
    /// `data` for the duration — no 'static or
    /// Arc needed.
    pub fn parallel_for<T, F>(
        &self,
        data: &'scope mut [T],
        config: ParallelConfig,
        f: F,
    ) where
        T: Send + 'scope,
        F: Fn(&mut T) + Send + Sync + 'scope;

    /// Parallel for-each over an immutable slice.
    pub fn parallel_for_ref<T, F>(
        &self,
        data: &'scope [T],
        config: ParallelConfig,
        f: F,
    ) where
        T: Sync + 'scope,
        F: Fn(&T) + Send + Sync + 'scope;

    /// Parallel map: transforms each element,
    /// writing results into `out`. `data` and
    /// `out` must have equal length.
    pub fn parallel_map<T, U, F>(
        &self,
        data: &'scope [T],
        out: &'scope mut [U],
        config: ParallelConfig,
        f: F,
    ) where
        T: Sync + 'scope,
        U: Send + 'scope,
        F: Fn(&T) -> U + Send + Sync + 'scope;

    /// Parallel reduce: splits data, reduces
    /// each chunk with `reduce_fn`, then merges
    /// partial results with `combine_fn`.
    pub fn parallel_reduce<T, R, RF, CF>(
        &self,
        data: &'scope [T],
        config: ParallelConfig,
        identity: R,
        reduce_fn: RF,
        combine_fn: CF,
    ) -> R
    where
        T: Sync + 'scope,
        R: Send + Clone + 'scope,
        RF: Fn(R, &T) -> R + Send + Sync + 'scope,
        CF: Fn(R, R) -> R + Send + Sync + 'scope;

    /// Parallel for-each over two aligned slices
    /// (zip). Both slices must have equal length.
    pub fn parallel_for_zip<A, B, F>(
        &self,
        a: &'scope mut [A],
        b: &'scope [B],
        config: ParallelConfig,
        f: F,
    ) where
        A: Send + 'scope,
        B: Sync + 'scope,
        F: Fn(&mut A, &B) + Send + Sync + 'scope;

    /// Parallel for-each with index. The closure
    /// receives (global_index, &mut item).
    pub fn parallel_for_indexed<T, F>(
        &self,
        data: &'scope mut [T],
        config: ParallelConfig,
        f: F,
    ) where
        T: Send + 'scope,
        F: Fn(usize, &mut T)
            + Send + Sync + 'scope;
}
```

**Splitting algorithm.** `parallel_for` uses recursive bisection:

```mermaid
flowchart TD
    A["parallel_for(slice, f)"] --> B{len > chunk?}
    B -->|Yes| C[Split at midpoint]
    C --> D["spawn left half"]
    C --> E["process right half"]
    D --> F["join left"]
    E --> F
    B -->|No| G["sequential for-each"]
```

1. If `len <= min_chunk_size`, process sequentially in the current task.
2. Otherwise, split the slice at the midpoint.
3. Spawn the left half as a scoped task on the work-stealing deque.
4. Process the right half in the current task (avoids one spawn overhead).
5. Join the left half before returning.

This recursive halving produces `O(log N)` tasks with `O(1)` synchronization per split. The
work-stealing pool balances load across cores automatically.

**Chunk size auto-tuning.** When `max_splits` is 0 (default), the system computes:

```text
max_splits = 2 * worker_count
min_chunk_size = max(64, len / max_splits)
```

This bounds total task count to `4 * cores` while keeping chunks large enough to amortize spawn
overhead. The factor of 2 over-partitions slightly to handle uneven per-element cost.

**Integration with ECS.** The ECS `par_iter` calls `scope.parallel_for` internally, splitting at
archetype chunk boundaries. Each chunk is a contiguous SoA slice, so `parallel_for` receives
pre-aligned, cache-friendly data. The ECS scheduler ensures that `parallel_for` borrows are
compatible with the system's declared access set — no runtime borrow checking needed.

**Integration with async executor.** Data-parallel operations are synchronous within a `Scope` —
they block the calling task until all chunks complete. This is intentional: `parallel_for` is for
CPU-bound bulk work within a single system, not for I/O-bound work. Async tasks that need data
parallelism call `pool.scope(|s| s.parallel_for(...))` inside their system body.

| Consumer | Primitive | Chunk Size | Notes |
|----------|-----------|-----------|-------|
| ECS par_iter | parallel_for | Archetype chunk (16 KiB) | Pre-split by archetype |
| Physics broadphase | parallel_for | 256 pairs | AABB overlap tests |
| Physics solver | parallel_for | 128 contacts | Constraint solving |
| Render culling | parallel_for_ref | 512 objects | Frustum test per object |
| AI perception | parallel_for_indexed | 64 queries | Budget-limited |
| Spatial index rebuild | parallel_reduce | 1024 nodes | BVH SAH evaluation |
| Asset processing | parallel_map | 1 asset | Coarse-grained |
| Particle update | parallel_for | 4096 particles | GPU fallback on mobile |

## Game Loop Graph

The game loop is not hardcoded. Each game defines its frame structure as a `GameLoopGraph` — a
high-level declarative DAG of `Phase` nodes. The graph is compiled once into a `TaskGraph` for
execution. When the active system set changes, the graph is recompiled.

### Concept

**GameLoopGraph** is a directed acyclic graph of phase nodes. Each phase contains ECS systems,
render passes, custom tasks, or nested sub-graphs. Edges express ordering constraints between
phases. The graph defines the frame structure for a specific game — physics-heavy simulations add
more physics sub-phases, render-heavy visualizers expand the render graph, and so on.

**Compilation.** `GameLoopGraph::compile()` transforms the declarative phase graph into an
executable `TaskGraph`:

1. Flatten phases into individual task nodes
2. Insert sync barriers for command buffer flushes
3. Resolve inter-system dependencies via access sets
4. Validate the DAG (cycle detection, access conflicts)
5. Produce a `CompiledFrame` with a `TaskGraph` and render submission ordering

**Render graph integration.** The render graph is a phase within the game loop graph. Render passes
become task graph nodes. GPU command encoding runs as scoped tasks on the thread pool. This means
GPU dispatch ordering is controlled by the same graph as CPU systems.

**CPU work graph emulation.** D3D12 work graphs allow GPU-driven scheduling of variable-rate work.
On platforms without native work graphs, the engine emulates them CPU-side by expanding work graph
nodes into indirect dispatch chains within the task graph. The task graph handles the fan-out.

### Compilation Pipeline

```mermaid
flowchart LR
    GLG["GameLoopGraph"] -->|compile| TG["TaskGraph"]
    TG --> TPE["ThreadPool::execute_graph()"]
    GLG -->|flatten| RP["RenderGraph phases"]
    RP -->|become| TN["Task nodes"]
    TN --> GPU["GPU command encoding"]
    GPU -->|"scoped tasks"| TPE
```

### Phase Types

```rust
/// A phase in the game loop graph.
pub enum PhaseNode {
    /// An ECS system phase (contains systems).
    Systems(SystemPhase),
    /// A render graph phase (contains render
    /// passes).
    RenderGraph(RenderGraphPhase),
    /// A custom task (user-defined closure).
    Task(TaskPhase),
    /// A sub-graph (nested phases).
    SubGraph(GameLoopGraph),
    /// A sync barrier (command buffer flush).
    Barrier,
}

/// High-level game loop structure. Defines one
/// frame as a DAG of phases.
pub struct GameLoopGraph {
    phases: Vec<PhaseNode>,
    edges: Vec<(PhaseId, PhaseId)>,
}

impl GameLoopGraph {
    pub fn new() -> Self;
    pub fn add_phase(
        &mut self,
        name: &'static str,
        node: PhaseNode,
    ) -> PhaseId;
    pub fn add_dependency(
        &mut self,
        before: PhaseId,
        after: PhaseId,
    );
    /// Compile the game loop graph into an
    /// executable task graph. Safe — validates
    /// the DAG (cycle detection, access conflicts)
    /// at compile time.
    pub fn compile(
        &self,
        world: &World,
        pool: &ThreadPool,
    ) -> Result<CompiledFrame, GraphError>;
}

/// A compiled frame ready for execution.
/// Immutable after compilation — can be reused
/// across frames with per-frame parameter binding.
pub struct CompiledFrame {
    task_graph: TaskGraph,
    render_submissions: Vec<RenderSubmission>,
}

impl CompiledFrame {
    /// Execute the frame. All task scheduling,
    /// parallel dispatch, and GPU submission
    /// happen within this call. Safe — borrows
    /// are scoped to the frame.
    pub fn execute(
        &self,
        world: &mut World,
        pool: &ThreadPool,
        io: &GameIoRuntime,
    );
}
```

### Default Phase Table

Each engine subsystem maps to a phase in the graph. Games customize this by adding, removing, or
reordering phases.

| Phase | Type | Contains | Dependencies |
|-------|------|----------|-------------|
| Input | Systems | input polling, action mapping | None |
| Networking | Systems | receive, replication | Input |
| Physics | Systems | broadphase, integration, constraints | Networking |
| AI | Systems | perception, navigation, behavior | Physics |
| Animation | Systems | skeletal, procedural, cloth | Physics, AI |
| Game Logic | Systems | abilities, quests, progression | AI |
| Transform Propagation | Systems | hierarchy, global transforms | Animation, Logic |
| Render Extraction | Systems | extract visible, build draw calls | Transform |
| Render Graph | RenderGraph | culling, encoding, GPU submit | Extraction |
| Audio | Systems | spatial mix, stream, output | Transform |
| Cleanup | Barrier | command flush, entity cleanup | All |

### Safety Guarantees

- **Safe public API.** All public types and methods are safe Rust. No `unsafe` in user-facing types.
- **Compile-time access validation.** Graph compilation validates access patterns at build time —
  conflicting borrows are compile errors, not runtime panics.
- **Scoped execution.** All borrows are valid for the frame duration. The `CompiledFrame::execute`
  call scopes every task borrow to the frame lifetime.
- **Type-enforced exclusivity.** The type system prevents data races: `&World` for read-only
  systems, `&mut World` only at sync barriers.
- **Encapsulated internals.** Internal `unsafe` (work-stealing deque, platform FFI) is encapsulated
  behind safe abstractions with documented invariants.

### Task Graph

```rust
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct TaskNodeId(pub(crate) u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskPriority {
    /// Frame-critical work. Dispatched to
    /// performance cores first.
    High,
    /// Default priority.
    Normal,
    /// Background work. May use efficiency cores.
    Low,
}

pub struct TaskGraphBuilder { /* ... */ }

impl TaskGraphBuilder {
    pub fn new() -> Self;

    /// Add a synchronous task.
    pub fn add_task<F>(
        &mut self,
        name: &'static str,
        f: F,
    ) -> TaskNodeId
    where
        F: FnOnce() + Send + 'static;

    /// Add an async task. The future is polled on
    /// a worker thread and may .await I/O or other
    /// async operations without blocking.
    pub fn add_async_task<F, Fut>(
        &mut self,
        name: &'static str,
        f: F,
    ) -> TaskNodeId
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send;

    /// Declare that `dependent` waits for
    /// `dependency`.
    pub fn add_dependency(
        &mut self,
        dependency: TaskNodeId,
        dependent: TaskNodeId,
    );

    /// Nested sub-graph whose completion acts as
    /// a single node.
    pub fn add_subgraph(
        &mut self,
        name: &'static str,
        subgraph: TaskGraph,
    ) -> TaskNodeId;

    pub fn set_priority(
        &mut self,
        priority: TaskPriority,
    );

    /// Validate DAG (cycle detection) and produce
    /// an immutable graph.
    pub fn build(
        self,
    ) -> Result<TaskGraph, TaskGraphError>;
}

pub struct TaskGraph { /* ... */ }
```

### Tokio I/O Runtime

The game loop owns a Tokio `current_thread` runtime. All async I/O (filesystem, networking, timers)
is driven through Tokio. The engine calls `runtime.poll()` at frame boundaries to drain ready
completions without blocking. `block_on()` is used only when a future must complete before
proceeding. Tokio handles platform differences internally (epoll on Linux, kqueue on macOS, IOCP on
Windows).

```rust
/// Wraps a Tokio current_thread runtime for
/// controlled I/O processing at frame boundaries.
/// Owned by the game loop thread.
pub struct GameIoRuntime { /* ... */ }

impl GameIoRuntime {
    /// Create a new Tokio current_thread runtime.
    /// Must be called from the game loop thread.
    pub fn new() -> Self;

    /// Non-blocking I/O poll. Drains ready I/O
    /// completions, runs woken futures until they
    /// yield, then returns immediately. Does not
    /// wait for new events. Called at frame
    /// boundaries and mid-frame poll points.
    pub fn poll(&self);

    /// Drive I/O futures until `future` completes.
    /// Blocks the calling thread. Use for operations
    /// that must finish before proceeding (GPU
    /// present, shutdown, task graph execution).
    pub fn block_on<F: Future>(
        &self,
        future: F,
    ) -> F::Output;

    /// Spawn a `'static` async I/O task onto the
    /// Tokio runtime. The task makes progress when
    /// `poll()` or `block_on()` is called.
    pub fn spawn<F>(
        &self,
        future: F,
    ) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    /// Async wait for the next frame boundary.
    /// Coroutines use this to spread work across
    /// frames.
    pub async fn next_frame(&self);
}

pub struct IoResult {
    pub bytes_transferred: u32,
    pub buffer: BufferSlot,
}

/// Pool of pre-allocated, aligned I/O buffers
/// for zero-copy.
pub struct BufferPool { /* ... */ }

pub struct BufferSlot { /* ... */ }

impl BufferPool {
    pub fn new(
        buffer_size: usize,
        count: u32,
    ) -> Self;
    pub fn acquire(&self) -> Option<BufferSlot>;
    pub fn release(&self, slot: BufferSlot);
}

impl BufferSlot {
    pub fn as_slice(&self) -> &[u8];
    pub fn as_mut_slice(&mut self) -> &mut [u8];
    pub fn len(&self) -> usize;
}
```

### Fiber Runtime

```rust
pub struct FiberConfig {
    /// Stack size per fiber in bytes.
    /// Default: 65536 (64 KiB).
    pub stack_size: usize,
    /// Number of pre-allocated fibers.
    pub pool_size: u32,
    /// Install guard pages for overflow detection.
    /// Default: true.
    pub guard_pages: bool,
}

pub struct FiberPool { /* ... */ }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FiberHandle(pub(crate) u32);

pub struct FiberYielder { /* ... */ }

impl FiberYielder {
    /// Yield execution. The fiber suspends and
    /// the worker thread picks up the next task.
    /// The fiber is re-queued immediately.
    pub fn yield_now(&self);

    /// Yield until the next frame boundary.
    pub fn yield_until_next_tick(&self);
}

impl FiberPool {
    pub fn new(config: FiberConfig) -> Self;

    /// Spawn a fiber for deep-call-stack workloads
    /// that cannot be expressed as async state
    /// machines.
    pub fn spawn<F>(
        &self,
        f: F,
    ) -> Result<FiberHandle, FiberError>
    where
        F: FnOnce(&FiberYielder) + Send + 'static;

    pub fn active_count(&self) -> u32;
    pub fn capacity(&self) -> u32;
}
```

Fibers are the fallback for workloads with deep recursion (procedural generation, complex
pathfinding). For all I/O and concurrency, prefer `async`/`await`.

### Async Synchronization Primitives

Synchronous locks are only permitted for sub-microsecond critical sections. Any wait that could
exceed a few microseconds must use the async variant to avoid blocking a worker thread even briefly.
In a game, even 1 ms of blocking has significant performance impact.

```rust
/// Async mutex. Waiters yield via .await instead
/// of spinning or parking. Use for any critical
/// section that may be contended.
pub struct AsyncMutex<T> { /* ... */ }

impl<T> AsyncMutex<T> {
    pub fn new(value: T) -> Self;

    /// Async lock. Yields if contended — the
    /// worker picks up other tasks while waiting.
    pub async fn lock(
        &self,
    ) -> AsyncMutexGuard<'_, T>;

    /// Non-blocking try_lock for very short
    /// critical sections where contention is
    /// known to be rare.
    pub fn try_lock(
        &self,
    ) -> Option<AsyncMutexGuard<'_, T>>;
}

pub struct AsyncMutexGuard<'a, T> { /* ... */ }

/// Async read-write lock. Multiple readers,
/// exclusive writers.
pub struct AsyncRwLock<T> { /* ... */ }

impl<T> AsyncRwLock<T> {
    pub fn new(value: T) -> Self;
    pub async fn read(
        &self,
    ) -> AsyncRwLockReadGuard<'_, T>;
    pub async fn write(
        &self,
    ) -> AsyncRwLockWriteGuard<'_, T>;
    pub fn try_read(
        &self,
    ) -> Option<AsyncRwLockReadGuard<'_, T>>;
    pub fn try_write(
        &self,
    ) -> Option<AsyncRwLockWriteGuard<'_, T>>;
}

/// Async barrier for synchronizing multiple tasks.
pub struct AsyncBarrier { /* ... */ }

impl AsyncBarrier {
    pub fn new(count: u32) -> Self;
    pub async fn wait(&self);
}
```

### Event Handlers

Both synchronous and asynchronous handlers are supported for all events. Async handlers allow
event-driven code to perform I/O or long operations without blocking the dispatch thread.

```rust
/// Synchronous event handler — runs inline during
/// dispatch. Only for cheap, non-blocking
/// operations.
pub trait EventHandler<E> {
    fn handle(&mut self, event: &E);
}

/// Asynchronous event handler — dispatched as an
/// async task. Use when the handler needs to
/// .await I/O or other async ops.
pub trait AsyncEventHandler<E> {
    fn handle(
        &mut self,
        event: &E,
    ) -> impl Future<Output = ()> + Send;
}

/// Event dispatcher supporting mixed sync and
/// async handlers.
pub struct EventDispatcher<E> { /* ... */ }

impl<E: Send + Sync + 'static> EventDispatcher<E> {
    pub fn new() -> Self;

    pub fn subscribe_sync<H>(
        &mut self,
        handler: H,
    ) where
        H: EventHandler<E> + Send + 'static;

    pub fn subscribe_async<H>(
        &mut self,
        handler: H,
    ) where
        H: AsyncEventHandler<E> + Send + 'static;

    /// Dispatch an event. Sync handlers run
    /// inline. Async handlers are spawned onto
    /// the thread pool.
    pub fn dispatch(
        &self,
        event: &E,
        pool: &ThreadPool,
    );
}
```

### Error Types

```rust
pub enum TaskGraphError {
    CycleDetected {
        cycle: Vec<TaskNodeId>,
    },
    InvalidDependency {
        from: TaskNodeId,
        to: TaskNodeId,
    },
    EmptyGraph,
}

pub enum IoError {
    NotFound,
    PermissionDenied,
    Cancelled,
    DeviceFull,
    /// Platform-specific error with OS error code.
    Platform { code: i32 },
}

pub enum FiberError {
    PoolExhausted,
    StackOverflow,
}
```

## Data Flow

### Frame Lifecycle with Tokio Runtime

The game loop runs on the dedicated game loop thread and owns the `GameIoRuntime` (Tokio
`current_thread`). It calls `poll()` at frame boundaries for non-blocking I/O draining, and
`block_on()` for operations that must complete. These are the only paths through which I/O futures
make progress. The OS never fires callbacks asynchronously -- we control exactly when completions
are processed.

On desktop (macOS, Windows, Linux), the game loop thread is typically thread 0. On mobile (iOS,
Android), the game loop thread is a dedicated thread separate from the OS main thread -- the OS main
thread runs the platform UI event loop (UIKit `CFRunLoop`, Android `Looper`) and forwards input
events to the game loop thread via a lock-free SPSC queue.

```rust
// Simplified game loop (runs on game loop thread)
let rt = GameIoRuntime::new();
loop {
    // ---- Non-blocking I/O poll at frame start ----
    rt.poll();

    // ---- Build and run ECS systems ----
    let graph = ecs.build_frame_graph();
    rt.block_on(pool.execute_graph(graph));

    // ---- Mid-frame I/O poll (optional) ----
    rt.poll();

    // ---- Render submission ----
    renderer.submit_commands();

    // ---- GPU sync is async: no CPU spin ----
    rt.block_on(renderer.present());
}
```

**"Wait for next frame" is async:** A coroutine that needs to spread work across frames calls
`io_runtime.next_frame().await`. This yields the task; it resumes at the next `poll()` call.

### I/O Completion Pipeline

1. An async task calls `tokio::fs::read(path).await`.
2. Tokio registers the operation with the platform I/O driver (epoll on Linux, kqueue on macOS, IOCP
   on Windows).
3. The future yields. The worker returns to the work-stealing loop.
4. The OS kernel completes the read asynchronously.
5. At the next `runtime.poll()` call (frame boundary), Tokio polls the I/O driver and wakes the
   future.
6. The woken task is re-enqueued on the thread pool. A worker resumes it from the `.await`.

No worker thread ever blocks on I/O. `poll()` and `block_on()` are the only points where I/O futures
make progress. Multiple `poll()` calls per frame reduce I/O latency below one frame.

### Scoped Execution

```rust
pool.scope(|scope| {
    let physics =
        scope.spawn(|| world.step_physics());
    let culling =
        scope.spawn(|| world.frustum_cull(&camera));

    // Both run in parallel, borrowing &world
    // and &camera.
    physics.join();
    culling.join();

    world.submit_render_commands();
});
```

### Fiber Yield / Resume Cycle

For deep-recursion workloads only (not for I/O — use async for I/O):

1. A fiber calls `yielder.yield_now()`.
2. Context (registers, stack pointer) is saved via platform context switch.
3. The worker returns to the work-stealing loop.
4. The fiber is re-queued. Any worker restores its context and resumes.
5. On completion, the stack returns to the `FiberPool` free list.

## Platform Considerations

### Windows

| Component | API | Notes |
|-----------|-----|-------|
| Threads | `CreateThread` | Via `windows-rs` crate |
| Affinity | `SetThreadAffinityMask` | Bitmask per logical core |
| Priority | `SetThreadPriority` | `THREAD_PRIORITY_HIGHEST` / `_LOWEST` |
| Hybrid detect | `cpuid` leaf 0x1A | Intel Thread Director (Alder Lake+) |
| Fibers | `CreateFiber` / `SwitchToFiber` | Native OS fiber support |
| I/O | Tokio (IOCP) | Tokio `current_thread` drives IOCP internally |

### macOS

| Component     | API                                 |
|---------------|-------------------------------------|
| Threads       | `pthread_create`                    |
| Affinity      | QoS classes                         |
| Priority      | QoS: `USER_INTERACTIVE` / `UTILITY` |
| Hybrid detect | `sysctl hw.nperflevels`             |
| Fibers        | Async/await coroutines              |
| I/O           | Tokio (kqueue)                      |

1. **Threads** -- Via libc crate
2. **Affinity** -- `pthread_set_qos_class_self_np` (no direct core pinning)
3. **Priority** -- macOS schedules P/E via QoS
4. **Hybrid detect** -- Apple Silicon P/E core counts
5. **Fibers** -- Async/await coroutines. Yield points become `.await` points. No GCD dispatch
   blocks, no custom assembly.
6. **I/O** -- Tokio `current_thread` using kqueue internally. All I/O driven by `runtime.poll()` at
   frame boundaries.

### Linux

| Component     | API                            |
|---------------|--------------------------------|
| Threads       | `pthread_create`               |
| Affinity      | `pthread_setaffinity_np`       |
| Priority      | `sched_setscheduler`           |
| Hybrid detect | `/sys/devices/system/cpu/`     |
| Fibers        | Custom assembly context switch |
| I/O           | Tokio (epoll)                  |

1. **Threads** -- Via libc crate
2. **Affinity** -- CPU set bitmask
3. **Priority** -- `SCHED_OTHER` + nice
4. **Hybrid detect** -- `cpuid` 0x1A or sysfs
5. **Fibers** -- `setjmp`/`longjmp` with explicit stack
6. **I/O** -- Tokio `current_thread` using epoll internally. All I/O driven by `runtime.poll()` at
   frame boundaries.

### Mobile and Console Platforms

| Platform | Thread Pool         | I/O Backend        |
|----------|---------------------|--------------------|
| iOS      | Custom (shared)     | Tokio (kqueue)     |
| Android  | std thread pool     | Tokio (epoll)      |
| Consoles | Platform thread API | Platform async I/O |

1. **iOS** -- QoS classes for thermal throttling. UIKit owns the OS main thread (`UIApplicationMain`
   / `CFRunLoop`), so the game loop runs on a dedicated game loop thread. Input events (touch,
   accelerometer, keyboard) arrive on the OS main thread via UIKit and are forwarded to the game
   loop thread through a lock-free SPSC queue. The render thread presents independently via Metal
   when frames are ready.
2. **Android** -- Thread affinity respects big.LITTLE core topology. The game loop runs on a
   dedicated thread, separate from the `NativeActivity` main thread. Input events are forwarded via
   the same SPSC queue pattern as iOS.
3. **Consoles** -- Vendor-specific thread affinity and priority. NDA APIs.

On all mobile and console platforms, the game loop thread is distinct from the OS main thread.
Desktop platforms (macOS, Windows, Linux) may run the game loop on thread 0.

Thread pool sizing adapts to core count: mobile devices typically have 4-8 cores with heterogeneous
performance. The `ThreadPool` constructor queries `std::thread::available_parallelism()` and applies
a platform-specific scaling factor.

### Scaling Tiers

| Tier | Core Count | Workers | Fiber Pool | Buffer Pool |
|------|-----------|---------|------------|-------------|
| Mobile | 4 P + 4 E | 4 | 32 fibers | 64 x 64 KiB |
| Desktop | 8 P + 8 E | 8 | 128 fibers | 256 x 64 KiB |
| High-end | 16 P + 16 E | 16 | 256 fibers | 512 x 64 KiB |

### Proposed Dependencies

| Crate | Purpose | Justification |
|-------|---------|---------------|
| `crossbeam-deque` | Chase-Lev deque | Industry-standard; used by rayon |
| `crossbeam-utils` | `CachePadded`, `Backoff` | Prevents false sharing on atomics |
| `tokio` | Async I/O runtime | `current_thread` for all I/O |
| `windows-rs` | Win32 API bindings | Zero-cost FFI to threads, fibers |
| `smallvec` | Inline small vectors | Task node dependent lists |

## Safety Invariants

### Tokio Runtime Single-Threaded Access (Medium)

The `GameIoRuntime` wraps a Tokio `current_thread` runtime, which is inherently single-threaded.
Only the game loop thread may call `poll()` or `block_on()`. Enforce via `debug_assert!` that the
calling thread matches the thread that created the runtime (not necessarily the OS main thread -- on
iOS and Android, the game loop thread is a dedicated thread separate from thread 0). Store the game
loop thread ID at `GameIoRuntime::new()` time and assert against it.

## Feasibility Notes

### Tokio `current_thread` Latency (Medium Risk)

Tokio's `current_thread` runtime polls I/O only inside `poll()` or `block_on()`. If the game loop
has long CPU-bound phases between `poll()` calls, I/O latency increases. **Mitigation:** Insert
additional `poll()` calls at mid-frame points to reduce worst-case I/O latency. Profile to find the
optimal number of poll points per frame.

### macOS Fiber Coroutines (Medium Risk)

macOS fibers use async/await coroutines instead of OS-level context switching. Yield points become
`.await` points. This aligns with the async-first constraint but means deep-recursion workloads on
macOS must be refactored into iterative async state machines. **Mitigation:** Provide a
`FiberIterator` helper that converts recursive algorithms into iterative `.await` loops.

## Test Plan

### Unit Tests

| Test                                   | Req      |
|----------------------------------------|----------|
| `test_work_stealing_10k_jobs`          | R-14.3.1 |
| `test_worker_count_matches_perf_cores` | R-14.3.1 |
| `test_graph_fan_out_fan_in`            | R-14.3.3 |
| `test_graph_nested_subgraph`           | R-14.3.3 |
| `test_graph_cycle_detection`           | R-14.3.3 |
| `test_fiber_suspend_resume`            | R-14.3.4 |
| `test_fiber_guard_page`                | R-14.3.4 |
| `test_tokio_async_io`                  | R-14.3.5 |
| `test_poll_drives_io`                 | R-14.3.5 |
| `test_async_mutex_no_block`            | R-14.3.5 |

1. **`test_work_stealing_10k_jobs`** -- Enqueue 10,000 jobs, verify all complete. Run under
   ThreadSanitizer.
2. **`test_worker_count_matches_perf_cores`** -- On hybrid CPU, assert workers = perf core count.
3. **`test_graph_fan_out_fan_in`** -- 1 root -> 4 parallel -> 1 join. Verify correct result.
4. **`test_graph_nested_subgraph`** -- Sub-graph completes before parent continuation.
5. **`test_graph_cycle_detection`** -- Cycle in graph -> `CycleDetected` error.
6. **`test_fiber_suspend_resume`** -- Fiber suspends, resumes on different worker.
7. **`test_fiber_guard_page`** -- 64 KiB fiber stack overflow -> guard page fault.
8. **`test_tokio_async_io`** -- Async task `.await`s `tokio::fs::read`; verify data, no worker
   blocks.
9. **`test_poll_drives_io`** -- Spawn I/O task, verify it only completes after `poll()` is called.
10. **`test_async_mutex_no_block`** -- Contended async mutex yields worker, does not block.

### Integration Tests

| Test                         | Req      |
|------------------------------|----------|
| `test_affinity_per_platform` | R-14.3.2 |
| `test_tokio_read_10mb`       | R-14.3.5 |
| `test_utilization_imbalance` | R-14.3.1 |
| `test_fiber_cross_thread`    | R-14.3.4 |
| `test_tokio_frame_boundary`  | R-14.3.5 |

1. **`test_affinity_per_platform`** -- Verify game loop thread on perf core, background on
   efficiency core.
2. **`test_tokio_read_10mb`** -- 10 MB async read via Tokio, no worker blocks, data integrity check.
3. **`test_utilization_imbalance`** -- Imbalanced graph, assert >= 80% utilization.
4. **`test_fiber_cross_thread`** -- Fiber suspended on worker N resumes on worker M.
5. **`test_tokio_frame_boundary`** -- Verify I/O futures only make progress inside `poll()` or
   `block_on()` calls.

### Benchmarks

| Benchmark | Target | Source |
|-----------|--------|--------|
| Job dispatch overhead | < 1 us per dispatch | US-14.3.11 |
| Work-stealing utilization | >= 80% across workers | R-14.3.1 |
| Fiber context switch | < 500 ns | US-14.3.9 |
| Tokio poll (100 completions) | < 50 us | US-14.3.10 |
| AsyncMutex contended lock | < 1 us (yield, not spin) | - |
| I/O throughput | >= 80% raw disk bandwidth | US-14.6.11 |

## Design Q & A

**Q1. What is the biggest constraint limiting this design?**

The macOS fiber model is the most limiting. macOS fibers use async/await coroutines instead of
OS-level context switching, so deep-recursion workloads must be refactored into iterative async
state machines. Lifting this would allow assembly-based context switching on macOS (like Linux),
giving uniform fiber behavior across all platforms.

**Q2. How can this design be improved?**

The `poll()` frequency is a critical tuning parameter (open question 3). A single `poll()` per frame
adds up to 16 ms I/O latency at 60 fps, which is unacceptable for networking. The design supports
multiple `poll()` calls per frame but does not specify the optimal count or placement. An adaptive
strategy that inserts extra `poll()` calls when I/O-heavy systems are active would balance latency
against overhead.

**Q3. Is there a better approach?**

Using Rayon for the work-stealing thread pool would provide a battle-tested implementation with
excellent steal heuristics. We are not using it because Rayon's scope model does not integrate with
async/await (Rayon tasks are synchronous closures, not futures). Building a custom pool lets us
unify sync tasks, async tasks, and fiber yield points under a single scheduling model. Tokio handles
all async I/O, so the custom pool focuses purely on CPU compute scheduling.

**Q4. Does this design solve all customer problems?**

US-14.3.12 requests configurable fiber stack sizes per workload category, but the design only
provides a single `FiberConfig::stack_size` (open question 5). Missing: fiber stack usage profiling
to help developers choose appropriate sizes. Missing: a mechanism for tasks to report their progress
(0-100%) for loading screen UIs. Adding progress reporting would enable loading bars for asset
streaming, procedural generation, and world loading (US-14.3.20).

**Q5. Is this design cohesive with the overall engine?**

The threading subsystem is the execution backbone referenced by every other design. Tokio's
`current_thread` runtime provides uniform I/O across all platforms, used consistently by networking
(transport, replication), platform I/O (filesystem, clipboard), and database access (MMO
persistence). The work-stealing pool with scoped execution matches the physics and rendering
designs' parallel iteration patterns. `AsyncMutex`/`AsyncRwLock` replace `std::sync` locks
engine-wide, enforcing the sub-microsecond blocking constraint. All proposed dependencies
(`crossbeam-deque`, `tokio`, `windows-rs`) are low-level libraries consistent with the no-frameworks
guideline.

## Open Questions

1. **Work-stealing victim selection** -- Randomized vs round-robin. Randomized avoids contention
   patterns but may increase steal latency.
2. **Task inline storage** -- Size of inline buffer for `ErasedTask` before heap fallback (64 bytes
   = cache line, 128 bytes = more coverage).
3. **`poll()` frequency** -- One `poll()` per frame adds up to 16 ms I/O latency at 60 fps. Multiple
   calls per frame reduce latency but add overhead. Optimal count depends on workload.
4. **Fiber stack size categories** -- Single size (64 KiB) vs per-workload categories (64/256 KiB/1
   MiB). US-14.3.12 requests configurability.
5. **GPU fence async integration** -- GPU present/fence wait should go through the Tokio runtime.
   Need to define how GPU completion events (Vulkan timeline semaphores, Metal command buffer
   completion handlers, D3D12 fence) integrate with Tokio's I/O driver.
6. **Tokio feature flags** -- Determine minimal Tokio feature set (`rt`, `io-util`, `fs`, `net`,
   `time`) to minimize binary size and compile time.
