export module harmonius.rg.sync;

import std;
import harmonius.gpu;
import harmonius.rg;
import harmonius.rg.compiler;

export namespace harmonius::rg::sync
{

  // ---------------------------------------------------------------------------
  // Barrier types
  // ---------------------------------------------------------------------------

  enum class BarrierType : std::uint8_t
  {
    memory,
    layout_transition,
    ownership_release,
    ownership_acquire,
    aliasing,
  };

  // ---------------------------------------------------------------------------
  // Barrier descriptor
  // ---------------------------------------------------------------------------

  struct BarrierDesc
  {
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

  class BarrierScheduler
  {
  public:
    struct BarrierSchedule
    {
      std::vector<std::pair<PassHandle, std::vector<BarrierDesc>>> per_pass_barriers;
      std::uint32_t total_barriers;
      std::uint32_t merged_count;
    };

    [[nodiscard]] auto compute(
        std::span<const compiler::ScheduledPass> passes) -> BarrierSchedule;
  };

  // ---------------------------------------------------------------------------
  // Timeline fence manager
  // ---------------------------------------------------------------------------

  class TimelineFenceManager
  {
  public:
    auto signal(QueueAffinity queue, std::uint64_t value) -> void;
    auto wait(QueueAffinity queue, std::uint64_t value) -> void;
    [[nodiscard]] auto is_complete(QueueAffinity queue, std::uint64_t value) const -> bool;
    auto wait_cpu(QueueAffinity queue, std::uint64_t value) -> void;
    [[nodiscard]] auto current_value(QueueAffinity queue) const -> std::uint64_t;
    auto advance_frame() -> void;
  };

} // namespace harmonius::rg::sync
