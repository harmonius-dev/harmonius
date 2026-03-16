# R-3.1 — Meshlet Pipeline

## Meshlet Generation

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-3.1.1 | The engine **SHALL** decompose all imported geometry into meshlets of ~64 vertices and ~124 triangles, each annotated with a bounding sphere, normal cone, and screen-space error bound, and organize them into a DAG whose every cut yields a watertight mesh. | [F-3.1.1](../../features/geometry-world/meshlet-pipeline.md) | Meshlet DAGs enable fine-grained GPU-driven LOD selection and culling as an atomic unit, eliminating whole-object LOD pops and supporting continuous quality gradation. | Import a reference mesh and confirm meshlet sizes stay within bounds, every DAG cut produces a watertight mesh (no T-junctions or holes), and bounding metadata is present on each node. |

## GPU-Driven Culling

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-3.1.2 | The engine **SHALL** implement a two-phase GPU-driven occlusion culling pipeline that tests instances against the previous frame's hierarchical depth buffer in phase one and re-projects newly unoccluded geometry against an updated HZB in phase two. | [F-3.1.2](../../features/geometry-world/meshlet-pipeline.md) | Two-phase culling eliminates one-frame-late disocclusion artifacts common in single-pass schemes, which is critical for dense MMO scenes with many dynamic occluders. | Render a scene where a large occluder moves to reveal geometry behind it and confirm no one-frame pop-in artifacts occur; measure that occluded instances produce zero rasterizer invocations. |
| R-3.1.3 | The engine **SHALL** perform per-meshlet frustum, backface-cone, and occlusion culling in task shaders, then per-triangle backface and small-triangle culling in mesh shaders, discarding sub-pixel and degenerate triangles before rasterization. | [F-3.1.3](../../features/geometry-world/meshlet-pipeline.md) | Two-level cull hierarchy (cluster then triangle) minimizes wasted rasterizer work across massive open-world scenes containing millions of meshlets. | Render a dense scene and verify via GPU counters that culled meshlets produce zero mesh shader invocations, and sub-pixel triangles are absent from the rasterized output. |

## Mesh Shaders / Indirect Draw Fallback

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-3.1.4 | The engine **SHALL** rasterize meshlets via task/mesh shaders when supported, and fall back to a compute compaction pass feeding multi-draw-indirect calls on hardware lacking mesh shader support, preserving GPU-driven culling benefits in both paths. | [F-3.1.4](../../features/geometry-world/meshlet-pipeline.md) | Mesh shaders offer optimal throughput on modern GPUs, but an indirect draw fallback ensures broad hardware compatibility without sacrificing GPU-driven culling. | Run the same test scene on mesh-shader-capable and non-capable hardware; confirm visual output is identical and GPU-driven culling remains active in both paths. |

## Meshlet LOD

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-3.1.5 | The engine **SHALL** select the coarsest meshlet hierarchy level whose projected screen-space error is below a configurable pixel-error threshold, with dithered crossfade transitions between LOD levels. | [F-3.1.5](../../features/geometry-world/meshlet-pipeline.md) | Per-meshlet-group LOD selection with crossfade prevents visible LOD pop artifacts and allows smooth quality gradation across a single mesh. | Dolly the camera toward a mesh and confirm triangle counts decrease with distance, the pixel-error threshold is never exceeded, and no visible pop occurs during LOD transitions. |

## Meshlet Streaming

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-3.1.6 | The engine **SHALL** organize meshlet data into fixed-size pages, stream them from disk on demand via platform-native async I/O, and prioritize page requests by screen-space contribution. | [F-3.1.6](../../features/geometry-world/meshlet-pipeline.md) | Page-based streaming with GPU feedback enables virtually unlimited world geometry without requiring the entire dataset in memory. | Load a scene exceeding VRAM capacity and confirm pages stream in priority order, no visual corruption occurs during streaming, and async I/O uses the platform-native API (IOCP / GCD / io_uring). |

## Visibility Buffer

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-3.1.7 | The engine **SHALL** write a 64-bit visibility buffer (triangle ID + instance ID) via atomic writes during meshlet rasterization and defer all material evaluation to a subsequent fullscreen compute pass that fetches vertex attributes only for visible pixels. | [F-3.1.7](../../features/geometry-world/meshlet-pipeline.md) | Visibility buffer rendering eliminates redundant shading, reduces bandwidth versus a traditional G-buffer, and scales to millions of unique materials. | Render a multi-material scene and confirm each visible pixel resolves to the correct triangle/instance, bandwidth is lower than an equivalent G-buffer path, and no shading occurs for occluded fragments. |
