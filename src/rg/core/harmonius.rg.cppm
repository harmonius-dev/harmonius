export module harmonius.rg;

import harmonius.gpu;

import std;

export namespace harmonius::rg {

// ---------------------------------------------------------------------------
// Handle types
// ---------------------------------------------------------------------------

/// Opaque handle identifying a render pass within the graph.
enum class PassHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

/// Opaque handle identifying a resource within the graph.
enum class ResourceHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

/// Opaque handle identifying a sub-graph template.
enum class SubGraphHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

/// Opaque handle identifying a gate for conditional pass execution.
enum class GateHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

/// Opaque handle identifying a chain of sequentially dependent passes.
enum class ChainHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

/// Opaque handle identifying a variant dispatch slot.
enum class VariantSlotHandle : std::uint32_t { kInvalid = std::numeric_limits<std::uint32_t>::max() };

// ---------------------------------------------------------------------------
// Classification enums
// ---------------------------------------------------------------------------

/// The type of work a render pass performs.
enum class PassType : std::uint8_t {
  kRasterization,
  kCompute,
  kRayTracingDispatch,
  kAccelerationStructureBuild,
  kTransfer,
  kMsaaResolve,
  kPresent,
  kHostCallback,
  kWorkGraph,
  kCheckerboardResolve,
};

/// How a resource is accessed by a pass.
enum class AccessMode : std::uint8_t {
  kRead,
  kWrite,
  kReadWrite,
};

/// The specific usage of a resource within a pass.
enum class UsageType : std::uint8_t {
  kColorAttachment,
  kDepthAttachment,
  kShaderRead,
  kStorageRead,
  kStorageWrite,
  kShadingRateAttachment,
  kIndirectArgument,
  kAccelerationStructureRead,
  kAccelerationStructureBuildWrite,
  kTransferSrc,
  kTransferDst,
  kPresent,
};

/// The GPU queue a pass should be submitted to.
enum class QueueAffinity : std::uint8_t {
  kGraphics,
  kAsyncCompute,
  kTransfer,
  kAny,
};

/// The lifetime category of a resource.
enum class ResourceCategory : std::uint8_t {
  kTransient,
  kPersistent,
  kImported,
  kHistory,
  kMultiFrameHistory,
  kSparse,
  kPoolBacked,
  kStaging,
  kAtlas,
  kAccelerationStructure,
};

/// The kind of validation error detected during graph compilation.
enum class ValidationErrorKind : std::uint8_t {
  kCycleDetected,
  kTypeMismatch,
  kUndeclaredResource,
  kQueueIncompatibility,
  kSingleWriterViolation,
  kVariantAmbiguity,
  kInstanceCountMismatch,
  kHardGateUnsatisfied,
};

// ---------------------------------------------------------------------------
// Shared vocabulary structs
// ---------------------------------------------------------------------------

/// Describes how a pass binds to a resource, including access mode and usage.
/// @threadsafety Instances are not thread-safe.
struct ResourceBinding {
  ResourceHandle resource;
  AccessMode access;
  UsageType usage;
  std::uint32_t array_layer = 0;
  std::uint32_t mip_level = 0;
  bool is_history = false;
};

/// A single validation error produced during graph compilation.
/// @threadsafety Instances are not thread-safe.
struct ValidationError {
  ValidationErrorKind kind;
  PassHandle pass;
  ResourceHandle resource;
  std::string message;
};

/// Aggregated compilation errors containing all validation failures.
/// @threadsafety Instances are not thread-safe.
struct CompileError {
  std::vector<ValidationError> errors;
};

}  // namespace harmonius::rg
