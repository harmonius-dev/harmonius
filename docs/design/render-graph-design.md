# Render Graph Module Design

Detailed module design for the Harmonius render graph library. Covers module decomposition,
responsibilities, C++26 API surfaces, build system, and shader toolchain. Derived from the 119
requirements in [RG-1 through RG-14](../requirements/6-render-graph/README.md) and the system
architecture in [architecture.md](../architecture.md).

## Contents

- [Render Graph Module Design](#render-graph-module-design)
  - [Contents](#contents)
  - [Build System and Shader Toolchain](#build-system-and-shader-toolchain)
    - [CMake Structure](#cmake-structure)
    - [vcpkg Dependencies](#vcpkg-dependencies)
    - [Shader Compilation Pipeline](#shader-compilation-pipeline)
  - [Module Map](#module-map)
    - [Namespace Layout](#namespace-layout)
    - [Library Targets](#library-targets)
  - [1. Core Types — `harmonius::rg`](#1-core-types--harmoniusrg)
    - [Handles](#handles)
    - [Pass Types](#pass-types)
    - [Access Modes and Usage Types](#access-modes-and-usage-types)
    - [Queue Types](#queue-types)
    - [Resource Categories](#resource-categories)
    - [Resource Binding Descriptor](#resource-binding-descriptor)
    - [Format Enumeration](#format-enumeration)
    - [Error Types](#error-types)
  - [2. Graph Builder — `harmonius::rg::builder`](#2-graph-builder--harmoniusrgbuilder)
    - [GraphBuilder](#graphbuilder)
    - [Pass Descriptor](#pass-descriptor)
    - [Resource Descriptors](#resource-descriptors)
    - [Sub-Graph Descriptors](#sub-graph-descriptors)
    - [Declared Graph](#declared-graph)
  - [3. Graph Compiler — `harmonius::rg::compiler`](#3-graph-compiler--harmoniusrgcompiler)
    - [Compilation Pipeline](#compilation-pipeline)
    - [GraphCompiler API](#graphcompiler-api)
    - [Execution Plan](#execution-plan)
    - [Validation (RG-13.4)](#validation-rg-134)
    - [Recompilation Triggers (RG-13.5)](#recompilation-triggers-rg-135)
  - [4. Resource System — `harmonius::rg::resource`](#4-resource-system--harmoniusrgresource)
    - [Resource Lifecycle](#resource-lifecycle)
    - [Aliasing Solver](#aliasing-solver)
    - [Pool Allocator](#pool-allocator)
    - [Ring Allocator](#ring-allocator)
    - [Residency Tracking](#residency-tracking)
  - [5. Synchronization Engine — `harmonius::rg::sync`](#5-synchronization-engine--harmoniusrgsync)
    - [Barrier Descriptors](#barrier-descriptors)
    - [Barrier Scheduler](#barrier-scheduler)
    - [Timeline Fence Manager](#timeline-fence-manager)
  - [6. Gating System — `harmonius::rg::gate`](#6-gating-system--harmoniusrggate)
    - [Gate Evaluation Flow](#gate-evaluation-flow)
    - [Capability Descriptor](#capability-descriptor)
    - [Gate Descriptors](#gate-descriptors)
    - [Gate Evaluator](#gate-evaluator)
  - [7. Execution Engine — `harmonius::rg::exec`](#7-execution-engine--harmoniusrgexec)
    - [Per-Frame Execution Sequence](#per-frame-execution-sequence)
    - [Executor API](#executor-api)
    - [Pass Context](#pass-context)
    - [Command Buffer Pool](#command-buffer-pool)
    - [Transfer Pass Injection](#transfer-pass-injection)
  - [8. Diagnostics — `harmonius::rg::diag`](#8-diagnostics--harmoniusrgdiag)
    - [Diagnostics API](#diagnostics-api)
  - [9. GPU Backend Abstraction — `harmonius::gpu`](#9-gpu-backend-abstraction--harmoniusgpu)

---

## Build System and Shader Toolchain

### CMake Structure

The project uses CMake 3.30+ with C++26 (`CMAKE_CXX_STANDARD 26`) and vcpkg for dependency
management. Each module is a separate static library target with explicit dependency edges.

```cmake
# Top-level CMakeLists.txt (sketch)
cmake_minimum_required(VERSION 3.30)
project(harmonius LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(VulkanHeaders CONFIG REQUIRED)
find_package(directx-headers CONFIG REQUIRED)

add_subdirectory(src/rg/core)
add_subdirectory(src/rg/builder)
add_subdirectory(src/rg/compiler)
add_subdirectory(src/rg/exec)
add_subdirectory(src/rg/diag)
add_subdirectory(src/gpu)
add_subdirectory(src/gpu/d3d12)
add_subdirectory(src/gpu/vulkan)
add_subdirectory(src/gpu/metal)
```

### vcpkg Dependencies

**Core dependencies** (all platforms):

| Package         | Purpose                                               |
| --------------- | ----------------------------------------------------- |
| `alembic`       | Alembic (`.abc`) animated mesh and scene data parsing |
| `cgltf`         | Lightweight glTF 2.0 parsing                          |
| `draco`         | Mesh geometry compression                             |
| `libpng`        | PNG image parsing                                     |
| `meshoptimizer` | Mesh optimization, meshlet generation, LOD generation |
| `openexr`       | EXR image parsing                                     |
| `zstd`          | General-purpose compression for asset bundle chunks   |

**Platform-specific dependencies:**

| Platform      | Package              | Purpose                    |
| ------------- | -------------------- | -------------------------- |
| Windows       | `directx-headers`    | D3D12 API headers          |
| Windows       | `directx-dxc`        | DXC shader compiler        |
| Windows       | Agility SDK (vendor) | Latest D3D12 runtime       |
| Linux/SteamOS | `vulkan`             | Vulkan API headers/loader  |
| macOS         | Metal framework      | Metal API (system)         |
| macOS         | `metal-shaderconvtr` | DXIL → Metal IR (vendored) |

GPU memory management, allocation tracking, and defragmentation are provided by the
GPU runtime layer (`harmonius::gpu_runtime::memory`) — no third-party allocators (VMA,
D3D12MA) are used. See [gpu-runtime.md](gpu-runtime.md) for details.

### Shader Compilation Pipeline

All shaders are authored in HLSL and compiled via DirectXShaderCompiler (DXC) to all backend
targets. Metal support uses `metal-shaderconverter` to convert DXIL bytecode to Metal IR.

```mermaid
flowchart LR
    HLSL["HLSL Sources<br/>(.hlsl)"]
    DXC["DirectXShaderCompiler<br/>(DXC)"]
    DXIL["DXIL Bytecode<br/>Direct3D 12"]
    SPIRV["SPIR-V<br/>Vulkan 1.4"]
    MSC["metal-shader-<br/>converter"]
    METALIR["Metal IR<br/>Metal 4"]

    HLSL --> DXC
    DXC -->|"Shader Model 6.9"| DXIL
    DXC -->|"-spirv flag"| SPIRV
    DXIL --> MSC --> METALIR
```

CMake integrates shader compilation as a custom build step:

```cmake
# Shader compilation targets (sketch)
function(harmonius_compile_shaders TARGET)
    foreach(SHADER ${ARGN})
        get_filename_component(NAME ${SHADER} NAME_WE)

        # DXIL output (D3D12)
        add_custom_command(OUTPUT ${NAME}.dxil
            COMMAND dxc -T cs_6_9 -Fo ${NAME}.dxil ${SHADER}
            DEPENDS ${SHADER})

        # SPIR-V output (Vulkan)
        add_custom_command(OUTPUT ${NAME}.spv
            COMMAND dxc -T cs_6_9 -spirv
                -fspv-target-env=vulkan1.3 -Fo ${NAME}.spv ${SHADER}
            DEPENDS ${SHADER})

        # Metal IR output (Metal)
        add_custom_command(OUTPUT ${NAME}.metallib
            COMMAND dxc -T cs_6_9 -Fo ${NAME}.dxil ${SHADER}
            COMMAND metal-shaderconverter ${NAME}.dxil -o ${NAME}.metallib
            DEPENDS ${SHADER})
    endforeach()
endfunction()
```

---

## Module Map

### Namespace Layout

All render graph types live under `harmonius::rg`. The GPU runtime layer lives under
`harmonius::gpu_runtime`. The GPU backend interface lives under `harmonius::gpu`. Render
graph modules access GPU operations through the GPU runtime — not directly through the
backend (R-1.1.7).

| Namespace                        | Architecture subsystem | Purpose                                       |
| -------------------------------- | ---------------------- | --------------------------------------------- |
| `harmonius::rg`                  | Core types             | Handles, enums, descriptors shared by all     |
| `harmonius::rg::builder`         | Graph Builder          | Declarative graph construction API            |
| `harmonius::rg::compiler`        | Graph Compiler         | DAG validation, optimization, plan output     |
| `harmonius::rg::resource`        | Resource System        | Allocation, aliasing, pools, ring buffers     |
| `harmonius::rg::sync`            | Synchronization Engine | Barriers, layout transitions, fences          |
| `harmonius::rg::gate`            | Gating System          | Capability gates, Budget gates, fallbacks     |
| `harmonius::rg::exec`            | Execution Engine       | Per-frame binding, parallel encoding, Submit  |
| `harmonius::rg::diag`            | Diagnostics            | Timestamps, statistics, memory metrics        |
| `harmonius::gpu_runtime`         | GPU Runtime            | Memory, state tracking, work graphs, compat   |
| `harmonius::gpu_runtime::memory` | Memory Manager         | Heap sub-alloc (TLSF), ring buffers, defrag   |
| `harmonius::gpu_runtime::state`  | State Tracker          | Redundant state elim, resource state cache    |
| `harmonius::gpu_runtime::work_graph` | Work Graph Runtime | Native GPU work graphs or CPU emulation    |
| `harmonius::gpu_runtime::compat` | Feature Emulation      | Split Barrier emulation, Barrier optimization |
| `harmonius::gpu`                 | Backend Interface      | D3D12, Vulkan 1.4, Metal 4 device layer       |

```mermaid
graph TD
    subgraph rg["harmonius::rg"]
        core["core<br/>Handles, enums, descriptors"]
        builder["builder<br/>Graph construction"]
        compiler["compiler<br/>DAG optimization"]
        resource["resource<br/>Allocation and aliasing"]
        sync["sync<br/>Barriers and fences"]
        gate["gate<br/>Capability and budget"]
        exec["exec<br/>Per-frame execution"]
        diag["diag<br/>Profiling and statistics"]
    end

    subgraph gprt["harmonius::gpu_runtime"]
        mem["memory<br/>Heap sub-alloc, ring buffers"]
        st["state<br/>State tracking, barriers"]
        wg["work_graph<br/>Native or emulated"]
        compat["compat<br/>Feature emulation"]
    end

    subgraph gpu["harmonius::gpu"]
        backend["backend<br/>D3D12, Vulkan, Metal"]
    end

    builder --> core
    compiler --> core
    compiler --> gate
    resource --> core
    resource --> mem
    sync --> core
    sync --> st
    gate --> core
    exec --> compiler
    exec --> resource
    exec --> sync
    exec --> wg
    exec --> st
    diag --> exec

    mem --> backend
    st --> backend
    wg --> backend
    compat --> backend
```

### Library Targets

Each module compiles to a static library. The GPU backend is selected at build time via CMake
and statically linked — only one backend exists per binary (see
[gpu-backend-interface.md](gpu-backend-interface.md) for rationale). The GPU runtime sits
between the render graph and the GPU backend (see [gpu-runtime.md](gpu-runtime.md)).

```mermaid
flowchart TD
    A["harmonius-rg-core"]
    B["harmonius-rg-builder"]
    C["harmonius-rg-compiler"]
    D["harmonius-rg-exec"]
    E["harmonius-rg-diag"]
    R["harmonius-gpu-runtime"]
    F["harmonius-gpu"]
    G["harmonius-gpu-d3d12"]
    H["harmonius-gpu-vulkan"]
    I["harmonius-gpu-metal"]

    B --> A
    C --> A
    C --> B
    D --> A
    D --> C
    D --> R
    E --> D
    R --> F
    G --> F
    H --> F
    I --> F
```

---

## 1. Core Types — `harmonius::rg`

Shared vocabulary types used by every module. No business logic — only type definitions, enums,
and strongly-typed handles.

**Requirements:** Provides the type foundation for all 119 render graph requirements.

### Handles

Strongly-typed opaque handles prevent accidental misuse. All handles are 32-bit indices into
dense registries.

```cpp
namespace harmonius::rg {

// Strongly-typed handles — distinct types prevent misuse at compile time
enum class PassHandle : uint32_t { kInvalid = UINT32_MAX };
enum class ResourceHandle : uint32_t { kInvalid = UINT32_MAX };
enum class SubGraphHandle : uint32_t { kInvalid = UINT32_MAX };
enum class GateHandle : uint32_t { kInvalid = UINT32_MAX };
enum class ChainHandle : uint32_t { kInvalid = UINT32_MAX };
enum class VariantSlotHandle : uint32_t { kInvalid = UINT32_MAX };

}  // namespace harmonius::rg
```

### Pass Types

Drawn from RG-1.1, RG-1.7, RG-1.13, RG-1.14. Exhaustive — no extensibility needed.

```cpp
namespace harmonius::rg {

enum class PassType : uint8_t {
  kRasterization,
  kCompute,
  kRayTracingDispatch,
  kAccelerationStructureBuild,
  kTransfer,
  kMsaaResolve,
  kPresent,
  kHostCallback,           // RG-1.7: CPU-only, no GPU commands
  kWorkGraph,              // RG-1.13: GPU self-scheduled
  kCheckerboardResolve,    // RG-1.14: half-res reconstruction
  kOpacityMicromapBuild,   // RG-2.25: dedicated OMM build pass
};

}  // namespace harmonius::rg
```

### Access Modes and Usage Types

```cpp
namespace harmonius::rg {

enum class AccessMode : uint8_t {
  kRead,
  kWrite,
  kReadWrite,
};

enum class UsageType : uint8_t {
  kColorAttachment,
  kDepthAttachment,
  kShaderRead,
  kStorageRead,
  kStorageWrite,
  kShadingRateAttachment,  // RG-2.12
  kIndirectArgument,       // RG-2.13
  kAccelerationStructureRead,
  kAccelerationStructureBuildWrite,
  kTransferSrc,
  kTransferDst,
  kPresent,
};

}  // namespace harmonius::rg
```

### Queue Types

```cpp
namespace harmonius::rg {

enum class QueueAffinity : uint8_t {
  kGraphics,
  kAsyncCompute,  // RG-4.2
  kTransfer,      // RG-4.3
  kAny,           // defaults to kGraphics (RG-4.1)
};

}  // namespace harmonius::rg
```

### Resource Categories

```cpp
namespace harmonius::rg {

enum class ResourceCategory : uint8_t {
  kTransient,              // RG-2.1: single-frame, aliasable
  kPersistent,             // RG-2.2: cross-frame, stable allocation
  kImported,               // RG-2.3: external allocation
  kHistory,                // RG-2.4: ping-pong double-buffer
  kMultiFrameHistory,      // RG-2.24: N-way rotation
  kSparse,                 // RG-2.9: tile-granularity residency
  kPoolBacked,             // RG-2.8: fixed-capacity pool
  kStaging,                // RG-2.10: ring-managed host-visible
  kAtlas,                  // RG-2.17: power-of-two tile slots
  kAccelerationStructure,  // RG-2.18: AS with build/read states
};

}  // namespace harmonius::rg
```

### Resource Binding Descriptor

The fundamental unit of pass I/O declaration (RG-1.1).

```cpp
namespace harmonius::rg {

struct ResourceBinding {
  ResourceHandle resource;
  AccessMode access;
  UsageType usage;
  uint32_t array_layer = 0;  // RG-1.5: specific layer target
  uint32_t mip_level = 0;    // RG-2.22: specific mip target
  bool is_history = false;   // reading previous frame
};

}  // namespace harmonius::rg
```

### Format and Sample Count

`Format` and `SampleCount` are defined canonically in `harmonius::gpu` (see
[gpu-backend-interface.md](gpu-backend-interface.md#format-mapping)) and re-exported into
`harmonius::rg`:

```cpp
namespace harmonius::rg {

// Re-exported from harmonius::gpu — single canonical definition
using gpu::Format;
using gpu::SampleCount;

}  // namespace harmonius::rg
```

### Error Types

```cpp
namespace harmonius::rg {

enum class ValidationErrorKind : uint8_t {
  kCycleDetected,          // RG-5.7
  kTypeMismatch,           // RG-13.4
  kUndeclaredResource,     // RG-13.4
  kQueueIncompatibility,   // RG-13.4
  kSingleWriterViolation,  // RG-3.5
  kVariantAmbiguity,       // RG-13.7
  kInstanceCountMismatch,  // RG-13.8
  kHardGateUnsatisfied,    // RG-6.2
  kSampleCountMismatch,    // RG-2.21
};

struct ValidationError {
  ValidationErrorKind kind;
  PassHandle pass;          // pass involved
  ResourceHandle resource;  // resource involved (if applicable)
  std::string message;
};

struct CompileError {
  std::vector<ValidationError> errors;
};

}  // namespace harmonius::rg
```

---

## 2. Graph Builder — `harmonius::rg::builder`

**Responsibility:** Constructs the declarative graph topology. All rendering features are
expressed through this interface without the graph knowing what they represent
(architecture: rendering-agnostic principle).

**Requirements covered:** RG-1.1 through RG-1.14, RG-2.1 through RG-2.25.

### GraphBuilder

The primary entry point for declaring the graph topology. Returns a `DeclaredGraph` on success.

```cpp
namespace harmonius::rg::builder {

class GraphBuilder {
 public:
  explicit GraphBuilder(const gate::CapabilityDescriptor& caps);

  // --- Pass declaration (RG-1.1, RG-1.2) ---
  PassHandle AddPass(PassDescriptor desc);

  // --- Pass chains (RG-1.3) ---
  ChainHandle BeginChain(std::string_view name);
  PassHandle AddChainStep(ChainHandle chain, PassDescriptor desc);
  void EndChain(ChainHandle chain);

  // --- Variant Dispatch (RG-1.4) ---
  VariantSlotHandle DeclareVariantSlot(std::string_view name);
  PassHandle AddVariant(VariantSlotHandle slot, std::string_view variant_name, PassDescriptor desc);

  // --- Sub-graph templates (RG-1.5, RG-9.1) ---
  SubGraphHandle DeclareSubgraphTemplate(std::string_view name, SubGraphDescriptor desc);
  void InstantiateSubgraph(SubGraphHandle tpl, uint32_t instance_count, std::span<const SubGraphBindings> bindings);

  // --- Resource declaration (RG-2.1–2.25) ---
  ResourceHandle DeclareTransient(TransientResourceDesc desc);
  ResourceHandle DeclarePersistent(PersistentResourceDesc desc);
  ResourceHandle DeclareImported(ImportedResourceDesc desc);
  ResourceHandle DeclareHistory(HistoryResourceDesc desc);
  ResourceHandle DeclareMultiFrameHistory(MultiFrameHistoryDesc desc);
  ResourceHandle DeclareSparse(SparseResourceDesc desc);
  ResourceHandle DeclarePool(PoolResourceDesc desc);
  ResourceHandle DeclareStaging(StagingBufferDesc desc);
  ResourceHandle DeclareAtlas(AtlasResourceDesc desc);
  ResourceHandle DeclareAccelerationStructure(AccelStructDesc desc);
  ResourceHandle DeclareRingBuffer(RingBufferDesc desc);      // RG-2.14
  ResourceHandle DeclareBindlessHeap(BindlessHeapDesc desc);  // RG-2.15

  // --- Gates (RG-6.1–6.7) ---
  GateHandle AttachCapabilityGate(PassHandle pass, gate::CapabilityGateDesc desc);
  GateHandle AttachBudgetGate(PassHandle pass, gate::BudgetGateDesc desc);
  GateHandle AttachPathConditionedGate(PassHandle pass,
                                          gate::PathConditionedGateDesc desc);  // RG-6.7
  GateHandle DeclareFallbackChain(std::string_view name, std::span<const gate::FallbackEntry> entries);

  // --- Ordering constraints (RG-5.3, RG-5.4) ---
  void AddOrderingEdge(PassHandle before, PassHandle after);
  void AddFrameDependency(PassHandle src, PassHandle dst,
                            uint32_t frame_offset);  // RG-5.4

  // --- Chain management (RG-1.3) ---
  void RemoveChainStep(ChainHandle chain, PassHandle pass);

  // --- Diagnostics attachment (RG-12.1–12.7) ---
  void AttachTimestampQuery(PassHandle pass, std::string_view name);
  void AttachStatisticsQuery(PassHandle pass, std::string_view name);
  void MarkDebugOverlay(PassHandle pass);  // RG-12.6

  // --- Build ---
  [[nodiscard]]
  std::expected<DeclaredGraph, CompileError> Build();
};

}  // namespace harmonius::rg::builder
```

### Pass Descriptor

```cpp
namespace harmonius::rg::builder {

struct PassDescriptor {
  std::string_view name;                      // RG-1.8: stable debug name
  PassType type;                              // RG-1.1
  QueueAffinity queue = QueueAffinity::kAny;  // RG-4.1
  std::vector<ResourceBinding> inputs;        // read bindings
  std::vector<ResourceBinding> outputs;       // write bindings
  bool conditional = false;                   // RG-1.6
  std::optional<RenderArea> render_area;      // RG-1.9
  std::vector<std::string_view> tags;         // RG-1.8: optional tags

  // Cost/priority for budget culling (RG-7.2)
  float estimated_cost_ms = 0.0f;
  uint32_t priority = 0;

  // RG-1.14: checkerboard frame parity — nullopt if unused, true/false alternates per frame
  std::optional<bool> frame_parity;

  // Execute callback — type-erased, invoked during encoding
  // For host_callback Passes (RG-1.7), this runs on CPU after predecessor fence
  std::move_only_function<void(exec::PassContext&) const> execute;
};

struct RenderArea {
  uint32_t x = 0;
  uint32_t y = 0;
  uint32_t width = 0;
  uint32_t height = 0;
  bool is_per_frame_binding = false;  // resolved at bind time
};

}  // namespace harmonius::rg::builder
```

### Resource Descriptors

```cpp
namespace harmonius::rg::builder {

struct TransientResourceDesc {
  std::string_view name;
  Format format;
  uint32_t width;
  uint32_t height;
  uint32_t depth = 1;
  uint32_t mip_levels = 1;                 // RG-2.22
  uint32_t array_layers = 1;               // RG-2.6
  SampleCount samples = SampleCount::kX1;  // RG-2.21

  // RG-2.5: resolution-scaled dimensions
  std::optional<std::string_view> resolution_param;
};

struct PersistentResourceDesc {
  std::string_view name;
  Format format;
  uint32_t width;
  uint32_t height;
  uint32_t depth = 1;
  uint32_t array_layers = 1;

  // RG-2.23: fixed capacity with runtime active extent
  std::optional<ActiveExtentDesc> active_extent;
};

struct HistoryResourceDesc {
  std::string_view name;
  Format format;
  uint32_t width;
  uint32_t height;

  // RG-2.5: resolution-scaled
  std::optional<std::string_view> resolution_param;
};

struct MultiFrameHistoryDesc {
  std::string_view name;
  Format format;
  uint32_t width;
  uint32_t height;
  uint32_t history_depth;  // RG-2.24: N >= 2

  std::optional<std::string_view> resolution_param;
};

struct ImportedResourceDesc {
  std::string_view name;
  gpu::ResourceHandle external_handle;  // opaque backend handle
  AccessMode initial_access;            // RG-2.3: explicit initial state
  UsageType initial_usage;
};

struct SparseResourceDesc {
  std::string_view name;
  Format format;
  uint32_t width;
  uint32_t height;
  uint32_t tile_width;  // RG-2.9: tile granularity
  uint32_t tile_height;
};

struct PoolResourceDesc {
  std::string_view name;
  Format format;
  uint32_t element_width;
  uint32_t element_height;
  uint32_t max_elements;  // RG-2.8: fixed capacity

  // RG-11.5: eviction callback
  std::move_only_function<std::vector<uint32_t>(uint32_t pool_id, uint32_t needed) const> eviction_callback;
};

struct StagingBufferDesc {
  std::string_view name;
  uint64_t size_bytes;  // RG-2.10
};

struct AtlasResourceDesc {
  std::string_view name;
  Format format;
  uint32_t width;
  uint32_t height;
  uint32_t tile_size;  // RG-2.17: power-of-two
};

struct AccelStructDesc {
  std::string_view name;
  ResourceCategory category;          // persistent or transient (scratch)
  bool has_opacity_micromap = false;  // RG-2.25
};

struct ActiveExtentDesc {
  uint32_t max_layers;
  uint32_t max_width;
  uint32_t max_height;
};

// RG-2.14: persistent ring buffer with per-slot single-writer enforcement
struct RingBufferDesc {
  std::string_view name;
  uint64_t slot_size_bytes;
  uint32_t slot_count;
};

// RG-2.15: bindless descriptor heap region managed by the render graph
struct BindlessHeapDesc {
  std::string_view name;
  uint32_t max_descriptors;
  uint32_t max_samplers;
};

}  // namespace harmonius::rg::builder
```

### Sub-Graph Descriptors

```cpp
namespace harmonius::rg::builder {

// RG-9.1: parameterized sub-graph template
struct SubGraphDescriptor {
  std::string_view name;
  uint32_t max_instances;  // RG-1.11: compile-time max
  std::vector<PassDescriptor> passes;
  std::vector<SubGraphParamSlot> param_slots;  // typed parameter declarations
};

struct SubGraphParamSlot {
  std::string_view name;
  bool is_shared;  // RG-9.3 (true) vs RG-9.2 (false)
};

// Per-instance bindings provided at instantiation
struct SubGraphBindings {
  std::vector<ResourceHandle> exclusive_resources;  // RG-9.2
  std::vector<ResourceHandle> shared_resources;     // RG-9.3
  uint32_t target_array_layer;                      // RG-9.4
  // RG-1.12: per-instance variant parameter overrides
  std::unordered_map<VariantSlotHandle, std::string_view> variant_selections;
};

}  // namespace harmonius::rg::builder
```

### Declared Graph

The immutable output of the builder, consumed by the compiler.

```cpp
namespace harmonius::rg::builder {

class DeclaredGraph {
 public:
  [[nodiscard]] std::span<const PassDescriptor> Passes() const;
  [[nodiscard]] uint32_t PassCount() const;
  [[nodiscard]] uint32_t ResourceCount() const;

  // Internal — used by compiler
  friend class compiler::GraphCompiler;

 private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
};

}  // namespace harmonius::rg::builder
```

---

## 3. Graph Compiler — `harmonius::rg::compiler`

**Responsibility:** Transforms the declared DAG into an optimized, immutable `ExecutionPlan`.
This is the most complex subsystem, performing validation, gate evaluation, dead-pass
elimination, topological sort, barrier scheduling, aliasing assignment, queue partitioning,
encoding dependency analysis, and multi-instance merging.

**Requirements covered:** RG-13.1 through RG-13.8, RG-5.1, RG-5.6, RG-5.7.

### Compilation Pipeline

The compiler runs nine sequential stages:

```mermaid
flowchart TD
    DG["Declared Graph"]
    V["1. Validate<br/>cycles, types, single-writer"]
    GE["2. Evaluate Gates<br/>capability pruning, fallback selection"]
    DP["3. Dead-Pass Elimination<br/>transitive cascade"]
    TS["4. Topological Sort<br/>deterministic ordering"]
    BS["5. Barrier Schedule<br/>RAW, WAW, layout transitions"]
    RA["6. Resource Aliasing<br/>lifetime intervals, heap assignment"]
    QP["7. Queue Partitioning<br/>graphics, async-compute, transfer"]
    EG["8. Encoding Plan<br/>parallelizable groups"]
    IM["9. Instance Merge<br/>sub-graph unification,<br/>shared barrier dedup (RG-9.3)"]
    EP["ExecutionPlan<br/>immutable output"]

    DG --> V --> GE --> DP --> TS --> BS --> RA --> QP --> EG --> IM --> EP
```

### GraphCompiler API

```cpp
namespace harmonius::rg::compiler {

struct CompileOptions {
  // RG-13.5: variant selection triggers recompile
  std::unordered_map<VariantSlotHandle, std::string_view> variant_selections;

  // RG-12.7: opt-out diagnostics at compile time
  bool enable_timestamp_queries = true;
  bool enable_statistics_queries = true;
  bool enable_debug_overlays = true;
};

class GraphCompiler {
 public:
  // Full compilation (RG-13.1)
  [[nodiscard]]
  std::expected<ExecutionPlan, CompileError> Compile(const builder::DeclaredGraph& graph,
                                                     const gate::CapabilityDescriptor& caps,
                                                     const CompileOptions& options = {});

  // Incremental recompilation for residency changes (RG-13.6)
  [[nodiscard]]
  std::expected<ExecutionPlan, CompileError> RecompileResidency(const ExecutionPlan& existing_plan,
                                                                std::span<const resource::ResidencyChange> changes);
};

}  // namespace harmonius::rg::compiler
```

### Execution Plan

The immutable output of compilation, consumed by the execution engine every frame.

```cpp
namespace harmonius::rg::compiler {

class ExecutionPlan {
 public:
  // Sorted pass list with barrier insertion points
  [[nodiscard]] std::span<const ScheduledPass> Passes() const;

  // Per-queue command lists
  [[nodiscard]] std::span<const QueueSubmission> QueueSubmissions() const;

  // Encoding dependency graph — which groups can encode in parallel
  [[nodiscard]] std::span<const EncodingGroup> EncodingGroups() const;

  // Resource aliasing map — transient resources sharing heap ranges
  [[nodiscard]] const resource::AliasingMap& AliasingMap() const;

  // Fence coordination points between queues
  [[nodiscard]] std::span<const FenceCoordination> FencePoints() const;

  // Transfer pass injection point index (RG-14.7)
  [[nodiscard]] uint32_t TransferInjectionIndex() const;

  // Active pass count (after gate evaluation and dead-pass elimination)
  [[nodiscard]] uint32_t ActivePassCount() const;

  // Per-pass conditional activation slots (RG-14.5)
  [[nodiscard]] std::span<const PassHandle> ConditionalPasses() const;

  // Resolution parameter slots (RG-2.20)
  [[nodiscard]] std::span<const ResolutionParam> ResolutionParams() const;

  friend class exec::Executor;

 private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
};

struct ScheduledPass {
  PassHandle handle;
  uint32_t execution_order;
  QueueAffinity queue;
  std::span<const sync::BarrierDesc> pre_barriers;   // barriers before this pass
  std::span<const sync::BarrierDesc> post_barriers;  // barriers after this pass
  uint32_t encoding_group;
  bool is_conditional;
};

struct EncodingGroup {
  uint32_t group_id;
  std::vector<PassHandle> passes;  // passes in this group
  bool parallel;                   // can encode concurrently
};

struct QueueSubmission {
  QueueAffinity queue;
  std::vector<PassHandle> passes;
  std::vector<FenceCoordination> fence_ops;
};

struct FenceCoordination {
  QueueAffinity source_queue;
  QueueAffinity dest_queue;
  uint64_t signal_value;
  uint64_t wait_value;
};

struct ResolutionParam {
  std::string_view name;
  float default_scale = 1.0f;
  float min_scale = 0.25f;
  float max_scale = 1.0f;
};

}  // namespace harmonius::rg::compiler
```

### Validation (RG-13.4)

```cpp
namespace harmonius::rg::compiler {

// Internal validation checks performed in stage 1
// Returns a list of errors; empty list means valid
std::vector<ValidationError> Validate(const builder::DeclaredGraph& graph);

}  // namespace harmonius::rg::compiler
```

The validator checks:

| Check                   | Error kind                | Source  |
| ----------------------- | ------------------------- | ------- |
| Cycle detection         | `kCycleDetected`          | RG-5.7  |
| Type mismatch           | `kTypeMismatch`           | RG-13.4 |
| Undeclared resource     | `kUndeclaredResource`     | RG-13.4 |
| Queue incompatibility   | `kQueueIncompatibility`   | RG-13.4 |
| Single-writer violation | `kSingleWriterViolation` | RG-3.5  |
| Variant ambiguity       | `kVariantAmbiguity`       | RG-13.7 |
| Instance count mismatch | `kInstanceCountMismatch` | RG-13.8 |
| Hard gate unsatisfied   | `kHardGateUnsatisfied`   | RG-6.2  |
| Sample count mismatch   | `kSampleCountMismatch`   | RG-2.21 |

### Recompilation Triggers (RG-13.5)

| Change type                                    | Triggers recompile? |
| ---------------------------------------------- | ------------------- |
| Pass topology (add/remove passes)              | Yes                 |
| Variant selection (AA, lighting, quality tier) | Yes                 |
| Capability set change                          | Yes                 |
| Residency state change                         | Partial (RG-13.6)   |
| Per-frame constants, enable flags, resolution  | No                  |
| Instance count (within max_instances)          | No                  |
| Buffer/texture handles                         | No                  |

---

## 4. Resource System — `harmonius::rg::resource`

**Responsibility:** Manages GPU resource declarations, lifetime computation, aliasing
assignment, pool allocation, and ring buffer management.

**Requirements covered:** RG-2.1–2.25, RG-8.1–8.6.

### Resource Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Declared
    Declared --> Validated: Compile()
    Validated --> Eliminated: dead pass elimination
    Validated --> Allocated: allocate backing memory
    Eliminated --> [*]
    Allocated --> Bound: bind per frame data
    Bound --> Encoding: begin parallel encode
    Encoding --> Submitted: submit command buffers
    Submitted --> Completed: fence signal
    Completed --> Bound: next frame
    Completed --> Released: Release()
    Released --> [*]
```

### Aliasing Solver

Computes lifetime intervals and assigns aliased heap slots for transient resources (RG-8.1–8.5).
The solver operates on both standalone transient resources and intra-pool elements (RG-8.3):
pool elements with disjoint lifetimes are assigned to the same backing heap slot within the
pool's reserved capacity.

```cpp
namespace harmonius::rg::resource {

struct LifetimeInterval {
  ResourceHandle resource;
  uint32_t first_write;  // execution step index
  uint32_t last_read;    // execution step index
};

struct AliasingAssignment {
  ResourceHandle resource;
  uint32_t heap_offset;
  uint32_t heap_size;
  uint32_t heap_index;  // which heap (same-type constraint, RG-8.5)
};

class AliasingMap {
 public:
  [[nodiscard]] std::span<const AliasingAssignment> Assignments() const;
  [[nodiscard]] uint64_t PeakMemoryBytes() const;
  [[nodiscard]] uint64_t TotalLogicalBytes() const;
  [[nodiscard]] float AliasingEfficiency() const;  // RG-8.6: ratio
};

class AliasingSolver {
 public:
  // Compute optimal aliasing from lifetime intervals
  [[nodiscard]]
  AliasingMap Solve(std::span<const LifetimeInterval> intervals, std::span<const ResourceSizeInfo> sizes);
};

struct ResourceSizeInfo {
  ResourceHandle resource;
  uint64_t size_bytes;
  gpu::HeapType heap_type;  // RG-8.5: same-type constraint
};

}  // namespace harmonius::rg::resource
```

### Pool Allocator

Fixed-capacity resource pools with eviction (RG-2.8, RG-8.3).

```cpp
namespace harmonius::rg::resource {

class PoolAllocator {
 public:
  explicit PoolAllocator(const builder::PoolResourceDesc& desc, gpu_runtime::memory::Allocator& allocator);

  // Allocate element from pool, invoking eviction if full (RG-11.5)
  [[nodiscard]]
  std::expected<gpu::ResourceHandle, PoolError> Allocate();

  // Release element back to pool
  void Release(gpu::ResourceHandle handle);

  // RG-7.5: utilization ratio for budget gating
  [[nodiscard]] float Utilization() const;

  [[nodiscard]] uint32_t Capacity() const;
  [[nodiscard]] uint32_t ActiveCount() const;
};

}  // namespace harmonius::rg::resource
```

### Ring Allocator

Lock-free ring buffer for per-frame transient allocations (RG-10.5, RG-8.4).

```cpp
namespace harmonius::rg::resource {

class RingAllocator {
 public:
  // Pre-allocate ring with frame_count slots (typically 3 for triple buffering)
  explicit RingAllocator(uint64_t slot_size_bytes, uint32_t frame_count, gpu_runtime::memory::Allocator& allocator);

  // Lock-free allocation from current frame's slot
  // Returns {offset, mapped_ptr} — no heap allocation (R-3.1.8)
  struct Allocation {
    uint64_t offset;
    void* mapped_ptr;
  };

  [[nodiscard]] std::optional<Allocation> Allocate(uint64_t size, uint64_t alignment);

  // Advance to next frame's slot (called after fence signal)
  void AdvanceFrame(uint32_t frame_index);

  // Underlying GPU buffer for binding
  [[nodiscard]] gpu::ResourceHandle Buffer() const;
};

}  // namespace harmonius::rg::resource
```

### Residency Tracking

```cpp
namespace harmonius::rg::resource {

// RG-11.3: structured buffer of per-tile residency state
struct ResidencyEntry {
  uint32_t tile_x;
  uint32_t tile_y;
  uint8_t resident;  // 0 = not resident, 1 = resident
};

// RG-13.6: incremental recompilation input
struct ResidencyChange {
  ResourceHandle resource;
  uint32_t tile_x;
  uint32_t tile_y;
  bool now_resident;
};

}  // namespace harmonius::rg::resource
```

### GPU Runtime API Usage

The resource system is the primary consumer of the GPU runtime's memory management APIs.
All GPU memory allocation flows through `gpu_runtime::memory::Allocator` — the render graph
never calls `gpu::Device` resource creation methods directly.

| Resource System Operation     | GPU Runtime API                                                 | Notes                                      |
| ----------------------------- | --------------------------------------------------------------- | ------------------------------------------ |
| Transient texture allocation  | `gpu_runtime::memory::Allocator::Allocate(kPlaced)`              | Placed in aliasing heap at computed offset |
| Transient Buffer allocation   | `gpu_runtime::memory::Allocator::Allocate(kPlaced)`              | Placed in aliasing heap at computed offset |
| Persistent texture allocation | `gpu_runtime::memory::Allocator::CreateTexture()`              | Committed resource with own memory         |
| Persistent Buffer allocation  | `gpu_runtime::memory::Allocator::CreateBuffer()`               | Committed resource with own memory         |
| Size/alignment query          | `gpu_runtime::memory::Allocator::QueryAllocationInfo()`       | Feeds aliasing solver packing              |
| Sparse texture creation       | `gpu_runtime::memory::Allocator::CreateSparseTexture()`       | Reserved/sparse-bound resource             |
| Sparse tile binding update    | `gpu_runtime::memory::Allocator::UpdateSparseBindings()`      | Maps/unmaps tiles per residency            |
| Staging Buffer allocation     | `gpu_runtime::memory::RingAllocator::Allocate()`                | Per-frame upload staging                   |
| Resource naming               | `gpu_runtime::state::TrackedCommandBuffer` (pass-through)       | Debug names from resource descriptors      |

**Aliasing heap optimization:** The aliasing solver computes non-overlapping lifetime intervals
and packs resources into the minimum number of heaps. The resource system delegates to
`gpu_runtime::memory::Allocator` with the `kPlaced` allocation strategy, which handles
platform-specific size queries, alignment, heap creation, and placed resource binding
internally. See [gpu-runtime.md](gpu-runtime.md) for memory allocator details.

```mermaid
flowchart TD
    AS["AliasingSolver.Solve()"] -->|"lifetime intervals + sizes"| Pack["Packing algorithm"]
    Pack -->|"heap_offset per resource"| Alloc["gpu_runtime::memory::Allocator::Allocate(kPlaced)"]
    Alloc --> T["TextureHandle"]
    Alloc --> B["BufferHandle"]
```

**Ring allocator GPU mapping:** The `gpu_runtime::memory::RingAllocator` manages an upload-heap
buffer internally. The mapped pointer is divided into `frame_count` slots. Lock-free atomic bumps
allocate within the current frame's slot — no GPU API calls on the hot path.

---

## 5. Synchronization Engine — `harmonius::rg::sync`

**Responsibility:** Derives all synchronization from declared pass I/O. No manual barriers.
Computes RAW barriers, WAW barriers, layout transitions, cross-queue ownership transfers,
barrier merging, and split barriers.

**Requirements covered:** RG-3.1–3.6, RG-10.6.

### Barrier Descriptors

```cpp
namespace harmonius::rg::sync {

enum class BarrierType : uint8_t {
  kMemory,            // RAW or WAW (RG-3.1, RG-3.2)
  kLayoutTransition,  // image layout change (RG-3.3)
  kOwnershipRelease,  // cross-queue Release (RG-3.4)
  kOwnershipAcquire,  // cross-queue acquire (RG-3.4)
  kAliasing,          // aliasing barrier for shared heap
};

struct BarrierDesc {
  BarrierType type;
  ResourceHandle resource;
  UsageType src_usage;
  UsageType dst_usage;
  QueueAffinity src_queue;
  QueueAffinity dst_queue;

  // Sub-resource targeting
  uint32_t mip_level = 0;
  uint32_t array_layer = 0;
  uint32_t mip_count = 1;
  uint32_t layer_count = 1;

  // Split barrier support (RG-3.6)
  bool is_split_begin = false;
  bool is_split_end = false;
};

}  // namespace harmonius::rg::sync
```

### Barrier Scheduler

```cpp
namespace harmonius::rg::sync {

class BarrierScheduler {
 public:
  // Compute barriers for the entire sorted pass list
  // Called by the compiler during stage 5
  struct BarrierSchedule {
    // barriers[i] = barriers to insert before pass i
    std::vector<std::vector<BarrierDesc>> pre_pass_barriers;
    std::vector<std::vector<BarrierDesc>> post_pass_barriers;
  };

  [[nodiscard]]
  BarrierSchedule Compute(std::span<const compiler::ScheduledPass> sorted_passes, const builder::DeclaredGraph& graph);

 private:
  // RG-3.6: merge compatible barriers at same sync point
  void MergeBarriers(std::vector<BarrierDesc>& barriers);

  // RG-3.6: split barriers where hardware supports
  void ApplySplitBarriers(BarrierSchedule& schedule, const gpu::DeviceCapabilities& caps);
};

}  // namespace harmonius::rg::sync
```

### Timeline Fence Manager

```cpp
namespace harmonius::rg::sync {

// RG-10.6: per-queue monotonically increasing fence counters
class TimelineFenceManager {
 public:
  explicit TimelineFenceManager(gpu_runtime::memory::Allocator& allocator);

  // Signal fence on queue after pass submission
  void Signal(QueueAffinity queue, uint64_t value);

  // Wait on fence value before starting pass on queue
  void Wait(QueueAffinity queue, uint64_t value);

  // Poll fence completion (non-blocking)
  [[nodiscard]] bool IsComplete(QueueAffinity queue, uint64_t value) const;

  // Block until fence completes
  void WaitCpu(QueueAffinity queue, uint64_t value);

  // Get current fence value for queue
  [[nodiscard]] uint64_t CurrentValue(QueueAffinity queue) const;

  // Advance all counters for new frame
  // RG-11.7: fence values are monotonic across frames — a frame N+1 pass can
  // wait on a frame N fence value via AddFrameDependency() in the graph builder.
  // The compiler emits the cross-frame wait using the recorded fence value.
  void AdvanceFrame();

 private:
  struct PerQueueFence {
    gpu::FenceHandle fence;
    uint64_t counter = 0;
  };
  std::array<PerQueueFence, 3> fences_;  // graphics, async-compute, transfer
};

}  // namespace harmonius::rg::sync
```

### GPU Runtime Barrier Translation

The synchronization engine translates render graph barrier descriptors into GPU runtime barrier
operations. This translation is the critical path for GPU synchronization correctness.

**Barrier descriptor translation:** The render graph's `rg::sync::BarrierDesc` uses high-level
`UsageType` enums. The synchronization engine translates these to the GPU backend interface's
fine-grained `gpu::PipelineStage`, `gpu::ResourceAccess`, and `gpu::TextureLayout` types and
enqueues them through `gpu_runtime::state::BarrierOptimizer`.

| `rg::UsageType`           | `gpu::PipelineStage`                | `gpu::ResourceAccess`    | `gpu::TextureLayout`       |
| ------------------------- | ----------------------------------- | ------------------------ | -------------------------- |
| `kColorAttachment`        | `kColorOutput`                      | `kColorAttachmentWrite` | `kColorAttachment`         |
| `kDepthAttachment`        | `kDepthStencil`                     | `kDepthStencilWrite`    | `kDepthStencilAttachment` |
| `kShaderRead`             | `kFragmentShader \| kComputeShader` | `kShaderRead`            | `kShaderReadOnly`         |
| `kStorageRead`            | `kComputeShader`                    | `kShaderRead`            | `kGeneral`                  |
| `kStorageWrite`           | `kComputeShader`                    | `kShaderWrite`           | `kGeneral`                  |
| `kShadingRateAttachment` | `kShadingRate`                      | `kShadingRateRead`      | `kShadingRate`             |
| `kIndirectArgument`       | `kIndirectArgument`                 | `kIndirectRead`          | N/A (Buffer only)          |
| `kTransferSrc`            | `kTransfer`                          | `kTransferRead`          | `kTransferSrc`             |
| `kTransferDst`            | `kTransfer`                          | `kTransferWrite`         | `kTransferDst`             |
| `Present`                 | `kNone`                              | `Present`                | `Present`                  |

**Barrier batching optimization:** The barrier scheduler merges compatible barriers at the same
synchronization point into a single batch of `gpu::TextureBarrier`, `gpu::BufferBarrier`, and
`gpu::GlobalBarrier` entries. These are enqueued through `gpu_runtime::state::BarrierOptimizer`,
which flushes them as a single optimized barrier call per synchronization point:

```cpp
// Barriers are enqueued through the GPU runtime barrier optimizer, which handles
// batching, deduplication, split barrier emulation, and backend-specific translation.
gpu_runtime::state::BarrierOptimizer& optimizer = /* ... */;
optimizer.Enqueue(gpu::TextureBarrier{/* transition */});
optimizer.Enqueue(gpu::BufferBarrier{/* transition */});
optimizer.Flush(tracked_cmd);  // emits optimal barrier calls on the tracked command buffer
```

**Split barrier optimization:** The `gpu_runtime::compat` layer transparently emits split
barriers on backends that support them. When producing and consuming passes are separated
by other work, the runtime inserts a split begin after the producer and a split end before
the consumer, allowing the GPU to overlap the transition with intervening work. On backends
without split barrier support (Metal), transitions are collapsed to immediate barriers at
the consumer. The render graph barrier scheduler is unaware of this optimization — it enqueues
transitions through `gpu_runtime::state::BarrierOptimizer` and the runtime handles the rest.
See [gpu-runtime.md](gpu-runtime.md) sections on feature emulation (GR-4.x) for details.

**Cross-queue ownership transfers:** When a barrier's `src_queue` differs from `dst_queue`, the
`gpu_runtime::compat` layer emits the appropriate release/acquire barrier pair. Backend-specific
details (queue family indices, layout transitions, unified memory elision) are handled
transparently by the runtime. See [gpu-runtime.md](gpu-runtime.md) for details.

**Timeline fence mapping:** The `TimelineFenceManager` manages timeline fences through the
GPU runtime layer. Fence creation, signaling, and waiting are delegated to the runtime, which
forwards them to the GPU backend interface internally.

---

## 6. Gating System — `harmonius::rg::gate`

**Responsibility:** Controls which passes are included in the execution plan based on hardware
capabilities, runtime budgets, and configuration. Evaluates gates at compile time (capability)
and runtime (budget), with fallback chain support.

**Requirements covered:** RG-6.1–6.7, RG-7.1–7.6.

### Gate Evaluation Flow

```mermaid
flowchart TD
    P["Pass with Gate"]
    CC{"Capability present?"}
    IH{"Hard gate?"}
    CE["Compile Error"]
    IF{"Fallback chain?"}
    PR["Prune Pass"]
    HB{"Has budget gate?"}
    BC{"Budget satisfied?"}
    IN["Include Pass"]
    RC["Runtime Cull"]
    CN{"More entries?"}
    EL["Eliminate Chain"]
    TN["Try Next Entry"]

    P --> CC
    CC -->|Yes| HB
    CC -->|No| IH
    IH -->|Yes| CE
    IH -->|No| IF
    IF -->|Yes| CN
    IF -->|No| PR
    CN -->|Yes| TN
    CN -->|No| EL
    TN --> CC
    HB -->|No| IN
    HB -->|Yes| BC
    BC -->|Yes| IN
    BC -->|No| RC
```

### Capability Descriptor

```cpp
namespace harmonius::rg::gate {

// RG-6.4: typed capability enumeration, immutable for graph lifetime
struct CapabilityDescriptor {
  bool mesh_shaders = false;
  bool ray_tracing = false;
  bool sparse_textures = false;
  bool async_compute_queue = false;
  bool transfer_queue = false;
  bool shading_rate_images = false;
  bool sixty_four_bit_atomics = false;
  bool gpu_work_graphs = false;
  bool opacity_micromaps = false;
  bool split_barriers = false;

  // Query by capability name for gate evaluation
  [[nodiscard]] bool Has(std::string_view capability) const;
};

}  // namespace harmonius::rg::gate
```

### Gate Descriptors

```cpp
namespace harmonius::rg::gate {

// RG-6.1, RG-6.2: capability gate
struct CapabilityGateDesc {
  std::string_view required_capability;
  bool hard = false;  // hard gate = compile error if missing
};

// RG-7.1: GPU timing feedback gate
struct BudgetGateDesc {
  std::string_view timestamp_query_name;  // which query to sample
  float threshold_ms;                     // cull if exceeded
  uint32_t priority;                      // culling order (RG-7.2)
};

// RG-7.5: pool utilization gate
struct PoolUtilizationGateDesc {
  ResourceHandle pool;
  float utilization_threshold;  // cull if exceeded
  uint32_t priority;
};

// RG-6.3: fallback chain entry
struct FallbackEntry {
  PassHandle pass;
  std::optional<CapabilityGateDesc> capability_gate;
  std::optional<BudgetGateDesc> budget_gate;  // RG-6.6: composite gate
};

// RG-6.7: path-conditioned variant gate
struct PathConditionedGateDesc {
  VariantSlotHandle variant_slot;
  std::string_view required_variant;
};

}  // namespace harmonius::rg::gate
```

### Gate Evaluator

```cpp
namespace harmonius::rg::gate {

class GateEvaluator {
 public:
  // Compile-time evaluation: prune passes with unsatisfied capability gates
  // Returns set of passes to remove from the graph
  [[nodiscard]]
  std::expected<std::vector<PassHandle>, CompileError> EvaluateCompileTime(const builder::DeclaredGraph& graph,
                                                                           const CapabilityDescriptor& caps);

  // Runtime evaluation: budget gates checked per-frame
  // Returns set of passes to skip this frame
  [[nodiscard]]
  std::vector<PassHandle> EvaluateRuntime(const compiler::ExecutionPlan& plan, const diag::TimestampResults& timing,
                                          std::span<const resource::PoolAllocator*> pools);
};

}  // namespace harmonius::rg::gate
```

---

## 7. Execution Engine — `harmonius::rg::exec`

**Responsibility:** Runs the compiled execution plan every frame. Handles per-frame data
binding, budget gate evaluation, transfer pass injection, parallel command encoding, ring
buffer allocation, submission ordering, and frame index management.

**Requirements covered:** RG-14.1–14.8, RG-10.1–10.7.

### Per-Frame Execution Sequence

```mermaid
sequenceDiagram
    participant App as Application
    participant Exec as Executor
    participant Enc as Encoder Threads
    participant GPU as GPU Queues

    App->>Exec: bind_frame_data()
    App->>Exec: set_activation_flags()
    App->>Exec: SetResolutionScale()
    Exec->>Exec: evaluate budget gates
    Exec->>Exec: inject transfer passes
    Exec->>Exec: bind residency map
    Exec->>Enc: dispatch encoding groups
    Enc->>Enc: acquire command buffers
    Enc->>Enc: encode passes in parallel
    Enc->>Enc: allocate from ring buffers
    Enc-->>Exec: return encoded buffers
    Exec->>GPU: submit in topological order
    GPU-->>Exec: timeline fence signals
    Exec->>Exec: advance frame index
```

### Executor API

```cpp
namespace harmonius::rg::exec {

class Executor {
 public:
  explicit Executor(gpu_runtime::memory::Allocator& allocator, gpu_runtime::state::TrackedCommandBuffer& cmd,
                    gpu_runtime::work_graph::WorkGraphExecutor& work_graphs, uint32_t frame_count = 3);

  // --- Per-frame data binding (RG-14.1–14.4) ---

  // RG-14.2: bind buffer/texture handles for this frame
  void BindResource(ResourceHandle slot, gpu::ResourceHandle handle);

  // RG-14.3: bind sub-graph instance parameters
  void BindSubgraphParams(SubGraphHandle tpl, uint32_t instance_index, std::span<const gpu::ResourceHandle> params);

  // RG-14.4, RG-7.4: set resolution scale for named parameter
  void SetResolutionScale(std::string_view param_name, float scale);

  // --- Activation control (RG-14.5) ---

  // Toggle conditional passes without recompilation
  void SetPassActive(PassHandle pass, bool active);

  // RG-1.10: per-instance enable on sub-graph instances
  void SetInstanceActive(SubGraphHandle tpl, uint32_t instance_index, bool active);

  // RG-1.11: per-frame instance count update without recompilation
  void SetInstanceCount(SubGraphHandle tpl, uint32_t count);

  // --- History control (RG-2.19) ---
  void InvalidateHistory(ResourceHandle history_resource);

  // --- Transfer injection (RG-14.7) ---
  void InjectTransfer(TransferPassDesc desc);

  // --- Residency Map (RG-14.8) ---
  void BindResidencyMap(ResourceHandle resource, gpu::ResourceHandle map_buffer);

  // --- Budget gate parameters (RG-7.6) ---
  void SetBudgetThreshold(GateHandle gate, float threshold_ms);

  // --- Execute frame ---
  void Execute(const compiler::ExecutionPlan& plan);

  // --- Frame management ---
  [[nodiscard]] uint64_t FrameIndex() const;  // RG-14.6

 private:
  void EvaluateBudgetGates();
  void DispatchEncodingGroups(const compiler::ExecutionPlan& plan);
  void SubmitCommandBuffers(const compiler::ExecutionPlan& plan);
  void AdvanceFrame();

  gpu_runtime::memory::Allocator& allocator_;
  gpu_runtime::state::TrackedCommandBuffer& tracked_cmd_;
  gpu_runtime::work_graph::WorkGraphExecutor& work_graphs_;
  sync::TimelineFenceManager fence_manager_;
  std::vector<CommandBufferPool> cmd_pools_;  // per-queue (RG-4.6)
  resource::RingAllocator ring_allocator_;
  uint64_t frame_index_ = 0;
  uint32_t frame_count_;
};

}  // namespace harmonius::rg::exec
```

### Pass Context

Provided to pass execute callbacks during encoding.

```cpp
namespace harmonius::rg::exec {

class PassContext {
 public:
  // Access the tracked command buffer for this pass
  [[nodiscard]] gpu_runtime::state::TrackedCommandBuffer& cmd() const;

  // Resolve a resource handle to its GPU-side handle for this frame
  [[nodiscard]] gpu::ResourceHandle Resolve(ResourceHandle handle) const;

  // Ring buffer allocation (RG-10.5) — lock-free, zero-allocation
  [[nodiscard]]
  resource::RingAllocator::Allocation AllocateConstants(uint64_t size, uint64_t alignment = 256);

  // Current frame index (RG-14.6)
  [[nodiscard]] uint64_t FrameIndex() const;

  // Per-pass render area (RG-1.9)
  [[nodiscard]] const builder::RenderArea& RenderArea() const;
};

}  // namespace harmonius::rg::exec
```

### Command Buffer Pool

```cpp
namespace harmonius::rg::exec {

// RG-10.2: thread-safe per-queue command buffer pool
class CommandBufferPool {
 public:
  explicit CommandBufferPool(gpu_runtime::memory::Allocator& allocator, QueueAffinity queue);

  // Lock-free acquire — no cross-queue or cross-thread contention
  [[nodiscard]] gpu_runtime::state::TrackedCommandBuffer acquire();

  // Return command buffer after submission
  void Release(gpu_runtime::state::TrackedCommandBuffer cmd);

  // Reset all buffers for new frame (called after fence signal)
  void ResetFrame(uint32_t frame_index);
};

}  // namespace harmonius::rg::exec
```

### Transfer Pass Injection

```cpp
namespace harmonius::rg::exec {

// RG-14.7: fault-driven transfer pass injection
struct TransferPassDesc {
  gpu::ResourceHandle src_staging;   // source staging buffer
  gpu::ResourceHandle dst_resource;  // destination device resource
  uint64_t src_offset;
  uint64_t dst_offset;
  uint64_t size_bytes;
  int32_t priority = 0;
  uint64_t completion_fence_value = 0;  // RG-11.2: host-pollable fence value // RG-5.5: higher = first
};

}  // namespace harmonius::rg::exec
```

### GPU Runtime API Usage

The execution engine is the primary consumer of the GPU runtime's tracked command buffer and
submission APIs. Every frame, it acquires tracked command buffers, records passes with
automatic state caching, emits barriers through the barrier optimizer, and submits work
through the GPU runtime.

**Per-frame execution translated to GPU runtime calls:**

```mermaid
sequenceDiagram
    participant Exec as Executor
    participant Pool as CommandBufferPool
    participant TCB as TrackedCommandBuffer
    participant RT as gpu_runtime

    Note over Exec,RT: Frame start — wait for frame N-2 to complete
    Exec->>RT: WaitFenceCpu(fence, frame_N_minus_2)
    Exec->>Pool: ResetFrame()

    Note over Exec,RT: Parallel encoding (per thread)
    Exec->>Pool: acquire()
    Pool-->>TCB: tracked command buffer
    TCB->>TCB: Begin()
    TCB->>TCB: Barrier(pre_barriers) [via BarrierOptimizer]
    TCB->>TCB: BeginRenderPass(desc)
    TCB->>TCB: SetPipeline(pso) [cached — skips if redundant]
    TCB->>TCB: PushConstants(data, size) [cached — skips if redundant]
    TCB->>TCB: DispatchMesh(x, y, z)
    TCB->>TCB: EndRenderPass()
    TCB->>TCB: Barrier(post_barriers) [via BarrierOptimizer]
    TCB->>TCB: End()

    Note over Exec,RT: Submission
    Exec->>RT: Submit(graphics, cmd_bufs, signals, waits)
    Exec->>RT: Submit(async_compute, cmd_bufs, signals, waits)
    Exec->>RT: Present(swapchain)
```

**Command pool lifecycle mapping:**

| Execution Engine Operation | GPU Runtime API                                                  | Frequency                     |
| -------------------------- | ---------------------------------------------------------------- | ----------------------------- |
| Pool creation (init)       | `gpu_runtime::state::TrackedCommandBuffer` pool initialization   | Once per (queue, thread) pair |
| Pool Reset                 | Pool Reset via GPU runtime                                       | Once per pool per frame       |
| Buffer allocation          | `gpu_runtime::state::TrackedCommandBuffer` allocation            | Once per pass per frame       |
| Begin recording            | `TrackedCommandBuffer::Begin()`                                  | Once per command Buffer       |
| End recording              | `TrackedCommandBuffer::End()`                                    | Once per command Buffer       |
| Submit to GPU              | Submission via GPU runtime                                       | Once per queue per frame      |

**Pass type to GPU command mapping:**

| `rg::PassType`                 | GPU Commands Emitted                                                                                        |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `rasterization`                | `BeginRenderPass` → `SetPipeline` → `DispatchMesh` / `DispatchMeshIndirectCount` → `EndRenderPass` |
| `Compute`                      | `SetPipeline` → `PushConstants` → `Dispatch` / `DispatchIndirect`                                        |
| `kRayTracingDispatch`         | `SetPipeline` → `TraceRays` / `TraceRaysIndirect`                                                       |
| `kAccelerationStructureBuild` | `BuildAccelerationStructure`                                                                              |
| `kTransfer`                     | `CopyBuffer` / `CopyBufferToTexture` / `CopyTextureToBuffer`                                         |
| `kMsaaResolve`                 | `BeginRenderPass` with `ResolveTexture` set → `EndRenderPass`                                          |
| `Present`                      | Present via GPU runtime                                                                                     |
| `kHostCallback`                | No GPU commands — CPU-only execution after fence Wait                                                       |
| `kWorkGraph`                   | `SetWorkGraph` → `DispatchGraph`                                                                         |
| `kCheckerboardResolve`         | `SetPipeline(resolve_compute_pso)` → `Dispatch` (Compute-based reconstruction)                             |
| `kOpacityMicromapBuild`       | RG-2.25: Build OMM; compiler enforces completion before any RT Dispatch referencing the parent BLAS          |

**Barrier emission:** The executor iterates the `ScheduledPass::pre_barriers` and
`ScheduledPass::post_barriers` from the execution plan. Each barrier set is enqueued through
`gpu_runtime::state::BarrierOptimizer`, which handles batching, deduplication, and split
barrier emulation before flushing to the `TrackedCommandBuffer`. The synchronization engine
has already translated `rg::sync::BarrierDesc` to `gpu::PipelineStage` / `gpu::ResourceAccess` /
`gpu::TextureLayout` values (see [GPU Runtime Barrier Translation](#gpu-runtime-barrier-translation)).

**Swapchain integration:** The executor acquires the next swapchain image and presents through
the GPU runtime layer. The resulting `TextureHandle` is bound as the imported backbuffer
resource. On window resize, the executor requests an idle wait and swapchain resize through
the runtime.

**Optimization: parallel encoding on multiple threads.** The execution engine dispatches
encoding groups to a thread pool. Each thread has its own `TrackedCommandBuffer` pool — no
cross-thread contention on pool or command buffer allocation. The pool reuses cached command
buffers from previous frames, avoiding per-frame GPU object creation. After all encoding groups
complete, the main thread submits the command buffers in topological order through the GPU
runtime.

---

## 8. Diagnostics — `harmonius::rg::diag`

**Responsibility:** Provides GPU timestamp queries, pipeline statistics, transfer throughput
metrics, queue depth counters, GPU readback, debug overlays, and memory diagnostics. All
instrumentation is zero-overhead when disabled (RG-12.7).

**Requirements covered:** RG-12.1–12.7, RG-8.6.

### Diagnostics API

```cpp
namespace harmonius::rg::diag {

// RG-12.1: per-pass GPU timestamp results
struct TimestampResults {
  struct Entry {
    std::string_view pass_name;
    uint64_t begin_ns;
    uint64_t end_ns;
    [[nodiscard]] double DurationMs() const { return static_cast<double>(end_ns - begin_ns) / 1'000'000.0; }
  };
  std::vector<Entry> entries;

  // Lookup by pass name
  [[nodiscard]] std::optional<Entry> Find(std::string_view name) const;
};

// RG-12.2: pipeline statistics
struct PipelineStatistics {
  struct Entry {
    std::string_view pass_name;
    uint64_t primitives_count;
    uint64_t invocations_count;
  };
  std::vector<Entry> entries;
};

// RG-12.3: transfer throughput
struct TransferStatistics {
  struct Entry {
    std::string_view pass_name;
    uint64_t bytes_transferred;
    double latency_ms;
  };
  std::vector<Entry> entries;
  uint64_t total_bytes_per_frame;
};

// RG-8.6: memory diagnostics
struct MemoryDiagnostics {
  uint64_t peak_aliased_bytes;
  uint64_t total_allocated_bytes;
  float aliasing_efficiency;  // logical / physical ratio
  uint32_t active_pool_count;
  uint32_t total_pool_capacity;
};

class DiagnosticsCollector {
 public:
  explicit DiagnosticsCollector(gpu_runtime::memory::Allocator& allocator);

  // Read results after frame fence signals
  [[nodiscard]] TimestampResults ReadTimestamps() const;
  [[nodiscard]] PipelineStatistics ReadStatistics() const;
  [[nodiscard]] TransferStatistics ReadTransferStats() const;
  [[nodiscard]] MemoryDiagnostics ReadMemoryStats() const;

  // RG-12.4: per-queue depth counters (no GPU sync needed)
  [[nodiscard]] uint32_t QueueDepth(QueueAffinity queue) const;

  // GPU readback (RG-12.5)
  struct ReadbackRequest {
    gpu::ResourceHandle src_buffer;
    uint64_t offset;
    uint64_t size;
  };
  void RequestReadback(ReadbackRequest req);
  [[nodiscard]] std::span<const uint8_t> ReadReadback() const;
};

}  // namespace harmonius::rg::diag
```

### GPU Runtime API Usage

The diagnostics module uses the GPU runtime's tracked command buffer for query and debug
labeling APIs to instrument passes without affecting rendering correctness. All
instrumentation is zero-overhead when disabled (RG-12.7) — the execution engine skips
query and label commands entirely.

**Timestamp query flow:**

```mermaid
sequenceDiagram
    participant Diag as DiagnosticsCollector
    participant Alloc as gpu_runtime::memory::Allocator
    participant TCB as gpu_runtime::state::TrackedCommandBuffer
    participant Buf as Readback Buffer

    Note over Diag,Buf: Initialization
    Diag->>Alloc: create query pool + readback buffer

    Note over Diag,Buf: Per-pass instrumentation
    TCB->>TCB: WriteTimestamp(pool, begin_index)
    TCB->>TCB: ... pass commands ...
    TCB->>TCB: WriteTimestamp(pool, end_index)

    Note over Diag,Buf: End of frame
    TCB->>TCB: ResolveQueryPool(pool, 0, count, readback_buf, 0)

    Note over Diag,Buf: After fence Signal (next frame)
    Diag->>Alloc: Map(readback_buf)
    Diag->>Diag: compute durations × timestamp_period_ns
    Diag->>Alloc: Unmap(readback_buf)
```

| Diagnostics Operation | GPU Runtime API                                               | Notes                                     |
| --------------------- | ------------------------------------------------------------- | ----------------------------------------- |
| Create query pool     | `gpu_runtime::state::TrackedCommandBuffer` (pass-through)     | Delegates to backend query pool creation  |
| Write timestamp       | `gpu_runtime::state::TrackedCommandBuffer::WriteTimestamp()` | Injected around pass recording            |
| Resolve queries       | `gpu_runtime::state::TrackedCommandBuffer::ResolveQueryPool()` | End-of-frame readback                  |
| Get timestamp period  | `gpu::DeviceCapabilities` (queried at init)                   | Stored as a capability value              |
| Read results          | `gpu_runtime::memory::Allocator` readback Buffer + Map        | Standard Buffer mapping                   |
| Begin debug label     | `gpu_runtime::state::TrackedCommandBuffer::BeginDebugLabel()` | Pass-through to backend                 |
| End debug label       | `gpu_runtime::state::TrackedCommandBuffer::EndDebugLabel()` | Pass-through to backend                   |
| Name resource         | `gpu_runtime::state::TrackedCommandBuffer` (pass-through)     | Delegates to backend naming               |

**GPU readback:** The `RequestReadback` method enqueues a `CopyBuffer` through the
`TrackedCommandBuffer` from device-local to a readback buffer allocated via the GPU runtime
allocator. After the frame fence signals, `ReadReadback` maps the readback buffer and returns
the contents.

---

## 9. GPU Runtime — `harmonius::gpu_runtime`

**Responsibility:** Shared services layer between the GPU backend interface and the render
graph. Provides memory management (heap sub-allocation, ring buffers, defragmentation), GPU
state tracking (redundant state elimination, resource state caching), transparent work graph
execution (native GPU work graphs when available, CPU-side emulation otherwise), and
cross-backend feature emulation (split barrier emulation, barrier optimization).

**Requirements covered:** GR-1.1–GR-4.5.

The render graph accesses all GPU operations through the GPU runtime — not directly through
the backend. This ensures:

- **Memory management** is centralized in one allocator (TLSF-based, replacing VMA/D3D12MA)
- **State tracking** eliminates redundant GPU API calls across passes
- **Work graph execution** is transparent — the render graph doesn't know whether passes
  are scheduled by the GPU (via native work graphs) or by the CPU (via emulation)
- **Feature emulation** handles cross-backend differences without per-backend logic in the
  render graph

The GPU runtime depends only on `harmonius::gpu` and has no knowledge of render graph
concepts (passes, resource lifetimes, DAGs).

Design, API surfaces, and class diagrams are in dedicated documents:

- [gpu-runtime.md](gpu-runtime.md) — module design, API surfaces, and integration points
- [gpu-runtime-classes.md](gpu-runtime-classes.md) — class diagrams and sequence diagrams

## 10. GPU Backend Interface — `harmonius::gpu`

**Responsibility:** Provides a thin abstraction over Direct3D 12, Vulkan 1.4, and Metal 4.
The GPU runtime layer uses this interface exclusively — no direct API calls. The backend is
selected at build time via CMake and statically linked into the binary. All dispatch is
static — no vtables, no virtual methods, no shared libraries. Backends are thin wrappers
that add no policy, caching, or memory management beyond direct native API calls.

**Requirements covered:** R-1.1.5 (native backends), R-1.2.1–1.2.3 (platform support).

The GPU backend interface, types, cross-backend compatibility shims, and the three backend
implementations are defined in their own dedicated documents:

- [gpu-backend-interface.md](gpu-backend-interface.md) — abstract interface, types, and
  cross-backend compatibility
- [gpu-backend-d3d12.md](gpu-backend-d3d12.md) — Direct3D 12 (Agility SDK 1.619+, SM 6.9)
- [gpu-backend-vulkan.md](gpu-backend-vulkan.md) — Vulkan 1.4
- [gpu-backend-metal.md](gpu-backend-metal.md) — Metal 4
