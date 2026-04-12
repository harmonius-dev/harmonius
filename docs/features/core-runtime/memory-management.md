# 1.7 — Memory Management

## Arena Allocators

| ID      | Feature                                     |
|---------|----------------------------------------------|
| F-1.7.1 | Per-Frame Arena Allocator                   |
| F-1.7.2 | Scoped Arena Allocator with Nested Lifetimes|

1. **F-1.7.1** — Provide a bump allocator that hands out memory from a pre-allocated contiguous
   block and resets to zero cost at frame boundaries. Per-frame arenas are used for transient
   allocations such as command buffers, query results, and scratch data. Eliminating individual
   deallocations reduces allocator overhead and fragmentation in hot loops processing hundreds of
   thousands of entities per tick.
   - **Platform:** Uses platform-native virtual memory APIs for backing storage (VirtualAlloc on
     Windows, mmap on POSIX).
2. **F-1.7.2** — Support nested arena scopes where a child scope allocates from the parent's
   remaining capacity and restores the parent's watermark on drop. Scoped arenas enable temporary
   allocations within a system's execution that are reclaimed without waiting for the frame
   boundary, reducing peak memory usage for systems with bursty allocation patterns.
   - **Deps:** F-1.7.1

## Pool Allocators

| ID      | Feature             |
|---------|----------------------|
| F-1.7.3 | Typed Pool Allocator|

1. **F-1.7.3** — Provide a fixed-size block pool allocator that manages objects of a single type.
   Free blocks are tracked via an intrusive free list, yielding O(1) allocation and deallocation
   with zero fragmentation. Typed pools back ECS component storage columns and resource containers
   where object lifetimes are independent and unpredictable.
   - **Platform:** Mobile: max pool size 16 MB per type, 256 MB total across all pools. Switch: max
     32 MB per type, 512 MB total. Desktop: configurable, default 256 MB per type. Pool growth uses
     virtual memory commit-on-demand on all platforms.

## Resource Handles

| ID      | Feature                   |
|---------|----------------------------|
| F-1.7.4 | Generational Index Handles|
| F-1.7.5 | Slot Map Container        |

1. **F-1.7.4** — Use generational indices — a packed index paired with a generation counter — as
   opaque handles to resources, assets, and entities. On deallocation the generation is incremented,
   causing stale handles to fail validation on next access. Generational handles provide O(1)
   access, O(1) validation, and safe recycling without garbage collection or reference counting
   overhead.
   - **Deps:** F-1.7.3 (Typed Pool Allocator)
2. **F-1.7.5** — Provide a slot map (dense-sparse set) that stores values in a contiguous dense
   array while exposing generational handles for stable external references. The dense array enables
   cache-friendly iteration while the sparse indirection table enables O(1) lookup by handle. Slot
   maps are the default container for ECS archetype columns and asset registries.
   - **Deps:** F-1.7.4

## Memory Budgets

| ID      | Feature                     |
|---------|------------------------------|
| F-1.7.6 | Per-Subsystem Memory Budgets|

1. **F-1.7.6** — Assign configurable memory budgets to engine subsystems (ECS storage, asset cache,
   GPU upload heaps, scratch allocators). Each subsystem draws from its budget through a quota-aware
   allocator that tracks current and peak usage. When a subsystem approaches its budget limit, it
   triggers eviction or backpressure policies rather than over-allocating. This prevents any single
   subsystem from starving others at MMO scale.
   - **Deps:** F-1.7.1, F-1.7.3
   - **Platform:** Mobile (2-6 GB): ECS 128 MB, asset cache 256 MB, GPU upload 64 MB, scratch 32 MB.
     Switch (4 GB): ECS 256 MB, asset cache 512 MB, GPU upload 128 MB, scratch 64 MB. Desktop (16+
     GB): all budgets configurable with higher defaults. High-end PC (64 GB): extended budgets for
     streaming-heavy open worlds.

## Profiling Hooks

| ID      | Feature                              |
|---------|---------------------------------------|
| F-1.7.7 | Memory Profiling and Telemetry Hooks |
| F-1.7.8 | Allocation Tagging and Categorization|

1. **F-1.7.7** — Instrument all allocators with optional profiling hooks that record allocation
   count, byte count, peak usage, and allocation call sites. Hooks are compiled out in release
   builds via conditional compilation. In development builds, telemetry is streamed to an external
   profiler or in-engine overlay, enabling developers to identify memory leaks, fragmentation, and
   budget violations across all subsystems.
   - **Deps:** F-1.7.6
2. **F-1.7.8** — Tag every allocation with a subsystem identifier and optional descriptive label.
   Tags propagate through scoped arenas and pool allocators so that child allocations inherit their
   parent's category. Aggregated per-tag statistics enable memory reports broken down by subsystem,
   asset type, or gameplay feature, which is critical for diagnosing memory growth in long-running
   MMO server processes.
   - **Deps:** F-1.7.7

## Numeric Types

| ID      | Feature                          |
|---------|-----------------------------------|
| F-1.7.9 | Arbitrary Precision Numeric Types|

1. **F-1.7.9** — Arbitrary precision integer and floating-point types for values beyond 64-bit
   range. The integer type supports 128-bit, 256-bit, or unlimited precision for cosmic distances,
   astronomical masses, and geological timescales. The float type uses significand + exponent
   representation with configurable precision and guaranteed deterministic cross-platform
   arithmetic. Conversion to/from f32/f64 for GPU operations. Formatting with unit suffixes ("2.4
   million light-years"). Used by the universe pipeline (F-3.6.60), large world coordinates
   (F-3.2.7), and sparse cosmic storage (F-3.6.63).
   - **Deps:** F-1.7.1 (Frame Arena Allocator)
   - **Platform:** Arbitrary precision is CPU-only. GPU uses camera-relative f32.

## Per-Worker-Thread Arenas

| ID       | Feature                           |
|----------|-----------------------------------|
| F-1.7.10 | Per-Worker-Thread Arena Allocation|

1. **F-1.7.10** — Provide one FrameArena per job system worker thread, indexed by the worker's
   thread-local index. Each arena is owned exclusively by its worker thread, requiring no atomics on
   the allocation hot path. Arenas reset at frame boundaries alongside the global frame arena. This
   ensures bump allocation scales linearly with worker count without contention.
   - **Deps:** F-1.7.1 (Per-Frame Arena Allocator)
   - **Platform:** Mobile: 2 worker arenas, 2 MB each. Switch: 2 worker arenas, 4 MB each. Desktop:
     (cores - 2) worker arenas, 8 MB each default. Arena size is configurable per platform tier.
