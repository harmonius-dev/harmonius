# GPU Backend -- Metal Class and Sequence Diagrams

Class diagrams for the Metal GPU backend and sequence diagrams showing Metal-specific
interactions. Companion to [gpu-backend-metal.md](gpu-backend-metal.md). See
[gpu-backend-interface-classes.md](gpu-backend-interface-classes.md) for the shared
concepts and types that these classes satisfy.

---

## Contents

- [Class Diagrams](#class-diagrams)
  - [1. Metal Backend Classes](#1-metal-backend-classes)
  - [2. Concept Satisfaction](#2-concept-satisfaction)
  - [3. Metal-Specific Types](#3-metal-specific-types)
- [Metal-Specific Details](#metal-specific-details)
  - [Metal 4 Command Model](#metal-4-command-model)
  - [Residency Management](#residency-management)
  - [Memory Management](#memory-management)
  - [Bindless Resources](#bindless-resources)
  - [Shader Function Linking](#shader-function-linking)
  - [Pipeline Caching](#pipeline-caching)
  - [No Explicit Image Layouts](#no-explicit-image-layouts)
  - [No Queue Ownership Transfers](#no-queue-ownership-transfers)
- [Sequence Diagrams](#sequence-diagrams)
  - [Metal Device Initialization](#metal-device-initialization)
  - [Metal 4 Command Recording](#metal-4-command-recording)
  - [MTLSharedEvent Multi-Queue Synchronization](#mtlsharedevent-multi-queue-synchronization)
  - [MTLStitchedLibrary Shader Linking](#mtlstitchedlibrary-shader-linking)

---

## Class Diagrams

### 1. Metal Backend Classes

`harmonius::gpu::metal` -- All four classes in the Metal backend with their complete
fields and methods. Each class satisfies its corresponding C++20 concept via
`static_assert`.

```mermaid
classDiagram
    class MetalDevice {
        -MTLDevice device_
        -MTLCommandQueue graphics_queue_
        -MTLCommandQueue compute_queue_
        -MTLCommandQueue copy_queue_
        -MTLBuffer argument_buffer_
        -MTLResidencySet residency_set_
        -MTL4Compiler compiler_
        -MTLSharedEvent graphics_event_
        -MTLSharedEvent compute_event_
        -MTLSharedEvent copy_event_
        -uint64_t next_idle_value_
        +capabilities() DeviceCapabilities
        +wait_idle() void
        +create_texture(TextureDesc) expected~TextureHandle~
        +create_buffer(BufferDesc) expected~BufferHandle~
        +create_heap(HeapDesc) expected~HeapHandle~
        +create_placed_texture(HeapHandle, uint64_t, TextureDesc) expected~TextureHandle~
        +create_placed_buffer(HeapHandle, uint64_t, BufferDesc) expected~BufferHandle~
        +create_sparse_texture(SparseTextureDesc) expected~TextureHandle~
        +create_fence(uint64_t) expected~FenceHandle~
        +create_mesh_render_pipeline(MeshRenderPipelineDesc) expected~PipelineHandle~
        +create_compute_pipeline(ComputePipelineDesc) expected~PipelineHandle~
        +create_ray_tracing_pipeline(RayTracingPipelineDesc) expected~PipelineHandle~
        +create_work_graph(WorkGraphDesc) expected~WorkGraphHandle~
        +create_descriptor_heap(DescriptorHeapDesc) expected~DescriptorHeapHandle~
        +create_swapchain(SwapchainDesc) expected~SwapchainHandle~
        +create_pipeline_cache(PipelineCacheDesc) expected~PipelineCacheHandle~
        +create_acceleration_structure(AccelerationStructureDesc) expected~AccelerationStructureHandle~
        +query_texture_allocation_info(TextureDesc) AllocationInfo
        +query_buffer_allocation_info(BufferDesc) AllocationInfo
        +submit(QueueType, cmd_bufs, signals, waits) void
        +map(BufferHandle) void_ptr
        +unmap(BufferHandle) void
        +set_name(TextureHandle, string_view) void
    }
    class MetalCommandPool {
        -MTL4CommandAllocator allocator_
        -QueueType queue_type_
        -uint32_t allocated_count_
        +allocate_command_buffer() MetalCommandBuffer
        +reset() void
        +allocated_count() uint32_t
    }
    class MetalCommandBuffer {
        -MTL4CommandBuffer cmd_
        -MTLRenderCommandEncoder active_render_encoder_
        -MTLComputeCommandEncoder active_compute_encoder_
        +begin() void
        +end() void
        +barrier(BarrierDesc) void
        +begin_render_pass(RenderPassDesc) void
        +end_render_pass() void
        +set_pipeline(PipelineHandle) void
        +dispatch_mesh(x, y, z) void
        +dispatch_mesh_indirect(buf, offset, count, stride) void
        +dispatch_mesh_indirect_count(args) void
        +dispatch(x, y, z) void
        +dispatch_indirect(buf, offset) void
        +trace_rays(TraceRaysDesc) void
        +trace_rays_indirect(buf, offset) void
        +build_acceleration_structure(BuildDesc) void
        +set_work_graph(WorkGraphHandle) void
        +dispatch_graph(DispatchGraphDesc) void
        +copy_buffer(src, src_off, dst, dst_off, size) void
        +copy_texture(src, sub, dst, sub, ext) void
        +copy_buffer_to_texture(args) void
        +copy_texture_to_buffer(args) void
        +set_viewport(Viewport) void
        +set_scissor(Scissor) void
        +push_constants(data, size, offset) void
        +bind_descriptor_heap(DescriptorHeapHandle) void
        +write_timestamp(pool, index) void
        +resolve_query_pool(pool, first, count, dst, offset) void
        +begin_debug_label(name) void
        +end_debug_label() void
        +insert_debug_label(name) void
    }
    class MetalFence {
        -MTLSharedEvent event_
        +signal_gpu(cmd value) void
        +wait_gpu(cmd value) void
        +completed_value() uint64_t
        +wait_cpu(value) void
    }
    MetalDevice --> MetalCommandPool : creates
    MetalCommandPool --> MetalCommandBuffer : allocates
    MetalDevice --> MetalFence : creates
```

### 2. Concept Satisfaction

How Metal classes satisfy the abstract `GpuDevice`, `GpuCommandBuffer`, and
`GpuCommandPool` concepts defined in
[gpu-backend-interface.md](gpu-backend-interface.md). Each satisfaction is enforced at
compile time via `static_assert`. See
[gpu-backend-interface-classes.md](gpu-backend-interface-classes.md) for the full
concept definitions.

```mermaid
classDiagram
    class GpuDevice {
        <<concept>>
        +capabilities() DeviceCapabilities
        +create_texture() TextureHandle
        +create_buffer() BufferHandle
        +create_fence() FenceHandle
        +submit() void
        +wait_idle() void
    }
    class GpuCommandBuffer {
        <<concept>>
        +begin() void
        +end() void
        +barrier() void
        +dispatch_mesh() void
        +dispatch() void
    }
    class GpuCommandPool {
        <<concept>>
        +allocate_command_buffer() CommandBuffer
        +reset() void
    }
    class MetalDevice {
        -MTLDevice device_
        -MTLResidencySet residency_set_
    }
    class MetalCommandBuffer {
        -MTL4CommandBuffer cmd_
    }
    class MetalCommandPool {
        -MTL4CommandAllocator allocator_
    }
    class MetalFence {
        -MTLSharedEvent event_
    }
    GpuDevice --> MetalDevice : satisfied by
    GpuCommandBuffer --> MetalCommandBuffer : satisfied by
    GpuCommandPool --> MetalCommandPool : satisfied by
    MetalDevice --> MetalCommandPool : creates
    MetalCommandPool --> MetalCommandBuffer : allocates
    MetalDevice --> MetalFence : creates
```

**Compile-time enforcement:**

```cpp
static_assert(GpuDevice<MetalDevice>);
static_assert(GpuCommandBuffer<MetalCommandBuffer>);
static_assert(GpuCommandPool<MetalCommandPool>);
```

### 3. Metal-Specific Types

Native Metal 4 and supporting types used internally by the Metal backend classes.
These types are not exposed through the abstract GPU interface.

```mermaid
classDiagram
    class MTL4CommandAllocator {
        +commandBuffer() MTL4CommandBuffer
        +reset() void
    }
    class MTL4CommandBuffer {
        +commitToQueue(queue) void
        +encodeSignalEvent() void
        +encodeWaitForEvent() void
    }
    class MTLResidencySet {
        +addAllocation(MTLHeap) void
        +commit() void
        +requestResidency() void
    }
    class MTLStitchedLibrary {
        +newFunctionWithName() MTLFunction
    }
    class MTLBinaryArchive {
        +addMeshRenderPipelineFunctions() void
        +serializeToURL() void
    }
    class MTL4ArgumentTable {
        +setBuffer(index) void
        +setTexture(index) void
    }
    MTL4CommandAllocator --> MTL4CommandBuffer : creates
```

---

## Metal-Specific Details

Key architectural differences between Metal and D3D12/Vulkan that affect how the Metal
backend implements the shared GPU interface.

### Metal 4 Command Model

Metal 4 introduces `MTL4CommandAllocator` and `MTL4CommandBuffer`, decoupling command
buffers from queues. This maps cleanly to the abstract `GpuCommandPool` /
`GpuCommandBuffer` split:

| Abstract Type | Metal 4 Type | Notes |
|---------------|-------------|-------|
| `GpuCommandPool` | `MTL4CommandAllocator` | Pool-backed memory; `reset()` recycles all |
| `GpuCommandBuffer` | `MTL4CommandBuffer` | Recording-ready from allocation (no begin) |
| `submit()` | `commitToQueue()` | Commit to a specific queue at submission |

Unlike D3D12 and Vulkan, Metal command buffers are immediately recording-ready after
allocation. The `begin()` method on `MetalCommandBuffer` is a no-op.

### Residency Management

Metal requires explicit residency management for resources accessed via argument
buffers. `MTLResidencySet` (Metal 4) provides batch residency control:

| Operation | Metal API | When |
|-----------|----------|------|
| Register heap | `addAllocation(MTLHeap)` | After heap creation |
| Apply changes | `commit()` | After adding/removing allocations |
| Make resident | `requestResidency()` | Before first use in command buffer |

All command buffers automatically include residency set resources once committed.

### Memory Management

Memory management (sub-allocation, defragmentation, budget tracking) is handled by
the GPU runtime layer (`harmonius::gpu_runtime::memory`). The Metal backend provides
only raw heap and resource creation primitives:

| Backend Method | Metal API Call |
|----------------|---------------|
| `create_heap()` | `newHeapWithDescriptor:` |
| `create_placed_texture()` | `newTextureWithDescriptor:offset:` on `MTLHeap` |
| `create_placed_buffer()` | `newBufferWithLength:options:offset:` on `MTLHeap` |
| `create_texture()` | `newTextureWithDescriptor:` |
| `create_buffer()` | `newBufferWithLength:options:` |

Memory budget monitoring uses `MTLDevice.currentAllocatedSize`, exposed through
the `DeviceCapabilities` struct.

### Bindless Resources

Metal supports bindless rendering through Argument Buffers Tier 2 and the Metal 4
`MTL4ArgumentTable`:

| Approach | Metal API | Use Case |
|----------|----------|----------|
| Tier 2 argument buffers | `MTLBuffer` with GPU resource IDs | Legacy bindless |
| MTL4 argument tables | `MTL4ArgumentTable` | Modern Metal 4 bindless |
| Push constants | `setBytes` (up to 4 KB per stage) | Per-draw inline data |

The `argument_buffer_` field in `MetalDevice` holds the bindless resource table. All
textures and buffers are registered in this table upon creation, enabling shader access
by index without per-draw binding changes.

### Shader Function Linking

Metal's `MTLStitchedLibrary` provides native support for composing shader functions at
runtime. Each `StitchNode` in the Harmonius stitching graph becomes an
`MTLFunctionStitchingFunctionNode`, and each `StitchEdge` becomes a connection between
node arguments.

| Stitching Step | Metal API |
|---------------|----------|
| Load functions | `MTLLibrary.newFunctionWithName` |
| Build graph | `MTLFunctionStitchingGraph` with `FunctionNode` entries |
| Create library | `MTLDevice.newLibraryWithStitchedDescriptor` |
| Specialize | `MTLFunctionConstantValues` applied to stitched function |
| Cache | `MTLBinaryArchive.addMeshRenderPipelineFunctions` |

Combined with Metal 4's `supportRenderTargetReassignment`, a stitched pipeline can be
specialized for different render target configurations without re-stitching or
recompilation.

### Pipeline Caching

`MTLBinaryArchive` stores pre-compiled pipeline binaries on disk, equivalent to
D3D12's pipeline state cache or Vulkan's `VkPipelineCache`. On subsequent launches,
the archive provides pre-compiled binaries without shader compilation:

| Operation | Metal API |
|-----------|----------|
| Create archive | `newBinaryArchiveWithDescriptor` (nil URL for fresh) |
| Add pipeline | `addMeshRenderPipelineFunctions` / `addComputePipelineFunctions` |
| Serialize | `serializeToURL` |
| Load on next launch | Set `url` on `MTLBinaryArchiveDescriptor` |
| Use at creation | Set `binaryArchives` on pipeline descriptor |

### No Explicit Image Layouts

Unlike D3D12 and Vulkan, Metal manages texture compression and layout internally. The
abstract interface's `TextureLayout` values are ignored on Metal. Barriers only need to
express stage/access dependencies via `memoryBarrierWithScope:` or
`memoryBarrierWithResources:`.

| Abstract Concept | Metal Equivalent |
|-----------------|-----------------|
| Texture layout transition | Not needed (Metal handles internally) |
| Memory barrier | `memoryBarrierWithScope:` / `memoryBarrierWithResources:` |
| Split barriers | Not supported (barriers are immediate) |

### No Queue Ownership Transfers

Apple Silicon uses a unified memory architecture. Resources are accessible from any
queue without release/acquire pairs. The abstract interface's `src_queue` / `dst_queue`
barrier fields are ignored on Metal.

| Abstract Concept | Metal Equivalent |
|-----------------|-----------------|
| Queue ownership release | Not needed (unified memory) |
| Queue ownership acquire | Not needed (unified memory) |
| Cross-queue synchronization | `MTLSharedEvent` signal/wait |

---

## Sequence Diagrams

### Metal Device Initialization

Feature detection, queue creation, memory management setup, bindless configuration,
and timeline fence creation.

```mermaid
sequenceDiagram
    participant App
    participant MD as MetalDevice
    participant MTL as MTLDevice
    App->>MD: MetalDevice(DeviceDesc)
    MD->>MTL: MTLCreateSystemDefaultDevice()
    MTL-->>MD: id MTLDevice

    Note over MD,MTL: Feature detection
    MD->>MTL: supportsFamily(Apple8)
    MTL-->>MD: mesh shader support
    MD->>MTL: supportsFamily(Apple7)
    MTL-->>MD: ray tracing support

    Note over MD,MTL: Queue creation
    MD->>MTL: newCommandQueue (graphics)
    MD->>MTL: newCommandQueue (compute)
    MD->>MTL: newCommandQueue (copy)

    Note over MD,MTL: Bindless and residency
    MD->>MTL: newBufferWithLength (argument buffer)
    MD->>MTL: newResidencySetWithDescriptor

    Note over MD,MTL: Timeline fences
    MD->>MTL: newSharedEvent (per queue x3)

    MD-->>App: MetalDevice ready
```

### Metal 4 Command Recording

Allocator lifecycle, command buffer creation, encoder transitions between render and
compute passes, and queue submission.

```mermaid
sequenceDiagram
    participant Pool as MetalCommandPool
    participant Alloc as MTL4CommandAllocator
    participant CB as MetalCommandBuffer
    participant Cmd as MTL4CommandBuffer
    participant RE as MTLRenderCommandEncoder
    participant CE as MTLComputeCommandEncoder
    participant Queue as MTLCommandQueue

    Note over Pool,Queue: Allocation
    Pool->>Alloc: commandBuffer
    Alloc-->>Pool: MTL4CommandBuffer
    Pool->>CB: MetalCommandBuffer(cmd)

    Note over CB,Queue: Recording
    CB->>CB: begin() (no-op on Metal)

    Note over CB,RE: Render pass
    CB->>Cmd: renderCommandEncoderWithDescriptor
    Cmd-->>CB: MTLRenderCommandEncoder
    CB->>RE: setRenderPipelineState(pipeline)
    CB->>RE: setObjectBytes (push constants)
    CB->>RE: drawMeshThreadgroups
    CB->>RE: endEncoding

    Note over CB,CE: Compute pass
    CB->>Cmd: computeCommandEncoder
    Cmd-->>CB: MTLComputeCommandEncoder
    CB->>CE: setComputePipelineState(pipeline)
    CB->>CE: setBytes (push constants)
    CB->>CE: dispatchThreadgroups
    CB->>CE: endEncoding

    Note over CB,Queue: Submission
    CB->>CB: end() (ends active encoder)
    CB->>Cmd: commitToQueue(graphics_queue)
```

### MTLSharedEvent Multi-Queue Synchronization

Timeline-based synchronization across graphics, compute, and copy queues using
`MTLSharedEvent`. Each queue signals a monotonically increasing value; dependent queues
wait on the required value before proceeding.

```mermaid
sequenceDiagram
    participant GFX as Graphics Queue
    participant AC as Compute Queue
    participant TX as Copy Queue
    participant SE as MTLSharedEvent
    participant CPU as Host

    Note over TX,SE: Frame N - Upload
    TX->>TX: Blit encoder (staging to device)
    TX->>SE: encodeSignalEvent(value=101)

    Note over GFX,SE: Graphics waits for upload
    GFX->>SE: encodeWaitForEvent(value=101)
    GFX->>GFX: Render encoder (mesh shader pass)
    GFX->>SE: encodeSignalEvent(value=201)

    Note over AC,SE: Async compute waits for GBuffer
    AC->>SE: encodeWaitForEvent(value=201)
    AC->>AC: Compute encoder (post-process)
    AC->>SE: encodeSignalEvent(value=301)

    Note over GFX,SE: Composite waits for post-process
    GFX->>SE: encodeWaitForEvent(value=301)
    GFX->>GFX: Render encoder (composite + present)
    GFX->>SE: encodeSignalEvent(value=401)

    Note over CPU,SE: CPU waits for frame completion
    CPU->>SE: notifyListener(value=401)
    SE-->>CPU: callback (frame complete)
```

### MTLStitchedLibrary Shader Linking

Five-step pipeline: load fragment functions from a compiled library, build a stitching
graph connecting surface evaluation, BSDF layers, and lighting, create the stitched
library, specialize with function constants, and cache in a binary archive.

```mermaid
sequenceDiagram
    participant App
    participant Lib as MTLLibrary
    participant SG as MTLFunctionStitchingGraph
    participant SL as MTLStitchedLibrary
    participant FC as MTLFunctionConstantValues
    participant BA as MTLBinaryArchive
    participant Dev as MTLDevice

    Note over App,Dev: Step 1 - Load fragment functions
    App->>Dev: newLibraryWithURL(fragment_lib)
    Dev-->>App: MTLLibrary
    App->>Lib: newFunctionWithName(pbr_standard)
    App->>Lib: newFunctionWithName(bsdf_clearcoat)
    App->>Lib: newFunctionWithName(lighting_deferred)

    Note over App,SG: Step 2 - Build stitching graph
    App->>SG: create InputNode(argumentIndex 0)
    App->>SG: create FunctionNode(pbr_standard)
    App->>SG: create FunctionNode(bsdf_clearcoat)
    App->>SG: create FunctionNode(lighting_deferred)
    App->>SG: set outputNode(lighting_deferred)

    Note over App,SL: Step 3 - Create stitched library
    App->>Dev: newLibraryWithStitchedDescriptor
    Dev-->>App: MTLStitchedLibrary

    Note over App,FC: Step 4 - Specialize with constants
    App->>FC: setConstantValue(shadow_quality)
    App->>FC: setConstantValue(quality_tier)
    App->>SL: newFunctionWithName(material_main, constants)
    SL-->>App: specialized MTLFunction

    Note over App,BA: Step 5 - Cache in binary archive
    App->>BA: addMeshRenderPipelineFunctions(desc)
    App->>BA: serializeToURL(cache_path)
```
