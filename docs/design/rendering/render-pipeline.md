# Render Pipeline Design

GPU hardware abstraction layer and render graph -- the foundational pipeline infrastructure for all
rendering subsystems.

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are defined in
> [features/](../../features/), [requirements/](../../requirements/), and
> [user-stories/](../../user-stories/).

### GPU Backend Trait and Interface (2.1)

| Feature  | Requirement |
|----------|-------------|
| F-2.1.1  | R-2.1.1     |
| F-2.1.2  | R-2.1.2     |
| F-2.1.3  | R-2.1.3     |
| F-2.1.4  | R-2.1.4     |
| F-2.1.5  | R-2.1.5     |
| F-2.1.6  | R-2.1.6     |

1. **F-2.1.1** -- GPU backend trait, static dispatch via generics
2. **F-2.1.2** -- Command buffer for graphics, compute, copy
3. **F-2.1.3** -- Unified pipeline state objects pre-validated
4. **F-2.1.4** -- Metal backend via Swift through swift-bridge
5. **F-2.1.5** -- D3D12 backend via windows-rs
6. **F-2.1.6** -- Vulkan backend via ash

### GPU Runtime (2.1 continued)

| Feature  | Requirement |
|----------|-------------|
| F-2.1.7  | R-2.1.7     |
| F-2.1.8  | R-2.1.8     |
| F-2.1.9  | R-2.1.9     |
| F-2.1.10 | R-2.1.10    |
| F-2.1.11 | R-2.1.11    |
| F-2.1.12 | R-2.1.12    |

1. **F-2.1.7** -- GPU heap sub-allocation from pre-allocated blocks
2. **F-2.1.8** -- CPU-side state tracking, redundant filter
3. **F-2.1.9** -- Barrier batching, merging, split barriers
4. **F-2.1.10** -- GPU work graph support (native + emulated)
5. **F-2.1.11** -- Cross-backend feature emulation
6. **F-2.1.12** -- GPU performance queries and profiling

### GPU Runtime Requirements (GR)

| Requirement             |
|-------------------------|
| GR-1.1 through GR-1.11 |
| GR-2.1 through GR-2.7  |
| GR-3.1 through GR-3.9  |
| GR-4.1 through GR-4.9  |

1. **GR-1.x** -- Memory: unified allocator, sub-alloc, ring, defrag, budgets, sparse
2. **GR-2.x** -- State tracking: pipeline/descriptor/dynamic/push caches, reset
3. **GR-3.x** -- Work graph: native/emulated, sync fidelity
4. **GR-4.x** -- Feature emulation: barriers, RT, mesh shaders

### Render Graph (2.2)

| Feature  | Requirement             |
|----------|-------------------------|
| F-2.2.1  | RG-1.1, RG-1.2, RG-1.3 |
| F-2.2.2  | RG-6.1 .. RG-6.7       |
| F-2.2.3  | RG-2.1, RG-2.5, RG-2.6 |
| F-2.2.4  | RG-8.1 .. RG-8.6       |
| F-2.2.5  | RG-3.1 .. RG-3.6       |
| F-2.2.6  | RG-4.1 .. RG-4.6       |
| F-2.2.7  | RG-5.1, RG-5.6, RG-5.7 |
| F-2.2.8  | RG-7.1 .. RG-7.6       |
| F-2.2.9  | RG-9.1 .. RG-9.5       |
| F-2.2.10 | RG-10.1 .. RG-10.7     |
| F-2.2.11 | RG-11.1 .. RG-11.7     |
| F-2.2.12 | RG-13.1 .. RG-13.8     |
| F-2.2.13 | RG-12.1 .. RG-12.7     |

1. **F-2.2.1** -- Declarative pass registration with typed I/O
2. **F-2.2.2** -- Capability gating and fallback chains
3. **F-2.2.3** -- Transient resource declaration
4. **F-2.2.4** -- Resource aliasing for memory reuse
5. **F-2.2.5** -- Automatic barrier insertion
6. **F-2.2.6** -- Multi-queue scheduling
7. **F-2.2.7** -- Topological sort, deterministic ordering
8. **F-2.2.8** -- Budget culling
9. **F-2.2.9** -- Multi-view execution
10. **F-2.2.10** -- Parallel command encoding
11. **F-2.2.11** -- Streaming integration
12. **F-2.2.12** -- Graph compilation and caching
13. **F-2.2.13** -- Render graph diagnostics

### Non-Functional Requirements

