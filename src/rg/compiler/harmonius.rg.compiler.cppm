export module harmonius.rg.compiler;

import std;
import harmonius.gpu;
import harmonius.rg;
import harmonius.rg.builder;

export namespace harmonius::rg::compiler
{

  // ---------------------------------------------------------------------------
  // Compile options
  // ---------------------------------------------------------------------------

  struct CompileOptions
  {
    std::unordered_map<std::string, std::string> variant_selections;
    bool enable_aliasing = true;
    bool enable_barrier_merging = true;
    bool enable_split_barriers = true;
    bool dump_graph = false;
  };

  // ---------------------------------------------------------------------------
  // Scheduled pass and related types
  // ---------------------------------------------------------------------------

  struct ScheduledPass
  {
    PassHandle handle;
    std::uint32_t execution_order;
    QueueAffinity queue;
    std::uint32_t encoding_group;
    bool is_conditional;
  };

  struct EncodingGroup
  {
    std::uint32_t group_id;
    std::vector<PassHandle> passes;
    bool parallel;
  };

  struct QueueSubmission
  {
    QueueAffinity queue;
    std::vector<PassHandle> passes;
    std::vector<std::pair<gpu::FenceHandle, std::uint64_t>> fence_signals;
    std::vector<std::pair<gpu::FenceHandle, std::uint64_t>> fence_waits;
  };

  struct FenceCoordination
  {
    QueueAffinity source_queue;
    QueueAffinity dest_queue;
    std::uint64_t signal_value;
    std::uint64_t wait_value;
  };

  struct ResolutionParam
  {
    std::string_view name;
    float default_scale = 1.0f;
    float min_scale = 0.5f;
    float max_scale = 1.0f;
  };

  // ---------------------------------------------------------------------------
  // Execution plan (output of the compiler)
  // ---------------------------------------------------------------------------

  class ExecutionPlan
  {
  public:
    [[nodiscard]] auto passes() const -> std::span<const ScheduledPass>;
    [[nodiscard]] auto queue_submissions() const -> std::span<const QueueSubmission>;
    [[nodiscard]] auto encoding_groups() const -> std::span<const EncodingGroup>;
    [[nodiscard]] auto fence_points() const -> std::span<const FenceCoordination>;
    [[nodiscard]] auto active_pass_count() const -> std::uint32_t;
    [[nodiscard]] auto resolution_params() const -> std::span<const ResolutionParam>;

  private:
    struct Impl;
    std::unique_ptr<Impl> impl_;
  };

  // ---------------------------------------------------------------------------
  // Graph compiler
  // ---------------------------------------------------------------------------

  class GraphCompiler
  {
  public:
    [[nodiscard]] auto compile(
        const builder::DeclaredGraph &graph,
        const CompileOptions &options = {}) -> std::expected<ExecutionPlan, CompileError>;

    auto recompile_residency(
        ExecutionPlan &plan,
        std::span<const std::pair<ResourceHandle, bool>> changes) -> void;
  };

  // ---------------------------------------------------------------------------
  // Free function: validation
  // ---------------------------------------------------------------------------

  [[nodiscard]] auto validate(
      const builder::DeclaredGraph &graph) -> std::expected<void, CompileError>;

} // namespace harmonius::rg::compiler
