# 9.2 — Morph Targets

## Blend Shapes

| ID      | Feature                      | Requirements |
|---------|------------------------------|--------------|
| F-9.2.1 | GPU Blend Shape Accumulation | R-9.2.1      |

1. **F-9.2.1** — Accumulates weighted morph target deltas (position and normal offsets) on the GPU
   via compute shaders, applied before skeletal skinning in the deformation pipeline. Supports an
   arbitrary number of active targets per mesh with sparse delta storage to minimize memory
   bandwidth.
   - **Deps:** F-9.1.1
   - **Platform:** Active morph target count per mesh scales per tier: mobile 8-16, Switch 16-32,
     desktop 64+. Sparse delta storage on all platforms.

## Corrective Blend Shapes

| ID      | Feature                 | Requirements |
|---------|-------------------------|--------------|
| F-9.2.2 | Corrective Blend Shapes | R-9.2.2      |

1. **F-9.2.2** — Automatically activates corrective morph targets driven by joint angles to fix
   deformation artifacts caused by extreme poses. Corrective shapes are authored as
   difference-from-expected deltas and triggered by combination rules (e.g., elbow bend past 120
   degrees).
   - **Deps:** F-9.2.1, F-9.1.1
   - **Platform:** Corrective blend shapes may be disabled on mobile under budget pressure (uses
     base skinning only for non-hero characters).

## Facial Animation

| ID      | Feature                 | Requirements |
|---------|-------------------------|--------------|
| F-9.2.3 | Facial Animation System | R-9.2.3      |

1. **F-9.2.3** — Drives facial blend shapes through a standardized set of face action units
   compatible with performance capture data. Supports both curve-driven keyframe animation and
   real-time parameter input for lip sync and expression blending. Enables unique NPC expressions
   across hundreds of visible characters in MMO city hubs.
   - **Deps:** F-9.2.1
   - **Platform:** Facial blend shape count per face: mobile 16-24 action units, desktop 52+
     (ARKit-compatible full set). Facial animation disabled for distant NPCs on mobile.

## Per-Vertex Animation

| ID      | Feature                       | Requirements |
|---------|-------------------------------|--------------|
| F-9.2.4 | Per-Vertex Animation Textures | R-9.2.4      |

1. **F-9.2.4** — Bakes complex deformations (fluid surfaces, tentacles, foliage sway) into vertex
   animation textures (VATs) sampled in the vertex shader. Each frame of the animation is stored as
   a texel row, enabling GPU-only playback with zero CPU cost.
   - **Platform:** VAT texture resolution scales per tier: mobile uses half-res VAT. VAT playback is
     GPU-only and lightweight on all platforms.

## Morph Target Streaming

| ID      | Feature                | Requirements |
|---------|------------------------|--------------|
| F-9.2.5 | Morph Target Streaming | R-9.2.5      |

1. **F-9.2.5** — Streams morph target delta buffers from disk on demand using async I/O, loading
   only the targets needed for currently visible characters. Evicts unused targets under memory
   pressure with an LRU policy. Critical for MMO-scale character customization.
   - **Deps:** F-9.2.1
   - **Platform:** Uses IOCP on Windows, GCD on macOS, io_uring on Linux.
