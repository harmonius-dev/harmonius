# Harmonius Requirements

Hierarchical requirements for the graphics framework. Each requirement has a
unique identifier (e.g., `R-1.1.3` is the third requirement in section 1.1).
Features reference requirements by ID (e.g., `R-1.1.2`).

## Sections

### 1 Architecture

- [1.1 Core Constraints](1-architecture/1.1-core-constraints.md) — safety,
  rendering paradigm, scope
- [1.2 Platform Support](1-architecture/1.2-platform-support.md) — target
  platforms and GPU APIs
- [1.3 Rendering Pipeline](1-architecture/1.3-rendering-pipeline.md) — HDR,
  G-buffer, motion vectors, post-processing, TAA

### 2 Renderer

These requirements do not apply to this framework. For details about this, see
[README.md](2-renderer/README.md)

- [2.1 Scene and Cameras](2-renderer/2.1-scene-and-cameras.md) — entities,
  cameras, culling, scene capture, dynamic resolution
- [2.2 Lighting](2-renderer/2.2-lighting.md) — point, spot, directional, area
  lights, IBL, IES, DDGI, god rays
- [2.3 Shadows and Occlusion](2-renderer/2.3-shadows-and-occlusion.md) — CSM,
  shadow atlas, soft shadows, AO, VSM, contact/DF/capsule shadows
- [2.4 Materials and Shading](2-renderer/2.4-materials-and-shading.md) — PBR,
  BSDF, SSS, alpha, shader graphs, material instances, decals
- [2.5 Ray Tracing](2-renderer/2.5-ray-tracing.md) — acceleration structures, RT
  reflections, RT GI, path tracing
- [2.6 Environment and Atmosphere](2-renderer/2.6-environment-and-atmosphere.md)
  — sky, fog, clouds, ocean, volumes
- [2.7 Geometry and Meshes](2-renderer/2.7-geometry-and-meshes.md) — meshlets,
  instancing, splines, tessellation, visibility buffer, VRS, LOD generation,
  LOD blending, skinned mesh LOD, static mesh merging
- [2.8 Worlds and Terrain](2-renderer/2.8-worlds-and-terrain.md) — streaming,
  voxels, heightmap terrain, HLOD, virtual textures, voxel LOD
- [2.9 Animation](2-renderer/2.9-animation.md) — skeletal, morph, state
  machines, IK, ragdoll, cloth, hair
- [2.10 UI and 2D](2-renderer/2.10-ui-and-2d.md) — vector/bitmap UI, sprites,
  tilemaps, isometric
- [2.11 Shader and Assets](2-renderer/2.11-shader-and-assets.md) — shader
  graphs, compilation, custom nodes, glTF import
- [2.12 Streaming and IO](2-renderer/2.12-streaming-and-io.md) — priorities,
  pools, tile/voxel streaming, async compute, sync
- [2.13 Post-Processing](2-renderer/2.13-post-processing.md) — bloom, DOF,
  motion blur, tonemapping, color grading, film grain
- [2.14 Anti-Aliasing and Upscaling](2-renderer/2.14-anti-aliasing.md) — TAA,
  TSR, FXAA, MSAA
- [2.15 Hair and Character Rendering](2-renderer/2.15-hair-and-characters.md) —
  strand hair, card hair, eye, cloth, skin shading
- [2.16 Foliage and Vegetation](2-renderer/2.16-foliage-and-vegetation.md) —
  hierarchical instancing, wind, fade, transmission
- [2.17 VFX and Particles](2-renderer/2.17-vfx-and-particles.md) — GPU
  particles, sprites, mesh, ribbons, fluid, collision

### 3 Nonfunctional

- [3.1 Performance](3-nonfunctional/3.1-performance.md) — rendering, memory,
  concurrency
- [3.2 Hardware](3-nonfunctional/3.2-hardware.md) — GPU capabilities and IO
- [3.3 Data Constraints](3-nonfunctional/3.3-data-constraints.md) — limits,
  conventions, formats
