/// @file harmonius.gpu_runtime.memory.cppm
/// @brief GPU memory management — heap sub-allocation (TLSF), ring buffers,
///        pool allocation, defragmentation, and budget tracking.
///
/// Replaces VMA and D3D12MA as the sole memory management layer. Built entirely
/// on top of harmonius::gpu device methods (create_heap, create_placed_texture,
/// etc.).
///
/// Requirements: GR-1.1–GR-1.11

export module harmonius.gpu_runtime.memory;

import harmonius.gpu;

import std;

export namespace harmonius::gpu_runtime::memory {

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// GPU heap residency type.
enum class HeapType : std::uint8_t {
    kDeviceLocal,
    kHostVisible,
    kHostCached,
};

/// Strategy used when placing a resource in GPU memory.
enum class AllocationStrategy : std::uint8_t {
    kSubAllocate,
    kCommitted,
    kPlaced,
};

/// Errors that can occur during GPU memory allocation.
enum class AllocationError : std::uint8_t {
    kOutOfMemory,
    kBudgetExceeded,
    kPoolExhausted,
    kInvalidAlignment,
};

// ---------------------------------------------------------------------------
// Configuration and result types
// ---------------------------------------------------------------------------

/// Configuration for a single GPU heap tier.
struct HeapConfig {
    HeapType type;
    std::uint64_t block_size;
    std::uint32_t max_blocks;
};

/// Describes a completed GPU memory allocation.
struct Allocation {
    gpu::HeapHandle heap;
    std::uint64_t offset;
    std::uint64_t size;
    HeapType heap_type;
    AllocationStrategy strategy;
};

/// Descriptor for requesting a GPU memory allocation.
struct AllocationDesc {
    HeapType heap_type = HeapType::kDeviceLocal;
    AllocationStrategy strategy = AllocationStrategy::kSubAllocate;
    std::uint64_t size = 0;
    std::uint64_t alignment = 0;
    std::string_view debug_name = {};
};

/// Current memory budget information for a heap type.
struct BudgetInfo {
    std::uint64_t total_budget;
    std::uint64_t current_usage;
    std::uint64_t peak_usage;
    float utilization;
};

/// Aggregate statistics across all active allocations.
struct AllocationStats {
    std::uint64_t total_allocated;
    std::uint64_t total_freed;
    std::uint32_t active_allocations;
    std::uint32_t total_heaps;
    float avg_fragmentation;
};

// ---------------------------------------------------------------------------
// Block allocator (TLSF)
// ---------------------------------------------------------------------------

/// TLSF-based block allocator for GPU heap sub-allocation.
/// @threadsafety Instances are not thread-safe.
class BlockAllocator {
 public:
    explicit BlockAllocator(std::uint64_t pool_size);

    /// A contiguous region within a block.
    struct Block {
        std::uint64_t offset;
        std::uint64_t size;
    };

    /// Allocate a region with the given size and alignment.
    [[nodiscard]] auto Allocate(std::uint64_t size, std::uint64_t alignment)
        -> std::expected<Block, AllocationError>;

    /// Free a previously allocated block.
    auto Free(Block block) -> void;

    /// Returns the fragmentation ratio in [0, 1].
    [[nodiscard]] auto FragmentationRatio() const -> float;

    /// Returns the size of the largest contiguous free region.
    [[nodiscard]] auto LargestFreeBlock() const -> std::uint64_t;

    /// Returns the total number of free bytes.
    [[nodiscard]] auto TotalFree() const -> std::uint64_t;

 private:
    std::uint64_t pool_size_;
};

// ---------------------------------------------------------------------------
// Heap block
// ---------------------------------------------------------------------------

/// A single GPU heap block backed by a TLSF allocator.
/// @threadsafety Instances are not thread-safe.
class HeapBlock {
 public:
    HeapBlock(const HeapBlock&) = delete;
    auto operator=(const HeapBlock&) -> HeapBlock& = delete;
    HeapBlock(HeapBlock&&) noexcept;
    auto operator=(HeapBlock&&) noexcept -> HeapBlock&;
    ~HeapBlock();

    /// Sub-allocate a region from this heap block.
    [[nodiscard]] auto Allocate(std::uint64_t size, std::uint64_t alignment)
        -> std::expected<BlockAllocator::Block, AllocationError>;

