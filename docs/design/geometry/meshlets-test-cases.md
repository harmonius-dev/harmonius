# Meshlet Pipeline Test Cases

Companion test cases for [meshlets.md](meshlets.md).

## Unit Tests

### TC-3.1.1.1 Partition 64v 124t Limits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cube mesh (8 vertices, 12 triangles) | Every meshlet has <= 64 vertices and <= 124 triangles | R-3.1.1 |
| 2 | Sphere mesh (10K vertices, 20K triangles) | Every meshlet has <= 64 vertices and <= 124 triangles | R-3.1.1 |

### TC-3.1.1.2 Bounding Sphere Encloses

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Partitioned cube mesh | All vertices in each meshlet lie within its bounding sphere | R-3.1.1 |
| 2 | Partitioned irregular mesh | All vertices within bounding sphere for every meshlet | R-3.1.1 |

### TC-3.1.1.3 Normal Cone Valid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Partitioned mesh, all meshlets | Cone axis is unit length (magnitude in [0.999, 1.001]) | R-3.1.1 |
| 2 | Partitioned mesh, all meshlets | Cone cutoff is in [-1.0, 1.0] | R-3.1.1 |

### TC-3.1.1.4 DAG Cut Watertight

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-level DAG from sphere mesh; take cut at level 0 | Resulting mesh is watertight (no T-junctions) | R-3.1.1 |
| 2 | Same DAG; take cut at level 1 | Resulting mesh is watertight | R-3.1.1 |
| 3 | Same DAG; take cut at level 2 (coarsest) | Resulting mesh is watertight | R-3.1.1 |

### TC-3.1.1.5 DAG Coarsening

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent node with 2 children | Parent group_count < sum of children's group_counts | R-3.1.1 |

### TC-3.1.5.1 LOD Error Monotonic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DAG from root to leaves | parent_error increases monotonically from leaves to root | R-3.1.5 |

### TC-3.1.5.2 LOD Select Coarsest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at 1000m, pixel_threshold=1.0 | LOD resolver selects root node (coarsest) | R-3.1.5 |

### TC-3.1.5.3 LOD Select Finest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at 0.1m, pixel_threshold=1.0 | LOD resolver selects leaf nodes (finest) | R-3.1.5 |

### TC-3.1.7.1 VisBuffer Encode Decode Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | instance_id=12345, triangle_id=67890 | encode() then decode() returns same values | R-3.1.7 |
| 2 | instance_id=0, triangle_id=0 | Round-trip returns (0, 0) | R-3.1.7 |
| 3 | instance_id=u32::MAX, triangle_id=u32::MAX | Round-trip returns (MAX, MAX) | R-3.1.7 |

### TC-3.1.7.2 VisBuffer Uniqueness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 distinct (instance_id, triangle_id) pairs | All 1000 packed u64 values are unique | R-3.1.7 |

### TC-3.1.6.1 Page Packing Fits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Baked mesh; inspect all pages | vertex_data + index_data per page <= MESHLET_PAGE_SIZE (64 KiB) | R-3.1.6 |

### TC-3.1.6.2 Page Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Baked mesh; inspect page offsets | All vertex_data_offset and index_data_offset aligned to 64 KiB | R-3.1.6 |

### TC-3.1.1.6 Empty Mesh Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | vertices=[], indices=[] | MeshletError::EmptyMesh | R-3.1.1 |

### TC-3.1.1.7 Degenerate Triangle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mesh with zero-area triangle (3 collinear vertices) | MeshletError::DegenerateGeometry { triangle_index } | R-3.1.1 |

## Integration Tests

### TC-3.1.1.I1 Bake Stanford Bunny

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stanford Bunny mesh (69K triangles) | Meshlet count, DAG depth, and page count within expected ranges | R-3.1.1 |
| 2 | Stanford Bunny bake stats | total_meshlets > 0; lod_levels >= 3; page_count > 0 | R-3.1.1 |

### TC-3.1.1.I2 Bake Parallel LOD Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bake on thread pool (4+ workers) vs single-threaded bake | Bit-identical MeshletMesh output | R-3.1.1 |

### TC-3.1.3.I1 Culling Frustum GPU

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene with all meshlets outside camera frustum | Zero mesh shader invocations (GPU counter = 0) | R-3.1.3 |

### TC-3.1.2.I1 Culling Occlusion Two-Phase

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move occluder to reveal geometry behind it | Newly visible geometry appears same frame (no one-frame pop-in) | R-3.1.2 |

### TC-3.1.3.I2 Culling Backface Cone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render sphere from outside | Roughly 50% of meshlets culled (backfacing) | R-3.1.3 |

### TC-3.1.4.I1 Fallback Indirect Draw

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disable mesh shaders; render reference scene | Visual output identical to mesh shader path (pixel diff < 1%) | R-3.1.4 |

### TC-3.1.7.I1 VisBuffer Material Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Multi-material mesh; readback material ID per pixel | Each pixel resolves to correct material index | R-3.1.7 |

### TC-3.1.6.I1 Streaming Page Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Request 10 pages via async I/O; decompress and upload | All pages have correct data (checksum matches) | R-3.1.6 |

### TC-3.1.6.I2 Streaming Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill page cache to capacity; request new pages | LRU eviction occurs; no data corruption in remaining pages | R-3.1.6 |

### TC-3.1.6.I3 Streaming Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Request pages at distances 5m, 50m, 500m | 5m page loads first; 500m page loads last | R-3.1.6 |

## Benchmarks

### TC-3.1.2.B1 Phase 1 and Phase 2 Cull

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100K instances, two-phase occlusion cull | GPU time | < 1 ms | R-3.1.2 |

### TC-3.1.4.B1 Mesh Shader Rasterize

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M meshlets, mesh shader rasterization | GPU time | < 4 ms | R-3.1.4 |

### TC-3.1.7.B1 Material Eval Fullscreen

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p fullscreen material evaluation compute pass | GPU time | < 2 ms | R-3.1.7 |

### TC-3.1.2.B2 HZB Build

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p depth buffer, full mip chain | GPU time | < 0.5 ms | R-3.1.2 |

### TC-3.1.1.B1 Offline Bake

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100K triangle mesh, full bake pipeline | Wall time | < 2 s | R-3.1.1 |

### TC-3.1.6.B1 Page Load and Decompress

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single 64 KiB page, async I/O + Zstd decompress | Wall time | < 1 ms | R-3.1.6 |

### TC-3.1.6.B2 Streaming Feedback and Readback

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Feedback pass + CPU readback | GPU + readback time | < 0.2 ms | R-3.1.6 |
