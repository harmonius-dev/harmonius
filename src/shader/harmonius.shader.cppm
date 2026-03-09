export module harmonius.shader;

import std;
import harmonius.gpu;

export namespace harmonius::shader
{

  // ---------------------------------------------------------------------------
  // Enums
  // ---------------------------------------------------------------------------

  enum class ShaderStage : std::uint8_t
  {
    task,
    mesh,
    pixel,
    compute,
    ray_generation,
    closest_hit,
    any_hit,
    miss,
    intersection,
  };

  enum class ShaderFunctionType : std::uint8_t
  {
    surface_evaluation,
    lighting_model,
    bsdf_layer,
    post_process_effect,
    utility,
  };

  enum class BindingType : std::uint8_t
  {
    srv,
    uav,
    cbv,
    sampler,
  };

  enum class SlotType : std::uint8_t
  {
    float1,
    float2,
    float3,
    float4,
    texture_handle,
    sampler_handle,
  };

  // ---------------------------------------------------------------------------
  // Slot and diagnostic types
  // ---------------------------------------------------------------------------

  struct TypedSlot
  {
    std::string_view name;
    SlotType type;
  };

  struct ShaderDiagnostic
  {
    std::string message;
    std::string_view source_file;
    std::uint32_t line = 0;
    std::uint32_t column = 0;
  };

  // ---------------------------------------------------------------------------
  // Permutation and specialization
  // ---------------------------------------------------------------------------

  struct PermutationKey
  {
    std::vector<std::pair<std::string, std::string>> defines;

    [[nodiscard]] auto hash() const -> std::uint64_t;
    [[nodiscard]] auto operator==(const PermutationKey &) const -> bool = default;
  };

  struct SpecializationMap
  {
    struct Entry
    {
      std::uint32_t constant_id;
      std::string_view axis_name;
      std::uint32_t default_value;
    };

    std::vector<Entry> entries;
  };

  struct SpecializationData
  {
    std::vector<std::pair<std::uint32_t, std::uint32_t>> constants;
  };

  // ---------------------------------------------------------------------------
  // Shader module types
  // ---------------------------------------------------------------------------

  struct ShaderModuleDesc
  {
    std::string_view source_path;
    ShaderStage stage;
    std::string_view entry_point = "main";
    PermutationKey permutation;
    std::uint64_t content_hash;
  };

  struct ShaderModule
  {
    std::vector<std::uint8_t> bytecode;
    ShaderStage stage;
    std::uint64_t content_hash;
  };

  // ---------------------------------------------------------------------------
  // Shader function types
  // ---------------------------------------------------------------------------

  struct FunctionSignature
  {
    std::vector<TypedSlot> inputs;
    std::vector<TypedSlot> outputs;
    std::vector<TypedSlot> resources;
  };

  struct ShaderFunctionDesc
  {
    std::string_view source_path;
    std::string_view function_name;
    ShaderFunctionType type;
    PermutationKey permutation;
    std::uint64_t content_hash;
  };

  struct ShaderFunction
  {
    std::string_view name;
    ShaderFunctionType type;
    std::vector<std::uint8_t> bytecode;
    FunctionSignature signature;
    std::uint64_t content_hash;
  };

  // ---------------------------------------------------------------------------
  // Reflection types
  // ---------------------------------------------------------------------------

  struct BindingReflection
  {
    std::string_view name;
    std::uint32_t register_index;
    std::uint32_t register_space;
    BindingType type;
    std::uint32_t array_size;
  };

  struct ThreadGroupSize
  {
    std::uint32_t x = 1;
    std::uint32_t y = 1;
    std::uint32_t z = 1;
  };

  struct MeshOutputLimits
  {
    std::uint32_t max_vertices = 0;
    std::uint32_t max_primitives = 0;
  };

  struct ShaderReflection
  {
    ShaderStage stage;
    std::string_view entry_point;
    std::vector<BindingReflection> bindings;
    ThreadGroupSize thread_group_size;
    MeshOutputLimits mesh_output_limits;
    std::uint32_t input_signature_hash;
    std::uint32_t output_signature_hash;
  };

  struct FunctionReflection
  {
    std::string_view name;
    FunctionSignature signature;
    std::vector<BindingReflection> bindings;
    std::vector<std::uint32_t> spec_constant_ids;
    bool has_side_effects;
  };

  // ---------------------------------------------------------------------------
  // Shader graph types
  // ---------------------------------------------------------------------------

  struct ShaderNode
  {
    std::uint32_t node_id;
    std::string_view type_name;
    std::vector<TypedSlot> inputs;
    std::vector<TypedSlot> outputs;
  };

  struct ShaderGraph
  {
    std::vector<ShaderNode> nodes;
    std::vector<std::pair<std::uint32_t, std::uint32_t>> edges;
  };

  struct ShaderGraphIR
  {
    std::vector<std::uint8_t> bytecode;
    std::uint64_t content_hash;
  };

  // ---------------------------------------------------------------------------
  // Shader fragment types
  // ---------------------------------------------------------------------------

  struct ShaderFragment
  {
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

  struct StitchEdge
  {
    std::uint32_t source_node;
    std::string_view source_slot;
    std::string_view dest_slot;
  };

  struct StitchNode
  {
    std::uint32_t node_id;
    std::string_view fragment_name;
    std::vector<StitchEdge> input_edges;
  };

