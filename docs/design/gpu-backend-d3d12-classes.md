# D3D12 GPU Backend Class and Sequence Diagrams

Class diagrams for the Direct3D 12 backend implementation and sequence diagrams showing D3D12-specific
interactions. Companion to [gpu-backend-d3d12.md](gpu-backend-d3d12.md). See
[gpu-backend-interface-classes.md](gpu-backend-interface-classes.md) for the shared concepts and types
(`GpuDevice`, `GpuCommandBuffer`, `GpuCommandPool`, `DeviceCapabilities`).

---

## Contents

- [Class Diagrams](#class-diagrams)
  - [1. D3D12 Backend Classes](#1-d3d12-backend-classes)
  - [2. Concept Satisfaction](#2-concept-satisfaction)
  - [3. Enhanced Barriers Mapping](#3-enhanced-barriers-mapping)
  - [4. Resource Creation via D3D12 API](#4-resource-creation-via-d3d12-api)
  - [5. Command List Type Mapping](#5-command-list-type-mapping)
- [Sequence Diagrams](#sequence-diagrams)
  - [D3D12 Device Initialization](#d3d12-device-initialization)
  - [Command Recording and Submission](#command-recording-and-submission)
  - [Work Graph Dispatch](#work-graph-dispatch)
  - [Enhanced Barrier Usage](#enhanced-barrier-usage)

---

## Class Diagrams

### 1. D3D12 Backend Classes

`harmonius::gpu::d3d12` -- All D3D12 backend classes with fields and methods.

```mermaid
classDiagram
    class D3D12Device {
        -ComPtr~IDXGIFactory7~ factory_
        -ComPtr~IDXGIAdapter4~ adapter_
        -ComPtr~ID3D12Device16~ device_
        -QueueSet queues_
        -ComPtr~ID3D12DescriptorHeap~ cbv_srv_uav_heap_
        -ComPtr~ID3D12DescriptorHeap~ sampler_heap_
        -ComPtr~ID3D12RootSignature~ global_root_signature_
        -uint32_t cbv_srv_uav_increment_
        -uint32_t sampler_increment_
        +D3D12Device(DeviceDesc)
        +Capabilities() DeviceCapabilities
        +WaitIdle() void
        +CreateTexture(TextureDesc) expected~TextureHandle~
        +CreateBuffer(BufferDesc) expected~BufferHandle~
        +CreateHeap(HeapDesc) expected~HeapHandle~
        +CreatePlacedTexture(HeapHandle, uint64_t, TextureDesc) expected~TextureHandle~
        +CreatePlacedBuffer(HeapHandle, uint64_t, BufferDesc) expected~BufferHandle~
        +CreateSparseTexture(SparseTextureDesc) expected~TextureHandle~
        +CreateFence(uint64_t) expected~FenceHandle~
        +CreateMeshRenderPipeline(MeshRenderPipelineDesc) expected~PipelineHandle~
        +CreateComputePipeline(ComputePipelineDesc) expected~PipelineHandle~
        +CreateRayTracingPipeline(RayTracingPipelineDesc) expected~PipelineHandle~
        +CreateWorkGraph(WorkGraphDesc) expected~WorkGraphHandle~
        +CreateDescriptorHeap(DescriptorHeapDesc) expected~DescriptorHeapHandle~
        +CreateSwapchain(SwapchainDesc) expected~SwapchainHandle~
        +CreatePipelineCache(PipelineCacheDesc) expected~PipelineCacheHandle~
        +CreateAccelerationStructure(AccelerationStructureDesc) expected~AccelerationStructureHandle~
        +QueryTextureAllocationInfo(TextureDesc) AllocationInfo
        +QueryBufferAllocationInfo(BufferDesc) AllocationInfo
        +ResizeSwapchain(SwapchainHandle, uint32_t, uint32_t) void
        +Submit(QueueType, cmd_bufs, signals, waits) void
        +CreateCommandPool(QueueType) D3D12CommandPool
        +Map(BufferHandle) void_ptr
        +Unmap(BufferHandle) void
    }

    class QueueSet {
        +ComPtr~ID3D12CommandQueue~ graphics
        +ComPtr~ID3D12CommandQueue~ compute
        +ComPtr~ID3D12CommandQueue~ copy
    }

    class D3D12CommandPool {
        -ComPtr~ID3D12CommandAllocator~ allocator_
        -vector~ComPtr~ID3D12GraphicsCommandList10~~ cached_lists_
        -uint32_t allocated_
        -D3D12_COMMAND_LIST_TYPE type_
        +D3D12CommandPool(ID3D12Device16*, D3D12_COMMAND_LIST_TYPE)
        +AllocateCommandBuffer() D3D12CommandBuffer
        +Reset() void
        +AllocatedCount() uint32_t
    }

    class D3D12CommandBuffer {
        -ComPtr~ID3D12GraphicsCommandList10~ list_
        -ID3D12CommandAllocator* current_allocator_
        +D3D12CommandBuffer(ComPtr~ID3D12GraphicsCommandList10~)
        +Begin() void
        +End() void
        +Barrier(BarrierDesc) void
        +BeginRenderPass(RenderPassDesc) void
        +EndRenderPass() void
        +SetPipeline(PipelineHandle) void
        +DispatchMesh(x, y, z) void
        +DispatchMeshIndirect(buf, offset, count, stride) void
        +DispatchMeshIndirectCount(args) void
        +Dispatch(x, y, z) void
        +DispatchIndirect(buf, offset) void
        +TraceRays(TraceRaysDesc) void
        +TraceRaysIndirect(buf, offset) void
        +BuildAccelerationStructure(BuildDesc) void
        +SetWorkGraph(WorkGraphHandle) void
        +DispatchGraph(DispatchGraphDesc) void
        +CopyBuffer(src, src_off, dst, dst_off, size) void
        +CopyTexture(src, sub, dst, sub, ext) void
        +CopyBufferToTexture(args) void
        +CopyTextureToBuffer(args) void
        +SetViewport(Viewport) void
        +SetScissor(Scissor) void
        +PushConstants(data, size, offset) void
        +BindDescriptorHeap(DescriptorHeapHandle) void
        +WriteTimestamp(pool, index) void
        +ResolveQueryPool(pool, first, count, dst, offset) void
        +BeginDebugLabel(name) void
        +EndDebugLabel() void
        +InsertDebugLabel(name) void
    }

    class D3D12Fence {
        -ComPtr~ID3D12Fence~ fence_
        -HANDLE event_
        +SignalGpu(ID3D12CommandQueue*, uint64_t) void
        +WaitGpu(ID3D12CommandQueue*, uint64_t) void
        +SignalCpu(uint64_t) void
        +WaitCpu(uint64_t) void
        +CompletedValue() uint64_t
    }

    D3D12Device *-- QueueSet
    D3D12Device --> D3D12CommandPool : creates
    D3D12Device --> D3D12Fence : creates
    D3D12CommandPool --> D3D12CommandBuffer : allocates
```

### 2. Concept Satisfaction

How D3D12 classes satisfy the `GpuDevice`, `GpuCommandPool`, and `GpuCommandBuffer` concepts defined
in [gpu-backend-interface-classes.md](gpu-backend-interface-classes.md). Each satisfaction is enforced
at compile time via `static_assert`.

```mermaid
classDiagram
    class GpuDevice {
        <<concept>>
        +Capabilities() DeviceCapabilities
        +CreateTexture(TextureDesc) expected~TextureHandle~
        +CreateBuffer(BufferDesc) expected~BufferHandle~
        +CreateHeap(HeapDesc) expected~HeapHandle~
        +CreateFence(uint64_t) expected~FenceHandle~
        +CreateCommandPool(QueueType) CommandPool
        +Submit(QueueType, bufs, signals, waits) void
        +WaitIdle() void
    }

    class GpuCommandPool {
        <<concept>>
        +AllocateCommandBuffer() CommandBuffer
        +Reset() void
        +AllocatedCount() uint32_t
    }

    class GpuCommandBuffer {
        <<concept>>
        +Begin() void
        +End() void
        +Barrier(BarrierDesc) void
        +DispatchMesh(x, y, z) void
        +Dispatch(x, y, z) void
        +TraceRays(TraceRaysDesc) void
        +DispatchGraph(DispatchGraphDesc) void
    }

    class D3D12Device {
        -ID3D12Device16 device_
        -ID3D12DescriptorHeap cbv_srv_uav_heap_
        -ID3D12RootSignature global_root_signature_
    }

    class D3D12CommandPool {
        -ID3D12CommandAllocator allocator_
        -vector cached_lists_
        -uint32_t allocated_
        -D3D12_COMMAND_LIST_TYPE type_
    }

    class D3D12CommandBuffer {
        -ID3D12GraphicsCommandList10 list_
        -ID3D12CommandAllocator current_allocator_
    }

    class DeviceCapabilities {
        +bool mesh_shaders
        +bool ray_tracing
        +bool work_graphs
        +bool split_barriers
        +bool sparse_textures
        +bool int64_atomics
        +bool variable_rate_shading
        +uint32_t max_push_constants_bytes
        +uint64_t device_local_memory_bytes
    }

    GpuDevice ..> D3D12Device : satisfied by
    GpuCommandPool ..> D3D12CommandPool : satisfied by
    GpuCommandBuffer ..> D3D12CommandBuffer : satisfied by
    D3D12Device --> D3D12CommandPool : creates
    D3D12CommandPool --> D3D12CommandBuffer : allocates
    D3D12Device --> DeviceCapabilities : exposes
```

**Compile-time enforcement:**

| D3D12 Class | Concept | Assertion |
|-------------|---------|-----------|
| `D3D12Device` | `GpuDevice` | `static_assert(GpuDevice<D3D12Device>)` |
| `D3D12CommandPool` | `GpuCommandPool` | `static_assert(GpuCommandPool<D3D12CommandPool>)` |
| `D3D12CommandBuffer` | `GpuCommandBuffer` | `static_assert(GpuCommandBuffer<D3D12CommandBuffer>)` |

### 3. Enhanced Barriers Mapping

How the abstract `BarrierDesc` maps to D3D12 Enhanced Barriers (`ID3D12GraphicsCommandList7::Barrier`).

```mermaid
classDiagram
    class BarrierDesc {
        +span~TextureBarrier~ texture_barriers
        +span~BufferBarrier~ buffer_barriers
        +span~GlobalBarrier~ global_barriers
    }

    class TextureBarrier {
        +PipelineStage src_stage
        +PipelineStage dst_stage
        +ResourceAccess src_access
        +ResourceAccess dst_access
        +TextureLayout old_layout
        +TextureLayout new_layout
        +TextureHandle texture
        +SubresourceRange subresource_range
        +bool discard
    }

    class D3D12_BARRIER_GROUP {
        +D3D12_BARRIER_TYPE Type
        +UINT32 NumBarriers
        +pTextureBarriers
        +pBufferBarriers
        +pGlobalBarriers
    }

    class D3D12_TEXTURE_BARRIER {
        +D3D12_BARRIER_SYNC SyncBefore
        +D3D12_BARRIER_SYNC SyncAfter
        +D3D12_BARRIER_ACCESS AccessBefore
        +D3D12_BARRIER_ACCESS AccessAfter
        +D3D12_BARRIER_LAYOUT LayoutBefore
        +D3D12_BARRIER_LAYOUT LayoutAfter
        +ID3D12Resource pResource
        +D3D12_BARRIER_SUBRESOURCE_RANGE Subresources
        +D3D12_TEXTURE_BARRIER_FLAGS Flags
    }

    class D3D12_BUFFER_BARRIER {
        +D3D12_BARRIER_SYNC SyncBefore
        +D3D12_BARRIER_SYNC SyncAfter
        +D3D12_BARRIER_ACCESS AccessBefore
        +D3D12_BARRIER_ACCESS AccessAfter
        +ID3D12Resource pResource
        +UINT64 Offset
        +UINT64 Size
    }

    class D3D12_GLOBAL_BARRIER {
        +D3D12_BARRIER_SYNC SyncBefore
        +D3D12_BARRIER_SYNC SyncAfter
        +D3D12_BARRIER_ACCESS AccessBefore
        +D3D12_BARRIER_ACCESS AccessAfter
    }

    BarrierDesc *-- TextureBarrier
    BarrierDesc --> D3D12_BARRIER_GROUP : maps to
    D3D12_BARRIER_GROUP *-- D3D12_TEXTURE_BARRIER
    D3D12_BARRIER_GROUP *-- D3D12_BUFFER_BARRIER
    D3D12_BARRIER_GROUP *-- D3D12_GLOBAL_BARRIER
    TextureBarrier --> D3D12_TEXTURE_BARRIER : to_d3d12_barrier
```

**Stage mapping (`PipelineStage` to `D3D12_BARRIER_SYNC`):**

| Abstract | D3D12 |
|----------|-------|
| `kMeshShader` | `VERTEX_SHADING` |
| `kTaskShader` | `VERTEX_SHADING` |
| `kFragmentShader` | `PIXEL_SHADING` |
| `kComputeShader` | `COMPUTE_SHADING` |
| `kRayTracingShader` | `RAYTRACING` |
| `kColorOutput` | `RENDER_TARGET` |
| `kDepthStencil` | `DEPTH_STENCIL` |
| `kTransfer` | `COPY` |
| `kAccelerationStructure` | `BUILD_RAYTRACING_ACCELERATION_STRUCTURE` |
| `all` | `ALL` |
| `SplitBegin` | Set `SyncAfter = SPLIT` |
| `SplitEnd` | Set `SyncBefore = SPLIT` |

**Layout mapping (`TextureLayout` to `D3D12_BARRIER_LAYOUT`):**

| Abstract | D3D12 |
|----------|-------|
| `undefined` | `UNDEFINED` |
| `kGeneral` | `UNORDERED_ACCESS` |
| `kColorAttachment` | `RENDER_TARGET` |
| `kDepthStencilAttachment` | `DEPTH_STENCIL_WRITE` |
| `kDepthStencilReadOnly` | `DEPTH_STENCIL_READ` |
| `kShaderReadOnly` | `SHADER_RESOURCE` |
| `kTransferSrc` | `COPY_SOURCE` |
| `kTransferDst` | `COPY_DEST` |
| `Present` | `PRESENT` |
| `kShadingRate` | `SHADING_RATE_SOURCE` |

### 4. Resource Creation via D3D12 API

How the abstract resource creation methods map to D3D12 allocation strategies. Memory
management (sub-allocation, defragmentation) is handled by the GPU runtime layer
(`harmonius::gpu_runtime::memory`); the D3D12 backend provides only the raw D3D12 API calls.

```mermaid
classDiagram
    class D3D12Device {
        +CreateTexture(TextureDesc) expected~TextureHandle~
        +CreateBuffer(BufferDesc) expected~BufferHandle~
        +CreateHeap(HeapDesc) expected~HeapHandle~
        +CreatePlacedTexture(HeapHandle, offset, TextureDesc) expected~TextureHandle~
    }

    class CommittedResource {
        <<allocation strategy>>
        +own implicit heap
        +persistent and imported resources
    }

    class PlacedResource {
        <<allocation strategy>>
        +share explicit heap
        +transient aliased resources
    }

    class ReservedResource {
        <<allocation strategy>>
        +virtual address only
        +sparse and streamed textures
    }

    class D3D12_HEAP_DESC {
        +UINT64 SizeInBytes
        +D3D12_HEAP_PROPERTIES Properties
        +D3D12_HEAP_FLAGS Flags
    }

    class ID3D12Device16 {
        +CreateCommittedResource3(desc, ...) HRESULT
        +CreatePlacedResource2(heap, offset, desc, ...) HRESULT
        +CreateReservedResource2(desc, ...) HRESULT
    }

    D3D12Device --> ID3D12Device16 : delegates to
    ID3D12Device16 --> CommittedResource : create_texture / create_buffer
    ID3D12Device16 --> PlacedResource : create_placed_texture
    ID3D12Device16 --> ReservedResource : update_sparse_bindings
    PlacedResource --> D3D12_HEAP_DESC : placed within
```

### 5. Command List Type Mapping

How `QueueType` maps to D3D12 command list types and allowed operations.

```mermaid
classDiagram
    class QueueType {
        <<enumeration>>
        graphics
        async_compute
        transfer
    }

    class D3D12_COMMAND_LIST_TYPE {
        <<enumeration>>
        DIRECT
        COMPUTE
        COPY
    }

    class DirectCapabilities {
        <<graphics queue>>
        +DispatchMesh
        +DispatchRays
        +Dispatch
        +DispatchGraph
        +CopyBufferRegion
        +CopyTextureRegion
        +BeginRenderPass
        +BuildRaytracingAccelerationStructure
    }

    class ComputeCapabilities {
        <<compute queue>>
        +Dispatch
        +CopyBufferRegion
        +CopyTextureRegion
        +UAV clear
        +ExecuteIndirect
    }

    class CopyCapabilities {
        <<copy queue>>
        +CopyBufferRegion
        +CopyTextureRegion
        +CopyResource
        +CopyTiles
    }

    QueueType --> D3D12_COMMAND_LIST_TYPE : maps to
    D3D12_COMMAND_LIST_TYPE --> DirectCapabilities : DIRECT
    D3D12_COMMAND_LIST_TYPE --> ComputeCapabilities : COMPUTE
    D3D12_COMMAND_LIST_TYPE --> CopyCapabilities : COPY
```

---

## Sequence Diagrams

### D3D12 Device Initialization

Full initialization sequence: DXGI factory, adapter enumeration, device creation, feature checks,
queue creation, descriptor heaps, root signature, and timeline fences.

```mermaid
sequenceDiagram
    participant App
    participant DXGI as DXGI Factory
    participant Dev as ID3D12Device16
    participant GFX as Graphics Queue
    participant CMP as Compute Queue
    participant CPY as Copy Queue

    Note over App,CPY: Device creation

    App->>DXGI: CreateDXGIFactory2(flags)
    DXGI-->>App: IDXGIFactory7

    App->>DXGI: EnumAdapterByGpuPreference(HIGH_PERFORMANCE)
    DXGI-->>App: IDXGIAdapter4

    App->>Dev: D3D12CreateDevice(adapter, FL 12_2)
    Dev-->>App: ID3D12Device16

    Note over App,Dev: Feature checks

    App->>Dev: CheckFeatureSupport(OPTIONS7)
    Dev-->>App: MeshShaderTier
    App->>Dev: CheckFeatureSupport(OPTIONS12)
    Dev-->>App: EnhancedBarriersSupported
    App->>Dev: CheckFeatureSupport(OPTIONS5)
    Dev-->>App: RaytracingTier
    App->>Dev: CheckFeatureSupport(OPTIONS21)
    Dev-->>App: WorkGraphsTier

    Note over App,CPY: Command queues

    App->>Dev: CreateCommandQueue(DIRECT, HIGH)
    Dev-->>GFX: ID3D12CommandQueue
    App->>Dev: CreateCommandQueue(COMPUTE, NORMAL)
    Dev-->>CMP: ID3D12CommandQueue
    App->>Dev: CreateCommandQueue(COPY, NORMAL)
    Dev-->>CPY: ID3D12CommandQueue

    Note over App,Dev: Descriptor heaps and root signature

    App->>Dev: CreateDescriptorHeap(CBV_SRV_UAV, 1M, SHADER_VISIBLE)
    Dev-->>App: cbv_srv_uav_heap_
    App->>Dev: CreateDescriptorHeap(SAMPLER, 2048, SHADER_VISIBLE)
    Dev-->>App: sampler_heap_
    App->>Dev: CreateRootSignature(bindless layout)
    Dev-->>App: global_root_signature_

    Note over App,Dev: Timeline fences

    App->>Dev: CreateFence(0, NONE)
    Dev-->>App: graphics_fence
    App->>Dev: CreateFence(0, NONE)
    Dev-->>App: compute_fence
    App->>Dev: CreateFence(0, NONE)
    Dev-->>App: copy_fence
```

### Command Recording and Submission

How `D3D12CommandPool` allocates command buffers, records commands via
`ID3D12GraphicsCommandList10`, and submits to a queue with fence synchronization.

```mermaid
sequenceDiagram
    participant Dev as D3D12Device
    participant Pool as D3D12CommandPool
    participant Cmd as D3D12CommandBuffer
    participant List as ID3D12GraphicsCommandList10
    participant Alloc as ID3D12CommandAllocator
    participant Queue as ID3D12CommandQueue
    participant Fence as D3D12Fence

    Note over Dev,Fence: Allocate and record

    Dev->>Pool: AllocateCommandBuffer()
    Pool->>Pool: check cached_lists_
    alt cached list available
        Pool->>Pool: reuse from cache
    else no cached list
        Pool->>List: CreateCommandList1(type)
    end
    Pool-->>Cmd: D3D12CommandBuffer

    Cmd->>List: Reset(allocator, nullptr)
    Note over Cmd,List: Recording state

    Cmd->>List: Barrier(barrier_groups)
    Cmd->>List: BeginRenderPass(desc)
    Cmd->>List: SetPipelineState(pso)
    Cmd->>List: SetGraphicsRootSignature(root_sig)
    Cmd->>List: SetDescriptorHeaps(heaps)
    Cmd->>List: DispatchMesh(x, y, z)
    Cmd->>List: EndRenderPass()
    Cmd->>List: Close()

    Note over Dev,Fence: Submit and synchronize

    Dev->>Queue: ExecuteCommandLists(cmd_list)
    Dev->>Queue: Signal(fence, value)
    Queue-->>Fence: GPU signals on completion

    Note over Dev,Fence: Next frame wait

    Dev->>Fence: WaitCpu(old_value)
    Dev->>Alloc: Reset()
    Dev->>Pool: Reset()
```

### Work Graph Dispatch

D3D12 work graph setup and dispatch via `ID3D12GraphicsCommandList10::SetProgram` and
`DispatchGraph`. Includes state object creation, backing memory allocation, and dispatch.

```mermaid
sequenceDiagram
    participant App
    participant Dev as D3D12Device
    participant SO as ID3D12StateObject
    participant WGP as ID3D12WorkGraphProperties
    participant Cmd as D3D12CommandBuffer
    participant List as ID3D12GraphicsCommandList10

    Note over App,List: State object creation

    App->>Dev: CreateWorkGraph(WorkGraphDesc)
    Dev->>SO: CreateStateObject(EXECUTABLE, DXIL lib + work graph subobject)
    SO-->>Dev: ID3D12StateObject

    Dev->>SO: QueryInterface(ID3D12WorkGraphProperties)
    SO-->>WGP: ID3D12WorkGraphProperties

    WGP->>WGP: GetWorkGraphIndex(program_name)
    WGP->>WGP: GetWorkGraphMemoryRequirements(index)
    WGP-->>Dev: MinSizeInBytes, MaxSizeInBytes

    Note over Dev,List: Backing memory (allocated via gpu_runtime::memory)

    Note over Cmd,List: Dispatch

    Cmd->>List: SetProgram(WORK_GRAPH, program_id, INITIALIZE, backing_memory)
    Cmd->>List: SetComputeRootSignature(global_root_signature)
    Cmd->>List: SetDescriptorHeaps(heaps)

    Cmd->>List: DispatchGraph(NODE_CPU_INPUT, entry_node, input_data)
    Note over List: GPU executes node graph with dynamic work generation
```

### Enhanced Barrier Usage

How the render graph's `BarrierDesc` is translated to D3D12 Enhanced Barriers, including split
barriers for overlapped transitions.

```mermaid
sequenceDiagram
    participant RG as Render Graph
    participant Cmd as D3D12CommandBuffer
    participant List as ID3D12GraphicsCommandList10

    Note over RG,List: Standard barrier

    RG->>Cmd: Barrier(texture: color_attachment to shader_read)
    Cmd->>Cmd: to_d3d12_sync(color_output) = RENDER_TARGET
    Cmd->>Cmd: to_d3d12_sync(fragment_shader) = PIXEL_SHADING
    Cmd->>Cmd: to_d3d12_layout(color_attachment) = RENDER_TARGET
    Cmd->>Cmd: to_d3d12_layout(shader_read_only) = SHADER_RESOURCE
    Cmd->>List: Barrier(D3D12_TEXTURE_BARRIER)

    Note over RG,List: Split barrier Begin (after producing pass)

    RG->>Cmd: Barrier(split_begin: depth to shader_read)
    Cmd->>Cmd: SyncBefore = DEPTH_STENCIL
    Cmd->>Cmd: SyncAfter = SPLIT
    Cmd->>List: Barrier(D3D12_TEXTURE_BARRIER)

    Note over RG,List: Intervening work executes on GPU

    RG->>Cmd: Dispatch(compute_pass)

    Note over RG,List: Split barrier End (before consuming pass)

    RG->>Cmd: Barrier(split_end: depth to shader_read)
    Cmd->>Cmd: SyncBefore = SPLIT
    Cmd->>Cmd: SyncAfter = PIXEL_SHADING
    Cmd->>Cmd: LayoutBefore = DEPTH_STENCIL_WRITE
    Cmd->>Cmd: LayoutAfter = SHADER_RESOURCE
    Cmd->>List: Barrier(D3D12_TEXTURE_BARRIER)

    Note over RG,List: Aliasing Barrier (transient resource activation)

    RG->>Cmd: Barrier(aliasing: activate placed resource)
    Cmd->>Cmd: AccessBefore = NO_ACCESS
    Cmd->>Cmd: LayoutBefore = UNDEFINED
    Cmd->>Cmd: Flags = DISCARD
    Cmd->>List: Barrier(D3D12_TEXTURE_BARRIER)
```
