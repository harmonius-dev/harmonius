# Requirements

Hierarchical requirements for the hybrid render graph library. Each requirement has a unique identifier (e.g., `R-1.1.3` is the third requirement in section 1.1). Features reference requirements by ID (e.g., `R-1.1.2`).

## Sections

### 1 Architecture
- [1.1 Core Constraints](1-architecture/1.1-core-constraints.md) — safety, rendering paradigm, scope
- [1.2 Platform Support](1-architecture/1.2-platform-support.md) — target platforms and GPU APIs
- [1.3 Rendering Pipeline](1-architecture/1.3-rendering-pipeline.md) — HDR, G-buffer, motion vectors, post-processing, TAA

### 2 Functional
- [2.1 Scene and Cameras](2-functional/2.1-scene-and-cameras.md) — entities, cameras, culling, scene capture, dynamic resolution
- [2.2 Lighting](2-functional/2.2-lighting.md) — point, spot, directional, area lights, IBL, IES, DDGI, god rays
- [2.3 Shadows and Occlusion](2-functional/2.3-shadows-and-occlusion.md) — CSM, shadow atlas, soft shadows, AO, VSM, contact/DF/capsule shadows
- [2.4 Materials and Shading](2-functional/2.4-materials-and-shading.md) — PBR, BSDF, SSS, alpha, shader graphs, material instances, decals
- [2.5 Ray Tracing](2-functional/2.5-ray-tracing.md) — acceleration structures, RT reflections, RT GI, path tracing
- [2.6 Environment and Atmosphere](2-functional/2.6-environment-and-atmosphere.md) — sky, fog, clouds, ocean, volumes
- [2.7 Geometry and Meshes](2-functional/2.7-geometry-and-meshes.md) — meshlets, instancing, splines, tessellation, visibility buffer, VRS
- [2.8 Worlds and Terrain](2-functional/2.8-worlds-and-terrain.md) — streaming, voxels, heightmap terrain, HLOD, virtual textures
- [2.9 Animation](2-functional/2.9-animation.md) — skeletal, morph, state machines, IK, ragdoll, cloth, hair
- [2.10 UI and 2D](2-functional/2.10-ui-and-2d.md) — vector/bitmap UI, sprites, tilemaps, isometric
- [2.11 Shader and Assets](2-functional/2.11-shader-and-assets.md) — shader graphs, compilation, custom nodes, glTF import
- [2.12 Streaming and IO](2-functional/2.12-streaming-and-io.md) — priorities, pools, tile/voxel streaming, async compute, sync
- [2.13 Post-Processing](2-functional/2.13-post-processing.md) — bloom, DOF, motion blur, tonemapping, color grading, film grain
- [2.14 Anti-Aliasing and Upscaling](2-functional/2.14-anti-aliasing.md) — TAA, TSR, FXAA, MSAA
- [2.15 Hair and Character Rendering](2-functional/2.15-hair-and-characters.md) — strand hair, card hair, eye, cloth, skin shading
- [2.16 Foliage and Vegetation](2-functional/2.16-foliage-and-vegetation.md) — hierarchical instancing, wind, fade, transmission
- [2.17 VFX and Particles](2-functional/2.17-vfx-and-particles.md) — GPU particles, sprites, mesh, ribbons, fluid, collision

### 3 Nonfunctional
- [3.1 Performance](3-nonfunctional/3.1-performance.md) — rendering, memory, concurrency
- [3.2 Hardware](3-nonfunctional/3.2-hardware.md) — GPU capabilities and IO
- [3.3 Data Constraints](3-nonfunctional/3.3-data-constraints.md) — limits, conventions, formats
- [3.4 Resource Budgets](3-nonfunctional/3.4-resource-budgets.md) — meshlets, lights, shadows, streaming, probes, sprites, atlases
- [3.5 Visual Quality](3-nonfunctional/3.5-visual-quality.md) — temporal stability, denoising, color precision, LOD, atmosphere

### 4 Quality
- [4.1 Testing and CI](4-quality/4.1-testing-and-ci.md) — testing strategy, CI, instrumentation

### 5 API and Extensibility
- [5.1 User API](5-api-and-extensibility/5.1-user-api.md) — declarative scene, quality config, error reporting, resource handles
- [5.2 Extensibility](5-api-and-extensibility/5.2-extensibility.md) — shader nodes, custom passes, material extensions, animation evaluators
