/// @file harmonius.gpu_runtime.memory.cppm
/// @brief GPU memory management — heap sub-allocation (TLSF), ring buffers,
///        pool allocation, defragmentation, and budget tracking.
///
/// Replaces VMA and D3D12MA as the sole memory management layer. Built entirely
/// on top of harmonius::gpu device methods (create_heap, create_placed_texture, etc.).
///
/// Requirements: GR-1.1–GR-1.11

export module harmonius.gpu_runtime.memory;

import std;
import harmonius.gpu;

export namespace harmonius::gpu_runtime::memory
{

  // ---------------------------------------------------------------------------
  // Enums
  // ---------------------------------------------------------------------------

  enum class HeapType : std::uint8_t
  {
    device_local,
    host_visible,
    host_cached,
  };

  enum class AllocationStrategy : std::uint8_t
  {
    sub_allocate,
    committed,
    placed,
  };

  enum class AllocationError : std::uint8_t
  {
    out_of_memory,
    budget_exceeded,
    pool_exhausted,
    invalid_alignment,
  };

  // ---------------------------------------------------------------------------
  // Configuration and result types
  // ---------------------------------------------------------------------------

  struct HeapConfig
  {
    HeapType type;
    std::uint64_t block_size;
    std::uint32_t max_blocks;
  };

  struct Allocation
  {
    gpu::HeapHandle heap;
    std::uint64_t offset;
    std::uint64_t size;
    HeapType heap_type;
    AllocationStrategy strategy;
  };

  struct AllocationDesc
  {
    HeapType heap_type = HeapType::device_local;
    AllocationStrategy strategy = AllocationStrategy::sub_allocate;
    std::uint64_t size = 0;
    std::uint64_t alignment = 0;
    std::string_view debug_name = {};
  };

  struct BudgetInfo
  {
    std::uint64_t total_budget;
    std::uint64_t current_usage;
    std::uint64_t peak_usage;
    float utilization;
  };

  struct AllocationStats
  {
    std::uint64_t total_allocated;
    std::uint64_t total_freed;
    std::uint32_t active_allocations;
    std::uint32_t total_heaps;
    float avg_fragmentation;
  };

  // ---------------------------------------------------------------------------
  // Block allocator (TLSF)
  // ---------------------------------------------------------------------------

  class BlockAllocator
  {
  public:
    explicit BlockAllocator(std::uint64_t pool_size);

    struct Block
    {
      std::uint64_t offset;
      std::uint64_t size;
    };

    [[nodiscard]] auto allocate(std::uint64_t size, std::uint64_t alignment)
        -> std::expected<Block, AllocationError>;

    auto free(Block block) -> void;

    [[nodiscard]] auto fragmentation_ratio() const -> float;
    [[nodiscard]] auto largest_free_block() const -> std::uint64_t;
    [[nodiscard]] auto total_free() const -> std::uint64_t;

  private:
    std::uint64_t pool_size_;
  };

  // ---------------------------------------------------------------------------
  // Heap block
  // ---------------------------------------------------------------------------

  class HeapBlock
  {
  public:
    [[nodiscard]] auto allocate(std::uint64_t size, std::uint64_t alignment)
        -> std::expected<BlockAllocator::Block, AllocationError>;

    auto free(BlockAllocator::Block block) -> void;

    [[nodiscard]] auto fragmentation_ratio() const -> float;
    [[nodiscard]] auto total_free() const -> std::uint64_t;

  private:
    gpu::HeapHandle heap_;
    BlockAllocator allocator_;
    std::uint64_t size_;
  };

  // ---------------------------------------------------------------------------
  // Heap pool
  // ---------------------------------------------------------------------------

  class HeapPool
  {
  public:
    struct SubAllocation
    {
      gpu::HeapHandle heap;
      std::uint64_t offset;
      std::uint64_t size;
    };

    [[nodiscard]] auto allocate(std::uint64_t size, std::uint64_t alignment)
        -> std::expected<SubAllocation, AllocationError>;

    auto free(SubAllocation alloc) -> void;

    [[nodiscard]] auto fragmentation() const -> float;

  private:
    HeapType type_;
    std::uint64_t block_size_;
    std::uint32_t max_blocks_;
    std::vector<HeapBlock> blocks_;
  };

  // ---------------------------------------------------------------------------
  // Budget tracker
  // ---------------------------------------------------------------------------

  class BudgetTracker
  {
  public:
    [[nodiscard]] auto check(HeapType type, std::uint64_t size) const -> bool;
    auto record_alloc(HeapType type, std::uint64_t size) -> void;
    auto record_free(HeapType type, std::uint64_t size) -> void;
    auto refresh() -> void;
    [[nodiscard]] auto budget(HeapType type) const -> BudgetInfo;

