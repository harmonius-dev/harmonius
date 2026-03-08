# GPU Backend Interface Class and Sequence Diagrams

Class diagrams for each shared type module and sequence diagrams showing cross-module interactions.
Companion to [gpu-backend-interface.md](gpu-backend-interface.md).

---

## Contents

- [Module Class Diagrams](#module-class-diagrams)
  - [1. Handles](#1-handles)
  - [2. Core Enums](#2-core-enums)
  - [3. Resource Descriptors](#3-resource-descriptors)
  - [4. Acceleration Structures](#4-acceleration-structures)
  - [5. Synchronization Types](#5-synchronization-types)
  - [6. Render Pass Types](#6-render-pass-types)
  - [7. Pipeline State Types](#7-pipeline-state-types)
  - [8. Resource Binding Types](#8-resource-binding-types)
  - [9. Diagnostics Types](#9-diagnostics-types)
  - [10. Indirect Command Types](#10-indirect-command-types)
  - [11. Device Capabilities](#11-device-capabilities)
  - [12. Error Types](#12-error-types)
- [Concepts](#concepts)
- [Cross-Module Relationships](#cross-module-relationships)
- [Sequence Diagrams](#sequence-diagrams)
  - [Backend Selection and Type Aliasing](#backend-selection-and-type-aliasing)
  - [Command Recording Lifecycle](#command-recording-lifecycle)
  - [Cross-Queue Synchronization with Timeline Fences](#cross-queue-synchronization-with-timeline-fences)

---

## Module Class Diagrams

### 1. Handles

`harmonius::gpu` -- Opaque typed handles. All are `enum class : uint64_t` with an
`invalid = 0` sentinel value. The backend stores implementation-specific data behind
each handle.

```mermaid
classDiagram
    class TextureHandle {
        <<enum class>>
        invalid
    }
    class BufferHandle {
        <<enum class>>
        invalid
    }
    class HeapHandle {
        <<enum class>>
        invalid
    }
    class AccelerationStructureHandle {
        <<enum class>>
        invalid
    }
    class FenceHandle {
        <<enum class>>
        invalid
    }
    class PipelineHandle {
        <<enum class>>
        invalid
    }
    class WorkGraphHandle {
        <<enum class>>
        invalid
    }
    class DescriptorHeapHandle {
        <<enum class>>
        invalid
    }
    class QueryPoolHandle {
        <<enum class>>
        invalid
    }
    class SwapchainHandle {
        <<enum class>>
        invalid
    }
    class PipelineCacheHandle {
        <<enum class>>
        invalid
    }
```

### 2. Core Enums

`harmonius::gpu` -- Vocabulary enumerations used across multiple modules.

```mermaid
classDiagram
    class HeapType {
        <<enum class>>
        device_local
        upload
        readback
    }
    class QueueType {
        <<enum class>>
        graphics
        async_compute
        transfer
    }
    class TextureDimension {
        <<enum class>>
        tex_2d
        tex_2d_array
        tex_3d
        tex_cube
        tex_cube_array
    }
    class SampleCount {
        <<enum class>>
        x1
        x2
        x4
    }
    class LoadOp {
        <<enum class>>
        load
        clear
        dont_care
    }
    class StoreOp {
        <<enum class>>
        store
        dont_care
    }
```

### 3. Resource Descriptors

`harmonius::gpu` -- Structs describing resource creation parameters. Usage flag
enums are bitmasks combined with bitwise OR.

```mermaid
classDiagram
    class TextureDesc {
        +string_view name
        +TextureDimension dimension
        +Format format
        +uint32_t width
        +uint32_t height
        +uint32_t depth_or_layers
        +uint32_t mip_levels
        +SampleCount samples
        +TextureUsageFlags usage
    }
    class TextureUsageFlagBits {
        <<enum class>>
        color_attachment
        depth_stencil_attachment
        shader_read
        storage_read_write
        transfer_src
        transfer_dst
        shading_rate_attachment
    }
    class BufferDesc {
        +string_view name
        +uint64_t size_bytes
        +HeapType heap_type
        +BufferUsageFlags usage
    }
    class BufferUsageFlagBits {
        <<enum class>>
        constant_buffer
        storage_buffer
        index_buffer
        indirect_argument
        transfer_src
        transfer_dst
        acceleration_structure
        shader_binding_table
    }
    class HeapDesc {
        +uint64_t size_bytes
        +HeapType type
    }
    class AllocationInfo {
        +uint64_t size_bytes
        +uint64_t alignment
    }
    class SparseTextureDesc {
        +TextureDesc base
        +uint32_t tile_width
        +uint32_t tile_height
    }
    class SparseTileBinding {
        +uint32_t mip_level
        +uint32_t array_layer
        +uint32_t tile_x
        +uint32_t tile_y
        +HeapHandle heap
        +uint64_t heap_offset
    }
    class DeviceDesc {
        +bool enable_validation
        +bool enable_gpu_capture
        +uint32_t frame_count
    }

    TextureDesc --> TextureDimension
    TextureDesc --> SampleCount
    TextureDesc --> TextureUsageFlagBits : usage flags
    BufferDesc --> HeapType
    BufferDesc --> BufferUsageFlagBits : usage flags
    HeapDesc --> HeapType
    SparseTextureDesc --> TextureDesc : base
    SparseTileBinding --> HeapHandle
```

### 4. Acceleration Structures

`harmonius::gpu` -- Ray tracing acceleration structure creation and build types.

```mermaid
classDiagram
    class AccelerationStructureType {
        <<enum class>>
        bottom_level
        top_level
    }
    class AccelerationStructureDesc {
        +AccelerationStructureType type
        +BufferHandle buffer
        +uint64_t offset
        +uint64_t size_bytes
    }
    class AccelerationStructureSizes {
        +uint64_t structure_size_bytes
        +uint64_t build_scratch_bytes
        +uint64_t update_scratch_bytes
    }
    class AccelerationStructureGeometry {
        +GeometryType type
        +BufferHandle vertex_buffer
        +uint64_t vertex_offset
        +uint32_t vertex_count
        +uint32_t vertex_stride
        +Format vertex_format
        +BufferHandle index_buffer
        +uint64_t index_offset
        +uint32_t index_count
        +Format index_format
        +BufferHandle transform_buffer
        +uint64_t transform_offset
        +BufferHandle aabb_buffer
        +uint64_t aabb_offset
        +uint32_t aabb_count
        +uint32_t aabb_stride
        +bool opaque
    }
    class GeometryType {
        <<enum class>>
        triangles
        aabbs
    }
    class AccelerationStructureBuildDesc {
        +AccelerationStructureHandle dst
        +AccelerationStructureHandle src
        +BufferHandle scratch
        +uint64_t scratch_offset
        +span~AccelerationStructureGeometry~ geometries
        +bool update
    }
    class AccelerationStructureInstance {
        +float transform_3x4
        +uint32_t instance_id
        +uint32_t mask
        +uint32_t sbt_offset
        +AccelerationStructureHandle blas
    }

    AccelerationStructureDesc --> AccelerationStructureType
    AccelerationStructureDesc --> BufferHandle
    AccelerationStructureGeometry --> GeometryType
    AccelerationStructureGeometry --> BufferHandle
    AccelerationStructureBuildDesc --> AccelerationStructureHandle
    AccelerationStructureBuildDesc --> BufferHandle
    AccelerationStructureBuildDesc o-- AccelerationStructureGeometry
    AccelerationStructureInstance --> AccelerationStructureHandle
```

### 5. Synchronization Types

`harmonius::gpu` -- Barrier descriptors, pipeline stages, resource access masks,
texture layouts, and fence operations. The render graph compiler produces these
barrier descriptions; the backend translates them to native API calls.

```mermaid
classDiagram
    class PipelineStage {
        +uint64_t none
        +uint64_t mesh_shader
        +uint64_t task_shader
        +uint64_t fragment_shader
        +uint64_t compute_shader
        +uint64_t ray_tracing_shader
        +uint64_t all_shading
        +uint64_t color_output
        +uint64_t depth_stencil
        +uint64_t transfer
        +uint64_t resolve
        +uint64_t acceleration_structure
        +uint64_t indirect_argument
        +uint64_t shading_rate
        +uint64_t all
        +uint64_t split_begin
        +uint64_t split_end
    }
    class ResourceAccess {
        +uint64_t none
        +uint64_t shader_read
        +uint64_t shader_write
        +uint64_t color_attachment_read
        +uint64_t color_attachment_write
        +uint64_t depth_stencil_read
        +uint64_t depth_stencil_write
        +uint64_t transfer_read
        +uint64_t transfer_write
        +uint64_t indirect_read
        +uint64_t acceleration_structure_read
        +uint64_t acceleration_structure_write
        +uint64_t shading_rate_read
        +uint64_t present
    }
    class TextureLayout {
        +undefined
        +general
        +color_attachment
        +depth_stencil_attachment
        +depth_stencil_read_only
        +shader_read_only
        +transfer_src
        +transfer_dst
        +present
        +shading_rate
    }
    class TextureSubresourceRange {
        +uint32_t base_mip_level
        +uint32_t mip_level_count
        +uint32_t base_array_layer
        +uint32_t array_layer_count
    }
    class TextureBarrier {
        +TextureHandle texture
        +PipelineStage src_stage
        +ResourceAccess src_access
        +TextureLayout old_layout
        +PipelineStage dst_stage
        +ResourceAccess dst_access
        +TextureLayout new_layout
        +TextureSubresourceRange subresource_range
        +QueueType src_queue
        +QueueType dst_queue
        +bool discard
    }
    class BufferBarrier {
        +BufferHandle buffer
        +PipelineStage src_stage
        +ResourceAccess src_access
        +PipelineStage dst_stage
        +ResourceAccess dst_access
        +uint64_t offset
        +uint64_t size
        +QueueType src_queue
        +QueueType dst_queue
    }
    class GlobalBarrier {
        +PipelineStage src_stage
        +ResourceAccess src_access
        +PipelineStage dst_stage
        +ResourceAccess dst_access
    }
    class BarrierDesc {
        +span~GlobalBarrier~ global_barriers
        +span~TextureBarrier~ texture_barriers
        +span~BufferBarrier~ buffer_barriers
    }
    class FenceSignal {
        +FenceHandle fence
        +uint64_t value
    }
    class FenceWait {
        +FenceHandle fence
        +uint64_t value
    }

    TextureBarrier --> PipelineStage
    TextureBarrier --> ResourceAccess
    TextureBarrier --> TextureLayout
    TextureBarrier --> TextureSubresourceRange
    BufferBarrier --> PipelineStage
    BufferBarrier --> ResourceAccess
    GlobalBarrier --> PipelineStage
    GlobalBarrier --> ResourceAccess
    BarrierDesc o-- GlobalBarrier
    BarrierDesc o-- TextureBarrier
    BarrierDesc o-- BufferBarrier
    FenceSignal --> FenceHandle
    FenceWait --> FenceHandle
```

### 6. Render Pass Types

`harmonius::gpu` -- Attachment descriptors for render passes and viewport/scissor
state.

```mermaid
classDiagram
    class ColorAttachment {
        +TextureHandle texture
        +uint32_t mip_level
        +uint32_t array_layer
        +LoadOp load_op
        +StoreOp store_op
        +float clear_color[4]
        +TextureHandle resolve_texture
    }
    class DepthStencilAttachment {
        +TextureHandle texture
        +uint32_t mip_level
        +uint32_t array_layer
        +LoadOp depth_load_op
        +StoreOp depth_store_op
        +float clear_depth
        +LoadOp stencil_load_op
        +StoreOp stencil_store_op
        +uint8_t clear_stencil
    }
    class ShadingRateAttachment {
        +TextureHandle texture
        +uint32_t tile_width
        +uint32_t tile_height
    }
    class RenderPassDesc {
        +span~ColorAttachment~ color_attachments
        +optional~DepthStencilAttachment~ depth_stencil
        +optional~ShadingRateAttachment~ shading_rate
        +Extent2D render_area
    }
    class Viewport {
        +float x
        +float y
        +float width
        +float height
        +float min_depth
        +float max_depth
    }
    class Scissor {
        +int32_t x
        +int32_t y
        +uint32_t width
        +uint32_t height
    }
    class Extent2D {
        +uint32_t width
        +uint32_t height
    }
    class Extent3D {
        +uint32_t width
        +uint32_t height
        +uint32_t depth
    }

    RenderPassDesc o-- ColorAttachment
    RenderPassDesc o-- DepthStencilAttachment
    RenderPassDesc o-- ShadingRateAttachment
    RenderPassDesc --> Extent2D
    ColorAttachment --> TextureHandle
    DepthStencilAttachment --> TextureHandle
    ShadingRateAttachment --> TextureHandle
```

### 7. Pipeline State Types

`harmonius::gpu` -- Shader bytecode, pipeline descriptors for mesh render, compute,
ray tracing, work graphs, and associated render state blocks.

```mermaid
classDiagram
    class ShaderBytecode {
        +const void* data
        +uint64_t size_bytes
    }
    class ShaderLibrary {
        +const void* data
        +uint64_t size_bytes
    }
    class MeshRenderPipelineDesc {
        +ShaderBytecode task_shader
        +ShaderBytecode mesh_shader
        +ShaderBytecode pixel_shader
        +span~Format~ color_formats
        +Format depth_stencil_format
        +SampleCount samples
        +BlendState blend
        +RasterizerState rasterizer
        +DepthStencilState depth_stencil
    }
    class LinkedMeshRenderPipelineDesc {
        +ShaderBytecode task_shader
        +ShaderBytecode mesh_shader
        +span~ShaderLibrary~ pixel_libraries
        +string_view entry_point
        +span~Format~ color_formats
        +Format depth_stencil_format
        +SampleCount samples
        +BlendState blend
        +RasterizerState rasterizer
        +DepthStencilState depth_stencil
    }
    class ComputePipelineDesc {
        +ShaderBytecode compute_shader
    }
    class RayTracingPipelineDesc {
        +ShaderBytecode ray_generation
        +span~ShaderBytecode~ miss_shaders
        +span~HitGroup~ hit_groups
        +uint32_t max_recursion_depth
        +uint32_t max_payload_size
        +uint32_t max_attribute_size
    }
    class HitGroup {
        +ShaderBytecode closest_hit
        +ShaderBytecode any_hit
        +ShaderBytecode intersection
    }
    class WorkGraphDesc {
        +ShaderBytecode state_object
        +string_view program_name
    }
    class BlendState {
        +span~Attachment~ attachments
    }
    class BlendAttachment {
        +bool blend_enable
        +BlendFactor src_color
        +BlendFactor dst_color
        +BlendOp color_op
        +BlendFactor src_alpha
        +BlendFactor dst_alpha
        +BlendOp alpha_op
        +ColorWriteMask write_mask
    }
    class RasterizerState {
        +CullMode cull_mode
        +FrontFace front_face
        +PolygonMode polygon_mode
        +bool depth_clamp
        +float depth_bias
        +float depth_bias_clamp
        +float depth_bias_slope
        +bool conservative_raster
    }
    class DepthStencilState {
        +bool depth_test_enable
        +bool depth_write_enable
        +CompareOp depth_compare
        +bool stencil_test_enable
        +StencilOpState front_stencil
        +StencilOpState back_stencil
    }
    class PipelineCacheDesc {
        +const void* initial_data
        +uint64_t initial_data_size
    }
    class SwapchainDesc {
        +void* native_window
        +uint32_t width
        +uint32_t height
        +Format format
        +uint32_t image_count
        +bool vsync
    }

    MeshRenderPipelineDesc --> ShaderBytecode
    MeshRenderPipelineDesc --> BlendState
    MeshRenderPipelineDesc --> RasterizerState
    MeshRenderPipelineDesc --> DepthStencilState
    LinkedMeshRenderPipelineDesc --> ShaderBytecode
    LinkedMeshRenderPipelineDesc --> ShaderLibrary
    LinkedMeshRenderPipelineDesc --> BlendState
    LinkedMeshRenderPipelineDesc --> RasterizerState
    LinkedMeshRenderPipelineDesc --> DepthStencilState
    ComputePipelineDesc --> ShaderBytecode
    RayTracingPipelineDesc --> ShaderBytecode
    RayTracingPipelineDesc o-- HitGroup
    HitGroup --> ShaderBytecode
    WorkGraphDesc --> ShaderBytecode
    BlendState o-- BlendAttachment
```

### 8. Resource Binding Types

`harmonius::gpu` -- Bindless descriptor heap and descriptor write operations.

```mermaid
classDiagram
    class DescriptorHeapDesc {
        +uint32_t max_descriptors
        +uint32_t max_samplers
    }
    class DescriptorType {
        <<enum class>>
        srv_texture
        srv_buffer
        uav_texture
        uav_buffer
        cbv
        sampler
        acceleration_structure
    }
    class DescriptorWrite {
        +DescriptorType type
        +uint32_t index
        +TextureHandle texture
        +Format format
        +uint32_t mip_level
        +uint32_t mip_count
        +uint32_t array_layer
        +uint32_t layer_count
        +BufferHandle buffer
        +uint64_t offset
        +uint64_t size
        +uint32_t structure_stride
        +AccelerationStructureHandle acceleration_structure
        +SamplerDesc sampler
    }

    DescriptorWrite --> DescriptorType
    DescriptorWrite --> TextureHandle
    DescriptorWrite --> BufferHandle
    DescriptorWrite --> AccelerationStructureHandle
```

### 9. Diagnostics Types

`harmonius::gpu` -- Query pools for timestamp and pipeline statistics.

```mermaid
classDiagram
    class QueryPoolDesc {
        +QueryType type
        +uint32_t count
    }
    class QueryType {
        <<enum class>>
        timestamp
        pipeline_statistics
    }

    QueryPoolDesc --> QueryType
```

### 10. Indirect Command Types

`harmonius::gpu` -- GPU-side argument structs for indirect dispatch and ray tracing.

```mermaid
classDiagram
    class DispatchMeshIndirectArgs {
        +uint32_t group_count_x
        +uint32_t group_count_y
        +uint32_t group_count_z
    }
    class DispatchIndirectArgs {
        +uint32_t group_count_x
        +uint32_t group_count_y
        +uint32_t group_count_z
    }
    class TraceRaysIndirectArgs {
        +uint32_t width
        +uint32_t height
        +uint32_t depth
    }
    class TraceRaysDesc {
        +BufferHandle raygen_sbt
        +uint64_t raygen_offset
        +uint64_t raygen_size
        +BufferHandle miss_sbt
        +uint64_t miss_offset
        +uint64_t miss_stride
        +uint64_t miss_size
        +BufferHandle hit_sbt
        +uint64_t hit_offset
        +uint64_t hit_stride
        +uint64_t hit_size
        +uint32_t width
        +uint32_t height
        +uint32_t depth
    }
    class DispatchGraphDesc {
        +uint32_t entry_point_index
        +const void* cpu_input
        +uint32_t cpu_input_size
        +BufferHandle gpu_input
        +uint64_t gpu_input_offset
        +uint32_t gpu_input_size
        +BufferHandle backing_memory
        +uint64_t backing_memory_offset
        +uint64_t backing_memory_size
    }

    TraceRaysDesc --> BufferHandle
    DispatchGraphDesc --> BufferHandle
```

### 11. Device Capabilities

`harmonius::gpu` -- Feature flags and hardware limits queried at initialization.
Consumed by the render graph's gating system to enable or disable passes.

```mermaid
classDiagram
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
        +uint32_t max_texture_dimension_2d
        +uint32_t max_texture_dimension_3d
        +uint32_t max_texture_array_layers
        +uint32_t max_buffer_size_bytes
        +uint32_t max_descriptor_count
        +uint32_t max_push_constants_bytes
        +uint32_t max_mesh_output_vertices
        +uint32_t max_mesh_output_primitives
        +uint32_t max_mesh_workgroup_size
        +uint32_t max_mesh_shared_memory_bytes
        +uint32_t max_task_workgroup_size
        +uint32_t max_task_shared_memory_bytes
        +uint32_t max_task_payload_bytes
        +uint64_t device_local_memory_bytes
        +uint64_t shared_memory_bytes
    }
```

### 12. Error Types

`harmonius::gpu` -- Error enumerations returned from device and pipeline operations.

```mermaid
classDiagram
    class ResourceError {
        <<enum class>>
        out_of_memory
        invalid_desc
        unsupported_format
    }
    class DeviceError {
        <<enum class>>
        device_lost
        out_of_memory
        initialization_failed
    }
    class PipelineError {
        <<enum class>>
        compilation_failed
        unsupported
        invalid_bytecode
    }
```

---

## Concepts

`harmonius::gpu` -- C++20 concepts defining the interface contracts. Each backend
provides a plain concrete class satisfying these concepts. No virtual dispatch, no
CRTP, no base classes. `static_assert` at the type alias site catches interface
violations at compile time.

```mermaid
classDiagram
    class GpuDevice {
        <<concept>>
        +capabilities() DeviceCapabilities
        +create_texture(TextureDesc) expected~TextureHandle~
        +create_buffer(BufferDesc) expected~BufferHandle~
        +create_heap(HeapDesc) expected~HeapHandle~
        +create_placed_texture(HeapHandle, uint64_t, TextureDesc) expected~TextureHandle~
        +create_placed_buffer(HeapHandle, uint64_t, BufferDesc) expected~BufferHandle~
        +create_sparse_texture(SparseTextureDesc) expected~TextureHandle~
        +create_fence(uint64_t) expected~FenceHandle~
        +create_command_pool(QueueType) CommandPool
        +submit(QueueType, cmd_bufs, signals, waits) void
        +create_mesh_render_pipeline(MeshRenderPipelineDesc) expected~PipelineHandle~
        +create_compute_pipeline(ComputePipelineDesc) expected~PipelineHandle~
        +create_ray_tracing_pipeline(RayTracingPipelineDesc) expected~PipelineHandle~
        +create_work_graph(WorkGraphDesc) expected~WorkGraphHandle~
        +create_descriptor_heap(DescriptorHeapDesc) expected~DescriptorHeapHandle~
        +create_swapchain(SwapchainDesc) expected~SwapchainHandle~
        +create_pipeline_cache(PipelineCacheDesc) expected~PipelineCacheHandle~
        +wait_idle() void
        +map(BufferHandle) void_ptr
        +unmap(BufferHandle) void
    }
    class GpuCommandBuffer {
        <<concept>>
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
    class GpuCommandPool {
        <<concept>>
        +allocate_command_buffer() CommandBuffer
        +reset() void
        +allocated_count() uint32_t
    }

    GpuDevice --> GpuCommandPool : creates
    GpuCommandPool --> GpuCommandBuffer : allocates
```

---

## Cross-Module Relationships

How the twelve type modules and three concepts connect. Arrows show which modules
depend on types from other modules.

```mermaid
classDiagram
    class Handles {
        <<module 1>>
    }
    class CoreEnums {
        <<module 2>>
    }
    class ResourceDescriptors {
        <<module 3>>
    }
    class AccelerationStructures {
        <<module 4>>
    }
    class SynchronizationTypes {
        <<module 5>>
    }
    class RenderPassTypes {
        <<module 6>>
    }
    class PipelineStateTypes {
        <<module 7>>
    }
    class ResourceBindingTypes {
        <<module 8>>
    }
    class DiagnosticsTypes {
        <<module 9>>
    }
    class IndirectCommandTypes {
        <<module 10>>
    }
    class DeviceCapabilities {
        <<module 11>>
    }
    class ErrorTypes {
        <<module 12>>
    }
    class GpuDevice {
        <<concept>>
    }
    class GpuCommandBuffer {
        <<concept>>
    }
    class GpuCommandPool {
        <<concept>>
    }

    ResourceDescriptors --> Handles : HeapHandle
    ResourceDescriptors --> CoreEnums : HeapType, TextureDimension, SampleCount
    AccelerationStructures --> Handles : BufferHandle, AccelerationStructureHandle
    SynchronizationTypes --> Handles : TextureHandle, BufferHandle, FenceHandle
    SynchronizationTypes --> CoreEnums : QueueType
    RenderPassTypes --> Handles : TextureHandle
    RenderPassTypes --> CoreEnums : LoadOp, StoreOp
    PipelineStateTypes --> CoreEnums : SampleCount
    ResourceBindingTypes --> Handles : TextureHandle, BufferHandle, AccelerationStructureHandle
    IndirectCommandTypes --> Handles : BufferHandle
    GpuDevice --> ResourceDescriptors : accepts descs
    GpuDevice --> Handles : returns handles
    GpuDevice --> ErrorTypes : returns errors
    GpuDevice --> DeviceCapabilities : exposes
    GpuDevice --> CoreEnums : QueueType
    GpuDevice --> PipelineStateTypes : accepts pipeline descs
    GpuDevice --> ResourceBindingTypes : accepts descriptor writes
    GpuDevice --> DiagnosticsTypes : creates query pools
    GpuCommandBuffer --> SynchronizationTypes : barrier()
    GpuCommandBuffer --> RenderPassTypes : begin_render_pass()
    GpuCommandBuffer --> Handles : PipelineHandle, WorkGraphHandle
    GpuCommandBuffer --> IndirectCommandTypes : trace_rays(), dispatch_graph()
    GpuCommandBuffer --> AccelerationStructures : build_acceleration_structure()
    GpuDevice --> GpuCommandPool : creates
    GpuCommandPool --> GpuCommandBuffer : allocates
```

### Handle-to-Module Ownership

Which concept methods create and destroy each handle type.

| Handle | Created By | Destroyed By |
|--------|-----------|-------------|
| `TextureHandle` | `GpuDevice::create_texture`, `create_placed_texture`, `create_sparse_texture` | `GpuDevice::destroy_texture` |
| `BufferHandle` | `GpuDevice::create_buffer`, `create_placed_buffer` | `GpuDevice::destroy_buffer` |
| `HeapHandle` | `GpuDevice::create_heap` | `GpuDevice::destroy_heap` |
| `AccelerationStructureHandle` | `GpuDevice::create_acceleration_structure` | `GpuDevice::destroy_acceleration_structure` |
| `FenceHandle` | `GpuDevice::create_fence` | `GpuDevice::destroy_fence` |
| `PipelineHandle` | `GpuDevice::create_mesh_render_pipeline`, `create_compute_pipeline`, `create_ray_tracing_pipeline` | `GpuDevice::destroy_pipeline` |
| `WorkGraphHandle` | `GpuDevice::create_work_graph` | `GpuDevice::destroy_work_graph` |
| `DescriptorHeapHandle` | `GpuDevice::create_descriptor_heap` | `GpuDevice::destroy_descriptor_heap` |
| `QueryPoolHandle` | `GpuDevice::create_query_pool` | `GpuDevice::destroy_query_pool` |
| `SwapchainHandle` | `GpuDevice::create_swapchain` | `GpuDevice::destroy_swapchain` |
| `PipelineCacheHandle` | `GpuDevice::create_pipeline_cache` | `GpuDevice::destroy_pipeline_cache` |

---

## Sequence Diagrams

### Backend Selection and Type Aliasing

How the build system selects a backend, aliases concrete types, and verifies them
against concepts at compile time.

```mermaid
sequenceDiagram
    participant CMake
    participant Compiler
    participant NS as gpu namespace
    participant SA as static_assert
    participant App

    Note over CMake,App: Build-time backend selection
    CMake->>Compiler: HARMONIUS_BACKEND_D3D12
    Compiler->>NS: Activate d3d12 branch

    Note over NS,SA: Type aliasing
    NS->>NS: using Device = D3D12Device
    NS->>NS: using CommandBuffer = D3D12CommandBuffer
    NS->>NS: using CommandPool = D3D12CommandPool

    Note over NS,SA: Concept verification
    NS->>SA: GpuDevice satisfied
    SA-->>NS: OK
    NS->>SA: GpuCommandBuffer satisfied
    SA-->>NS: OK
    NS->>SA: GpuCommandPool satisfied
    SA-->>NS: OK

    Note over App,NS: Direct static dispatch
    App->>NS: gpu::Device device(desc)
    NS-->>App: D3D12Device ctor (no vtable)
    App->>NS: device.create_texture(td)
    NS-->>App: Inlined via LTO
```

### Command Recording Lifecycle

Pool creation, command buffer allocation, recording, submission, and per-frame
reset.

```mermaid
sequenceDiagram
    participant Exec as Executor
    participant Dev as Device
    participant Pool as CommandPool
    participant Cmd as CommandBuffer
    participant GPU

    Exec->>Dev: create_command_pool(graphics)
    Dev-->>Exec: CommandPool

    Exec->>Pool: allocate_command_buffer()
    Pool-->>Exec: CommandBuffer

    Exec->>Cmd: begin()
    Cmd->>Cmd: barrier(pre_barriers)
    Cmd->>Cmd: begin_render_pass(rp_desc)
    Cmd->>Cmd: set_pipeline(mesh_pso)
    Cmd->>Cmd: push_constants(draw_data)
    Cmd->>Cmd: dispatch_mesh(x, y, z)
    Cmd->>Cmd: end_render_pass()
    Exec->>Cmd: end()

    Exec->>Dev: submit(graphics, [cmd], signals, waits)
    Dev->>GPU: Queue submit
    GPU-->>Dev: Fence signaled

    Exec->>Pool: reset()
```

### Cross-Queue Synchronization with Timeline Fences

Three queues coordinated via monotonically increasing timeline fence values.
Transfer uploads first, async compute waits on transfer, graphics waits on
compute. CPU paces frames by waiting on the graphics fence.

```mermaid
sequenceDiagram
    participant App as Executor
    participant GFX as Graphics Queue
    participant AC as Async Compute
    participant TX as Transfer Queue
    participant FA as Fence A
    participant FB as Fence B
    participant FC as Fence C

    Note over App,FC: Frame N

    App->>TX: submit(transfer, [upload_cmd])
    TX->>TX: Copy staging to device
    TX->>FC: signal(1)

    App->>AC: submit(compute, [cull_cmd], wait FC:1)
    FC-->>AC: wait satisfied
    AC->>AC: GPU culling pass
    AC->>FB: signal(1)

    App->>GFX: submit(graphics, [render_cmd], wait FB:1)
    FB-->>GFX: wait satisfied
    GFX->>GFX: Mesh shader rendering
    GFX->>FA: signal(1)

    App->>FA: wait_fence_cpu(1)
    FA-->>App: Frame N complete

    Note over App,FC: Frame N+1

    App->>TX: submit(transfer, signal FC:2)
    App->>AC: submit(compute, wait FC:2, signal FB:2)
    App->>GFX: submit(graphics, wait FB:2, signal FA:2)
```
