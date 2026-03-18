# R-1.7 — Memory Management Requirements

## Arena Allocators

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-1.7.1  | [F-1.7.1](../../features/core-runtime/memory-management.md) |
| R-1.7.1a | [F-1.7.1](../../features/core-runtime/memory-management.md) |
| R-1.7.2  | [F-1.7.2](../../features/core-runtime/memory-management.md) |

1. **R-1.7.1** — The engine **SHALL** provide a bump allocator backed by pre-allocated contiguous
   memory that resets at frame boundaries with zero per-deallocation cost, using platform-native
   virtual memory APIs (`VirtualAlloc` on Windows, `mmap` on POSIX) for backing storage.
   - **Rationale:** Per-frame arenas eliminate individual deallocation overhead and fragmentation in
     hot loops processing hundreds of thousands of entities per tick.
   - **Verification:** Benchmark: perform 100,000 allocations of varying sizes within a frame, reset
     the arena, and verify total allocation time is under 1 ms. Verify zero memory leaks after reset
     by asserting the watermark returns to the base address.
2. **R-1.7.1a** — The per-frame arena **SHALL** produce zero external fragmentation (bump allocation
   only). If an allocation request exceeds the remaining arena capacity, the engine **SHALL** return
   an out-of-memory error with the requested size and remaining capacity rather than invoking
   undefined behavior. The arena **SHALL** support a configurable initial capacity (default 8 MiB)
   and grow by doubling up to a configurable maximum.
   - **Rationale:** Frame arenas must be predictable; silent overflow causes memory corruption that
     is difficult to diagnose in production.
   - **Verification:** Fill an arena to 99% capacity, attempt an allocation that exceeds remaining
     space, and verify the error is returned with correct sizes. Verify zero fragmentation by
     checking that the watermark equals the sum of all allocation sizes plus alignment padding.
3. **R-1.7.2** — The engine **SHALL** support nested arena scopes where a child scope allocates from
   the parent's remaining capacity and restores the parent's watermark on drop.
   - **Rationale:** Scoped arenas reduce peak memory usage for systems with bursty allocation
     patterns by reclaiming memory before the frame boundary.
   - **Verification:** Unit test: create a parent arena with 1 MB capacity. Open a child scope,
     allocate 512 KB, drop the child scope, and verify the parent's available capacity returns to 1
     MB. Verify that allocations in the child scope are valid while the scope is alive.

## Pool Allocators

| ID      | Derived From                                                |
|---------|-------------------------------------------------------------|
| R-1.7.3 | [F-1.7.3](../../features/core-runtime/memory-management.md) |

1. **R-1.7.3** — The engine **SHALL** provide a fixed-size block pool allocator with O(1) allocation
   and O(1) deallocation using an intrusive free list, producing zero fragmentation for objects of a
   single type.
   - **Rationale:** Typed pools back ECS component storage columns and resource containers where
     object lifetimes are independent and unpredictable.
   - **Verification:** Unit test: allocate and deallocate 10,000 objects in random order. Verify all
     allocations succeed with O(1) cost (benchmark confirms constant time regardless of pool
     occupancy). Verify zero external fragmentation by checking that total pool memory equals
     block_count * block_size.

## Resource Handles

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-1.7.4  | [F-1.7.4](../../features/core-runtime/memory-management.md) |
| R-1.7.5  | [F-1.7.5](../../features/core-runtime/memory-management.md) |
| R-1.7.5a | [F-1.7.5](../../features/core-runtime/memory-management.md) |

1. **R-1.7.4** — The engine **SHALL** use generational indices (packed index + generation counter)
   as opaque handles for resources, assets, and entities, incrementing the generation on
   deallocation to invalidate stale handles with O(1) access and O(1) validation.
   - **Rationale:** Generational handles provide safe recycling without garbage collection or
     reference counting overhead.
   - **Verification:** Unit test: allocate a handle, read its value, deallocate it, allocate a new
     handle at the same index. Verify the old handle fails validation. Verify the new handle
     succeeds. Benchmark: validate 1 million handles and verify O(1) per-validation cost.
