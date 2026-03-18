# R-2.10 — Scene Rendering Pipeline Requirements

## ECS-to-Renderer Bridge

| ID       | Derived From                                                     |
|----------|------------------------------------------------------------------|
| R-2.10.1 | [F-2.10.1](../../features/rendering/scene-rendering-pipeline.md) |
| R-2.10.2 | [F-2.10.2](../../features/rendering/scene-rendering-pipeline.md) |
| R-2.10.3 | [F-2.10.3](../../features/rendering/scene-rendering-pipeline.md) |

1. **R-2.10.1** — The engine **SHALL** extract visible ECS entities with renderable components into
   a renderer-owned snapshot each frame, decoupled from the ECS world, running extraction on a
   dedicated thread using immutable queries so that simulation and rendering advance concurrently
   without data races.
   - **Rationale:** Decoupled extraction allows the simulation to advance independently of
     rendering, enabling pipelined CPU parallelism.
   - **Verification:** Profile a frame and verify extraction runs on a separate thread from
     simulation; confirm the renderer's snapshot is immutable during rendering; verify no data race
     warnings under thread sanitizer.
2. **R-2.10.2** — The engine **SHALL** store renderer-side proxy components (mesh, material,
   transform, bounds) in a flat structure-of-arrays layout optimized for GPU upload and
   cache-friendly iteration, containing only the data required by the GPU pipeline and discarding
   simulation-only fields.
   - **Rationale:** SoA proxy layout maximizes GPU upload bandwidth and CPU cache utilization during
     draw list construction.
   - **Verification:** Measure cache miss rates during proxy iteration and confirm they are lower
     than an equivalent array-of-structures layout; verify proxies contain no simulation-only fields
     by auditing the proxy struct definition.
3. **R-2.10.3** — The engine **SHALL** use dirty flags on ECS components to drive incremental proxy
   updates, re-uploading only entities whose transform, material, or mesh changed since the previous
   extraction, reducing per-frame CPU and bus bandwidth from O(N) to O(changed).
   - **Rationale:** Full re-extraction every frame is prohibitively expensive at high entity counts;
     incremental updates bound cost by the number of changes.
   - **Verification:** Render a scene with 100,000 entities and modify 100 of them; measure the
     extraction upload size and confirm it is proportional to 100 entities, not 100,000; verify
     dirty flags are cleared after extraction.

## View Setup and Multi-View

| ID       | Derived From                                                     |
|----------|------------------------------------------------------------------|
| R-2.10.4 | [F-2.10.4](../../features/rendering/scene-rendering-pipeline.md) |
| R-2.10.5 | [F-2.10.5](../../features/rendering/scene-rendering-pipeline.md) |

1. **R-2.10.4** — The engine **SHALL** register each active view (main camera, shadow cascades,
   reflection probes, split-screen players, VR eyes) with its projection matrix, view matrix,
   viewport rect, and quality tier as first-class inputs to the render graph's multi-view execution.
   - **Rationale:** First-class view registration enables the render graph to schedule and execute
     per-view passes independently.
   - **Verification:** Register a main camera, a shadow cascade, and a reflection probe view; verify
     each view's matrices, viewport, and quality tier are correctly passed to the render graph;
     confirm all three views produce independent render output.
2. **R-2.10.5** — The engine **SHALL** render multiple independent views simultaneously from a
   single extracted scene snapshot, with separate culling and draw passes per view, scaling to
   dozens of concurrent views (shadow cascades, reflection probes, player cameras) for scenes with
   many shadow-casting lights.
   - **Rationale:** Multi-view rendering from a shared snapshot avoids redundant extraction and
     enables efficient rendering of shadow maps, reflections, and split-screen cameras.
   - **Verification:** Configure 20+ concurrent views (4 shadow cascades, 6 reflection probes,
     multiple player cameras) and verify each produces correct independent output from the same
     scene data; confirm VR stereo views use single-pass instanced rendering where the backend
     supports viewport instancing.

## Draw Submission and Batching

| ID       | Derived From                                                     |
|----------|------------------------------------------------------------------|
| R-2.10.6 | [F-2.10.6](../../features/rendering/scene-rendering-pipeline.md) |
| R-2.10.7 | [F-2.10.7](../../features/rendering/scene-rendering-pipeline.md) |
| R-2.10.8 | [F-2.10.8](../../features/rendering/scene-rendering-pipeline.md) |

1. **R-2.10.6** — The engine **SHALL** assemble per-view draw lists by iterating extracted proxies
   and emitting draw commands keyed by material, mesh, and render state, with sort keys encoding
   pipeline state, material ID, and depth to minimize state changes during submission.
   - **Rationale:** Sorted draw lists minimize GPU state switching overhead, which is a primary
     bottleneck in draw-call-heavy scenes.
   - **Verification:** Render a scene with diverse materials and verify draw commands are sorted by
     sort key; measure GPU state change count and confirm it is lower than an unsorted submission of
     the same draws.
