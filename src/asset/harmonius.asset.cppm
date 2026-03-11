export module harmonius.asset;

import harmonius.gpu;
import harmonius.rg.exec;

import std;

export namespace harmonius::asset {

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// The type of an asset in the pipeline.
enum class AssetType : std::uint8_t {
    kMesh,
    kTexture,
    kMaterial,
    kTerrainTile,
    kShader,
    kAccelerationStructure,
};

/// Priority level for asset streaming requests.
enum class StreamPriority : std::uint8_t {
    kCritical = 0,
    kHigh = 1,
    kNormal = 2,
    kLow = 3,
};

// ---------------------------------------------------------------------------
// Identity and handle types
// ---------------------------------------------------------------------------

/// Unique identifier for an asset, derived from its content hash.
enum class AssetId : std::uint64_t {
    kInvalid = 0,
};

/// Generational handle to a registered asset resource.
struct AssetHandle {
    std::uint32_t index;
    std::uint32_t generation;

    /// Returns true if this handle refers to a valid asset slot.
    [[nodiscard]] auto IsValid() const -> bool;
};

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors that can occur during asset cooking.
enum class CookError : std::uint8_t {
    kUnsupportedType,
    kInvalidData,
    kProcessorNotRegistered,
};

/// Errors that can occur during asset operations.
enum class AssetError : std::uint8_t {
    kNotFound,
    kInvalidHandle,
    kTypeMismatch,
    kLoadFailed,
};

/// Errors that can occur during heap operations.
enum class HeapError : std::uint8_t {
    kOutOfCapacity,
    kInvalidAllocation,
};

// ---------------------------------------------------------------------------
// Raw asset
// ---------------------------------------------------------------------------

/// An unprocessed asset loaded from its source file.
struct RawAsset {
    AssetId id;
    AssetType type;
    std::string source_path;
    std::vector<std::uint8_t> raw_data;
};

// ---------------------------------------------------------------------------
// Cooked asset types
// ---------------------------------------------------------------------------

/// Metadata describing a cooked asset's GPU resource properties.
struct CookedMetadata {
    std::uint32_t meshlet_count = 0;
    std::uint32_t vertex_count = 0;
    std::uint32_t triangle_count = 0;
    gpu::Format format = gpu::Format::kUndefined;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t mip_levels = 0;
    std::uint32_t array_layers = 0;
    std::uint64_t gpu_size_bytes = 0;
};

/// A fully processed asset ready for GPU upload.
struct CookedAsset {
    AssetId id;
    AssetType type;
    std::string_view target_backend;
    std::vector<std::uint8_t> data;
    std::uint64_t content_hash;
    CookedMetadata metadata;
};

// ---------------------------------------------------------------------------
// Meshlet types
// ---------------------------------------------------------------------------

/// Mesh data decomposed into GPU-friendly meshlet clusters.
struct MeshletData {
    /// A single meshlet cluster within the mesh.
    struct Meshlet {
        std::uint32_t vertex_offset;
        std::uint32_t triangle_offset;
        std::uint8_t vertex_count;
        std::uint8_t triangle_count;
        std::array<float, 4> bounding_sphere;
        std::array<float, 4> normal_cone;
    };

    std::vector<Meshlet> meshlets;
    std::vector<std::uint8_t> vertex_data;
    std::vector<std::uint8_t> triangle_data;
    std::uint32_t vertex_stride;
};

// ---------------------------------------------------------------------------
// Texture cook options
// ---------------------------------------------------------------------------

/// Configuration options for texture compression during cooking.
struct TextureCookOptions {
    gpu::Format target_format = gpu::Format::kBc7Unorm;
    bool generate_mips = true;
    bool srgb = false;
    std::uint32_t max_dimension = 4096;
};

// ---------------------------------------------------------------------------
// Bundle manifest types
// ---------------------------------------------------------------------------

/// Manifest describing the layout of bundled assets on disk.
struct BundleManifest {
    /// An entry describing a single bundle file.
    struct BundleEntry {
        std::string bundle_path;
        std::uint64_t total_size;
        std::vector<AssetId> asset_ids;
    };

    /// An entry describing a single asset within a bundle.
    struct AssetEntry {
        AssetId id;
        AssetType type;
        std::uint64_t gpu_size;
        std::uint32_t bundle_index;
        std::uint32_t chunk_index;
        std::uint64_t chunk_offset;
        std::uint64_t chunk_size;
        std::uint16_t priority_bias;
        CookedMetadata metadata;
    };

    std::vector<BundleEntry> bundles;
    std::vector<AssetEntry> assets;

