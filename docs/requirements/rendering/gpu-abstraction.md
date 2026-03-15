# GPU Runtime Requirements

> **Scope:** GPU runtime internals -- memory management, state tracking, work graph emulation,
> and feature emulation. For the backend trait interface (Metal, D3D12, Vulkan), see
> [gpu-abstraction-layer.md](gpu-abstraction-layer.md).

Requirements for the GPU runtime layer (`harmonius::gpu_runtime`), a shared services module that
sits between the GPU backend interface (`harmonius::gpu`) and consumers such as the render graph
(`harmonius::rg`) and asset pipeline (`harmonius::asset`).

**Total: 36 requirements** across 4 categories.

## GR-1: Memory Management

### GR-1.1 Unified Memory Allocator

The GPU runtime SHALL provide a unified memory allocator managing all GPU memory allocation across
all backends, built entirely on the GPU backend interface without third-party allocators.

**Derived from:** R-1.1.5, R-1.1.7

### GR-1.2 Block Sub-Allocation

O(1) allocation and deallocation with bounded external fragmentation, respecting per-resource
alignment requirements.

**Derived from:** R-3.1.8, R-3.4.1

### GR-1.3 Committed Allocation

Dedicated heap allocations for resources requiring dedicated memory.

**Derived from:** R-3.4.1

### GR-1.4 Placed Allocation for Aliasing

Placed allocations within shared heaps for render graph transient resource aliasing.

**Derived from:** RG-8.1, RG-8.2, RG-8.5

### GR-1.5 Ring Buffer Allocation

Per-frame staging and constant buffer ring allocation with zero heap allocations on the hot path.

**Derived from:** R-3.1.8, RG-8.4, RG-10.5

### GR-1.6 Online Defragmentation

Incremental GPU-side defragmentation without pipeline stalls.

**Derived from:** R-3.1.8, R-3.4.1

### GR-1.7 Memory Budget Tracking

Per-heap-type budget querying and usage tracking with error on over-commitment.

**Derived from:** R-3.4.1, RG-12.6

### GR-1.8 Heap Type Selection

Automatic memory heap type selection based on declared usage flags.

**Derived from:** RG-8.5

### GR-1.9 Allocation Metadata Query

Queryable metadata (size, alignment, heap type, offset, handle) for any active allocation.

**Derived from:** RG-8.6, RG-12.6

### GR-1.10 Pool-Scoped Allocation

Named allocation pools with configurable maximum capacity and element size class.

**Derived from:** RG-2.8, RG-11.5

### GR-1.11 Sparse Resource Binding

Virtual texture tile binding management with batch bind/unbind operations.

**Derived from:** RG-2.9

## GR-2: State Tracking

### GR-2.1 Tracked Command Buffer

Wrapper that suppresses redundant API calls by comparing against cached state.

**Derived from:** R-3.1.2, R-3.1.8

### GR-2.2 Pipeline State Cache

Cache of currently bound pipeline handles per bind point, skipping redundant binds.

**Derived from:** R-3.1.2

### GR-2.3 Descriptor Heap Cache

Cache of currently bound descriptor heap handle, skipping redundant binds.

**Derived from:** R-3.1.2

### GR-2.4 Dynamic State Cache

Cache of viewport, scissor, and blend constant state, suppressing identical calls.

**Derived from:** R-3.1.2

### GR-2.5 Push Constant Cache

Cache of most recently written push constant data, skipping identical writes.

**Derived from:** R-3.1.2

### GR-2.6 Resource State Cache

Per-resource layout/access state cache for minimum barrier computation.

**Derived from:** R-3.1.2, RG-3.1, RG-3.2

### GR-2.7 Command Buffer State Reset

All cached state reset to unknown/invalid on `Begin()`.

**Derived from:** R-3.1.2

## GR-3: Work Graph Runtime

### GR-3.1 Transparent Work Graph Execution

Automatic selection between native GPU work graphs and CPU-side emulation based on capabilities.

**Derived from:** R-2.7.13, RG-1.13

### GR-3.2 Native Work Graph Path

Render graph execution plan translated to GPU work graph program with GPU self-scheduling.

**Derived from:** R-2.7.13

### GR-3.3 CPU-Side Emulation Path

CPU-side command buffer recording and submission matching native path results.

**Derived from:** R-2.7.13

### GR-3.4 Unified Executor API

Single execution API accepting compiled plan and capabilities without manual path selection.

**Derived from:** R-1.1.7, R-5.1.2

### GR-3.5 Incremental Plan Translation

Cached GPU work graph programs re-translated only on execution plan changes.

**Derived from:** R-3.1.8

### GR-3.6 Per-Frame Data Injection

Per-frame resource bindings, constants, and activation flags injected before dispatch.

**Derived from:** RG-14.1, RG-14.2, RG-14.3

### GR-3.7 Diagnostic Instrumentation

Timestamp insertion on both native and emulated paths.

**Derived from:** RG-12.1, RG-12.7

### GR-3.8 Synchronization Fidelity

Emulated path replicating native GPU work graph synchronization guarantees via explicit barriers and
timeline fences.

**Derived from:** RG-3.1, RG-3.3, GR-3.3

### GR-3.9 Backing Memory Management

Backing memory for native work graph programs allocated from the unified allocator, reused across
frames.

**Derived from:** GR-1.1, GR-3.2, GR-3.5

## GR-4: Feature Emulation

### GR-4.1 Capability-Aware Command Recording

Automatic native/emulated path selection per operation based on device capabilities.

**Derived from:** R-1.1.5, R-1.1.7

### GR-4.2 Split Barrier Emulation

Deferred begin-barrier with combined immediate barrier at split-end on unsupported backends.

**Derived from:** RG-3.3

### GR-4.3 Barrier Batching

Multiple barriers at the same synchronization scope merged into single backend API calls.

**Derived from:** R-3.1.2, RG-3.6

### GR-4.4 Barrier Deduplication

Elision of barriers for resources already in the target state.

**Derived from:** R-3.1.2, RG-3.6

### GR-4.5 Queue Ownership Transfer Elision

Queue ownership transfer barriers elided on unified memory architectures.

**Derived from:** RG-3.4

### GR-4.6 Ray Tracing Pipeline Dispatch Emulation

`TraceRays()` translated to compute shader dispatches with inline ray queries when dedicated RT
pipelines are unavailable.

**Derived from:** R-1.1.5, R-1.1.7, R-2.5.1

### GR-4.7 Shader Binding Table Emulation

SBT replacement with flat material data buffer accessible via bindless resource indexing.

**Derived from:** GR-4.6, R-2.5.2

### GR-4.8 RT Pipeline Pair Registration

Registration and capability-based selection of native RT and compute shader fallback pipeline pairs.

**Derived from:** GR-4.1, GR-4.6, R-1.1.7

### GR-4.9 Acceleration Structure Binding Adaptation

Acceleration structure binding adaptation between RT pipeline descriptor layout and compute shader
resource views.

**Derived from:** GR-4.6, R-2.5.2
