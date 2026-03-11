export module harmonius.rg.diag;

import harmonius.gpu;
import harmonius.rg;

import std;

export namespace harmonius::rg::diag {

// ---------------------------------------------------------------------------
// Timestamp diagnostics
// ---------------------------------------------------------------------------

/// A single GPU timestamp measurement for a render pass.
/// @threadsafety Instances are not thread-safe.
struct TimestampEntry {
  std::string_view pass_name;
  std::uint64_t begin_ns;
  std::uint64_t end_ns;

  /// Returns the duration of the pass in milliseconds.
  [[nodiscard]] auto DurationMs() const -> float { return static_cast<float>(end_ns - begin_ns) / 1'000'000.0f; }
};

/// Collection of GPU timestamp results for all measured passes.
/// @threadsafety Instances are not thread-safe.
struct TimestampResults {
  std::vector<TimestampEntry> entries;

  /// Finds a timestamp entry by pass name.
  [[nodiscard]] auto Find(std::string_view name) const -> std::optional<TimestampEntry>;
};

// ---------------------------------------------------------------------------
// Pipeline statistics
// ---------------------------------------------------------------------------

/// GPU pipeline statistics for a single render pass.
/// @threadsafety Instances are not thread-safe.
struct PipelineStatisticsEntry {
  std::string_view pass_name;
  std::uint64_t primitives_count;
  std::uint64_t invocations_count;
};

/// Collection of pipeline statistics for all measured passes.
/// @threadsafety Instances are not thread-safe.
struct PipelineStatistics {
  std::vector<PipelineStatisticsEntry> entries;
};

// ---------------------------------------------------------------------------
// Transfer statistics
// ---------------------------------------------------------------------------

/// Transfer statistics for a single pass.
/// @threadsafety Instances are not thread-safe.
struct TransferEntry {
  std::string_view pass_name;
  std::uint64_t bytes_transferred;
  float latency_ms;
};

/// Aggregated transfer statistics for a frame.
/// @threadsafety Instances are not thread-safe.
struct TransferStatistics {
  std::vector<TransferEntry> entries;
  std::uint64_t total_bytes_per_frame;
};

// ---------------------------------------------------------------------------
// Memory diagnostics
// ---------------------------------------------------------------------------

/// Memory usage diagnostics including aliasing efficiency and pool statistics.
/// @threadsafety Instances are not thread-safe.
struct MemoryDiagnostics {
  std::uint64_t peak_aliased_bytes;
  std::uint64_t total_allocated_bytes;
  float aliasing_efficiency;
  std::uint32_t active_pool_count;
  std::uint64_t total_pool_capacity;
};

// ---------------------------------------------------------------------------
// Readback
// ---------------------------------------------------------------------------

/// Describes a request to read back GPU resource data to the CPU.
/// @threadsafety Instances are not thread-safe.
struct ReadbackRequest {
  ResourceHandle resource;
  std::uint32_t mip = 0;
  std::uint32_t layer = 0;
  std::move_only_function<void(std::span<const std::uint8_t>)> callback;
};

// ---------------------------------------------------------------------------
// Diagnostics collector
// ---------------------------------------------------------------------------

/// Collects GPU performance diagnostics, statistics, and readback data.
/// @threadsafety Instances are not thread-safe.
class DiagnosticsCollector {
 public:
  /// Reads GPU timestamp results for the most recently completed frame.
  [[nodiscard]] auto ReadTimestamps() const -> TimestampResults;

  /// Reads pipeline statistics for the most recently completed frame.
  [[nodiscard]] auto ReadStatistics() const -> PipelineStatistics;

  /// Reads transfer statistics for the most recently completed frame.
  [[nodiscard]] auto ReadTransferStats() const -> TransferStatistics;

  /// Reads memory diagnostics for the current frame.
  [[nodiscard]] auto ReadMemoryStats() const -> MemoryDiagnostics;

  /// Returns the current queue depth for the specified queue.
  [[nodiscard]] auto QueueDepth(QueueAffinity queue) const -> std::uint32_t;

  /// Enqueues a readback request for a GPU resource.
  auto RequestReadback(ReadbackRequest req) -> void;

  /// Processes completed readback requests, invoking their callbacks.
  auto ReadReadback() -> void;
};

}  // namespace harmonius::rg::diag
