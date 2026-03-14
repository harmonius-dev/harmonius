# 1.7 — Memory Management

## Arena Allocators

### F-1.7.1 Per-Frame Arena Allocator

Provide a bump allocator that hands out memory from a pre-allocated contiguous block and resets to zero cost at
frame boundaries. Per-frame arenas are used for transient allocations such as command buffers, query results, and
scratch data. Eliminating individual deallocations reduces allocator overhead and fragmentation in hot loops
processing hundreds of thousands of entities per tick.

- **Requirements:** R-1.7.1
- **Dependencies:** None
- **Platform notes:** Uses platform-native virtual memory APIs for backing storage (VirtualAlloc on Windows,
  mmap on POSIX).

### F-1.7.2 Scoped Arena Allocator with Nested Lifetimes

Support nested arena scopes where a child scope allocates from the parent's remaining capacity and restores the
parent's watermark on drop. Scoped arenas enable temporary allocations within a system's execution that are
reclaimed without waiting for the frame boundary, reducing peak memory usage for systems with bursty allocation
patterns.

- **Requirements:** R-1.7.2
- **Dependencies:** F-1.7.1
- **Platform notes:** None

## Pool Allocators

### F-1.7.3 Typed Pool Allocator

Provide a fixed-size block pool allocator that manages objects of a single type. Free blocks are tracked via an
intrusive free list, yielding O(1) allocation and deallocation with zero fragmentation. Typed pools back ECS
component storage columns and resource containers where object lifetimes are independent and unpredictable.

- **Requirements:** R-1.7.3
- **Dependencies:** None
- **Platform notes:** None

## Resource Handles

### F-1.7.4 Generational Index Handles

Use generational indices — a packed index paired with a generation counter — as opaque handles to resources, assets,
and entities. On deallocation the generation is incremented, causing stale handles to fail validation on next
access. Generational handles provide O(1) access, O(1) validation, and safe recycling without garbage collection
or reference counting overhead.

- **Requirements:** R-1.7.4
- **Dependencies:** F-1.7.3 (Typed Pool Allocator)
- **Platform notes:** None

### F-1.7.5 Slot Map Container

Provide a slot map (dense-sparse set) that stores values in a contiguous dense array while exposing generational
handles for stable external references. The dense array enables cache-friendly iteration while the sparse
indirection table enables O(1) lookup by handle. Slot maps are the default container for ECS archetype columns and
asset registries.

- **Requirements:** R-1.7.5
- **Dependencies:** F-1.7.4
- **Platform notes:** None

## Memory Budgets

### F-1.7.6 Per-Subsystem Memory Budgets

Assign configurable memory budgets to engine subsystems (ECS storage, asset cache, GPU upload heaps, scratch
allocators). Each subsystem draws from its budget through a quota-aware allocator that tracks current and peak
usage. When a subsystem approaches its budget limit, it triggers eviction or backpressure policies rather than
over-allocating. This prevents any single subsystem from starving others at MMO scale.

- **Requirements:** R-1.7.6
- **Dependencies:** F-1.7.1, F-1.7.3
- **Platform notes:** None

## Profiling Hooks

### F-1.7.7 Memory Profiling and Telemetry Hooks

Instrument all allocators with optional profiling hooks that record allocation count, byte count, peak usage, and
allocation call sites. Hooks are compiled out in release builds via conditional compilation. In development builds,
telemetry is streamed to an external profiler or in-engine overlay, enabling developers to identify memory leaks,
fragmentation, and budget violations across all subsystems.

- **Requirements:** R-1.7.7
- **Dependencies:** F-1.7.6
- **Platform notes:** None

### F-1.7.8 Allocation Tagging and Categorization

Tag every allocation with a subsystem identifier and optional descriptive label. Tags propagate through scoped
arenas and pool allocators so that child allocations inherit their parent's category. Aggregated per-tag statistics
enable memory reports broken down by subsystem, asset type, or gameplay feature, which is critical for diagnosing
memory growth in long-running MMO server processes.

- **Requirements:** R-1.7.8
- **Dependencies:** F-1.7.7
- **Platform notes:** None