    /// Finds an asset entry by its identifier, or returns nullptr.
    [[nodiscard]] auto Find(AssetId id) const -> const AssetEntry*;
};

// ---------------------------------------------------------------------------
// Chunk layout
// ---------------------------------------------------------------------------

/// Describes the memory layout of asset chunks within a bundle.
struct ChunkLayout {
    static constexpr std::uint64_t kAlignment = 65536;

    /// A contiguous region within a chunk.
    struct Region {
        std::uint64_t offset;
        std::uint64_t size;
        AssetType type;
    };

    std::vector<Region> regions;
    std::uint64_t total_size;
};

// ---------------------------------------------------------------------------
// Streaming types
// ---------------------------------------------------------------------------

/// A request to stream an asset chunk into GPU memory.
struct StreamRequest {
    AssetId asset_id;
    std::uint32_t chunk_index;
    StreamPriority priority;
    float camera_distance;
};

/// The result of a completed asynchronous I/O operation.
struct IOCompletion {
    AssetId asset_id;
    gpu::BufferHandle staging_buffer;
    std::uint64_t buffer_offset;
    std::uint64_t size;
    bool success;
};

// ---------------------------------------------------------------------------
// IoBackend concept
// ---------------------------------------------------------------------------

/// An I/O backend capable of submitting asynchronous file reads.
template <typename B>
concept IoBackend = requires(B b, std::string_view path, std::uint64_t file_offset,
                             std::uint64_t size, gpu::BufferHandle staging_buffer,
                             std::uint64_t buffer_offset) {
    { b.SubmitRead(path, file_offset, size, staging_buffer, buffer_offset) } -> std::same_as<void>;
    { b.PollCompletions() } -> std::same_as<std::vector<IOCompletion>>;
};

// ---------------------------------------------------------------------------
// AssetCooker
// ---------------------------------------------------------------------------

/// Transforms raw assets into cooked, GPU-ready representations.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
class AssetCooker {
 public:
    explicit AssetCooker(std::string_view target_backend);
    ~AssetCooker();

    AssetCooker(const AssetCooker&) = delete;
    auto operator=(const AssetCooker&) -> AssetCooker& = delete;

    AssetCooker(AssetCooker&&) noexcept;
    auto operator=(AssetCooker&&) noexcept -> AssetCooker&;

    /// Cooks a single raw asset into its GPU-ready form.
    [[nodiscard]] auto Cook(const RawAsset& raw) -> std::expected<CookedAsset, CookError>;

    /// Cooks a batch of raw assets, returning results for each.
    [[nodiscard]] auto CookBatch(std::span<const RawAsset> assets)
        -> std::vector<std::expected<CookedAsset, CookError>>;

    /// Registers a custom processor function for the given asset type.
    auto RegisterProcessor(AssetType type, std::move_only_function<CookedAsset(const RawAsset&)> fn) -> void;
};

// ---------------------------------------------------------------------------
// MeshletBuilder
// ---------------------------------------------------------------------------

/// Builds meshlet data from raw vertex and index buffers.
///
/// @threadsafety Not thread-safe.
class MeshletBuilder {
 public:
    /// Decomposes a mesh into meshlet clusters.
    [[nodiscard]] auto Build(std::span<const float> positions,
                             std::span<const std::uint32_t> indices,
                             std::uint32_t vertex_stride) -> MeshletData;
};

// ---------------------------------------------------------------------------
// TextureCompressor
// ---------------------------------------------------------------------------

/// Compresses RGBA texture data into block-compressed GPU formats.
///
/// @threadsafety Not thread-safe.
class TextureCompressor {
 public:
    /// Compresses raw RGBA data according to the provided options.
    [[nodiscard]] auto Compress(std::span<const std::uint8_t> rgba_data, std::uint32_t width,
                                std::uint32_t height,
                                const TextureCookOptions& options) -> CookedAsset;
};

// ---------------------------------------------------------------------------
// StreamingScheduler
// ---------------------------------------------------------------------------

/// Schedules and prioritizes asset streaming from disk to GPU memory.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
template <IoBackend B>
class StreamingScheduler {
 public:
    /// Constructs a streaming scheduler with the given I/O backend.
    explicit StreamingScheduler(B& backend);
    ~StreamingScheduler();

    StreamingScheduler(const StreamingScheduler&) = delete;
    auto operator=(const StreamingScheduler&) -> StreamingScheduler& = delete;

    StreamingScheduler(StreamingScheduler&&) noexcept;
    auto operator=(StreamingScheduler&&) noexcept -> StreamingScheduler&;

