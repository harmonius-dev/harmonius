export module harmonius.rg;

import std;
import harmonius.gpu;

export namespace harmonius::rg
{

  // ---------------------------------------------------------------------------
  // Handle types
  // ---------------------------------------------------------------------------

  enum class PassHandle : std::uint32_t
  {
    invalid = std::numeric_limits<std::uint32_t>::max()
  };

  enum class ResourceHandle : std::uint32_t
  {
    invalid = std::numeric_limits<std::uint32_t>::max()
  };

  enum class SubGraphHandle : std::uint32_t
  {
    invalid = std::numeric_limits<std::uint32_t>::max()
  };

  enum class GateHandle : std::uint32_t
  {
    invalid = std::numeric_limits<std::uint32_t>::max()
  };

  enum class ChainHandle : std::uint32_t
  {
    invalid = std::numeric_limits<std::uint32_t>::max()
  };

  enum class VariantSlotHandle : std::uint32_t
  {
    invalid = std::numeric_limits<std::uint32_t>::max()
  };

  // ---------------------------------------------------------------------------
  // Classification enums
  // ---------------------------------------------------------------------------

  enum class PassType : std::uint8_t
  {
    rasterization,
    compute,
    ray_tracing_dispatch,
    acceleration_structure_build,
    transfer,
    msaa_resolve,
    present,
    host_callback,
    work_graph,
    checkerboard_resolve,
  };

  enum class AccessMode : std::uint8_t
  {
    read,
    write,
    read_write,
  };

  enum class UsageType : std::uint8_t
  {
    color_attachment,
    depth_attachment,
    shader_read,
    storage_read,
    storage_write,
    shading_rate_attachment,
    indirect_argument,
    acceleration_structure_read,
    acceleration_structure_build_write,
    transfer_src,
    transfer_dst,
    present,
  };

  enum class QueueAffinity : std::uint8_t
  {
    graphics,
    async_compute,
    transfer,
    any,
  };

  enum class ResourceCategory : std::uint8_t
  {
    transient,
    persistent,
    imported,
    history,
    multi_frame_history,
    sparse,
    pool_backed,
    staging,
    atlas,
    acceleration_structure,
  };

  enum class ValidationErrorKind : std::uint8_t
  {
    cycle_detected,
    type_mismatch,
    undeclared_resource,
    queue_incompatibility,
    single_writer_violation,
    variant_ambiguity,
    instance_count_mismatch,
    hard_gate_unsatisfied,
  };

  // ---------------------------------------------------------------------------
  // Shared vocabulary structs
  // ---------------------------------------------------------------------------

  struct ResourceBinding
  {
    ResourceHandle resource;
    AccessMode access;
    UsageType usage;
    std::uint32_t array_layer = 0;
    std::uint32_t mip_level = 0;
    bool is_history = false;
  };

  struct ValidationError
  {
    ValidationErrorKind kind;
    PassHandle pass;
    ResourceHandle resource;
    std::string message;
  };

  struct CompileError
  {
    std::vector<ValidationError> errors;
  };

} // namespace harmonius::rg
