module;

#include <d3d12.h>
#include <dxgi1_6.h>

export module harmonius.gpu.d3d12;

import harmonius.gpu;
import harmonius.gpu.concepts;

import std;

export namespace harmonius::gpu::d3d12 {

// ---------------------------------------------------------------------------
// Forward declarations
// ---------------------------------------------------------------------------

class D3D12CommandBuffer;

// ---------------------------------------------------------------------------
// D3D12CommandPool
// ---------------------------------------------------------------------------

/// A pool that allocates and recycles D3D12 command buffers.
/// @threadsafety Instances are not thread-safe.
class D3D12CommandPool {
 public:
    using CommandBufferType = D3D12CommandBuffer;

    /// Allocates a new command buffer from this pool.
    [[nodiscard]] auto AllocateCommandBuffer() -> D3D12CommandBuffer;

    /// Resets all command buffers allocated from this pool.
    auto Reset() -> void;

    /// Returns the number of command buffers currently allocated.
    [[nodiscard]] auto AllocatedCount() const -> std::uint32_t;

    D3D12CommandPool(const D3D12CommandPool&) = delete;
    auto operator=(const D3D12CommandPool&) -> D3D12CommandPool& = delete;
    D3D12CommandPool(D3D12CommandPool&&) noexcept;
    auto operator=(D3D12CommandPool&&) noexcept -> D3D12CommandPool&;
    ~D3D12CommandPool();
};

// ---------------------------------------------------------------------------
// D3D12CommandBuffer
// ---------------------------------------------------------------------------

/// A D3D12 command buffer that records GPU commands for submission.
/// @threadsafety Instances are not thread-safe.
class D3D12CommandBuffer {
 public:
    /// Begins recording commands into this command buffer.
    auto Begin() -> void;

    /// Ends recording commands into this command buffer.
    auto End() -> void;

    /// Inserts pipeline barriers for resource synchronization.
    auto Barrier(const BarrierDesc& desc) -> void;

    /// Begins a render pass with the specified attachments.
    auto BeginRenderPass(const RenderPassDesc& desc) -> void;

    /// Ends the current render pass.
    auto EndRenderPass() -> void;

    /// Binds a pipeline state object for subsequent draw or dispatch calls.
    auto SetPipeline(PipelineHandle pipeline) -> void;

    /// Dispatches mesh shader workgroups.
    auto DispatchMesh(std::uint32_t group_count_x, std::uint32_t group_count_y,
                      std::uint32_t group_count_z) -> void;

    /// Dispatches mesh shader workgroups using indirect arguments from a buffer.
    auto DispatchMeshIndirect(BufferHandle args, std::uint64_t offset, std::uint32_t draw_count,
                              std::uint32_t stride) -> void;

    /// Dispatches mesh shader workgroups with an indirect count from a buffer.
    auto DispatchMeshIndirectCount(BufferHandle args, std::uint64_t arg_offset, BufferHandle count_buffer,
                                   std::uint64_t count_offset, std::uint32_t max_draw_count,
                                   std::uint32_t stride) -> void;

    /// Dispatches compute shader workgroups.
    auto Dispatch(std::uint32_t group_count_x, std::uint32_t group_count_y,
                  std::uint32_t group_count_z) -> void;

    /// Dispatches compute shader workgroups using indirect arguments from a buffer.
    auto DispatchIndirect(BufferHandle args, std::uint64_t offset) -> void;

    /// Dispatches ray tracing work.
    auto TraceRays(const TraceRaysDesc& desc) -> void;

    /// Dispatches ray tracing work using indirect arguments from a buffer.
    auto TraceRaysIndirect(BufferHandle args, std::uint64_t offset) -> void;

    /// Builds or updates a ray tracing acceleration structure.
    auto BuildAccelerationStructure(const AccelerationStructureBuildDesc& desc) -> void;

    /// Sets the active work graph program for subsequent dispatches.
    auto SetWorkGraph(WorkGraphHandle handle) -> void;

