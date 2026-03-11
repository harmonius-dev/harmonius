export module harmonius.rg.compiler;

import harmonius.gpu;
import harmonius.rg;
import harmonius.rg.builder;

import std;

export namespace harmonius::rg::compiler {

// ---------------------------------------------------------------------------
// Compile options
// ---------------------------------------------------------------------------

/// Configuration options controlling graph compilation behavior.
/// @threadsafety Instances are not thread-safe.
struct CompileOptions {
  std::unordered_map<std::string, std::string> variant_selections;
  bool enable_aliasing = true;
  bool enable_barrier_merging = true;
  bool enable_split_barriers = true;
  bool dump_graph = false;
};

// ---------------------------------------------------------------------------
// Scheduled pass and related types
// ---------------------------------------------------------------------------

/// A pass that has been scheduled for execution with queue and ordering info.
/// @threadsafety Instances are not thread-safe.
struct ScheduledPass {
  PassHandle handle;
  std::uint32_t execution_order;
  QueueAffinity queue;
  std::uint32_t encoding_group;
  bool is_conditional;
};

/// A group of passes that can be encoded into a single command buffer.
/// @threadsafety Instances are not thread-safe.
struct EncodingGroup {
  std::uint32_t group_id;
  std::vector<PassHandle> passes;
  bool parallel;
};

/// Describes a submission of passes to a specific GPU queue with fence operations.
/// @threadsafety Instances are not thread-safe.
struct QueueSubmission {
  QueueAffinity queue;
  std::vector<PassHandle> passes;
  std::vector<std::pair<gpu::FenceHandle, std::uint64_t>> fence_signals;
  std::vector<std::pair<gpu::FenceHandle, std::uint64_t>> fence_waits;
};

/// Describes fence coordination between two GPU queues.
/// @threadsafety Instances are not thread-safe.
struct FenceCoordination {
  QueueAffinity source_queue;
  QueueAffinity dest_queue;
  std::uint64_t signal_value;
  std::uint64_t wait_value;
};

/// Describes a dynamic resolution parameter with scale bounds.
/// @threadsafety Instances are not thread-safe.
struct ResolutionParam {
  std::string_view name;
  float default_scale = 1.0f;
  float min_scale = 0.5f;
  float max_scale = 1.0f;
};

// ---------------------------------------------------------------------------
// Execution plan (output of the compiler)
// ---------------------------------------------------------------------------

/// The compiled execution plan containing scheduled passes, barriers, and fence coordination.
/// @threadsafety Instances are not thread-safe.
class ExecutionPlan {
 public:
  /// Returns the scheduled passes in execution order.
  [[nodiscard]] auto Passes() const -> std::span<const ScheduledPass>;

  /// Returns the queue submissions for this plan.
  [[nodiscard]] auto QueueSubmissions() const -> std::span<const QueueSubmission>;

  /// Returns the encoding groups for parallel command buffer recording.
  [[nodiscard]] auto EncodingGroups() const -> std::span<const EncodingGroup>;

  /// Returns the fence coordination points between queues.
  [[nodiscard]] auto FencePoints() const -> std::span<const FenceCoordination>;

  /// Returns the number of active (non-culled) passes.
  [[nodiscard]] auto ActivePassCount() const -> std::uint32_t;

  /// Returns the resolution parameters used for dynamic resolution.
  [[nodiscard]] auto ResolutionParams() const -> std::span<const ResolutionParam>;

 private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
};

// ---------------------------------------------------------------------------
// Graph compiler
// ---------------------------------------------------------------------------

/// Compiles a declared render graph into an optimized execution plan.
/// @threadsafety Instances are not thread-safe.
class GraphCompiler {
 public:
  /// Compiles the declared graph with the given options into an execution plan.
  [[nodiscard]] auto Compile(const builder::DeclaredGraph& graph, const CompileOptions& options = {})
      -> std::expected<ExecutionPlan, CompileError>;

  /// Recompiles residency changes into an existing execution plan.
  [[nodiscard]] auto RecompileResidency(
      ExecutionPlan& plan,
      std::span<const std::pair<ResourceHandle, bool>> changes) -> std::expected<void, CompileError>;
};

// ---------------------------------------------------------------------------
// Free function: validation
// ---------------------------------------------------------------------------

/// Validates a declared graph without compiling it.
[[nodiscard]] auto Validate(const builder::DeclaredGraph& graph) -> std::expected<void, CompileError>;

}  // namespace harmonius::rg::compiler
