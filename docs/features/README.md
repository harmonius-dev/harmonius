# Features

Hierarchical feature specification for the hybrid render graph library. Each feature has a unique identifier (e.g., `F-2.1.3` is the third feature in section 2.1). Features reference requirements from [docs/requirements/](../requirements/) by ID (e.g., `R-1.1.2`).

## Sections

### 1 Rendering
- [1.1 Core Rendering](1-rendering/1.1-core-rendering.md) — culling, projection, instancing, area lights, scene capture, dynamic resolution
- [1.2 Lighting and Materials](1-rendering/1.2-lighting-and-materials.md) — forward+, deferred, PBR, BSDF, material instances, decals, shading models
- [1.3 Shadows and Effects](1-rendering/1.3-shadows-and-effects.md) — shadow maps, virtual shadow maps, AO, SSS, contact shadows
- [1.4 Post-Processing](1-rendering/1.4-post-processing.md) — bloom, DOF, motion blur, tonemapping, color grading, film grain
- [1.5 Anti-Aliasing and Upscaling](1-rendering/1.5-anti-aliasing.md) — TAA, temporal super resolution, FXAA, MSAA

### 2 Advanced Rendering
- [2.1 Ray Tracing](2-advanced-rendering/2.1-ray-tracing.md) — acceleration structures, RT reflections, RT GI, path tracing
- [2.2 Environment](2-advanced-rendering/2.2-environment.md) — sky, volumetrics, clouds, fog, water, heterogeneous volumes
- [2.3 Hair and Character Rendering](2-advanced-rendering/2.3-hair-and-characters.md) — strand hair, card hair, eye, cloth, skin

### 3 Geometry
- [3.1 Meshlet Pipeline](3-geometry/3.1-meshlet-pipeline.md) — virtualized geometry, mesh shaders, tessellation, visibility buffer, VRS
- [3.2 Worlds and Terrain](3-geometry/3.2-worlds-and-terrain.md) — streaming, voxels, terrain, HLOD, procedural generation
- [3.3 Foliage and Vegetation](3-geometry/3.3-foliage-and-vegetation.md) — hierarchical instancing, wind, fade, transmission

### 4 Animation
- [4.1 Animation](4-animation/4.1-animation.md) — skeletal, morph targets, state machines, IK, cloth, hair

### 5 UI and 2D
- [5.1 UI and 2D](5-ui-and-2d/5.1-ui-and-2d.md) — vector/bitmap UI, sprites, tilemaps, isometric

### 6 Tooling and IO
- [6.1 Shader and Asset Pipeline](6-tooling-and-io/6.1-shader-and-assets.md) — shader graph, compilation, asset import
- [6.2 IO and Streaming](6-tooling-and-io/6.2-io-and-streaming.md) — transfer queues, streaming, platform IO, multi-view

### 7 VFX
- [7.1 Particle and VFX Rendering](7-vfx/7.1-particle-rendering.md) — GPU particles, sprites, mesh particles, ribbons, fluid simulation
