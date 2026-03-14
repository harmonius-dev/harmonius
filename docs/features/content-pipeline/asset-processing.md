# 12.2 — Asset Processing

## Texture Compression

### F-12.2.1 Texture Compression (BC, ASTC, ETC)

Offline compression of imported textures into GPU-native block-compressed formats. BC7/BC6H targets
desktop and console; ASTC targets mobile and Apple Silicon; ETC2 provides a fallback for older
mobile GPUs. Quality levels and format selection are driven by import presets and per-platform
override tables.

- **Requirements:** R-12.2.1
- **Dependencies:** F-12.1.3, F-12.1.4
- **Platform notes:** Format selection is gated by target platform capabilities: BC on Windows/Xbox,
  ASTC on macOS/iOS/Android, ETC2 as mobile fallback.

## Mesh Optimization

### F-12.2.2 LOD Generation

Automatic discrete LOD chain generation using edge-collapse mesh simplification. Each LOD level
targets a triangle-count ratio with configurable error thresholds to preserve silhouettes. An
MMO-scale world with millions of unique meshes requires fully automated LOD generation with no
manual artist intervention.

- **Requirements:** R-12.2.2
- **Dependencies:** F-12.1.1, F-12.1.2
- **Platform notes:** None

### F-12.2.3 Meshlet Building

Partition each LOD mesh into fixed-size meshlets (max 64 vertices, 124 triangles) optimized for mesh
shader dispatch. Meshlet bounds (AABB + normal cone) are precomputed for GPU-driven culling.
Meshlet building is a prerequisite for the engine's GPU-driven rendering pipeline.

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
- **Dependencies:** F-12.1.1, F-12.1.2
- **Platform notes:** None

### F-12.2.6 Audio Encoding

Encode imported audio into runtime formats: Opus for voice and music (high compression ratio for
MMO patch sizes), ADPCM for short sound effects (low decode latency), and raw PCM for
latency-critical audio. Encoding parameters are controlled by import presets.

- **Requirements:** R-12.2.6
- **Dependencies:** F-12.1.5
- **Platform notes:** None

## Shader Compilation

### F-12.2.7 Shader Cross-Compilation (HLSL to SPIR-V / MSL / DXIL)

Compile HLSL source and shader graph IR into platform-native shader bytecode. SPIR-V targets Vulkan,
MSL targets Metal via SPIRV-Cross, and DXIL targets Direct3D 12 via DXC. Permutation explosion is
managed by compiling only reachable variants identified through static analysis of material
parameter usage.

- **Requirements:** R-12.2.7
- **Dependencies:** None
- **Platform notes:** DXC runs on Windows and Linux; SPIRV-Cross and Metal compiler run on macOS.
  Cross-compilation requires a multi-platform build farm.

## Dependency Tracking

### F-12.2.8 Asset Dependency Graphs

Track directed acyclic dependency graphs between all assets: meshes reference materials, materials
reference textures, prefabs reference meshes and scripts. The dependency graph drives incremental
rebuilds, ensures correct build ordering, detects circular references, and enables impact analysis
when a shared texture or material changes across thousands of dependent assets.

- **Requirements:** R-12.2.8
- **Dependencies:** F-12.3.2
- **Platform notes:** None
