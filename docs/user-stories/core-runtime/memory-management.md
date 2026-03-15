# Memory Management User Stories

## Arena Allocators

## US-1.7.1 Use Per-Frame Arena Allocators for Transient Data

**As an** engine developer (P-26), **I want** a bump allocator that resets at zero cost at
frame boundaries, **so that** transient allocations for command buffers, query results, and
scratch data avoid individual deallocation overhead and fragmentation in hot loops.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bump allocation from pre-allocated contiguous block | F-1.7.1 | R-1.7.1 |
| Zero-cost reset at frame boundaries | F-1.7.1 | R-1.7.1 |
| Platform-native virtual memory backing (VirtualAlloc, mmap) | F-1.7.1 | R-1.7.1 |

## US-1.7.2 Benchmark Arena Allocator vs General-Purpose Allocator

**As an** engine tester (P-27), **I want** to benchmark per-frame arena allocation throughput
against general-purpose allocation, **so that** I can quantify the speedup from eliminating
individual deallocations in hot paths.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Arena allocation throughput measured in ops/sec | F-1.7.1 | R-1.7.1 |
| Comparison with general-purpose allocator | F-1.7.1 | R-1.7.1 |
| Zero fragmentation verified after frame reset | F-1.7.1 | R-1.7.1 |

## US-1.7.3 Reclaim Temporary Allocations Before Frame End

**As an** engine developer (P-26), **I want** nested arena scopes that reclaim memory before
the frame boundary, **so that** systems with bursty allocation patterns reduce peak memory
usage by freeing temporary allocations immediately when the scope exits.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Child scope allocates from parent's remaining capacity | F-1.7.2 | R-1.7.2 |
| Parent's watermark restored on child scope drop | F-1.7.2 | R-1.7.2 |
| Peak memory reduced for bursty patterns | F-1.7.2 | R-1.7.2 |

## US-1.7.4 Verify Nested Arena Scope Correctness

**As an** engine tester (P-27), **I want** to verify that nested arena scopes correctly
restore parent watermarks and that child allocations are not accessible after scope exit,
**so that** scoped memory management is safe and correct.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Parent watermark restored exactly on child drop | F-1.7.2 | R-1.7.2 |
| Child allocation memory not reachable after scope exit | F-1.7.2 | R-1.7.2 |
| Deeply nested scopes (10+ levels) work correctly | F-1.7.2 | R-1.7.2 |

## Pool Allocators

## US-1.7.5 Allocate and Free Fixed-Size Blocks in O(1)

**As an** engine developer (P-26), **I want** a fixed-size block pool with O(1) alloc and
dealloc via an intrusive free list, **so that** ECS component columns and resource containers
have zero fragmentation and constant-time lifecycle operations.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| O(1) allocation and deallocation | F-1.7.3 | R-1.7.3 |
| Intrusive free list with zero fragmentation | F-1.7.3 | R-1.7.3 |
| Virtual memory commit-on-demand growth | F-1.7.3 | R-1.7.3 |
| Platform-appropriate size limits | F-1.7.3 | R-1.7.3 |

## US-1.7.6 Use Pool Allocators for ECS Component Storage

**As a** game developer (P-15), **I want** ECS component columns backed by typed pool
allocators, **so that** component allocation and deallocation are constant-time and do not
cause memory fragmentation over long sessions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Component columns use typed pool allocators | F-1.7.3 | R-1.7.3 |
| No fragmentation after extended spawn/despawn cycles | F-1.7.3 | R-1.7.3 |
| Pool growth transparent to callers | F-1.7.3 | R-1.7.3 |

## Resource Handles

## US-1.7.7 Reference Resources Safely via Generational Handles

**As an** engine developer (P-26), **I want** generational index handles with O(1) access,
O(1) validation, and safe recycling, **so that** stale references to despawned entities and
unloaded assets are detected without garbage collection or reference counting.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| O(1) access via packed index | F-1.7.4 | R-1.7.4 |
| O(1) validation via generation comparison | F-1.7.4 | R-1.7.4 |
| Safe recycling without GC or reference counting | F-1.7.4 | R-1.7.4 |

## US-1.7.8 Iterate Dense Data With Stable External References

**As an** engine developer (P-26), **I want** a slot map storing values in a dense array with
generational handle lookup via a sparse indirection table, **so that** I get cache-friendly
iteration for archetype columns while maintaining stable external references.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Dense array for cache-friendly iteration | F-1.7.5 | R-1.7.5 |
| Sparse indirection table for O(1) handle lookup | F-1.7.5 | R-1.7.5 |
| Generational handles remain valid across dense array compaction | F-1.7.5 | R-1.7.5 |

## US-1.7.9 Verify Slot Map Handle Stability During Compaction

**As an** engine tester (P-27), **I want** to verify that slot map handles remain valid after
dense array insertions, removals, and compactions, **so that** external references never
silently point to wrong data.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Handle lookup correct after interleaved insert/remove | F-1.7.5 | R-1.7.5 |
| Stale handle detected after slot removal | F-1.7.5 | R-1.7.5 |
| Dense iteration order stable for unchanged entries | F-1.7.5 | R-1.7.5 |

## Memory Budgets

