# Meshlet Pipeline Test Cases

Companion test cases for [meshlets.md](meshlets.md).

## Unit Tests

### TC-3.1.1.1 Partition 64v 124t Limits

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |
| 2 | R-3.1.1     |

1. **#1** — Cube mesh (8 vertices, 12 triangles)
   - **Expected:** Every meshlet has <= 64 vertices and <= 124 triangles
2. **#2** — Sphere mesh (10K vertices, 20K triangles)
   - **Expected:** Every meshlet has <= 64 vertices and <= 124 triangles

### TC-3.1.1.2 Bounding Sphere Encloses

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |
| 2 | R-3.1.1     |

1. **#1** — Partitioned cube mesh
   - **Expected:** All vertices in each meshlet lie within its bounding sphere
2. **#2** — Partitioned irregular mesh
   - **Expected:** All vertices within bounding sphere for every meshlet

### TC-3.1.1.3 Normal Cone Valid

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |
| 2 | R-3.1.1     |

1. **#1** — Partitioned mesh, all meshlets
   - **Expected:** Cone axis is unit length (magnitude in [0.999, 1.001])
2. **#2** — Partitioned mesh, all meshlets
   - **Expected:** Cone cutoff is in [-1.0, 1.0]

### TC-3.1.1.4 DAG Cut Watertight

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |
| 2 | R-3.1.1     |
| 3 | R-3.1.1     |

1. **#1** — 3-level DAG from sphere mesh; take cut at level 0
   - **Expected:** Resulting mesh is watertight (no T-junctions)
2. **#2** — Same DAG; take cut at level 1
   - **Expected:** Resulting mesh is watertight
3. **#3** — Same DAG; take cut at level 2 (coarsest)
   - **Expected:** Resulting mesh is watertight

### TC-3.1.1.5 DAG Coarsening

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |

1. **#1** — Parent node with 2 children
   - **Expected:** Parent group_count < sum of children's group_counts

### TC-3.1.5.1 LOD Error Monotonic

| # | Requirement |
|---|-------------|
| 1 | R-3.1.5     |

1. **#1** — DAG from root to leaves
   - **Expected:** parent_error increases monotonically from leaves to root

### TC-3.1.5.2 LOD Select Coarsest

| # | Requirement |
|---|-------------|
| 1 | R-3.1.5     |

1. **#1** — Camera at 1000m, pixel_threshold=1.0
   - **Expected:** LOD resolver selects root node (coarsest)

### TC-3.1.5.3 LOD Select Finest

| # | Requirement |
|---|-------------|
| 1 | R-3.1.5     |

1. **#1** — Camera at 0.1m, pixel_threshold=1.0
   - **Expected:** LOD resolver selects leaf nodes (finest)

### TC-3.1.7.1 VisBuffer Encode Decode Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-3.1.7     |
| 2 | R-3.1.7     |
| 3 | R-3.1.7     |

1. **#1** — instance_id=12345, triangle_id=67890
   - **Expected:** encode() then decode() returns same values
2. **#2** — instance_id=0, triangle_id=0
   - **Expected:** Round-trip returns (0, 0)
3. **#3** — instance_id=u32::MAX, triangle_id=u32::MAX
   - **Expected:** Round-trip returns (MAX, MAX)

### TC-3.1.7.2 VisBuffer Uniqueness

| # | Requirement |
|---|-------------|
| 1 | R-3.1.7     |

1. **#1** — 1000 distinct (instance_id, triangle_id) pairs
   - **Expected:** All 1000 packed u64 values are unique

### TC-3.1.6.1 Page Packing Fits

| # | Requirement |
|---|-------------|
| 1 | R-3.1.6     |

1. **#1** — Baked mesh; inspect all pages
   - **Expected:** vertex_data + index_data per page <= MESHLET_PAGE_SIZE (64 KiB)

### TC-3.1.6.2 Page Alignment

| # | Requirement |
|---|-------------|
| 1 | R-3.1.6     |

1. **#1** — Baked mesh; inspect page offsets
   - **Expected:** All vertex_data_offset and index_data_offset aligned to 64 KiB

### TC-3.1.1.6 Empty Mesh Error

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |

1. **#1** — vertices=[], indices=[]
   - **Expected:** MeshletError::EmptyMesh

### TC-3.1.1.7 Degenerate Triangle

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |

1. **#1** — Mesh with zero-area triangle (3 collinear vertices)
   - **Expected:** MeshletError::DegenerateGeometry { triangle_index }

## Integration Tests

### TC-3.1.1.I1 Bake Stanford Bunny

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |
| 2 | R-3.1.1     |

1. **#1** — Stanford Bunny mesh (69K triangles)
   - **Expected:** Meshlet count, DAG depth, and page count within expected ranges
2. **#2** — Stanford Bunny bake stats
   - **Expected:** total_meshlets > 0; lod_levels >= 3; page_count > 0

### TC-3.1.1.I2 Bake Parallel LOD Determinism

| # | Requirement |
|---|-------------|
| 1 | R-3.1.1     |

1. **#1** — Bake on thread pool (4+ workers) vs single-threaded bake
   - **Expected:** Bit-identical MeshletMesh output

### TC-3.1.3.I1 Culling Frustum GPU

| # | Requirement |
|---|-------------|
| 1 | R-3.1.3     |

1. **#1** — Scene with all meshlets outside camera frustum
   - **Expected:** Zero mesh shader invocations (GPU counter = 0)

### TC-3.1.2.I1 Culling Occlusion Two-Phase

| # | Requirement |
|---|-------------|
| 1 | R-3.1.2     |

1. **#1** — Move occluder to reveal geometry behind it
   - **Expected:** Newly visible geometry appears same frame (no one-frame pop-in)

### TC-3.1.3.I2 Culling Backface Cone

| # | Requirement |
|---|-------------|
| 1 | R-3.1.3     |

1. **#1** — Render sphere from outside
   - **Expected:** Roughly 50% of meshlets culled (backfacing)

### TC-3.1.4.I1 Fallback Indirect Draw

| # | Requirement |
|---|-------------|
| 1 | R-3.1.4     |

1. **#1** — Disable mesh shaders; render reference scene
   - **Expected:** Visual output identical to mesh shader path (pixel diff < 1%)

### TC-3.1.7.I1 VisBuffer Material Correctness

| # | Requirement |
|---|-------------|
| 1 | R-3.1.7     |

1. **#1** — Multi-material mesh; readback material ID per pixel
   - **Expected:** Each pixel resolves to correct material index

### TC-3.1.6.I1 Streaming Page Load

| # | Requirement |
|---|-------------|
| 1 | R-3.1.6     |

1. **#1** — Request 10 pages via async I/O; decompress and upload
   - **Expected:** All pages have correct data (checksum matches)

### TC-3.1.6.I2 Streaming Eviction

| # | Requirement |
|---|-------------|
| 1 | R-3.1.6     |

1. **#1** — Fill page cache to capacity; request new pages
   - **Expected:** LRU eviction occurs; no data corruption in remaining pages

### TC-3.1.6.I3 Streaming Priority

| # | Requirement |
|---|-------------|
| 1 | R-3.1.6     |

1. **#1** — Request pages at distances 5m, 50m, 500m
   - **Expected:** 5m page loads first; 500m page loads last

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
