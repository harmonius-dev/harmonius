export module harmonius.rg.builder;

import std;
import harmonius.gpu;
import harmonius.rg;

export namespace harmonius::rg::builder
{

  // ---------------------------------------------------------------------------
  // Geometry helpers
  // ---------------------------------------------------------------------------

  struct RenderArea
  {
    std::uint32_t x = 0;
    std::uint32_t y = 0;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    bool is_per_frame_binding = false;
  };

  struct ActiveExtentDesc
  {
    std::string_view name;
    float scale = 1.0f;
  };

  // ---------------------------------------------------------------------------
  // Pass descriptor
  // ---------------------------------------------------------------------------

  struct PassDescriptor
  {
    std::string_view name;
    PassType type = PassType::compute;
    QueueAffinity queue = QueueAffinity::graphics;
    std::vector<ResourceBinding> inputs;
    std::vector<ResourceBinding> outputs;
    bool conditional = false;
    std::optional<RenderArea> render_area;
    std::vector<std::string_view> tags;
    float estimated_cost_ms = 0.0f;
    std::uint32_t priority = 0;
    std::function<void(void *)> execute;
  };

  // ---------------------------------------------------------------------------
  // Resource descriptors
  // ---------------------------------------------------------------------------

  struct TransientResourceDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    gpu::TextureDimension dimension = gpu::TextureDimension::tex_2d;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    gpu::SampleCount samples = gpu::SampleCount::x1;
    gpu::TextureUsageFlags usage = {};
    std::optional<std::string_view> resolution_param;
  };

  struct PersistentResourceDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    gpu::TextureDimension dimension = gpu::TextureDimension::tex_2d;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    gpu::SampleCount samples = gpu::SampleCount::x1;
    gpu::TextureUsageFlags usage = {};
  };

  struct HistoryResourceDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    gpu::TextureDimension dimension = gpu::TextureDimension::tex_2d;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    gpu::SampleCount samples = gpu::SampleCount::x1;
    gpu::TextureUsageFlags usage = {};
  };

  struct MultiFrameHistoryDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    gpu::TextureDimension dimension = gpu::TextureDimension::tex_2d;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    gpu::SampleCount samples = gpu::SampleCount::x1;
    gpu::TextureUsageFlags usage = {};
    std::uint32_t history_length = 2;
  };

  struct ImportedResourceDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    gpu::TextureDimension dimension = gpu::TextureDimension::tex_2d;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    gpu::SampleCount samples = gpu::SampleCount::x1;
    gpu::TextureUsageFlags usage = {};
  };

  struct SparseResourceDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    gpu::TextureDimension dimension = gpu::TextureDimension::tex_2d;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    gpu::TextureUsageFlags usage = {};
    std::uint32_t tile_width = 256;
    std::uint32_t tile_height = 256;
  };

  struct PoolResourceDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    gpu::TextureDimension dimension = gpu::TextureDimension::tex_2d;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    gpu::SampleCount samples = gpu::SampleCount::x1;
    gpu::TextureUsageFlags usage = {};
    std::uint32_t pool_size = 1;
  };

  struct StagingBufferDesc
  {
    std::string_view name;
    std::uint64_t size_bytes = 0;
    gpu::HeapType heap_type = gpu::HeapType::upload;
    gpu::BufferUsageFlags usage = {};
  };

  struct AtlasResourceDesc
  {
    std::string_view name;
    gpu::Format format = gpu::Format::rgba8_unorm;
    std::uint32_t atlas_width = 0;
    std::uint32_t atlas_height = 0;
    std::uint32_t tile_width = 256;
    std::uint32_t tile_height = 256;
    gpu::TextureUsageFlags usage = {};
  };

  struct AccelStructDesc
  {
    std::string_view name;
    gpu::BufferUsageFlags usage = {};
    std::uint64_t max_size_bytes = 0;
    bool allow_update = false;
  };

  // ---------------------------------------------------------------------------
  // Sub-graph descriptors
  // ---------------------------------------------------------------------------

  struct SubGraphParamSlot
  {
    std::string_view name;
    ResourceCategory category;
  };

  struct SubGraphDescriptor
  {
    std::string_view name;
    std::vector<SubGraphParamSlot> params;
    std::function<void(void *)> build_fn;
  };

  struct SubGraphBindings
  {
    std::vector<std::pair<std::string_view, ResourceHandle>> bindings;
  };

  // ---------------------------------------------------------------------------
  // Declared graph (output of the builder)
  // ---------------------------------------------------------------------------

  class DeclaredGraph
  {
  public:
    [[nodiscard]] auto pass_count() const -> std::uint32_t;
    [[nodiscard]] auto resource_count() const -> std::uint32_t;

  private:
    struct Impl;
    std::unique_ptr<Impl> impl_;

    friend class GraphBuilder;
  };

  // ---------------------------------------------------------------------------
  // Graph builder
  // ---------------------------------------------------------------------------

  class GraphBuilder
  {
  public:
    explicit GraphBuilder(std::uint32_t frame_count = 3);

    // Pass management
    [[nodiscard]] auto add_pass(PassDescriptor desc) -> PassHandle;
    [[nodiscard]] auto begin_chain(std::string_view name) -> ChainHandle;
    auto add_chain_step(ChainHandle chain, PassHandle pass) -> void;
    auto end_chain(ChainHandle chain) -> void;

    // Variant dispatch
    [[nodiscard]] auto declare_variant_slot(std::string_view name) -> VariantSlotHandle;
    auto add_variant(VariantSlotHandle slot, std::string_view value, PassHandle pass) -> void;

    // Sub-graphs
    [[nodiscard]] auto declare_subgraph_template(SubGraphDescriptor desc) -> SubGraphHandle;
    auto instantiate_subgraph(SubGraphHandle tmpl, SubGraphBindings bindings) -> void;

    // Resource declarations
    [[nodiscard]] auto declare_transient(TransientResourceDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_persistent(PersistentResourceDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_history(HistoryResourceDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_multi_frame_history(MultiFrameHistoryDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_imported(ImportedResourceDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_sparse(SparseResourceDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_pool(PoolResourceDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_staging(StagingBufferDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_atlas(AtlasResourceDesc desc) -> ResourceHandle;
    [[nodiscard]] auto declare_accel_struct(AccelStructDesc desc) -> ResourceHandle;

    // Active extent for dynamic resolution
    auto set_active_extent(ActiveExtentDesc desc) -> void;

    // Gate attachment
    auto attach_gate(PassHandle pass, GateHandle gate) -> void;

    // Ordering
    auto add_ordering_edge(PassHandle before, PassHandle after) -> void;

    // Build
    [[nodiscard]] auto build() -> std::expected<DeclaredGraph, CompileError>;
  };

} // namespace harmonius::rg::builder
