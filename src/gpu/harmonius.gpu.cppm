/// @file harmonius.gpu.cppm
/// @brief Core GPU abstraction types, enumerations, and descriptors.
export module harmonius.gpu;

import std;

export namespace harmonius::gpu {

// ---------------------------------------------------------------------------
// Handle types
// ---------------------------------------------------------------------------

/// An opaque handle to a GPU texture resource.
enum class TextureHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU buffer resource.
enum class BufferHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU memory heap.
enum class HeapHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU acceleration structure.
enum class AccelerationStructureHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU timeline fence.
enum class FenceHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU pipeline state object.
enum class PipelineHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU work graph program.
enum class WorkGraphHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU descriptor heap.
enum class DescriptorHeapHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU query pool.
enum class QueryPoolHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU swapchain.
enum class SwapchainHandle : std::uint64_t { kInvalid = 0 };

/// An opaque handle to a GPU pipeline cache.
enum class PipelineCacheHandle : std::uint64_t { kInvalid = 0 };

// ---------------------------------------------------------------------------
// Format
// ---------------------------------------------------------------------------

/// A GPU texture or vertex format specifying channel layout and encoding.
enum class Format : std::uint32_t {
    kUndefined = 0,

    // 8-bit
    kR8Unorm,
    kR8Snorm,
    kR8Uint,
    kR8Sint,

    // 16-bit
    kR16Float,
    kR16Uint,
    kR16Sint,
    kR16Unorm,
    kRg8Unorm,
    kRg8Snorm,
    kRg8Uint,
    kRg8Sint,

    // 32-bit
    kR32Float,
    kR32Uint,
    kR32Sint,
    kRg16Float,
    kRg16Uint,
    kRg16Sint,
    kRgba8Unorm,
    kRgba8Snorm,
    kRgba8Uint,
    kRgba8Sint,
    kRgba8Srgb,
    kBgra8Unorm,
    kBgra8Srgb,
    kRgb10a2Unorm,
    kRgb10a2Uint,
    kRg11b10Float,

    // 64-bit
    kRg32Float,
    kRg32Uint,
    kRg32Sint,
    kRgba16Float,
    kRgba16Uint,
    kRgba16Sint,
    kRgba16Unorm,

    // 128-bit
    kRgba32Float,
    kRgba32Uint,
    kRgba32Sint,

    // Depth/stencil
    kD16Unorm,
    kD32Float,
    kD24UnormS8Uint,
    kD32FloatS8Uint,

