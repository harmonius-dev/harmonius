# Core Rendering Test Cases

Companion test cases for [core-rendering.md](core-rendering.md).

## Unit Tests

### TC-2.3.7.1 Sort Key Opaque Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Opaque keys: pipeline=1, mat=2, depth=0.5; pipeline=1, mat=1, depth=0.8 | Keys sort by pipeline, then material, then front-to-back depth | R-2.3.7 |

### TC-2.3.7.2 Sort Key Transparent Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transparent keys: depth=0.3, depth=0.8 | depth=0.8 sorts before depth=0.3 (back-to-front) | R-2.3.7, R-2.4.5 |

### TC-2.3.6.1 Perspective Reverse-Z Near Far

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | fov_y=PI/4, aspect=16/9, near=0.1, far=1000.0 | Near plane maps to depth 1.0, far maps to 0.0 | R-2.3.6 |
| 2 | fov_y=PI/4, aspect=16/9, near=0.1, far=None | Valid matrix; infinite far plane | R-2.3.6 |

### TC-2.3.5.1 Orthographic Reverse-Z

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | left=-10, right=10, bottom=-10, top=10, near=0.1, far=100 | Linear depth in [0,1] with reverse mapping | R-2.3.5 |

### TC-2.3.2.1 Frustum Plane Extraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known view-projection matrix, point inside frustum | Point classified as inside | R-2.3.2 |
| 2 | Known view-projection matrix, point outside frustum | Point classified as outside | R-2.3.2 |

### TC-2.3.7.3 Render Layer Mask Filtering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity layer=0x02, view layer=0x01 | Entity excluded (masks do not intersect) | R-2.3.7 |
| 2 | Entity layer=0x03, view layer=0x01 | Entity included (masks intersect) | R-2.3.7 |

### TC-2.3.13.1 Instance Flags Packing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | InstanceFlags: CAST_SHADOWS, TWO_SIDED | Round-trip through u32 preserves all flags | R-2.3.13 |
| 2 | TWO_SIDED flag set | Backface cull skipped for this instance | R-2.3.13 |

### TC-2.3.11.1 DRS Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Step-change GPU load from 8 ms to 16 ms, target=16 ms | Scale converges within 5 frames, oscillation < 5% | NFR-2.3.2 |

### TC-2.3.11.2 DRS Clamp Bounds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | min_scale=0.5, max_scale=1.0, extreme load | Scale stays in [0.5, 1.0] | R-2.3.11 |

### TC-2.4.3.1 Material Permutation Cache

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same PermutationKey queried twice | Same cached PipelineState returned both times | R-2.4.3 |

### TC-2.10.8.1 Bindless Alloc Free Reuse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate index, free it, allocate again | Second allocation returns the freed index | R-2.10.8 |

### TC-2.3.7.4 Batch Compaction Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 instances across 10 materials | Exactly 10 indirect draw calls | NFR-2.3.3 |

### TC-2.3.6.2 View Uniform Struct Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `ViewUniform` struct | Size matches expected GPU layout (std140/std430) | R-2.3.6 |

### TC-2.3.1.1 Light GPU Struct Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `LightGpu` struct | 16-byte aligned, matches HLSL `cbuffer` layout | R-2.3.1 |

### TC-2.3.3.1 Two-Sided Skips Backface Cull

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Meshlet flagged two-sided, facing away from camera | Meshlet not culled by normal cone test | R-2.3.3 |

### TC-2.3.13.2 Alpha Cutout in Shadow Pass

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Alpha-cutout geometry in shadow view | Shadow draw commands generated | R-2.3.13 |

### TC-2.10.2.1 Proxy Insert Remove Reuse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert entity A (index=0), remove A, insert B | Entity B gets index=0 (recycled) | R-2.10.2 |

### TC-2.10.3.1 Proxy Dirty Flags Cleared

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set DIRTY_TRANSFORM, call `clear_dirty_flags()` | `is_dirty()` returns false | R-2.10.3 |

### TC-2.10.3.2 Proxy Incremental Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Update transform only on proxy | DIRTY_TRANSFORM set; mesh_id and material_id unchanged | R-2.10.3 |

## Integration Tests

### TC-2.3.1.I1 Forward Deferred Parity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same scene, forward+ and deferred paths | Pixel-identical lighting within FP tolerance | R-2.3.1 |

### TC-2.3.2.I1 Frustum Cull GPU vs CPU

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 meshlets, 1-degree camera sweeps | GPU cull results match CPU reference | R-2.3.2 |

### TC-2.3.4.I1 HZB Two Phase No Popin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fast camera pan revealing occluded geometry | No single-frame pop-in; phase 2 recovers same frame | R-2.3.4 |

### TC-2.3.4.I2 HZB Occlusion Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dense urban scene | >= 30% draw call reduction from occlusion culling | R-2.3.4 |

### TC-2.3.9.I1 Cubemap Face Seams

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dynamic cubemap render | Seamless edges between faces (no seam artifacts) | R-2.3.9 |

### TC-2.3.10.I1 Scene Capture Same Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene capture texture used as material input | Usable in the same frame it was rendered | R-2.3.10 |

### TC-2.3.11.I1 DRS Under Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene exceeds budget, then load drops | Resolution decreases within 5 frames, then increases | R-2.3.11 |

### TC-2.3.6.I1 Reverse-Z Cross Backend

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Depth buffer on Metal, D3D12, Vulkan | Clears to 0.0, comparison uses GREATER on all backends | R-2.3.6 |

### TC-2.3.12.I1 SSS Profile Scatter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Skin SSS profile | Visible light transmission at shadow terminator | R-2.3.12 |

### TC-2.3.13.I1 Alpha Cutout Shadow Shape

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Alpha-cutout quad with known mask | Shadow shape matches alpha mask | R-2.3.13 |

### TC-2.10.1.I1 Extraction 100K Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100K entities, full extraction | Completes in < 2.0 ms | NFR-2.10.1 |

### TC-2.10.1.I2 Extraction 1 Percent Dirty

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100K entities, 1% dirty | Completes in < 0.5 ms | NFR-2.10.1 |

### TC-2.10.2.I1 Draw List Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Draw list construction | >= 500K proxies/ms/core | NFR-2.10.2 |

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
