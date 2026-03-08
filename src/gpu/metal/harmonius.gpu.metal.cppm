module;

#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>

export module harmonius.gpu.metal;

import std;
import harmonius.gpu;
import harmonius.gpu.concepts;

export namespace harmonius::gpu::metal
{

  // ---------------------------------------------------------------------------
  // Forward declarations
  // ---------------------------------------------------------------------------

  class MetalCommandBuffer;

  // ---------------------------------------------------------------------------
  // MetalCommandPool
  // ---------------------------------------------------------------------------

  class MetalCommandPool
  {
  public:
    using CommandBufferType = MetalCommandBuffer;

    auto allocate_command_buffer() -> MetalCommandBuffer;
    void reset();
    auto allocated_count() const -> std::uint32_t;
  };

  // ---------------------------------------------------------------------------
  // MetalCommandBuffer
  // ---------------------------------------------------------------------------

  class MetalCommandBuffer
  {
  public:
    void begin();
    void end();

    void barrier(const BarrierDesc &desc);

    void begin_render_pass(const RenderPassDesc &desc);
    void end_render_pass();

    void set_pipeline(PipelineHandle pipeline);

    void dispatch_mesh(std::uint32_t group_count_x,
                       std::uint32_t group_count_y,
                       std::uint32_t group_count_z);
    void dispatch(std::uint32_t group_count_x,
                  std::uint32_t group_count_y,
                  std::uint32_t group_count_z);
    void trace_rays(const TraceRaysDesc &desc);
    void dispatch_graph(const DispatchGraphDesc &desc);
    void build_acceleration_structure(const AccelerationStructureBuildDesc &desc);

    void copy_buffer_to_buffer(BufferHandle src,
                               std::uint64_t src_offset,
                               BufferHandle dst,
                               std::uint64_t dst_offset,
                               std::uint64_t size);
    void copy_buffer_to_texture(BufferHandle src,
                                TextureHandle dst,
                                const TextureSubresource &subresource);
    void copy_texture_to_buffer(TextureHandle src,
                                BufferHandle dst,
                                const TextureSubresource &subresource);
    void copy_texture_to_texture(TextureHandle src,
                                 TextureHandle dst,
                                 const TextureSubresource &src_subresource,
                                 const TextureSubresource &dst_subresource);

    void set_viewport(const Viewport &viewport);
    void set_scissor(const Scissor &scissor);

    void push_constants(const void *data, std::uint32_t size);
    void bind_descriptor_heap(DescriptorHeapHandle heap);
    void write_timestamp(QueryPoolHandle pool, std::uint32_t index);

    void begin_debug_label(std::string_view label);
    void end_debug_label();
  };

  // ---------------------------------------------------------------------------
  // MetalDevice
  // ---------------------------------------------------------------------------

  class MetalDevice
  {
  public:
    using CommandBufferType = MetalCommandBuffer;
    using CommandPoolType = MetalCommandPool;

    auto capabilities() const -> DeviceCapabilities;
    auto queue_count(QueueType type) const -> std::uint32_t;

    auto create_texture(const TextureDesc &desc) -> std::expected<TextureHandle, ResourceError>;
    void destroy_texture(TextureHandle handle);

    auto create_buffer(const BufferDesc &desc) -> std::expected<BufferHandle, ResourceError>;
    void destroy_buffer(BufferHandle handle);

    auto create_heap(const HeapDesc &desc) -> std::expected<HeapHandle, ResourceError>;
    void destroy_heap(HeapHandle handle);

    auto create_placed_texture(const TextureDesc &desc,
                               HeapHandle heap,
                               std::uint64_t offset) -> std::expected<TextureHandle, ResourceError>;
    auto create_placed_buffer(const BufferDesc &desc,
                              HeapHandle heap,
                              std::uint64_t offset) -> std::expected<BufferHandle, ResourceError>;

    auto create_sparse_texture(const SparseTextureDesc &desc) -> std::expected<TextureHandle, ResourceError>;
    void update_sparse_bindings(TextureHandle texture,
                                std::span<const SparseTileBinding> bindings);

    auto create_acceleration_structure(const AccelerationStructureDesc &desc)
        -> std::expected<AccelerationStructureHandle, ResourceError>;
    void destroy_acceleration_structure(AccelerationStructureHandle handle);
    auto get_acceleration_structure_sizes(const AccelerationStructureDesc &desc) const
        -> AccelerationStructureSizes;

    auto create_fence(std::uint64_t initial_value) -> std::expected<FenceHandle, DeviceError>;
    void destroy_fence(FenceHandle handle);

    auto create_command_pool(QueueType type) -> std::expected<MetalCommandPool, DeviceError>;
    void destroy_command_pool(MetalCommandPool pool);

    auto create_mesh_render_pipeline(const MeshRenderPipelineDesc &desc)
        -> std::expected<PipelineHandle, PipelineError>;
    auto create_compute_pipeline(const ComputePipelineDesc &desc)
        -> std::expected<PipelineHandle, PipelineError>;
    auto create_ray_tracing_pipeline(const RayTracingPipelineDesc &desc)
        -> std::expected<PipelineHandle, PipelineError>;
    void destroy_pipeline(PipelineHandle handle);

    auto create_work_graph(const WorkGraphDesc &desc)
        -> std::expected<WorkGraphHandle, PipelineError>;
    void destroy_work_graph(WorkGraphHandle handle);

    auto create_descriptor_heap(const DescriptorHeapDesc &desc)
        -> std::expected<DescriptorHeapHandle, DeviceError>;
    void destroy_descriptor_heap(DescriptorHeapHandle handle);
    void write_descriptor(DescriptorHeapHandle heap,
                          std::uint32_t index,
                          const DescriptorWrite &write);

    auto create_query_pool(const QueryPoolDesc &desc) -> std::expected<QueryPoolHandle, DeviceError>;
    void destroy_query_pool(QueryPoolHandle handle);

    auto create_swapchain(const SwapchainDesc &desc) -> std::expected<SwapchainHandle, DeviceError>;
    void destroy_swapchain(SwapchainHandle handle);

    auto create_pipeline_cache(const PipelineCacheDesc &desc)
        -> std::expected<PipelineCacheHandle, DeviceError>;
    void destroy_pipeline_cache(PipelineCacheHandle handle);

    void submit(QueueType type,
                std::span<const MetalCommandBuffer> command_buffers,
                std::span<const FenceSignal> signal_fences,
                std::span<const FenceWait> wait_fences);

    auto map(BufferHandle handle) -> void *;
    void unmap(BufferHandle handle);

    void wait_idle();

    void set_name(TextureHandle handle, std::string_view name);
    void set_name(BufferHandle handle, std::string_view name);

    auto present(SwapchainHandle swapchain) -> std::expected<TextureHandle, DeviceError>;
    void resize_swapchain(SwapchainHandle swapchain,
                          std::uint32_t width,
                          std::uint32_t height);
  };

  // ---------------------------------------------------------------------------
  // Concept verification
  // ---------------------------------------------------------------------------

  static_assert(GpuCommandPool<MetalCommandPool>);
  static_assert(GpuCommandBuffer<MetalCommandBuffer>);
  static_assert(GpuDevice<MetalDevice>);

} // namespace harmonius::gpu::metal
