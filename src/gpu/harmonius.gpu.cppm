export module harmonius.gpu;

import std;

export namespace harmonius::gpu
{

  // ---------------------------------------------------------------------------
  // Handle types
  // ---------------------------------------------------------------------------

  enum class TextureHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class BufferHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class HeapHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class AccelerationStructureHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class FenceHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class PipelineHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class WorkGraphHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class DescriptorHeapHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class QueryPoolHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class SwapchainHandle : std::uint64_t
  {
    invalid = 0
  };
  enum class PipelineCacheHandle : std::uint64_t
  {
    invalid = 0
  };

  // ---------------------------------------------------------------------------
  // Format
  // ---------------------------------------------------------------------------

  enum class Format : std::uint32_t
  {
    undefined = 0,

    // 8-bit
    r8_unorm,
    r8_snorm,
    r8_uint,
    r8_sint,

    // 16-bit
    r16_float,
    r16_uint,
    r16_sint,
    r16_unorm,
    rg8_unorm,
    rg8_snorm,
    rg8_uint,
    rg8_sint,

    // 32-bit
    r32_float,
    r32_uint,
    r32_sint,
    rg16_float,
    rg16_uint,
    rg16_sint,
    rgba8_unorm,
    rgba8_snorm,
    rgba8_uint,
    rgba8_sint,
    rgba8_srgb,
    bgra8_unorm,
    bgra8_srgb,
    rgb10a2_unorm,
    rgb10a2_uint,
    rg11b10_float,

    // 64-bit
    rg32_float,
    rg32_uint,
    rg32_sint,
    rgba16_float,
    rgba16_uint,
    rgba16_sint,
    rgba16_unorm,

    // 128-bit
    rgba32_float,
    rgba32_uint,
    rgba32_sint,

    // Depth/stencil
    d16_unorm,
    d32_float,
    d24_unorm_s8_uint,
    d32_float_s8_uint,

    // Compressed
    bc1_unorm,
    bc1_srgb,
    bc2_unorm,
    bc2_srgb,
    bc3_unorm,
    bc3_srgb,
    bc4_unorm,
    bc4_snorm,
    bc5_unorm,
    bc5_snorm,
    bc6h_ufloat,
    bc6h_sfloat,
    bc7_unorm,
    bc7_srgb,
  };

  // ---------------------------------------------------------------------------
  // Core enums
  // ---------------------------------------------------------------------------

  enum class HeapType : std::uint8_t
  {
    device_local,
    upload,
    readback,
  };

  enum class QueueType : std::uint8_t
  {
    graphics,
    async_compute,
    transfer,
  };

  enum class TextureDimension : std::uint8_t
  {
    tex_2d,
    tex_2d_array,
    tex_3d,
    tex_cube,
    tex_cube_array,
  };

  enum class SampleCount : std::uint8_t
  {
    x1 = 1,
    x2 = 2,
    x4 = 4,
  };

  enum class LoadOp : std::uint8_t
  {
    load,
    clear,
    dont_care,
  };

  enum class StoreOp : std::uint8_t
  {
    store,
    dont_care,
  };

  enum class TextureLayout : std::uint8_t
  {
    undefined,
    general,
    color_attachment,
    depth_stencil_attachment,
    depth_stencil_read_only,
    shader_read_only,
    transfer_src,
    transfer_dst,
    present,
    shading_rate,
  };

  enum class QueryType : std::uint8_t
  {
    timestamp,
    pipeline_statistics,
  };

  enum class AccelerationStructureType : std::uint8_t
  {
    bottom_level,
    top_level,
  };

  enum class GeometryType : std::uint8_t
  {
    triangles,
    aabbs,
  };

  // ---------------------------------------------------------------------------
  // Bitmask enums — texture usage
  // ---------------------------------------------------------------------------

  enum class TextureUsageFlagBits : std::uint32_t
  {
    color_attachment = 1 << 0,
    depth_stencil_attachment = 1 << 1,
    shader_read = 1 << 2,
    storage_read_write = 1 << 3,
    transfer_src = 1 << 4,
    transfer_dst = 1 << 5,
    shading_rate_attachment = 1 << 6,
  };

  using TextureUsageFlags = std::uint32_t;

  constexpr auto operator|(TextureUsageFlagBits a, TextureUsageFlagBits b) -> TextureUsageFlags
  {
    return static_cast<TextureUsageFlags>(a) | static_cast<TextureUsageFlags>(b);
  }