    /// Dispatches a work graph program.
    auto DispatchGraph(const DispatchGraphDesc& desc) -> void;

    /// Copies data between two GPU buffers.
    auto CopyBuffer(BufferHandle src, std::uint64_t src_offset, BufferHandle dst,
                    std::uint64_t dst_offset, std::uint64_t size) -> void;

    /// Copies a region between two GPU textures.
    auto CopyTexture(TextureHandle src, const TextureSubresource& src_sub, TextureHandle dst,
                     const TextureSubresource& dst_sub, Extent3D extent) -> void;

    /// Copies data from a GPU buffer to a GPU texture.
    auto CopyBufferToTexture(BufferHandle src, std::uint64_t src_offset, TextureHandle dst,
                             const TextureSubresource& subresource, Extent3D extent) -> void;

    /// Copies data from a GPU texture to a GPU buffer.
    auto CopyTextureToBuffer(TextureHandle src, const TextureSubresource& subresource, BufferHandle dst,
                             std::uint64_t dst_offset, Extent3D extent) -> void;

    /// Sets the viewport rectangle.
    auto SetViewport(const Viewport& viewport) -> void;

    /// Sets the scissor rectangle.
    auto SetScissor(const Scissor& scissor) -> void;

    /// Uploads push constant data to the pipeline.
    auto PushConstants(const void* data, std::uint32_t size, std::uint32_t offset) -> void;

    /// Binds a descriptor heap for bindless resource access.
    auto BindDescriptorHeap(DescriptorHeapHandle heap) -> void;

    /// Writes a GPU timestamp into a query pool slot.
    auto WriteTimestamp(QueryPoolHandle pool, std::uint32_t index) -> void;

    /// Resolves query results from a query pool into a buffer.
    auto ResolveQueryPool(QueryPoolHandle pool, std::uint32_t first_query, std::uint32_t query_count,
                          BufferHandle dst, std::uint64_t dst_offset) -> void;

    /// Begins a debug label region in the command buffer.
    auto BeginDebugLabel(std::string_view label) -> void;

    /// Ends the current debug label region.
    auto EndDebugLabel() -> void;

    /// Inserts a single debug label marker.
    auto InsertDebugLabel(std::string_view label) -> void;

    D3D12CommandBuffer(const D3D12CommandBuffer&) = delete;
    auto operator=(const D3D12CommandBuffer&) -> D3D12CommandBuffer& = delete;
    D3D12CommandBuffer(D3D12CommandBuffer&&) noexcept;
    auto operator=(D3D12CommandBuffer&&) noexcept -> D3D12CommandBuffer&;
    ~D3D12CommandBuffer();
};

// ---------------------------------------------------------------------------
// D3D12Device
// ---------------------------------------------------------------------------

/// A D3D12 GPU device that manages resources, pipelines, and command submission.
/// @threadsafety Instances are not thread-safe.
class D3D12Device {
 public:
    using CommandBufferType = D3D12CommandBuffer;
    using CommandPoolType = D3D12CommandPool;

    /// Returns the capabilities and limits of this GPU device.
    [[nodiscard]] auto Capabilities() const -> DeviceCapabilities;

    /// Returns the number of queues available for the given queue type.
    [[nodiscard]] auto QueueCount(QueueType type) const -> std::uint32_t;

    /// Creates a GPU texture resource.
    [[nodiscard]] auto CreateTexture(const TextureDesc& desc)
        -> std::expected<TextureHandle, ResourceError>;

    /// Destroys a GPU texture resource.
    auto DestroyTexture(TextureHandle handle) -> void;

    /// Creates a GPU buffer resource.
    [[nodiscard]] auto CreateBuffer(const BufferDesc& desc)
        -> std::expected<BufferHandle, ResourceError>;

    /// Destroys a GPU buffer resource.
    auto DestroyBuffer(BufferHandle handle) -> void;

    /// Creates a GPU memory heap.
    [[nodiscard]] auto CreateHeap(const HeapDesc& desc) -> std::expected<HeapHandle, ResourceError>;

