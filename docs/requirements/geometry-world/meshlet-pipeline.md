# R-3.1 -- Meshlet Pipeline Requirements

## Meshlet Decomposition

1. **R-3.1.1** -- The engine **SHALL** decompose imported geometry into meshlets of ~64 vertices and
   ~124 triangles with per-meshlet bounding sphere, normal cone, and screen-space error bounds
   organized into a DAG hierarchy.
   - **Rationale:** Meshlet DAGs enable fine-grained GPU culling and continuous LOD as atomic
     rendering units.
   - **Verification:** Import a mesh. Assert meshlets have correct bounds. Assert any DAG cut yields
     a watertight mesh.

## Two-Phase Occlusion Culling

2. **R-3.1.2** -- The engine **SHALL** perform two-phase GPU-driven occlusion culling: phase one
   tests against the previous frame's HZB; phase two retests newly unoccluded geometry against an
   updated HZB.
   - **Rationale:** Two-phase culling eliminates one-frame-late artifacts common in single-pass
     occlusion culling.
   - **Verification:** Place geometry behind an occluder. Move the camera to reveal it. Assert
     geometry appears within one frame with no pop-in delay.

## Cluster and Triangle Culling

3. **R-3.1.3** -- The engine **SHALL** perform per-meshlet frustum, backface cone, and occlusion
   culling in task shaders, and per-triangle backface and small-triangle culling in mesh shaders.
   - **Rationale:** Two-level culling minimizes rasterizer waste across massive open-world scenes.
   - **Verification:** Render a scene with many off-screen meshlets. Assert culled meshlets produce
     zero rasterizer invocations.

## Mesh Shader Fallback

4. **R-3.1.4** -- The engine **SHALL** use task/mesh shaders on supported hardware and fall back to
   compute compaction plus multi-draw-indirect on hardware lacking mesh shader support.
   - **Rationale:** Fallback preserves GPU-driven culling benefits on older GPUs.
   - **Verification:** Run on hardware without mesh shaders. Assert visual output matches the mesh
     shader path.

## Screen-Space Error LOD

5. **R-3.1.5** -- The engine **SHALL** select LOD per meshlet group based on projected screen-space
   error, choosing the coarsest level meeting a configurable pixel-error threshold, with dithered
   crossfade transitions.
   - **Rationale:** Per-meshlet LOD enables smooth quality gradation within a single mesh.
   - **Verification:** View a mesh at varying distances. Assert LOD transitions are smooth with no
     visible pop.

## On-Demand Page Streaming

6. **R-3.1.6** -- The engine **SHALL** organize meshlet data into fixed-size pages streamed from
   disk on demand via Tokio async I/O, prioritized by screen-space contribution.
   - **Rationale:** Page streaming enables virtually unlimited world geometry.
   - **Verification:** Load a scene exceeding VRAM. Assert pages stream without render stalls.
     Assert highest priority pages load first.

## Visibility Buffer

7. **R-3.1.7** -- The engine **SHALL** store a 64-bit visibility buffer (triangle ID + instance ID
   per pixel) and evaluate materials in a deferred fullscreen compute pass.
   - **Rationale:** Visibility buffer eliminates redundant shading and reduces bandwidth versus a
     G-buffer.
   - **Verification:** Render a scene with millions of unique materials. Assert only visible pixels
     are shaded. Assert bandwidth is lower than an equivalent G-buffer.
