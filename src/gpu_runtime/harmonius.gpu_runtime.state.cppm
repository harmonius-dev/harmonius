/// @file harmonius.gpu_runtime.state.cppm
/// @brief GPU state tracking — tracked command buffers, resource state cache,
///        barrier optimization (batching, deduplication, elision).
///
/// Wraps gpu::CommandBuffer to filter redundant state-setting calls and
/// optimize barrier emission.
///
/// Requirements: GR-2.1–GR-2.7

export module harmonius.gpu_runtime.state;

import harmonius.gpu;

import std;

export namespace harmonius::gpu_runtime::state {

// ---------------------------------------------------------------------------
// Resource state
// ---------------------------------------------------------------------------

/// Describes the current pipeline stage, access, layout, and queue ownership of a resource.
struct ResourceState {
    gpu::PipelineStage stage = gpu::PipelineStage::kNone;
    gpu::ResourceAccess access = gpu::ResourceAccess::kNone;
    gpu::TextureLayout layout = gpu::TextureLayout::kUndefined;
    gpu::QueueType owner = gpu::QueueType::kGraphics;
};

// ---------------------------------------------------------------------------
// Resource state cache
// ---------------------------------------------------------------------------

/// Tracks the current state of every registered GPU resource.
/// @threadsafety Instances are not thread-safe.
class ResourceStateCache {
 public:
    /// Register a texture with its initial state.
    auto RegisterResource(gpu::TextureHandle handle, ResourceState initial) -> void;

    /// Register a buffer with its initial state.
    auto RegisterResource(gpu::BufferHandle handle, ResourceState initial) -> void;

    /// Returns the current state of the given texture.
    [[nodiscard]] auto CurrentState(gpu::TextureHandle handle) const -> ResourceState;

    /// Returns the current state of the given buffer.
    [[nodiscard]] auto CurrentState(gpu::BufferHandle handle) const -> ResourceState;

    /// Transition a texture to a new state.
    auto Transition(gpu::TextureHandle handle, ResourceState new_state) -> void;

    /// Transition a buffer to a new state.
    auto Transition(gpu::BufferHandle handle, ResourceState new_state) -> void;

    /// Returns true if the texture needs a state transition to reach the target.
    [[nodiscard]] auto NeedsTransition(gpu::TextureHandle handle, ResourceState target) const -> bool;

    /// Returns true if the buffer needs a state transition to reach the target.
    [[nodiscard]] auto NeedsTransition(gpu::BufferHandle handle, ResourceState target) const -> bool;

    /// Unregister a texture from the cache.
    auto Unregister(gpu::TextureHandle handle) -> void;

    /// Unregister a buffer from the cache.
    auto Unregister(gpu::BufferHandle handle) -> void;

 private:
    std::unordered_map<gpu::TextureHandle, ResourceState> texture_states_;
    std::unordered_map<gpu::BufferHandle, ResourceState> buffer_states_;
};

// ---------------------------------------------------------------------------
// Barrier optimizer
// ---------------------------------------------------------------------------

/// Batches, deduplicates, and elides GPU barriers for optimal submission.
/// @threadsafety Instances are not thread-safe.
class BarrierOptimizer {
 public:
    explicit BarrierOptimizer(ResourceStateCache& state_cache, const gpu::DeviceCapabilities& caps);

    /// Enqueue a barrier for deferred batch submission.
    auto Enqueue(const gpu::BarrierDesc& desc) -> void;

    /// Flush all pending barriers, returning the optimized batch.
    [[nodiscard]] auto Flush() -> std::vector<gpu::BarrierDesc>;

    /// Begin a split barrier.
    auto SplitBegin(const gpu::BarrierDesc& desc) -> void;

    /// End a split barrier.
    auto SplitEnd(const gpu::BarrierDesc& desc) -> void;

