# R-3.1 — Meshlet Pipeline

## R-3.1.1 Meshlet Decomposition and Hierarchy

The engine **SHALL** decompose all imported geometry into meshlets of ~64 vertices and ~124 triangles, each
annotated with a bounding sphere, normal cone, and screen-space error bound, and organize them into a DAG
whose every cut yields a watertight mesh.

- **Derived from:** [F-3.1.1](../../features/geometry-world/meshlet-pipeline.md)
- **Rationale:** Meshlet DAGs enable fine-grained GPU-driven LOD selection and culling as an atomic unit,
  eliminating whole-object LOD pops and supporting continuous quality gradation.
- **Verification:** Import a reference mesh and confirm meshlet sizes stay within bounds, every DAG cut
  produces a watertight mesh (no T-junctions or holes), and bounding metadata is present on each node.

## R-3.1.2 Two-Phase Occlusion Culling

The engine **SHALL** implement a two-phase GPU-driven occlusion culling pipeline that tests instances against
the previous frame's hierarchical depth buffer in phase one and re-projects newly unoccluded geometry against
an updated HZB in phase two.

- **Derived from:** [F-3.1.2](../../features/geometry-world/meshlet-pipeline.md)
- **Rationale:** Two-phase culling eliminates one-frame-late disocclusion artifacts common in single-pass
  schemes, which is critical for dense MMO scenes with many dynamic occluders.
- **Verification:** Render a scene where a large occluder moves to reveal geometry behind it and confirm no
  one-frame pop-in artifacts occur; measure that occluded instances produce zero rasterizer invocations.

## R-3.1.3 Cluster and Triangle Culling

The engine **SHALL** perform per-meshlet frustum, backface-cone, and occlusion culling in task shaders, then
per-triangle backface and small-triangle culling in mesh shaders, discarding sub-pixel and degenerate
triangles before rasterization.

- **Derived from:** [F-3.1.3](../../features/geometry-world/meshlet-pipeline.md)
- **Rationale:** Two-level cull hierarchy (cluster then triangle) minimizes wasted rasterizer work across
  massive open-world scenes containing millions of meshlets.
- **Verification:** Render a dense scene and verify via GPU counters that culled meshlets produce zero mesh
  shader invocations, and sub-pixel triangles are absent from the rasterized output.

## R-3.1.4 Mesh Shader Pipeline with Indirect Draw Fallback

The engine **SHALL** rasterize meshlets via task/mesh shaders when supported, and fall back to a compute
compaction pass feeding multi-draw-indirect calls on hardware lacking mesh shader support, preserving
GPU-driven culling benefits in both paths.

- **Derived from:** [F-3.1.4](../../features/geometry-world/meshlet-pipeline.md)
- **Rationale:** Mesh shaders offer optimal throughput on modern GPUs, but an indirect draw fallback ensures
  broad hardware compatibility without sacrificing GPU-driven culling.
- **Verification:** Run the same test scene on mesh-shader-capable and non-capable hardware; confirm visual
  output is identical and GPU-driven culling remains active in both paths.

## R-3.1.5 Screen-Space Error LOD Selection

The engine **SHALL** select the coarsest meshlet hierarchy level whose projected screen-space error is below a
configurable pixel-error threshold, with dithered crossfade transitions between LOD levels.

- **Derived from:** [F-3.1.5](../../features/geometry-world/meshlet-pipeline.md)
- **Rationale:** Per-meshlet-group LOD selection with crossfade prevents visible LOD pop artifacts and allows
  smooth quality gradation across a single mesh.
- **Verification:** Dolly the camera toward a mesh and confirm triangle counts decrease with distance, the
  pixel-error threshold is never exceeded, and no visible pop occurs during LOD transitions.

## R-3.1.6 On-Demand Meshlet Page Streaming

The engine **SHALL** organize meshlet data into fixed-size pages, stream them from disk on demand via
platform-native async I/O, and prioritize page requests by screen-space contribution.

- **Derived from:** [F-3.1.6](../../features/geometry-world/meshlet-pipeline.md)
- **Rationale:** Page-based streaming with GPU feedback enables virtually unlimited world geometry without
  requiring the entire dataset in memory.
- **Verification:** Load a scene exceeding VRAM capacity and confirm pages stream in priority order, no
  visual corruption occurs during streaming, and async I/O uses the platform-native API (IOCP / GCD /
  io_uring).

## R-3.1.7 Visibility Buffer Rendering

The engine **SHALL** write a 64-bit visibility buffer (triangle ID + instance ID) via atomic writes during
meshlet rasterization and defer all material evaluation to a subsequent fullscreen compute pass that fetches
vertex attributes only for visible pixels.

- **Derived from:** [F-3.1.7](../../features/geometry-world/meshlet-pipeline.md)
- **Rationale:** Visibility buffer rendering eliminates redundant shading, reduces bandwidth versus a
  traditional G-buffer, and scales to millions of unique materials.
- **Verification:** Render a multi-material scene and confirm each visible pixel resolves to the correct
  triangle/instance, bandwidth is lower than an equivalent G-buffer path, and no shading occurs for
  occluded fragments.