    /// Destroys a GPU memory heap.
    auto DestroyHeap(HeapHandle handle) -> void;

    /// Creates a texture placed in an existing heap at the given offset.
    [[nodiscard]] auto CreatePlacedTexture(HeapHandle heap, std::uint64_t offset, const TextureDesc& desc)
        -> std::expected<TextureHandle, ResourceError>;

    /// Creates a buffer placed in an existing heap at the given offset.
    [[nodiscard]] auto CreatePlacedBuffer(HeapHandle heap, std::uint64_t offset, const BufferDesc& desc)
        -> std::expected<BufferHandle, ResourceError>;

    /// Queries the allocation size and alignment for a texture descriptor.
    [[nodiscard]] auto QueryTextureAllocationInfo(const TextureDesc& desc) const -> AllocationInfo;

    /// Queries the allocation size and alignment for a buffer descriptor.
    [[nodiscard]] auto QueryBufferAllocationInfo(const BufferDesc& desc) const -> AllocationInfo;

    /// Creates a sparse (tiled) GPU texture.
    [[nodiscard]] auto CreateSparseTexture(const SparseTextureDesc& desc)
        -> std::expected<TextureHandle, ResourceError>;

    /// Updates sparse tile bindings for a texture.
    auto UpdateSparseBindings(TextureHandle texture, std::span<const SparseTileBinding> bindings) -> void;

    /// Maps a buffer for CPU access and returns a pointer to the mapped memory.
    [[nodiscard]] auto Map(BufferHandle handle) -> void*;

    /// Unmaps a previously mapped buffer.
    auto Unmap(BufferHandle handle) -> void;

    /// Creates a ray tracing acceleration structure.
    [[nodiscard]] auto CreateAccelerationStructure(const AccelerationStructureDesc& desc)
        -> std::expected<AccelerationStructureHandle, ResourceError>;

    /// Queries the memory requirements for building an acceleration structure.
    [[nodiscard]] auto QueryAccelerationStructureSizes(const AccelerationStructureDesc& desc) const
        -> AccelerationStructureSizes;

    /// Destroys a ray tracing acceleration structure.
    auto DestroyAccelerationStructure(AccelerationStructureHandle handle) -> void;

    /// Creates a timeline fence with the given initial value.
    [[nodiscard]] auto CreateFence(std::uint64_t initial_value)
        -> std::expected<FenceHandle, DeviceError>;

    /// Destroys a timeline fence.
    auto DestroyFence(FenceHandle handle) -> void;

    /// Returns the last completed value of a timeline fence.
    [[nodiscard]] auto FenceCompletedValue(FenceHandle handle) const -> std::uint64_t;

    /// Blocks the CPU until a fence reaches the specified value.
    auto WaitFenceCpu(FenceHandle handle, std::uint64_t value) -> void;

    /// Creates a command pool for the given queue type.
    [[nodiscard]] auto CreateCommandPool(QueueType type) -> D3D12CommandPool;

    /// Submits command buffers to a queue with fence signal and wait operations.
    auto Submit(QueueType type, std::span<D3D12CommandBuffer*> command_buffers,
                std::span<const FenceSignal> signal_fences, std::span<const FenceWait> wait_fences) -> void;

    /// Creates a mesh-shader render pipeline.
    [[nodiscard]] auto CreateMeshRenderPipeline(const MeshRenderPipelineDesc& desc)
        -> std::expected<PipelineHandle, PipelineError>;

    /// Creates a compute pipeline.
    [[nodiscard]] auto CreateComputePipeline(const ComputePipelineDesc& desc)
        -> std::expected<PipelineHandle, PipelineError>;

    /// Creates a ray tracing pipeline.
    [[nodiscard]] auto CreateRayTracingPipeline(const RayTracingPipelineDesc& desc)
        -> std::expected<PipelineHandle, PipelineError>;

    /// Destroys a pipeline state object.
    auto DestroyPipeline(PipelineHandle handle) -> void;

