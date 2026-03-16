# 9.2 — Morph Targets

## Blend Shapes

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.2.1 | GPU Blend Shape Accumulation | Accumulates weighted morph target deltas (position and normal offsets) on the GPU via compute shaders, applied before skeletal skinning in the deformation pipeline. Supports an arbitrary number of active targets per mesh with sparse delta storage to minimize memory bandwidth. | R-9.2.1 | F-9.1.1 | Active morph target count per mesh scales per tier: mobile 8-16, Switch 16-32, desktop 64+. Sparse delta storage on all platforms. |

## Corrective Blend Shapes

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.2.2 | Corrective Blend Shapes | Automatically activates corrective morph targets driven by joint angles to fix deformation artifacts caused by extreme poses. Corrective shapes are authored as difference-from-expected deltas and triggered by combination rules (e.g., elbow bend past 120 degrees). | R-9.2.2 | F-9.2.1, F-9.1.1 | Corrective blend shapes may be disabled on mobile under budget pressure (uses base skinning only for non-hero characters). |

## Facial Animation

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.2.3 | Facial Animation System | Drives facial blend shapes through a standardized set of face action units compatible with performance capture data. Supports both curve-driven keyframe animation and real-time parameter input for lip sync and expression blending. Enables unique NPC expressions across hundreds of visible characters in MMO city hubs. | R-9.2.3 | F-9.2.1 | Facial blend shape count per face: mobile 16-24 action units, desktop 52+ (ARKit-compatible full set). Facial animation disabled for distant NPCs on mobile. |

## Per-Vertex Animation

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.2.4 | Per-Vertex Animation Textures | Bakes complex deformations (fluid surfaces, tentacles, foliage sway) into vertex animation textures (VATs) sampled in the vertex shader. Each frame of the animation is stored as a texel row, enabling GPU-only playback with zero CPU cost. | R-9.2.4 | None | VAT texture resolution scales per tier: mobile uses half-res VAT. VAT playback is GPU-only and lightweight on all platforms. |

## Morph Target Streaming

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.2.5 | Morph Target Streaming | Streams morph target delta buffers from disk on demand using async I/O, loading only the targets needed for currently visible characters. Evicts unused targets under memory pressure with an LRU policy. Critical for MMO-scale character customization. | R-9.2.5 | F-9.2.1 | Uses IOCP on Windows, GCD on macOS, io_uring on Linux. |
