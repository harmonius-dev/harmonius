# GPU Runtime Requirements

> **Scope:** GPU runtime internals -- memory management, state tracking, work graph emulation, and
> feature emulation. For the backend trait interface (Metal, D3D12, Vulkan), see
> [gpu-abstraction-layer.md](gpu-abstraction-layer.md).

Requirements for the GPU runtime layer (`harmonius::gpu_runtime`), a shared services module that
sits between the GPU backend interface (`harmonius::gpu`) and consumers such as the render graph
(`harmonius::rg`) and asset pipeline (`harmonius::asset`).

**Total: 36 requirements** across 4 categories.

## GR-1: Memory Management

| ID | Requirement | Derived From |
|----|-------------|--------------|
| GR-1.1 | The GPU runtime SHALL provide a unified memory allocator managing all GPU memory allocation across all backends, built entirely on the GPU backend interface without third-party allocators. | R-1.1.5, R-1.1.7 |
| GR-1.2 | O(1) allocation and deallocation with bounded external fragmentation, respecting per-resource alignment requirements. | R-3.1.8, R-3.4.1 |
| GR-1.3 | Dedicated heap allocations for resources requiring dedicated memory. | R-3.4.1 |
| GR-1.4 | Placed allocations within shared heaps for render graph transient resource aliasing. | RG-8.1, RG-8.2, RG-8.5 |
| GR-1.5 | Per-frame staging and constant buffer ring allocation with zero heap allocations on the hot path. | R-3.1.8, RG-8.4, RG-10.5 |
| GR-1.6 | Incremental GPU-side defragmentation without pipeline stalls. | R-3.1.8, R-3.4.1 |
| GR-1.7 | Per-heap-type budget querying and usage tracking with error on over-commitment. | R-3.4.1, RG-12.6 |
| GR-1.8 | Automatic memory heap type selection based on declared usage flags. | RG-8.5 |
| GR-1.9 | Queryable metadata (size, alignment, heap type, offset, handle) for any active allocation. | RG-8.6, RG-12.6 |
| GR-1.10 | Named allocation pools with configurable maximum capacity and element size class. | RG-2.8, RG-11.5 |
| GR-1.11 | Virtual texture tile binding management with batch bind/unbind operations. | RG-2.9 |

## GR-2: State Tracking

| ID | Requirement | Derived From |
|----|-------------|--------------|
| GR-2.1 | Wrapper that suppresses redundant API calls by comparing against cached state. | R-3.1.2, R-3.1.8 |
| GR-2.2 | Cache of currently bound pipeline handles per bind point, skipping redundant binds. | R-3.1.2 |
| GR-2.3 | Cache of currently bound descriptor heap handle, skipping redundant binds. | R-3.1.2 |
| GR-2.4 | Cache of viewport, scissor, and blend constant state, suppressing identical calls. | R-3.1.2 |
| GR-2.5 | Cache of most recently written push constant data, skipping identical writes. | R-3.1.2 |
| GR-2.6 | Per-resource layout/access state cache for minimum barrier computation. | R-3.1.2, RG-3.1, RG-3.2 |
| GR-2.7 | All cached state reset to unknown/invalid on `Begin()`. | R-3.1.2 |

## GR-3: Work Graph Runtime

| ID | Requirement | Derived From |
|----|-------------|--------------|
| GR-3.1 | Automatic selection between native GPU work graphs and CPU-side emulation based on capabilities. | R-2.7.13, RG-1.13 |
| GR-3.2 | Render graph execution plan translated to GPU work graph program with GPU self-scheduling. | R-2.7.13 |
| GR-3.3 | CPU-side command buffer recording and submission matching native path results. | R-2.7.13 |
| GR-3.4 | Single execution API accepting compiled plan and capabilities without manual path selection. | R-1.1.7, R-5.1.2 |
| GR-3.5 | Cached GPU work graph programs re-translated only on execution plan changes. | R-3.1.8 |
| GR-3.6 | Per-frame resource bindings, constants, and activation flags injected before dispatch. | RG-14.1, RG-14.2, RG-14.3 |
| GR-3.7 | Timestamp insertion on both native and emulated paths. | RG-12.1, RG-12.7 |
| GR-3.8 | Emulated path replicating native GPU work graph synchronization guarantees via explicit barriers and timeline fences. | RG-3.1, RG-3.3, GR-3.3 |
| GR-3.9 | Backing memory for native work graph programs allocated from the unified allocator, reused across frames. | GR-1.1, GR-3.2, GR-3.5 |

## GR-4: Feature Emulation

| ID | Requirement | Derived From |
|----|-------------|--------------|
| GR-4.1 | Automatic native/emulated path selection per operation based on device capabilities. | R-1.1.5, R-1.1.7 |
| GR-4.2 | Deferred begin-barrier with combined immediate barrier at split-end on unsupported backends. | RG-3.3 |
| GR-4.3 | Multiple barriers at the same synchronization scope merged into single backend API calls. | R-3.1.2, RG-3.6 |
| GR-4.4 | Elision of barriers for resources already in the target state. | R-3.1.2, RG-3.6 |
| GR-4.5 | Queue ownership transfer barriers elided on unified memory architectures. | RG-3.4 |
| GR-4.6 | `TraceRays()` translated to compute shader dispatches with inline ray queries when dedicated RT pipelines are unavailable. | R-1.1.5, R-1.1.7, R-2.5.1 |
| GR-4.7 | SBT replacement with flat material data buffer accessible via bindless resource indexing. | GR-4.6, R-2.5.2 |
| GR-4.8 | Registration and capability-based selection of native RT and compute shader fallback pipeline pairs. | GR-4.1, GR-4.6, R-1.1.7 |
| GR-4.9 | Acceleration structure binding adaptation between RT pipeline descriptor layout and compute shader resource views. | GR-4.6, R-2.5.2 |
