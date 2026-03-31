# 2.10 — Scene Rendering Pipeline

## ECS-to-Renderer Bridge

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-2.10.1 | Render Proxy Extraction                 |
| F-2.10.2 | Render Component System                 |
| F-2.10.3 | Change Detection and Incremental Update |

1. **F-2.10.1** — Each frame, visible ECS entities with renderable components are extracted into a
   renderer-owned snapshot decoupled from the ECS world. Extraction runs on a dedicated thread,
   reading component data via immutable queries so the simulation can advance concurrently with
   rendering.
   - **Platform:** All platforms: full quality. Mobile: extraction thread shares cores with fewer
     available threads; budget for lower entity counts.
2. **F-2.10.2** — Lightweight render-side proxy components (mesh, material, transform, bounds)
   stored in a flat structure-of-arrays layout optimized for GPU upload and cache-friendly
   iteration. Proxies hold only the data the GPU pipeline needs, discarding simulation-only fields.
   - **Deps:** F-2.10.1
   - **Platform:** All platforms: full quality. SoA layout benefits cache performance uniformly
     across all hardware tiers.
3. **F-2.10.3** — Dirty flags on ECS components drive incremental proxy updates. Only entities whose
   transform, material, or mesh changed since the previous extraction are re-uploaded, reducing
   per-frame CPU and bus bandwidth from O(N) to O(changed) at MMO entity counts.
   - **Deps:** F-2.10.1, F-2.10.2
   - **Platform:** All platforms: full quality. Especially critical on mobile where bus bandwidth is
     limited (LPDDR4/5 shared between CPU and GPU).

## View Setup and Camera

| ID       | Feature               |
|----------|-----------------------|
| F-2.10.4 | View and Camera Setup |
| F-2.10.5 | Multi-View Rendering  |

1. **F-2.10.4** — Each active view (main camera, shadow cascades, reflection probes, split-screen
   players, VR eyes) is registered with its projection matrix, view matrix, viewport rect, and
   quality tier. Views are first-class inputs to the render graph's multi-view execution.
   - **Deps:** F-2.2.9
2. **F-2.10.5** — Simultaneous rendering of many independent views from a single extracted scene
   snapshot. Shadow cascade views, reflection probe views, and player camera views share the same
   proxy data but execute separate culling and draw passes, scaling to dozens of concurrent views
   for MMO scenes with many shadow-casting lights.
   - **Deps:** F-2.10.4, F-2.2.9
   - **Platform:** VR stereo views may use single-pass instanced rendering where the backend
     supports viewport instancing.

## Draw List Construction and Batching

| ID       | Feature                     |
|----------|-----------------------------|
| F-2.10.6 | Draw List Construction      |
| F-2.10.7 | GPU-Driven Batch Compaction |

1. **F-2.10.6** — Per-view draw lists are assembled by iterating extracted proxies and emitting draw
   commands keyed by material, mesh, and render state. Sort keys encode pipeline state, material ID,
   and depth to minimize state changes during submission.
   - **Deps:** F-2.10.2, F-2.10.4
   - **Platform:** All platforms: full quality. Sort key order is critical on mobile where state
     changes incur higher driver overhead.
2. **F-2.10.7** — After CPU-side draw list construction, a GPU compute pass compacts surviving draws
   (post-cull) into contiguous indirect draw buffers grouped by material. This eliminates per-draw
   CPU dispatch overhead and enables rendering hundreds of thousands of meshlet instances with
   minimal draw calls.
   - **Deps:** F-2.10.6, F-2.1.7
   - **Platform:** Mobile: requires indirect draw support (Vulkan 1.1+ / Metal GPU family 3+).
     Compaction buffer sized for lower draw counts. Switch: full indirect compaction.
     Desktop/High-end: full quality with large compaction buffers.

## Material Parameter Binding

| ID       | Feature                    |
|----------|----------------------------|
| F-2.10.8 | Material Parameter Binding |

1. **F-2.10.8** — Per-draw material parameters (textures, constants, samplers) are bound via
   bindless descriptor indices written into a per-instance data buffer. The shader reads parameters
   by index, eliminating descriptor set switching between draws and enabling material-agnostic
   batching.
   - **Deps:** F-2.10.6, F-2.1.7
   - **Platform:** Metal uses argument buffers. D3D12 uses root descriptor tables or bindless SRV
     heaps. Vulkan uses descriptor indexing with `VK_EXT_descriptor_indexing`.

## Debug Visualization

| ID        | Feature                        |
|-----------|--------------------------------|
| F-2.10.9  | Debug Visualization and Gizmos |
| F-2.10.10 | Buffer Visualization Modes     |

1. **F-2.10.9** — Immediate-mode debug drawing API for lines, wireframe shapes, text labels, and
   custom gizmos rendered as an overlay pass. Debug primitives are batched into a single vertex
   buffer per frame and drawn after the final scene composite. Disabled in shipping builds via
   compile-time gating.
   - **Deps:** F-2.2.1
   - **Platform:** All platforms: debug-only feature stripped from shipping builds. No platform
     scaling needed.
2. **F-2.10.10** — Diagnostic render modes that replace final output with intermediate buffer
   contents: depth, normals, motion vectors, roughness, metallic, base color, ambient occlusion,
   light complexity, and overdraw heat maps. Selectable at runtime via a debug menu.
   - **Deps:** F-2.10.9
   - **Platform:** All platforms: debug-only feature stripped from shipping builds. Mobile: G-buffer
     visualization unavailable when deferred path is disabled.