 private:
    ResourceStateCache& state_cache_;
    const gpu::DeviceCapabilities& caps_;
    std::vector<gpu::BarrierDesc> pending_;
    std::vector<gpu::BarrierDesc> deferred_splits_;
};

// ---------------------------------------------------------------------------
// Tracked command buffer
// ---------------------------------------------------------------------------

/// Command buffer wrapper that filters redundant state changes and batches barriers.
/// @threadsafety Instances are not thread-safe.
class TrackedCommandBuffer {
 public:
    /// Bind a pipeline, skipping if already bound.
    auto SetPipeline(gpu::PipelineHandle handle) -> void;

    /// Bind a descriptor heap, skipping if already bound.
    auto BindDescriptorHeap(gpu::DescriptorHeapHandle handle) -> void;

    /// Set the viewport, skipping if unchanged.
    auto SetViewport(const gpu::Viewport& vp) -> void;

    /// Set the scissor rectangle, skipping if unchanged.
    auto SetScissor(const gpu::Scissor& rect) -> void;

    /// Set push constant data.
    auto PushConstants(const void* data, std::uint32_t size, std::uint32_t offset = 0) -> void;

    /// Enqueue a barrier for deferred submission.
    auto Barrier(const gpu::BarrierDesc& desc) -> void;

    /// Flush all pending barriers to the underlying command buffer.
    auto FlushBarriers() -> void;

    /// Begin recording commands.
    auto Begin() -> void;

    /// End recording commands.
    auto End() -> void;

    /// Begin a render pass.
    auto BeginRenderPass(const gpu::RenderPassDesc& desc) -> void;

    /// End the current render pass.
    auto EndRenderPass() -> void;

    /// Dispatch a mesh shader workload.
    auto DispatchMesh(std::uint32_t x, std::uint32_t y, std::uint32_t z) -> void;

    /// Dispatch a compute shader workload.
    auto Dispatch(std::uint32_t x, std::uint32_t y, std::uint32_t z) -> void;

    /// Dispatch a compute shader workload with indirect arguments.
    auto DispatchIndirect(gpu::BufferHandle args, std::uint64_t offset) -> void;

    /// Dispatch a ray tracing workload.
    auto TraceRays(const gpu::TraceRaysDesc& desc) -> void;

    /// Build or update an acceleration structure.
    auto BuildAccelerationStructure(const gpu::AccelerationStructureBuildDesc& desc) -> void;

    /// Copy data between buffers.
    auto CopyBuffer(gpu::BufferHandle src, std::uint64_t src_offset, gpu::BufferHandle dst,
                    std::uint64_t dst_offset, std::uint64_t size) -> void;

    /// Copy data from a buffer to a texture.
    auto CopyBufferToTexture(gpu::BufferHandle src, std::uint64_t src_offset, gpu::TextureHandle dst,
                             const gpu::TextureSubresource& sub, gpu::Extent3D extent) -> void;

    /// Copy data from a texture to a buffer.
    auto CopyTextureToBuffer(gpu::TextureHandle src, const gpu::TextureSubresource& sub,
                             gpu::BufferHandle dst, std::uint64_t dst_offset,
                             gpu::Extent3D extent) -> void;

    /// Copy data between textures.
    auto CopyTexture(gpu::TextureHandle src, const gpu::TextureSubresource& src_sub,
                     gpu::TextureHandle dst, const gpu::TextureSubresource& dst_sub,
                     gpu::Extent3D extent) -> void;

    /// Begin a debug label region.
    auto BeginDebugLabel(std::string_view name) -> void;

    /// End the current debug label region.
    auto EndDebugLabel() -> void;

 private:
    gpu::PipelineHandle bound_pipeline_ = gpu::PipelineHandle::kInvalid;
    gpu::DescriptorHeapHandle bound_heap_ = gpu::DescriptorHeapHandle::kInvalid;
    std::optional<gpu::Viewport> current_viewport_;
    std::optional<gpu::Scissor> current_scissor_;
    std::array<std::uint8_t, 128> push_constant_cache_ = {};
    std::uint32_t push_constant_size_ = 0;
    std::vector<gpu::BarrierDesc> pending_barriers_;
};

}  // namespace harmonius::gpu_runtime::state