| NFR | Target |
|-----|--------|
| NFR-2.1.1 | Abstraction < 5% overhead vs raw API (10k draws) |
| NFR-2.1.2 | OS GPU allocs < 64/frame, O(1) sub-alloc |
| NFR-2.1.3 | State tracker >= 20% reduction, <= 64 KB/cmd buf |

## Overview

### GPU Abstraction Layer

The GPU abstraction provides a unified, type-safe interface across Metal (macOS/iOS), D3D12
(Windows), and Vulkan (Linux/Android). Two crates:

1. **`harmonius_gpu`** -- Backend trait interface. Defines `GpuBackend`, `CommandBuffer`,
   `PipelineState`, resources, swapchain, shader compilation. Static dispatch via `cfg`-gated type
   aliases eliminates vtable overhead.
2. **`harmonius_gpu_runtime`** -- Shared services: memory sub-allocation, state tracking, barrier
   optimization, descriptor binding, work graphs, feature emulation, profiling.

HLSL is the sole shader language. The `dxc` CLI compiles HLSL to DXIL and SPIR-V as a subprocess
during asset processing. The `metal-shaderconverter` CLI translates DXIL to metallib as a
subprocess. No runtime shader compilation in shipping builds.

### Render Graph

The render graph is a DAG-based frame graph modeling an entire frame's GPU work as typed passes
connected by resource dependencies. The compiler derives barriers, queue assignments, resource
lifetimes, memory aliasing, and execution order.

Key goals:

1. **Zero manual barriers** -- derived from resource declarations
2. **Minimal VRAM** -- transient resources share memory via interference-graph aliasing
3. **Multi-queue overlap** -- async compute/transfer overlap
4. **Parallel encoding** -- independent passes encode on threads
5. **Compile once, execute many** -- topology-data separation
6. **Budget-aware** -- GPU timing feedback drives pass culling

## Architecture

### GPU Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_gpu"
        BT[GpuBackend Trait]
        CB[CommandBuffer Trait]
        PS[PipelineState]
        RS[Resource Types]
        SC[Swapchain]
        SH[Shader Compiler]
    end

    subgraph "harmonius_gpu_runtime"
        MA[MemoryAllocator]
        ST[StateTracker]
        BO[BarrierOptimizer]
        WG[WorkGraphRuntime]
        FE[FeatureEmulation]
        PQ[ProfilingQueries]
        DB[DescriptorBinder]
    end

    BT --> MA
    BT --> ST
    BT --> BO
    BT --> DB
    CB --> ST
```

### Render Graph Module Boundaries

```mermaid
graph TD
    subgraph rg["harmonius_rg"]
        GB[GraphBuilder]
        GC[GraphCompiler]
        EP[ExecutionPlan]
        RM[ResourceManager]
        BA[BarrierAnalyzer]
        QS[QueueScheduler]
        AL[AliasingAllocator]
        DG[Diagnostics]
    end

    subgraph gpu["harmonius_gpu"]
        BTR[Backend Trait]
        CBR[CommandBuffer]
        TF[TimelineFence]
        MAR[MemoryAllocator]
    end

    GB --> GC
    GC --> EP
    GC --> BA
    GC --> QS
    GC --> AL
    EP --> RM
    RM --> MAR
    BA --> BTR
    QS --> TF
    EP --> CBR
```

### Shader Compilation Pipeline

```mermaid
flowchart LR
    HLSL["HLSL Source"] --> DXC["dxc CLI\n(subprocess)"]
    DXC -->|"--target dxil"| DXIL["DXIL"]
    DXC -->|"--target spirv"| SPIRV["SPIR-V"]
    DXIL --> MSC["metal-shaderconverter\nCLI (subprocess)"]
    MSC --> METALLIB["metallib"]
    DXIL -->|D3D12| D3D12P["D3D12 Pipeline"]
    SPIRV -->|Vulkan| VKP["Vulkan Pipeline"]
    METALLIB -->|Metal| MTLP["Metal Pipeline"]
```

### Graph Lifecycle

```mermaid
flowchart LR
    A[Declare Passes] --> B[Build Graph]
    B --> C[Compile]
    C --> D[Capability Gate]
    D --> E[Dead-Pass Eliminate]
    E --> F[Topological Sort]
    F --> G[Barrier Analysis]
    G --> H[Queue Assignment]
    H --> I[Resource Aliasing]
    I --> J[ExecutionPlan]
    J --> K{Topology Changed?}
    K -->|No| L[Bind Per-Frame Data]
    K -->|Yes| C
    L --> M[Parallel Encode]
    M --> N[Submit to GPU]
    N --> O[Collect Timestamps]
    O --> L