  // ---------------------------------------------------------------------------
  // Bitmask enums — buffer usage
  // ---------------------------------------------------------------------------

  enum class BufferUsageFlagBits : std::uint32_t
  {
    constant_buffer = 1 << 0,
    storage_buffer = 1 << 1,
    index_buffer = 1 << 2,
    indirect_argument = 1 << 3,
    transfer_src = 1 << 4,
    transfer_dst = 1 << 5,
    acceleration_structure = 1 << 6,
    shader_binding_table = 1 << 7,
  };

  using BufferUsageFlags = std::uint32_t;

  constexpr auto operator|(BufferUsageFlagBits a, BufferUsageFlagBits b) -> BufferUsageFlags
  {
    return static_cast<BufferUsageFlags>(a) | static_cast<BufferUsageFlags>(b);
  }

  // ---------------------------------------------------------------------------
  // Bitmask enums — pipeline stage
  // ---------------------------------------------------------------------------

  enum class PipelineStage : std::uint64_t
  {
    none = 0,
    mesh_shader = 1ULL << 0,
    task_shader = 1ULL << 1,
    fragment_shader = 1ULL << 2,
    compute_shader = 1ULL << 3,
    ray_tracing_shader = 1ULL << 4,
    all_shading = 1ULL << 5,
    color_output = 1ULL << 6,
    depth_stencil = 1ULL << 7,
    transfer = 1ULL << 8,
    resolve = 1ULL << 9,
    acceleration_structure = 1ULL << 10,
    indirect_argument = 1ULL << 11,
    shading_rate = 1ULL << 12,
    all = 0xFFFFFFFFFFFFFFFF,
    split_begin = 1ULL << 62,
    split_end = 1ULL << 63,
  };

  constexpr auto operator|(PipelineStage a, PipelineStage b) -> PipelineStage
  {
    return static_cast<PipelineStage>(
        static_cast<std::uint64_t>(a) | static_cast<std::uint64_t>(b));
  }
  constexpr auto operator&(PipelineStage a, PipelineStage b) -> PipelineStage
  {
    return static_cast<PipelineStage>(
        static_cast<std::uint64_t>(a) & static_cast<std::uint64_t>(b));
  }
  constexpr auto operator~(PipelineStage a) -> PipelineStage
  {
    return static_cast<PipelineStage>(~static_cast<std::uint64_t>(a));
  }

  // ---------------------------------------------------------------------------
  // Bitmask enums — resource access
  // ---------------------------------------------------------------------------

  enum class ResourceAccess : std::uint64_t
  {
    none = 0,
    shader_read = 1ULL << 0,
    shader_write = 1ULL << 1,
    color_attachment_read = 1ULL << 2,
    color_attachment_write = 1ULL << 3,
    depth_stencil_read = 1ULL << 4,
    depth_stencil_write = 1ULL << 5,
    transfer_read = 1ULL << 6,
    transfer_write = 1ULL << 7,
    indirect_read = 1ULL << 8,
    acceleration_structure_read = 1ULL << 9,
    acceleration_structure_write = 1ULL << 10,
    shading_rate_read = 1ULL << 11,
    resolve_read = 1ULL << 12,
    resolve_write = 1ULL << 13,
    present = 1ULL << 14,
  };

  constexpr auto operator|(ResourceAccess a, ResourceAccess b) -> ResourceAccess
  {
    return static_cast<ResourceAccess>(
        static_cast<std::uint64_t>(a) | static_cast<std::uint64_t>(b));
  }
  constexpr auto operator&(ResourceAccess a, ResourceAccess b) -> ResourceAccess
  {
    return static_cast<ResourceAccess>(
        static_cast<std::uint64_t>(a) & static_cast<std::uint64_t>(b));
  }
  constexpr auto operator~(ResourceAccess a) -> ResourceAccess
  {
    return static_cast<ResourceAccess>(~static_cast<std::uint64_t>(a));
  }

  // ---------------------------------------------------------------------------
  // Error types
  // ---------------------------------------------------------------------------

  enum class ResourceError : std::uint8_t
  {
    out_of_memory,
    invalid_format,
    invalid_dimensions,
    unsupported_usage,
  };

  enum class DeviceError : std::uint8_t
  {
    initialization_failed,
    feature_not_supported,
    out_of_memory,
  };

  enum class PipelineError : std::uint8_t
  {
    compilation_failed,
    unsupported,
    invalid_state,
  };

  // ---------------------------------------------------------------------------
  // Pipeline state helper enums
  // ---------------------------------------------------------------------------

