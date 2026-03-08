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
  // ResourceHandle variant
  // ---------------------------------------------------------------------------

  struct ResourceHandle
  {
    enum class Kind : std::uint8_t
    {
      texture,
      buffer
    };

    Kind kind;
    std::uint64_t value;
  };

  // ---------------------------------------------------------------------------
  // Format
  // ---------------------------------------------------------------------------

  enum class Format : std::uint32_t
  {
    undefined,
    r8_unorm,
    r8_snorm,
    r8_uint,
    r8_sint,
    r16_float,
    r16_uint,
    r16_sint,
    r16_unorm,
    r32_float,
    r32_uint,
    r32_sint,
    rg8_unorm,
    rg8_snorm,
    rg16_float,
    rg16_uint,
    rg16_sint,
    rg32_float,
    rg32_uint,
    rg32_sint,
    rgba8_unorm,
    rgba8_snorm,
    rgba8_srgb,
    rgba8_uint,
    rgba8_sint,
    bgra8_unorm,
    bgra8_srgb,
    rgb10a2_unorm,
    rg11b10_float,
    rgba16_float,
    rgba16_uint,
    rgba16_sint,
    rgba16_unorm,
    rgba32_float,
    rgba32_uint,
    rgba32_sint,
    d16_unorm,
    d32_float,
    d24_unorm_s8_uint,
    d32_float_s8_uint,
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
  // Scalar enums
  // ---------------------------------------------------------------------------

  enum class SampleCount : std::uint8_t
  {
    x1 = 1,
    x2 = 2,
    x4 = 4,
    x8 = 8,
  };

  enum class QueueType : std::uint8_t
  {
    graphics,
    async_compute,
    transfer,
  };

  enum class TextureDimension : std::uint8_t
  {
    tex_1d,
    tex_2d,
    tex_3d,
    tex_cube,
  };

  enum class MemoryType : std::uint8_t
  {
    device_local,
    host_visible,
    host_cached,
  };

  enum class HeapType : std::uint8_t
  {
    default_heap,
    upload,
    readback,
  };

  enum class TextureLayout : std::uint8_t
  {
    undefined,
    general,
    color_attachment,
    depth_stencil_attachment,
    depth_stencil_read_only,
    shader_read,
    transfer_src,
    transfer_dst,
    present,
    shading_rate,
  };

  enum class Backend : std::uint8_t
  {
    metal,
    d3d12,
    vulkan,
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

  // ---------------------------------------------------------------------------
  // Bitmask enums
  // ---------------------------------------------------------------------------

  enum class TextureUsage : std::uint32_t
  {
    shader_read = 1,
    shader_write = 2,
    color_attachment = 4,
    depth_stencil_attachment = 8,
    transfer_src = 16,
    transfer_dst = 32,
    shading_rate = 64,
  };

  constexpr auto operator|(TextureUsage a, TextureUsage b) -> TextureUsage
  {
    return static_cast<TextureUsage>(
        static_cast<std::uint32_t>(a) | static_cast<std::uint32_t>(b));
  }
  constexpr auto operator&(TextureUsage a, TextureUsage b) -> TextureUsage
  {
    return static_cast<TextureUsage>(
        static_cast<std::uint32_t>(a) & static_cast<std::uint32_t>(b));
  }
  constexpr auto operator~(TextureUsage a) -> TextureUsage
  {
    return static_cast<TextureUsage>(~static_cast<std::uint32_t>(a));
  }
  constexpr auto operator|=(TextureUsage &a, TextureUsage b) -> TextureUsage &
  {
    a = a | b;
    return a;
  }
  constexpr auto operator&=(TextureUsage &a, TextureUsage b) -> TextureUsage &
  {
    a = a & b;
    return a;
  }

  enum class BufferUsage : std::uint32_t
  {
    vertex = 1,
    index = 2,
    constant = 4,
    storage = 8,
    indirect = 16,
    transfer_src = 32,
    transfer_dst = 64,
    acceleration_structure = 128,
  };

  constexpr auto operator|(BufferUsage a, BufferUsage b) -> BufferUsage
  {
    return static_cast<BufferUsage>(
        static_cast<std::uint32_t>(a) | static_cast<std::uint32_t>(b));
  }
  constexpr auto operator&(BufferUsage a, BufferUsage b) -> BufferUsage
  {
    return static_cast<BufferUsage>(
        static_cast<std::uint32_t>(a) & static_cast<std::uint32_t>(b));
  }
  constexpr auto operator~(BufferUsage a) -> BufferUsage
  {
    return static_cast<BufferUsage>(~static_cast<std::uint32_t>(a));
  }
  constexpr auto operator|=(BufferUsage &a, BufferUsage b) -> BufferUsage &
  {
    a = a | b;
    return a;
  }
  constexpr auto operator&=(BufferUsage &a, BufferUsage b) -> BufferUsage &
  {
    a = a & b;
    return a;
  }

  enum class PipelineStage : std::uint32_t
  {
    top = 1,
    task_shader = 2,
    mesh_shader = 4,
    pixel_shader = 8,
    compute = 16,
    transfer = 32,
    acceleration_structure = 64,
    all_graphics = 128,
    all_commands = 256,
    bottom = 512,
    host = 1024,
    ray_tracing = 2048,
  };

  constexpr auto operator|(PipelineStage a, PipelineStage b) -> PipelineStage
  {
    return static_cast<PipelineStage>(
        static_cast<std::uint32_t>(a) | static_cast<std::uint32_t>(b));
  }
  constexpr auto operator&(PipelineStage a, PipelineStage b) -> PipelineStage
  {
    return static_cast<PipelineStage>(
        static_cast<std::uint32_t>(a) & static_cast<std::uint32_t>(b));
  }
  constexpr auto operator~(PipelineStage a) -> PipelineStage
  {
    return static_cast<PipelineStage>(~static_cast<std::uint32_t>(a));
  }
  constexpr auto operator|=(PipelineStage &a, PipelineStage b) -> PipelineStage &
  {
    a = a | b;
    return a;
  }
  constexpr auto operator&=(PipelineStage &a, PipelineStage b) -> PipelineStage &
  {
    a = a & b;
    return a;
  }

  enum class ResourceAccess : std::uint32_t
  {
    none = 0,
    shader_read = 1,
    shader_write = 2,
    color_attachment_read = 4,
    color_attachment_write = 8,
    depth_stencil_read = 16,
    depth_stencil_write = 32,
    transfer_read = 64,
    transfer_write = 128,
    acceleration_structure_read = 256,
    acceleration_structure_write = 512,
    indirect_command_read = 1024,
    present = 2048,
  };

  constexpr auto operator|(ResourceAccess a, ResourceAccess b) -> ResourceAccess
  {
    return static_cast<ResourceAccess>(
        static_cast<std::uint32_t>(a) | static_cast<std::uint32_t>(b));
  }
  constexpr auto operator&(ResourceAccess a, ResourceAccess b) -> ResourceAccess
  {
    return static_cast<ResourceAccess>(
        static_cast<std::uint32_t>(a) & static_cast<std::uint32_t>(b));
  }
  constexpr auto operator~(ResourceAccess a) -> ResourceAccess
  {
    return static_cast<ResourceAccess>(~static_cast<std::uint32_t>(a));
  }
  constexpr auto operator|=(ResourceAccess &a, ResourceAccess b) -> ResourceAccess &
  {
    a = a | b;
    return a;
  }
  constexpr auto operator&=(ResourceAccess &a, ResourceAccess b) -> ResourceAccess &
  {
    a = a & b;
    return a;
  }

  // ---------------------------------------------------------------------------
  // Error types
  // ---------------------------------------------------------------------------

  struct ResourceError
  {
    std::string message;
  };

  struct DeviceError
  {
    std::string message;
  };

  struct PipelineError
  {
    std::string message;
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

  enum class FillMode : std::uint8_t
  {
    solid,
    wireframe,
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
  // Basic geometry and subresource structs
  // ---------------------------------------------------------------------------

  struct Extent3D
  {
    std::uint32_t width = 1;
    std::uint32_t height = 1;
    std::uint32_t depth = 1;
  };

  struct TextureSubresource
  {
    std::uint32_t mip_level = 0;
    std::uint32_t array_layer = 0;
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

  struct ShaderBytecode
  {
    std::span<const std::uint8_t> data;
  };

  // ---------------------------------------------------------------------------
  // Resource descriptor structs
  // ---------------------------------------------------------------------------

  struct TextureDesc
  {
    Format format = Format::rgba8_unorm;
    TextureDimension dimension = TextureDimension::tex_2d;
    std::uint32_t width = 1;
    std::uint32_t height = 1;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    SampleCount samples = SampleCount::x1;
    TextureUsage usage = TextureUsage::shader_read;
    MemoryType memory = MemoryType::device_local;
  };

  struct BufferDesc
  {
    std::uint64_t size = 0;
    BufferUsage usage = BufferUsage::storage;
    MemoryType memory = MemoryType::device_local;
  };

  struct HeapDesc
  {
    std::uint64_t size = 0;
    HeapType type = HeapType::default_heap;
  };

  struct SparseTextureDesc
  {
    TextureDesc base;
    std::uint32_t tile_width = 256;
    std::uint32_t tile_height = 256;
  };

  struct SparseTileBinding
  {
    std::uint32_t x;
    std::uint32_t y;
    std::uint32_t z;
    std::uint32_t mip_level;
    HeapHandle heap;
    std::uint64_t heap_offset;
  };

  struct AccelerationStructureDesc
  {
    AccelerationStructureType type;
    std::uint64_t max_size_bytes;
  };

  struct AccelerationStructureSizes
  {
    std::uint64_t result_size;
    std::uint64_t build_scratch_size;
    std::uint64_t update_scratch_size;
  };

  struct AccelerationStructureBuildDesc
  {
    AccelerationStructureHandle dst;
    BufferHandle scratch;
    std::uint64_t scratch_offset;
    bool update;
  };

  // ---------------------------------------------------------------------------
  // Pipeline state structs
  // ---------------------------------------------------------------------------

  struct BlendState
  {
    bool enabled = false;
    BlendFactor src_color = BlendFactor::one;
    BlendFactor dst_color = BlendFactor::zero;
    BlendOp color_op = BlendOp::add;
    BlendFactor src_alpha = BlendFactor::one;
    BlendFactor dst_alpha = BlendFactor::zero;
    BlendOp alpha_op = BlendOp::add;
  };

  struct DepthStencilState
  {
    bool depth_test_enabled = false;
    bool depth_write_enabled = true;
    CompareOp depth_compare = CompareOp::less;
    bool stencil_test_enabled = false;
  };

  struct RasterizerState
  {
    FillMode fill = FillMode::solid;
    CullMode cull = CullMode::back;
    FrontFace front_face = FrontFace::counter_clockwise;
    float depth_bias = 0.0f;
    float depth_bias_clamp = 0.0f;
    float depth_bias_slope = 0.0f;
  };

  // ---------------------------------------------------------------------------
  // Render pass structs
  // ---------------------------------------------------------------------------

  struct ColorAttachment
  {
    TextureHandle texture;
    TextureHandle resolve_texture{TextureHandle::invalid};
    Format format;
    std::array<float, 4> clear_color{};
    bool clear = false;
    bool store = true;
  };

  struct DepthStencilAttachment
  {
    TextureHandle texture;
    Format format;
    float clear_depth = 1.0f;
    std::uint8_t clear_stencil = 0;
    bool clear = false;
    bool store = true;
    bool read_only = false;
  };

  struct RenderPassDesc
  {
    std::vector<ColorAttachment> color_attachments;
    std::optional<DepthStencilAttachment> depth_stencil;
    Viewport viewport;
    Scissor scissor;
  };

  // ---------------------------------------------------------------------------
  // Barrier structs
  // ---------------------------------------------------------------------------

  struct TextureBarrier
  {
    TextureHandle texture;
    PipelineStage src_stage;
    PipelineStage dst_stage;
    ResourceAccess src_access;
    ResourceAccess dst_access;
    TextureLayout old_layout;
    TextureLayout new_layout;
    std::uint32_t src_queue = 0;
    std::uint32_t dst_queue = 0;
    std::uint32_t base_mip = 0;
    std::uint32_t mip_count = 1;
    std::uint32_t base_layer = 0;
    std::uint32_t layer_count = 1;
  };

  struct BufferBarrier
  {
    BufferHandle buffer;
    PipelineStage src_stage;
    PipelineStage dst_stage;
    ResourceAccess src_access;
    ResourceAccess dst_access;
    std::uint64_t offset = 0;
    std::uint64_t size = std::numeric_limits<std::uint64_t>::max();
    std::uint32_t src_queue = 0;
    std::uint32_t dst_queue = 0;
  };

  struct GlobalBarrier
  {
    PipelineStage src_stage;
    PipelineStage dst_stage;
    ResourceAccess src_access;
    ResourceAccess dst_access;
  };

  struct BarrierDesc
  {
    std::vector<TextureBarrier> texture_barriers;
    std::vector<BufferBarrier> buffer_barriers;
    std::vector<GlobalBarrier> global_barriers;
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
  // Pipeline descriptor structs
  // ---------------------------------------------------------------------------

  struct MeshRenderPipelineDesc
  {
    ShaderBytecode task_shader;
    ShaderBytecode mesh_shader;
    ShaderBytecode pixel_shader;
    std::vector<Format> color_formats;
    Format depth_format = Format::undefined;
    SampleCount samples = SampleCount::x1;
    BlendState blend;
    DepthStencilState depth_stencil;
    RasterizerState rasterizer;
    PipelineCacheHandle cache{PipelineCacheHandle::invalid};
  };

  struct ComputePipelineDesc
  {
    ShaderBytecode compute_shader;
    PipelineCacheHandle cache{PipelineCacheHandle::invalid};
  };

  struct RayTracingShaderGroup
  {
    enum class Type : std::uint8_t
    {
      general,
      triangles_hit_group,
      procedural_hit_group,
    };

    Type type = Type::general;
    ShaderBytecode general_shader;
    ShaderBytecode closest_hit_shader;
    ShaderBytecode any_hit_shader;
    ShaderBytecode intersection_shader;
  };

  struct RayTracingPipelineDesc
  {
    std::vector<RayTracingShaderGroup> shader_groups;
    std::uint32_t max_recursion_depth = 1;
    PipelineCacheHandle cache{PipelineCacheHandle::invalid};
  };

  struct WorkGraphDesc
  {
    ShaderBytecode program;
    std::uint64_t backing_memory_size = 0;
    PipelineCacheHandle cache{PipelineCacheHandle::invalid};
  };

  struct TraceRaysDesc
  {
    ShaderBytecode ray_gen;
    ShaderBytecode miss;
    ShaderBytecode hit_group;
    ShaderBytecode callable;
    std::uint32_t width = 1;
    std::uint32_t height = 1;
    std::uint32_t depth = 1;
  };

  struct DispatchGraphDesc
  {
    WorkGraphHandle program;
    std::uint32_t count = 1;
    BufferHandle backing_memory;
  };

  // ---------------------------------------------------------------------------
  // Descriptor and query structs
  // ---------------------------------------------------------------------------

  struct DescriptorHeapDesc
  {
    std::uint32_t max_descriptors = 1000000;
  };

  struct DescriptorWrite
  {
    enum class Type : std::uint8_t
    {
      srv_texture,
      srv_buffer,
      uav_texture,
      uav_buffer,
      sampler,
    };

    Type type;
    std::uint64_t handle;
  };

  struct QueryPoolDesc
  {
    QueryType type;
    std::uint32_t count;
  };

  // ---------------------------------------------------------------------------
  // Swapchain and pipeline cache
  // ---------------------------------------------------------------------------

  struct SwapchainDesc
  {
    void *native_window = nullptr;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t buffer_count = 3;
    Format format = Format::bgra8_unorm;
    bool vsync = true;
  };

  struct PipelineCacheDesc
  {
    std::span<const std::uint8_t> initial_data;
  };

  // ---------------------------------------------------------------------------
  // Device capabilities
  // ---------------------------------------------------------------------------

  struct DeviceCapabilities
  {
    bool mesh_shaders = false;
    bool ray_tracing = false;
    bool ray_tracing_pipeline = false;
    bool work_graphs = false;
    bool sparse_resources = false;
    bool descriptor_indexing = false;
    bool enhanced_barriers = false;
    bool variable_rate_shading = false;
    bool sampler_feedback = false;
    bool int64_atomics = false;

    std::uint32_t max_mesh_output_vertices = 0;
    std::uint32_t max_mesh_output_primitives = 0;
    std::uint32_t max_task_workgroup_size = 0;
    std::uint32_t max_mesh_workgroup_size = 0;
    std::uint32_t max_threadgroup_memory_bytes = 0;
    std::uint32_t max_texture_dimension_2d = 0;
    std::uint32_t max_buffer_size_bytes = 0;

    Backend backend = Backend::metal;
  };

} // namespace harmonius::gpu
