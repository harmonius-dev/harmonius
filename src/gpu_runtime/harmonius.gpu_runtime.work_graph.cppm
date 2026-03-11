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

import harmonius.gpu;
import harmonius.gpu_runtime.state;

import std;

export namespace harmonius::gpu_runtime::work_graph {

// ---------------------------------------------------------------------------
// Execution plan view types
// ---------------------------------------------------------------------------

/// Reference to a resource used by a render pass.
struct ResourceRef {
    std::uint32_t resource_id;
    gpu::ResourceAccess access;
    gpu::TextureLayout layout;
};

/// A single pass within the execution plan.
struct PassNode {
    std::uint32_t id;
    gpu::PipelineHandle pipeline;
    std::span<const ResourceRef> inputs;
    std::span<const ResourceRef> outputs;
    gpu::QueueType queue;
    bool active;
};

/// A dependency edge between two passes.
struct Dependency {
    std::uint32_t src_pass;
    std::uint32_t dst_pass;
};

/// A group of barriers to be issued before a specific pass.
struct BarrierGroup {
    std::uint32_t before_pass;
    std::span<const gpu::BarrierDesc> barriers;
};

/// Read-only view of a compiled execution plan.
struct ExecutionPlanView {
    std::span<const PassNode> passes;
    std::span<const Dependency> dependencies;
    std::span<const BarrierGroup> barriers;
};

// ---------------------------------------------------------------------------
// Per-frame data
// ---------------------------------------------------------------------------

/// Binding of a GPU resource to a descriptor slot.
struct ResourceBinding {
    std::uint32_t slot;
    gpu::TextureHandle texture;
    gpu::BufferHandle buffer;
};

/// Per-frame dynamic data supplied to the executor.
struct FrameData {
    std::span<const ResourceBinding> resource_bindings;
    std::span<const std::uint8_t> push_constant_data;
    std::span<const bool> pass_activation_flags;
    std::uint64_t frame_index;
};

// ---------------------------------------------------------------------------
// Work graph executor
// ---------------------------------------------------------------------------

/// Executes a render graph plan via native work graphs or CPU-side emulation.
/// @threadsafety Instances are not thread-safe.
class WorkGraphExecutor {
 public:
    /// Install a new execution plan, rebuilding the work graph program if native.
    auto SetExecutionPlan(const ExecutionPlanView& plan) -> void;

    /// Execute the current plan for one frame.
    auto Execute(const FrameData& data, state::TrackedCommandBuffer& cmd,
                 const gpu::DeviceCapabilities& caps) -> void;

    /// Returns true if using native GPU work graphs.
    [[nodiscard]] auto IsNative() const -> bool;

 private:
    bool use_native_ = false;
    gpu::WorkGraphHandle cached_program_ = gpu::WorkGraphHandle::kInvalid;
    std::uint64_t plan_version_ = 0;
};

}  // namespace harmonius::gpu_runtime::work_graph
