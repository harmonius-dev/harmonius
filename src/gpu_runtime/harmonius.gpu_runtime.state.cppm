/// @file harmonius.gpu_runtime.state.cppm
/// @brief GPU state tracking — tracked command buffers, resource state cache,
///        barrier optimization (batching, deduplication, elision).
///
/// Wraps gpu::CommandBuffer to filter redundant state-setting calls and
/// optimize barrier emission.
///
/// Requirements: GR-2.1–GR-2.7

export module harmonius.gpu_runtime.state;

import std;
import harmonius.gpu;

export namespace harmonius::gpu_runtime::state
{

  // ---------------------------------------------------------------------------
  // Resource state
  // ---------------------------------------------------------------------------

  struct ResourceState
  {
    gpu::PipelineStage stage = gpu::PipelineStage::none;
    gpu::ResourceAccess access = gpu::ResourceAccess::none;
    gpu::TextureLayout layout = gpu::TextureLayout::undefined;
    gpu::QueueType owner = gpu::QueueType::graphics;
  };

  // ---------------------------------------------------------------------------
  // Resource state cache
  // ---------------------------------------------------------------------------

  class ResourceStateCache
  {
  public:
    auto register_resource(gpu::TextureHandle handle, ResourceState initial) -> void;
    auto register_resource(gpu::BufferHandle handle, ResourceState initial) -> void;

    [[nodiscard]] auto current_state(gpu::TextureHandle handle) const -> ResourceState;
    [[nodiscard]] auto current_state(gpu::BufferHandle handle) const -> ResourceState;

    auto transition(gpu::TextureHandle handle, ResourceState new_state) -> void;
    auto transition(gpu::BufferHandle handle, ResourceState new_state) -> void;

    [[nodiscard]] auto needs_transition(gpu::TextureHandle handle,
                                        ResourceState target) const -> bool;
    [[nodiscard]] auto needs_transition(gpu::BufferHandle handle,
                                        ResourceState target) const -> bool;

    auto unregister(gpu::TextureHandle handle) -> void;
    auto unregister(gpu::BufferHandle handle) -> void;

  private:
    std::unordered_map<gpu::TextureHandle, ResourceState> texture_states_;
    std::unordered_map<gpu::BufferHandle, ResourceState> buffer_states_;
  };

  // ---------------------------------------------------------------------------
  // Barrier optimizer
  // ---------------------------------------------------------------------------

  class BarrierOptimizer
  {
  public:
    explicit BarrierOptimizer(ResourceStateCache &state_cache,
                              const gpu::DeviceCapabilities &caps);

    auto enqueue(const gpu::BarrierDesc &desc) -> void;

    [[nodiscard]] auto flush() -> std::vector<gpu::BarrierDesc>;

    auto split_begin(const gpu::BarrierDesc &desc) -> void;
    auto split_end(const gpu::BarrierDesc &desc) -> void;

  private:
    ResourceStateCache &state_cache_;
    const gpu::DeviceCapabilities &caps_;
    std::vector<gpu::BarrierDesc> pending_;
    std::vector<gpu::BarrierDesc> deferred_splits_;
  };

  // ---------------------------------------------------------------------------
  // Tracked command buffer
  // ---------------------------------------------------------------------------

  class TrackedCommandBuffer
  {
  public:
    auto set_pipeline(gpu::PipelineHandle handle) -> void;
    auto bind_descriptor_heap(gpu::DescriptorHeapHandle handle) -> void;
    auto set_viewport(const gpu::Viewport &vp) -> void;
    auto set_scissor(const gpu::Scissor &rect) -> void;
    auto push_constants(const void *data, std::uint32_t size,
                        std::uint32_t offset = 0) -> void;

    auto barrier(const gpu::BarrierDesc &desc) -> void;
    auto flush_barriers() -> void;

    auto begin() -> void;
    auto end() -> void;
    auto begin_render_pass(const gpu::RenderPassDesc &desc) -> void;
    auto end_render_pass() -> void;
    auto dispatch_mesh(std::uint32_t x, std::uint32_t y, std::uint32_t z) -> void;
    auto dispatch(std::uint32_t x, std::uint32_t y, std::uint32_t z) -> void;
    auto dispatch_indirect(gpu::BufferHandle args, std::uint64_t offset) -> void;
    auto trace_rays(const gpu::TraceRaysDesc &desc) -> void;
    auto build_acceleration_structure(const gpu::AccelerationStructureBuildDesc &desc) -> void;
    auto copy_buffer(gpu::BufferHandle src, std::uint64_t src_offset,
                     gpu::BufferHandle dst, std::uint64_t dst_offset,
                     std::uint64_t size) -> void;
    auto copy_buffer_to_texture(gpu::BufferHandle src, std::uint64_t src_offset,
                                gpu::TextureHandle dst,
                                const gpu::TextureSubresource &sub,
                                gpu::Extent3D extent) -> void;
    auto copy_texture_to_buffer(gpu::TextureHandle src,
                                const gpu::TextureSubresource &sub,
                                gpu::BufferHandle dst, std::uint64_t dst_offset,
                                gpu::Extent3D extent) -> void;
    auto copy_texture(gpu::TextureHandle src, const gpu::TextureSubresource &src_sub,
                      gpu::TextureHandle dst, const gpu::TextureSubresource &dst_sub,
                      gpu::Extent3D extent) -> void;
    auto begin_debug_label(std::string_view name) -> void;
    auto end_debug_label() -> void;

  private:
    gpu::PipelineHandle bound_pipeline_ = gpu::PipelineHandle::invalid;
    gpu::DescriptorHeapHandle bound_heap_ = gpu::DescriptorHeapHandle::invalid;
    std::optional<gpu::Viewport> current_viewport_;
    std::optional<gpu::Scissor> current_scissor_;
    std::array<std::uint8_t, 128> push_constant_cache_ = {};
    std::uint32_t push_constant_size_ = 0;
    std::vector<gpu::BarrierDesc> pending_barriers_;
  };

} // namespace harmonius::gpu_runtime::state
