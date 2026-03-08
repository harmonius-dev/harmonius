export module harmonius.gpu.concepts;

import std;
import harmonius.gpu;

export namespace harmonius::gpu
{

  // ---------------------------------------------------------------------------
  // GpuCommandBuffer concept
  // ---------------------------------------------------------------------------

  template <typename B>
  concept GpuCommandBuffer = requires(
      B b,
      const BarrierDesc &barrier_desc,
      const RenderPassDesc &rp_desc,
      PipelineHandle pipeline,
      std::uint32_t u32,
      std::uint64_t u64,
      const TraceRaysDesc &trace_desc,
      const DispatchGraphDesc &graph_desc,
      const AccelerationStructureBuildDesc &build_desc,
      BufferHandle buf,
      TextureHandle tex,
      const TextureSubresource &sub,
      const Viewport &vp,
      const Scissor &sc,
      const void *data,
      DescriptorHeapHandle desc_heap,
      QueryPoolHandle query_pool,
      std::string_view label) {
    { b.begin() } -> std::same_as<void>;
    { b.end() } -> std::same_as<void>;

    { b.barrier(barrier_desc) } -> std::same_as<void>;

    { b.begin_render_pass(rp_desc) } -> std::same_as<void>;
    { b.end_render_pass() } -> std::same_as<void>;

    { b.set_pipeline(pipeline) } -> std::same_as<void>;

    { b.dispatch_mesh(u32, u32, u32) } -> std::same_as<void>;
    { b.dispatch(u32, u32, u32) } -> std::same_as<void>;
    { b.trace_rays(trace_desc) } -> std::same_as<void>;
    { b.dispatch_graph(graph_desc) } -> std::same_as<void>;
    { b.build_acceleration_structure(build_desc) } -> std::same_as<void>;

    { b.copy_buffer_to_buffer(buf, u64, buf, u64, u64) } -> std::same_as<void>;
    { b.copy_buffer_to_texture(buf, tex, sub) } -> std::same_as<void>;
    { b.copy_texture_to_buffer(tex, buf, sub) } -> std::same_as<void>;
    { b.copy_texture_to_texture(tex, tex, sub, sub) } -> std::same_as<void>;

    { b.set_viewport(vp) } -> std::same_as<void>;
    { b.set_scissor(sc) } -> std::same_as<void>;

    { b.push_constants(data, u32) } -> std::same_as<void>;
    { b.bind_descriptor_heap(desc_heap) } -> std::same_as<void>;
    { b.write_timestamp(query_pool, u32) } -> std::same_as<void>;

    { b.begin_debug_label(label) } -> std::same_as<void>;
    { b.end_debug_label() } -> std::same_as<void>;
  };

  // ---------------------------------------------------------------------------
  // GpuCommandPool concept
  // ---------------------------------------------------------------------------

  template <typename P>
  concept GpuCommandPool =
      GpuCommandBuffer<typename P::CommandBufferType> &&
      requires(P p, const P cp) {
        { p.allocate_command_buffer() } -> std::same_as<typename P::CommandBufferType>;
        { p.reset() } -> std::same_as<void>;
        { cp.allocated_count() } -> std::same_as<std::uint32_t>;
      };

  // ---------------------------------------------------------------------------
  // GpuDevice concept
  // ---------------------------------------------------------------------------

