export module harmonius.shader;

import harmonius.gpu;

import std;

export namespace harmonius::shader {

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// The pipeline stage a shader targets.
enum class ShaderStage : std::uint8_t {
    kTask,
    kMesh,
    kPixel,
    kCompute,
    kRayGeneration,
    kClosestHit,
    kAnyHit,
    kMiss,
    kIntersection,
};

/// The functional category of a shader function.
enum class ShaderFunctionType : std::uint8_t {
    kSurfaceEvaluation,
    kLightingModel,
    kBsdfLayer,
    kPostProcessEffect,
    kUtility,
};

/// The type of a resource binding in a shader.
enum class BindingType : std::uint8_t {
    kSrv,
    kUav,
    kCbv,
    kSampler,
};

/// The data type of a shader function slot.
enum class SlotType : std::uint8_t {
    kFloat1,
    kFloat2,
    kFloat3,
    kFloat4,
    kTextureHandle,
    kSamplerHandle,
};

// ---------------------------------------------------------------------------
// Slot and diagnostic types
// ---------------------------------------------------------------------------

/// A named, typed slot used in shader function signatures.
struct TypedSlot {
    std::string_view name;
    SlotType type;
};

/// A diagnostic message produced during shader compilation.
struct ShaderDiagnostic {
    std::string message;
    std::string_view source_file;
    std::uint32_t line = 0;
    std::uint32_t column = 0;
};

// ---------------------------------------------------------------------------
// Permutation and specialization
// ---------------------------------------------------------------------------

/// A set of preprocessor defines identifying a shader permutation.
struct PermutationKey {
    std::vector<std::pair<std::string, std::string>> defines;

    /// Computes a hash of this permutation key.
    [[nodiscard]] auto Hash() const -> std::uint64_t;

    /// Compares two permutation keys for equality.
    [[nodiscard]] auto operator==(const PermutationKey&) const -> bool = default;
};

/// Maps specialization constant IDs to named axes with defaults.
struct SpecializationMap {
    /// A single specialization constant mapping entry.
    struct Entry {
        std::uint32_t constant_id;
        std::string_view axis_name;
        std::uint32_t default_value;
    };