    /// Submits a batch of streaming requests for prioritization.
    auto Request(std::span<const StreamRequest> requests) -> void;

    /// Processes pending requests and returns transfer pass descriptors.
    [[nodiscard]] auto ProcessPending() -> std::vector<rg::exec::TransferPassDesc>;

    /// Sets the eviction policy used when memory pressure requires unloading.
    auto SetEvictionPolicy(std::move_only_function<std::vector<AssetId>(std::uint32_t)> policy) -> void;

    /// Returns whether the asset with the given identifier is resident in GPU memory.
    [[nodiscard]] auto IsResident(AssetId id) const -> bool;

    /// Returns the ratio of resident assets to total requested assets.
    [[nodiscard]] auto ResidencyRatio() const -> float;

 private:
    B* backend_;
};

// ---------------------------------------------------------------------------
// ResourceRegistry
// ---------------------------------------------------------------------------

/// Tracks mappings between asset identifiers and GPU resource handles.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
class ResourceRegistry {
 public:
    /// Registers a GPU texture for the given asset and returns a handle.
    [[nodiscard]] auto RegisterTexture(AssetId id, gpu::TextureHandle gpu_texture,
                                       const CookedMetadata& metadata) -> AssetHandle;

    /// Registers a GPU buffer for the given asset and returns a handle.
    [[nodiscard]] auto RegisterBuffer(AssetId id, gpu::BufferHandle gpu_buffer,
                                      const CookedMetadata& metadata) -> AssetHandle;

    /// Unregisters the asset associated with the given handle.
    auto Unregister(AssetHandle handle) -> void;

    /// Resolves an asset handle to its underlying GPU texture handle.
    [[nodiscard]] auto ResolveTexture(AssetHandle handle) const
        -> std::expected<gpu::TextureHandle, AssetError>;

    /// Resolves an asset handle to its underlying GPU buffer handle.
    [[nodiscard]] auto ResolveBuffer(AssetHandle handle) const
        -> std::expected<gpu::BufferHandle, AssetError>;

    /// Resolves an asset handle to its bindless descriptor index.
    [[nodiscard]] auto ResolveDescriptorIndex(AssetHandle handle) const
        -> std::expected<std::uint32_t, AssetError>;

    /// Returns the number of currently registered assets.
    [[nodiscard]] auto RegisteredCount() const -> std::uint32_t;

    /// Returns the total GPU memory in bytes used by registered assets.
    [[nodiscard]] auto TotalGpuBytes() const -> std::uint64_t;
};

// ---------------------------------------------------------------------------
// BindlessDescriptorHeap
// ---------------------------------------------------------------------------

/// Manages a bindless descriptor heap for GPU resource indexing.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
class BindlessDescriptorHeap {
 public:
    explicit BindlessDescriptorHeap(std::uint32_t max_descriptors = 1'000'000);
    ~BindlessDescriptorHeap();

    BindlessDescriptorHeap(const BindlessDescriptorHeap&) = delete;
    auto operator=(const BindlessDescriptorHeap&) -> BindlessDescriptorHeap& = delete;

    BindlessDescriptorHeap(BindlessDescriptorHeap&&) noexcept;
    auto operator=(BindlessDescriptorHeap&&) noexcept -> BindlessDescriptorHeap&;

    /// Allocates a descriptor slot for a texture and returns its index.
    [[nodiscard]] auto AllocateTexture(gpu::TextureHandle texture)
        -> std::expected<std::uint32_t, HeapError>;

    /// Allocates a descriptor slot for a buffer and returns its index.
    [[nodiscard]] auto AllocateBuffer(gpu::BufferHandle buffer)
        -> std::expected<std::uint32_t, HeapError>;

    /// Frees the descriptor slot at the given index.
    auto Free(std::uint32_t index) -> void;

    /// Updates the texture at the given descriptor index.
    auto UpdateTexture(std::uint32_t index, gpu::TextureHandle texture) -> void;

    /// Updates the buffer at the given descriptor index.
    auto UpdateBuffer(std::uint32_t index, gpu::BufferHandle buffer) -> void;

    /// Returns the GPU handle to the underlying descriptor heap.
    [[nodiscard]] auto GpuHandle() const -> gpu::DescriptorHeapHandle;

    /// Returns the number of currently allocated descriptors.
    [[nodiscard]] auto AllocatedCount() const -> std::uint32_t;

    /// Returns the maximum capacity of the descriptor heap.
    [[nodiscard]] auto Capacity() const -> std::uint32_t;
};

}  // namespace harmonius::asset
