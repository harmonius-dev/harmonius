/// @file harmonius.gpu_runtime.work_graph.cppm
/// @brief Work graph runtime — transparent execution via native GPU work graphs
///        (D3D12) or CPU-side emulation (all backends).
///
/// When DeviceCapabilities::work_graphs is true, translates the render graph
/// execution plan into a GPU work graph program for GPU self-scheduling.
/// When false, performs traditional CPU-side command buffer recording per pass.
///
/// Requirements: GR-3.1–GR-3.7

export module harmonius.gpu_runtime.work_graph;

import std;
import harmonius.gpu;
import harmonius.gpu_runtime.state;

export namespace harmonius::gpu_runtime::work_graph
{

  // ---------------------------------------------------------------------------
  // Execution plan view types
  // ---------------------------------------------------------------------------

  struct ResourceRef
  {
    std::uint32_t resource_id;
    gpu::ResourceAccess access;
    gpu::TextureLayout layout;
  };

  struct PassNode
  {
    std::uint32_t id;
    gpu::PipelineHandle pipeline;
    std::span<const ResourceRef> inputs;
    std::span<const ResourceRef> outputs;
    gpu::QueueType queue;
    bool active;
  };

  struct Dependency
  {
    std::uint32_t src_pass;
    std::uint32_t dst_pass;
  };

  struct BarrierGroup
  {
    std::uint32_t before_pass;
    std::span<const gpu::BarrierDesc> barriers;
  };

  struct ExecutionPlanView
  {
    std::span<const PassNode> passes;
    std::span<const Dependency> dependencies;
    std::span<const BarrierGroup> barriers;
  };

  // ---------------------------------------------------------------------------
  // Per-frame data
  // ---------------------------------------------------------------------------

  struct ResourceBinding
  {
    std::uint32_t slot;
    gpu::TextureHandle texture;
    gpu::BufferHandle buffer;
  };

  struct FrameData
  {
    std::span<const ResourceBinding> resource_bindings;
    std::span<const std::uint8_t> push_constant_data;
    std::span<const bool> pass_activation_flags;
    std::uint64_t frame_index;
  };

  // ---------------------------------------------------------------------------
  // Work graph executor
  // ---------------------------------------------------------------------------

  class WorkGraphExecutor
  {
  public:
    auto set_execution_plan(const ExecutionPlanView &plan) -> void;

    auto execute(const FrameData &data,
                 state::TrackedCommandBuffer &cmd,
                 const gpu::DeviceCapabilities &caps) -> void;

    [[nodiscard]] auto is_native() const -> bool;

  private:
    bool use_native_ = false;
    gpu::WorkGraphHandle cached_program_ = gpu::WorkGraphHandle::invalid;
    std::uint64_t plan_version_ = 0;
  };

} // namespace harmonius::gpu_runtime::work_graph