2. **R-1.7.5** — The engine **SHALL** provide a slot map (dense-sparse set) storing values in a
   contiguous dense array with generational handle-based O(1) lookup via a sparse indirection table.
   - **Rationale:** Slot maps combine cache-friendly iteration (dense array) with stable external
     references (generational handles) for ECS archetype columns and asset registries.
   - **Verification:** Unit test: insert 10,000 items, remove 5,000 at random positions, verify
     iteration over dense array visits exactly 5,000 items contiguously. Verify lookup by handle
     returns correct values for live items and errors for removed items.
3. **R-1.7.5a** — Slot maps **SHALL** support at least 4 million entries. Accessing a slot map with
   a stale generational handle **SHALL** return a typed error (not a panic) indicating the expected
   and actual generations. Insertion into a full slot map **SHALL** either grow the backing storage
   or return a capacity error, depending on configuration.
   - **Rationale:** Slot maps back entity storage and asset registries; stale handle access is a
     common bug that needs clear diagnostics rather than crashes.
   - **Verification:** Unit test: insert 4 million entries and verify all lookups succeed. Access a
     stale handle and verify the error includes generation mismatch details. Fill a fixed-capacity
     slot map and verify the capacity error is returned.

## Memory Budgets

| ID      | Derived From                                                |
|---------|-------------------------------------------------------------|
| R-1.7.6 | [F-1.7.6](../../features/core-runtime/memory-management.md) |

1. **R-1.7.6** — The engine **SHALL** assign configurable memory budgets to engine subsystems (ECS
   storage, asset cache, GPU upload heaps, scratch allocators), triggering eviction or backpressure
   policies when a subsystem approaches its budget limit rather than over-allocating.
   - **Rationale:** Per-subsystem budgets prevent any single subsystem from starving others in
     memory-constrained environments.
   - **Verification:** Integration test: set a 100 MB budget for the asset cache. Load assets until
     the budget is reached. Verify the next load triggers eviction of the least-recently-used asset
     rather than exceeding the budget. Verify current and peak usage are tracked accurately.

## Profiling Hooks

| ID      | Derived From                                                |
|---------|-------------------------------------------------------------|
| R-1.7.7 | [F-1.7.7](../../features/core-runtime/memory-management.md) |
| R-1.7.8 | [F-1.7.8](../../features/core-runtime/memory-management.md) |

1. **R-1.7.7** — The engine **SHALL** instrument all allocators with optional profiling hooks
   recording allocation count, byte count, peak usage, and call sites, compiled out in release
   builds via conditional compilation.
   - **Rationale:** Memory profiling is essential for identifying leaks, fragmentation, and budget
     violations during development.
   - **Verification:** Unit test (dev build): perform 1,000 allocations across 3 allocators. Verify
     profiling hooks report correct allocation counts, byte totals, and peak usage. Verify release
     build binary does not contain profiling instrumentation (symbol check or binary size
     comparison).
2. **R-1.7.8** — The engine **SHALL** tag every allocation with a subsystem identifier and optional
   descriptive label, propagating tags through scoped arenas and pool allocators so that child
   allocations inherit their parent's category.
   - **Rationale:** Per-tag statistics enable memory reports broken down by subsystem for diagnosing
     growth in long-running processes.
   - **Verification:** Unit test: create a tagged parent arena ("physics"), open a child scope,
     allocate within both. Query aggregated statistics and verify allocations from the child scope
     are reported under the "physics" tag. Verify per-tag byte counts sum to total allocation bytes.

## Numeric Types

| ID      | Derived From                                                |
|---------|-------------------------------------------------------------|
| R-1.7.9 | [F-1.7.9](../../features/core-runtime/memory-management.md) |

1. **R-1.7.9** — The engine **SHALL** provide arbitrary precision integers (128-bit, 256-bit,
   unlimited) and floats with deterministic cross-platform arithmetic. 128-bit fixed-point **SHALL**
   provide sub-meter precision at 4.4x10^26 meter range. Conversions to/from f32/f64 **SHALL** be
   provided.
   - **Rationale:** 64-bit types cannot represent cosmic distances with sufficient precision.
   - **Verification:** Compute distance at 10M light-years on all platforms; verify bit-identical.
     Verify sub-meter resolution at 10^26 scale.