  struct StitchingGraph
  {
    std::vector<StitchNode> nodes;
    std::uint32_t output_node;
    SpecializationData specialization;
    std::uint64_t content_hash;
  };

  // ---------------------------------------------------------------------------
  // Pipeline key
  // ---------------------------------------------------------------------------

  struct PipelineKey
  {
    PermutationKey shader_permutation;
    SpecializationData specialization;
    std::vector<gpu::Format> color_formats;
    gpu::Format depth_format = gpu::Format::undefined;
    gpu::SampleCount samples = gpu::SampleCount::x1;
    std::uint64_t render_state_hash;

    [[nodiscard]] auto hash() const -> std::uint64_t;
  };

  // ---------------------------------------------------------------------------
  // ShaderCompiler
  // ---------------------------------------------------------------------------

  class ShaderCompiler
  {
  public:
    explicit ShaderCompiler(std::string_view target_backend);

    [[nodiscard]] auto compile(const ShaderModuleDesc &desc)
        -> std::expected<ShaderModule, ShaderDiagnostic>;

    [[nodiscard]] auto compile_batch(std::span<const ShaderModuleDesc> descs)
        -> std::vector<std::expected<ShaderModule, ShaderDiagnostic>>;

    [[nodiscard]] auto compile_function(const ShaderFunctionDesc &desc)
        -> std::expected<ShaderFunction, ShaderDiagnostic>;

    [[nodiscard]] auto link(
        std::span<const ShaderFunction> functions,
        const StitchingGraph &graph,
        const SpecializationData &spec)
        -> std::expected<ShaderModule, ShaderDiagnostic>;

    auto add_include_path(std::string_view path) -> void;
    auto set_global_define(std::string_view name, std::string_view value) -> void;

  };

  // ---------------------------------------------------------------------------
  // ShaderReflector
  // ---------------------------------------------------------------------------

  class ShaderReflector
  {
  public:
    [[nodiscard]] auto reflect(const ShaderModule &mod)
        -> std::expected<ShaderReflection, ShaderDiagnostic>;

    [[nodiscard]] auto reflect_function(const ShaderFunction &func)
        -> std::expected<FunctionReflection, ShaderDiagnostic>;
  };

  // ---------------------------------------------------------------------------
  // ShaderGraphCompiler
  // ---------------------------------------------------------------------------

  class ShaderGraphCompiler
  {
  public:
    [[nodiscard]] auto compile(const ShaderGraph &graph)
        -> std::expected<ShaderGraphIR, std::vector<ShaderDiagnostic>>;

    [[nodiscard]] auto generate_hlsl(
        const ShaderGraphIR &ir,
        const PermutationKey &perm) -> std::string;

    [[nodiscard]] auto generate_fragments(const ShaderGraphIR &ir)
        -> std::expected<std::vector<ShaderFragment>, std::vector<ShaderDiagnostic>>;

    [[nodiscard]] auto compute_specialization_map(const ShaderGraphIR &ir)
        -> SpecializationMap;
  };

  // ---------------------------------------------------------------------------
  // ShaderFragmentLibrary
  // ---------------------------------------------------------------------------

  class ShaderFragmentLibrary
  {
  public:
    auto register_fragment(ShaderFragment fragment) -> void;

    [[nodiscard]] auto find(std::string_view name) const -> const ShaderFragment *;

    [[nodiscard]] auto find_by_type(ShaderFunctionType type) const
        -> std::vector<const ShaderFragment *>;

    [[nodiscard]] auto compile_all(const ShaderCompiler &compiler)
        -> std::expected<void, std::vector<ShaderDiagnostic>>;

    [[nodiscard]] auto get_compiled(std::string_view name) const
        -> const ShaderFunction *;

    [[nodiscard]] auto library_hash() const -> std::uint64_t;
    [[nodiscard]] auto size() const -> std::uint32_t;
  };

  // ---------------------------------------------------------------------------
  // ShaderStitcher
  // ---------------------------------------------------------------------------

  class ShaderStitcher
  {
  public:
    explicit ShaderStitcher(
        const ShaderFragmentLibrary &library,
        std::string_view target_backend);

    [[nodiscard]] auto validate(const StitchingGraph &graph)
        -> std::expected<void, std::vector<ShaderDiagnostic>>;

    [[nodiscard]] auto link(const StitchingGraph &graph)
        -> std::expected<ShaderModule, std::vector<ShaderDiagnostic>>;
  };

  // ---------------------------------------------------------------------------
  // PipelineStateCache
  // ---------------------------------------------------------------------------

  class PipelineStateCache
  {
  public:
    [[nodiscard]] auto find_or_create(const PipelineKey &key)
        -> std::expected<gpu::PipelineHandle, gpu::PipelineError>;

    [[nodiscard]] auto hit_count() const -> std::uint64_t;
    [[nodiscard]] auto miss_count() const -> std::uint64_t;

    auto save_to_disk(std::string_view path) -> void;
    auto load_from_disk(std::string_view path) -> void;
  };

  // ---------------------------------------------------------------------------
  // ShaderDeliveryManager
  // ---------------------------------------------------------------------------

  class ShaderDeliveryManager
  {
  public:
    auto watch_directory(std::string_view path) -> void;
    auto poll_changes() -> std::vector<std::string>;
    auto invalidate(std::string_view shader_path) -> void;
  };

} // namespace harmonius::shader