  enum class BlendFactor : std::uint8_t
  {
    zero,
    one,
    src_color,
    one_minus_src_color,
    dst_color,
    one_minus_dst_color,
    src_alpha,
    one_minus_src_alpha,
    dst_alpha,
    one_minus_dst_alpha,
  };

  enum class BlendOp : std::uint8_t
  {
    add,
    subtract,
    reverse_subtract,
    min,
    max,
  };

  enum class ColorWriteMask : std::uint8_t
  {
    none = 0,
    red = 1 << 0,
    green = 1 << 1,
    blue = 1 << 2,
    alpha = 1 << 3,
    all = red | green | blue | alpha,
  };

  enum class CompareOp : std::uint8_t
  {
    never,
    less,
    equal,
    less_equal,
    greater,
    not_equal,
    greater_equal,
    always,
  };

  enum class StencilOp : std::uint8_t
  {
    keep,
    zero,
    replace,
    increment_clamp,
    decrement_clamp,
    invert,
    increment_wrap,
    decrement_wrap,
  };

  enum class PolygonMode : std::uint8_t
  {
    fill,
    line,
  };

  enum class CullMode : std::uint8_t
  {
    none,
    front,
    back,
  };

  enum class FrontFace : std::uint8_t
  {
    counter_clockwise,
    clockwise,
  };

  // ---------------------------------------------------------------------------
  // Descriptor types
  // ---------------------------------------------------------------------------

  enum class DescriptorType : std::uint8_t
  {
    srv_texture,
    srv_buffer,
    uav_texture,
    uav_buffer,
    cbv,
    sampler,
    acceleration_structure,
  };

  // ---------------------------------------------------------------------------
  // Geometry structs
  // ---------------------------------------------------------------------------

  struct Extent2D
  {
    std::uint32_t width = 0;
    std::uint32_t height = 0;
  };

  struct Extent3D
  {
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth = 1;
  };

  struct Viewport
  {
    float x = 0;
    float y = 0;
    float width = 0;
    float height = 0;
    float min_depth = 0;
    float max_depth = 1;
  };

  struct Scissor
  {
    std::int32_t x = 0;
    std::int32_t y = 0;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
  };

  struct TextureSubresource
  {
    std::uint32_t mip_level = 0;
    std::uint32_t array_layer = 0;
  };

  struct TextureSubresourceRange
  {
    std::uint32_t base_mip_level = 0;
    std::uint32_t mip_level_count = 1;
    std::uint32_t base_array_layer = 0;
    std::uint32_t array_layer_count = 1;
  };

  // ---------------------------------------------------------------------------
  // Shader bytecode
  // ---------------------------------------------------------------------------

  struct ShaderBytecode
  {
    const void *data = nullptr;
    std::uint64_t size_bytes = 0;
  };

  struct ShaderLibrary
  {
    const void *data = nullptr;
    std::uint64_t size_bytes = 0;
  };

  // ---------------------------------------------------------------------------
  // Device descriptor
  // ---------------------------------------------------------------------------

  struct DeviceDesc
  {
    bool enable_validation = false;
    bool enable_gpu_capture = false;
    std::uint32_t frame_count = 3;
  };

  // ---------------------------------------------------------------------------
  // Resource descriptors
  // ---------------------------------------------------------------------------

  struct TextureDesc
  {
    std::string_view name;
    TextureDimension dimension = TextureDimension::tex_2d;
    Format format = Format::rgba8_unorm;
    std::uint32_t width = 1;
    std::uint32_t height = 1;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    SampleCount samples = SampleCount::x1;
    TextureUsageFlags usage = {};
  };

  struct BufferDesc
  {
    std::string_view name;
    std::uint64_t size_bytes = 0;
    HeapType heap_type = HeapType::device_local;
    BufferUsageFlags usage = {};
  };

  struct HeapDesc
  {
    std::uint64_t size_bytes = 0;
    HeapType type = HeapType::device_local;
  };

  struct AllocationInfo
  {
    std::uint64_t size_bytes = 0;
    std::uint64_t alignment = 0;
  };

  struct SparseTextureDesc
  {
    TextureDesc base;
    std::uint32_t tile_width = 256;
    std::uint32_t tile_height = 256;
  };

  struct SparseTileBinding
  {
    std::uint32_t mip_level;
    std::uint32_t array_layer;
    std::uint32_t tile_x;
    std::uint32_t tile_y;
    HeapHandle heap;
    std::uint64_t heap_offset;
  };

