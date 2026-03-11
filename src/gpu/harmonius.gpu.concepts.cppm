/// @file harmonius.gpu.concepts.cppm
/// @brief C++20 concepts defining the GPU backend interface contracts.
export module harmonius.gpu.concepts;

import harmonius.gpu;

import std;

export namespace harmonius::gpu {

// ---------------------------------------------------------------------------
// GpuCommandBuffer concept
// ---------------------------------------------------------------------------

/// Defines the interface required for a GPU command buffer implementation.
template <typename B>
concept GpuCommandBuffer =
    requires(B b, const BarrierDesc& barrier_desc, const RenderPassDesc& rp_desc, PipelineHandle pipeline,
             WorkGraphHandle wgh, std::uint32_t u32, std::uint64_t u64, const TraceRaysDesc& trace_desc,
             const DispatchGraphDesc& graph_desc, const AccelerationStructureBuildDesc& build_desc, BufferHandle buf,
             TextureHandle tex, const TextureSubresource& sub, const Viewport& vp, const Scissor& sc, Extent3D ext,
             const void* data, DescriptorHeapHandle desc_heap, QueryPoolHandle query_pool, std::string_view label) {
        { b.Begin() } -> std::same_as<void>;
        { b.End() } -> std::same_as<void>;

        { b.Barrier(barrier_desc) } -> std::same_as<void>;

        { b.BeginRenderPass(rp_desc) } -> std::same_as<void>;
        { b.EndRenderPass() } -> std::same_as<void>;

        { b.SetPipeline(pipeline) } -> std::same_as<void>;

        // Mesh shader dispatch
        { b.DispatchMesh(u32, u32, u32) } -> std::same_as<void>;
        { b.DispatchMeshIndirect(buf, u64, u32, u32) } -> std::same_as<void>;
        { b.DispatchMeshIndirectCount(buf, u64, buf, u64, u32, u32) } -> std::same_as<void>;

        // Compute dispatch
        { b.Dispatch(u32, u32, u32) } -> std::same_as<void>;
        { b.DispatchIndirect(buf, u64) } -> std::same_as<void>;

        // Ray tracing
        { b.TraceRays(trace_desc) } -> std::same_as<void>;
        { b.TraceRaysIndirect(buf, u64) } -> std::same_as<void>;

        // Acceleration structures
        { b.BuildAccelerationStructure(build_desc) } -> std::same_as<void>;

        // Work graphs
        { b.SetWorkGraph(wgh) } -> std::same_as<void>;
        { b.DispatchGraph(graph_desc) } -> std::same_as<void>;

        // Copy commands
        { b.CopyBuffer(buf, u64, buf, u64, u64) } -> std::same_as<void>;
        { b.CopyTexture(tex, sub, tex, sub, ext) } -> std::same_as<void>;
        { b.CopyBufferToTexture(buf, u64, tex, sub, ext) } -> std::same_as<void>;
        { b.CopyTextureToBuffer(tex, sub, buf, u64, ext) } -> std::same_as<void>;

        // Viewport and scissor
        { b.SetViewport(vp) } -> std::same_as<void>;
        { b.SetScissor(sc) } -> std::same_as<void>;

        // Push constants
        { b.PushConstants(data, u32, u32) } -> std::same_as<void>;

        // Descriptor binding
        { b.BindDescriptorHeap(desc_heap) } -> std::same_as<void>;

        // Queries
        { b.WriteTimestamp(query_pool, u32) } -> std::same_as<void>;
        { b.ResolveQueryPool(query_pool, u32, u32, buf, u64) } -> std::same_as<void>;

        // Debug labels
        { b.BeginDebugLabel(label) } -> std::same_as<void>;
        { b.EndDebugLabel() } -> std::same_as<void>;
        { b.InsertDebugLabel(label) } -> std::same_as<void>;
    };

// ---------------------------------------------------------------------------
// GpuCommandPool concept
// ---------------------------------------------------------------------------

/// Defines the interface required for a GPU command pool implementation.
template <typename P>
concept GpuCommandPool = GpuCommandBuffer<typename P::CommandBufferType> && requires(P p, const P cp) {
    { p.AllocateCommandBuffer() } -> std::same_as<typename P::CommandBufferType>;
    { p.Reset() } -> std::same_as<void>;
    { cp.AllocatedCount() } -> std::convertible_to<std::uint32_t>;
};

// ---------------------------------------------------------------------------
// GpuDevice concept
// ---------------------------------------------------------------------------

/// Defines the interface required for a GPU device implementation.
template <typename D>
concept GpuDevice =
    GpuCommandPool<typename D::CommandPoolType> &&
    requires(D d, const D cd, QueueType qt, const TextureDesc& tex_desc, TextureHandle tex,
             const BufferDesc& buf_desc, BufferHandle buf, const HeapDesc& heap_desc, HeapHandle heap,
             std::uint64_t u64, const SparseTextureDesc& sparse_tex_desc,
             std::span<const SparseTileBinding> bindings, const AccelerationStructureDesc& accel_desc,
             AccelerationStructureHandle accel, FenceHandle fence, const MeshRenderPipelineDesc& mesh_desc,
             const ComputePipelineDesc& compute_desc, const RayTracingPipelineDesc& rt_desc,
             const WorkGraphDesc& wg_desc, PipelineHandle pipeline, WorkGraphHandle work_graph,
             const DescriptorHeapDesc& dh_desc, DescriptorHeapHandle desc_heap, std::uint32_t u32,
             const DescriptorWrite& dw, const QueryPoolDesc& qp_desc, QueryPoolHandle query_pool,
             const SwapchainDesc& sc_desc, SwapchainHandle swapchain, const PipelineCacheDesc& pc_desc,
             PipelineCacheHandle pipeline_cache, std::span<typename D::CommandBufferType*> cmd_bufs,
             std::span<const FenceSignal> signals, std::span<const FenceWait> waits, std::string_view name) {
        // Queries
        { cd.Capabilities() } -> std::same_as<DeviceCapabilities>;
        { cd.QueueCount(qt) } -> std::same_as<std::uint32_t>;

        // Texture
        { d.CreateTexture(tex_desc) } -> std::same_as<std::expected<TextureHandle, ResourceError>>;
        { d.DestroyTexture(tex) } -> std::same_as<void>;

        // Buffer
        { d.CreateBuffer(buf_desc) } -> std::same_as<std::expected<BufferHandle, ResourceError>>;
        { d.DestroyBuffer(buf) } -> std::same_as<void>;

        // Heap
        { d.CreateHeap(heap_desc) } -> std::same_as<std::expected<HeapHandle, ResourceError>>;
        { d.DestroyHeap(heap) } -> std::same_as<void>;

        // Placed resources
        {
            d.CreatePlacedTexture(heap, u64, tex_desc)
        } -> std::same_as<std::expected<TextureHandle, ResourceError>>;
        {
            d.CreatePlacedBuffer(heap, u64, buf_desc)
        } -> std::same_as<std::expected<BufferHandle, ResourceError>>;

        // Allocation info queries
        { cd.QueryTextureAllocationInfo(tex_desc) } -> std::same_as<AllocationInfo>;
        { cd.QueryBufferAllocationInfo(buf_desc) } -> std::same_as<AllocationInfo>;

        // Sparse resources
        {
            d.CreateSparseTexture(sparse_tex_desc)
        } -> std::same_as<std::expected<TextureHandle, ResourceError>>;
        { d.UpdateSparseBindings(tex, bindings) } -> std::same_as<void>;

        // Memory mapping
        { d.Map(buf) } -> std::same_as<void*>;
        { d.Unmap(buf) } -> std::same_as<void>;

        // Acceleration structures
        {
            d.CreateAccelerationStructure(accel_desc)
        } -> std::same_as<std::expected<AccelerationStructureHandle, ResourceError>>;
        { cd.QueryAccelerationStructureSizes(accel_desc) } -> std::same_as<AccelerationStructureSizes>;
        { d.DestroyAccelerationStructure(accel) } -> std::same_as<void>;

        // Fences
        { d.CreateFence(u64) } -> std::same_as<std::expected<FenceHandle, DeviceError>>;
        { d.DestroyFence(fence) } -> std::same_as<void>;
        { cd.FenceCompletedValue(fence) } -> std::same_as<std::uint64_t>;
        { d.WaitFenceCpu(fence, u64) } -> std::same_as<void>;

        // Command pools
        { d.CreateCommandPool(qt) };

        // Submission
        { d.Submit(qt, cmd_bufs, signals, waits) } -> std::same_as<void>;

        // Pipelines
        {
            d.CreateMeshRenderPipeline(mesh_desc)
        } -> std::same_as<std::expected<PipelineHandle, PipelineError>>;
        {
            d.CreateComputePipeline(compute_desc)
        } -> std::same_as<std::expected<PipelineHandle, PipelineError>>;
        {
            d.CreateRayTracingPipeline(rt_desc)
        } -> std::same_as<std::expected<PipelineHandle, PipelineError>>;
        { d.DestroyPipeline(pipeline) } -> std::same_as<void>;

        // Work graphs
        { d.CreateWorkGraph(wg_desc) } -> std::same_as<std::expected<WorkGraphHandle, PipelineError>>;
        { d.DestroyWorkGraph(work_graph) } -> std::same_as<void>;

        // Descriptor heaps
        {
            d.CreateDescriptorHeap(dh_desc)
        } -> std::same_as<std::expected<DescriptorHeapHandle, DeviceError>>;
        { d.WriteDescriptor(desc_heap, u32, dw) } -> std::same_as<void>;
        { d.DestroyDescriptorHeap(desc_heap) } -> std::same_as<void>;

        // Query pools
        { d.CreateQueryPool(qp_desc) } -> std::same_as<std::expected<QueryPoolHandle, DeviceError>>;
        { d.DestroyQueryPool(query_pool) } -> std::same_as<void>;
        { cd.TimestampPeriodNs() } -> std::same_as<double>;

        // Swapchain
        { d.CreateSwapchain(sc_desc) } -> std::same_as<std::expected<SwapchainHandle, DeviceError>>;
        {
            d.AcquireNextImage(swapchain)
        } -> std::same_as<std::expected<TextureHandle, DeviceError>>;
        { d.Present(swapchain) } -> std::same_as<void>;
        { d.ResizeSwapchain(swapchain, u32, u32) } -> std::same_as<void>;
        { d.DestroySwapchain(swapchain) } -> std::same_as<void>;

        // Pipeline cache
        {
            d.CreatePipelineCache(pc_desc)
        } -> std::same_as<std::expected<PipelineCacheHandle, DeviceError>>;
        { d.SerializePipelineCache(pipeline_cache) } -> std::same_as<std::vector<std::uint8_t>>;
        { d.DestroyPipelineCache(pipeline_cache) } -> std::same_as<void>;

        // Debug naming
        { d.SetName(tex, name) } -> std::same_as<void>;
        { d.SetName(buf, name) } -> std::same_as<void>;
        { d.SetName(accel, name) } -> std::same_as<void>;
        { d.SetName(pipeline, name) } -> std::same_as<void>;
        { d.SetName(fence, name) } -> std::same_as<void>;

        // Device operations
        { d.WaitIdle() } -> std::same_as<void>;
    };

}  // namespace harmonius::gpu
