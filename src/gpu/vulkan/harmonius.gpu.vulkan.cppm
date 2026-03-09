module;

#include <vulkan/vulkan.h>

export module harmonius.gpu.vulkan;

import std;
import harmonius.gpu;
import harmonius.gpu.concepts;

export namespace harmonius::gpu::vulkan
{

  // ---------------------------------------------------------------------------
  // Forward declarations
  // ---------------------------------------------------------------------------

  class VulkanCommandBuffer;

  // ---------------------------------------------------------------------------
  // VulkanCommandPool
  // ---------------------------------------------------------------------------

  class VulkanCommandPool
  {
  public:
    using CommandBufferType = VulkanCommandBuffer;

    auto allocate_command_buffer() -> VulkanCommandBuffer;
    auto reset() -> void;
    auto allocated_count() const -> std::uint32_t;
  };

  // ---------------------------------------------------------------------------
  // VulkanCommandBuffer
  // ---------------------------------------------------------------------------

  class VulkanCommandBuffer
  {
  public:
    auto begin() -> void;
    auto end() -> void;

    auto barrier(const BarrierDesc &desc) -> void;

    auto begin_render_pass(const RenderPassDesc &desc) -> void;
    auto end_render_pass() -> void;

    auto set_pipeline(PipelineHandle pipeline) -> void;

    // Mesh shader dispatch
    auto dispatch_mesh(std::uint32_t group_count_x,
                       std::uint32_t group_count_y,
                       std::uint32_t group_count_z) -> void;
    auto dispatch_mesh_indirect(BufferHandle args,
                                std::uint64_t offset,
                                std::uint32_t draw_count,
                                std::uint32_t stride) -> void;
    auto dispatch_mesh_indirect_count(BufferHandle args,
                                      std::uint64_t arg_offset,
                                      BufferHandle count_buffer,
                                      std::uint64_t count_offset,
                                      std::uint32_t max_draw_count,
                                      std::uint32_t stride) -> void;

    // Compute dispatch
    auto dispatch(std::uint32_t group_count_x,
                  std::uint32_t group_count_y,
                  std::uint32_t group_count_z) -> void;
    auto dispatch_indirect(BufferHandle args, std::uint64_t offset) -> void;

    // Ray tracing
    auto trace_rays(const TraceRaysDesc &desc) -> void;
    auto trace_rays_indirect(BufferHandle args, std::uint64_t offset) -> void;

    // Acceleration structures
    auto build_acceleration_structure(const AccelerationStructureBuildDesc &desc) -> void;

    // Work graphs
    auto set_work_graph(WorkGraphHandle handle) -> void;
    auto dispatch_graph(const DispatchGraphDesc &desc) -> void;

    // Copy commands
    auto copy_buffer(BufferHandle src, std::uint64_t src_offset,
                     BufferHandle dst, std::uint64_t dst_offset,
                     std::uint64_t size) -> void;
    auto copy_texture(TextureHandle src, const TextureSubresource &src_sub,
                      TextureHandle dst, const TextureSubresource &dst_sub,
                      Extent3D extent) -> void;
    auto copy_buffer_to_texture(BufferHandle src, std::uint64_t src_offset,
                                TextureHandle dst,
                                const TextureSubresource &subresource,
                                Extent3D extent) -> void;
    auto copy_texture_to_buffer(TextureHandle src,
                                const TextureSubresource &subresource,
                                BufferHandle dst, std::uint64_t dst_offset,
                                Extent3D extent) -> void;

    // Viewport and scissor
    auto set_viewport(const Viewport &viewport) -> void;
    auto set_scissor(const Scissor &scissor) -> void;

    // Push constants
    auto push_constants(const void *data, std::uint32_t size,
                        std::uint32_t offset) -> void;

    // Descriptor binding
    auto bind_descriptor_heap(DescriptorHeapHandle heap) -> void;

    // Queries
    auto write_timestamp(QueryPoolHandle pool, std::uint32_t index) -> void;
    auto resolve_query_pool(QueryPoolHandle pool,
                            std::uint32_t first_query,
                            std::uint32_t query_count,
                            BufferHandle dst,
                            std::uint64_t dst_offset) -> void;

    // Debug labels
    auto begin_debug_label(std::string_view label) -> void;
    auto end_debug_label() -> void;
    auto insert_debug_label(std::string_view label) -> void;
  };

  // ---------------------------------------------------------------------------
  // VulkanDevice
  // ---------------------------------------------------------------------------

  class VulkanDevice
  {
  public:
    using CommandBufferType = VulkanCommandBuffer;
    using CommandPoolType = VulkanCommandPool;

    // Queries
    auto capabilities() const -> DeviceCapabilities;
    auto queue_count(QueueType type) const -> std::uint32_t;

    // Texture
    auto create_texture(const TextureDesc &desc)
        -> std::expected<TextureHandle, ResourceError>;
    auto destroy_texture(TextureHandle handle) -> void;

    // Buffer
    auto create_buffer(const BufferDesc &desc)
        -> std::expected<BufferHandle, ResourceError>;
    auto destroy_buffer(BufferHandle handle) -> void;

    // Heap
    auto create_heap(const HeapDesc &desc)
        -> std::expected<HeapHandle, ResourceError>;
    auto destroy_heap(HeapHandle handle) -> void;

