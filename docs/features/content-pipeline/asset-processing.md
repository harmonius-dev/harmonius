# 12.2 — Asset Processing

## Texture Compression

| ID       | Feature                             |
|----------|-------------------------------------|
| F-12.2.1 | Texture Compression (BC, ASTC, ETC) |

1. **F-12.2.1** — Offline compression of imported textures into GPU-native block-compressed formats.
   BC7/BC6H targets desktop and console; ASTC targets mobile and Apple Silicon; ETC2 provides a
   fallback for older mobile GPUs. Quality levels and format selection are driven by import presets
   and per-platform override tables. ASTC on macOS/iOS/Android, ETC2 as mobile fallback.
   - **Deps:** F-12.1.2 (Texture Source Import)
   - **Platform:** Format selection is gated by target platform capabilities: BC on Windows/Xbox,

## Mesh Optimization

| ID       | Feature                                |
|----------|----------------------------------------|
| F-12.2.2 | LOD Generation                         |
| F-12.2.3 | Meshlet Building                       |
| F-12.2.4 | Vertex Cache and Overdraw Optimization |

1. **F-12.2.2** — Automatic discrete LOD chain generation using edge-collapse mesh simplification.
   Each LOD level targets a triangle-count ratio with configurable error thresholds to preserve
   silhouettes. Fully automated LOD generation with no manual artist intervention for worlds with
   millions of unique meshes.
   - **Deps:** F-12.1.1 (Native Asset Ingestion)
2. **F-12.2.3** — Partition each LOD mesh into fixed-size meshlets (max 64 vertices, 124 triangles)
   optimized for mesh shader dispatch. Meshlet bounds (AABB + normal cone) are precomputed for
   GPU-driven culling. Meshlet building is a prerequisite for the engine's GPU-driven rendering
   pipeline.
   - **Deps:** F-12.2.2
3. **F-12.2.4** — Reorder triangles and vertices within each meshlet to maximize post-transform
   vertex cache hit rates and minimize overdraw. Uses meshoptimizer's vertex cache and overdraw
   optimizers as a post-process after meshlet partitioning.
   - **Deps:** F-12.2.3

## Lightmap and Audio Processing

| ID       | Feature                |
|----------|------------------------|
| F-12.2.5 | Lightmap UV Generation |
| F-12.2.6 | Audio Encoding         |

1. **F-12.2.5** — Automatic generation of non-overlapping lightmap UV atlases for static geometry.
   The UV unwrap minimizes chart stretching and maximizes texel density uniformity. Atlas packing
   groups charts by lightmap page to reduce texture switches during baked GI evaluation.
   - **Deps:** F-12.1.1 (Native Asset Ingestion)
2. **F-12.2.6** — Encode imported audio into runtime formats: Opus for voice and music (high
   compression ratio for patch distribution), ADPCM for short sound effects (low decode latency),
   and raw PCM for latency-critical audio. Encoding parameters are controlled by import presets.
   - **Deps:** F-12.1.3 (Audio Source Import)

## Shader Compilation

| ID       | Feature                              |
|----------|--------------------------------------|
| F-12.2.7 | Shader Graph to GLSL Code Generation |

1. **F-12.2.7** — The shader graph compiler traverses the visual node graph (F-15.8.5) and emits
   clean, human-readable GLSL source files. Each graph node is an GLSL function — the compiler
   resolves input connections, generates typed local variables for intermediate values, and composes
   the node functions into a complete shader entry point. Custom nodes are user-authored GLSL
   snippets with function signatures; the compiler calls them with resolved inputs, producing no
   template syntax in the output. The generated .glsl files are valid standalone GLSL with proper
   formatting, comments annotating which graph node produced each section, and correct indentation.
   Output files are fully compatible with GLSL syntax highlighting and GLSL Tools features
   (IntelliSense, error squiggles, go-to-definition) in any IDE. Permutation explosion is managed by
   compiling only reachable variants identified through static analysis of material parameter usage.
   A Rust-based code generation module (using a templating library internally for the compiler, NOT
   in the output) produces the GLSL text. template markers, preprocessor hacks, or non-standard
   extensions.
   - **Deps:** F-15.8.5 (Shader and Material Graphs)
   - **Platform:** Generated GLSL targets Vulkan SPIR-V 1.0+. Output files are pure GLSL with no

## Dependency Tracking

| ID       | Feature                 |
|----------|-------------------------|
| F-12.2.8 | Asset Dependency Graphs |

1. **F-12.2.8** — Track directed acyclic dependency graphs between all assets: meshes reference
   materials, materials reference textures, entity templates reference meshes and scripts. The
   dependency graph drives incremental rebuilds, ensures correct build ordering, detects circular
   references, and enables impact analysis when a shared texture or material changes across
   thousands of dependent assets.
   - **Deps:** F-12.3.2

## Shader Bytecode Compilation

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-12.2.9 | naga Pipeline |

1. **F-12.2.9** — Compile the generated GLSL source files (F-12.2.7) into SPIR-V bytecode via the
   `naga` in-process on every platform. `naga` performs validation, optimization passes
   (dead code elimination, constant folding), and reflection (binding layouts, push constant ranges,
   workgroup sizes). Compiled SPIR-V is cached in the shared build cache (F-15.11.2) keyed by the
   GLSL source hash. Compilation errors report the original GLSL line number and the graph node that
   generated it, enabling click-to-navigate from compiler errors to the visual graph node.
   - **Deps:** F-12.2.7 (GLSL Code Generation), F-15.8.5 (Shader and Material Graphs)
   - **Platform:** SPIR-V targets Vulkan 1.4+ on all platforms

## Collision Shape Generation

| ID        | Feature                         |
|-----------|---------------------------------|
| F-12.2.10 | Collision Shape Auto-Generation |

1. **F-12.2.10** — Generate physics collision shapes automatically from source meshes during asset
   processing. Supports V-HACD convex decomposition for complex concave meshes, quickhull convex
   wrap for simple hulls, and analytic bounding primitive fitting (AABB, sphere, capsule) for fast
   approximation. Parameters are declared per-mesh in the asset import settings so designers get
   usable colliders without manual authoring. Generated shapes are stored alongside the render mesh
   in the processed asset and referenced by the physics collider component at runtime.
   - **Deps:** F-12.1.1 (Native Asset Ingestion)
   - **Platform:** V-HACD decomposition runs offline on desktop build machines. Mobile and console
     builds use precomputed collision shapes — runtime generation is never performed on shipping
     devices.
