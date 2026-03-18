# Core Rendering Test Cases

Companion test cases for [core-rendering.md](core-rendering.md).

## Unit Tests

### TC-2.3.7.1 Sort Key Opaque Ordering

| # | Requirement |
|---|-------------|
| 1 | R-2.3.7     |

1. **#1** — Opaque keys: pipeline=1, mat=2, depth=0.5; pipeline=1, mat=1, depth=0.8
   - **Expected:** Keys sort by pipeline, then material, then front-to-back depth

### TC-2.3.7.2 Sort Key Transparent Ordering

| # | Requirement      |
|---|------------------|
| 1 | R-2.3.7, R-2.4.5 |

1. **#1** — Transparent keys: depth=0.3, depth=0.8
   - **Expected:** depth=0.8 sorts before depth=0.3 (back-to-front)

### TC-2.3.6.1 Perspective Reverse-Z Near Far

| # | Requirement |
|---|-------------|
| 1 | R-2.3.6     |
| 2 | R-2.3.6     |

1. **#1** — fov_y=PI/4, aspect=16/9, near=0.1, far=1000.0
   - **Expected:** Near plane maps to depth 1.0, far maps to 0.0
2. **#2** — fov_y=PI/4, aspect=16/9, near=0.1, far=None
   - **Expected:** Valid matrix; infinite far plane

### TC-2.3.5.1 Orthographic Reverse-Z

| # | Requirement |
|---|-------------|
| 1 | R-2.3.5     |

1. **#1** — left=-10, right=10, bottom=-10, top=10, near=0.1, far=100
   - **Expected:** Linear depth in [0,1] with reverse mapping

### TC-2.3.2.1 Frustum Plane Extraction

| # | Requirement |
|---|-------------|
| 1 | R-2.3.2     |
| 2 | R-2.3.2     |

1. **#1** — Known view-projection matrix, point inside frustum
   - **Expected:** Point classified as inside
2. **#2** — Known view-projection matrix, point outside frustum
   - **Expected:** Point classified as outside

### TC-2.3.7.3 Render Layer Mask Filtering

| # | Requirement |
|---|-------------|
| 1 | R-2.3.7     |
| 2 | R-2.3.7     |

1. **#1** — Entity layer=0x02, view layer=0x01
   - **Expected:** Entity excluded (masks do not intersect)
2. **#2** — Entity layer=0x03, view layer=0x01
   - **Expected:** Entity included (masks intersect)

### TC-2.3.13.1 Instance Flags Packing

| # | Requirement |
|---|-------------|
| 1 | R-2.3.13    |
| 2 | R-2.3.13    |

1. **#1** — InstanceFlags: CAST_SHADOWS, TWO_SIDED
   - **Expected:** Round-trip through u32 preserves all flags
2. **#2** — TWO_SIDED flag set
   - **Expected:** Backface cull skipped for this instance

### TC-2.3.11.1 DRS Convergence

| # | Requirement |
|---|-------------|
| 1 | NFR-2.3.2   |

1. **#1** — Step-change GPU load from 8 ms to 16 ms, target=16 ms
   - **Expected:** Scale converges within 5 frames, oscillation < 5%

### TC-2.3.11.2 DRS Clamp Bounds

| # | Requirement |
|---|-------------|
| 1 | R-2.3.11    |

1. **#1** — min_scale=0.5, max_scale=1.0, extreme load
   - **Expected:** Scale stays in [0.5, 1.0]

### TC-2.4.3.1 Material Permutation Cache

| # | Requirement |
|---|-------------|
| 1 | R-2.4.3     |

1. **#1** — Same PermutationKey queried twice
   - **Expected:** Same cached PipelineState returned both times

### TC-2.10.8.1 Bindless Alloc Free Reuse

| # | Requirement |
|---|-------------|
| 1 | R-2.10.8    |

1. **#1** — Allocate index, free it, allocate again
   - **Expected:** Second allocation returns the freed index

### TC-2.3.7.4 Batch Compaction Count

| # | Requirement |
|---|-------------|
| 1 | NFR-2.3.3   |

1. **#1** — 10,000 instances across 10 materials
   - **Expected:** Exactly 10 indirect draw calls

### TC-2.3.6.2 View Uniform Struct Size

| # | Requirement |
|---|-------------|
| 1 | R-2.3.6     |

1. **#1** — `ViewUniform` struct
   - **Expected:** Size matches expected GPU layout (std140/std430)

### TC-2.3.1.1 Light GPU Struct Alignment

| # | Requirement |
|---|-------------|
| 1 | R-2.3.1     |

1. **#1** — `LightGpu` struct
   - **Expected:** 16-byte aligned, matches HLSL `cbuffer` layout

### TC-2.3.3.1 Two-Sided Skips Backface Cull

| # | Requirement |
|---|-------------|
| 1 | R-2.3.3     |

1. **#1** — Meshlet flagged two-sided, facing away from camera
   - **Expected:** Meshlet not culled by normal cone test

### TC-2.3.13.2 Alpha Cutout in Shadow Pass

| # | Requirement |
|---|-------------|
| 1 | R-2.3.13    |

1. **#1** — Alpha-cutout geometry in shadow view
   - **Expected:** Shadow draw commands generated

### TC-2.10.2.1 Proxy Insert Remove Reuse