    std::vector<Entry> entries;
};

/// Concrete specialization constant values for a shader variant.
struct SpecializationData {
    std::vector<std::pair<std::uint32_t, std::uint32_t>> constants;
};

// ---------------------------------------------------------------------------
// Shader module types
// ---------------------------------------------------------------------------

/// Description of a shader module to compile.
struct ShaderModuleDesc {
    std::string_view source_path;
    ShaderStage stage;
    std::string_view entry_point = "main";
    PermutationKey permutation;
    std::uint64_t content_hash;
};

/// A compiled shader module containing GPU bytecode.
struct ShaderModule {
    std::vector<std::uint8_t> bytecode;
    ShaderStage stage;
    std::uint64_t content_hash;
};

// ---------------------------------------------------------------------------
// Shader function types
// ---------------------------------------------------------------------------

/// The input/output/resource signature of a shader function.
struct FunctionSignature {
    std::vector<TypedSlot> inputs;
    std::vector<TypedSlot> outputs;
    std::vector<TypedSlot> resources;
};

/// Description of a shader function to compile.
struct ShaderFunctionDesc {
    std::string_view source_path;
    std::string_view function_name;
    ShaderFunctionType type;
    PermutationKey permutation;
    std::uint64_t content_hash;
};

/// A compiled shader function with bytecode and signature metadata.
struct ShaderFunction {
    std::string_view name;
    ShaderFunctionType type;
    std::vector<std::uint8_t> bytecode;
    FunctionSignature signature;
    std::uint64_t content_hash;
};

// ---------------------------------------------------------------------------
// Reflection types
// ---------------------------------------------------------------------------

/// Reflection data for a single resource binding.
struct BindingReflection {
    std::string_view name;
    std::uint32_t register_index;
    std::uint32_t register_space;
    BindingType type;
    std::uint32_t array_size;
};

/// Thread group dimensions for a compute or mesh shader.
struct ThreadGroupSize {
    std::uint32_t x = 1;
    std::uint32_t y = 1;
    std::uint32_t z = 1;
};

/// Maximum output limits for a mesh shader.
struct MeshOutputLimits {
    std::uint32_t max_vertices = 0;
    std::uint32_t max_primitives = 0;
};

/// Full reflection data for a compiled shader module.
struct ShaderReflection {
    ShaderStage stage;
    std::string_view entry_point;
    std::vector<BindingReflection> bindings;
    ThreadGroupSize thread_group_size;
    MeshOutputLimits mesh_output_limits;
    std::uint32_t input_signature_hash;
    std::uint32_t output_signature_hash;
};

/// Reflection data for a compiled shader function.
struct FunctionReflection {
    std::string_view name;
    FunctionSignature signature;
    std::vector<BindingReflection> bindings;
    std::vector<std::uint32_t> spec_constant_ids;
    bool has_side_effects;
};

// ---------------------------------------------------------------------------
// Shader graph types
// ---------------------------------------------------------------------------

/// A node in a shader graph with typed input/output slots.
struct ShaderNode {
    std::uint32_t node_id;
    std::string_view type_name;
    std::vector<TypedSlot> inputs;
    std::vector<TypedSlot> outputs;
};

/// A directed graph of shader nodes connected by edges.
struct ShaderGraph {
    std::vector<ShaderNode> nodes;
    std::vector<std::pair<std::uint32_t, std::uint32_t>> edges;
};

/// Intermediate representation bytecode produced from a shader graph.
struct ShaderGraphIR {
    std::vector<std::uint8_t> bytecode;
    std::uint64_t content_hash;
};

// ---------------------------------------------------------------------------
// Shader fragment types
// ---------------------------------------------------------------------------

/// A reusable shader fragment with IR bytecode and specialization support.
struct ShaderFragment {
    std::string_view name;
    ShaderFunctionType type;
    FunctionSignature signature;
    std::vector<std::uint8_t> ir_bytecode;
    std::vector<std::uint32_t> spec_constant_ids;
    std::uint64_t content_hash;
};

// ---------------------------------------------------------------------------
// Stitching graph types
// ---------------------------------------------------------------------------

/// An edge connecting a source node's output slot to a destination slot.
struct StitchEdge {
    std::uint32_t source_node;
    std::string_view source_slot;
    std::string_view dest_slot;
};

/// A node in the stitching graph referencing a named fragment.
struct StitchNode {
    std::uint32_t node_id;
    std::string_view fragment_name;
    std::vector<StitchEdge> input_edges;
};

/// A graph describing how shader fragments are stitched together.
struct StitchingGraph {
    std::vector<StitchNode> nodes;
    std::uint32_t output_node;
    SpecializationData specialization;
    std::uint64_t content_hash;
};

// ---------------------------------------------------------------------------
// Pipeline key
// ---------------------------------------------------------------------------

/// Identifies a unique pipeline state configuration for caching.
struct PipelineKey {
    PermutationKey shader_permutation;
    SpecializationData specialization;
    std::vector<gpu::Format> color_formats;
    gpu::Format depth_format = gpu::Format::kUndefined;
    gpu::SampleCount samples = gpu::SampleCount::kX1;
    std::uint64_t render_state_hash;

    /// Computes a hash of this pipeline key.
    [[nodiscard]] auto Hash() const -> std::uint64_t;
};

// ---------------------------------------------------------------------------
// ShaderCompiler
// ---------------------------------------------------------------------------

/// Compiles shader source code into GPU bytecode.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
class ShaderCompiler {
 public:
    explicit ShaderCompiler(std::string_view target_backend);
    ~ShaderCompiler();

    ShaderCompiler(const ShaderCompiler&) = delete;
    auto operator=(const ShaderCompiler&) -> ShaderCompiler& = delete;