    /// Creates a work graph program.
    [[nodiscard]] auto CreateWorkGraph(const WorkGraphDesc& desc)
        -> std::expected<WorkGraphHandle, PipelineError>;

    /// Destroys a work graph program.
    auto DestroyWorkGraph(WorkGraphHandle handle) -> void;

    /// Creates a bindless descriptor heap.
    [[nodiscard]] auto CreateDescriptorHeap(const DescriptorHeapDesc& desc)
        -> std::expected<DescriptorHeapHandle, DeviceError>;

    /// Writes a descriptor into a descriptor heap slot.
    auto WriteDescriptor(DescriptorHeapHandle heap, std::uint32_t index,
                         const DescriptorWrite& write) -> void;

    /// Destroys a descriptor heap.
    auto DestroyDescriptorHeap(DescriptorHeapHandle handle) -> void;

    /// Creates a query pool for timestamps or pipeline statistics.
    [[nodiscard]] auto CreateQueryPool(const QueryPoolDesc& desc)
        -> std::expected<QueryPoolHandle, DeviceError>;

    /// Destroys a query pool.
    auto DestroyQueryPool(QueryPoolHandle handle) -> void;

    /// Returns the timestamp period in nanoseconds per tick.
    [[nodiscard]] auto TimestampPeriodNs() const -> double;

    /// Creates a swapchain for presenting to a window.
    [[nodiscard]] auto CreateSwapchain(const SwapchainDesc& desc)
        -> std::expected<SwapchainHandle, DeviceError>;

    /// Acquires the next presentable image from a swapchain.
    [[nodiscard]] auto AcquireNextImage(SwapchainHandle swapchain)
        -> std::expected<TextureHandle, DeviceError>;

    /// Presents a swapchain image to the display.
    auto Present(SwapchainHandle swapchain) -> void;

    /// Resizes a swapchain to new dimensions.
    auto ResizeSwapchain(SwapchainHandle swapchain, std::uint32_t width, std::uint32_t height) -> void;

    /// Destroys a swapchain.
    auto DestroySwapchain(SwapchainHandle handle) -> void;

    /// Creates a pipeline cache for accelerating pipeline compilation.
    [[nodiscard]] auto CreatePipelineCache(const PipelineCacheDesc& desc)
        -> std::expected<PipelineCacheHandle, DeviceError>;

    /// Serializes the contents of a pipeline cache to a byte vector.
    [[nodiscard]] auto SerializePipelineCache(PipelineCacheHandle handle) -> std::vector<std::uint8_t>;

    /// Destroys a pipeline cache.
    auto DestroyPipelineCache(PipelineCacheHandle handle) -> void;

    /// Sets a debug name on a texture resource.
    auto SetName(TextureHandle handle, std::string_view name) -> void;

    /// Sets a debug name on a buffer resource.
    auto SetName(BufferHandle handle, std::string_view name) -> void;

    /// Sets a debug name on an acceleration structure.
    auto SetName(AccelerationStructureHandle handle, std::string_view name) -> void;

    /// Sets a debug name on a pipeline state object.
    auto SetName(PipelineHandle handle, std::string_view name) -> void;

    /// Sets a debug name on a fence.
    auto SetName(FenceHandle handle, std::string_view name) -> void;

    /// Blocks until all GPU work has completed.
    auto WaitIdle() -> void;

    D3D12Device(const D3D12Device&) = delete;
    auto operator=(const D3D12Device&) -> D3D12Device& = delete;
    D3D12Device(D3D12Device&&) noexcept;
    auto operator=(D3D12Device&&) noexcept -> D3D12Device&;
    ~D3D12Device();
};

// ---------------------------------------------------------------------------
// Concept verification
// ---------------------------------------------------------------------------

static_assert(GpuCommandPool<D3D12CommandPool>);
static_assert(GpuCommandBuffer<D3D12CommandBuffer>);
static_assert(GpuDevice<D3D12Device>);

}  // namespace harmonius::gpu::d3d12
