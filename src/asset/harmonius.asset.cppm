export module harmonius.asset;

import std;
import harmonius.gpu;
import harmonius.rg.exec;

export namespace harmonius::asset
{

  // ---------------------------------------------------------------------------
  // Enums
  // ---------------------------------------------------------------------------

  enum class AssetType : std::uint8_t
  {
    mesh,
    texture,
    material,
    terrain_tile,
    shader,
    acceleration_structure,
  };

  enum class StreamPriority : std::uint8_t
  {
    critical = 0,
    high = 1,
    normal = 2,
    low = 3,
  };

  // ---------------------------------------------------------------------------
  // Identity and handle types
  // ---------------------------------------------------------------------------

  enum class AssetId : std::uint64_t
  {
    invalid = 0,
  };

  struct AssetHandle
  {
    std::uint32_t index;
    std::uint32_t generation;

    [[nodiscard]] auto is_valid() const -> bool;
  };

  // ---------------------------------------------------------------------------
  // Error types
  // ---------------------------------------------------------------------------

  struct CookError
  {
    std::string message;
  };

  struct AssetError
  {
    std::string message;
  };

  struct HeapError
  {
    std::string message;
  };

  // ---------------------------------------------------------------------------
  // Raw asset
  // ---------------------------------------------------------------------------

  struct RawAsset
  {
    AssetId id;
    AssetType type;
    std::string source_path;
    std::vector<std::uint8_t> raw_data;
  };

  // ---------------------------------------------------------------------------
  // Cooked asset types
  // ---------------------------------------------------------------------------

  struct CookedMetadata
  {
    std::uint32_t meshlet_count = 0;
    std::uint32_t vertex_count = 0;
    std::uint32_t triangle_count = 0;
    gpu::Format format = gpu::Format::undefined;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t mip_levels = 0;
    std::uint32_t array_layers = 0;
    std::uint64_t gpu_size_bytes = 0;
  };

  struct CookedAsset
  {
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

  struct MeshletData
  {
    struct Meshlet
    {
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

  struct TextureCookOptions
  {
    gpu::Format target_format = gpu::Format::bc7_unorm;
    bool generate_mips = true;
    bool srgb = false;
    std::uint32_t max_dimension = 4096;
  };

  // ---------------------------------------------------------------------------
  // Bundle manifest types
  // ---------------------------------------------------------------------------

  struct BundleManifest
  {
    struct BundleEntry
    {
      std::string bundle_path;
      std::uint64_t total_size;
      std::vector<AssetId> asset_ids;
    };

    struct AssetEntry
    {
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

    [[nodiscard]] auto find(AssetId id) const -> const AssetEntry *;
  };

  // ---------------------------------------------------------------------------
  // Chunk layout
  // ---------------------------------------------------------------------------

  struct ChunkLayout
  {
    static constexpr std::uint64_t alignment = 65536;

    struct Region
    {
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

  struct StreamRequest
  {
    AssetId asset_id;
    std::uint32_t chunk_index;
    StreamPriority priority;
    float camera_distance;
  };

  struct IOCompletion
  {
    AssetId asset_id;
    gpu::BufferHandle staging_buffer;
    std::uint64_t buffer_offset;
    std::uint64_t size;
    bool success;
  };

  // ---------------------------------------------------------------------------
  // AssetCooker
  // ---------------------------------------------------------------------------

  class AssetCooker
  {
  public:
    explicit AssetCooker(std::string_view target_backend);

    [[nodiscard]] auto cook(const RawAsset &raw)
        -> std::expected<CookedAsset, CookError>;

    [[nodiscard]] auto cook_batch(std::span<const RawAsset> assets)
        -> std::vector<std::expected<CookedAsset, CookError>>;

    auto register_processor(
        AssetType type,
        std::function<CookedAsset(const RawAsset &)> fn) -> void;
  };

  // ---------------------------------------------------------------------------
  // MeshletBuilder
  // ---------------------------------------------------------------------------