    /// Free a sub-allocated region.
    auto Free(BlockAllocator::Block block) -> void;

    /// Returns the fragmentation ratio in [0, 1].
    [[nodiscard]] auto FragmentationRatio() const -> float;

    /// Returns the total number of free bytes.
    [[nodiscard]] auto TotalFree() const -> std::uint64_t;

 private:
    gpu::HeapHandle heap_;
    BlockAllocator allocator_;
    std::uint64_t size_;
};

// ---------------------------------------------------------------------------
// Heap pool
// ---------------------------------------------------------------------------

/// Collection of heap blocks for a single heap type.
/// @threadsafety Instances are not thread-safe.
class HeapPool {
 public:
    /// A sub-allocation within a heap pool.
    struct SubAllocation {
        gpu::HeapHandle heap;
        std::uint64_t offset;
        std::uint64_t size;
    };

    /// Allocate a region from the pool, growing if necessary.
    [[nodiscard]] auto Allocate(std::uint64_t size, std::uint64_t alignment)
        -> std::expected<SubAllocation, AllocationError>;

    /// Free a previously allocated region.
    auto Free(SubAllocation alloc) -> void;

    /// Returns the average fragmentation across all blocks.
    [[nodiscard]] auto Fragmentation() const -> float;

 private:
    HeapType type_;
    std::uint64_t block_size_;
    std::uint32_t max_blocks_;
    std::vector<HeapBlock> blocks_;
};

// ---------------------------------------------------------------------------
// Budget tracker
// ---------------------------------------------------------------------------

/// Tracks GPU memory budgets per heap type.
/// @threadsafety Instances are not thread-safe.
class BudgetTracker {
 public:
    /// Returns true if an allocation of the given size is within budget.
    [[nodiscard]] auto Check(HeapType type, std::uint64_t size) const -> bool;

    /// Record a new allocation against the budget.
    auto RecordAlloc(HeapType type, std::uint64_t size) -> void;

    /// Record a freed allocation against the budget.
    auto RecordFree(HeapType type, std::uint64_t size) -> void;

    /// Refresh budget data from the OS/driver.
    auto Refresh() -> void;

    /// Returns the current budget info for the given heap type.
    [[nodiscard]] auto Budget(HeapType type) const -> BudgetInfo;

 private:
    std::array<BudgetInfo, 3> budgets_;
};

// ---------------------------------------------------------------------------
// Allocator
// ---------------------------------------------------------------------------

/// Top-level GPU memory allocator managing pools, budgets, and resource creation.
/// @threadsafety Instances are not thread-safe.
class Allocator {
 public:
    explicit Allocator(gpu::DeviceCapabilities caps, std::span<const HeapConfig> configs);

    /// Allocate GPU memory according to the given descriptor.
    [[nodiscard]] auto Allocate(const AllocationDesc& desc) -> std::expected<Allocation, AllocationError>;

    /// Free a previously allocated region.
    auto Free(const Allocation& alloc) -> void;

    /// Create a texture with a backing allocation.
    [[nodiscard]] auto CreateTexture(const gpu::TextureDesc& desc,
                                     AllocationStrategy strategy = AllocationStrategy::kSubAllocate)
        -> std::expected<std::pair<gpu::TextureHandle, Allocation>, AllocationError>;

    /// Create a buffer with a backing allocation.
    [[nodiscard]] auto CreateBuffer(const gpu::BufferDesc& desc,
                                    AllocationStrategy strategy = AllocationStrategy::kSubAllocate)
        -> std::expected<std::pair<gpu::BufferHandle, Allocation>, AllocationError>;

    /// Destroy a texture and free its backing allocation.
    auto DestroyTexture(gpu::TextureHandle handle, const Allocation& alloc) -> void;

    /// Destroy a buffer and free its backing allocation.
    auto DestroyBuffer(gpu::BufferHandle handle, const Allocation& alloc) -> void;

    /// Returns the current budget info for the given heap type.
    [[nodiscard]] auto Budget(HeapType type) const -> BudgetInfo;

    /// Returns aggregate allocation statistics.
    [[nodiscard]] auto Stats() const -> AllocationStats;

 private:
    std::array<HeapPool, 3> pools_;
    BudgetTracker budget_;
};

// ---------------------------------------------------------------------------
// Ring allocator
// ---------------------------------------------------------------------------

/// Per-frame ring buffer allocator for transient upload data.
/// @threadsafety Instances are not thread-safe.
class RingAllocator {
 public:
    /// Configuration for a ring allocator.
    struct Config {
        std::uint64_t total_size;
        std::uint32_t frame_count;
    };

