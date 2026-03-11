export module harmonius.rg.gate;

import harmonius.rg;

import std;

export namespace harmonius::rg::gate {

// ---------------------------------------------------------------------------
// Capability descriptor
// ---------------------------------------------------------------------------

/// Describes the set of GPU capabilities available on the current device.
/// @threadsafety Instances are not thread-safe.
struct CapabilityDescriptor {
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

  /// Returns true if the named capability is supported.
  [[nodiscard]] auto Has(std::string_view cap) const -> bool;
};

// ---------------------------------------------------------------------------
// Gate descriptors
// ---------------------------------------------------------------------------

/// Describes a gate that requires a specific GPU capability.
/// @threadsafety Instances are not thread-safe.
struct CapabilityGateDesc {
  std::string_view required_capability;
  bool hard = true;
};

/// Describes a gate based on GPU time budget thresholds.
/// @threadsafety Instances are not thread-safe.
struct BudgetGateDesc {
  std::string_view timestamp_query_name;
  float threshold_ms = 2.0f;
  std::uint32_t priority = 0;
};

/// Describes a gate based on resource pool utilization.
/// @threadsafety Instances are not thread-safe.
struct PoolUtilizationGateDesc {
  float utilization_threshold = 0.9f;
  std::uint32_t priority = 0;
};

/// Describes a fallback entry mapping a pass to optional capability and budget gates.
/// @threadsafety Instances are not thread-safe.
struct FallbackEntry {
  PassHandle pass;
  std::optional<CapabilityGateDesc> capability_gate;
  std::optional<BudgetGateDesc> budget_gate;
};

/// Describes a gate conditioned on a specific variant selection.
/// @threadsafety Instances are not thread-safe.
struct PathConditionedGateDesc {
  VariantSlotHandle variant_slot;
  std::string_view required_variant;
};

// ---------------------------------------------------------------------------
// Gate evaluator
// ---------------------------------------------------------------------------

/// Evaluates gates at compile time and runtime to determine active passes.
/// @threadsafety Instances are not thread-safe.
class GateEvaluator {
 public:
  /// Evaluates capability gates at compile time and returns handles of satisfied gates.
  [[nodiscard]] auto EvaluateCompileTime(const CapabilityDescriptor& caps,
                                          std::span<const CapabilityGateDesc> gates) -> std::vector<GateHandle>;

  /// Evaluates budget and pool utilization gates at runtime and returns handles of satisfied gates.
  [[nodiscard]] auto EvaluateRuntime(std::span<const BudgetGateDesc> budget_gates,
                                      std::span<const PoolUtilizationGateDesc> pool_gates)
      -> std::vector<GateHandle>;
};

}  // namespace harmonius::rg::gate
