# 3.1 — Meshlet Pipeline

## Meshlet Generation

| ID      | Feature |
|---------|------------------------------------- |
| F-3.1.1 | Meshlet Decomposition and Hierarchy |

1. **F-3.1.1** — All imported geometry is decomposed into meshlets of ~64 vertices and ~124
   triangles with per-meshlet bounding sphere, normal cone, and screen-space error bounds. Meshlets
   are organized into a DAG where parent nodes represent coarsened groups of child meshlets, forming
   a continuous LOD hierarchy. Any cut through the DAG yields a watertight mesh, enabling
   fine-grained GPU-driven LOD selection and culling as the atomic unit of the rendering pipeline.
   - **Platform:** Meshlet size (64v/124t) is fixed across platforms. DAG depth and coarsening
     aggressiveness scale per tier — mobile uses fewer hierarchy levels.

## GPU-Driven Culling

| ID      | Feature |
|---------|------------------------------ |
| F-3.1.2 | Two-Phase Occlusion Culling |
| F-3.1.3 | Cluster and Triangle Culling |

1. **F-3.1.2** — A two-phase GPU-driven culling pipeline. Phase one tests all instances against the
   previous frame's hierarchical depth buffer (HZB) to produce an initial visible set. Phase two
   re-projects newly unoccluded geometry against an updated HZB built from phase-one results. This
   eliminates one-frame-late artifacts common in single-pass occlusion culling for MMO scenes with
   dense occluders.
   - **Deps:** F-3.1.1
   - **Platform:** HZB resolution scales per platform. Mobile uses half-res HZB; desktop uses
     full-res. Phase-two retest may be skipped on mobile under budget pressure.
2. **F-3.1.3** — Task shaders perform per-meshlet frustum, backface cone, and occlusion culling.
   Surviving meshlets are dispatched to mesh shaders that execute per-triangle backface and
   small-triangle culling, discarding degenerate and sub-pixel triangles before rasterization. This
   two-level cull hierarchy minimizes wasted rasterizer work across massive open-world scenes.
   - **Deps:** F-3.1.2
   - **Platform:** Task/mesh shader path on hardware with VK_EXT_mesh_shader, Vulkan, or Vulkan mesh
     shaders. Falls back to compute + vertex pipeline on mobile/older GPUs.

## Mesh Shaders / Indirect Draw Fallback

| ID      | Feature |
|---------|-------------------------------------------------- |
| F-3.1.4 | Mesh Shader Pipeline with Indirect Draw Fallback |

1. **F-3.1.4** — Task (amplification) shaders perform per-meshlet culling and LOD selection, then
   dispatch surviving meshlets to mesh shaders that emit triangles directly to the rasterizer. On
   hardware lacking mesh shader support, culling results are written to GPU buffers consumed by
   multi-draw-indirect calls via a compute compaction pass. This fallback preserves GPU-driven
   culling benefits while using the traditional vertex pipeline.
   - **Deps:** F-3.1.1, F-3.1.3
   - **Platform:** Mesh shader path requires VK_EXT_mesh_shader, Vulkan, or Vulkan mesh shaders;
     indirect draw fallback used on older GPUs

## Meshlet LOD

| ID      | Feature |
|---------|---------------------------------- |
| F-3.1.5 | Screen-Space Error LOD Selection |

1. **F-3.1.5** — Runtime LOD selection driven by projected screen-space error of each meshlet
   hierarchy node. The task shader evaluates each node's error bound against the current camera,
   selecting the coarsest hierarchy level that meets the pixel-error threshold. LOD decisions are
   per-meshlet-group with dithered crossfade transitions, enabling smooth quality gradation across a
   single mesh rather than whole-object LOD pops.
   - **Deps:** F-3.1.1, F-3.1.4
   - **Platform:** Pixel-error threshold is higher on mobile (more aggressive LOD) to reduce
     triangle count. Crossfade dither pattern simplified on mobile.

## Meshlet Streaming

| ID      | Feature |
|---------|---------------------------------- |
| F-3.1.6 | On-Demand Meshlet Page Streaming |

1. **F-3.1.6** — Meshlet data is organized into fixed-size pages that stream from disk on demand. A
   GPU-driven feedback pass identifies which meshlet pages are needed for the current view. The
   streaming system prioritizes page requests by screen-space contribution and loads them
   asynchronously via the transfer queue, enabling virtually unlimited world geometry in seamless
   open-world and MMO environments.
   - **Deps:** F-3.1.1
   - **Platform:** Tokio handles platform I/O internally (IOCP on Windows, kqueue on macOS, epoll on
     Linux).

## Visibility Buffer

| ID      | Feature |
|---------|----------------------------- |
| F-3.1.7 | Visibility Buffer Rendering |

1. **F-3.1.7** — A 64-bit visibility buffer stores triangle ID and instance ID per pixel via atomic
   writes during the meshlet rasterization pass. Material evaluation is fully deferred to a
   subsequent fullscreen compute pass that fetches vertex attributes only for visible pixels. This
   eliminates redundant shading, reduces bandwidth versus a traditional G-buffer, and scales to
   millions of unique materials across MMO-scale scenes.
   - **Deps:** F-3.1.4
   - **Platform:** Requires 64-bit atomics support