- [3.4 Resource Budgets](3-nonfunctional/3.4-resource-budgets.md) — meshlets,
  lights, shadows, streaming, probes, sprites, atlases
- [3.5 Visual Quality](3-nonfunctional/3.5-visual-quality.md) — temporal
  stability, denoising, color precision, LOD, atmosphere

### 4 Quality

- [4.1 Testing and CI](4-quality/4.1-testing-and-ci.md) — testing strategy, CI,
  instrumentation

### 5 API and Extensibility

- [5.1 User API](5-api-and-extensibility/5.1-user-api.md) — declarative scene,
  quality config, error reporting, resource handles
- [5.2 Extensibility](5-api-and-extensibility/5.2-extensibility.md) — shader
  nodes, custom passes, material extensions, animation evaluators

### 6 Render Graph

Low-level requirements for the declarative render graph library. See
[README.md](6-render-graph/README.md) for details and traceability.

- [6.1 Pass Declaration](6-render-graph/6.1-pass-declaration.md) — pass types,
  typed I/O, ordered chains, variants, conditional passes
- [6.2 Resource Management](6-render-graph/6.2-resource-management.md) —
  transient, persistent, imported, history resources, formats, atlas
  sub-allocation
- [6.3 Barriers and Sync](6-render-graph/6.3-barriers-and-sync.md) — automatic
  barriers, layout transitions, batching, cross-queue ownership
- [6.4 Queue Assignment](6-render-graph/6.4-queue-assignment.md) — per-pass
  queue affinity, async compute, transfer queue, fallback
- [6.5 Scheduling and Ordering](6-render-graph/6.5-scheduling-and-ordering.md)
  — topological sort, sub-graph instantiation, priority scheduling
- [6.6 Capability Gating](6-render-graph/6.6-capability-gating.md) — hardware
  gates, hard/soft gates, fallback chains, capability descriptors
- [6.7 Budget Culling](6-render-graph/6.7-budget-culling.md) — GPU timing
  gates, cost/priority annotations, cascading dead-pass elimination
- [6.8 Resource Aliasing](6-render-graph/6.8-resource-aliasing.md) — lifetime
  intervals, aliased heap allocation, pool-based aliasing, memory diagnostics
- [6.9 Multi-View Execution](6-render-graph/6.9-multi-view-execution.md) —
  parameterized sub-graph templates, per-instance resources, array-layer
  targeting
- [6.10 Parallel Encoding](6-render-graph/6.10-parallel-encoding.md) —
  independent command buffers, thread-safe pools, encoding dependency graph
- [6.11 Streaming Integration](6-render-graph/6.11-streaming-integration.md) —
  transfer queue passes, completion fences, residency tracking, eviction
- [6.12 Diagnostics](6-render-graph/6.12-diagnostics.md) — GPU timestamp
  queries, pipeline statistics, transfer throughput, debug overlays
- [6.13 Graph Compilation](6-render-graph/6.13-graph-compilation.md) — DAG
  compilation, dead-pass elimination, compile-time validation, incremental
  recompilation
- [6.14 Per-Frame Execution](6-render-graph/6.14-per-frame-execution.md) —
  topology-data separation, per-frame binding, dynamic resolution, pass
  activation flags

### 7 GPU Runtime

Requirements for the GPU runtime layer. See
[README.md](7-gpu-runtime/README.md) for details and design principles.

- [7.1 Memory Management](7-gpu-runtime/7.1-memory-management.md) — heap
  management, sub-allocation, ring buffers, defragmentation, budget
- [7.2 State Tracking](7-gpu-runtime/7.2-state-tracking.md) — redundant state
  elimination, resource state cache, tracked command buffers
- [7.3 Work Graph Runtime](7-gpu-runtime/7.3-work-graph-runtime.md) — native
  GPU work graph execution, CPU-side emulation, transparent dispatch
- [7.4 Feature Emulation](7-gpu-runtime/7.4-feature-emulation.md) —
  cross-backend feature emulation, barrier optimization, RT pipeline emulation