    ShaderCompiler(ShaderCompiler&&) noexcept;
    auto operator=(ShaderCompiler&&) noexcept -> ShaderCompiler&;

    /// Compiles a single shader module from its description.
    [[nodiscard]] auto Compile(const ShaderModuleDesc& desc)
        -> std::expected<ShaderModule, ShaderDiagnostic>;

    /// Compiles a batch of shader modules, returning results for each.
    [[nodiscard]] auto CompileBatch(std::span<const ShaderModuleDesc> descs)
        -> std::vector<std::expected<ShaderModule, ShaderDiagnostic>>;

    /// Compiles a shader function from its description.
    [[nodiscard]] auto CompileFunction(const ShaderFunctionDesc& desc)
        -> std::expected<ShaderFunction, ShaderDiagnostic>;

    /// Links compiled shader functions into a single module using the stitching graph.
    [[nodiscard]] auto Link(std::span<const ShaderFunction> functions,
                            const StitchingGraph& graph,
                            const SpecializationData& spec)
        -> std::expected<ShaderModule, ShaderDiagnostic>;

    /// Adds a directory to the include search path.
    auto AddIncludePath(std::string_view path) -> void;

    /// Sets a global preprocessor define for all compilations.
    auto SetGlobalDefine(std::string_view name, std::string_view value) -> void;
};

// ---------------------------------------------------------------------------
// ShaderReflector
// ---------------------------------------------------------------------------

/// Extracts reflection metadata from compiled shader modules and functions.
///
/// @threadsafety Not thread-safe.
class ShaderReflector {
 public:
    /// Reflects a compiled shader module to extract binding and stage metadata.
    [[nodiscard]] auto Reflect(const ShaderModule& mod)
        -> std::expected<ShaderReflection, ShaderDiagnostic>;

    /// Reflects a compiled shader function to extract its signature and bindings.
    [[nodiscard]] auto ReflectFunction(const ShaderFunction& func)
        -> std::expected<FunctionReflection, ShaderDiagnostic>;
};

// ---------------------------------------------------------------------------
// ShaderGraphCompiler
// ---------------------------------------------------------------------------

/// Compiles shader graphs into intermediate representation and HLSL.
///
/// @threadsafety Not thread-safe.
class ShaderGraphCompiler {
 public:
    /// Compiles a shader graph into intermediate representation bytecode.
    [[nodiscard]] auto Compile(const ShaderGraph& graph)
        -> std::expected<ShaderGraphIR, std::vector<ShaderDiagnostic>>;

    /// Generates HLSL source code from shader graph IR for the given permutation.
    [[nodiscard]] auto GenerateHlsl(const ShaderGraphIR& ir, const PermutationKey& perm)
        -> std::string;

    /// Generates reusable shader fragments from shader graph IR.
    [[nodiscard]] auto GenerateFragments(const ShaderGraphIR& ir)
        -> std::expected<std::vector<ShaderFragment>, std::vector<ShaderDiagnostic>>;

    /// Computes the specialization map from shader graph IR.
    [[nodiscard]] auto ComputeSpecializationMap(const ShaderGraphIR& ir) -> SpecializationMap;
};

// ---------------------------------------------------------------------------
// ShaderFragmentLibrary
// ---------------------------------------------------------------------------

/// Stores and manages reusable shader fragments for stitching.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
class ShaderFragmentLibrary {
 public:
    /// Registers a shader fragment in the library.
    auto RegisterFragment(ShaderFragment fragment) -> void;

    /// Finds a fragment by name, or returns nullptr if not found.
    [[nodiscard]] auto Find(std::string_view name) const -> const ShaderFragment*;

    /// Returns all fragments matching the given function type.
    [[nodiscard]] auto FindByType(ShaderFunctionType type) const
        -> std::vector<const ShaderFragment*>;

    /// Compiles all registered fragments using the given compiler.
    [[nodiscard]] auto CompileAll(const ShaderCompiler& compiler)
        -> std::expected<void, std::vector<ShaderDiagnostic>>;