    /// Result of a ring allocation.
    struct RingAllocation {
        gpu::BufferHandle buffer;
        std::uint64_t offset;
        std::uint64_t size;
        void* mapped;
    };

    /// Allocate transient memory from the ring buffer.
    [[nodiscard]] auto Allocate(std::uint64_t size, std::uint64_t alignment = 256)
        -> std::expected<RingAllocation, AllocationError>;

    /// Advance to the next frame, reclaiming the oldest frame's memory.
    auto BeginFrame(std::uint64_t frame_index) -> void;

    /// Returns the number of bytes used in the current frame.
    [[nodiscard]] auto UsedThisFrame() const -> std::uint64_t;

    /// Returns the per-frame capacity.
    [[nodiscard]] auto CapacityPerFrame() const -> std::uint64_t;

 private:
    gpu::BufferHandle buffer_;
    void* mapped_ptr_ = nullptr;
    std::uint64_t total_size_;
    std::uint32_t frame_count_;
    std::uint64_t current_offset_;
};

// ---------------------------------------------------------------------------
// Pool allocator
// ---------------------------------------------------------------------------

/// Descriptor for a fixed-size element pool.
struct PoolDesc {
    std::string_view name;
    HeapType heap_type;
    std::uint64_t element_size;
    std::uint32_t max_elements;
};

/// Fixed-size element pool allocator backed by a single GPU heap.
/// @threadsafety Instances are not thread-safe.
class PoolAllocator {
 public:
    /// A slot within the pool.
    struct PoolSlot {
        std::uint32_t index;
        std::uint64_t heap_offset;
    };

    PoolAllocator(const PoolAllocator&) = delete;
    auto operator=(const PoolAllocator&) -> PoolAllocator& = delete;
    PoolAllocator(PoolAllocator&&) noexcept;
    auto operator=(PoolAllocator&&) noexcept -> PoolAllocator&;
    ~PoolAllocator();

    /// Allocate a slot from the pool.
    [[nodiscard]] auto Allocate() -> std::expected<PoolSlot, AllocationError>;

    /// Free a previously allocated slot.
    auto Free(PoolSlot slot) -> void;

    /// Returns the number of active allocations.
    [[nodiscard]] auto ActiveCount() const -> std::uint32_t;

    /// Returns the maximum number of elements.
    [[nodiscard]] auto Capacity() const -> std::uint32_t;

    /// Returns the utilization ratio in [0, 1].
    [[nodiscard]] auto Utilization() const -> float;

 private:
    gpu::HeapHandle heap_;
    PoolDesc desc_;
    std::vector<bool> slot_active_;
    std::uint32_t free_head_;
};

// ---------------------------------------------------------------------------
// Defragmentation engine
// ---------------------------------------------------------------------------

/// Aggregate statistics for a defragmentation pass.
struct DefragStats {
    std::uint32_t moves_completed;
    std::uint32_t moves_remaining;
    std::uint64_t bytes_moved;
    std::uint64_t bytes_freed;
    float fragmentation_before;
    float fragmentation_after;
};

/// Incremental GPU memory defragmentation engine.
/// @threadsafety Instances are not thread-safe.
class DefragEngine {
 public:
    explicit DefragEngine(Allocator& allocator);

    /// A single defragmentation step describing allocation moves.
    struct DefragStep {
        std::vector<Allocation> old_allocs;
        std::vector<Allocation> new_allocs;
    };

    /// Returns true if the given heap type exceeds the fragmentation threshold.
    [[nodiscard]] auto NeedsDefrag(HeapType type, float threshold = 0.2f) const -> bool;

    /// Plan a defragmentation step with up to max_moves relocations.
    [[nodiscard]] auto PlanStep(HeapType type, std::uint32_t max_moves = 8) -> DefragStep;

    /// Commit a previously planned defragmentation step.
    auto CommitStep(const DefragStep& step) -> void;

    /// Returns defragmentation statistics for the given heap type.
    [[nodiscard]] auto Stats(HeapType type) const -> DefragStats;

 private:
    Allocator& allocator_;
};

}  // namespace harmonius::gpu_runtime::memory
