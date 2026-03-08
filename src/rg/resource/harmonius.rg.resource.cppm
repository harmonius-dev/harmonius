export module harmonius.rg.resource;

import std;
import harmonius.gpu;
import harmonius.rg;

export namespace harmonius::rg::resource
{

  // ---------------------------------------------------------------------------
  // Lifetime and aliasing structs
  // ---------------------------------------------------------------------------

  struct LifetimeInterval
  {
    ResourceHandle resource;
    std::uint32_t first_write;
    std::uint32_t last_read;
  };

  struct AliasingAssignment
  {
    ResourceHandle resource;
    std::uint64_t heap_offset;
    std::uint64_t heap_size;
    std::uint32_t heap_index;
  };

  struct ResourceSizeInfo
  {
    ResourceHandle resource;
    std::uint64_t size_bytes;
    gpu::HeapType heap_type;
  };

  struct PoolError
  {
    std::string message;
  };

  // ---------------------------------------------------------------------------
  // Residency tracking
  // ---------------------------------------------------------------------------

  struct ResidencyEntry
  {
    ResourceHandle resource;
    bool is_resident;
    std::uint32_t frame_last_used;
  };

  struct ResidencyChange
  {
    ResourceHandle resource;
    bool becoming_resident;
  };

  // ---------------------------------------------------------------------------
  // Aliasing map
  // ---------------------------------------------------------------------------

  class AliasingMap
  {
  public:
    [[nodiscard]] auto assignments() const -> std::span<const AliasingAssignment>;
    [[nodiscard]] auto peak_memory_bytes() const -> std::uint64_t;
    [[nodiscard]] auto total_logical_bytes() const -> std::uint64_t;
    [[nodiscard]] auto aliasing_efficiency() const -> float;

  private:
    struct Impl;
    std::unique_ptr<Impl> impl_;
  };

  // ---------------------------------------------------------------------------
  // Aliasing solver
  // ---------------------------------------------------------------------------

  class AliasingSolver
  {
  public:
    [[nodiscard]] auto solve(
        std::span<const LifetimeInterval> lifetimes,
        std::span<const ResourceSizeInfo> sizes) -> AliasingMap;
  };

  // ---------------------------------------------------------------------------
  // Pool allocator
  // ---------------------------------------------------------------------------

  class PoolAllocator
  {
  public:
    [[nodiscard]] auto allocate(std::uint64_t size) -> std::expected<gpu::BufferHandle, PoolError>;
    auto release(gpu::BufferHandle handle) -> void;
    [[nodiscard]] auto utilization() const -> float;
    [[nodiscard]] auto capacity() const -> std::uint64_t;
    [[nodiscard]] auto active_count() const -> std::uint32_t;
  };

  // ---------------------------------------------------------------------------
  // Ring allocator
  // ---------------------------------------------------------------------------

  class RingAllocator
  {
  public:
    struct Allocation
    {
      gpu::BufferHandle buffer;
      std::uint64_t offset;
      std::uint64_t size;
    };

    [[nodiscard]] auto allocate(
        std::uint64_t size,
        std::uint64_t alignment = 256) -> std::expected<Allocation, PoolError>;

    auto advance_frame() -> void;

    [[nodiscard]] auto buffer() const -> gpu::BufferHandle;
  };

} // namespace harmonius::rg::resource