2. **R-2.10.7** — The engine **SHALL** compact post-cull draw commands into contiguous indirect draw
   buffers grouped by material via a GPU compute pass, eliminating per-draw CPU dispatch overhead
   and enabling rendering of hundreds of thousands of meshlet instances with minimal draw calls.
   - **Rationale:** GPU-driven compaction removes CPU bottlenecks from per-draw dispatch, enabling
     massive scene complexity.
   - **Verification:** Render a scene with 500,000+ meshlet instances and verify the compute
     compaction pass produces contiguous indirect draw buffers; confirm the CPU issues fewer draw
     calls than the number of visible instances; measure and confirm CPU draw submission time does
     not scale linearly with instance count.
3. **R-2.10.8** — The engine **SHALL** bind per-draw material parameters (textures, constants,
   samplers) via bindless descriptor indices written into a per-instance data buffer, with the
   shader reading parameters by index, eliminating descriptor set switching between draws and
   enabling material-agnostic batching.
   - **Rationale:** Bindless material binding removes descriptor set switching as a batching
     barrier, enabling draws with different materials to share the same pipeline state.
   - **Verification:** Render a scene with 100+ distinct materials and verify no descriptor set
     rebinding occurs between draws; confirm Metal uses argument buffers, D3D12 uses bindless SRV
     heaps or root descriptor tables, and Vulkan uses descriptor indexing.

## Debug and Diagnostics

| ID        | Derived From                                                      |
|-----------|-------------------------------------------------------------------|
| R-2.10.9  | [F-2.10.9](../../features/rendering/scene-rendering-pipeline.md)  |
| R-2.10.10 | [F-2.10.10](../../features/rendering/scene-rendering-pipeline.md) |

1. **R-2.10.9** — The engine **SHALL** provide an immediate-mode debug drawing API for lines,
   wireframe shapes, text labels, and custom gizmos, batched into a single vertex buffer per frame
   and rendered as an overlay pass after the final scene composite, disabled in shipping builds via
   compile-time gating.
   - **Rationale:** Debug visualization is essential for development-time debugging of spatial data,
     physics, and gameplay systems.
   - **Verification:** Issue debug draw calls for lines, boxes, and text labels and verify they
     appear as an overlay on the final image; compile a shipping build and confirm no debug draw
     code or GPU overhead is present.
2. **R-2.10.10** — The engine **SHALL** provide diagnostic render modes that replace final output
   with intermediate buffer contents (depth, normals, motion vectors, roughness, metallic, base
   color, ambient occlusion, light complexity, overdraw heat maps), selectable at runtime via a
   debug menu.
   - **Rationale:** Buffer visualization enables rapid diagnosis of rendering issues by isolating
     individual G-buffer and intermediate buffer contributions.
   - **Verification:** Select each buffer visualization mode at runtime and verify the output
     displays the correct buffer contents; confirm at least depth, normals, motion vectors,
     roughness, metallic, base color, AO, light complexity, and overdraw modes are available.

## Non-Functional Requirements

| ID         |
|------------|
| NFR-2.10.1 |
| NFR-2.10.2 |
| NFR-2.10.3 |

1. **NFR-2.10.1** — Render proxy extraction **SHALL** complete in under 2.0 ms for a scene with
   100,000 entities, and incremental updates (O(changed)) **SHALL** complete in under 0.5 ms when
   fewer than 1% of entities have changed.
   - **Rationale:** Extraction runs on the critical path between simulation and rendering; exceeding
     this budget would increase input-to-display latency.
   - **Verification:** Profile extraction with 100,000 entities and 100% change rate (first frame).
     Verify completion under 2.0 ms. Profile with 0.1% change rate and verify completion under 0.5
     ms.
2. **NFR-2.10.2** — Draw list construction **SHALL** process at least 500,000 proxy entries per
   millisecond on a single CPU core, with sort key computation and material grouping included.
   - **Rationale:** Draw list assembly is CPU-bound work that must keep pace with GPU consumption to
     prevent pipeline bubbles.
   - **Verification:** Measure draw list construction throughput with 500,000 proxy entries and
     verify completion under 1.0 ms on a single core.
3. **NFR-2.10.3** — Debug visualization and gizmo rendering **SHALL** produce exactly zero CPU
   overhead, zero GPU overhead, and zero binary size contribution in shipping (release) builds via
   compile-time gating.
   - **Rationale:** Debug features must be cost-free in player-facing builds.
   - **Verification:** Compile a shipping build and verify no debug draw symbols, buffers, or GPU
     passes are present in the binary or render graph.