    /// Returns the compiled function for the named fragment, or nullptr.
    [[nodiscard]] auto GetCompiled(std::string_view name) const -> const ShaderFunction*;

    /// Returns a hash representing the entire library's contents.
    [[nodiscard]] auto LibraryHash() const -> std::uint64_t;

    /// Returns the number of registered fragments.
    [[nodiscard]] auto Size() const -> std::uint32_t;
};

// ---------------------------------------------------------------------------
// ShaderStitcher
// ---------------------------------------------------------------------------

/// Validates and links shader fragments into complete shader modules.
///
/// @threadsafety Not thread-safe.
class ShaderStitcher {
 public:
    explicit ShaderStitcher(const ShaderFragmentLibrary& library,
                            std::string_view target_backend);
    ~ShaderStitcher();

    ShaderStitcher(const ShaderStitcher&) = delete;
    auto operator=(const ShaderStitcher&) -> ShaderStitcher& = delete;

    ShaderStitcher(ShaderStitcher&&) noexcept;
    auto operator=(ShaderStitcher&&) noexcept -> ShaderStitcher&;

    /// Validates a stitching graph for correctness.
    [[nodiscard]] auto Validate(const StitchingGraph& graph)
        -> std::expected<void, std::vector<ShaderDiagnostic>>;

    /// Links fragments according to the stitching graph into a shader module.
    [[nodiscard]] auto Link(const StitchingGraph& graph)
        -> std::expected<ShaderModule, std::vector<ShaderDiagnostic>>;
};

// ---------------------------------------------------------------------------
// PipelineStateCache
// ---------------------------------------------------------------------------

/// Caches compiled pipeline state objects to avoid redundant creation.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
class PipelineStateCache {
 public:
    ~PipelineStateCache();

    PipelineStateCache(const PipelineStateCache&) = delete;
    auto operator=(const PipelineStateCache&) -> PipelineStateCache& = delete;

    PipelineStateCache(PipelineStateCache&&) noexcept;
    auto operator=(PipelineStateCache&&) noexcept -> PipelineStateCache&;

    /// Finds a cached pipeline or creates a new one for the given key.
    [[nodiscard]] auto FindOrCreate(const PipelineKey& key)
        -> std::expected<gpu::PipelineHandle, gpu::PipelineError>;

    /// Returns the number of cache hits since creation.
    [[nodiscard]] auto HitCount() const -> std::uint64_t;

    /// Returns the number of cache misses since creation.
    [[nodiscard]] auto MissCount() const -> std::uint64_t;

    /// Serializes the cache to disk at the given path.
    auto SaveToDisk(std::string_view path) -> void;

    /// Loads a previously serialized cache from disk.
    auto LoadFromDisk(std::string_view path) -> void;
};

// ---------------------------------------------------------------------------
// ShaderDeliveryManager
// ---------------------------------------------------------------------------

/// Monitors shader source directories for changes and manages invalidation.
///
/// @threadsafety Not thread-safe. Synchronize externally if shared across threads.
class ShaderDeliveryManager {
 public:
    ~ShaderDeliveryManager();

    ShaderDeliveryManager(const ShaderDeliveryManager&) = delete;
    auto operator=(const ShaderDeliveryManager&) -> ShaderDeliveryManager& = delete;

    ShaderDeliveryManager(ShaderDeliveryManager&&) noexcept;
    auto operator=(ShaderDeliveryManager&&) noexcept -> ShaderDeliveryManager&;

    /// Begins watching a directory for shader source changes.
    auto WatchDirectory(std::string_view path) -> void;

    /// Returns paths of shaders that changed since the last poll.
    [[nodiscard]] auto PollChanges() -> std::vector<std::string>;

    /// Invalidates any cached state for the shader at the given path.
    auto Invalidate(std::string_view shader_path) -> void;
};

}  // namespace harmonius::shader
