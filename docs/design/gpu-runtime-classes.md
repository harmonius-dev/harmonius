# GPU Runtime Class Diagrams

Class diagrams, sequence diagrams, and type definitions for the GPU runtime module
(`harmonius::gpu_runtime`). Companion to [gpu-runtime.md](gpu-runtime.md).

---

## Contents

- [GPU Runtime Class Diagrams](#gpu-runtime-class-diagrams)
  - [Contents](#contents)
  - [1. Memory Manager](#1-memory-manager)
    - [Class Diagram](#class-diagram)
    - [Allocation Flow](#allocation-flow)
    - [Defragmentation Flow](#defragmentation-flow)
  - [2. State Tracker](#2-state-tracker)
    - [Class Diagram](#class-diagram-1)
    - [Barrier Optimization Flow](#barrier-optimization-flow)
  - [3. Work Graph Runtime](#3-work-graph-runtime)
    - [Class Diagram](#class-diagram-2)
    - [Native Execution Flow](#native-execution-flow)
    - [Emulated Execution Flow](#emulated-execution-flow)
  - [4. Feature Emulation (Compat)](#4-feature-emulation-compat)
    - [Class Diagram](#class-diagram-3)
    - [RT Dispatch Flow](#rt-dispatch-flow)
  - [5. Module Dependencies](#5-module-dependencies)

---

## 1. Memory Manager

### Class Diagram

```mermaid
classDiagram
    class Allocator {
        -gpu::Device& device_
        -array~HeapPool, 3~ pools_
        -BudgetTracker budget_
        +Allocator(gpu::Device&, span~HeapConfig~)
        +Allocate(AllocationDesc) expected~Allocation~
        +Free(Allocation) void
        +CreateTexture(TextureDesc, AllocationStrategy) expected~pair~
        +CreateBuffer(BufferDesc, AllocationStrategy) expected~pair~
        +DestroyTexture(TextureHandle, Allocation) void
        +DestroyBuffer(BufferHandle, Allocation) void
        +Budget(HeapType) BudgetInfo
        +Stats() AllocationStats
    }

    class HeapPool {
        -HeapType type_
        -uint64_t block_size_
        -uint32_t max_blocks_
        -vector~HeapBlock~ blocks_
        +Allocate(uint64_t size, uint64_t alignment) expected~SubAllocation~
        +Free(SubAllocation) void
        +Fragmentation() float
    }

    class HeapBlock {
        -gpu::HeapHandle heap_
        -BlockAllocator allocator_
        -uint64_t size_
        +Allocate(uint64_t size, uint64_t alignment) expected~Block~
        +Free(Block) void
        +FragmentationRatio() float
        +TotalFree() uint64_t
    }

    class BlockAllocator {
        -uint64_t pool_size_
        +Allocate(uint64_t size, uint64_t alignment) expected~Block~
        +Free(Block) void
        +FragmentationRatio() float
        +LargestFreeBlock() uint64_t
        +TotalFree() uint64_t
    }

    class RingAllocator {
        -gpu::Device& device_
        -gpu::BufferHandle buffer_
        -void* mapped_ptr_
        -uint64_t total_size_
        -uint32_t frame_count_
        -uint64_t current_offset_
        +RingAllocator(gpu::Device&, Config)
        +Allocate(uint64_t size, uint64_t alignment) expected~RingAllocation~
        +BeginFrame(uint64_t frame_index) void
        +UsedThisFrame() uint64_t
        +CapacityPerFrame() uint64_t
    }

    class PoolAllocator {
        -gpu::Device& device_
        -gpu::HeapHandle heap_
        -PoolDesc desc_
        -vector~bool~ slot_active_
        -uint32_t free_head_
        +PoolAllocator(gpu::Device&, PoolDesc)
        +Allocate() expected~PoolSlot~
        +Free(PoolSlot) void
        +ActiveCount() uint32_t
        +Capacity() uint32_t
        +Utilization() float
    }

    class DefragEngine {
        -Allocator& allocator_
        -gpu::Device& device_
        +NeedsDefrag(HeapType, float threshold) bool
        +PlanStep(HeapType, uint32_t max_moves) DefragStep
        +CommitStep(DefragStep) void
        +Stats(HeapType) DefragStats
    }

    class BudgetTracker {
        -gpu::Device& device_
        -array~BudgetInfo, 3~ budgets_
        +Check(HeapType, uint64_t size) bool
        +RecordAlloc(HeapType, uint64_t size) void
        +RecordFree(HeapType, uint64_t size) void
        +Refresh() void
        +Budget(HeapType) BudgetInfo
    }

    class Allocation {
        +gpu::HeapHandle heap
        +uint64_t offset
        +uint64_t size
        +HeapType heap_type
        +AllocationStrategy strategy
    }

    class AllocationDesc {
        +HeapType heap_type
        +AllocationStrategy strategy
        +uint64_t size
        +uint64_t alignment
        +string_view debug_name
    }

    class HeapConfig {
        +HeapType type
        +uint64_t block_size
        +uint32_t max_blocks
    }

    class RingAllocation {
        +gpu::BufferHandle buffer
        +uint64_t offset
        +uint64_t size
        +void* mapped
    }

    class PoolSlot {
        +uint32_t index
        +uint64_t heap_offset
    }

    class PoolDesc {
        +string_view name
        +HeapType heap_type
        +uint64_t element_size
        +uint32_t max_elements
    }

    class DefragStep {
        +vector~CopyRegion~ copies
        +vector~Allocation~ old_allocs
        +vector~Allocation~ new_allocs
    }

    class DefragStats {
        +uint32_t moves_completed
        +uint32_t moves_remaining
        +uint64_t bytes_moved
        +uint64_t bytes_freed
        +float fragmentation_before
        +float fragmentation_after
    }

    class BudgetInfo {
        +uint64_t total_budget
        +uint64_t current_usage
        +uint64_t peak_usage
        +float utilization
    }

    class AllocationStats {
        +uint64_t total_allocated
        +uint64_t total_freed
        +uint32_t active_allocations
        +uint32_t total_heaps
        +float avg_fragmentation
    }

    class HeapType {
        <<enumeration>>
        device_local
        host_visible
        host_cached
    }

    class AllocationStrategy {
        <<enumeration>>
        sub_allocate
        committed
        placed
    }

    class AllocationError {
        <<enumeration>>
        out_of_memory
        budget_exceeded
        pool_exhausted
        invalid_alignment
    }

    Allocator --> HeapPool : owns 3
    Allocator --> BudgetTracker : owns
    Allocator --> AllocationDesc : accepts
    Allocator --> Allocation : returns
    HeapPool --> HeapBlock : owns N
    HeapPool --> HeapConfig : configured by
    HeapBlock --> BlockAllocator : owns
    RingAllocator --> RingAllocation : returns
    PoolAllocator --> PoolSlot : returns
    PoolAllocator --> PoolDesc : configured by
    DefragEngine --> Allocator : references
    DefragEngine --> DefragStep : returns
    DefragEngine --> DefragStats : returns
    BudgetTracker --> BudgetInfo : returns
    Allocator --> AllocationStats : returns
    Allocation --> HeapType : uses
    Allocation --> AllocationStrategy : uses
```

### Allocation Flow

```mermaid
sequenceDiagram
    participant C as Consumer
    participant A as Allocator
    participant BT as BudgetTracker
    participant HP as HeapPool
    participant HB as HeapBlock
    participant BA as BlockAllocator
    participant GPU as gpu::Device

    C->>A: CreateTexture(desc, sub_allocate)
    A->>GPU: QueryTextureAllocationInfo(desc)
    GPU-->>A: size, alignment

    A->>BT: Check(device_local, size)
    BT-->>A: within budget

    A->>HP: Allocate(size, alignment)
    HP->>HB: Allocate(size, alignment)
    HB->>BA: Allocate(size, alignment)
    BA-->>HB: Block{offset, size}

    alt Block found in existing heap
        HB-->>HP: SubAllocation{heap, offset, size}
    else No space in existing heaps
        HP->>GPU: CreateHeap(HeapDesc)
        GPU-->>HP: HeapHandle
        HP->>HP: create new HeapBlock
        HP->>HB: Allocate(size, alignment)
        HB->>BA: Allocate(size, alignment)
        BA-->>HB: Block{offset, size}
        HB-->>HP: SubAllocation{heap, offset, size}
    end

    HP-->>A: SubAllocation
    A->>GPU: CreatePlacedTexture(heap, offset, desc)
    GPU-->>A: TextureHandle
    A->>BT: RecordAlloc(device_local, size)
    A-->>C: {TextureHandle, Allocation}
```

### Defragmentation Flow

```mermaid
sequenceDiagram
    participant App as Application
    participant DE as DefragEngine
    participant A as Allocator
    participant GPU as gpu::Device
    participant CB as gpu::CommandBuffer

    App->>DE: NeedsDefrag(device_local, 0.2)
    DE-->>App: true (fragmentation > 20%)

    App->>DE: PlanStep(device_local, 8)
    DE->>A: find movable allocations
    DE->>A: allocate new locations (compact)
    DE-->>App: DefragStep{copies, old_allocs, new_allocs}

    Note over App,CB: Record GPU copy commands
    loop For each copy in DefragStep
        App->>CB: CopyBuffer(src, dst)
    end
    App->>GPU: Submit(transfer, cmd_buf, signal_fence)

    Note over App,DE: After fence signals (GPU copies complete)
    App->>DE: CommitStep(step)
    DE->>A: Free(old_allocs)
    DE->>A: update internal tracking
```

---

## 2. State Tracker

### Class Diagram

```mermaid
classDiagram
    class TrackedCommandBuffer {
        -gpu::CommandBuffer& inner_
        -gpu::DeviceCapabilities& caps_
        -PipelineHandle bound_pipeline_
        -DescriptorHeapHandle bound_heap_
        -optional~Viewport~ current_viewport_
        -optional~ScissorRect~ current_scissor_
        -array~uint8_t, 128~ push_constant_cache_
        -vector~BarrierDesc~ pending_barriers_
        +TrackedCommandBuffer(gpu::CommandBuffer&, DeviceCapabilities&)
        +SetPipeline(PipelineHandle) void
        +BindDescriptorHeap(DescriptorHeapHandle) void
        +SetViewport(Viewport) void
        +SetScissor(ScissorRect) void
        +PushConstants(void*, uint32_t, uint32_t) void
        +Barrier(BarrierDesc) void
        +FlushBarriers() void
        +Begin() void
        +End() void
        +BeginRenderPass(RenderPassDesc) void
        +EndRenderPass() void
        +DispatchMesh(uint32_t, uint32_t, uint32_t) void
        +Dispatch(uint32_t, uint32_t, uint32_t) void
        +Inner() gpu::CommandBuffer&
    }

    class ResourceStateCache {
        -unordered_map~TextureHandle, ResourceState~ texture_states_
        -unordered_map~BufferHandle, ResourceState~ buffer_states_
        +RegisterResource(TextureHandle, ResourceState) void
        +RegisterResource(BufferHandle, ResourceState) void
        +CurrentState(TextureHandle) ResourceState
        +CurrentState(BufferHandle) ResourceState
        +Transition(TextureHandle, ResourceState) void
        +Transition(BufferHandle, ResourceState) void
        +NeedsTransition(TextureHandle, ResourceState) bool
        +NeedsTransition(BufferHandle, ResourceState) bool
        +Unregister(TextureHandle) void
        +Unregister(BufferHandle) void
    }

    class BarrierOptimizer {
        -ResourceStateCache& state_cache_
        -gpu::DeviceCapabilities& caps_
        -vector~BarrierDesc~ pending_
        -vector~BarrierDesc~ deferred_splits_
        +BarrierOptimizer(ResourceStateCache&, DeviceCapabilities&)
        +Enqueue(BarrierDesc) void
        +Flush() vector~BarrierDesc~
        +SplitBegin(BarrierDesc) void
        +SplitEnd(BarrierDesc) void
    }

    class ResourceState {
        +PipelineStage stage
        +ResourceAccess access
        +TextureLayout layout
        +QueueType owner
    }

    TrackedCommandBuffer --> BarrierOptimizer : uses
    BarrierOptimizer --> ResourceStateCache : queries
    ResourceStateCache --> ResourceState : stores
```

### Barrier Optimization Flow

```mermaid
sequenceDiagram
    participant Caller as Caller
    participant BO as BarrierOptimizer
    participant RSC as ResourceStateCache
    participant CB as gpu::CommandBuffer

    Note over Caller,CB: Enqueue barriers
    Caller->>BO: Enqueue(barrier_A: tex1 → shader_read)
    Caller->>BO: Enqueue(barrier_B: tex2 → shader_read)
    Caller->>BO: Enqueue(barrier_C: tex1 → shader_read)

    Note over Caller,CB: Flush — optimize and emit
    Caller->>BO: Flush()

    BO->>RSC: NeedsTransition(tex1, shader_read)?
    RSC-->>BO: true (currently render_target)
    BO->>RSC: NeedsTransition(tex2, shader_read)?
    RSC-->>BO: false (already shader_read)
    Note over BO: barrier_B elided — tex2 already in target state
    Note over BO: barrier_C deduplicated — same as barrier_A

    BO->>CB: Barrier([barrier_A])
    Note over BO: Only 1 barrier emitted (from 3 requested)

    BO->>RSC: Transition(tex1, shader_read)
```

---

## 3. Work Graph Runtime

### Class Diagram

```mermaid
classDiagram
    class WorkGraphExecutor {
        -gpu::Device& device_
        -bool use_native_
        -WorkGraphHandle cached_program_
        -uint64_t plan_version_
        +WorkGraphExecutor(gpu::Device&)
        +SetExecutionPlan(ExecutionPlanView) void
        +Execute(FrameData, TrackedCommandBuffer&, DeviceCapabilities&) void
        +IsNative() bool
    }

    class ExecutionPlanView {
        +span~PassNode~ passes
        +span~Dependency~ dependencies
        +span~BarrierGroup~ barriers
    }

    class PassNode {
        +uint32_t id
        +PipelineHandle pipeline
        +span~ResourceRef~ inputs
        +span~ResourceRef~ outputs
        +QueueType queue
        +bool active
    }

    class Dependency {
        +uint32_t src_pass
        +uint32_t dst_pass
    }

    class FrameData {
        +span~ResourceBinding~ resource_bindings
        +span~uint8_t~ push_constant_data
        +span~bool~ pass_activation_flags
        +uint64_t frame_index
    }

    class ResourceBinding {
        +uint32_t slot
        +gpu::TextureHandle texture
        +gpu::BufferHandle buffer
    }

    class ResourceRef {
        +uint32_t resource_id
        +ResourceAccess access
        +TextureLayout layout
    }

    class BarrierGroup {
        +uint32_t before_pass
        +span~BarrierDesc~ barriers
    }

    WorkGraphExecutor --> ExecutionPlanView : accepts
    WorkGraphExecutor --> FrameData : accepts
    ExecutionPlanView --> PassNode : contains
    ExecutionPlanView --> Dependency : contains
    ExecutionPlanView --> BarrierGroup : contains
    PassNode --> ResourceRef : has inputs/outputs
    FrameData --> ResourceBinding : contains
```

### Native Execution Flow

```mermaid
sequenceDiagram
    participant RG as Render Graph
    participant WGE as WorkGraphExecutor
    participant GPU as gpu::Device
    participant CB as gpu::CommandBuffer

    Note over RG,CB: One-time: after graph compilation
    RG->>WGE: SetExecutionPlan(plan)
    WGE->>WGE: translate plan → work graph desc
    WGE->>GPU: CreateWorkGraph(desc)
    GPU-->>WGE: WorkGraphHandle
    WGE->>WGE: cache program

    Note over RG,CB: Per-frame
    RG->>WGE: Execute(frame_data, cmd, caps)
    WGE->>WGE: write per-frame data to root input buffer
    WGE->>CB: SetWorkGraph(cached_program)
    WGE->>CB: DispatchGraph(root_input)
    Note over CB: GPU self-schedules all passes
```

### Emulated Execution Flow

```mermaid
sequenceDiagram
    participant RG as Render Graph
    participant WGE as WorkGraphExecutor
    participant TCB as TrackedCommandBuffer
    participant CB as gpu::CommandBuffer

    Note over RG,CB: Per-frame
    RG->>WGE: Execute(frame_data, tcb, caps)

    loop For each pass in topological order
        WGE->>WGE: check pass_activation_flags[i]
        alt pass active
            WGE->>TCB: FlushBarriers() [pre-pass barriers]
            WGE->>TCB: SetPipeline(pass.pipeline)
            WGE->>TCB: PushConstants(frame_data)
            WGE->>TCB: dispatch/draw/trace (pass-specific)
            WGE->>TCB: Barrier(post-pass barriers)
        end
    end

    TCB->>CB: forwarded commands (deduplicated)
```

---

## 4. Feature Emulation (Compat)

### Class Diagram

```mermaid
classDiagram
    class RayTracingAdapter {
        -gpu::DeviceCapabilities& caps_
        -memory::Allocator& allocator_
        -bool emulated_
        -unordered_map~uint64_t, PipelinePair~ pairs_
        -unordered_map~uint64_t, BufferHandle~ sbt_buffers_
        +RayTracingAdapter(DeviceCapabilities&, Allocator&)
        +RegisterPipelinePair(uint64_t id, PipelinePair) void
        +UnregisterPipelinePair(uint64_t id) void
        +BuildSbt(uint64_t pipeline_id, SbtLayout) void
        +Dispatch(uint64_t pipeline_id, TraceRaysDesc, TrackedCommandBuffer&) bool
        +IsEmulated() bool
    }

    class PipelinePair {
        +gpu::PipelineHandle rt_pipeline
        +gpu::PipelineHandle compute_fallback
    }

    class SbtLayout {
        +span~SbtRecord~ raygen_records
        +span~SbtRecord~ miss_records
        +span~SbtRecord~ hit_group_records
        +span~SbtRecord~ callable_records
        +uint32_t record_stride
    }

    class SbtRecord {
        +span~uint8_t~ local_root_args
    }

    RayTracingAdapter --> PipelinePair : stores
    RayTracingAdapter --> SbtLayout : accepts
    RayTracingAdapter --> Allocator : uses
    RayTracingAdapter --> TrackedCommandBuffer : records into
    SbtLayout --> SbtRecord : contains
```

### RT Dispatch Flow

```mermaid
sequenceDiagram
    participant Caller as Caller (Render Graph)
    participant RTA as RayTracingAdapter
    participant TCB as TrackedCommandBuffer
    participant CB as gpu::CommandBuffer

    Caller->>RTA: Dispatch(pipeline_id, trace_rays_desc, tcb)

    alt Native RT (D3D12 / Vulkan)
        RTA->>TCB: SetPipeline(rt_pipeline)
        TCB->>CB: SetPipeline(rt_pipeline)
        RTA->>CB: TraceRays(desc)
        RTA-->>Caller: false (not emulated)
    else Emulated RT (Metal)
        RTA->>TCB: SetPipeline(compute_fallback)
        TCB->>CB: SetPipeline(compute_fallback)
        RTA->>TCB: PushConstants(width, height, depth, sbt_index)
        RTA->>TCB: Dispatch(⌈w/8⌉, ⌈h/8⌉, d)
        TCB->>CB: Dispatch(⌈w/8⌉, ⌈h/8⌉, d)
        RTA-->>Caller: true (emulated)
    end
```

---

## 5. Module Dependencies

Complete dependency graph showing how the GPU runtime fits between the render graph and
GPU backend.

```mermaid
flowchart TD
    subgraph app["Application"]
        A["User Code"]
    end

    subgraph rg["harmonius::rg"]
        rg_core["rg-core"]
        rg_builder["rg-builder"]
        rg_compiler["rg-compiler"]
        rg_resource["rg-resource"]
        rg_sync["rg-sync"]
        rg_gate["rg-gate"]
        rg_exec["rg-exec"]
        rg_diag["rg-diag"]
    end

    subgraph gprt["harmonius::gpu_runtime"]
        gprt_mem["memory"]
        gprt_state["state"]
        gprt_wg["work_graph"]
        gprt_compat["compat"]
    end

    subgraph gpu["harmonius::gpu"]
        gpu_iface["interface + concepts"]
        gpu_d3d12["d3d12"]
        gpu_vulkan["vulkan"]
        gpu_metal["metal"]
    end

    A --> rg_builder
    A --> rg_exec

    rg_builder --> rg_core
    rg_compiler --> rg_core
    rg_compiler --> rg_gate
    rg_resource --> rg_core
    rg_resource --> gprt_mem
    rg_sync --> rg_core
    rg_sync --> gprt_state
    rg_gate --> rg_core
    rg_exec --> rg_compiler
    rg_exec --> rg_resource
    rg_exec --> rg_sync
    rg_exec --> gprt_wg
    rg_exec --> gprt_state
    rg_exec --> gprt_compat
    rg_diag --> rg_exec

    gprt_mem --> gpu_iface
    gprt_state --> gpu_iface
    gprt_wg --> gpu_iface
    gprt_compat --> gpu_iface

    gpu_d3d12 --> gpu_iface
    gpu_vulkan --> gpu_iface
    gpu_metal --> gpu_iface
```
