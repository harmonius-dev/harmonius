export module harmonius.rg.builder;

import harmonius.gpu;
import harmonius.rg;

import std;

export namespace harmonius::rg::builder {

// ---------------------------------------------------------------------------
// Geometry helpers
// ---------------------------------------------------------------------------

/// Describes the render area for a rasterization pass.
/// @threadsafety Instances are not thread-safe.
struct RenderArea {
  std::uint32_t x = 0;
  std::uint32_t y = 0;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  bool is_per_frame_binding = false;
};

/// Describes an active extent for dynamic resolution scaling.
/// @threadsafety Instances are not thread-safe.
struct ActiveExtentDesc {
  std::string_view name;
  float scale = 1.0f;
};

// ---------------------------------------------------------------------------
// Pass descriptor
// ---------------------------------------------------------------------------

/// Describes a render pass including its resource bindings and execution callback.
/// @threadsafety Instances are not thread-safe.
struct PassDescriptor {
  std::string_view name;
  PassType type = PassType::kCompute;
  QueueAffinity queue = QueueAffinity::kGraphics;
  std::vector<ResourceBinding> inputs;
  std::vector<ResourceBinding> outputs;
  bool conditional = false;
  std::optional<RenderArea> render_area;
  std::vector<std::string_view> tags;
  float estimated_cost_ms = 0.0f;
  std::uint32_t priority = 0;
  std::move_only_function<void(void*)> execute;
};

// ---------------------------------------------------------------------------
// Resource descriptors
// ---------------------------------------------------------------------------

/// Describes a transient resource that lives only within a single frame.
/// @threadsafety Instances are not thread-safe.
struct TransientResourceDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  gpu::TextureDimension dimension = gpu::TextureDimension::kTex2d;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  gpu::SampleCount samples = gpu::SampleCount::kX1;
  gpu::TextureUsageFlags usage = {};
  std::optional<std::string_view> resolution_param;
};

/// Describes a persistent resource that survives across frames.
/// @threadsafety Instances are not thread-safe.
struct PersistentResourceDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  gpu::TextureDimension dimension = gpu::TextureDimension::kTex2d;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  gpu::SampleCount samples = gpu::SampleCount::kX1;
  gpu::TextureUsageFlags usage = {};
};

/// Describes a history resource providing access to the previous frame's data.
/// @threadsafety Instances are not thread-safe.
struct HistoryResourceDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  gpu::TextureDimension dimension = gpu::TextureDimension::kTex2d;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  gpu::SampleCount samples = gpu::SampleCount::kX1;
  gpu::TextureUsageFlags usage = {};
};

/// Describes a multi-frame history resource retaining data across multiple frames.
/// @threadsafety Instances are not thread-safe.
struct MultiFrameHistoryDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  gpu::TextureDimension dimension = gpu::TextureDimension::kTex2d;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  gpu::SampleCount samples = gpu::SampleCount::kX1;
  gpu::TextureUsageFlags usage = {};
  std::uint32_t history_length = 2;
};

/// Describes an externally created resource imported into the render graph.
/// @threadsafety Instances are not thread-safe.
struct ImportedResourceDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  gpu::TextureDimension dimension = gpu::TextureDimension::kTex2d;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  gpu::SampleCount samples = gpu::SampleCount::kX1;
  gpu::TextureUsageFlags usage = {};
};

/// Describes a sparse (partially resident) texture resource.
/// @threadsafety Instances are not thread-safe.
struct SparseResourceDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  gpu::TextureDimension dimension = gpu::TextureDimension::kTex2d;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  gpu::TextureUsageFlags usage = {};
  std::uint32_t tile_width = 256;
  std::uint32_t tile_height = 256;
};

/// Describes a pooled resource backed by a fixed-size resource pool.
/// @threadsafety Instances are not thread-safe.
struct PoolResourceDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  gpu::TextureDimension dimension = gpu::TextureDimension::kTex2d;
  std::uint32_t width = 0;
  std::uint32_t height = 0;
  std::uint32_t depth_or_layers = 1;
  std::uint32_t mip_levels = 1;
  gpu::SampleCount samples = gpu::SampleCount::kX1;
  gpu::TextureUsageFlags usage = {};
  std::uint32_t pool_size = 1;
};

/// Describes a staging buffer for CPU-GPU data transfers.
/// @threadsafety Instances are not thread-safe.
struct StagingBufferDesc {
  std::string_view name;
  std::uint64_t size_bytes = 0;
  gpu::HeapType heap_type = gpu::HeapType::kUpload;
  gpu::BufferUsageFlags usage = {};
};

/// Describes an atlas resource composed of tiled sub-regions.
/// @threadsafety Instances are not thread-safe.
struct AtlasResourceDesc {
  std::string_view name;
  gpu::Format format = gpu::Format::kRgba8Unorm;
  std::uint32_t atlas_width = 0;
  std::uint32_t atlas_height = 0;
  std::uint32_t tile_width = 256;
  std::uint32_t tile_height = 256;
  gpu::TextureUsageFlags usage = {};
};

/// Describes an acceleration structure resource for ray tracing.
/// @threadsafety Instances are not thread-safe.
struct AccelStructDesc {
  std::string_view name;
  gpu::BufferUsageFlags usage = {};
  std::uint64_t max_size_bytes = 0;
  bool allow_update = false;
};

// ---------------------------------------------------------------------------
// Sub-graph descriptors
// ---------------------------------------------------------------------------

