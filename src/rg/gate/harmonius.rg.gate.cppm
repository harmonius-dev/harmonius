export module harmonius.rg.gate;

import std;
import harmonius.rg;

export namespace harmonius::rg::gate
{

  // ---------------------------------------------------------------------------
  // Capability descriptor
  // ---------------------------------------------------------------------------

  struct CapabilityDescriptor
  {
    // Required capabilities
    bool mesh_shaders = false;
    bool bindless_resources = false;
    bool timeline_fences = false;
    bool async_compute_queue = false;
    bool transfer_queue = false;

    // Soft-gated capabilities
    bool ray_tracing = false;
    bool ray_tracing_inline = false;
    bool opacity_micromaps = false;
    bool sparse_textures = false;
    bool int64_atomics = false;
    bool variable_rate_shading = false;
    bool work_graphs = false;
    bool split_barriers = false;
    bool shader_function_linking = false;

    [[nodiscard]] auto has(std::string_view cap) const -> bool;
  };

  // ---------------------------------------------------------------------------
  // Gate descriptors
  // ---------------------------------------------------------------------------

  struct CapabilityGateDesc
  {
    std::string_view required_capability;
    bool hard = true;
  };

  struct BudgetGateDesc
  {
    std::string_view timestamp_query_name;
    float threshold_ms = 2.0f;
    std::uint32_t priority = 0;
  };

  struct PoolUtilizationGateDesc
  {
    float utilization_threshold = 0.9f;
    std::uint32_t priority = 0;
  };

  struct FallbackEntry
  {
    PassHandle pass;
    std::optional<CapabilityGateDesc> capability_gate;
    std::optional<BudgetGateDesc> budget_gate;
  };

  struct PathConditionedGateDesc
  {
    VariantSlotHandle variant_slot;
    std::string_view required_variant;
  };

  // ---------------------------------------------------------------------------
  // Gate evaluator
  // ---------------------------------------------------------------------------

  class GateEvaluator
  {
  public:
    [[nodiscard]] auto evaluate_compile_time(
        const CapabilityDescriptor &caps,
        std::span<const CapabilityGateDesc> gates) -> std::vector<GateHandle>;

    [[nodiscard]] auto evaluate_runtime(
        std::span<const BudgetGateDesc> budget_gates,
        std::span<const PoolUtilizationGateDesc> pool_gates) -> std::vector<GateHandle>;
  };

} // namespace harmonius::rg::gate
