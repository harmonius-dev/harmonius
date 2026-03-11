export module harmonius.rg.exec;

import harmonius.gpu;
import harmonius.rg;
import harmonius.rg.compiler;

import std;

export namespace harmonius::rg::exec {

// ---------------------------------------------------------------------------
// Execution errors
// ---------------------------------------------------------------------------

/// Errors that can occur during render graph execution.
enum class ExecutionError : std::uint8_t {
  kCommandBufferAllocationFailed,
  kSubmissionFailed,
  kDeviceLost,
};

// ---------------------------------------------------------------------------
// Resolved resource (texture or buffer)
// ---------------------------------------------------------------------------

/// A GPU resource resolved from a render graph handle to a concrete texture or buffer.
/// @threadsafety Instances are not thread-safe.
struct ResolvedResource {
  gpu::TextureHandle texture = gpu::TextureHandle::kInvalid;
  gpu::BufferHandle buffer = gpu::BufferHandle::kInvalid;

  /// Returns true if this resolved resource is a texture.
  [[nodiscard]] auto IsTexture() const -> bool;

  /// Returns true if this resolved resource is a buffer.
  [[nodiscard]] auto IsBuffer() const -> bool;
};

// ---------------------------------------------------------------------------
// Transfer pass descriptor
// ---------------------------------------------------------------------------

/// Describes a transfer operation injected into the render graph execution.
/// @threadsafety Instances are not thread-safe.
struct TransferPassDesc {
  gpu::BufferHandle src_staging;
  ResourceHandle dst_resource;
  std::uint64_t src_offset = 0;
  std::uint64_t dst_offset = 0;
  std::uint64_t size = 0;
  std::uint32_t priority = 0;
};

// ---------------------------------------------------------------------------
// Pass context
// ---------------------------------------------------------------------------

/// Provides resource resolution and per-frame state during pass execution.
/// @threadsafety Instances are not thread-safe.
class PassContext {
 public:
  /// Resolves a render graph resource handle to its concrete GPU resource.
  [[nodiscard]] auto Resolve(ResourceHandle handle) const -> ResolvedResource;

  /// Allocates transient constant buffer space and returns the buffer handle and offset.
  [[nodiscard]] auto AllocateConstants(std::uint64_t size) -> std::pair<gpu::BufferHandle, std::uint64_t>;

  /// Returns the current frame index.
  [[nodiscard]] auto FrameIndex() const -> std::uint32_t;
};

// ---------------------------------------------------------------------------
// Command buffer pool
// ---------------------------------------------------------------------------

/// Manages a pool of reusable command buffers for render graph execution.
/// @threadsafety Instances are not thread-safe.
class CommandBufferPool {
 public:
  /// Resets all command buffers for the current frame.
  auto ResetFrame() -> void;

  /// Returns the number of command buffers currently allocated.
  [[nodiscard]] auto AllocatedCount() const -> std::uint32_t;
};

// ---------------------------------------------------------------------------
// Executor
// ---------------------------------------------------------------------------

/// Executes a compiled render graph execution plan on the GPU.
/// @threadsafety Instances are not thread-safe.
class Executor {
 public:
  /// Binds an external texture to a render graph resource handle.
  auto BindTexture(ResourceHandle rg_handle, gpu::TextureHandle gpu_handle) -> void;

  /// Binds an external buffer to a render graph resource handle.
  auto BindBuffer(ResourceHandle rg_handle, gpu::BufferHandle gpu_handle) -> void;

  /// Sets the dynamic resolution scale for the named resolution parameter.
  auto SetResolutionScale(std::string_view name, float scale) -> void;

  /// Enables or disables a pass at runtime.
  auto SetPassActive(PassHandle pass, bool active) -> void;

  /// Invalidates history data for a resource, forcing a fresh write next frame.
  auto InvalidateHistory(ResourceHandle resource) -> void;

  /// Injects a transfer operation into the execution plan.
  auto InjectTransfer(TransferPassDesc desc) -> void;

  /// Sets the GPU time budget threshold for the named query.
  auto SetBudgetThreshold(std::string_view query_name, float threshold_ms) -> void;

  /// Executes the given execution plan, recording and submitting command buffers.
  [[nodiscard]] auto Execute(const compiler::ExecutionPlan& plan)
      -> std::expected<void, ExecutionError>;

  /// Returns the current frame index.
  [[nodiscard]] auto FrameIndex() const -> std::uint32_t;

 private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
};

}  // namespace harmonius::rg::exec
