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
  - [9. GPU Backend](#9-gpu-backend)
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
        invalid
    }
    class ResourceHandle["ResourceHandle «enumeration»"] {
        invalid
    }
    class SubGraphHandle["SubGraphHandle «enumeration»"] {
        invalid
    }
    class GateHandle["GateHandle «enumeration»"] {
        invalid
    }
    class ChainHandle["ChainHandle «enumeration»"] {
        invalid
    }
    class VariantSlotHandle["VariantSlotHandle «enumeration»"] {
        invalid
    }
    class PassType["PassType «enumeration»"] {
        rasterization
        compute
        ray_tracing_dispatch
        acceleration_structure_build
        transfer
        msaa_resolve
        present
        host_callback
        work_graph
        checkerboard_resolve
    }
    class AccessMode["AccessMode «enumeration»"] {
        read
        write
        read_write
    }
    class UsageType["UsageType «enumeration»"] {
        color_attachment
        depth_attachment
        shader_read
        storage_read
        storage_write
        shading_rate_attachment
        indirect_argument
        acceleration_structure_read
        acceleration_structure_build_write
        transfer_src
        transfer_dst
        present
    }
    class QueueAffinity["QueueAffinity «enumeration»"] {
        graphics
        async_compute
        transfer
        any
    }
    class ResourceCategory["ResourceCategory «enumeration»"] {
        transient
        persistent
        imported
        history
        multi_frame_history
        sparse
        pool_backed
        staging
        atlas
        acceleration_structure
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
        cycle_detected
        type_mismatch
        undeclared_resource
        queue_incompatibility
        single_writer_violation
        variant_ambiguity
        instance_count_mismatch
        hard_gate_unsatisfied
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
        +add_pass(PassDescriptor) PassHandle
        +begin_chain(string_view) ChainHandle
        +add_chain_step(ChainHandle, PassDescriptor) PassHandle
        +end_chain(ChainHandle) void
        +declare_variant_slot(string_view) VariantSlotHandle
        +add_variant(VariantSlotHandle, string_view, PassDescriptor) PassHandle
        +declare_subgraph_template(string_view, SubGraphDescriptor) SubGraphHandle
        +instantiate_subgraph(SubGraphHandle, uint32_t, span) void
        +declare_transient(TransientResourceDesc) ResourceHandle
        +declare_persistent(PersistentResourceDesc) ResourceHandle
        +declare_imported(ImportedResourceDesc) ResourceHandle
        +declare_history(HistoryResourceDesc) ResourceHandle
        +declare_multi_frame_history(MultiFrameHistoryDesc) ResourceHandle
        +declare_sparse(SparseResourceDesc) ResourceHandle
        +declare_pool(PoolResourceDesc) ResourceHandle
        +declare_staging(StagingBufferDesc) ResourceHandle
        +declare_atlas(AtlasResourceDesc) ResourceHandle
        +declare_acceleration_structure(AccelStructDesc) ResourceHandle
        +attach_capability_gate(PassHandle, CapabilityGateDesc) GateHandle
        +attach_budget_gate(PassHandle, BudgetGateDesc) GateHandle
        +declare_fallback_chain(string_view, span) GateHandle
        +add_ordering_edge(PassHandle, PassHandle) void
        +attach_timestamp_query(PassHandle, string_view) void
        +attach_statistics_query(PassHandle, string_view) void
        +mark_debug_overlay(PassHandle) void
        +build() expected~DeclaredGraph, CompileError~
    }
    class DeclaredGraph {
        +passes() span~PassDescriptor~
        +pass_count() uint32_t
        +resource_count() uint32_t
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
        +compile(DeclaredGraph, CapabilityDescriptor, CompileOptions) expected~ExecutionPlan, CompileError~
        +recompile_residency(ExecutionPlan, span~ResidencyChange~) expected~ExecutionPlan, CompileError~
        -validate(DeclaredGraph) vector~ValidationError~
        -evaluate_gates(DeclaredGraph, CapabilityDescriptor) vector~PassHandle~
        -eliminate_dead_passes(DeclaredGraph) void
        -topological_sort() void
        -compute_barriers() BarrierSchedule
        -compute_aliasing() AliasingMap
        -partition_queues() void
        -plan_encoding() void
        -merge_instances() void
    }
    class ExecutionPlan {
        +passes() span~ScheduledPass~
        +queue_submissions() span~QueueSubmission~
        +encoding_groups() span~EncodingGroup~
        +aliasing_map() AliasingMap
        +fence_points() span~FenceCoordination~
        +transfer_injection_index() uint32_t
        +active_pass_count() uint32_t
        +conditional_passes() span~PassHandle~
        +resolution_params() span~ResolutionParam~
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
        +solve(span~LifetimeInterval~, span~ResourceSizeInfo~) AliasingMap
    }
    class AliasingMap {
        +assignments() span~AliasingAssignment~
        +peak_memory_bytes() uint64_t
        +total_logical_bytes() uint64_t
        +aliasing_efficiency() float
    }
    class PoolAllocator {
        +PoolAllocator(PoolResourceDesc, Device)
        +allocate() expected~ResourceHandle, PoolError~
        +release(ResourceHandle) void
        +utilization() float
        +capacity() uint32_t
        +active_count() uint32_t
    }
    class RingAllocator {
        +RingAllocator(uint64_t slot_size, uint32_t frame_count, Device)
        +allocate(uint64_t size, uint64_t alignment) optional~Allocation~
        +advance_frame(uint32_t frame_index) void
        +buffer() gpu_ResourceHandle
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

    AliasingSolver --> AliasingMap : produces
    AliasingSolver --> LifetimeInterval : reads
    AliasingSolver --> ResourceSizeInfo : reads
    AliasingMap *-- AliasingAssignment
    PoolAllocator --> Device : uses
    RingAllocator --> Device : uses
```

### 5. Synchronization Engine

`harmonius::rg::sync` — Barriers, layout transitions, timeline fences.

```mermaid
classDiagram
    class BarrierScheduler {
        +compute(span~ScheduledPass~, DeclaredGraph) BarrierSchedule
        -merge_barriers(vector~BarrierDesc~) void
        -apply_split_barriers(BarrierSchedule, DeviceCapabilities) void
    }
    class BarrierSchedule {
        +vector~vector~BarrierDesc~~ pre_pass_barriers
        +vector~vector~BarrierDesc~~ post_pass_barriers
    }
    class TimelineFenceManager {
        +TimelineFenceManager(Device)
        +signal(QueueAffinity, uint64_t) void
        +wait(QueueAffinity, uint64_t) void
        +is_complete(QueueAffinity, uint64_t) bool
        +wait_cpu(QueueAffinity, uint64_t) void
        +current_value(QueueAffinity) uint64_t
        +advance_frame() void
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
        memory
        layout_transition
        ownership_release
        ownership_acquire
        aliasing
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
        +evaluate_compile_time(DeclaredGraph, CapabilityDescriptor) expected~vector~PassHandle~, CompileError~
        +evaluate_runtime(ExecutionPlan, TimestampResults, span~PoolAllocator~) vector~PassHandle~
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
        +has(string_view) bool
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
        +bind_resource(ResourceHandle, gpu_ResourceHandle) void
        +bind_subgraph_params(SubGraphHandle, uint32_t, span) void
        +set_resolution_scale(string_view, float) void
        +set_pass_active(PassHandle, bool) void
        +set_instance_active(SubGraphHandle, uint32_t, bool) void
        +invalidate_history(ResourceHandle) void
        +inject_transfer(TransferPassDesc) void
        +bind_residency_map(ResourceHandle, gpu_ResourceHandle) void
        +set_budget_threshold(GateHandle, float) void
        +execute(ExecutionPlan) void
        +frame_index() uint64_t
        -evaluate_budget_gates() void
        -dispatch_encoding_groups(ExecutionPlan) void
        -submit_command_buffers(ExecutionPlan) void
        -advance_frame() void
        -Device device_
        -TimelineFenceManager fence_manager_
        -vector~CommandBufferPool~ cmd_pools_
        -RingAllocator ring_allocator_
        -uint64_t frame_index_
        -uint32_t frame_count_
    }
    class PassContext {
        +cmd() CommandBuffer
        +resolve(ResourceHandle) gpu_ResourceHandle
        +allocate_constants(uint64_t, uint64_t) Allocation
        +frame_index() uint64_t
        +render_area() RenderArea
    }
    class CommandBufferPool {
        +CommandBufferPool(Device, QueueAffinity)
        +acquire() CommandBuffer
        +release(CommandBuffer) void
        +reset_frame(uint32_t) void
    }
    class TransferPassDesc {
        +gpu_ResourceHandle src_staging
        +gpu_ResourceHandle dst_resource
        +uint64_t src_offset
        +uint64_t dst_offset
        +uint64_t size_bytes
        +int32_t priority
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
        +read_timestamps() TimestampResults
        +read_statistics() PipelineStatistics
        +read_transfer_stats() TransferStatistics
        +read_memory_stats() MemoryDiagnostics
        +queue_depth(QueueAffinity) uint32_t
        +request_readback(ReadbackRequest) void
        +read_readback() span~uint8_t~
    }
    class TimestampResults {
        +vector~Entry~ entries
        +find(string_view) optional~Entry~
    }
    class TimestampEntry {
        +string_view pass_name
        +uint64_t begin_ns
        +uint64_t end_ns
        +duration_ms() double
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

### 9. GPU Backend

`harmonius::gpu` — Concrete device, command buffer, and command pool types selected at compile
time. Interface contracts are defined as C++20 concepts (`GpuDevice`, `GpuCommandBuffer`,
`GpuCommandPool`) and enforced via `static_assert`. No virtual dispatch — one backend is compiled
per binary. See the dedicated GPU backend design documents:

- [gpu-backend-interface.md](gpu-backend-interface.md) — concepts, types, and cross-backend
  compatibility
- [gpu-backend-d3d12.md](gpu-backend-d3d12.md) — Direct3D 12 (Agility SDK 1.619+, SM 6.9)
- [gpu-backend-vulkan.md](gpu-backend-vulkan.md) — Vulkan 1.4
- [gpu-backend-metal.md](gpu-backend-metal.md) — Metal 4

```mermaid
classDiagram
    class GpuDevice["GpuDevice «concept»"] {
        +capabilities() DeviceCapabilities
        +create_texture(TextureDesc) expected~TextureHandle~
        +create_buffer(BufferDesc) expected~BufferHandle~
        +create_heap(HeapDesc) expected~HeapHandle~
        +create_placed_texture(HeapHandle, uint64_t, TextureDesc) expected~TextureHandle~
        +create_placed_buffer(HeapHandle, uint64_t, BufferDesc) expected~BufferHandle~
        +create_sparse_texture(SparseTextureDesc) expected~TextureHandle~
        +update_sparse_bindings(TextureHandle, span) void
        +query_texture_allocation_info(TextureDesc) AllocationInfo
        +query_buffer_allocation_info(BufferDesc) AllocationInfo
        +create_fence(uint64_t) expected~FenceHandle~
        +fence_completed_value(FenceHandle) uint64_t
        +wait_fence_cpu(FenceHandle, uint64_t) void
        +create_command_pool(QueueType) CommandPool
        +submit(QueueType, cmd_bufs, signals, waits) void
        +create_mesh_render_pipeline(MeshRenderPipelineDesc) expected~PipelineHandle~
        +create_compute_pipeline(ComputePipelineDesc) expected~PipelineHandle~
        +create_ray_tracing_pipeline(RayTracingPipelineDesc) expected~PipelineHandle~
        +create_work_graph(WorkGraphDesc) expected~WorkGraphHandle~
        +create_descriptor_heap(DescriptorHeapDesc) expected~DescriptorHeapHandle~
        +create_query_pool(QueryPoolDesc) expected~QueryPoolHandle~
        +timestamp_period_ns() double
        +create_swapchain(SwapchainDesc) expected~SwapchainHandle~
        +acquire_next_image(SwapchainHandle) expected~TextureHandle~
        +present(SwapchainHandle) void
        +resize_swapchain(SwapchainHandle, uint32_t, uint32_t) void
        +create_pipeline_cache(PipelineCacheDesc) expected~PipelineCacheHandle~
        +serialize_pipeline_cache(PipelineCacheHandle) vector~uint8_t~
        +wait_idle() void
        +set_name(handle, string_view) void
        +map(BufferHandle) void_ptr
        +unmap(BufferHandle) void
    }
    class GpuCommandBuffer["GpuCommandBuffer «concept»"] {
        +begin() void
        +end() void
        +barrier(BarrierDesc) void
        +begin_render_pass(RenderPassDesc) void
        +end_render_pass() void
        +set_pipeline(PipelineHandle) void
        +dispatch_mesh(x, y, z) void
        +dispatch_mesh_indirect(buf, offset, count, stride) void
        +dispatch_mesh_indirect_count(args...) void
        +dispatch(x, y, z) void
        +dispatch_indirect(buf, offset) void
        +trace_rays(TraceRaysDesc) void
        +trace_rays_indirect(buf, offset) void
        +build_acceleration_structure(BuildDesc) void
        +set_work_graph(WorkGraphHandle) void
        +dispatch_graph(DispatchGraphDesc) void
        +copy_buffer(src, src_off, dst, dst_off, size) void
        +copy_buffer_to_texture(args...) void
        +copy_texture_to_buffer(args...) void
        +set_viewport(Viewport) void
        +set_scissor(Scissor) void
        +push_constants(data, size, offset) void
        +bind_descriptor_heap(DescriptorHeapHandle) void
        +write_timestamp(pool, index) void
        +resolve_query_pool(pool, first, count, dst, offset) void
        +begin_debug_label(name) void
        +end_debug_label() void
    }
    class GpuCommandPool["GpuCommandPool «concept»"] {
        +allocate_command_buffer() CommandBuffer
        +reset() void
        +allocated_count() uint32_t
    }
    class D3D12Device {
        -ID3D12Device16 device_
        -D3D12MA_Allocator allocator_
        -ID3D12DescriptorHeap cbv_srv_uav_heap_
        -ID3D12RootSignature global_root_signature_
    }
    class VulkanDevice {
        -VkDevice device_
        -VmaAllocator allocator_
        -VkDescriptorPool bindless_pool_
        -VkPipelineLayout global_layout_
    }
    class MetalDevice {
        -MTLDevice device_
        -MTLResidencySet residency_set_
        -MTL4Compiler compiler_
        -VmaVirtualBlock virtual_block_
    }
    class D3D12CommandBuffer {
        -ID3D12GraphicsCommandList10 list_
    }
    class VulkanCommandBuffer {
        -VkCommandBuffer buffer_
    }
    class MetalCommandBuffer {
        -MTL4CommandBuffer cmd_
    }
    class D3D12CommandPool {
        -ID3D12CommandAllocator allocator_
        -vector cached_lists_
    }
    class VulkanCommandPool {
        -VkCommandPool pool_
        -vector cached_buffers_
    }
    class MetalCommandPool {
        -MTL4CommandAllocator allocator_
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

    GpuDevice ..> D3D12Device : satisfied by
    GpuDevice ..> VulkanDevice : satisfied by
    GpuDevice ..> MetalDevice : satisfied by
    GpuCommandBuffer ..> D3D12CommandBuffer : satisfied by
    GpuCommandBuffer ..> VulkanCommandBuffer : satisfied by
    GpuCommandBuffer ..> MetalCommandBuffer : satisfied by
    GpuCommandPool ..> D3D12CommandPool : satisfied by
    GpuCommandPool ..> VulkanCommandPool : satisfied by
    GpuCommandPool ..> MetalCommandPool : satisfied by
    D3D12Device --> D3D12CommandPool : creates
    VulkanDevice --> VulkanCommandPool : creates
    MetalDevice --> MetalCommandPool : creates
    D3D12CommandPool --> D3D12CommandBuffer : allocates
    VulkanCommandPool --> VulkanCommandBuffer : allocates
    MetalCommandPool --> MetalCommandBuffer : allocates
    D3D12Device --> DeviceCapabilities : exposes
    VulkanDevice --> DeviceCapabilities : exposes
    MetalDevice --> DeviceCapabilities : exposes
```

---

## Cross-Module Relationships

How the nine modules depend on each other at the class level.

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
    class RingAllocator["RingAllocator «resource»"]
    class PoolAllocator["PoolAllocator «resource»"]
    class DiagnosticsCollector["DiagnosticsCollector «diag»"]
    class Device["Device «gpu»"]
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
    Executor --> RingAllocator : owns
    Executor --> Device : "submit(), present(), wait_idle()"
    DiagnosticsCollector --> Executor : reads metrics from
    DiagnosticsCollector --> Device : "create_query_pool(), timestamp_period_ns()"
    PoolAllocator --> Device : "create_texture(), create_buffer()"
    RingAllocator --> Device : "create_buffer(upload), map()"
    AliasingSolver --> Device : "query_allocation_info()"
    BarrierScheduler --> DeviceCapabilities : "split_barriers check"
    GateEvaluator --> DiagnosticsCollector : reads timing from
    GateEvaluator --> DeviceCapabilities : "capability checks"
    CommandBufferPool --> Device : "create_command_pool()"
    TimelineFenceManager --> Device : "create_fence(), fence_completed_value()"
```

### Render Graph to GPU Backend Type Mapping

How render graph types translate into GPU backend types at the boundary between the two layers.

| Render Graph Type | GPU Backend Type | Translation Point |
|------------------|-----------------|-------------------|
| `rg::QueueAffinity` | `gpu::QueueType` | Direct 1:1 enum mapping (`graphics` → `graphics`, etc.) |
| `rg::UsageType` | `gpu::PipelineStage` + `gpu::ResourceAccess` + `gpu::TextureLayout` | `BarrierScheduler` performs the multi-field translation |
| `rg::sync::BarrierDesc` | `gpu::BarrierDesc` (containing `gpu::TextureBarrier` / `gpu::BufferBarrier` / `gpu::GlobalBarrier`) | Synchronization engine translates at compile time |
| `rg::builder::TransientResourceDesc` | `gpu::TextureDesc` or `gpu::BufferDesc` | Resource system maps format, dimensions, usage flags |
| `rg::builder::PassDescriptor` (execute callback) | `gpu::CommandBuffer` method calls | `PassContext::cmd()` exposes the command buffer |
| `rg::compiler::FenceCoordination` | `gpu::FenceSignal` + `gpu::FenceWait` in `gpu::Device::submit()` | `TimelineFenceManager` translates fence operations |
| `rg::resource::AliasingAssignment` | `gpu::Device::create_placed_texture()` / `create_placed_buffer()` at heap offset | Resource system creates placed resources from assignments |
| `rg::gate::CapabilityDescriptor` | `gpu::DeviceCapabilities` | 1:1 field mapping — populated from `gpu::Device::capabilities()` at init |

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
    App->>GB: add_pass(), declare_transient(), attach_gate()
    App->>GB: declare_subgraph_template(), instantiate()
    App->>GB: build()
    GB-->>App: DeclaredGraph

    Note over App,GPU: Phase 2 - Compile (once)
    App->>GC: compile(DeclaredGraph, capabilities)
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
        App->>Exec: set_resolution_scale()
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

    GC->>Val: validate(declared_graph)
    Val-->>GC: errors or ok
    GC->>GE: evaluate_compile_time(graph, caps)
    GE-->>GC: pruned pass set
    GC->>DPE: eliminate(graph, pruned)
    DPE-->>GC: minimal graph
    GC->>TS: sort(minimal_graph)
    TS-->>GC: sorted passes
    GC->>BS: compute(sorted_passes, graph)
    BS-->>GC: barrier schedule
    GC->>AS: solve(lifetimes, sizes)
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
    App->>Exec: set_pass_active(bloom, false)
    App->>Exec: set_resolution_scale("render", 0.75)
    App->>Exec: inject_transfer(upload_desc)

    Exec->>BG: evaluate(timing_results, pools)
    BG-->>Exec: culled passes

    Note over T0,T1: Encoding Group 0 (parallel)
    Exec->>Pool: acquire(graphics)
    Pool-->>T0: cmd_buf_0
    Pool-->>T1: cmd_buf_1
    par Thread 0
        T0->>T0: encode shadow cascade 0
        T0->>Ring: allocate_constants(256)
        Ring-->>T0: offset + mapped_ptr
    and Thread 1
        T1->>T1: encode shadow cascade 1
        T1->>Ring: allocate_constants(256)
        Ring-->>T1: offset + mapped_ptr
    end
    T0-->>Exec: cmd_buf_0
    T1-->>Exec: cmd_buf_1

    Note over Exec,GPU: Submit in topological order
    Exec->>GPU: submit(graphics, [cmd_buf_0, cmd_buf_1])
    Exec->>GPU: signal(graphics_fence, frame_N)
    GPU-->>Exec: fence signal
    Exec->>Exec: advance_frame()
    Exec->>Ring: advance_frame(frame_index)
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
    Exec->>Exec: inject_transfer(staging, dest, priority)
    Exec->>TX: upload pass
    TX->>TX: copy staging to device-local
    TX-->>Exec: completion fence signal
    Exec->>Exec: update residency map
    Exec->>GFX: bind updated residency map
    GFX->>GFX: consuming pass reads new tile
    Diag->>Diag: record transfer throughput
```