  private:
    std::array<BudgetInfo, 3> budgets_;
  };

  // ---------------------------------------------------------------------------
  // Allocator
  // ---------------------------------------------------------------------------

  class Allocator
  {
  public:
    explicit Allocator(gpu::DeviceCapabilities caps, std::span<const HeapConfig> configs);

    [[nodiscard]] auto allocate(const AllocationDesc &desc)
        -> std::expected<Allocation, AllocationError>;

    auto free(const Allocation &alloc) -> void;

    [[nodiscard]] auto create_texture(
        const gpu::TextureDesc &desc,
        AllocationStrategy strategy = AllocationStrategy::sub_allocate)
        -> std::expected<std::pair<gpu::TextureHandle, Allocation>, AllocationError>;

    [[nodiscard]] auto create_buffer(
        const gpu::BufferDesc &desc,
        AllocationStrategy strategy = AllocationStrategy::sub_allocate)
        -> std::expected<std::pair<gpu::BufferHandle, Allocation>, AllocationError>;

    auto destroy_texture(gpu::TextureHandle handle, const Allocation &alloc) -> void;
    auto destroy_buffer(gpu::BufferHandle handle, const Allocation &alloc) -> void;

    [[nodiscard]] auto budget(HeapType type) const -> BudgetInfo;
    [[nodiscard]] auto stats() const -> AllocationStats;

  private:
    std::array<HeapPool, 3> pools_;
    BudgetTracker budget_;
  };

  // ---------------------------------------------------------------------------
  // Ring allocator
  // ---------------------------------------------------------------------------

  class RingAllocator
  {
  public:
    struct Config
    {
      std::uint64_t total_size;
      std::uint32_t frame_count;
    };

    struct RingAllocation
    {
      gpu::BufferHandle buffer;
      std::uint64_t offset;
      std::uint64_t size;
      void *mapped;
    };

    [[nodiscard]] auto allocate(std::uint64_t size, std::uint64_t alignment = 256)
        -> std::expected<RingAllocation, AllocationError>;

    auto begin_frame(std::uint64_t frame_index) -> void;

    [[nodiscard]] auto used_this_frame() const -> std::uint64_t;
    [[nodiscard]] auto capacity_per_frame() const -> std::uint64_t;

  private:
    gpu::BufferHandle buffer_;
    void *mapped_ptr_ = nullptr;
    std::uint64_t total_size_;
    std::uint32_t frame_count_;
    std::uint64_t current_offset_;
  };

  // ---------------------------------------------------------------------------
  // Pool allocator
  // ---------------------------------------------------------------------------

  struct PoolDesc
  {
    std::string_view name;
    HeapType heap_type;
    std::uint64_t element_size;
    std::uint32_t max_elements;
  };

  class PoolAllocator
  {
  public:
    struct PoolSlot
    {
      std::uint32_t index;
      std::uint64_t heap_offset;
    };

    [[nodiscard]] auto allocate() -> std::expected<PoolSlot, AllocationError>;
    auto free(PoolSlot slot) -> void;

    [[nodiscard]] auto active_count() const -> std::uint32_t;
    [[nodiscard]] auto capacity() const -> std::uint32_t;
    [[nodiscard]] auto utilization() const -> float;

  private:
    gpu::HeapHandle heap_;
    PoolDesc desc_;
    std::vector<bool> slot_active_;
    std::uint32_t free_head_;
  };

  // ---------------------------------------------------------------------------
  // Defragmentation engine
  // ---------------------------------------------------------------------------

  struct DefragStats
  {
    std::uint32_t moves_completed;
    std::uint32_t moves_remaining;
    std::uint64_t bytes_moved;
    std::uint64_t bytes_freed;
    float fragmentation_before;
    float fragmentation_after;
  };

  class DefragEngine
  {
  public:
    struct DefragStep
    {
      std::vector<Allocation> old_allocs;
      std::vector<Allocation> new_allocs;
    };

    [[nodiscard]] auto needs_defrag(HeapType type, float threshold = 0.2f) const -> bool;
    [[nodiscard]] auto plan_step(HeapType type, std::uint32_t max_moves = 8) -> DefragStep;
    auto commit_step(const DefragStep &step) -> void;
    [[nodiscard]] auto stats(HeapType type) const -> DefragStats;

  private:
    Allocator *allocator_;
  };

} // namespace harmonius::gpu_runtime::memory
