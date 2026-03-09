/// @file harmonius.gpu_runtime.compat.cppm
/// @brief Cross-backend feature emulation — split barrier emulation,
///        barrier batching, queue ownership elision, ray tracing pipeline emulation.
///
/// Provides capability-aware wrappers that automatically select native or
/// emulated code paths based on DeviceCapabilities.
///
/// Requirements: GR-4.1–GR-4.9

export module harmonius.gpu_runtime.compat;

import std;
import harmonius.gpu;
import harmonius.gpu_runtime.memory;
import harmonius.gpu_runtime.state;

export namespace harmonius::gpu_runtime::compat
{

  // ---------------------------------------------------------------------------
  // Pipeline pair for RT emulation
  // ---------------------------------------------------------------------------

  struct PipelinePair
  {
    gpu::PipelineHandle rt_pipeline;
    gpu::PipelineHandle compute_fallback;
  };

  // ---------------------------------------------------------------------------
  // SBT types
  // ---------------------------------------------------------------------------

  struct SbtRecord
  {
    std::span<const std::uint8_t> local_root_args;
  };

  struct SbtLayout
  {
    std::span<const SbtRecord> raygen_records;
    std::span<const SbtRecord> miss_records;
    std::span<const SbtRecord> hit_group_records;
    std::span<const SbtRecord> callable_records;
    std::uint32_t record_stride;
  };

  // ---------------------------------------------------------------------------
  // Ray tracing adapter
  // ---------------------------------------------------------------------------

  class RayTracingAdapter
  {
  public:
    explicit RayTracingAdapter(const gpu::DeviceCapabilities &caps,
                               memory::Allocator &allocator);

    auto register_pipeline_pair(std::uint64_t id, const PipelinePair &pair) -> void;
    auto unregister_pipeline_pair(std::uint64_t id) -> void;

    auto build_sbt(std::uint64_t pipeline_id, const SbtLayout &layout) -> void;

    auto dispatch(std::uint64_t pipeline_id,
                  const gpu::TraceRaysDesc &desc,
                  state::TrackedCommandBuffer &cmd) -> bool;

    [[nodiscard]] auto is_emulated() const -> bool;

  private:
    const gpu::DeviceCapabilities &caps_;
    memory::Allocator &allocator_;
    bool emulated_;
    std::unordered_map<std::uint64_t, PipelinePair> pairs_;
    std::unordered_map<std::uint64_t, gpu::BufferHandle> sbt_buffers_;
  };

} // namespace harmonius::gpu_runtime::compat
