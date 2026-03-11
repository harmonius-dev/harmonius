# Render Graph Class and Sequence Diagrams

Class diagrams for each module and sequence diagrams showing inter-module interactions.
Companion to [render-graph-design.md](render-graph-design.md).

---

## Contents

- [Module Class Diagrams](#module-class-diagrams)
  - [1. Core Types](#1-core-types)
  - [2. Graph Builder](#2-graph-builder)
  - [3. Graph Compiler](#3-graph-compiler)
  - [4. Resource System](#4-resource-system)
  - [5. Synchronization Engine](#5-synchronization-engine)
  - [6. Gating System](#6-gating-system)
  - [7. Execution Engine](#7-execution-engine)
  - [8. Diagnostics](#8-diagnostics)
  - [9. GPU Runtime](#9-gpu-runtime)
- [Cross-Module Relationships](#cross-module-relationships)
- [Sequence Diagrams](#sequence-diagrams)
  - [Full Lifecycle](#full-lifecycle)
  - [Compilation Pipeline](#compilation-pipeline)
  - [Per-Frame Execution](#per-frame-execution)
  - [Parallel Encoding](#parallel-encoding)
  - [Streaming Fault Resolution](#streaming-fault-resolution)

---

## Module Class Diagrams

### 1. Core Types

`harmonius::rg` — Shared vocabulary types with no business logic.

```mermaid
classDiagram
    class PassHandle["PassHandle «enumeration»"] {
        kInvalid
    }
    class ResourceHandle["ResourceHandle «enumeration»"] {
        kInvalid
    }
    class SubGraphHandle["SubGraphHandle «enumeration»"] {
        kInvalid
    }
    class GateHandle["GateHandle «enumeration»"] {
        kInvalid
    }
    class ChainHandle["ChainHandle «enumeration»"] {
        kInvalid
    }
    class VariantSlotHandle["VariantSlotHandle «enumeration»"] {
        kInvalid
    }
    class PassType["PassType «enumeration»"] {
        kRasterization
        kCompute
        kRayTracingDispatch
        kAccelerationStructureBuild
        kTransfer
        kMsaaResolve
        kPresent
        kHostCallback
        kWorkGraph
        kCheckerboardResolve
        kOpacityMicromapBuild
    }
    class AccessMode["AccessMode «enumeration»"] {
        kRead
        kWrite
        kReadWrite
    }
    class UsageType["UsageType «enumeration»"] {
        kColorAttachment
        kDepthAttachment
        kShaderRead
        kStorageRead
        kStorageWrite
        kShadingRateAttachment
        kIndirectArgument
        kAccelerationStructureRead
        kAccelerationStructureBuildWrite
        kTransferSrc
        kTransferDst
        kPresent
    }
    class QueueAffinity["QueueAffinity «enumeration»"] {
        kGraphics
        kAsyncCompute
        kTransfer
        kAny
    }
    class ResourceCategory["ResourceCategory «enumeration»"] {
        kTransient
        kPersistent
        kImported
        kHistory
        kMultiFrameHistory
        kSparse
        kPoolBacked
        kStaging
        kAtlas
        kAccelerationStructure
    }
    class ResourceBinding["ResourceBinding «struct»"] {
        +ResourceHandle resource
        +AccessMode access
        +UsageType usage
        +uint32_t array_layer
        +uint32_t mip_level
        +bool is_history
    }
    class ValidationErrorKind["ValidationErrorKind «enumeration»"] {
        kCycleDetected
        kTypeMismatch
        kUndeclaredResource
        kQueueIncompatibility
        kSingleWriterViolation
        kVariantAmbiguity
        kInstanceCountMismatch
        kHardGateUnsatisfied
        kSampleCountMismatch
    }
    class ValidationError["ValidationError «struct»"] {
        +ValidationErrorKind kind
        +PassHandle pass
        +ResourceHandle resource
        +string message
    }
    class CompileError["CompileError «struct»"] {
        +vector~ValidationError~ errors
    }

    ResourceBinding --> ResourceHandle
    ResourceBinding --> AccessMode
    ResourceBinding --> UsageType
    ValidationError --> ValidationErrorKind
    ValidationError --> PassHandle
    ValidationError --> ResourceHandle
    CompileError --> ValidationError
```

### 2. Graph Builder

`harmonius::rg::builder` — Declarative graph construction.

```mermaid
classDiagram
    class GraphBuilder {
        +GraphBuilder(CapabilityDescriptor caps)
        +AddPass(PassDescriptor) PassHandle
        +BeginChain(string_view) ChainHandle
        +AddChainStep(ChainHandle, PassDescriptor) PassHandle
        +EndChain(ChainHandle) void
        +DeclareVariantSlot(string_view) VariantSlotHandle
        +AddVariant(VariantSlotHandle, string_view, PassDescriptor) PassHandle
        +DeclareSubgraphTemplate(string_view, SubGraphDescriptor) SubGraphHandle
        +InstantiateSubgraph(SubGraphHandle, uint32_t, span) void
        +DeclareTransient(TransientResourceDesc) ResourceHandle
        +DeclarePersistent(PersistentResourceDesc) ResourceHandle
        +DeclareImported(ImportedResourceDesc) ResourceHandle
        +DeclareHistory(HistoryResourceDesc) ResourceHandle
        +DeclareMultiFrameHistory(MultiFrameHistoryDesc) ResourceHandle
        +DeclareSparse(SparseResourceDesc) ResourceHandle
        +DeclarePool(PoolResourceDesc) ResourceHandle
        +DeclareStaging(StagingBufferDesc) ResourceHandle
        +DeclareAtlas(AtlasResourceDesc) ResourceHandle
        +DeclareAccelerationStructure(AccelStructDesc) ResourceHandle
        +AttachCapabilityGate(PassHandle, CapabilityGateDesc) GateHandle
        +AttachBudgetGate(PassHandle, BudgetGateDesc) GateHandle
        +DeclareFallbackChain(string_view, span) GateHandle
        +AddOrderingEdge(PassHandle, PassHandle) void
        +AddFrameDependency(PassHandle, PassHandle, uint32_t) void
        +RemoveChainStep(ChainHandle, PassHandle) void
        +DeclareRingBuffer(RingBufferDesc) ResourceHandle
        +DeclareBindlessHeap(BindlessHeapDesc) ResourceHandle
        +AttachPathConditionedGate(PassHandle, PathConditionedGateDesc) GateHandle
        +AttachTimestampQuery(PassHandle, string_view) void
        +AttachStatisticsQuery(PassHandle, string_view) void
        +MarkDebugOverlay(PassHandle) void
        +Build() expected~DeclaredGraph, CompileError~
    }
    class DeclaredGraph {
        +Passes() span~PassDescriptor~
        +PassCount() uint32_t
        +ResourceCount() uint32_t
        -Impl impl_
    }
    class PassDescriptor {
        +string_view name
        +PassType type
        +QueueAffinity queue
        +vector~ResourceBinding~ inputs
        +vector~ResourceBinding~ outputs
        +bool conditional
        +optional~RenderArea~ render_area
        +vector~string_view~ tags
        +float estimated_cost_ms
        +uint32_t priority
        +optional~bool~ frame_parity
        +move_only_function execute
    }
    class SubGraphDescriptor {
        +string_view name
        +uint32_t max_instances
        +vector~PassDescriptor~ passes
        +vector~SubGraphParamSlot~ param_slots
    }
    class SubGraphBindings {
        +vector~ResourceHandle~ exclusive_resources
        +vector~ResourceHandle~ shared_resources
        +uint32_t target_array_layer
        +unordered_map variant_selections
    }
    class TransientResourceDesc {
        +string_view name
        +Format format
        +uint32_t width
        +uint32_t height
        +uint32_t depth
        +uint32_t mip_levels
        +uint32_t array_layers
        +SampleCount samples
        +optional~string_view~ resolution_param
    }
    class PersistentResourceDesc {
        +string_view name
        +Format format
        +uint32_t width
        +uint32_t height
        +uint32_t depth
        +uint32_t array_layers
        +optional~ActiveExtentDesc~ active_extent
    }
    class HistoryResourceDesc {
        +string_view name
        +Format format
        +uint32_t width
        +uint32_t height
        +optional~string_view~ resolution_param
    }
    class MultiFrameHistoryDesc {
        +string_view name
        +Format format
        +uint32_t width
        +uint32_t height
        +uint32_t history_depth
        +optional~string_view~ resolution_param
    }
    class ImportedResourceDesc {
        +string_view name
        +gpu_ResourceHandle external_handle
        +AccessMode initial_access
        +UsageType initial_usage
    }
    class SparseResourceDesc {
        +string_view name
        +Format format
        +uint32_t width
        +uint32_t height
        +uint32_t tile_width
        +uint32_t tile_height
    }
    class PoolResourceDesc {
        +string_view name
        +Format format
        +uint32_t element_width
        +uint32_t element_height
        +uint32_t max_elements
        +move_only_function eviction_callback
    }
    class StagingBufferDesc {
        +string_view name
        +uint64_t size_bytes
    }
    class AtlasResourceDesc {
        +string_view name
        +Format format
        +uint32_t width
        +uint32_t height
        +uint32_t tile_size
    }
    class AccelStructDesc {
        +string_view name
        +ResourceCategory category
        +bool has_opacity_micromap
    }
    class RingBufferDesc {
        +string_view name
        +uint64_t slot_size_bytes
        +uint32_t slot_count
    }
    class BindlessHeapDesc {
        +string_view name
        +uint32_t max_descriptors
        +uint32_t max_samplers
    }
    class ActiveExtentDesc {
        +uint32_t max_layers
        +uint32_t max_width
        +uint32_t max_height
    }
    class SubGraphParamSlot {
        +string_view name
        +bool is_shared
    }
    class RenderArea {
        +uint32_t x
        +uint32_t y
        +uint32_t width
        +uint32_t height
        +bool is_per_frame_binding
    }

    GraphBuilder --> DeclaredGraph : produces
    GraphBuilder --> PassDescriptor : accepts
    GraphBuilder --> SubGraphDescriptor : accepts
    GraphBuilder --> TransientResourceDesc : accepts
    GraphBuilder --> PersistentResourceDesc : accepts
    GraphBuilder --> ImportedResourceDesc : accepts
    GraphBuilder --> HistoryResourceDesc : accepts
    GraphBuilder --> MultiFrameHistoryDesc : accepts
    GraphBuilder --> SparseResourceDesc : accepts
    GraphBuilder --> PoolResourceDesc : accepts
    GraphBuilder --> StagingBufferDesc : accepts
    GraphBuilder --> AtlasResourceDesc : accepts
    GraphBuilder --> AccelStructDesc : accepts
    GraphBuilder --> RingBufferDesc : accepts
    GraphBuilder --> BindlessHeapDesc : accepts
    DeclaredGraph *-- PassDescriptor
    PassDescriptor --> ResourceBinding
    PassDescriptor --> RenderArea
    SubGraphDescriptor *-- PassDescriptor
    SubGraphDescriptor *-- SubGraphParamSlot
    PersistentResourceDesc --> ActiveExtentDesc
    AccelStructDesc --> ResourceCategory
```

### 3. Graph Compiler

`harmonius::rg::compiler` — Nine-stage DAG optimization pipeline.

```mermaid
classDiagram
    class GraphCompiler {
        +Compile(DeclaredGraph, CapabilityDescriptor, CompileOptions) expected~ExecutionPlan, CompileError~
        +RecompileResidency(ExecutionPlan, span~ResidencyChange~) expected~ExecutionPlan, CompileError~
        -Validate(DeclaredGraph) vector~ValidationError~
        -EvaluateGates(DeclaredGraph, CapabilityDescriptor) vector~PassHandle~
        -EliminateDeadPasses(DeclaredGraph) void
        -TopologicalSort() void
        -ComputeBarriers() BarrierSchedule
        -ComputeAliasing() AliasingMap
        -PartitionQueues() void
        -PlanEncoding() void
        -MergeInstances() void
    }
    class ExecutionPlan {
        +Passes() span~ScheduledPass~
        +QueueSubmissions() span~QueueSubmission~
        +EncodingGroups() span~EncodingGroup~
        +AliasingMap() AliasingMap
        +FencePoints() span~FenceCoordination~
        +TransferInjectionIndex() uint32_t
        +ActivePassCount() uint32_t
        +ConditionalPasses() span~PassHandle~
        +ResolutionParams() span~ResolutionParam~
        -Impl impl_
    }
    class ScheduledPass {
        +PassHandle handle
        +uint32_t execution_order
        +QueueAffinity queue
        +span~BarrierDesc~ pre_barriers
        +span~BarrierDesc~ post_barriers
        +uint32_t encoding_group
        +bool is_conditional
    }
    class EncodingGroup {
        +uint32_t group_id
        +vector~PassHandle~ passes
        +bool parallel
    }
    class QueueSubmission {
        +QueueAffinity queue
        +vector~PassHandle~ passes
        +vector~FenceCoordination~ fence_ops
    }
    class CompileOptions {
        +unordered_map variant_selections
        +bool enable_timestamp_queries
        +bool enable_statistics_queries
        +bool enable_debug_overlays
    }
    class FenceCoordination {
        +QueueAffinity source_queue
        +QueueAffinity dest_queue
        +uint64_t signal_value
        +uint64_t wait_value
    }
    class ResolutionParam {
        +string_view name
        +float default_scale
        +float min_scale
        +float max_scale
    }

    GraphCompiler --> ExecutionPlan : produces
    GraphCompiler --> DeclaredGraph : consumes
    GraphCompiler --> CompileOptions : reads
    ExecutionPlan *-- ScheduledPass
    ExecutionPlan *-- EncodingGroup
    ExecutionPlan *-- QueueSubmission
    ExecutionPlan *-- FenceCoordination
    ExecutionPlan *-- ResolutionParam
    ExecutionPlan --> AliasingMap
    QueueSubmission *-- FenceCoordination
```

### 4. Resource System

`harmonius::rg::resource` — Lifetime analysis, aliasing, pools, ring buffers.

```mermaid
classDiagram
    class AliasingSolver {
        +Solve(span~LifetimeInterval~, span~ResourceSizeInfo~) AliasingMap
    }
    class AliasingMap {
        +Assignments() span~AliasingAssignment~
        +PeakMemoryBytes() uint64_t
        +TotalLogicalBytes() uint64_t
        +AliasingEfficiency() float
    }
    class PoolAllocator {
        +PoolAllocator(PoolResourceDesc, Device)
        +Allocate() expected~ResourceHandle, PoolError~
        +Release(ResourceHandle) void
        +Utilization() float
        +Capacity() uint32_t
        +ActiveCount() uint32_t
    }
    class RingAllocator {
        +RingAllocator(uint64_t slot_size, uint32_t frame_count, Device)
        +Allocate(uint64_t size, uint64_t alignment) optional~Allocation~
        +AdvanceFrame(uint32_t frame_index) void
        +Buffer() gpu_ResourceHandle
    }
    class LifetimeInterval {
        +ResourceHandle resource
        +uint32_t first_write
        +uint32_t last_read
    }
    class AliasingAssignment {
        +ResourceHandle resource
        +uint32_t heap_offset
        +uint32_t heap_size
        +uint32_t heap_index
    }
    class ResourceSizeInfo {
        +ResourceHandle resource
        +uint64_t size_bytes
        +HeapType heap_type
    }
    class ResidencyEntry {
        +uint32_t tile_x
        +uint32_t tile_y
        +uint8_t resident
    }
    class ResidencyChange {
        +ResourceHandle resource
        +uint32_t tile_x
        +uint32_t tile_y
        +bool now_resident
    }

    class AtlasAllocator {
        +AtlasAllocator(AtlasResourceDesc, Device)
        +AllocateTile(uint64_t owner_key) expected~AtlasTileRect, PoolError~
        +ReleaseTile(uint64_t owner_key) void
        +Utilization() float
        +Capacity() uint32_t
    }
    class AtlasTileRect {
        +uint32_t x
        +uint32_t y
        +uint32_t width
        +uint32_t height
    }

    AliasingSolver --> AliasingMap : produces
    AliasingSolver --> LifetimeInterval : reads
    AliasingSolver --> ResourceSizeInfo : reads
    AliasingMap *-- AliasingAssignment
    PoolAllocator --> Device : uses
    RingAllocator --> Device : uses
    AtlasAllocator --> Device : uses
    AtlasAllocator --> AtlasTileRect : returns
```

### 5. Synchronization Engine

`harmonius::rg::sync` — Barriers, layout transitions, timeline fences.

```mermaid
classDiagram
    class BarrierScheduler {
        +Compute(span~ScheduledPass~, DeclaredGraph) BarrierSchedule
        -MergeBarriers(vector~BarrierDesc~) void
        -ApplySplitBarriers(BarrierSchedule, DeviceCapabilities) void
    }
    class BarrierSchedule {
        +vector~vector~BarrierDesc~~ pre_pass_barriers
        +vector~vector~BarrierDesc~~ post_pass_barriers
    }
    class TimelineFenceManager {
        +TimelineFenceManager(Device)
        +Signal(QueueAffinity, uint64_t) void
        +Wait(QueueAffinity, uint64_t) void
        +IsComplete(QueueAffinity, uint64_t) bool
        +WaitCpu(QueueAffinity, uint64_t) void
        +CurrentValue(QueueAffinity) uint64_t
        +AdvanceFrame() void
        -array~PerQueueFence, 3~ fences_
    }
    class BarrierDesc {
        +BarrierType type
        +ResourceHandle resource
        +UsageType src_usage
        +UsageType dst_usage
        +QueueAffinity src_queue
        +QueueAffinity dst_queue
        +uint32_t mip_level
        +uint32_t array_layer
        +uint32_t mip_count
        +uint32_t layer_count
        +bool is_split_begin
        +bool is_split_end
    }
    class BarrierType["BarrierType «enumeration»"] {
        kMemory
        kLayoutTransition
        kOwnershipRelease
        kOwnershipAcquire
        kAliasing
    }

    BarrierScheduler --> BarrierSchedule : produces
    BarrierSchedule *-- BarrierDesc
    BarrierDesc --> BarrierType
    TimelineFenceManager --> Device : uses
```

### 6. Gating System

`harmonius::rg::gate` — Compile-time and runtime pass gating.

```mermaid
classDiagram
    class GateEvaluator {
        +EvaluateCompileTime(DeclaredGraph, CapabilityDescriptor) expected~vector~PassHandle~, CompileError~
        +EvaluateRuntime(ExecutionPlan, TimestampResults, span~PoolAllocator~) vector~PassHandle~
    }
    class CapabilityDescriptor {
        +bool mesh_shaders
        +bool ray_tracing
        +bool sparse_textures
        +bool async_compute_queue
        +bool transfer_queue
        +bool shading_rate_images
        +bool sixty_four_bit_atomics
        +bool gpu_work_graphs
        +bool opacity_micromaps
        +bool split_barriers
        +Has(string_view) bool
    }
    class CapabilityGateDesc {
        +string_view required_capability
        +bool hard
    }
    class BudgetGateDesc {
        +string_view timestamp_query_name
        +float threshold_ms
        +uint32_t priority
    }
    class PoolUtilizationGateDesc {
        +ResourceHandle pool
        +float utilization_threshold
        +uint32_t priority
    }
    class FallbackEntry {
        +PassHandle pass
        +optional~CapabilityGateDesc~ capability_gate
        +optional~BudgetGateDesc~ budget_gate
    }
    class PathConditionedGateDesc {
        +VariantSlotHandle variant_slot
        +string_view required_variant
    }

    GateEvaluator --> CapabilityDescriptor : reads
    GateEvaluator --> FallbackEntry : evaluates
    FallbackEntry --> CapabilityGateDesc
    FallbackEntry --> BudgetGateDesc
```

### 7. Execution Engine

`harmonius::rg::exec` — Per-frame binding, encoding, submission.

```mermaid
classDiagram
    class Executor {
        +Executor(Device, uint32_t frame_count)
        +BindResource(ResourceHandle, gpu_ResourceHandle) void
        +BindSubgraphParams(SubGraphHandle, uint32_t, span) void
        +SetResolutionScale(string_view, float) void
        +SetPassActive(PassHandle, bool) void
        +SetInstanceActive(SubGraphHandle, uint32_t, bool) void
        +SetInstanceCount(SubGraphHandle, uint32_t) void
        +InvalidateHistory(ResourceHandle) void
        +InjectTransfer(TransferPassDesc) void
        +BindResidencyMap(ResourceHandle, gpu_ResourceHandle) void
        +SetBudgetThreshold(GateHandle, float) void
        +Execute(ExecutionPlan) void
        +FrameIndex() uint64_t
        -EvaluateBudgetGates() void
        -DispatchEncodingGroups(ExecutionPlan) void
        -SubmitCommandBuffers(ExecutionPlan) void
        -AdvanceFrame() void
        -Device device_
        -TimelineFenceManager fence_manager_
        -vector~CommandBufferPool~ cmd_pools_
        -RingAllocator ring_allocator_
        -uint64_t frame_index_
        -uint32_t frame_count_
    }
    class PassContext {
        +cmd() CommandBuffer
        +Resolve(ResourceHandle) gpu_ResourceHandle
        +AllocateConstants(uint64_t, uint64_t) Allocation
        +FrameIndex() uint64_t
        +RenderArea() RenderArea
    }
    class CommandBufferPool {
        +CommandBufferPool(Device, QueueAffinity)
        +acquire() CommandBuffer
        +Release(CommandBuffer) void
        +ResetFrame(uint32_t) void
    }
    class TransferPassDesc {
        +gpu_ResourceHandle src_staging
        +gpu_ResourceHandle dst_resource
        +uint64_t src_offset
        +uint64_t dst_offset
        +uint64_t size_bytes
        +int32_t priority
        +uint64_t completion_fence_value
    }

    Executor --> ExecutionPlan : reads
    Executor --> TimelineFenceManager : owns
    Executor *-- CommandBufferPool
    Executor --> RingAllocator : owns
    Executor --> PassContext : creates
    Executor --> Device : uses
    PassContext --> CommandBuffer : wraps
    PassContext --> RingAllocator : allocates from
```

### 8. Diagnostics

`harmonius::rg::diag` — GPU profiling and memory metrics.

```mermaid
classDiagram
    class DiagnosticsCollector {
        +DiagnosticsCollector(Device)
        +ReadTimestamps() TimestampResults
        +ReadStatistics() PipelineStatistics
        +ReadTransferStats() TransferStatistics
        +ReadMemoryStats() MemoryDiagnostics
        +QueueDepth(QueueAffinity) uint32_t
        +RequestReadback(ReadbackRequest) void
        +ReadReadback() span~uint8_t~
    }
    class TimestampResults {
        +vector~Entry~ entries
        +Find(string_view) optional~Entry~
    }
    class TimestampEntry {
        +string_view pass_name
        +uint64_t begin_ns
        +uint64_t end_ns
        +DurationMs() double
    }
    class PipelineStatistics {
        +vector~Entry~ entries
    }
    class PipelineStatisticsEntry {
        +string_view pass_name
        +uint64_t primitives_count
        +uint64_t invocations_count
    }
    class TransferStatistics {
        +vector~Entry~ entries
        +uint64_t total_bytes_per_frame
        +unordered_map per_pool_bytes
        +unordered_map per_queue_bytes
    }
    class TransferStatisticsEntry {
        +string_view pass_name
        +uint64_t bytes_transferred
        +double latency_ms
    }
    class MemoryDiagnostics {
        +uint64_t peak_aliased_bytes
        +uint64_t total_allocated_bytes
        +float aliasing_efficiency
        +uint32_t active_pool_count
        +uint32_t total_pool_capacity
    }
    class ReadbackRequest {
        +gpu_ResourceHandle src_buffer
        +uint64_t offset
        +uint64_t size
    }

    DiagnosticsCollector --> TimestampResults : produces
    DiagnosticsCollector --> PipelineStatistics : produces
    DiagnosticsCollector --> TransferStatistics : produces
    DiagnosticsCollector --> MemoryDiagnostics : produces
    DiagnosticsCollector --> ReadbackRequest : accepts
    DiagnosticsCollector --> Device : uses
    TimestampResults *-- TimestampEntry
    PipelineStatistics *-- PipelineStatisticsEntry
    TransferStatistics *-- TransferStatisticsEntry
```

### 9. GPU Runtime

`harmonius::gpu_runtime` — Shared services built on the GPU backend interface. The render graph
depends on the GPU runtime layer, not on the GPU backend directly. See [gpu-runtime.md](gpu-runtime.md)
and [gpu-runtime-classes.md](gpu-runtime-classes.md) for full class diagrams.

This section shows only the abstract concept interfaces that the render graph interacts with.
Concrete backend implementations (D3D12, Vulkan, Metal) are documented in:

- [gpu-backend-interface.md](gpu-backend-interface.md) — concepts, types, and cross-backend
  compatibility
- [gpu-backend-d3d12.md](gpu-backend-d3d12.md) — Direct3D 12 (Agility SDK 1.619+, SM 6.9)
- [gpu-backend-vulkan.md](gpu-backend-vulkan.md) — Vulkan 1.4
- [gpu-backend-metal.md](gpu-backend-metal.md) — Metal 4

```mermaid
classDiagram
    class GpuDevice["GpuDevice «concept»"] {
        +Capabilities() DeviceCapabilities
        +CreateTexture(TextureDesc) expected~TextureHandle~
        +CreateBuffer(BufferDesc) expected~BufferHandle~
        +CreateHeap(HeapDesc) expected~HeapHandle~
        +CreatePlacedTexture(HeapHandle, uint64_t, TextureDesc) expected~TextureHandle~
        +CreatePlacedBuffer(HeapHandle, uint64_t, BufferDesc) expected~BufferHandle~
        +CreateSparseTexture(SparseTextureDesc) expected~TextureHandle~
        +UpdateSparseBindings(TextureHandle, span) void
        +QueryTextureAllocationInfo(TextureDesc) AllocationInfo
        +QueryBufferAllocationInfo(BufferDesc) AllocationInfo
        +CreateFence(uint64_t) expected~FenceHandle~
        +FenceCompletedValue(FenceHandle) uint64_t
        +WaitFenceCpu(FenceHandle, uint64_t) void
        +CreateCommandPool(QueueType) CommandPool
        +Submit(QueueType, cmd_bufs, signals, waits) void
        +CreateMeshRenderPipeline(MeshRenderPipelineDesc) expected~PipelineHandle~
        +CreateComputePipeline(ComputePipelineDesc) expected~PipelineHandle~
        +CreateRayTracingPipeline(RayTracingPipelineDesc) expected~PipelineHandle~
        +CreateWorkGraph(WorkGraphDesc) expected~WorkGraphHandle~
        +CreateDescriptorHeap(DescriptorHeapDesc) expected~DescriptorHeapHandle~
        +CreateQueryPool(QueryPoolDesc) expected~QueryPoolHandle~
        +TimestampPeriodNs() double
        +CreateSwapchain(SwapchainDesc) expected~SwapchainHandle~
        +AcquireNextImage(SwapchainHandle) expected~TextureHandle~
        +Present(SwapchainHandle) void
        +ResizeSwapchain(SwapchainHandle, uint32_t, uint32_t) void
        +CreatePipelineCache(PipelineCacheDesc) expected~PipelineCacheHandle~
        +SerializePipelineCache(PipelineCacheHandle) vector~uint8_t~
        +WaitIdle() void
        +SetName(handle, string_view) void
        +Map(BufferHandle) void_ptr
        +Unmap(BufferHandle) void
    }
    class GpuCommandBuffer["GpuCommandBuffer «concept»"] {
        +Begin() void
        +End() void
        +Barrier(BarrierDesc) void
        +BeginRenderPass(RenderPassDesc) void
        +EndRenderPass() void
        +SetPipeline(PipelineHandle) void
        +DispatchMesh(x, y, z) void
        +DispatchMeshIndirect(buf, offset, count, stride) void
        +DispatchMeshIndirectCount(args...) void
        +Dispatch(x, y, z) void
        +DispatchIndirect(buf, offset) void
        +TraceRays(TraceRaysDesc) void
        +TraceRaysIndirect(buf, offset) void
        +BuildAccelerationStructure(BuildDesc) void
        +SetWorkGraph(WorkGraphHandle) void
        +DispatchGraph(DispatchGraphDesc) void
        +CopyBuffer(src, src_off, dst, dst_off, size) void
        +CopyBufferToTexture(args...) void
        +CopyTextureToBuffer(args...) void
        +SetViewport(Viewport) void
        +SetScissor(Scissor) void
        +PushConstants(data, size, offset) void
        +BindDescriptorHeap(DescriptorHeapHandle) void
        +WriteTimestamp(pool, index) void
        +ResolveQueryPool(pool, first, count, dst, offset) void
        +BeginDebugLabel(name) void
        +EndDebugLabel() void
    }
    class GpuCommandPool["GpuCommandPool «concept»"] {
        +AllocateCommandBuffer() CommandBuffer
        +Reset() void
        +AllocatedCount() uint32_t
    }
    class DeviceCapabilities {
        +bool mesh_shaders
        +bool bindless_resources
        +bool timeline_fences
        +bool ray_tracing
        +bool ray_tracing_inline
        +bool opacity_micromaps
        +bool sparse_textures
        +bool int64_atomics
        +bool async_compute_queue
        +bool transfer_queue
        +bool variable_rate_shading
        +bool work_graphs
        +bool split_barriers
        +bool shader_function_linking
        +uint32_t max_push_constants_bytes
        +uint64_t device_local_memory_bytes
    }

    GpuDevice --> DeviceCapabilities : exposes
    GpuDevice --> GpuCommandPool : creates
    GpuCommandPool --> GpuCommandBuffer : allocates
```

---

## Cross-Module Relationships

How the modules depend on each other at the class level. The render graph interacts with the
GPU through the GPU runtime layer (`gpu_runtime`) — it never uses backend APIs directly.

```mermaid
classDiagram
    class GraphBuilder["GraphBuilder «builder»"]
    class DeclaredGraph["DeclaredGraph «builder»"]
    class GraphCompiler["GraphCompiler «compiler»"]
    class ExecutionPlan["ExecutionPlan «compiler»"]
    class GateEvaluator["GateEvaluator «gate»"]
    class BarrierScheduler["BarrierScheduler «sync»"]
    class AliasingSolver["AliasingSolver «resource»"]
    class Executor["Executor «exec»"]
    class TimelineFenceManager["TimelineFenceManager «sync»"]
    class CommandBufferPool["CommandBufferPool «exec»"]
    class DiagnosticsCollector["DiagnosticsCollector «diag»"]
    class Allocator["Allocator «gpu_runtime::memory»"]
    class RingAllocator["RingAllocator «gpu_runtime::memory»"]
    class PoolAllocator["PoolAllocator «gpu_runtime::memory»"]
    class TrackedCommandBuffer["TrackedCommandBuffer «gpu_runtime::state»"]
    class BarrierOptimizer["BarrierOptimizer «gpu_runtime::state»"]
    class WorkGraphExecutor["WorkGraphExecutor «gpu_runtime::work_graph»"]
    class DeviceCapabilities["DeviceCapabilities «gpu»"]

    GraphBuilder --> DeclaredGraph : produces
    GraphCompiler --> DeclaredGraph : consumes
    GraphCompiler --> GateEvaluator : delegates to
    GraphCompiler --> BarrierScheduler : delegates to
    GraphCompiler --> AliasingSolver : delegates to
    GraphCompiler --> ExecutionPlan : produces
    Executor --> ExecutionPlan : reads
    Executor --> TimelineFenceManager : owns
    Executor --> CommandBufferPool : owns
    Executor --> Allocator : "Allocate(), Free()"
    Executor --> RingAllocator : "Allocate() per-frame staging"
    Executor --> TrackedCommandBuffer : "records commands"
    Executor --> WorkGraphExecutor : "Execute()"
    DiagnosticsCollector --> Executor : reads metrics from
    PoolAllocator --> Allocator : "Allocate(), Free()"
    AliasingSolver --> Allocator : "query_allocation_info()"
    BarrierScheduler --> BarrierOptimizer : "Enqueue(), Flush()"
    BarrierScheduler --> DeviceCapabilities : "split_barriers check"
    GateEvaluator --> DiagnosticsCollector : reads timing from
    GateEvaluator --> DeviceCapabilities : "capability checks"
```

### Render Graph to GPU Runtime Type Mapping

How render graph types translate into GPU runtime and backend interface types at the boundary.
The render graph depends on `gpu_runtime` types for all GPU interaction — it never references
backend-specific types (D3D12, Vulkan, Metal).

| Render Graph Type | GPU Runtime / Interface Type | Translation Point |
|------------------|----------------------------|-------------------|
| `rg::QueueAffinity` | `gpu::QueueType` | Direct 1:1 enum mapping (`kGraphics` → `kGraphics`, etc.) |
| `rg::UsageType` | `gpu::PipelineStage` + `gpu::ResourceAccess` + `gpu::TextureLayout` | `BarrierScheduler` performs the multi-field translation |
| `rg::sync::BarrierDesc` | `gpu_runtime::state::BarrierOptimizer::Enqueue()` | Synchronization engine enqueues barriers at Compile time |
| `rg::builder::TransientResourceDesc` | `gpu_runtime::memory::Allocator::Allocate()` | Resource system allocates via the runtime allocator |
| `rg::builder::PassDescriptor` (Execute callback) | `gpu_runtime::state::TrackedCommandBuffer` method calls | `PassContext::cmd()` exposes the tracked command Buffer |
| `rg::compiler::FenceCoordination` | `gpu::FenceSignal` + `gpu::FenceWait` | `TimelineFenceManager` translates fence operations |
| `rg::resource::AliasingAssignment` | `gpu_runtime::memory::Allocator::Allocate()` with placed strategy | Resource system creates placed resources from Assignments |
| `rg::gate::CapabilityDescriptor` | `gpu::DeviceCapabilities` | 1:1 field mapping — populated from device Capabilities at init |

---

## Sequence Diagrams

### Full Lifecycle

Build, compile, then execute across multiple frames.

```mermaid
sequenceDiagram
    participant App
    participant GB as GraphBuilder
    participant GC as GraphCompiler
    participant Exec as Executor
    participant GPU

    Note over App,GPU: Phase 1 - Build (once)
    App->>GB: AddPass(), DeclareTransient(), AttachGate()
    App->>GB: DeclareSubgraphTemplate(), instantiate()
    App->>GB: Build()
    GB-->>App: DeclaredGraph

    Note over App,GPU: Phase 2 - Compile (once)
    App->>GC: Compile(DeclaredGraph, capabilities)
    GC->>GC: validate
    GC->>GC: evaluate gates
    GC->>GC: dead-pass elimination
    GC->>GC: topological sort
    GC->>GC: barrier schedule
    GC->>GC: resource aliasing
    GC->>GC: queue partitioning
    GC->>GC: encoding plan
    GC->>GC: instance merge
    GC-->>App: ExecutionPlan

    Note over App,GPU: Phase 3 - Execute (every frame)
    loop Every Frame
        App->>Exec: bind_frame_data()
        App->>Exec: set_activation_flags()
        App->>Exec: SetResolutionScale()
        Exec->>Exec: evaluate budget gates
        Exec->>GPU: parallel encode + submit
        GPU-->>Exec: fence signals
        Exec->>Exec: advance frame index
    end
```

### Compilation Pipeline

Internal detail of the nine compiler stages.

```mermaid
sequenceDiagram
    participant GC as GraphCompiler
    participant Val as Validator
    participant GE as GateEvaluator
    participant DPE as DeadPassEliminator
    participant TS as TopologicalSorter
    participant BS as BarrierScheduler
    participant AS as AliasingSolver
    participant QP as QueuePartitioner
    participant EP as EncodingPlanner
    participant IM as InstanceMerger

    GC->>Val: Validate(declared_graph)
    Val-->>GC: errors or ok
    GC->>GE: EvaluateCompileTime(graph, caps)
    GE-->>GC: pruned pass set
    GC->>DPE: eliminate(graph, pruned)
    DPE-->>GC: minimal graph
    GC->>TS: sort(minimal_graph)
    TS-->>GC: sorted passes
    GC->>BS: Compute(sorted_passes, graph)
    BS-->>GC: barrier schedule
    GC->>AS: Solve(lifetimes, sizes)
    AS-->>GC: aliasing map
    GC->>QP: partition(sorted_passes, affinities)
    QP-->>GC: queue submissions
    GC->>EP: plan(sorted_passes, dependencies)
    EP-->>GC: encoding groups
    GC->>IM: merge(instances, shared_resources)
    IM-->>GC: unified DAG
    GC-->>GC: assemble ExecutionPlan
```

### Per-Frame Execution

Detailed frame execution showing parallel encoding and submission.

```mermaid
sequenceDiagram
    participant App
    participant Exec as Executor
    participant BG as BudgetGateEval
    participant Pool as CommandBufferPool
    participant T0 as Thread 0
    participant T1 as Thread 1
    participant Ring as RingAllocator
    participant GPU

    App->>Exec: bind_resource(slot, handle)
    App->>Exec: SetPassActive(bloom, false)
    App->>Exec: SetResolutionScale("render", 0.75)
    App->>Exec: InjectTransfer(upload_desc)

    Exec->>BG: evaluate(timing_results, pools)
    BG-->>Exec: culled passes

    Note over T0,T1: Encoding Group 0 (parallel)
    Exec->>Pool: acquire(graphics)
    Pool-->>T0: cmd_buf_0
    Pool-->>T1: cmd_buf_1
    par Thread 0
        T0->>T0: encode shadow cascade 0
        T0->>Ring: AllocateConstants(256)
        Ring-->>T0: offset + mapped_ptr
    and Thread 1
        T1->>T1: encode shadow cascade 1
        T1->>Ring: AllocateConstants(256)
        Ring-->>T1: offset + mapped_ptr
    end
    T0-->>Exec: cmd_buf_0
    T1-->>Exec: cmd_buf_1

    Note over Exec,GPU: Submit in topological order
    Exec->>GPU: Submit(graphics, [cmd_buf_0, cmd_buf_1])
    Exec->>GPU: Signal(graphics_fence, frame_N)
    GPU-->>Exec: fence signal
    Exec->>Exec: AdvanceFrame()
    Exec->>Ring: AdvanceFrame(frame_index)
```

### Parallel Encoding

How encoding groups map to threads.

```mermaid
sequenceDiagram
    participant Sched as EncodingScheduler
    participant T0 as Thread 0
    participant T1 as Thread 1
    participant T2 as Thread 2
    participant Pool as CmdBufferPool

    Note over Sched,Pool: Group 0 - 3 parallel passes
    Sched->>T0: encode(cascade_0)
    Sched->>T1: encode(cascade_1)
    Sched->>T2: encode(cascade_2)
    T0->>Pool: acquire()
    T1->>Pool: acquire()
    T2->>Pool: acquire()
    T0-->>Sched: done
    T1-->>Sched: done
    T2-->>Sched: done

    Note over Sched,Pool: Group 1 - 1 serial pass
    Sched->>T0: encode(gbuffer)
    T0->>Pool: acquire()
    T0-->>Sched: done

    Note over Sched,Pool: Group 2 - 1 serial pass
    Sched->>T0: encode(lighting)
    T0-->>Sched: done

    Note over Sched,Pool: Group 3 - 3 parallel passes
    Sched->>T0: encode(bloom)
    Sched->>T1: encode(ao)
    Sched->>T2: encode(volumetrics)
    T0-->>Sched: done
    T1-->>Sched: done
    T2-->>Sched: done

    Sched->>Sched: reorder to topological order
```

### Streaming Fault Resolution

How a residency fault flows through the system across two frames.

```mermaid
sequenceDiagram
    participant Renderer
    participant IO as IO System
    participant Exec as Executor
    participant TX as Transfer Queue
    participant GFX as Graphics Queue
    participant Diag as Diagnostics

    Note over Renderer,Diag: Frame N - fault detected
    GFX->>Renderer: page fault (tile not resident)
    Renderer->>IO: request_tile(page_id, priority=critical)
    IO->>IO: read from disk (platform-native IO)
    IO-->>Exec: staging buffer ready

    Note over Renderer,Diag: Frame N+1 - upload injected
    Exec->>Exec: InjectTransfer(staging, dest, priority)
    Exec->>TX: upload pass
    TX->>TX: copy staging to device-local
    TX-->>Exec: completion fence signal
    Exec->>Exec: update residency map
    Exec->>GFX: bind updated residency map
    GFX->>GFX: consuming pass reads new tile
    Diag->>Diag: record transfer throughput
```
