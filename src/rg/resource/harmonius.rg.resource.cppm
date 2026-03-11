export module harmonius.rg.resource;

import harmonius.gpu;
import harmonius.rg;

import std;

export namespace harmonius::rg::resource {

// ---------------------------------------------------------------------------
// Lifetime and aliasing structs
// ---------------------------------------------------------------------------

/// Describes the lifetime of a resource as the range of passes that read/write it.
/// @threadsafety Instances are not thread-safe.
struct LifetimeInterval {
  ResourceHandle resource;
  std::uint32_t first_write;
  std::uint32_t last_read;
};

/// Describes the heap placement of a resource after aliasing analysis.
/// @threadsafety Instances are not thread-safe.
struct AliasingAssignment {
  ResourceHandle resource;
  std::uint64_t heap_offset;
  std::uint64_t heap_size;
  std::uint32_t heap_index;
};

/// Describes the size and heap type of a resource.
/// @threadsafety Instances are not thread-safe.
struct ResourceSizeInfo {
  ResourceHandle resource;
  std::uint64_t size_bytes;
  gpu::HeapType heap_type;
};

/// Errors that can occur during resource pool operations.
enum class PoolError : std::uint8_t {
  kOutOfMemory,
  kPoolExhausted,
  kInvalidHandle,
};

// ---------------------------------------------------------------------------
// Residency tracking
// ---------------------------------------------------------------------------

/// Tracks the residency state of a resource.
/// @threadsafety Instances are not thread-safe.
struct ResidencyEntry {
  ResourceHandle resource;
  bool is_resident;
  std::uint32_t frame_last_used;
};

/// Describes a pending residency state change for a resource.
/// @threadsafety Instances are not thread-safe.
struct ResidencyChange {
  ResourceHandle resource;
  bool becoming_resident;
};

// ---------------------------------------------------------------------------
// Aliasing map
// ---------------------------------------------------------------------------

/// The result of aliasing analysis, mapping resources to shared heap regions.
/// @threadsafety Instances are not thread-safe.
class AliasingMap {
 public:
  /// Returns the aliasing assignments for all resources.
  [[nodiscard]] auto Assignments() const -> std::span<const AliasingAssignment>;

  /// Returns the peak memory usage in bytes across all heaps.
  [[nodiscard]] auto PeakMemoryBytes() const -> std::uint64_t;

  /// Returns the total logical memory required without aliasing.
  [[nodiscard]] auto TotalLogicalBytes() const -> std::uint64_t;

  /// Returns the aliasing efficiency as a ratio of peak to total logical bytes.
  [[nodiscard]] auto AliasingEfficiency() const -> float;

 private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
};

// ---------------------------------------------------------------------------
// Aliasing solver
// ---------------------------------------------------------------------------

/// Solves resource aliasing to minimize peak memory usage.
/// @threadsafety Instances are not thread-safe.
class AliasingSolver {
 public:
  /// Computes optimal aliasing assignments for the given resource lifetimes and sizes.
  [[nodiscard]] auto Solve(std::span<const LifetimeInterval> lifetimes, std::span<const ResourceSizeInfo> sizes)
      -> AliasingMap;
};

// ---------------------------------------------------------------------------
// Pool allocator
// ---------------------------------------------------------------------------

/// Allocates GPU buffers from a fixed-size pool.
/// @threadsafety Instances are not thread-safe.
class PoolAllocator {
 public:
  /// Allocates a buffer of the given size from the pool.
  [[nodiscard]] auto Allocate(std::uint64_t size) -> std::expected<gpu::BufferHandle, PoolError>;

  /// Releases a previously allocated buffer back to the pool.
  auto Release(gpu::BufferHandle handle) -> void;

  /// Returns the current utilization of the pool as a ratio.
  [[nodiscard]] auto Utilization() const -> float;

  /// Returns the total capacity of the pool in bytes.
  [[nodiscard]] auto Capacity() const -> std::uint64_t;

  /// Returns the number of active allocations.
  [[nodiscard]] auto ActiveCount() const -> std::uint32_t;
};

// ---------------------------------------------------------------------------
// Ring allocator
// ---------------------------------------------------------------------------

/// Allocates sub-regions from a ring buffer, advancing per frame.
/// @threadsafety Instances are not thread-safe.
class RingAllocator {
 public:
  /// A sub-allocation within the ring buffer.
  /// @threadsafety Instances are not thread-safe.
  struct Allocation {
    gpu::BufferHandle buffer;
    std::uint64_t offset;
    std::uint64_t size;
  };

  /// Allocates a region of the given size and alignment from the ring buffer.
  [[nodiscard]] auto Allocate(std::uint64_t size, std::uint64_t alignment = 256)
      -> std::expected<Allocation, PoolError>;

  /// Advances the ring buffer to the next frame, reclaiming old allocations.
  auto AdvanceFrame() -> void;

  /// Returns the underlying GPU buffer handle.
  [[nodiscard]] auto Buffer() const -> gpu::BufferHandle;
};

}  // namespace harmonius::rg::resource