  // ---------------------------------------------------------------------------
  // Acceleration structure types
  // ---------------------------------------------------------------------------

  struct AccelerationStructureDesc
  {
    AccelerationStructureType type;
    BufferHandle buffer;
    std::uint64_t offset = 0;
    std::uint64_t size_bytes = 0;
  };

  struct AccelerationStructureSizes
  {
    std::uint64_t structure_size_bytes = 0;
    std::uint64_t build_scratch_bytes = 0;
    std::uint64_t update_scratch_bytes = 0;
  };

  struct AccelerationStructureGeometry
  {
    GeometryType type;

    BufferHandle vertex_buffer;
    std::uint64_t vertex_offset = 0;
    std::uint32_t vertex_count = 0;
    std::uint32_t vertex_stride = 0;
    Format vertex_format = Format::r32_float;
    BufferHandle index_buffer;
    std::uint64_t index_offset = 0;
    std::uint32_t index_count = 0;
    Format index_format = Format::r32_uint;
    BufferHandle transform_buffer;
    std::uint64_t transform_offset = 0;

    BufferHandle aabb_buffer;
    std::uint64_t aabb_offset = 0;
    std::uint32_t aabb_count = 0;
    std::uint32_t aabb_stride = 0;

    bool opaque = true;
  };

  struct AccelerationStructureBuildDesc
  {
    AccelerationStructureHandle dst;
    AccelerationStructureHandle src;
    BufferHandle scratch;
    std::uint64_t scratch_offset = 0;
    std::span<const AccelerationStructureGeometry> geometries;
    bool update = false;
  };

  struct AccelerationStructureInstance
  {
    float transform[3][4];
    std::uint32_t instance_id = 0;
    std::uint32_t mask = 0xFF;
    std::uint32_t sbt_offset = 0;
    AccelerationStructureHandle blas;
  };

  // ---------------------------------------------------------------------------
  // Pipeline state structs
  // ---------------------------------------------------------------------------

  struct BlendAttachment
  {
    bool blend_enable = false;
    BlendFactor src_color = BlendFactor::one;
    BlendFactor dst_color = BlendFactor::zero;
    BlendOp color_op = BlendOp::add;
    BlendFactor src_alpha = BlendFactor::one;
    BlendFactor dst_alpha = BlendFactor::zero;
    BlendOp alpha_op = BlendOp::add;
    ColorWriteMask write_mask = ColorWriteMask::all;
  };

  struct BlendState
  {
    std::span<const BlendAttachment> attachments;
  };

  struct StencilOpState
  {
    StencilOp fail_op = StencilOp::keep;
    StencilOp pass_op = StencilOp::keep;
    StencilOp depth_fail_op = StencilOp::keep;
    CompareOp compare_op = CompareOp::always;
    std::uint8_t compare_mask = 0xFF;
    std::uint8_t write_mask = 0xFF;
    std::uint8_t reference = 0;
  };

  struct DepthStencilState
  {
    bool depth_test_enable = true;
    bool depth_write_enable = true;
    CompareOp depth_compare = CompareOp::greater_equal;
    bool stencil_test_enable = false;
    StencilOpState front_stencil;
    StencilOpState back_stencil;
  };

  struct RasterizerState
  {
    CullMode cull_mode = CullMode::back;
    FrontFace front_face = FrontFace::counter_clockwise;
    PolygonMode polygon_mode = PolygonMode::fill;
    bool depth_clamp = false;
    float depth_bias = 0;
    float depth_bias_clamp = 0;
    float depth_bias_slope = 0;
    bool conservative_raster = false;
  };

  // ---------------------------------------------------------------------------
  // Render pass structs
  // ---------------------------------------------------------------------------

  struct ColorAttachment
  {
    TextureHandle texture;
    std::uint32_t mip_level = 0;
    std::uint32_t array_layer = 0;
    LoadOp load_op = LoadOp::clear;
    StoreOp store_op = StoreOp::store;
    float clear_color[4] = {0, 0, 0, 0};
    TextureHandle resolve_texture;
  };

  struct DepthStencilAttachment
  {
    TextureHandle texture;
    std::uint32_t mip_level = 0;
    std::uint32_t array_layer = 0;
    LoadOp depth_load_op = LoadOp::clear;
    StoreOp depth_store_op = StoreOp::store;
    float clear_depth = 0.0f;
    LoadOp stencil_load_op = LoadOp::dont_care;
    StoreOp stencil_store_op = StoreOp::dont_care;
    std::uint8_t clear_stencil = 0;
  };

