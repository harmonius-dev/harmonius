export module harmonius.rg.diag;

import std;
import harmonius.gpu;
import harmonius.rg;

export namespace harmonius::rg::diag
{

  // ---------------------------------------------------------------------------
  // Timestamp diagnostics
  // ---------------------------------------------------------------------------

  struct TimestampEntry
  {
    std::string_view pass_name;
    std::uint64_t begin_ns;
    std::uint64_t end_ns;

    [[nodiscard]] auto duration_ms() const -> float
    {
      return static_cast<float>(end_ns - begin_ns) / 1'000'000.0f;
    }
  };

  struct TimestampResults
  {
    std::vector<TimestampEntry> entries;

    [[nodiscard]] auto find(std::string_view name) const -> std::optional<TimestampEntry>;
  };

  // ---------------------------------------------------------------------------
  // Pipeline statistics
  // ---------------------------------------------------------------------------

  struct PipelineStatisticsEntry
  {
    std::string_view pass_name;
    std::uint64_t primitives_count;
    std::uint64_t invocations_count;
  };

  struct PipelineStatistics
  {
    std::vector<PipelineStatisticsEntry> entries;
  };

  // ---------------------------------------------------------------------------
  // Transfer statistics
  // ---------------------------------------------------------------------------

  struct TransferEntry
  {
    std::string_view pass_name;
    std::uint64_t bytes_transferred;
    float latency_ms;
  };

  struct TransferStatistics
  {
    std::vector<TransferEntry> entries;
    std::uint64_t total_bytes_per_frame;
  };

  // ---------------------------------------------------------------------------
  // Memory diagnostics
  // ---------------------------------------------------------------------------

  struct MemoryDiagnostics
  {
    std::uint64_t peak_aliased_bytes;
    std::uint64_t total_allocated_bytes;
    float aliasing_efficiency;
    std::uint32_t active_pool_count;
    std::uint64_t total_pool_capacity;
  };

  // ---------------------------------------------------------------------------
  // Readback
  // ---------------------------------------------------------------------------

  struct ReadbackRequest
  {
    ResourceHandle resource;
    std::uint32_t mip = 0;
    std::uint32_t layer = 0;
    std::function<void(std::span<const std::uint8_t>)> callback;
  };

  // ---------------------------------------------------------------------------
  // Diagnostics collector
  // ---------------------------------------------------------------------------

  class DiagnosticsCollector
  {
  public:
    [[nodiscard]] auto read_timestamps() const -> TimestampResults;
    [[nodiscard]] auto read_statistics() const -> PipelineStatistics;
    [[nodiscard]] auto read_transfer_stats() const -> TransferStatistics;
    [[nodiscard]] auto read_memory_stats() const -> MemoryDiagnostics;
    [[nodiscard]] auto queue_depth(QueueAffinity queue) const -> std::uint32_t;
    auto request_readback(ReadbackRequest req) -> void;
    auto read_readback() -> void;
  };

} // namespace harmonius::rg::diag
