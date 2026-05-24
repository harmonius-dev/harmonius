# Rendering Pipeline

How geometry flows from the simulation through extraction, sorting, state management, and draw
submission to the graphics card.

## What it covers

- ECS-to-renderer bridge: extracting visible entities into a renderer-owned snapshot each frame.
- Render proxies: flat, array-of-structures layout with incremental dirty-flag updates.
- Multi-view setup: projection, viewport, quality tier, supporting split-screen, VR, shadows, and
  probe rendering from a single snapshot.
- Draw list assembly: per-view sorting by material, mesh, render state with GPU batch compaction.
- Bindless descriptor indexing: per-instance material parameter indices eliminating descriptor set
  switches.
- Render layers: 32-bit bitmask filtering entities, cameras, and lights independently.
- Transform interpolation: smooth rendering when simulation and render rates differ.
- Per-frame ring buffering: GPU resource indexing by frame-in-flight preventing CPU/GPU
  synchronization stalls.
- Debug visualization: immediate-mode draw API and diagnostic modes (depth, normals, roughness,
  overdraw).

## Concepts

### Extraction and Snapshot

The renderer extracts visible entities into its own snapshot on a dedicated thread using immutable
queries. This decouples extraction from the simulation loop, enabling concurrent CPU work. Render
proxies store only GPU-needed data in SoA layout; dirty flags track changes per proxy, reducing
per-frame bandwidth from O(N) to O(changed).

### View Setup and Multi-Render Targeting

Each active view registers projection matrix, view matrix, viewport rect, and quality tier. Multiple
views can render from a single extracted snapshot, supporting split-screen displays, VR stereo
pairs, shadow map rendering, and reflection probe capture all sharing the same proxy data.

### Draw List Compilation and Batching

Per-view draw lists are sorted by material, mesh, and render state. The renderer compacts batches on
the GPU, producing contiguous indirect draw buffers grouped by material. Bindless descriptor
indexing binds per-draw material parameters via per-instance buffer indices, eliminating expensive
descriptor set switching and enabling thousands of material-diverse draws at scale.

### Render Layer Filtering

A 32-bit bitmask per entity, camera, and light controls visibility. An entity is visible to a
camera only when their masks overlap. The same layer filtering applies to shadows and probe
rendering, enabling selective visibility for editor overlays, minimap views, and debug visualizations
without scene duplication.

### Temporal Coherence

Transform interpolation uses an alpha factor for smooth rendering when frame rates differ from the
fixed timestep. Per-frame ring buffering indexes GPU resources by frame-in-flight count (typically
2–3), allowing the CPU to write next-frame data while the GPU consumes the current frame without
pipeline stalls.

## How it fits

- See [lighting-and-materials](./lighting-and-materials.md) for per-fragment cost and light
  culling.
- See [anti-aliasing-and-upscaling](./anti-aliasing-and-upscaling.md) for temporal anti-aliasing
  and sample reuse across frames.
- See [effects-and-styles](./effects-and-styles.md) for post-processing, bloom, and deferred
  decals.
- Integrates with [../core-runtime/simulation-loop.md](../core-runtime/simulation-loop.md) for
  frame phases and determinism.
- Integrates with [../core-runtime/world-and-entities.md](../core-runtime/world-and-entities.md)
  for entity and component queries.