| # | Requirement |
|---|-------------|
| 1 | R-2.10.2    |

1. **#1** — Insert entity A (index=0), remove A, insert B
   - **Expected:** Entity B gets index=0 (recycled)

### TC-2.10.3.1 Proxy Dirty Flags Cleared

| # | Requirement |
|---|-------------|
| 1 | R-2.10.3    |

1. **#1** — Set DIRTY_TRANSFORM, call `clear_dirty_flags()`
   - **Expected:** `is_dirty()` returns false

### TC-2.10.3.2 Proxy Incremental Update

| # | Requirement |
|---|-------------|
| 1 | R-2.10.3    |

1. **#1** — Update transform only on proxy
   - **Expected:** DIRTY_TRANSFORM set; mesh_id and material_id unchanged

## Integration Tests

### TC-2.3.1.I1 Forward Deferred Parity

| # | Requirement |
|---|-------------|
| 1 | R-2.3.1     |

1. **#1** — Same scene, forward+ and deferred paths
   - **Expected:** Pixel-identical lighting within FP tolerance

### TC-2.3.2.I1 Frustum Cull GPU vs CPU

| # | Requirement |
|---|-------------|
| 1 | R-2.3.2     |

1. **#1** — 10,000 meshlets, 1-degree camera sweeps
   - **Expected:** GPU cull results match CPU reference

### TC-2.3.4.I1 HZB Two Phase No Popin

| # | Requirement |
|---|-------------|
| 1 | R-2.3.4     |

1. **#1** — Fast camera pan revealing occluded geometry
   - **Expected:** No single-frame pop-in; phase 2 recovers same frame

### TC-2.3.4.I2 HZB Occlusion Reduction

| # | Requirement |
|---|-------------|
| 1 | R-2.3.4     |

1. **#1** — Dense urban scene
   - **Expected:** >= 30% draw call reduction from occlusion culling

### TC-2.3.9.I1 Cubemap Face Seams

| # | Requirement |
|---|-------------|
| 1 | R-2.3.9     |

1. **#1** — Dynamic cubemap render
   - **Expected:** Seamless edges between faces (no seam artifacts)

### TC-2.3.10.I1 Scene Capture Same Frame

| # | Requirement |
|---|-------------|
| 1 | R-2.3.10    |

1. **#1** — Scene capture texture used as material input
   - **Expected:** Usable in the same frame it was rendered

### TC-2.3.11.I1 DRS Under Load

| # | Requirement |
|---|-------------|
| 1 | R-2.3.11    |

1. **#1** — Scene exceeds budget, then load drops
   - **Expected:** Resolution decreases within 5 frames, then increases

### TC-2.3.6.I1 Reverse-Z Cross Backend

| # | Requirement |
|---|-------------|
| 1 | R-2.3.6     |

1. **#1** — Depth buffer on Metal, D3D12, Vulkan
   - **Expected:** Clears to 0.0, comparison uses GREATER on all backends

### TC-2.3.12.I1 SSS Profile Scatter

| # | Requirement |
|---|-------------|
| 1 | R-2.3.12    |

1. **#1** — Skin SSS profile
   - **Expected:** Visible light transmission at shadow terminator

### TC-2.3.13.I1 Alpha Cutout Shadow Shape

| # | Requirement |
|---|-------------|
| 1 | R-2.3.13    |

1. **#1** — Alpha-cutout quad with known mask
   - **Expected:** Shadow shape matches alpha mask

### TC-2.10.1.I1 Extraction 100K Entities

| # | Requirement |
|---|-------------|
| 1 | NFR-2.10.1  |

1. **#1** — 100K entities, full extraction
   - **Expected:** Completes in < 2.0 ms

### TC-2.10.1.I2 Extraction 1 Percent Dirty

| # | Requirement |
|---|-------------|
| 1 | NFR-2.10.1  |

1. **#1** — 100K entities, 1% dirty
   - **Expected:** Completes in < 0.5 ms

### TC-2.10.2.I1 Draw List Throughput

| # | Requirement |
|---|-------------|
| 1 | NFR-2.10.2  |

1. **#1** — Draw list construction
   - **Expected:** >= 500K proxies/ms/core

## Benchmarks

### TC-2.3.1.B1 Culling Pipeline

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M meshlets, 1080p | GPU time | < 1.0 ms | NFR-2.3.1 |

### TC-2.3.3.B1 Batch Compaction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10K instances | GPU time | < 0.1 ms | NFR-2.3.3 |

### TC-2.3.4.B1 HZB Build

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p, full mip chain | GPU time | < 0.3 ms | R-2.3.4 |

### TC-2.4.1.B1 Forward Plus Tile Cull

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 lights | GPU time | < 0.5 ms | R-2.4.1 |

### TC-2.4.2.B1 Deferred Light Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 lights | GPU time | < 0.5 ms | R-2.4.2 |

### TC-2.3.2.B1 DRS Feedback Loop

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | DRS feedback overhead | CPU time | < 0.01 ms | NFR-2.3.2 |

### TC-2.10.3.B1 Instance Buffer Upload

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50K instances | CPU time | < 0.5 ms | R-2.10.3 |

### TC-2.4.6.B1 Material Instance Upload

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1K dirty material instances | CPU time | < 0.1 ms | R-2.4.6 |
