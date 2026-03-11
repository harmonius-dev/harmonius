/// @file harmonius.gpu_runtime.compat.cppm
/// @brief Cross-backend feature emulation — split barrier emulation,
///        barrier batching, queue ownership elision, ray tracing pipeline
///        emulation.
///
/// Provides capability-aware wrappers that automatically select native or
/// emulated code paths based on DeviceCapabilities.
///
/// Requirements: GR-4.1–GR-4.9

export module harmonius.gpu_runtime.compat;

import harmonius.gpu;
import harmonius.gpu_runtime.memory;
import harmonius.gpu_runtime.state;

import std;

export namespace harmonius::gpu_runtime::compat {

// ---------------------------------------------------------------------------
// Pipeline pair for RT emulation
// ---------------------------------------------------------------------------

/// A native ray tracing pipeline paired with its compute fallback.
struct PipelinePair {
    gpu::PipelineHandle rt_pipeline;
    gpu::PipelineHandle compute_fallback;
};

// ---------------------------------------------------------------------------
// SBT types
// ---------------------------------------------------------------------------

/// A single shader binding table record containing local root arguments.
struct SbtRecord {
    std::span<const std::uint8_t> local_root_args;
};

/// Layout of a shader binding table across all shader stages.
struct SbtLayout {
    std::span<const SbtRecord> raygen_records;
    std::span<const SbtRecord> miss_records;
    std::span<const SbtRecord> hit_group_records;
    std::span<const SbtRecord> callable_records;
    std::uint32_t record_stride;
};

// ---------------------------------------------------------------------------
// Ray tracing adapter
// ---------------------------------------------------------------------------

/// Capability-aware ray tracing dispatcher that falls back to compute emulation.
/// @threadsafety Instances are not thread-safe.
class RayTracingAdapter {
 public:
    explicit RayTracingAdapter(const gpu::DeviceCapabilities& caps, memory::Allocator& allocator);

    /// Register a native/fallback pipeline pair.
    auto RegisterPipelinePair(std::uint64_t id, const PipelinePair& pair) -> void;

    /// Unregister a pipeline pair.
    auto UnregisterPipelinePair(std::uint64_t id) -> void;

    /// Build a shader binding table for the given pipeline.
    auto BuildSbt(std::uint64_t pipeline_id, const SbtLayout& layout) -> void;

    /// Dispatch ray tracing work, using emulation if necessary. Returns true if emulated.
    [[nodiscard]] auto Dispatch(std::uint64_t pipeline_id, const gpu::TraceRaysDesc& desc,
                                state::TrackedCommandBuffer& cmd) -> bool;

    /// Returns true if ray tracing is emulated via compute.
    [[nodiscard]] auto IsEmulated() const -> bool;

 private:
    const gpu::DeviceCapabilities& caps_;
    memory::Allocator& allocator_;
    bool emulated_;
    std::unordered_map<std::uint64_t, PipelinePair> pairs_;
    std::unordered_map<std::uint64_t, gpu::BufferHandle> sbt_buffers_;
};

}  // namespace harmonius::gpu_runtime::compat