```

### Parallel Encoding and Submission

```mermaid
sequenceDiagram
    participant ML as Main Loop
    participant EP as ExecutionPlan
    participant TP as ThreadPool
    participant W1 as Worker 1
    participant W2 as Worker 2
    participant GQ as Graphics Queue
    participant AQ as Async Compute

    ML->>EP: execute(frame_data, backend, pool)
    EP->>EP: bind per-frame data

    par Parallel Encoding
        EP->>W1: encode group A (shadow)
        EP->>W2: encode group B (GBuffer)
        W1-->>EP: command buffer A
        W2-->>EP: command buffer B
    end

    EP->>GQ: submit A, B topologically
    EP->>AQ: submit async compute
    Note over AQ,GQ: Cross-queue fences sync
    GQ-->>EP: graphics fence signaled
    EP->>ML: frame complete
```

### Core Class Diagram

```mermaid
classDiagram
    class GpuBackend {
        +Device: GpuDevice
        +CommandBuffer: CommandBuffer
        +Swapchain: Swapchain
        +Fence: GpuFence
        +name() &str
        +create_device(config) Result~Device~
    }

    class GpuDevice {
        +create_buffer(desc) Result~BufferHandle~
        +create_texture(desc) Result~TextureHandle~
        +create_sampler(desc) Result~SamplerHandle~
        +create_graphics_pipeline(desc) Result
        +create_compute_pipeline(desc) Result
        +submit(queue, bufs, signal, wait) Result
        +create_fence(initial) Result~FenceHandle~
        +wait_fence(handle, value) Future
        +capabilities() DeviceCapabilities
        +memory_budget() GpuMemoryBudget
    }

    class CommandBuffer {
        +begin() Result
        +end() Result
        +begin_render_pass(desc)
        +end_render_pass()
        +bind_graphics_pipeline(handle)
        +bind_compute_pipeline(handle)
        +bind_group(index, group)
        +draw(verts, insts, first_v, first_i)
        +draw_indexed_indirect(buf, off, cnt, stride)
        +dispatch(x, y, z)
        +resource_barrier(barriers)
        +set_viewport(viewport)
    }

    class GpuHandle~T~ {
        +index u32
        +generation u32
    }

    class Format {
        R8Unorm
        Rgba8Unorm
        Rgba16Float
        Depth32Float
        Bc7Unorm
    }

    class ResourceState {
        Undefined
        ShaderRead
        ShaderWrite
        RenderTarget
        DepthStencilWrite
        Present
    }

    class QueueType {
        Graphics
        Compute
        Copy
    }

    class GpuAllocator {
        +alloc(desc) Result~GpuAllocation~
        +alloc_committed(desc) Result~GpuAllocation~
        +alloc_placed(heap, desc) Result~GpuAllocation~
        +alloc_ring(size, align) Result~RingSlice~
        +free(alloc)
        +defragment_step() bool
        +advance_frame()
        +query_budget() GpuMemoryBudget
    }

    class GpuAllocation {
        +heap_index u32
        +offset u64
        +size u64
        +heap_type HeapType
    }

    class RingBuffer {
        +alloc(size, align) Result~RingSlice~
        +advance_frame()
    }

    class HeapType {
        Local
        Upload
        Readback
    }

    class TrackedCommandBuffer~B~ {
        +begin() Result
        +end() Result
        +bind_graphics_pipeline(handle)
        +bind_group(index, group)
        +set_viewport(vp)
        +resource_state(resource) ResourceState
    }

    class BarrierOptimizer {
        +transition(resource, before, after)
        +begin_split_barrier(resource, before, after)
        +flush~B~(cmd)
        +has_pending() bool
    }

    class GpuProfiler {
        +begin_query~B~(cmd, label)
        +end_query~B~(cmd)
        +resolve_previous_frame() Vec~GpuPassTiming~
        +advance_frame()
    }

    class DeviceCapabilities {
        +backend BackendType
        +mesh_shaders bool
        +ray_tracing bool
        +work_graphs bool
        +bindless_resources bool
        +max_texture_dimension_2d u32
        +local_memory_size u64
    }

    class FeatureEmulation {
        +new(caps) Self
        +dispatch_mesh~B~(cmd, x, y, z)
        +trace_rays~B~(cmd, w, h, d)
    }

    class WorkGraphRuntime {
        +compile(desc) Result~WorkGraphHandle~
        +dispatch~B~(cmd, handle, input)
        +emit_tasks(builder, handle, feedback)
        +is_native() bool
    }

    class ShaderCompiler {
        +new() Result~Self~
        +compile(desc) Result~ShaderCompileResult~
    }

    class ShaderTarget {
        Dxil
        SpirV
        MetalLib
    }

    class BackendType {
        Metal
        D3D12
        Vulkan
    }

    GpuBackend --> GpuDevice
    GpuBackend --> CommandBuffer
    GpuDevice --> GpuHandle
    GpuAllocator *-- RingBuffer
    GpuAllocator --> GpuAllocation
    GpuAllocator --> HeapType
    TrackedCommandBuffer --> CommandBuffer
    BarrierOptimizer --> ResourceState
    GpuProfiler --> DeviceCapabilities
    FeatureEmulation --> DeviceCapabilities
    WorkGraphRuntime --> DeviceCapabilities
    ShaderCompiler --> ShaderTarget
