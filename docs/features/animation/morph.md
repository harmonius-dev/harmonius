# 9.2 — Morph Targets

## Blend Shapes

### F-9.2.1 GPU Blend Shape Accumulation

Accumulates weighted morph target deltas (position and normal offsets) on the GPU via compute
shaders, applied before skeletal skinning in the deformation pipeline. Supports an arbitrary number
of active targets per mesh with sparse delta storage to minimize memory bandwidth.

- **Requirements:** R-9.2.1
- **Dependencies:** F-9.1.1
- **Platform notes:** Active morph target count per mesh scales per tier: mobile 8-16, Switch 16-32,
  desktop 64+. Sparse delta storage on all platforms.

## Corrective Blend Shapes

### F-9.2.2 Corrective Blend Shapes

Automatically activates corrective morph targets driven by joint angles to fix deformation artifacts
caused by extreme poses. Corrective shapes are authored as difference-from-expected deltas and
triggered by combination rules (e.g., elbow bend past 120 degrees), enabling high-fidelity character
deformation without manual per-frame intervention.

- **Requirements:** R-9.2.2
- **Dependencies:** F-9.2.1, F-9.1.1
- **Platform notes:** Corrective blend shapes may be disabled on mobile under budget pressure (uses
  base skinning only for non-hero characters).

## Facial Animation

### F-9.2.3 Facial Animation System

Drives facial blend shapes through a standardized set of face action units compatible with
performance capture data. Supports both curve-driven keyframe animation and real-time parameter
input for lip sync and expression blending. Enables unique NPC expressions across hundreds of
visible characters in MMO city hubs.

- **Requirements:** R-9.2.3
- **Dependencies:** F-9.2.1
- **Platform notes:** Facial blend shape count per face: mobile 16-24 action units, desktop 52+
  (ARKit-compatible full set). Facial animation disabled for distant NPCs on mobile.

## Per-Vertex Animation

### F-9.2.4 Per-Vertex Animation Textures

Bakes complex deformations (fluid surfaces, tentacles, foliage sway) into vertex animation textures
(VATs) sampled in the vertex shader. Each frame of the animation is stored as a texel row, enabling
GPU-only playback with zero CPU cost. Ideal for decorative environmental animations and distant
crowd characters.

- **Requirements:** R-9.2.4
- **Dependencies:** None
- **Platform notes:** VAT texture resolution scales per tier: mobile uses half-res VAT. VAT playback
  is GPU-only and lightweight on all platforms.

## Morph Target Streaming

### F-9.2.5 Morph Target Streaming

Streams morph target delta buffers from disk on demand using async I/O, loading only the targets
needed for currently visible characters. Evicts unused targets under memory pressure with an LRU
policy. Critical for MMO-scale character customization where each player may have unique facial
morph sets that cannot all reside in memory simultaneously.

- **Requirements:** R-9.2.5
- **Dependencies:** F-9.2.1
- **Platform notes:** Uses IOCP on Windows, GCD on macOS, io_uring on Linux
