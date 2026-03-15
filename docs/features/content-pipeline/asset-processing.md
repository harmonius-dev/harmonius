# 12.2 — Asset Processing

## Texture Compression

### F-12.2.1 Texture Compression (BC, ASTC, ETC)

Offline compression of imported textures into GPU-native block-compressed formats. BC7/BC6H targets
desktop and console; ASTC targets mobile and Apple Silicon; ETC2 provides a fallback for older
mobile GPUs. Quality levels and format selection are driven by import presets and per-platform
override tables.

- **Requirements:** R-12.2.1
- **Dependencies:** F-12.1.2 (Texture Source Import)
- **Platform notes:** Format selection is gated by target platform capabilities: BC on Windows/Xbox,
  ASTC on macOS/iOS/Android, ETC2 as mobile fallback.

## Mesh Optimization

### F-12.2.2 LOD Generation

Automatic discrete LOD chain generation using edge-collapse mesh simplification. Each LOD level
targets a triangle-count ratio with configurable error thresholds to preserve silhouettes. Fully
automated LOD generation with no manual artist intervention for worlds with millions of unique
meshes.

- **Requirements:** R-12.2.2
- **Dependencies:** F-12.1.1 (Native Asset Ingestion)
- **Platform notes:** None

### F-12.2.3 Meshlet Building

Partition each LOD mesh into fixed-size meshlets (max 64 vertices, 124 triangles) optimized for mesh
shader dispatch. Meshlet bounds (AABB + normal cone) are precomputed for GPU-driven culling. Meshlet
building is a prerequisite for the engine's GPU-driven rendering pipeline.

- **Requirements:** R-12.2.3
- **Dependencies:** F-12.2.2
- **Platform notes:** None

### F-12.2.4 Vertex Cache and Overdraw Optimization

Reorder triangles and vertices within each meshlet to maximize post-transform vertex cache hit rates
and minimize overdraw. Uses meshoptimizer's vertex cache and overdraw optimizers as a post-process
after meshlet partitioning.

- **Requirements:** R-12.2.4
- **Dependencies:** F-12.2.3
- **Platform notes:** None

## Lightmap and Audio Processing

### F-12.2.5 Lightmap UV Generation

Automatic generation of non-overlapping lightmap UV atlases for static geometry. The UV unwrap
minimizes chart stretching and maximizes texel density uniformity. Atlas packing groups charts by
lightmap page to reduce texture switches during baked GI evaluation.

- **Requirements:** R-12.2.5
- **Dependencies:** F-12.1.1 (Native Asset Ingestion)
- **Platform notes:** None

### F-12.2.6 Audio Encoding

Encode imported audio into runtime formats: Opus for voice and music (high compression ratio for
patch distribution), ADPCM for short sound effects (low decode latency), and raw PCM for
latency-critical audio. Encoding parameters are controlled by import presets.

- **Requirements:** R-12.2.6
- **Dependencies:** F-12.1.3 (Audio Source Import)
- **Platform notes:** None

## Shader Compilation

### F-12.2.7 Shader Graph to HLSL Code Generation

The shader graph compiler traverses the visual node graph (F-15.8.5) and emits clean, human-readable
HLSL source files. Each graph node is an HLSL function — the compiler resolves input connections,
generates typed local variables for intermediate values, and composes the node functions into a
complete shader entry point. Custom nodes are user-authored HLSL snippets with function signatures;
the compiler calls them with resolved inputs, producing no template syntax in the output. The
generated .hlsl files are valid standalone HLSL with proper formatting, comments annotating which
graph node produced each section, and correct indentation. Output files are fully compatible with
HLSL syntax highlighting and HLSL Tools features (IntelliSense, error squiggles, go-to-definition)
in any IDE. Permutation explosion is managed by compiling only reachable variants identified through
static analysis of material parameter usage. A Rust-based code generation module (using a templating
library internally for the compiler, NOT in the output) produces the HLSL text.

- **Requirements:** R-12.2.7
- **Dependencies:** F-15.8.5 (Shader and Material Graphs)
- **Platform notes:** Generated HLSL targets Shader Model 6.0+. Output files are pure HLSL with no
  template markers, preprocessor hacks, or non-standard extensions.

## Dependency Tracking

### F-12.2.8 Asset Dependency Graphs

Track directed acyclic dependency graphs between all assets: meshes reference materials, materials
reference textures, prefabs reference meshes and scripts. The dependency graph drives incremental
rebuilds, ensures correct build ordering, detects circular references, and enables impact analysis
when a shared texture or material changes across thousands of dependent assets.

- **Requirements:** R-12.2.8
- **Dependencies:** F-12.3.2
- **Platform notes:** None

## Shader Bytecode Compilation

### F-12.2.9 DXC and Metal Shader Converter Pipeline

Compile the generated HLSL source files (F-12.2.7) into platform-native shader bytecode. DXC
(Direct3D Shader Compiler) compiles HLSL to DXIL for D3D12 and HLSL to SPIR-V for Vulkan. Metal
Shader Converter translates DXIL to MSL for Metal. Both DXC and Metal Shader Converter are C++
libraries accessed via cxx.rs FFI bindings. DXC performs validation, optimization passes (dead code
elimination, constant folding), and reflection (extracting binding layouts, push constant ranges,
workgroup sizes). Compiled bytecode is cached in the shared build cache (F-15.11.2) keyed by the
HLSL source hash. Compilation errors report the original HLSL line number and the graph node that
generated it, enabling click-to-navigate from compiler errors to the visual graph node.

- **Requirements:** R-12.2.9
- **Dependencies:** F-12.2.7 (HLSL Code Generation), F-15.8.5 (Shader and Material Graphs),
  F-15.11.2 (Shader Compilation Cache)
- **Platform notes:** DXC DXIL output targets Shader Model 6.0+. DXC SPIR-V output targets Vulkan
  1.2+. Metal Shader Converter MSL output targets Metal 2.0+.
