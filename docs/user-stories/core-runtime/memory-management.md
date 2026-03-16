# Memory Management User Stories

## Arena Allocators

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.7.1 | engine developer (P-26) | a bump allocator that resets at zero cost at frame boundaries, so that transient allocations for command buffers, query results, and scratch data avoid individual deallocation overhead and fragmentation in hot loops | Bump allocation from pre-allocated contiguous block<br>Zero-cost reset at frame boundaries<br>Platform-native virtual memory backing (VirtualAlloc, mmap) | F-1.7.1 | R-1.7.1 |
| US-1.7.2 | engine tester (P-27) | to benchmark per-frame arena allocation throughput against general-purpose allocation, so that I can quantify the speedup from eliminating individual deallocations in hot paths | Arena allocation throughput measured in ops/sec<br>Comparison with general-purpose allocator<br>Zero fragmentation verified after frame reset | F-1.7.1 | R-1.7.1 |
| US-1.7.3 | engine developer (P-26) | nested arena scopes that reclaim memory before the frame boundary, so that systems with bursty allocation patterns reduce peak memory usage by freeing temporary allocations immediately when the scope exits | Child scope allocates from parent's remaining capacity<br>Parent's watermark restored on child scope drop<br>Peak memory reduced for bursty patterns | F-1.7.2 | R-1.7.2 |
| US-1.7.4 | engine tester (P-27) | to verify that nested arena scopes correctly restore parent watermarks and that child allocations are not accessible after scope exit, so that scoped memory management is safe and correct | Parent watermark restored exactly on child drop<br>Child allocation memory not reachable after scope exit<br>Deeply nested scopes (10+ levels) work correctly | F-1.7.2 | R-1.7.2 |

## Pool Allocators

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.7.5 | engine developer (P-26) | a fixed-size block pool with O(1) alloc and dealloc via an intrusive free list, so that ECS component columns and resource containers have zero fragmentation and constant-time lifecycle operations | O(1) allocation and deallocation<br>Intrusive free list with zero fragmentation<br>Virtual memory commit-on-demand growth<br>Platform-appropriate size limits | F-1.7.3 | R-1.7.3 |
| US-1.7.6 | game developer (P-15) | ECS component columns backed by typed pool allocators, so that component allocation and deallocation are constant-time and do not cause memory fragmentation over long sessions | Component columns use typed pool allocators<br>No fragmentation after extended spawn/despawn cycles<br>Pool growth transparent to callers | F-1.7.3 | R-1.7.3 |

## Resource Handles

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.7.7 | engine developer (P-26) | generational index handles with O(1) access, O(1) validation, and safe recycling, so that stale references to despawned entities and unloaded assets are detected without garbage collection or reference counting | O(1) access via packed index<br>O(1) validation via generation comparison<br>Safe recycling without GC or reference counting | F-1.7.4 | R-1.7.4 |
| US-1.7.8 | engine developer (P-26) | a slot map storing values in a dense array with generational handle lookup via a sparse indirection table, so that I get cache-friendly iteration for archetype columns while maintaining stable external references | Dense array for cache-friendly iteration<br>Sparse indirection table for O(1) handle lookup<br>Generational handles remain valid across dense array compaction | F-1.7.5 | R-1.7.5 |
| US-1.7.9 | engine tester (P-27) | to verify that slot map handles remain valid after dense array insertions, removals, and compactions, so that external references never silently point to wrong data | Handle lookup correct after interleaved insert/remove<br>Stale handle detected after slot removal<br>Dense iteration order stable for unchanged entries | F-1.7.5 | R-1.7.5 |

## Memory Budgets

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.7.10 | engine developer (P-26) | configurable memory budgets per engine subsystem with eviction or backpressure policies on limit, so that no single subsystem starves others and memory usage is predictable at scale | Budgets configurable per subsystem<br>Eviction or backpressure triggered at budget limit<br>Platform-appropriate default budgets | F-1.7.6 | R-1.7.6 |
| US-1.7.11 | game developer (P-15) | to configure memory budgets per platform target (mobile 2-6 GB, Switch 4 GB, desktop 16+ GB), so that the engine runs within hardware constraints on every supported device | Mobile: ECS 128 MB, asset cache 256 MB, GPU 64 MB<br>Switch: ECS 256 MB, asset cache 512 MB, GPU 128 MB<br>Desktop: configurable with higher defaults | F-1.7.6 | R-1.7.6 |
| US-1.7.12 | engine tester (P-27) | to verify that subsystem memory budgets trigger eviction or backpressure before over-allocating, so that no out-of-memory crashes occur in long-running sessions | Subsystem at budget triggers eviction policy<br>Total memory stays within configured budget<br>No OOM after 24-hour sustained server operation | F-1.7.6 | R-1.7.6 |

## Profiling Hooks

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.7.13 | QA engineer (P-19) | allocator profiling hooks recording allocation counts, byte totals, peak usage, and call sites that compile out in release builds, so that I can identify memory leaks and budget violations without impacting shipping performance | Allocation count, byte count, peak usage recorded<br>Call sites captured for leak identification<br>Hooks compiled out in release builds<br>Telemetry streamable to external profiler | F-1.7.7 | R-1.7.7 |
| US-1.7.14 | engine developer (P-26) | every allocation tagged with a subsystem identifier that propagates through child allocators, so that I can generate per-subsystem memory reports to diagnose growth in long-running server processes | Subsystem identifier tag on every allocation<br>Tags propagate through scoped arenas and pool allocators<br>Per-tag statistics aggregated for reports | F-1.7.8 | R-1.7.8 |
| US-1.7.15 | engine tester (P-27) | to verify that allocation tags correctly propagate from parent to child allocators, so that memory reports accurately attribute all allocations to their originating subsystem | Child arena allocations tagged with parent's subsystem<br>Pool allocations inherit correct subsystem tag<br>Report totals match actual allocated bytes per subsystem | F-1.7.8 | R-1.7.8 |

## Numeric Types

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-1.7.16 | game developer (P-15) | arbitrary precision integer and float types with deterministic cross-platform arithmetic, so that I can represent cosmic distances, astronomical masses, and geological timescales beyond the range of 64-bit types | 128-bit, 256-bit, and unlimited precision integers<br>Configurable precision floats with deterministic arithmetic<br>Conversion to/from f32/f64 for GPU operations<br>Unit suffix formatting ("2.4 million light-years") | F-1.7.9 | R-1.7.9 |
| US-1.7.17 | engine tester (P-27) | to verify that arbitrary precision arithmetic produces identical results on all supported platforms, so that cosmic distance calculations are deterministic for multiplayer synchronization | Same result on Windows, macOS, and Linux<br>Same result on ARM and x86 architectures<br>Conversion to f32/f64 deterministic across platforms | F-1.7.9 | R-1.7.9 |
| US-1.7.18 | designer (P-5) | the visual editor to display arbitrary precision values with human-readable unit formatting, so that I can understand cosmic distances and timescales when editing universe-scale properties | Values displayed with unit suffixes (light-years, AU)<br>Editable in human-readable format<br>Precision type indicated in editor UI | F-1.7.9 | R-1.7.9 |
