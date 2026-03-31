# User Stories -- 3.1 Meshlet Pipeline

## Meshlet Generation

| ID         | Persona                  |
|------------|--------------------------|
| US-3.1.1.1 | engine developer (P-26)  |
| US-3.1.1.2 | technical artist (P-13)  |
| US-3.1.1.3 | environment artist (P-8) |
| US-3.1.1.4 | engine developer (P-26)  |

1. **US-3.1.1.1** -- **As a** engine developer (P-26), **I want** imported geometry decomposed into
   meshlets of ~64 vertices and ~124 triangles with bounding data, **so that** the GPU can cull and
   render at meshlet granularity.
2. **US-3.1.1.2** -- **As a** technical artist (P-13), **I want** meshlets organized into a DAG
   hierarchy for continuous LOD, **so that** any cut through the DAG yields a watertight mesh.
3. **US-3.1.1.3** -- **As a** environment artist (P-8), **I want** the meshlet pipeline to handle
   all imported assets automatically, **so that** I do not manually configure meshlet decomposition
   per asset.
4. **US-3.1.1.4** -- **As a** engine developer (P-26), **I want** per-meshlet normal cones and
   screen-space error bounds stored in the hierarchy, **so that** backface and LOD culling use
   precomputed data.

## GPU-Driven Culling

| ID         | Persona                  |
|------------|--------------------------|
| US-3.1.2.1 | engine developer (P-26)  |
| US-3.1.2.2 | technical artist (P-13)  |
| US-3.1.2.3 | level designer (P-6)     |
| US-3.1.3.1 | engine developer (P-26)  |
| US-3.1.3.2 | technical artist (P-13)  |
| US-3.1.3.3 | engine developer (P-26)  |

1. **US-3.1.2.1** -- **As a** engine developer (P-26), **I want** two-phase GPU-driven occlusion
   culling using the previous frame's HZB, **so that** one-frame-late artifacts are eliminated.
2. **US-3.1.2.2** -- **As a** technical artist (P-13), **I want** phase-two reprojection against an
   updated HZB, **so that** newly unoccluded geometry appears without delay.
3. **US-3.1.2.3** -- **As a** level designer (P-6), **I want** dense scenes with many occluders to
   cull efficiently, **so that** interiors and cluttered environments maintain frame rate.
4. **US-3.1.3.1** -- **As a** engine developer (P-26), **I want** task shaders to perform
   per-meshlet frustum, backface cone, and occlusion culling, **so that** only surviving meshlets
   reach the rasterizer.
5. **US-3.1.3.2** -- **As a** technical artist (P-13), **I want** mesh shaders to discard sub-pixel
   and degenerate triangles, **so that** the rasterizer wastes no work on invisible geometry.
6. **US-3.1.3.3** -- **As a** engine developer (P-26), **I want** a compute-plus-vertex fallback on
   hardware without mesh shader support, **so that** GPU-driven culling works on older GPUs.

## Mesh Shader Pipeline

| ID         | Persona                  |
|------------|--------------------------|
| US-3.1.4.1 | engine developer (P-26)  |
| US-3.1.4.2 | technical artist (P-13)  |
| US-3.1.4.3 | engine developer (P-26)  |

1. **US-3.1.4.1** -- **As a** engine developer (P-26), **I want** task shaders to dispatch surviving
   meshlets to mesh shaders that emit triangles directly, **so that** the full mesh shader pipeline
   is used on modern GPUs.
2. **US-3.1.4.2** -- **As a** technical artist (P-13), **I want** the indirect draw fallback to
   produce identical visual results, **so that** content looks the same on older hardware.
3. **US-3.1.4.3** -- **As a** engine developer (P-26), **I want** a compute compaction pass to feed
   multi-draw-indirect on legacy hardware, **so that** GPU-driven culling benefits are preserved
   without mesh shaders.

## Meshlet LOD

| ID         | Persona                  |
|------------|--------------------------|
| US-3.1.5.1 | technical artist (P-13)  |
| US-3.1.5.2 | engine developer (P-26)  |
| US-3.1.5.3 | environment artist (P-8) |
| US-3.1.5.4 | level designer (P-6)     |

1. **US-3.1.5.1** -- **As a** technical artist (P-13), **I want** LOD selection driven by projected
   screen-space error per meshlet group, **so that** the coarsest acceptable level is chosen
   automatically.
2. **US-3.1.5.2** -- **As a** engine developer (P-26), **I want** the task shader to evaluate error
   bounds against the current camera, **so that** LOD decisions happen on the GPU per meshlet group.
3. **US-3.1.5.3** -- **As a** environment artist (P-8), **I want** dithered crossfade transitions
   between LOD levels, **so that** quality gradation across a mesh is smooth with no visible pops.
4. **US-3.1.5.4** -- **As a** level designer (P-6), **I want** the pixel-error threshold
   configurable per platform tier, **so that** mobile uses more aggressive LOD to reduce triangle
   count.

## Mesh Streaming

| ID         | Persona                  |
|------------|--------------------------|
| US-3.1.6.1 | engine developer (P-26)  |
| US-3.1.6.2 | level designer (P-6)     |
| US-3.1.6.3 | technical artist (P-13)  |
| US-3.1.6.4 | environment artist (P-8) |

1. **US-3.1.6.1** -- **As a** engine developer (P-26), **I want** meshlet data organized into
   fixed-size pages streamed from disk on demand, **so that** world geometry is not limited by
   available memory.
2. **US-3.1.6.2** -- **As a** level designer (P-6), **I want** a GPU feedback pass to identify which
   meshlet pages the current view needs, **so that** streaming prioritizes visible content.
3. **US-3.1.6.3** -- **As a** technical artist (P-13), **I want** page requests prioritized by
   screen-space contribution, **so that** the most visually impactful geometry loads first.
4. **US-3.1.6.4** -- **As a** environment artist (P-8), **I want** pages loaded asynchronously via
   the transfer queue, **so that** streaming never stalls the render thread.

## Visibility Buffer

| ID         | Persona                  |
|------------|--------------------------|
| US-3.1.7.1 | engine developer (P-26)  |
| US-3.1.7.2 | technical artist (P-13)  |
| US-3.1.7.3 | engine developer (P-26)  |

1. **US-3.1.7.1** -- **As a** engine developer (P-26), **I want** a 64-bit visibility buffer storing
   triangle and instance IDs per pixel, **so that** material evaluation is fully deferred to visible
   pixels only.
2. **US-3.1.7.2** -- **As a** technical artist (P-13), **I want** material shading in a fullscreen
   compute pass that fetches attributes only for visible pixels, **so that** redundant shading is
   eliminated.
3. **US-3.1.7.3** -- **As a** engine developer (P-26), **I want** the visibility buffer to scale to
   millions of unique materials, **so that** large open-world scenes render without material
   batching limits.