```

### Render Graph Class Diagram

```mermaid
classDiagram
    class GraphBuilder {
        +add_raster_pass(name) PassBuilder
        +add_compute_pass(name) PassBuilder
        +add_ray_tracing_pass(name) PassBuilder
        +add_transfer_pass(name) PassBuilder
        +create_transient_texture(desc) ResourceHandle
        +create_transient_buffer(desc) ResourceHandle
        +import_texture(tex, desc) ResourceHandle
        +add_sub_graph_template(name, t) SubGraphHandle
        +add_variant_slot(name) VariantSlotBuilder
        +build() Result~RenderGraph~
    }

    class RenderGraph {
        +compile(caps, config) Result~ExecutionPlan~
        +pass_count() u32
        +resource_count() u32
    }

    class ExecutionPlan {
        +execute~B~(data, backend, pool)
        +set_pass_enabled(pass, enabled)
        +set_resolution_scale(scale)
        +update_gpu_timings(timings)
        +active_pass_count() u32
        +aliased_heap_size() u64
        +execution_order() Vec~PassHandle~
    }

    class PassDecl {
        +name &str
        +pass_type PassType
        +reads Vec~ResourceUsage~
        +writes Vec~ResourceUsage~
        +queue_affinity QueueAffinity
        +capability_gate CapabilityGate
        +priority PassPriority
        +cost_estimate f32
        +enabled bool
    }

    class PassType {
        Rasterization
        Compute
        RayTracingDispatch
        AccelerationStructureBuild
        Transfer
        Present
        HostCallback
    }

    class QueueAffinity {
        Graphics
        AsyncCompute
        Transfer
        Any
    }

    class ResourceLifetime {
        Transient
        Persistent
        Imported
        History
    }

    class PassPriority {
        Required
        High
        Normal
        Low
        Optional
    }

    class Capability {
        MeshShaders
        RayTracing
        RayQuery
        WorkGraphs
        VariableRateShading
    }

    class CapabilityGate {
        None
        RequireAll
        RequireAny
        Hard
        Soft
    }

    class ResourceHandle {
        +index u32
        +version u32
    }

    class PassHandle {
        +0 u32
    }

    class SubGraphHandle {
        +0 u32
    }

    class PassBuilder {
        +write_color(slot, resource) Self
        +write_depth(resource) Self
        +read_texture(resource) Self
        +read_storage(resource) Self
        +queue_affinity(affinity) Self
        +capability_gate(gate) Self
        +priority(priority) Self
        +execute~F~(f) Self
        +finish() PassHandle
    }

    class PassEncoder {
        +bind_graphics_pipeline(pipeline)
        +set_viewport(viewport)
        +draw(verts, insts, first_v, first_i)
        +dispatch(x, y, z)
        +resolve_texture(resource) TextureView
    }

    class RenderGraphPhase {
        +compile(device) Result
        +emit_tasks(builder, frame_data)
    }

    class CompilationConfig {
        +variant_selections Vec
        +target_budget_ms f32
        +budget_culling_enabled bool
        +resolution_scale f32
        +diagnostics_enabled bool
    }

    class RenderGraphError {
        CycleDetected
        TypeMismatch
        UndeclaredResource
        SingleWriterViolation
        CapabilityNotMet
        EmptyGraph
    }

    class GpuError {
        DeviceCreationFailed
        OutOfMemory
        ResourceCreationFailed
        PipelineCreationFailed
        InvalidHandle
        SwapchainLost
    }

    GraphBuilder ..> RenderGraph : builds
    RenderGraph ..> ExecutionPlan : compiles
    RenderGraph *-- PassDecl
    PassDecl --> PassType
    PassDecl --> QueueAffinity
    PassDecl --> CapabilityGate
    PassDecl --> PassPriority
    ExecutionPlan --> PassHandle
    GraphBuilder --> PassBuilder
    PassBuilder --> PassHandle
    PassEncoder --> ResourceHandle
    RenderGraphPhase --> ExecutionPlan