    // Placed resources
    auto create_placed_texture(HeapHandle heap, std::uint64_t offset,
                               const TextureDesc &desc)
        -> std::expected<TextureHandle, ResourceError>;
    auto create_placed_buffer(HeapHandle heap, std::uint64_t offset,
                              const BufferDesc &desc)
        -> std::expected<BufferHandle, ResourceError>;

    // Allocation info queries
    auto query_texture_allocation_info(const TextureDesc &desc) const -> AllocationInfo;
    auto query_buffer_allocation_info(const BufferDesc &desc) const -> AllocationInfo;

    // Sparse resources
    auto create_sparse_texture(const SparseTextureDesc &desc)
        -> std::expected<TextureHandle, ResourceError>;
    auto update_sparse_bindings(TextureHandle texture,
                                std::span<const SparseTileBinding> bindings) -> void;

    // Memory mapping
    auto map(BufferHandle handle) -> void *;
    auto unmap(BufferHandle handle) -> void;

    // Acceleration structures
    auto create_acceleration_structure(const AccelerationStructureDesc &desc)
        -> std::expected<AccelerationStructureHandle, ResourceError>;
    auto query_acceleration_structure_sizes(const AccelerationStructureDesc &desc) const
        -> AccelerationStructureSizes;
    auto destroy_acceleration_structure(AccelerationStructureHandle handle) -> void;

    // Fences
    auto create_fence(std::uint64_t initial_value)
        -> std::expected<FenceHandle, DeviceError>;
    auto destroy_fence(FenceHandle handle) -> void;
    auto fence_completed_value(FenceHandle handle) const -> std::uint64_t;
    auto wait_fence_cpu(FenceHandle handle, std::uint64_t value) -> void;

    // Command pools
    auto create_command_pool(QueueType type) -> VulkanCommandPool;

    // Submission
    auto submit(QueueType type,
                std::span<VulkanCommandBuffer *> command_buffers,
                std::span<const FenceSignal> signal_fences,
                std::span<const FenceWait> wait_fences) -> void;

    // Pipelines
    auto create_mesh_render_pipeline(const MeshRenderPipelineDesc &desc)
        -> std::expected<PipelineHandle, PipelineError>;
    auto create_compute_pipeline(const ComputePipelineDesc &desc)
        -> std::expected<PipelineHandle, PipelineError>;
    auto create_ray_tracing_pipeline(const RayTracingPipelineDesc &desc)
        -> std::expected<PipelineHandle, PipelineError>;
    auto destroy_pipeline(PipelineHandle handle) -> void;

    // Work graphs
    auto create_work_graph(const WorkGraphDesc &desc)
        -> std::expected<WorkGraphHandle, PipelineError>;
    auto destroy_work_graph(WorkGraphHandle handle) -> void;

    // Descriptor heaps
    auto create_descriptor_heap(const DescriptorHeapDesc &desc)
        -> std::expected<DescriptorHeapHandle, DeviceError>;
    auto write_descriptor(DescriptorHeapHandle heap,
                          std::uint32_t index,
                          const DescriptorWrite &write) -> void;
    auto destroy_descriptor_heap(DescriptorHeapHandle handle) -> void;

    // Query pools
    auto create_query_pool(const QueryPoolDesc &desc)
        -> std::expected<QueryPoolHandle, DeviceError>;
    auto destroy_query_pool(QueryPoolHandle handle) -> void;
    auto timestamp_period_ns() const -> double;

    // Swapchain
    auto create_swapchain(const SwapchainDesc &desc)
        -> std::expected<SwapchainHandle, DeviceError>;
    auto acquire_next_image(SwapchainHandle swapchain)
        -> std::expected<TextureHandle, DeviceError>;
    auto present(SwapchainHandle swapchain) -> void;
    auto resize_swapchain(SwapchainHandle swapchain,
                          std::uint32_t width,
                          std::uint32_t height) -> void;
    auto destroy_swapchain(SwapchainHandle handle) -> void;

    // Pipeline cache
    auto create_pipeline_cache(const PipelineCacheDesc &desc)
        -> std::expected<PipelineCacheHandle, DeviceError>;
    auto serialize_pipeline_cache(PipelineCacheHandle handle)
        -> std::vector<std::uint8_t>;
    auto destroy_pipeline_cache(PipelineCacheHandle handle) -> void;

    // Debug naming
    auto set_name(TextureHandle handle, std::string_view name) -> void;
    auto set_name(BufferHandle handle, std::string_view name) -> void;
    auto set_name(AccelerationStructureHandle handle, std::string_view name) -> void;
    auto set_name(PipelineHandle handle, std::string_view name) -> void;
    auto set_name(FenceHandle handle, std::string_view name) -> void;

    // Device operations
    auto wait_idle() -> void;
  };

  // ---------------------------------------------------------------------------
  // Concept verification
  // ---------------------------------------------------------------------------

  static_assert(GpuCommandPool<VulkanCommandPool>);
  static_assert(GpuCommandBuffer<VulkanCommandBuffer>);
  static_assert(GpuDevice<VulkanDevice>);

} // namespace harmonius::gpu::vulkan