/// Describes a parameter slot for a sub-graph template.
/// @threadsafety Instances are not thread-safe.
struct SubGraphParamSlot {
  std::string_view name;
  ResourceCategory category;
};

/// Describes a reusable sub-graph template with parameterized resource bindings.
/// @threadsafety Instances are not thread-safe.
struct SubGraphDescriptor {
  std::string_view name;
  std::vector<SubGraphParamSlot> params;
  std::move_only_function<void(void*)> build_fn;
};

/// Maps sub-graph parameter names to concrete resource handles.
/// @threadsafety Instances are not thread-safe.
struct SubGraphBindings {
  std::vector<std::pair<std::string_view, ResourceHandle>> bindings;
};

// ---------------------------------------------------------------------------
// Declared graph (output of the builder)
// ---------------------------------------------------------------------------

/// The immutable result of building a render graph, ready for compilation.
/// @threadsafety Instances are not thread-safe.
class DeclaredGraph {
 public:
  /// Returns the number of passes in the declared graph.
  [[nodiscard]] auto PassCount() const -> std::uint32_t;

  /// Returns the number of resources in the declared graph.
  [[nodiscard]] auto ResourceCount() const -> std::uint32_t;

 private:
  struct Impl;
  std::unique_ptr<Impl> impl_;

  friend class GraphBuilder;
};

// ---------------------------------------------------------------------------
// Graph builder
// ---------------------------------------------------------------------------

/// Fluent API for declaring render passes, resources, and their dependencies.
/// @threadsafety Instances are not thread-safe.
class GraphBuilder {
 public:
  /// Constructs a graph builder with the specified frame buffering count.
  explicit GraphBuilder(std::uint32_t frame_count = 3);

  // Pass management

  /// Adds a render pass described by the given descriptor and returns its handle.
  [[nodiscard]] auto AddPass(PassDescriptor desc) -> PassHandle;

  /// Begins a new pass chain and returns its handle.
  [[nodiscard]] auto BeginChain(std::string_view name) -> ChainHandle;

  /// Adds a pass as the next step in the given chain.
  auto AddChainStep(ChainHandle chain, PassHandle pass) -> void;

  /// Finalizes the given chain.
  auto EndChain(ChainHandle chain) -> void;

  // Variant dispatch

  /// Declares a variant slot for runtime dispatch and returns its handle.
  [[nodiscard]] auto DeclareVariantSlot(std::string_view name) -> VariantSlotHandle;

  /// Registers a pass as a candidate for the given variant slot and value.
  auto AddVariant(VariantSlotHandle slot, std::string_view value, PassHandle pass) -> void;

  // Sub-graphs

  /// Declares a reusable sub-graph template and returns its handle.
  [[nodiscard]] auto DeclareSubgraphTemplate(SubGraphDescriptor desc) -> SubGraphHandle;

  /// Instantiates a sub-graph template with concrete resource bindings.
  auto InstantiateSubgraph(SubGraphHandle tmpl, SubGraphBindings bindings) -> void;

  // Resource declarations

  /// Declares a transient resource and returns its handle.
  [[nodiscard]] auto DeclareTransient(TransientResourceDesc desc) -> ResourceHandle;

  /// Declares a persistent resource and returns its handle.
  [[nodiscard]] auto DeclarePersistent(PersistentResourceDesc desc) -> ResourceHandle;

  /// Declares a history resource and returns its handle.
  [[nodiscard]] auto DeclareHistory(HistoryResourceDesc desc) -> ResourceHandle;

  /// Declares a multi-frame history resource and returns its handle.
  [[nodiscard]] auto DeclareMultiFrameHistory(MultiFrameHistoryDesc desc) -> ResourceHandle;

  /// Declares an imported resource and returns its handle.
  [[nodiscard]] auto DeclareImported(ImportedResourceDesc desc) -> ResourceHandle;

  /// Declares a sparse resource and returns its handle.
  [[nodiscard]] auto DeclareSparse(SparseResourceDesc desc) -> ResourceHandle;

  /// Declares a pooled resource and returns its handle.
  [[nodiscard]] auto DeclarePool(PoolResourceDesc desc) -> ResourceHandle;

  /// Declares a staging buffer and returns its handle.
  [[nodiscard]] auto DeclareStaging(StagingBufferDesc desc) -> ResourceHandle;

  /// Declares an atlas resource and returns its handle.
  [[nodiscard]] auto DeclareAtlas(AtlasResourceDesc desc) -> ResourceHandle;

  /// Declares an acceleration structure and returns its handle.
  [[nodiscard]] auto DeclareAccelStruct(AccelStructDesc desc) -> ResourceHandle;

  // Active extent for dynamic resolution

  /// Sets the active extent descriptor for dynamic resolution scaling.
  auto SetActiveExtent(ActiveExtentDesc desc) -> void;

  // Gate attachment

  /// Attaches a gate to a pass for conditional execution.
  auto AttachGate(PassHandle pass, GateHandle gate) -> void;

  // Ordering

  /// Adds an explicit ordering dependency between two passes.
  auto AddOrderingEdge(PassHandle before, PassHandle after) -> void;

  // Build

  /// Builds the declared graph, returning it or compilation errors.
  [[nodiscard]] auto Build() -> std::expected<DeclaredGraph, CompileError>;
};

}  // namespace harmonius::rg::builder