```

## API Design

### Descriptor Binding Model

| Bind Group | Frequency | Contents |
|------------|-----------|----------|
| 0 | Per-frame | Global constants, shadow maps, IBL |
| 1 | Per-pass | Render targets, pass constants |
| 2 | Per-material | Textures, material params, samplers |
| 3 | Per-draw | Transform, instance data |

### Backend Mapping

| Backend | BindGroup mapping |
|---------|-------------------|
| D3D12 | Root signature table entries |
| Vulkan | VkDescriptorSet per bind group |
| Metal | Argument buffer per bind group |

### Resource Aliasing Algorithm

1. **Lifetime interval** -- find first write, last read
2. **Interference graph** -- edges for overlapping lifetimes
3. **Graph coloring** -- greedy color assignment
4. **Heap packing** -- largest resource per color class

### Budget Culling Flow

```mermaid
flowchart TD
    A[Read GPU timings] --> B{Over budget?}
    B -->|No| C[Execute all passes]
    B -->|Yes| D[Sort by priority]
    D --> E[Cull lowest-priority]
    E --> F[Cascade-cull dependents]
    F --> G[Recalculate total]
    G --> H{Still over?}
    H -->|Yes| E
    H -->|No| I[Execute remaining]
```

## Platform Considerations

### GPU Backend Comparison

| Component | D3D12 | Vulkan | Metal |
|-----------|-------|--------|-------|
| Device | ID3D12Device | VkDevice (ash) | MTLDevice (swift-bridge) |
| Cmd buf | ID3D12GraphicsCommandList | VkCommandBuffer | MTLCommandBuffer |
| Fence | ID3D12Fence | VkSemaphore (timeline) | MTLEvent |
| Barriers | ResourceBarrier | vkCmdPipelineBarrier2 | Driver-managed |
| Descriptors | Root sig + heap | VkDescriptorSet | Argument buffers |
| Shader fmt | DXIL | SPIR-V | metallib |

### Alignment Requirements

| Resource | D3D12 | Vulkan | Metal |
|----------|-------|--------|-------|
| Constant buf | 256 B | Queried | 256 B |
| Storage buf | 16 B | Queried | 16 B |
| Texture | 64 KB | Queried | 4096 B |

### Feature Support Matrix

| Feature | D3D12 | Vulkan | Metal |
|---------|-------|--------|-------|
| Mesh shaders | FL 12.2+ | VK_EXT_mesh_shader | Apple 7+ |
| Ray tracing | DXR 1.1 | VK_KHR_ray_tracing | Apple 9+ |
| Work graphs | Native | Emulated | Emulated |
| Bindless | SM 6.6 | descriptor_indexing | Arg buf T2 |

### Queue Model

| Backend | Graphics | Compute | Transfer |
|---------|----------|---------|----------|
| D3D12 | Direct queue | Compute queue | Copy queue |
| Vulkan | Graphics family | Compute family | Transfer family |
| Metal | MTLCommandQueue | Private queue | Shared |

### Proposed Dependencies

| Crate |
|-------|
| `windows` |
| `ash` |
| `smallvec` |
| `bitflags` |

1. **`windows`** -- D3D12/DXGI bindings
2. **`ash`** -- Zero-overhead Vulkan function loader
3. **`smallvec`** -- Inline-allocated small vectors
4. **`bitflags`** -- Ergonomic bitflag operations

## Test Plan

Test cases are defined inline below.

### Unit Tests (GPU Abstraction)

| Test | Req |
|------|-----|
| `test_static_dispatch_no_vtable` | R-2.1.1, NFR-2.1.1 |
| `test_buffer_create_destroy` | R-2.1.1 |
| `test_texture_create_all_formats` | R-2.1.1 |
| `test_cmd_buf_graphics_compute_copy` | R-2.1.2 |
| `test_pso_invalid_combination` | R-2.1.3 |
| `test_metal_ffi_swift_bridge` | R-2.1.4 |
| `test_d3d12_no_cpp` | R-2.1.5 |
| `test_vulkan_validation_zero_errors` | R-2.1.6 |
| `test_suballoc_alignment_d3d12` | R-2.1.7, GR-1.2 |
| `test_suballoc_alignment_vulkan` | R-2.1.7, GR-1.2 |
| `test_suballoc_alignment_metal` | R-2.1.7, GR-1.2 |
| `test_state_tracker_redundant_bind` | R-2.1.8, GR-2.2 |
| `test_barrier_merge` | R-2.1.9 |
| `test_barrier_noop_metal` | R-2.1.9 |
| `test_split_barrier_overlap` | R-2.1.9, GR-4.2 |
| `test_work_graph_native_d3d12` | R-2.1.10, GR-3.2 |
| `test_work_graph_emulated` | R-2.1.10, GR-3.3 |
| `test_emulation_no_runtime_branch` | R-2.1.11, GR-4.1 |
| `test_timestamp_query_readback` | R-2.1.12 |
| `test_ring_buffer_zero_alloc` | GR-1.5 |
| `test_fence_async_no_spin` | constraints |

### Unit Tests (Render Graph)

| Test | Req |
|------|-----|
| `test_empty_graph_error` | RG-13.4 |
| `test_cycle_detection` | RG-5.7 |
| `test_single_writer_violation` | RG-3.5 |
| `test_topological_sort_stability` | RG-5.6 |
| `test_dead_pass_elimination` | RG-13.2 |
| `test_capability_gate_soft` | RG-6.2 |
| `test_capability_gate_hard` | RG-6.2 |
| `test_fallback_chain` | RG-6.3 |
| `test_variant_selection` | RG-13.7 |
| `test_barrier_raw` | RG-3.1 |
| `test_barrier_waw` | RG-3.2 |
| `test_cross_queue_barrier` | RG-3.4 |
| `test_aliasing_non_overlapping` | RG-8.2 |
| `test_aliasing_efficiency` | RG-8.6 |
| `test_encoding_groups` | RG-10.4 |
| `test_sub_graph_instances` | RG-9.5 |
| `test_history_resource` | RG-2.4 |
| `test_budget_cull_lowest` | RG-7.2 |
| `test_budget_never_cull_required` | RG-7.2 |
| `test_diagnostics_pass_timing` | RG-12.1 |

### Integration Tests

| Test | Req |
|------|-----|
| `test_cross_backend_image_diff` | R-2.1.1 |
| `test_10k_draws_overhead` | NFR-2.1.1 |
| `test_state_tracker_reduction` | NFR-2.1.3 |
| `test_shader_compile_all_targets` | constraints |
| `test_full_frame_graph` | RG-13.1 |
| `test_multi_view_shadow_cascades` | RG-9.1 |
| `test_parallel_encoding_correctness` | RG-10.1 |
| `test_d3d12_barrier_mapping` | RG-3.1 |
| `test_metal_no_intra_queue_barriers` | RG-3.1 |

### Benchmarks

| Benchmark | Target |
|-----------|--------|
| Abstraction overhead (10k draws) | < 5% vs raw |
| Sub-alloc latency | O(1) amortized |
| OS GPU allocs per frame | < 64 |
| State tracker reduction | >= 20% |
| Graph compilation (50 passes) | < 1 ms |
| Per-frame execution overhead | < 0.5 ms |
| Parallel encoding (8 threads) | >= 4x speedup |
| Aliasing efficiency | >= 40% saved |
| Barrier analysis (50 passes) | < 200 us |

## Open Questions

1. **Descriptor heap on D3D12** -- Monolithic shader-visible heap vs ring-allocated regions. Ring
   matches engine pattern but needs careful index management.
2. **Metal argument buffer tier** -- Require Tier 2 (Apple 6+) or provide Tier 1 fallback with
   descriptor workarounds.
3. **GPU fence reactor integration** -- Event-based (efficient) vs poll-based (simpler). Need to
   decide per-backend.
4. **Aliasing heuristic** -- Greedy graph coloring vs interval scheduling (optimal for interval
   graphs).
5. **Split barrier placement** -- Immediate after producer vs deferred to maximize overlap. Second
   backward pass needed.
6. **Work graph integration** -- Boundary between render-graph and GPU-managed scheduling needs
   definition.