  struct ShadingRateAttachment
  {
    TextureHandle texture;
    std::uint32_t tile_width = 16;
    std::uint32_t tile_height = 16;
  };

  struct RenderPassDesc
  {
    std::span<const ColorAttachment> color_attachments;
    std::optional<DepthStencilAttachment> depth_stencil;
    std::optional<ShadingRateAttachment> shading_rate;
    Extent2D render_area;
  };

  // ---------------------------------------------------------------------------
  // Barrier structs
  // ---------------------------------------------------------------------------

  struct TextureBarrier
  {
    TextureHandle texture;
    PipelineStage src_stage;
    ResourceAccess src_access;
    TextureLayout old_layout;
    PipelineStage dst_stage;
    ResourceAccess dst_access;
    TextureLayout new_layout;
    TextureSubresourceRange subresource_range;
    QueueType src_queue = QueueType::graphics;
    QueueType dst_queue = QueueType::graphics;
    bool discard = false;
  };

  struct BufferBarrier
  {
    BufferHandle buffer;
    PipelineStage src_stage;
    ResourceAccess src_access;
    PipelineStage dst_stage;
    ResourceAccess dst_access;
    std::uint64_t offset = 0;
    std::uint64_t size = ~0ULL;
    QueueType src_queue = QueueType::graphics;
    QueueType dst_queue = QueueType::graphics;
  };

  struct GlobalBarrier
  {
    PipelineStage src_stage;
    ResourceAccess src_access;
    PipelineStage dst_stage;
    ResourceAccess dst_access;
  };

  struct BarrierDesc
  {
    std::span<const GlobalBarrier> global_barriers;
    std::span<const TextureBarrier> texture_barriers;
    std::span<const BufferBarrier> buffer_barriers;
  };

  // ---------------------------------------------------------------------------
  // Fence synchronization
  // ---------------------------------------------------------------------------

  struct FenceSignal
  {
    FenceHandle fence;
    std::uint64_t value;
  };

  struct FenceWait
  {
    FenceHandle fence;
    std::uint64_t value;
  };

  // ---------------------------------------------------------------------------
  // Pipeline descriptors
  // ---------------------------------------------------------------------------

  struct MeshRenderPipelineDesc
  {
    ShaderBytecode task_shader;
    ShaderBytecode mesh_shader;
    ShaderBytecode pixel_shader;
    std::span<const Format> color_formats;
    Format depth_stencil_format = Format::undefined;
    SampleCount samples = SampleCount::x1;
    BlendState blend;
    RasterizerState rasterizer;
    DepthStencilState depth_stencil;
  };

  struct LinkedMeshRenderPipelineDesc
  {
    ShaderBytecode task_shader;
    ShaderBytecode mesh_shader;
    std::span<const ShaderLibrary> pixel_libraries;
    std::string_view entry_point;
    std::span<const Format> color_formats;
    Format depth_stencil_format = Format::undefined;
    SampleCount samples = SampleCount::x1;
    BlendState blend;
    RasterizerState rasterizer;
    DepthStencilState depth_stencil;
  };

  struct ComputePipelineDesc
  {
    ShaderBytecode compute_shader;
  };

  struct HitGroup
  {
    ShaderBytecode closest_hit;
    ShaderBytecode any_hit;
    ShaderBytecode intersection;
  };

  struct RayTracingPipelineDesc
  {
    ShaderBytecode ray_generation;
    std::span<const ShaderBytecode> miss_shaders;
    std::span<const HitGroup> hit_groups;
    std::uint32_t max_recursion_depth = 1;
    std::uint32_t max_payload_size = 32;
    std::uint32_t max_attribute_size = 8;
  };

  struct WorkGraphDesc
  {
    ShaderBytecode state_object;
    std::string_view program_name;
  };

  struct WorkGraphMemoryRequirements
  {
    std::uint64_t min_backing_memory_bytes = 0;
    std::uint64_t max_backing_memory_bytes = 0;
    std::uint64_t max_input_records_bytes = 0;
  };

  // ---------------------------------------------------------------------------
  // Dispatch descriptors
  // ---------------------------------------------------------------------------

  struct TraceRaysDesc
  {
    BufferHandle raygen_sbt;
    std::uint64_t raygen_offset;
    std::uint64_t raygen_size;
    BufferHandle miss_sbt;
    std::uint64_t miss_offset;
    std::uint64_t miss_stride;
    std::uint64_t miss_size;
    BufferHandle hit_sbt;
    std::uint64_t hit_offset;
    std::uint64_t hit_stride;
    std::uint64_t hit_size;
    std::uint32_t width;
    std::uint32_t height;
    std::uint32_t depth = 1;
  };

