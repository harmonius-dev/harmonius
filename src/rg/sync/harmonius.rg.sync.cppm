export module harmonius.rg.sync;

import harmonius.gpu;
import harmonius.rg;
import harmonius.rg.compiler;

import std;

export namespace harmonius::rg::sync {

// ---------------------------------------------------------------------------
// Sync errors
// ---------------------------------------------------------------------------

/// Errors that can occur during synchronization operations.
enum class SyncError : std::uint8_t {
  kFenceCreationFailed,
  kTimeout,
  kDeviceLost,
};

// ---------------------------------------------------------------------------
// Barrier types
// ---------------------------------------------------------------------------

/// The type of GPU barrier to insert between passes.
enum class BarrierType : std::uint8_t {
  kMemory,
  kLayoutTransition,
  kOwnershipRelease,
  kOwnershipAcquire,
  kAliasing,
};

// ---------------------------------------------------------------------------
// Barrier descriptor
// ---------------------------------------------------------------------------

/// Describes a GPU barrier including resource transitions and queue ownership transfers.
/// @threadsafety Instances are not thread-safe.
struct BarrierDesc {
  BarrierType type;
  ResourceHandle resource;
  UsageType src_usage;
  UsageType dst_usage;
  QueueAffinity src_queue;
  QueueAffinity dst_queue;
  std::uint32_t base_mip = 0;
  std::uint32_t mip_count = 1;
  std::uint32_t base_layer = 0;
  std::uint32_t layer_count = 1;
  bool is_split_begin = false;
  bool is_split_end = false;
};

// ---------------------------------------------------------------------------
// Barrier scheduler
// ---------------------------------------------------------------------------

/// Computes optimal barrier placement for a sequence of scheduled passes.
/// @threadsafety Instances are not thread-safe.
class BarrierScheduler {
 public:
  /// The computed barrier schedule mapping passes to their required barriers.
  /// @threadsafety Instances are not thread-safe.
  struct BarrierSchedule {
    std::vector<std::pair<PassHandle, std::vector<BarrierDesc>>> per_pass_barriers;
    std::uint32_t total_barriers;
    std::uint32_t merged_count;
  };

  /// Computes the barrier schedule for the given sequence of scheduled passes.
  [[nodiscard]] auto Compute(std::span<const compiler::ScheduledPass> passes) -> BarrierSchedule;
};

// ---------------------------------------------------------------------------
// Timeline fence manager
// ---------------------------------------------------------------------------

/// Manages timeline fences for GPU-GPU and GPU-CPU synchronization across queues.
/// @threadsafety Instances are not thread-safe.
class TimelineFenceManager {
 public:
  /// Signals the fence on the given queue with the specified value.
  [[nodiscard]] auto Signal(QueueAffinity queue, std::uint64_t value)
      -> std::expected<void, SyncError>;

  /// Inserts a GPU-side wait on the given queue for the specified fence value.
  [[nodiscard]] auto Wait(QueueAffinity queue, std::uint64_t value)
      -> std::expected<void, SyncError>;

  /// Returns true if the fence on the given queue has reached the specified value.
  [[nodiscard]] auto IsComplete(QueueAffinity queue, std::uint64_t value) const -> bool;

  /// Blocks the CPU until the fence on the given queue reaches the specified value.
  [[nodiscard]] auto WaitCpu(QueueAffinity queue, std::uint64_t value)
      -> std::expected<void, SyncError>;

  /// Returns the current fence value for the given queue.
  [[nodiscard]] auto CurrentValue(QueueAffinity queue) const -> std::uint64_t;

  /// Advances internal frame tracking for fence recycling.
  auto AdvanceFrame() -> void;
};

}  // namespace harmonius::rg::sync