    // Compressed
    kBc1Unorm,
    kBc1Srgb,
    kBc2Unorm,
    kBc2Srgb,
    kBc3Unorm,
    kBc3Srgb,
    kBc4Unorm,
    kBc4Snorm,
    kBc5Unorm,
    kBc5Snorm,
    kBc6hUfloat,
    kBc6hSfloat,
    kBc7Unorm,
    kBc7Srgb,
};

// ---------------------------------------------------------------------------
// Core enums
// ---------------------------------------------------------------------------

/// The memory heap type for a GPU resource allocation.
enum class HeapType : std::uint8_t {
    kDeviceLocal,
    kUpload,
    kReadback,
};

/// The type of GPU command queue.
enum class QueueType : std::uint8_t {
    kGraphics,
    kAsyncCompute,
    kTransfer,
};

/// The dimensionality of a GPU texture.
enum class TextureDimension : std::uint8_t {
    kTex2d,
    kTex2dArray,
    kTex3d,
    kTexCube,
    kTexCubeArray,
};

/// The multisample count for a GPU texture.
enum class SampleCount : std::uint8_t {
    kX1 = 1,
    kX2 = 2,
    kX4 = 4,
};

/// The operation to perform when loading a render target attachment.
enum class LoadOp : std::uint8_t {
    kLoad,
    kClear,
    kDontCare,
};

/// The operation to perform when storing a render target attachment.
enum class StoreOp : std::uint8_t {
    kStore,
    kDontCare,
};

/// The memory layout of a GPU texture for synchronization purposes.
enum class TextureLayout : std::uint8_t {
    kUndefined,
    kGeneral,
    kColorAttachment,
    kDepthStencilAttachment,
    kDepthStencilReadOnly,
    kShaderReadOnly,
    kTransferSrc,
    kTransferDst,
    kPresent,
    kShadingRate,
};

/// The type of GPU query.
enum class QueryType : std::uint8_t {
    kTimestamp,
    kPipelineStatistics,
};

/// The type of ray tracing acceleration structure.
enum class AccelerationStructureType : std::uint8_t {
    kBottomLevel,
    kTopLevel,
};

/// The type of geometry in a bottom-level acceleration structure.
enum class GeometryType : std::uint8_t {
    kTriangles,
    kAabbs,
};

// ---------------------------------------------------------------------------
// Bitmask enums -- texture usage
// ---------------------------------------------------------------------------

/// Bit flags describing how a GPU texture may be used.
enum class TextureUsageFlagBits : std::uint32_t {
    kColorAttachment = 1 << 0,
    kDepthStencilAttachment = 1 << 1,
    kShaderRead = 1 << 2,
    kStorageReadWrite = 1 << 3,
    kTransferSrc = 1 << 4,
    kTransferDst = 1 << 5,
    kShadingRateAttachment = 1 << 6,
};

/// A bitmask of TextureUsageFlagBits values.
using TextureUsageFlags = std::uint32_t;

/// Combines two texture usage flag bits into a bitmask.
[[nodiscard]] constexpr auto operator|(TextureUsageFlagBits a, TextureUsageFlagBits b) -> TextureUsageFlags {
    return static_cast<TextureUsageFlags>(a) | static_cast<TextureUsageFlags>(b);
}

// ---------------------------------------------------------------------------
// Bitmask enums -- buffer usage
// ---------------------------------------------------------------------------

/// Bit flags describing how a GPU buffer may be used.
enum class BufferUsageFlagBits : std::uint32_t {
    kConstantBuffer = 1 << 0,
    kStorageBuffer = 1 << 1,
    kIndexBuffer = 1 << 2,
    kIndirectArgument = 1 << 3,
    kTransferSrc = 1 << 4,
    kTransferDst = 1 << 5,
    kAccelerationStructure = 1 << 6,
    kShaderBindingTable = 1 << 7,
};

/// A bitmask of BufferUsageFlagBits values.
using BufferUsageFlags = std::uint32_t;

/// Combines two buffer usage flag bits into a bitmask.
[[nodiscard]] constexpr auto operator|(BufferUsageFlagBits a, BufferUsageFlagBits b) -> BufferUsageFlags {
    return static_cast<BufferUsageFlags>(a) | static_cast<BufferUsageFlags>(b);
}

// ---------------------------------------------------------------------------
// Bitmask enums -- pipeline stage
// ---------------------------------------------------------------------------

/// Bit flags identifying GPU pipeline stages for synchronization.
enum class PipelineStage : std::uint64_t {
    kNone = 0,
    kMeshShader = 1ULL << 0,
    kTaskShader = 1ULL << 1,
    kFragmentShader = 1ULL << 2,
    kComputeShader = 1ULL << 3,
    kRayTracingShader = 1ULL << 4,
    kAllShading = 1ULL << 5,
    kColorOutput = 1ULL << 6,
    kDepthStencil = 1ULL << 7,
    kTransfer = 1ULL << 8,
    kResolve = 1ULL << 9,
    kAccelerationStructure = 1ULL << 10,
    kIndirectArgument = 1ULL << 11,
    kShadingRate = 1ULL << 12,
    kAll = 0xFFFFFFFFFFFFFFFF,
    kSplitBegin = 1ULL << 62,
    kSplitEnd = 1ULL << 63,
};

/// Combines two pipeline stage flags with bitwise OR.
[[nodiscard]] constexpr auto operator|(PipelineStage a, PipelineStage b) -> PipelineStage {
    return static_cast<PipelineStage>(static_cast<std::uint64_t>(a) | static_cast<std::uint64_t>(b));
}

/// Intersects two pipeline stage flags with bitwise AND.
[[nodiscard]] constexpr auto operator&(PipelineStage a, PipelineStage b) -> PipelineStage {
    return static_cast<PipelineStage>(static_cast<std::uint64_t>(a) & static_cast<std::uint64_t>(b));
}

/// Inverts a pipeline stage bitmask.
[[nodiscard]] constexpr auto operator~(PipelineStage a) -> PipelineStage {
    return static_cast<PipelineStage>(~static_cast<std::uint64_t>(a));
}

// ---------------------------------------------------------------------------
// Bitmask enums -- resource access
// ---------------------------------------------------------------------------

/// Bit flags identifying GPU resource access types for synchronization.
enum class ResourceAccess : std::uint64_t {
    kNone = 0,
    kShaderRead = 1ULL << 0,
    kShaderWrite = 1ULL << 1,
    kColorAttachmentRead = 1ULL << 2,
    kColorAttachmentWrite = 1ULL << 3,
    kDepthStencilRead = 1ULL << 4,
    kDepthStencilWrite = 1ULL << 5,
    kTransferRead = 1ULL << 6,
    kTransferWrite = 1ULL << 7,
    kIndirectRead = 1ULL << 8,
    kAccelerationStructureRead = 1ULL << 9,
    kAccelerationStructureWrite = 1ULL << 10,
    kShadingRateRead = 1ULL << 11,
    kResolveRead = 1ULL << 12,
    kResolveWrite = 1ULL << 13,
    kPresent = 1ULL << 14,
};

/// Combines two resource access flags with bitwise OR.
[[nodiscard]] constexpr auto operator|(ResourceAccess a, ResourceAccess b) -> ResourceAccess {
    return static_cast<ResourceAccess>(static_cast<std::uint64_t>(a) | static_cast<std::uint64_t>(b));
}

/// Intersects two resource access flags with bitwise AND.
[[nodiscard]] constexpr auto operator&(ResourceAccess a, ResourceAccess b) -> ResourceAccess {
    return static_cast<ResourceAccess>(static_cast<std::uint64_t>(a) & static_cast<std::uint64_t>(b));
}

/// Inverts a resource access bitmask.
[[nodiscard]] constexpr auto operator~(ResourceAccess a) -> ResourceAccess {
    return static_cast<ResourceAccess>(~static_cast<std::uint64_t>(a));
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error codes returned when a GPU resource creation fails.
enum class ResourceError : std::uint8_t {
    kOutOfMemory,
    kInvalidFormat,
    kInvalidDimensions,
    kUnsupportedUsage,
};

/// Error codes returned when a GPU device operation fails.
enum class DeviceError : std::uint8_t {
    kInitializationFailed,
    kFeatureNotSupported,
    kOutOfMemory,
};

/// Error codes returned when a GPU pipeline creation fails.
enum class PipelineError : std::uint8_t {
    kCompilationFailed,
    kUnsupported,
    kInvalidState,
};

// ---------------------------------------------------------------------------
// Pipeline state helper enums
// ---------------------------------------------------------------------------

/// A blend factor used in color blending equations.
enum class BlendFactor : std::uint8_t {
    kZero,
    kOne,
    kSrcColor,
    kOneMinusSrcColor,
    kDstColor,
    kOneMinusDstColor,
    kSrcAlpha,
    kOneMinusSrcAlpha,
    kDstAlpha,
    kOneMinusDstAlpha,
};

/// A blend operation applied to blended color or alpha values.
enum class BlendOp : std::uint8_t {
    kAdd,
    kSubtract,
    kReverseSubtract,
    kMin,
    kMax,
};

/// A bitmask controlling which color channels are written.
enum class ColorWriteMask : std::uint8_t {
    kNone = 0,
    kRed = 1 << 0,
    kGreen = 1 << 1,
    kBlue = 1 << 2,
    kAlpha = 1 << 3,
    kAll = kRed | kGreen | kBlue | kAlpha,
};

/// A comparison function used in depth and stencil tests.
enum class CompareOp : std::uint8_t {
    kNever,
    kLess,
    kEqual,
    kLessEqual,
    kGreater,
    kNotEqual,
    kGreaterEqual,
    kAlways,
};

/// An operation applied to the stencil buffer.
enum class StencilOp : std::uint8_t {
    kKeep,
    kZero,
    kReplace,
    kIncrementClamp,
    kDecrementClamp,
    kInvert,
    kIncrementWrap,
    kDecrementWrap,
};

/// The polygon rasterization mode.
enum class PolygonMode : std::uint8_t {
    kFill,
    kLine,
};

/// The triangle face culling mode.
enum class CullMode : std::uint8_t {
    kNone,
    kFront,
    kBack,
};

/// The winding order used to determine front-facing triangles.
enum class FrontFace : std::uint8_t {
    kCounterClockwise,
    kClockwise,
};

// ---------------------------------------------------------------------------
// Descriptor types
// ---------------------------------------------------------------------------

/// The type of a descriptor in a bindless descriptor heap.
enum class DescriptorType : std::uint8_t {
    kSrvTexture,
    kSrvBuffer,
    kUavTexture,
    kUavBuffer,
    kCbv,
    kSampler,
    kAccelerationStructure,
};

// ---------------------------------------------------------------------------
// Geometry structs
// ---------------------------------------------------------------------------

/// A 2D extent in texels.
struct Extent2D {
    std::uint32_t width = 0;
    std::uint32_t height = 0;
};

/// A 3D extent in texels.
struct Extent3D {
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::uint32_t depth = 1;
};

/// A viewport rectangle with depth range.
struct Viewport {
    float x = 0;
    float y = 0;
    float width = 0;
    float height = 0;
    float min_depth = 0;
    float max_depth = 1;
};

/// A scissor rectangle for pixel clipping.
struct Scissor {
    std::int32_t x = 0;
    std::int32_t y = 0;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
};

/// Identifies a single mip level and array layer of a texture.
struct TextureSubresource {
    std::uint32_t mip_level = 0;
    std::uint32_t array_layer = 0;
};

/// Identifies a contiguous range of texture subresources.
struct TextureSubresourceRange {
    std::uint32_t base_mip_level = 0;
    std::uint32_t mip_level_count = 1;
    std::uint32_t base_array_layer = 0;
    std::uint32_t array_layer_count = 1;
};

// ---------------------------------------------------------------------------
// Shader bytecode
// ---------------------------------------------------------------------------

/// A reference to compiled shader bytecode.
struct ShaderBytecode {
    const void* data = nullptr;
    std::uint64_t size_bytes = 0;
};

/// A reference to a compiled shader library for function linking.
struct ShaderLibrary {
    const void* data = nullptr;
    std::uint64_t size_bytes = 0;
};

// ---------------------------------------------------------------------------
// Device descriptor
// ---------------------------------------------------------------------------

/// Configuration for GPU device initialization.
struct DeviceDesc {
    bool enable_validation = false;
    bool enable_gpu_capture = false;
    std::uint32_t frame_count = 3;
};

// ---------------------------------------------------------------------------
// Resource descriptors
// ---------------------------------------------------------------------------

/// Descriptor for creating a GPU texture.
struct TextureDesc {
    std::string_view name;
    TextureDimension dimension = TextureDimension::kTex2d;
    Format format = Format::kRgba8Unorm;
    std::uint32_t width = 1;
    std::uint32_t height = 1;
    std::uint32_t depth_or_layers = 1;
    std::uint32_t mip_levels = 1;
    SampleCount samples = SampleCount::kX1;
    TextureUsageFlags usage = {};
};

/// Descriptor for creating a GPU buffer.
struct BufferDesc {
    std::string_view name;
    std::uint64_t size_bytes = 0;
    HeapType heap_type = HeapType::kDeviceLocal;
    BufferUsageFlags usage = {};
};

/// Descriptor for creating a GPU memory heap.
struct HeapDesc {
    std::uint64_t size_bytes = 0;
    HeapType type = HeapType::kDeviceLocal;
};

/// Size and alignment requirements for a GPU resource allocation.
struct AllocationInfo {
    std::uint64_t size_bytes = 0;
    std::uint64_t alignment = 0;
};

/// Descriptor for creating a sparse (tiled) GPU texture.
struct SparseTextureDesc {
    TextureDesc base;
    std::uint32_t tile_width = 256;
    std::uint32_t tile_height = 256;
};

/// Describes a single sparse tile binding to a heap region.
struct SparseTileBinding {
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

/// Descriptor for creating a ray tracing acceleration structure.
struct AccelerationStructureDesc {
    AccelerationStructureType type;
    BufferHandle buffer;
    std::uint64_t offset = 0;
    std::uint64_t size_bytes = 0;
};

/// Memory size requirements for building an acceleration structure.
struct AccelerationStructureSizes {
    std::uint64_t structure_size_bytes = 0;
    std::uint64_t build_scratch_bytes = 0;
    std::uint64_t update_scratch_bytes = 0;
};

/// Geometry data for a bottom-level acceleration structure build.
struct AccelerationStructureGeometry {
    GeometryType type;

    BufferHandle vertex_buffer;
    std::uint64_t vertex_offset = 0;
    std::uint32_t vertex_count = 0;
    std::uint32_t vertex_stride = 0;
    Format vertex_format = Format::kR32Float;
    BufferHandle index_buffer;
    std::uint64_t index_offset = 0;
    std::uint32_t index_count = 0;
    Format index_format = Format::kR32Uint;
    BufferHandle transform_buffer;
    std::uint64_t transform_offset = 0;

    BufferHandle aabb_buffer;
    std::uint64_t aabb_offset = 0;
    std::uint32_t aabb_count = 0;
    std::uint32_t aabb_stride = 0;

    bool opaque = true;
};

/// Descriptor for an acceleration structure build or update operation.
struct AccelerationStructureBuildDesc {
    AccelerationStructureHandle dst;
    AccelerationStructureHandle src;
    BufferHandle scratch;
    std::uint64_t scratch_offset = 0;
    std::span<const AccelerationStructureGeometry> geometries;
    bool update = false;
};

/// An instance within a top-level acceleration structure.
struct AccelerationStructureInstance {
    float transform[3][4];
    std::uint32_t instance_id = 0;
    std::uint32_t mask = 0xFF;
    std::uint32_t sbt_offset = 0;
    AccelerationStructureHandle blas;
};

// ---------------------------------------------------------------------------
// Pipeline state structs
// ---------------------------------------------------------------------------

/// Per-attachment color blending configuration.
struct BlendAttachment {
    bool blend_enable = false;
    BlendFactor src_color = BlendFactor::kOne;
    BlendFactor dst_color = BlendFactor::kZero;
    BlendOp color_op = BlendOp::kAdd;
    BlendFactor src_alpha = BlendFactor::kOne;
    BlendFactor dst_alpha = BlendFactor::kZero;
    BlendOp alpha_op = BlendOp::kAdd;
    ColorWriteMask write_mask = ColorWriteMask::kAll;
};

/// Blend state across all color attachments.
struct BlendState {
    std::span<const BlendAttachment> attachments;
};

/// Stencil test and operation configuration for one face.
struct StencilOpState {
    StencilOp fail_op = StencilOp::kKeep;
    StencilOp pass_op = StencilOp::kKeep;
    StencilOp depth_fail_op = StencilOp::kKeep;
    CompareOp compare_op = CompareOp::kAlways;
    std::uint8_t compare_mask = 0xFF;
    std::uint8_t write_mask = 0xFF;
    std::uint8_t reference = 0;
};

/// Depth and stencil test configuration.
struct DepthStencilState {
    bool depth_test_enable = true;
    bool depth_write_enable = true;
    CompareOp depth_compare = CompareOp::kGreaterEqual;
    bool stencil_test_enable = false;
    StencilOpState front_stencil;
    StencilOpState back_stencil;
};

/// Rasterizer state configuration.
struct RasterizerState {
    CullMode cull_mode = CullMode::kBack;
    FrontFace front_face = FrontFace::kCounterClockwise;
    PolygonMode polygon_mode = PolygonMode::kFill;
    bool depth_clamp = false;
    float depth_bias = 0;
    float depth_bias_clamp = 0;
    float depth_bias_slope = 0;
    bool conservative_raster = false;
};

// ---------------------------------------------------------------------------
// Render pass structs
// ---------------------------------------------------------------------------

/// A color attachment for a render pass.
struct ColorAttachment {
    TextureHandle texture;
    std::uint32_t mip_level = 0;
    std::uint32_t array_layer = 0;
    LoadOp load_op = LoadOp::kClear;
    StoreOp store_op = StoreOp::kStore;
    float clear_color[4] = {0, 0, 0, 0};
    TextureHandle resolve_texture;
};

/// A depth/stencil attachment for a render pass.
struct DepthStencilAttachment {
    TextureHandle texture;
    std::uint32_t mip_level = 0;
    std::uint32_t array_layer = 0;
    LoadOp depth_load_op = LoadOp::kClear;
    StoreOp depth_store_op = StoreOp::kStore;
    float clear_depth = 0.0f;
    LoadOp stencil_load_op = LoadOp::kDontCare;
    StoreOp stencil_store_op = StoreOp::kDontCare;
    std::uint8_t clear_stencil = 0;
};

/// A variable-rate shading attachment for a render pass.
struct ShadingRateAttachment {
    TextureHandle texture;
    std::uint32_t tile_width = 16;
    std::uint32_t tile_height = 16;
};

/// Descriptor for beginning a render pass.
struct RenderPassDesc {
    std::span<const ColorAttachment> color_attachments;
    std::optional<DepthStencilAttachment> depth_stencil;
    std::optional<ShadingRateAttachment> shading_rate;
    Extent2D render_area;
};

// ---------------------------------------------------------------------------
// Barrier structs
// ---------------------------------------------------------------------------

/// A synchronization barrier for a GPU texture resource.
struct TextureBarrier {
    TextureHandle texture;
    PipelineStage src_stage;
    ResourceAccess src_access;
    TextureLayout old_layout;
    PipelineStage dst_stage;
    ResourceAccess dst_access;
    TextureLayout new_layout;
    TextureSubresourceRange subresource_range;
    QueueType src_queue = QueueType::kGraphics;
    QueueType dst_queue = QueueType::kGraphics;
    bool discard = false;
};

/// A synchronization barrier for a GPU buffer resource.
struct BufferBarrier {
    BufferHandle buffer;
    PipelineStage src_stage;
    ResourceAccess src_access;
    PipelineStage dst_stage;
    ResourceAccess dst_access;
    std::uint64_t offset = 0;
    std::uint64_t size = ~0ULL;
    QueueType src_queue = QueueType::kGraphics;
    QueueType dst_queue = QueueType::kGraphics;
};

/// A global synchronization barrier across all resources.
struct GlobalBarrier {
    PipelineStage src_stage;
    ResourceAccess src_access;
    PipelineStage dst_stage;
    ResourceAccess dst_access;
};

/// A collection of barriers to insert into a command buffer.
struct BarrierDesc {
    std::span<const GlobalBarrier> global_barriers;
    std::span<const TextureBarrier> texture_barriers;
    std::span<const BufferBarrier> buffer_barriers;
};

// ---------------------------------------------------------------------------
// Fence synchronization
// ---------------------------------------------------------------------------

/// A fence signal operation with a target timeline value.
struct FenceSignal {
    FenceHandle fence;
    std::uint64_t value;
};

/// A fence wait operation blocking until a timeline value is reached.
struct FenceWait {
    FenceHandle fence;
    std::uint64_t value;
};

// ---------------------------------------------------------------------------
// Pipeline descriptors
// ---------------------------------------------------------------------------

/// Descriptor for creating a mesh-shader render pipeline.
struct MeshRenderPipelineDesc {
    ShaderBytecode task_shader;
    ShaderBytecode mesh_shader;
    ShaderBytecode pixel_shader;
    std::span<const Format> color_formats;
    Format depth_stencil_format = Format::kUndefined;
    SampleCount samples = SampleCount::kX1;
    BlendState blend;
    RasterizerState rasterizer;
    DepthStencilState depth_stencil;
};

/// Descriptor for creating a linked mesh-shader render pipeline with function linking.
struct LinkedMeshRenderPipelineDesc {
    ShaderBytecode task_shader;
    ShaderBytecode mesh_shader;
    std::span<const ShaderLibrary> pixel_libraries;
    std::string_view entry_point;
    std::span<const Format> color_formats;
    Format depth_stencil_format = Format::kUndefined;
    SampleCount samples = SampleCount::kX1;
    BlendState blend;
    RasterizerState rasterizer;
    DepthStencilState depth_stencil;
};

/// Descriptor for creating a compute pipeline.
struct ComputePipelineDesc {
    ShaderBytecode compute_shader;
};

/// A hit group for a ray tracing pipeline.
struct HitGroup {
    ShaderBytecode closest_hit;
    ShaderBytecode any_hit;
    ShaderBytecode intersection;
};

/// Descriptor for creating a ray tracing pipeline.
struct RayTracingPipelineDesc {
    ShaderBytecode ray_generation;
    std::span<const ShaderBytecode> miss_shaders;
    std::span<const HitGroup> hit_groups;
    std::uint32_t max_recursion_depth = 1;
    std::uint32_t max_payload_size = 32;
    std::uint32_t max_attribute_size = 8;
};

/// Descriptor for creating a work graph program.
struct WorkGraphDesc {
    ShaderBytecode state_object;
    std::string_view program_name;
};

/// Memory requirements for a work graph program.
struct WorkGraphMemoryRequirements {
    std::uint64_t min_backing_memory_bytes = 0;
    std::uint64_t max_backing_memory_bytes = 0;
    std::uint64_t max_input_records_bytes = 0;
};

// ---------------------------------------------------------------------------
// Dispatch descriptors
// ---------------------------------------------------------------------------

/// Descriptor for a ray tracing dispatch.
struct TraceRaysDesc {
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

/// Descriptor for a work graph dispatch.
struct DispatchGraphDesc {
    std::uint32_t entry_point_index = 0;
    const void* cpu_input = nullptr;
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

/// Arguments for an indirect mesh shader dispatch.
struct DispatchMeshIndirectArgs {
    std::uint32_t group_count_x;
    std::uint32_t group_count_y;
    std::uint32_t group_count_z;
};

/// Arguments for an indirect compute dispatch.
struct DispatchIndirectArgs {
    std::uint32_t group_count_x;
    std::uint32_t group_count_y;
    std::uint32_t group_count_z;
};

/// Arguments for an indirect ray tracing dispatch.
struct TraceRaysIndirectArgs {
    std::uint32_t width;
    std::uint32_t height;
    std::uint32_t depth;
};

// ---------------------------------------------------------------------------
// Descriptor and query structs
// ---------------------------------------------------------------------------

/// Descriptor for creating a bindless descriptor heap.
struct DescriptorHeapDesc {
    std::uint32_t max_descriptors = 1'000'000;
    std::uint32_t max_samplers = 2048;
};

/// Descriptor for a GPU sampler object.
struct SamplerDesc {
    // Sampler configuration -- populated by consuming code
};

/// A single descriptor write operation targeting a descriptor heap slot.
struct DescriptorWrite {
    DescriptorType type;
    std::uint32_t index;

    TextureHandle texture;
    Format format = Format::kUndefined;
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

/// Descriptor for creating a GPU query pool.
struct QueryPoolDesc {
    QueryType type = QueryType::kTimestamp;
    std::uint32_t count = 256;
};

// ---------------------------------------------------------------------------
// Swapchain and pipeline cache
// ---------------------------------------------------------------------------

/// Descriptor for creating a GPU swapchain.
struct SwapchainDesc {
    void* native_window = nullptr;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    Format format = Format::kBgra8Unorm;
    std::uint32_t image_count = 3;
    bool vsync = true;
};

/// Descriptor for creating a pipeline cache.
struct PipelineCacheDesc {
    const void* initial_data = nullptr;
    std::uint64_t initial_data_size = 0;
};

// ---------------------------------------------------------------------------
// Device capabilities
// ---------------------------------------------------------------------------

/// Reported GPU device capabilities and limits.
struct DeviceCapabilities {
    // Required Capabilities (initialization fails without these)
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

}  // namespace harmonius::gpu