## US-1.7.10 Enforce Per-Subsystem Memory Budgets

**As an** engine developer (P-26), **I want** configurable memory budgets per engine subsystem
with eviction or backpressure policies on limit, **so that** no single subsystem starves
others and memory usage is predictable at scale.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Budgets configurable per subsystem | F-1.7.6 | R-1.7.6 |
| Eviction or backpressure triggered at budget limit | F-1.7.6 | R-1.7.6 |
| Platform-appropriate default budgets | F-1.7.6 | R-1.7.6 |

## US-1.7.11 Configure Memory Budgets for Different Hardware

**As a** game developer (P-15), **I want** to configure memory budgets per platform target
(mobile 2-6 GB, Switch 4 GB, desktop 16+ GB), **so that** the engine runs within hardware
constraints on every supported device.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Mobile: ECS 128 MB, asset cache 256 MB, GPU 64 MB | F-1.7.6 | R-1.7.6 |
| Switch: ECS 256 MB, asset cache 512 MB, GPU 128 MB | F-1.7.6 | R-1.7.6 |
| Desktop: configurable with higher defaults | F-1.7.6 | R-1.7.6 |

## US-1.7.12 Verify Memory Budgets Prevent Over-Allocation

**As an** engine tester (P-27), **I want** to verify that subsystem memory budgets trigger
eviction or backpressure before over-allocating, **so that** no out-of-memory crashes occur
in long-running sessions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Subsystem at budget triggers eviction policy | F-1.7.6 | R-1.7.6 |
| Total memory stays within configured budget | F-1.7.6 | R-1.7.6 |
| No OOM after 24-hour sustained server operation | F-1.7.6 | R-1.7.6 |

## Profiling Hooks

## US-1.7.13 Profile Memory Usage in Development Builds

**As a** QA engineer (P-19), **I want** allocator profiling hooks recording allocation counts,
byte totals, peak usage, and call sites that compile out in release builds, **so that** I can
identify memory leaks and budget violations without impacting shipping performance.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Allocation count, byte count, peak usage recorded | F-1.7.7 | R-1.7.7 |
| Call sites captured for leak identification | F-1.7.7 | R-1.7.7 |
| Hooks compiled out in release builds | F-1.7.7 | R-1.7.7 |
| Telemetry streamable to external profiler | F-1.7.7 | R-1.7.7 |

## US-1.7.14 Tag Allocations by Subsystem for Memory Reports

**As an** engine developer (P-26), **I want** every allocation tagged with a subsystem
identifier that propagates through child allocators, **so that** I can generate per-subsystem
memory reports to diagnose growth in long-running server processes.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Subsystem identifier tag on every allocation | F-1.7.8 | R-1.7.8 |
| Tags propagate through scoped arenas and pool allocators | F-1.7.8 | R-1.7.8 |
| Per-tag statistics aggregated for reports | F-1.7.8 | R-1.7.8 |

## US-1.7.15 Verify Allocation Tags Propagate Through Child Allocators

**As an** engine tester (P-27), **I want** to verify that allocation tags correctly propagate
from parent to child allocators, **so that** memory reports accurately attribute all
allocations to their originating subsystem.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Child arena allocations tagged with parent's subsystem | F-1.7.8 | R-1.7.8 |
| Pool allocations inherit correct subsystem tag | F-1.7.8 | R-1.7.8 |
| Report totals match actual allocated bytes per subsystem | F-1.7.8 | R-1.7.8 |

## Numeric Types

## US-1.7.16 Represent Cosmic Distances With Arbitrary Precision

**As a** game developer (P-15), **I want** arbitrary precision integer and float types with
deterministic cross-platform arithmetic, **so that** I can represent cosmic distances,
astronomical masses, and geological timescales beyond the range of 64-bit types.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 128-bit, 256-bit, and unlimited precision integers | F-1.7.9 | R-1.7.9 |
| Configurable precision floats with deterministic arithmetic | F-1.7.9 | R-1.7.9 |
| Conversion to/from f32/f64 for GPU operations | F-1.7.9 | R-1.7.9 |
| Unit suffix formatting ("2.4 million light-years") | F-1.7.9 | R-1.7.9 |

## US-1.7.17 Verify Arbitrary Precision Determinism Across Platforms

**As an** engine tester (P-27), **I want** to verify that arbitrary precision arithmetic
produces identical results on all supported platforms, **so that** cosmic distance
calculations are deterministic for multiplayer synchronization.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Same result on Windows, macOS, and Linux | F-1.7.9 | R-1.7.9 |
| Same result on ARM and x86 architectures | F-1.7.9 | R-1.7.9 |
| Conversion to f32/f64 deterministic across platforms | F-1.7.9 | R-1.7.9 |

## US-1.7.18 Understand Precision Types in the Visual Editor

**As a** designer (P-5), **I want** the visual editor to display arbitrary precision values
with human-readable unit formatting, **so that** I can understand cosmic distances and
timescales when editing universe-scale properties.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Values displayed with unit suffixes (light-years, AU) | F-1.7.9 | R-1.7.9 |
| Editable in human-readable format | F-1.7.9 | R-1.7.9 |
| Precision type indicated in editor UI | F-1.7.9 | R-1.7.9 |