  class MeshletBuilder
  {
  public:
    [[nodiscard]] auto build(
        std::span<const float> positions,
        std::span<const std::uint32_t> indices,
        std::uint32_t vertex_stride) -> MeshletData;
  };

  // ---------------------------------------------------------------------------
  // TextureCompressor
  // ---------------------------------------------------------------------------

  class TextureCompressor
  {
  public:
    [[nodiscard]] auto compress(
        std::span<const std::uint8_t> rgba_data,
        std::uint32_t width,
        std::uint32_t height,
        const TextureCookOptions &options) -> CookedAsset;
  };

  // ---------------------------------------------------------------------------
  // IOBackend
  // ---------------------------------------------------------------------------

  class IOBackend
  {
  public:
    virtual ~IOBackend() = default;

    virtual auto submit_read(
        std::string_view path,
        std::uint64_t file_offset,
        std::uint64_t size,
        gpu::BufferHandle staging_buffer,
        std::uint64_t buffer_offset) -> void = 0;

    [[nodiscard]] virtual auto poll_completions()
        -> std::vector<IOCompletion> = 0;
  };

  // ---------------------------------------------------------------------------
  // StreamingScheduler
  // ---------------------------------------------------------------------------

  class StreamingScheduler
  {
  public:
    auto request(std::span<const StreamRequest> requests) -> void;

    [[nodiscard]] auto process_pending()
        -> std::vector<rg::exec::TransferPassDesc>;

    auto set_eviction_policy(
        std::function<std::vector<AssetId>(std::uint32_t)> policy) -> void;

    [[nodiscard]] auto is_resident(AssetId id) const -> bool;
    [[nodiscard]] auto residency_ratio() const -> float;

    auto set_io_backend(std::unique_ptr<IOBackend> backend) -> void;
  };

  // ---------------------------------------------------------------------------
  // ResourceRegistry
  // ---------------------------------------------------------------------------

  class ResourceRegistry
  {
  public:
    [[nodiscard]] auto register_texture(
        AssetId id,
        gpu::TextureHandle gpu_texture,
        const CookedMetadata &metadata) -> AssetHandle;

    [[nodiscard]] auto register_buffer(
        AssetId id,
        gpu::BufferHandle gpu_buffer,
        const CookedMetadata &metadata) -> AssetHandle;

    auto unregister(AssetHandle handle) -> void;

    [[nodiscard]] auto resolve_texture(AssetHandle handle) const
        -> std::expected<gpu::TextureHandle, AssetError>;

    [[nodiscard]] auto resolve_buffer(AssetHandle handle) const
        -> std::expected<gpu::BufferHandle, AssetError>;

    [[nodiscard]] auto resolve_descriptor_index(AssetHandle handle) const
        -> std::expected<std::uint32_t, AssetError>;

    [[nodiscard]] auto registered_count() const -> std::uint32_t;
    [[nodiscard]] auto total_gpu_bytes() const -> std::uint64_t;
  };

  // ---------------------------------------------------------------------------
  // BindlessDescriptorHeap
  // ---------------------------------------------------------------------------

  class BindlessDescriptorHeap
  {
  public:
    explicit BindlessDescriptorHeap(
        std::uint32_t max_descriptors = 1'000'000);

    [[nodiscard]] auto allocate_texture(gpu::TextureHandle texture)
        -> std::expected<std::uint32_t, HeapError>;

    [[nodiscard]] auto allocate_buffer(gpu::BufferHandle buffer)
        -> std::expected<std::uint32_t, HeapError>;

    auto free(std::uint32_t index) -> void;

    auto update_texture(std::uint32_t index, gpu::TextureHandle texture) -> void;
    auto update_buffer(std::uint32_t index, gpu::BufferHandle buffer) -> void;

    [[nodiscard]] auto gpu_handle() const -> gpu::DescriptorHeapHandle;
    [[nodiscard]] auto allocated_count() const -> std::uint32_t;
    [[nodiscard]] auto capacity() const -> std::uint32_t;
  };

} // namespace harmonius::asset
