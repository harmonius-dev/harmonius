# R-1.7 — Memory Management Requirements

## Arena Allocators

1. **R-1.7.1** — The engine **SHALL** provide a bump allocator backed by pre-allocated contiguous
   memory that resets at frame boundaries with zero per-deallocation cost, using platform-native
   virtual memory APIs (VirtualAlloc on Windows, mmap on POSIX).
   - **Rationale:** Per-frame arenas eliminate individual deallocation overhead and fragmentation in
     hot loops.
   - **Verification:** Benchmark: 100,000 allocations of varying sizes within a frame; verify under
     1 ms total. Verify watermark returns to base address after reset.
2. **R-1.7.2** — The per-frame arena **SHALL** produce zero external fragmentation. If an allocation
   exceeds remaining capacity, the engine **SHALL** return an out-of-memory error with requested
   size and remaining capacity. Initial capacity **SHALL** be configurable (default 8 MiB), growing
   by doubling up to a configurable maximum.
   - **Rationale:** Frame arenas must be predictable; silent overflow causes memory corruption.
   - **Verification:** Fill arena to 99% capacity; attempt allocation exceeding remaining space;
     verify error with correct sizes. Verify watermark equals sum of allocations plus alignment
     padding.
3. **R-1.7.3** — The engine **SHALL** support nested arena scopes where a child scope allocates from
   the parent's remaining capacity and restores the parent's watermark on drop.
   - **Rationale:** Scoped arenas reduce peak memory for systems with bursty allocation patterns.
   - **Verification:** Create 1 MB parent; open child scope; allocate 512 KB; drop child; verify
     parent capacity returns to 1 MB. Verify child allocations valid while scope alive.

## Pool Allocators

1. **R-1.7.4** — The engine **SHALL** provide a fixed-size block pool allocator with O(1) allocation
   and O(1) deallocation using an intrusive free list, producing zero fragmentation. Pool growth
   **SHALL** use virtual memory commit-on-demand.
   - **Rationale:** Typed pools back ECS component columns where object lifetimes are unpredictable.
   - **Verification:** Allocate and deallocate 10,000 objects in random order; verify O(1) cost.
     Verify total pool memory equals block_count * block_size.

## Resource Handles

1. **R-1.7.5** — The engine **SHALL** use generational indices (packed index + generation counter)
   as opaque handles, incrementing generation on deallocation to invalidate stale handles with O(1)
   access and O(1) validation.
   - **Rationale:** Generational handles provide safe recycling without garbage collection.
   - **Verification:** Allocate handle; deallocate; allocate at same index; verify old handle fails
     validation. Benchmark 1M validations at O(1) cost.
2. **R-1.7.6** — The engine **SHALL** provide a slot map (dense-sparse set) storing values in a
   contiguous dense array with generational handle-based O(1) lookup via a sparse indirection table.
   Slot maps **SHALL** support at least 4 million entries. Stale handle access **SHALL** return a
   typed error with expected/actual generations.
   - **Rationale:** Slot maps combine cache-friendly iteration with stable external references.
   - **Verification:** Insert 10,000 items; remove 5,000 at random; verify dense iteration visits
     exactly 5,000. Verify stale handle returns generation mismatch error. Insert 4M entries; verify
     all lookups succeed.

## Memory Budgets

1. **R-1.7.7** — The engine **SHALL** assign configurable memory budgets to subsystems (ECS, asset
   cache, GPU upload heaps, scratch allocators), triggering eviction or backpressure when
   approaching the limit.
   - **Rationale:** Per-subsystem budgets prevent any single subsystem from starving others.
   - **Verification:** Set 100 MB asset cache budget; load until limit; verify next load triggers
     LRU eviction. Verify current and peak usage tracked accurately.

## Profiling Hooks

1. **R-1.7.8** — The engine **SHALL** instrument all allocators with optional profiling hooks
   recording allocation count, byte count, peak usage, and call sites, compiled out in release
   builds via conditional compilation.
   - **Rationale:** Memory profiling is essential for identifying leaks and budget violations.
   - **Verification:** Dev build: 1,000 allocations across 3 allocators; verify correct counts,
     totals, and peak. Verify release binary has no profiling instrumentation.
2. **R-1.7.9** — The engine **SHALL** tag every allocation with a subsystem identifier and optional
   label, propagating tags through scoped arenas and pool allocators so child allocations inherit
   the parent category.
   - **Rationale:** Per-tag statistics enable memory reports broken down by subsystem.
   - **Verification:** Create tagged parent arena ("physics"); allocate in child scope; verify child
     allocations reported under "physics" tag. Verify per-tag byte counts sum to total.

## Numeric Types

1. **R-1.7.10** — The engine **SHALL** provide arbitrary precision integers (128-bit, 256-bit,
   unlimited) and floats with deterministic cross-platform arithmetic. 128-bit fixed-point **SHALL**
   provide sub-meter precision at 4.4e26 meter range. Conversions to/from f32/f64 **SHALL** be
   provided. Formatting with unit suffixes **SHALL** be supported.
   - **Rationale:** 64-bit types cannot represent cosmic distances with sufficient precision.
   - **Verification:** Compute distance at 10M light-years on all platforms; verify bit-identical
     results. Verify sub-meter resolution at 1e26 scale. Verify conversions to f32/f64 are
     deterministic across platforms.

## Per-Worker-Thread Arenas

1. **R-1.7.11** — The engine **SHALL** provide one FrameArena per job system worker thread, indexed
   by worker index, with no atomics on the allocation hot path.
   - **Rationale:** Per-thread arenas eliminate atomic contention on the allocation fast path,
     ensuring bump allocation scales linearly with worker count.
   - **Verification:** Allocate from per-thread arenas on 8 worker threads concurrently; verify no
     atomic instructions on hot path (inspect assembly). Verify each arena is independent and resets
     correctly at frame boundary.