  struct DispatchGraphDesc
  {
    std::uint32_t entry_point_index = 0;
    const void *cpu_input = nullptr;
    std::uint32_t cpu_input_size = 0;
    BufferHandle gpu_input;
    std::uint64_t gpu_input_offset = 0;
    std::uint32_t gpu_input_size = 0;
    BufferHandle backing_memory;
    std::uint64_t backing_memory_offset = 0;
    std::uint64_t backing_memory_size = 0;
  };

  // ---------------------------------------------------------------------------
  // Indirect command argument structs
  // ---------------------------------------------------------------------------

  struct DispatchMeshIndirectArgs
  {
    std::uint32_t group_count_x;
    std::uint32_t group_count_y;
    std::uint32_t group_count_z;
  };

  struct DispatchIndirectArgs
  {
    std::uint32_t group_count_x;
    std::uint32_t group_count_y;
    std::uint32_t group_count_z;
  };

  struct TraceRaysIndirectArgs
  {
    std::uint32_t width;
    std::uint32_t height;
    std::uint32_t depth;
  };

  // ---------------------------------------------------------------------------
  // Descriptor and query structs
  // ---------------------------------------------------------------------------

  struct DescriptorHeapDesc
  {
    std::uint32_t max_descriptors = 1'000'000;
    std::uint32_t max_samplers = 2048;
  };

  struct SamplerDesc
  {
    // Sampler configuration — populated by consuming code
  };

  struct DescriptorWrite
  {
    DescriptorType type;
    std::uint32_t index;

    TextureHandle texture;
    Format format = Format::undefined;
    std::uint32_t mip_level = 0;
    std::uint32_t mip_count = ~0u;
    std::uint32_t array_layer = 0;
    std::uint32_t layer_count = ~0u;

    BufferHandle buffer;
    std::uint64_t offset = 0;
    std::uint64_t size = ~0ULL;
    std::uint32_t structure_stride = 0;

    AccelerationStructureHandle acceleration_structure;

    SamplerDesc sampler;
  };

  struct QueryPoolDesc
  {
    QueryType type = QueryType::timestamp;
    std::uint32_t count = 256;
  };

  // ---------------------------------------------------------------------------
  // Swapchain and pipeline cache
  // ---------------------------------------------------------------------------

  struct SwapchainDesc
  {
    void *native_window = nullptr;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    Format format = Format::bgra8_unorm;
    std::uint32_t image_count = 3;
    bool vsync = true;
  };

  struct PipelineCacheDesc
  {
    const void *initial_data = nullptr;
    std::uint64_t initial_data_size = 0;
  };

  // ---------------------------------------------------------------------------
  // Device capabilities
  // ---------------------------------------------------------------------------

  struct DeviceCapabilities
  {
    // Required capabilities (initialization fails without these)
    bool mesh_shaders = false;
    bool bindless_resources = false;
    bool timeline_fences = false;
    bool async_compute_queue = false;
    bool transfer_queue = false;

    // Soft-gated capabilities
    bool ray_tracing = false;
    bool ray_tracing_inline = false;
    bool opacity_micromaps = false;
    bool sparse_textures = false;
    bool int64_atomics = false;
    bool variable_rate_shading = false;
    bool work_graphs = false;
    bool split_barriers = false;
    bool shader_function_linking = false;

    // Limits
    std::uint32_t max_texture_dimension_2d = 0;
    std::uint32_t max_texture_dimension_3d = 0;
    std::uint32_t max_texture_array_layers = 0;
    std::uint32_t max_buffer_size_bytes = 0;
    std::uint32_t max_descriptor_count = 0;
    std::uint32_t max_push_constants_bytes = 0;

    // Mesh shader limits
    std::uint32_t max_mesh_output_vertices = 0;
    std::uint32_t max_mesh_output_primitives = 0;
    std::uint32_t max_mesh_workgroup_size = 0;
    std::uint32_t max_mesh_shared_memory_bytes = 0;
    std::uint32_t max_task_workgroup_size = 0;
    std::uint32_t max_task_shared_memory_bytes = 0;
    std::uint32_t max_task_payload_bytes = 0;

    // Memory
    std::uint64_t device_local_memory_bytes = 0;
    std::uint64_t shared_memory_bytes = 0;
  };

} // namespace harmonius::gpu