  template <typename D>
  concept GpuDevice =
      GpuCommandPool<typename D::CommandPoolType> &&
      requires(
          D d,
          const D cd,
          QueueType qt,
          const TextureDesc &tex_desc,
          TextureHandle tex,
          const BufferDesc &buf_desc,
          BufferHandle buf,
          const HeapDesc &heap_desc,
          HeapHandle heap,
          std::uint64_t u64,
          const SparseTextureDesc &sparse_tex_desc,
          std::span<const SparseTileBinding> bindings,
          const AccelerationStructureDesc &accel_desc,
          AccelerationStructureHandle accel,
          FenceHandle fence,
          typename D::CommandPoolType pool,
          const MeshRenderPipelineDesc &mesh_desc,
          const ComputePipelineDesc &compute_desc,
          const RayTracingPipelineDesc &rt_desc,
          const WorkGraphDesc &wg_desc,
          PipelineHandle pipeline,
          WorkGraphHandle work_graph,
          const DescriptorHeapDesc &dh_desc,
          DescriptorHeapHandle desc_heap,
          std::uint32_t u32,
          const DescriptorWrite &dw,
          const QueryPoolDesc &qp_desc,
          QueryPoolHandle query_pool,
          const SwapchainDesc &sc_desc,
          SwapchainHandle swapchain,
          const PipelineCacheDesc &pc_desc,
          PipelineCacheHandle pipeline_cache,
          std::span<const typename D::CommandBufferType> cmd_bufs,
          std::span<const FenceSignal> signals,
          std::span<const FenceWait> waits,
          std::string_view name) {
        // Queries
        { cd.capabilities() } -> std::same_as<DeviceCapabilities>;
        { cd.queue_count(qt) } -> std::same_as<std::uint32_t>;

        // Texture
        { d.create_texture(tex_desc) } -> std::same_as<std::expected<TextureHandle, ResourceError>>;
        { d.destroy_texture(tex) } -> std::same_as<void>;

        // Buffer
        { d.create_buffer(buf_desc) } -> std::same_as<std::expected<BufferHandle, ResourceError>>;
        { d.destroy_buffer(buf) } -> std::same_as<void>;

        // Heap
        { d.create_heap(heap_desc) } -> std::same_as<std::expected<HeapHandle, ResourceError>>;
        { d.destroy_heap(heap) } -> std::same_as<void>;

        // Placed resources
        { d.create_placed_texture(tex_desc, heap, u64) } -> std::same_as<std::expected<TextureHandle, ResourceError>>;
        { d.create_placed_buffer(buf_desc, heap, u64) } -> std::same_as<std::expected<BufferHandle, ResourceError>>;

        // Sparse resources
        { d.create_sparse_texture(sparse_tex_desc) } -> std::same_as<std::expected<TextureHandle, ResourceError>>;
        { d.update_sparse_bindings(tex, bindings) } -> std::same_as<void>;

        // Acceleration structures
        { d.create_acceleration_structure(accel_desc) } -> std::same_as<std::expected<AccelerationStructureHandle, ResourceError>>;
        { d.destroy_acceleration_structure(accel) } -> std::same_as<void>;
        { cd.get_acceleration_structure_sizes(accel_desc) } -> std::same_as<AccelerationStructureSizes>;

        // Fences
        { d.create_fence(u64) } -> std::same_as<std::expected<FenceHandle, DeviceError>>;
        { d.destroy_fence(fence) } -> std::same_as<void>;

        // Command pools
        { d.create_command_pool(qt) } -> std::same_as<std::expected<typename D::CommandPoolType, DeviceError>>;
        { d.destroy_command_pool(pool) } -> std::same_as<void>;

        // Pipelines
        { d.create_mesh_render_pipeline(mesh_desc) } -> std::same_as<std::expected<PipelineHandle, PipelineError>>;
        { d.create_compute_pipeline(compute_desc) } -> std::same_as<std::expected<PipelineHandle, PipelineError>>;
        { d.create_ray_tracing_pipeline(rt_desc) } -> std::same_as<std::expected<PipelineHandle, PipelineError>>;
        { d.destroy_pipeline(pipeline) } -> std::same_as<void>;

        // Work graphs
        { d.create_work_graph(wg_desc) } -> std::same_as<std::expected<WorkGraphHandle, PipelineError>>;
        { d.destroy_work_graph(work_graph) } -> std::same_as<void>;

        // Descriptor heaps
        { d.create_descriptor_heap(dh_desc) } -> std::same_as<std::expected<DescriptorHeapHandle, DeviceError>>;
        { d.destroy_descriptor_heap(desc_heap) } -> std::same_as<void>;
        { d.write_descriptor(desc_heap, u32, dw) } -> std::same_as<void>;

        // Query pools
        { d.create_query_pool(qp_desc) } -> std::same_as<std::expected<QueryPoolHandle, DeviceError>>;
        { d.destroy_query_pool(query_pool) } -> std::same_as<void>;

        // Swapchain
        { d.create_swapchain(sc_desc) } -> std::same_as<std::expected<SwapchainHandle, DeviceError>>;
        { d.destroy_swapchain(swapchain) } -> std::same_as<void>;

        // Pipeline cache
        { d.create_pipeline_cache(pc_desc) } -> std::same_as<std::expected<PipelineCacheHandle, DeviceError>>;
        { d.destroy_pipeline_cache(pipeline_cache) } -> std::same_as<void>;

        // Submission and synchronization
        { d.submit(qt, cmd_bufs, signals, waits) } -> std::same_as<void>;

        // Memory mapping
        { d.map(buf) } -> std::same_as<void *>;
        { d.unmap(buf) } -> std::same_as<void>;

        // Device operations
        { d.wait_idle() } -> std::same_as<void>;

        // Debug naming
        { d.set_name(tex, name) } -> std::same_as<void>;
        { d.set_name(buf, name) } -> std::same_as<void>;

        // Presentation
        { d.present(swapchain) } -> std::same_as<std::expected<TextureHandle, DeviceError>>;
        { d.resize_swapchain(swapchain, u32, u32) } -> std::same_as<void>;
      };

} // namespace harmonius::gpu
