# World Geometry Test Cases

Companion test cases for [world-geometry.md](world-geometry.md).

---

## Meshlet Pipeline

### Unit Tests

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

### Integration Tests

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

### Benchmarks

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

---

## Terrain

### Unit Tests

### TC-3.2.1.1 Tile Height Sample Bilinear

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |
| 2 | R-3.2.1     |

1. **#1** — Known 4x4 grid, sample at UV (0.5, 0.5)
   - **Expected:** Bilinear interpolation matches expected value
2. **#2** — Known 4x4 grid, sample at UV (0.0, 0.0)
   - **Expected:** Returns corner height exactly

### TC-3.2.1.2 Tile Height at World

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |
| 2 | R-3.2.1     |

1. **#1** — World position (50, 0, 50), tile covers [0..100]
   - **Expected:** Correct height returned
2. **#2** — World position (150, 0, 150), tile covers [0..100]
   - **Expected:** Returns None (out-of-bounds)

### TC-3.2.1.3 Tile Normal Flat

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — Flat tile (all heights equal at y=10)
   - **Expected:** Normal = (0, 1, 0)

### TC-3.2.1.4 Tile Normal Slope

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — 45-degree slope tile
   - **Expected:** Normal is correct and normalized (magnitude = 1.0)

### TC-3.2.3.1 Clipmap Ring Spacing

| # | Requirement |
|---|-------------|
| 1 | R-3.2.3     |

1. **#1** — 8 rings, base_spacing=1.0m
   - **Expected:** Ring N spacing = 2^N meters (1, 2, 4, 8, 16, 32, 64, 128)

### TC-3.2.3.2 Clipmap Morph Boundaries

| # | Requirement |
|---|-------------|
| 1 | R-3.2.3     |
| 2 | R-3.2.3     |

1. **#1** — Vertex at ring boundary
   - **Expected:** morph_factor = 1.0
2. **#2** — Vertex at ring center
   - **Expected:** morph_factor = 0.0

### TC-3.2.3.3 Clipmap Screen Error

| # | Requirement |
|---|-------------|
| 1 | R-3.2.3     |

1. **#1** — 1m spacing at 100m distance, 90-deg FOV, 1080p
   - **Expected:** Screen error < pixel threshold

### TC-3.2.4.1 Hole Mask Bit Operations

| # | Requirement |
|---|-------------|
| 1 | R-3.2.4     |
| 2 | R-3.2.4     |
| 3 | R-3.2.4     |

1. **#1** — Set hole at (5,7); query is_hole(5,7)
   - **Expected:** true
2. **#2** — Clear hole at (5,7); query is_hole(5,7)
   - **Expected:** false
3. **#3** — Set hole at (5,7); query is_hole(5,8)
   - **Expected:** false (adjacent unaffected)

### TC-3.2.4.2 Hole Mask Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-3.2.4     |

1. **#1** — Set 100 random holes; read all back
   - **Expected:** All 100 holes read as true; all others read as false

### TC-3.2.5.1 Splatmap Weight Sum

| # | Requirement |
|---|-------------|
| 1 | R-3.2.5     |

1. **#1** — 4 layers with random weights per vertex
   - **Expected:** Per-vertex weights sum to 255

### TC-3.2.5.2 Splatmap 16 Layers

| # | Requirement |
|---|-------------|
| 1 | R-3.2.5     |

1. **#1** — Tile with 16 active layers
   - **Expected:** All 16 layer indices valid; weight lookup returns > 0 for each

### TC-3.2.6.1 Heightfield Collider Ray Hit

| # | Requirement |
|---|-------------|
| 1 | R-3.2.6     |

1. **#1** — Ray from (50, 100, 50) downward (-Y)
   - **Expected:** Hit at terrain surface; normal is upward (Y > 0)

### TC-3.2.6.2 Heightfield Collider Ray Miss

| # | Requirement |
|---|-------------|
| 1 | R-3.2.6     |

1. **#1** — Ray parallel to terrain surface (direction = (1, 0, 0))
   - **Expected:** No hit

### TC-3.2.6.3 Heightfield Collider Hole Passthrough

| # | Requirement |
|---|-------------|
| 1 | R-3.2.6     |

1. **#1** — Ray through hole region (downward)
   - **Expected:** No hit (ray passes through hole)

### TC-3.2.7.1 World Position Camera Relative

| # | Requirement |
|---|-------------|
| 1 | R-3.2.7     |

1. **#1** — Position=(100000.5, 0, 100000.5), camera=(100000, 0, 100000)
   - **Expected:** Relative = (0.5, 0, 0.5)

### TC-3.2.7.2 World Position No Jitter

| # | Requirement |
|---|-------------|
| 1 | R-3.2.7     |

1. **#1** — 10 positions at 50km from origin; convert to camera-relative f32
   - **Expected:** Sub-millimeter precision (error < 0.001m)

### TC-3.2.9.1 Voxel SDF Sample

| # | Requirement |
|---|-------------|
| 1 | R-3.2.9     |

1. **#1** — Known SDF grid; sample at cell centers
   - **Expected:** Values match expected SDF within 1e-5 tolerance

### TC-3.2.9.2 Voxel Homogeneous Zero Memory

| # | Requirement |
|---|-------------|
| 1 | R-3.2.9     |

1. **#1** — 1000 homogeneous air nodes
   - **Expected:** Total memory = node metadata only; no per-cell arrays

### TC-3.2.10.1 Hybrid Representation Selection

| # | Requirement |
|---|-------------|
| 1 | R-3.2.10    |
| 2 | R-3.2.10    |

1. **#1** — Flat region (no vertical complexity)
   - **Expected:** RepresentationTag = Heightfield
2. **#2** — Region with cave
   - **Expected:** RepresentationTag = Voxel

### TC-3.2.12.1 Marching Cubes Sphere

| # | Requirement |
|---|-------------|
| 1 | R-3.2.12    |

1. **#1** — SDF sphere, radius=5
   - **Expected:** Mesh is closed; vertex count > 0; all normals point outward

### TC-3.2.12.2 Dual Contouring Cube

| # | Requirement |
|---|-------------|
| 1 | R-3.2.12    |

1. **#1** — SDF axis-aligned cube
   - **Expected:** Sharp edges preserved (adjacent face normal angle approximately 90 deg)

### TC-3.2.12.3 Transvoxel No Cracks

| # | Requirement |
|---|-------------|
| 1 | R-3.2.12    |

1. **#1** — Two adjacent chunks at different LODs
   - **Expected:** Shared edge vertices match exactly (no T-junctions)

### TC-3.2.12.4 Meshing Incremental

| # | Requirement |
|---|-------------|
| 1 | R-3.2.12    |

1. **#1** — Modify 1 voxel cell
   - **Expected:** Only containing chunk re-meshed; neighbors unchanged

### TC-3.2.13.1 SDF Edit Add

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Add brush at center, radius=3m
   - **Expected:** SDF values decreased within radius; unchanged outside

### TC-3.2.13.2 SDF Edit Subtract

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Subtract brush at center, radius=3m
   - **Expected:** SDF values increased within radius (material removed)

### TC-3.2.13.3 SDF Edit Smooth

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Smooth brush on noisy SDF region
   - **Expected:** SDF variance decreased within brush radius

### TC-3.2.13.4 SDF Edit Flatten

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Flatten brush with target_height=10.0
   - **Expected:** Surface converges to height=10.0 within brush radius

### TC-3.2.13.5 Edit Delta Size

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Single-cell edit; serialize EditDelta
   - **Expected:** Serialized size < 1 KB

### TC-3.2.13.6 Edit Log Undo

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Apply 5 edits; undo 3
   - **Expected:** Volume matches state after edit 2 exactly

### TC-3.2.13.7 Edit Log Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Apply edits; serialize log; deserialize; replay on fresh volume
   - **Expected:** Identical result to original

### TC-3.2.13.8 Edit Throttle

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Queue 100 edits, max_per_frame=10
   - **Expected:** Flush returns exactly 10; 90 remain queued

### TC-3.2.14.1 Voxel RLE Compression

| # | Requirement |
|---|-------------|
| 1 | R-3.2.14    |

1. **#1** — Homogeneous air chunk, RLE compress
   - **Expected:** Compression ratio > 100:1

### TC-3.2.14.2 Voxel LZ4 Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-3.2.14    |

1. **#1** — Surface chunk, RLE+LZ4 compress then decompress
   - **Expected:** Bit-exact match with original

### TC-3.2.14.3 Voxel Compression Ratio

| # | Requirement |
|---|-------------|
| 1 | R-3.2.14    |

1. **#1** — Mixed terrain chunk (air + surface + solid)
   - **Expected:** Overall compression ratio >= 10:1

### TC-3.2.1.5 Residency Load Nearest First

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — Camera at (0,0); 20 tiles at varying distances
   - **Expected:** to_load sorted by distance ascending

### TC-3.2.1.6 Residency Evict Farthest

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — Move camera 10 tiles away
   - **Expected:** Old distant tiles appear in to_evict list

### TC-3.2.14.4 Residency Budget Enforcement

| # | Requirement |
|---|-------------|
| 1 | R-3.2.14    |

1. **#1** — Budget=10 tiles; load 15
   - **Expected:** Eviction kicks in at 10; only 10 resident

### TC-3.2.2.1 Virtual Page Allocate Release

| # | Requirement |
|---|-------------|
| 1 | R-3.2.2     |

1. **#1** — Allocate 100 pages; release 50; allocate 50
   - **Expected:** No memory leak; all allocations succeed

### TC-3.2.2.2 Virtual Indirection Resolve

| # | Requirement |
|---|-------------|
| 1 | R-3.2.2     |

1. **#1** — Allocate page; resolve VirtualPageId
   - **Expected:** Returns correct PhysicalSlot

### TC-3.2.11.1 Planet Spherical Round-Trip

| # | Requirement |
|---|-------------|
| 1 | R-3.2.11    |

1. **#1** — World position -> spherical coords -> world position
   - **Expected:** Sub-meter precision (error < 1.0m)

### TC-3.2.11.2 Planet Gravity Toward Center

| # | Requirement |
|---|-------------|
| 1 | R-3.2.11    |

1. **#1** — Position on planet surface
   - **Expected:** Gravity vector points toward (0, 0, 0)

### TC-3.2.11.3 Planet Cube Face Projection

| # | Requirement |
|---|-------------|
| 1 | R-3.2.11    |

1. **#1** — 6 axis-aligned positions (+X, -X, +Y, -Y, +Z, -Z)
   - **Expected:** Each maps to correct CubeFace

### Integration Tests

### TC-3.2.1.I1 Stream 100 Tiles No Holes

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — Stream 100 tiles via async I/O
   - **Expected:** All load; no placeholder tiles remain after settling

### TC-3.2.1.I2 Stream Teleport Cancel

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — Start loading 50 tiles; teleport camera 1km away
   - **Expected:** Old loads cancelled; new loads issued

### TC-3.2.1.I3 Stream Budget Eviction

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — Stream 200 tiles, budget for 100
   - **Expected:** 100 resident, 100 evicted, no memory leak

### TC-3.2.3.I1 Clipmap Visual No Seams

| # | Requirement |
|---|-------------|
| 1 | R-3.2.3     |

1. **#1** — Render 8-ring clipmap
   - **Expected:** Adjacent ring vertex heights match within epsilon at morph boundaries

### TC-3.2.4.I1 Collision Matches Visual

| # | Requirement |
|---|-------------|
| 1 | R-3.2.4     |

1. **#1** — Paint holes on terrain
   - **Expected:** Visual mesh discards same triangles as collision rejects

### TC-3.2.10.I1 Hybrid Boundary Stitch

| # | Requirement |
|---|-------------|
| 1 | R-3.2.10    |

1. **#1** — Heightfield tile adjacent to voxel chunk
   - **Expected:** Shared edge vertices produce watertight mesh

### TC-3.2.13.I1 Voxel Edit Mesh Collision Sync

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Dig tunnel through terrain
   - **Expected:** Mesh has hole; collision allows passage; NavMesh path exists

### TC-3.2.13.I2 Multiplayer Edit Replication

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Client applies edit; server validates and replicates
   - **Expected:** Second client receives edit; terrain identical

### TC-3.2.13.I3 Multiplayer Restricted Zone

| # | Requirement |
|---|-------------|
| 1 | R-3.2.13    |

1. **#1** — Client attempts edit in restricted zone
   - **Expected:** EditRejected error; terrain unchanged

### TC-3.2.2.I1 Virtual Texture Feedback Loop

| # | Requirement |
|---|-------------|
| 1 | R-3.2.2     |

1. **#1** — Render terrain; read feedback; load pages; render again
   - **Expected:** Requested pages now resident

### TC-3.2.1.I4 Platform Async IO Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-3.2.1     |

1. **#1** — Same streaming test on Windows, macOS, Linux
   - **Expected:** Identical tile data loaded on all platforms

### Benchmarks

### TC-3.2.1.B1 Tile Decode

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 257x257 U16 heightfield + LZ4 decompress | Wall time | < 1 ms | R-3.2.1 |

### TC-3.2.3.B1 Clipmap LOD Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 tiles, LOD ring update | Wall time | < 500 us | R-3.2.3 |

### TC-3.2.6.B1 Heightfield Ray Cast

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single tile heightfield ray cast | Wall time | < 5 us | R-3.2.6 |

### TC-3.2.12.B1 Voxel Meshing CPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16x16x16 chunk, Marching Cubes | Wall time | < 5 ms | R-3.2.12 |

### TC-3.2.12.B2 Voxel Meshing GPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16x16x16 chunk, GPU compute meshing | GPU time | < 2 ms | R-3.2.12 |

### TC-3.2.13.B1 SDF Edit Apply

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single edit, radius=5m | Wall time | < 1 ms | R-3.2.13 |

### TC-3.2.13.B2 EditDelta Serialize

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single edit delta serialization | Wall time | < 100 us | R-3.2.13 |

### TC-3.2.14.B1 RLE LZ4 Decompress

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Surface chunk, RLE+LZ4 decompression | Wall time | < 500 us | R-3.2.14 |

### TC-3.2.2.B1 Virtual Texture Page Upload

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 128x128 BC7 texture page upload | Wall time | < 200 us | R-3.2.2 |

### TC-3.2.5.B1 Splatmap Blend

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4 layers, 257x257 tile | Wall time | < 2 ms | R-3.2.5 |

### TC-3.2.1.B2 Residency Compute

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000-tile radius, residency calculation | Wall time | < 200 us | R-3.2.1 |

---

## Environment Systems

### Unit Tests

### TC-3.3.2.1 Placement Respects Slope

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |
| 2 | R-3.3.2     |

1. **#1** — Slope range [0, 30], terrain with 35-degree face
   - **Expected:** Zero instances placed on 35-degree face
2. **#2** — Slope range [0, 30], terrain with 25-degree face
   - **Expected:** Instances placed on 25-degree face

### TC-3.3.2.2 Placement Respects Altitude

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |
| 2 | R-3.3.2     |
| 3 | R-3.3.2     |

1. **#1** — Altitude range [100, 500], point at y=50
   - **Expected:** No instance placed
2. **#2** — Altitude range [100, 500], point at y=300
   - **Expected:** Instance placed
3. **#3** — Altitude range [100, 500], point at y=600
   - **Expected:** No instance placed

### TC-3.3.2.3 Placement Density Correlation

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |
| 2 | R-3.3.2     |

1. **#1** — Density map at 100%, area=100m^2, density=10/m^2
   - **Expected:** Instance count approximately 1000 (within 5%)
2. **#2** — Density map at 50%, area=100m^2, density=10/m^2
   - **Expected:** Instance count approximately 500 (within 5%)

### TC-3.3.3.1 LOD Distance Thresholds

| # | Requirement |
|---|-------------|
| 1 | R-3.3.3     |
| 2 | R-3.3.3     |
| 3 | R-3.3.3     |

1. **#1** — Camera at 5m, thresholds=[10, 50, 200]
   - **Expected:** LOD index = 0 (full detail)
2. **#2** — Camera at 30m, thresholds=[10, 50, 200]
   - **Expected:** LOD index = 1 (mid detail)
3. **#3** — Camera at 100m, thresholds=[10, 50, 200]
   - **Expected:** LOD index = 2 (billboard)

### TC-3.3.3.2 Crossfade Active in Range

| # | Requirement |
|---|-------------|
| 1 | R-3.3.3     |
| 2 | R-3.3.3     |

1. **#1** — Camera at LOD boundary (10m), crossfade_width=2m
   - **Expected:** Dither factor in (0.0, 1.0) (crossfade active)
2. **#2** — Camera at 5m, LOD boundary=10m, crossfade_width=2m
   - **Expected:** Dither factor = 0.0 (no crossfade)

### TC-3.3.4.1 Wind Three Layers

| # | Requirement |
|---|-------------|
| 1 | R-3.3.4     |
| 2 | R-3.3.4     |

1. **#1** — All 3 wind layers enabled (trunk, branch, leaf)
   - **Expected:** Vertex displacement has 3 distinct frequency components
2. **#2** — Only trunk layer enabled
   - **Expected:** Vertex displacement has 1 frequency component

### TC-3.3.4.2 Wind Reads Shared Field

| # | Requirement |
|---|-------------|
| 1 | R-3.3.4     |

1. **#1** — Shared wind field texture (F-4.7.5) updated
   - **Expected:** Foliage wind system reads updated wind values

### TC-3.3.5.1 Interaction Decay

| # | Requirement |
|---|-------------|
| 1 | R-3.3.5     |
| 2 | R-3.3.5     |

1. **#1** — Write impulse magnitude=1.0 to interaction buffer; wait 2s, decay_time=2s
   - **Expected:** Magnitude approximately 0.0
2. **#2** — Write impulse magnitude=1.0; wait 0.5s, decay_time=2s
   - **Expected:** Magnitude approximately 0.75

### TC-3.3.6.1 Grass Density Scales

| # | Requirement |
|---|-------------|
| 1 | R-3.3.6     |
| 2 | R-3.3.6     |

1. **#1** — Camera at 10m; grass blades in view = N1
   - **Expected:** N1 > 0
2. **#2** — Camera at 50m; grass blades in view = N2
   - **Expected:** N2 < N1 (density decreases)

### TC-3.3.7.1 Tree Subsurface Transmission

| # | Requirement |
|---|-------------|
| 1 | R-3.3.7     |
| 2 | R-3.3.7     |

1. **#1** — Render tree with backlight (sun behind tree)
   - **Expected:** Leaf pixels have subsurface contribution > 0
2. **#2** — Render tree with frontlight (sun in front of tree)
   - **Expected:** Leaf pixels have zero or minimal subsurface contribution

### TC-3.4.1.1 FFT Tile Continuity

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |

1. **#1** — Sample displacement at tile boundary, 60 frames
   - **Expected:** Delta < 0.001m across all frames

### TC-3.4.1.2 FFT Cascade Count

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |
| 2 | R-3.4.1     |

1. **#1** — Configure 3 cascades
   - **Expected:** 3 IFFT dispatches with distinct resolutions
2. **#2** — Configure 1 cascade
   - **Expected:** 1 IFFT dispatch

### TC-3.4.1.3 Spectrum Types

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |
| 2 | R-3.4.1     |
| 3 | R-3.4.1     |

1. **#1** — Phillips spectrum initialization
   - **Expected:** Distinct frequency distribution from JONSWAP
2. **#2** — JONSWAP spectrum initialization
   - **Expected:** Distinct frequency distribution from TMA
3. **#3** — TMA spectrum initialization
   - **Expected:** Distinct frequency distribution from Phillips

### TC-3.4.2.1 Shoreline Opacity Fade

| # | Requirement |
|---|-------------|
| 1 | R-3.4.2     |
| 2 | R-3.4.2     |

1. **#1** — Water at depth > 2m from terrain surface
   - **Expected:** Opacity = 1.0
2. **#2** — Water at depth = 0 (terrain intersection)
   - **Expected:** Opacity = 0.0

### TC-3.4.2.2 Shoreline Foam Mask

| # | Requirement |
|---|-------------|
| 1 | R-3.4.2     |
| 2 | R-3.4.2     |

1. **#1** — Pixel within configured shoreline band
   - **Expected:** Foam mask > 0
2. **#2** — Pixel far from shore (10m+ depth)
   - **Expected:** Foam mask = 0

### TC-3.4.3.1 Underwater Fog Density

| # | Requirement |
|---|-------------|
| 1 | R-3.4.3     |
| 2 | R-3.4.3     |

1. **#1** — Camera at 5m depth
   - **Expected:** Fog density > fog at 1m depth (Beer-Lambert)
2. **#2** — Camera at 1m depth
   - **Expected:** Fog density > 0

### TC-3.4.3.2 Underwater Absorption

| # | Requirement |
|---|-------------|
| 1 | R-3.4.3     |

1. **#1** — Scene color at 10m depth, absorption spectrum configured
   - **Expected:** Color shifted toward absorption spectrum

### TC-3.4.4.1 Caustics Depth Scaling

| # | Requirement |
|---|-------------|
| 1 | R-3.4.4     |
| 2 | R-3.4.4     |

1. **#1** — Seabed at 2m depth, wave amplitude=1.0
   - **Expected:** Caustic intensity = I1
2. **#2** — Seabed at 10m depth, wave amplitude=1.0
   - **Expected:** Caustic intensity < I1 (decreases with depth)

### TC-3.4.5.1 Fresnel Grazing Angle

| # | Requirement |
|---|-------------|
| 1 | R-3.4.5     |
| 2 | R-3.4.5     |

1. **#1** — Water surface viewed at grazing angle (> 80 deg from normal)
   - **Expected:** Reflection > refraction
2. **#2** — Water surface viewed at steep angle (< 20 deg from normal)
   - **Expected:** Refraction > reflection

### TC-3.4.6.1 Flow Map UV Advance

| # | Requirement |
|---|-------------|
| 1 | R-3.4.6     |

1. **#1** — Uniform rightward flow (1, 0); advance 10 frames
   - **Expected:** Normal UV offset advances rightward each frame

### TC-3.4.7.1 Foam Coverage Decay

| # | Requirement |
|---|-------------|
| 1 | R-3.4.7     |
| 2 | R-3.4.7     |

1. **#1** — All foam sources active; foam_lifetime=2s; wait 3s
   - **Expected:** Foam coverage = 0 (fully decayed)
2. **#2** — All foam sources active; foam_lifetime=2s; wait 1s
   - **Expected:** Foam coverage > 0 (still decaying)

### TC-3.4.7.2 Foam Jacobian Whitecaps

| # | Requirement |
|---|-------------|
| 1 | R-3.4.7     |
| 2 | R-3.4.7     |

1. **#1** — Jacobian value < whitecap threshold
   - **Expected:** Foam present at that pixel
2. **#2** — Jacobian value > whitecap threshold
   - **Expected:** No foam at that pixel

### TC-3.5.1.1 Sky Luminance Monotonic

| # | Requirement |
|---|-------------|
| 1 | R-3.5.1     |

1. **#1** — Evaluate sky for zenith angles 0, 30, 60, 90 deg (opposite sun)
   - **Expected:** Luminance decreases monotonically

### TC-3.5.1.2 Sky Warm at Sunset

| # | Requirement |
|---|-------------|
| 1 | R-3.5.1     |
| 2 | R-3.5.1     |

1. **#1** — Sun at 5 degrees above horizon
   - **Expected:** Chromaticity shifts warm (higher red/orange)
2. **#2** — Sun at 60 degrees above horizon
   - **Expected:** Chromaticity neutral/blue

### TC-3.5.2.1 Aerial Perspective Depth

| # | Requirement |
|---|-------------|
| 1 | R-3.5.2     |

1. **#1** — Render objects at 1km, 10km, 50km from camera
   - **Expected:** 50km pixel color closer to horizon color than 1km pixel

### TC-3.5.2.2 LUT Recompute on Change

| # | Requirement |
|---|-------------|
| 1 | R-3.5.2     |

1. **#1** — Change AtmosphereConfig parameters
   - **Expected:** LUTs are recomputed (dirty flag set and cleared)

### TC-3.5.2.3 Mie Sun Halo

| # | Requirement |
|---|-------------|
| 1 | R-3.5.2     |

1. **#1** — Sample sky luminance near sun direction
   - **Expected:** Bright halo from Mie scattering (luminance peak)

### TC-3.5.3.1 Cloud Coverage Correlation

| # | Requirement |
|---|-------------|
| 1 | R-3.5.3     |
| 2 | R-3.5.3     |

1. **#1** — Weather map at 50% coverage
   - **Expected:** Cloud pixels non-transparent where coverage > 0
2. **#2** — Weather map at 0% coverage
   - **Expected:** No cloud pixels visible

### TC-3.5.3.2 Cloud Temporal Savings

| # | Requirement |
|---|-------------|
| 1 | R-3.5.3     |

1. **#1** — Temporal reprojection enabled vs disabled
   - **Expected:** Per-frame sample count reduced by >= 50%

### TC-3.5.4.1 Cloud Shadow Modulation

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Terrain under cloud shadow
   - **Expected:** Shadowed pixels receive less direct light than unshadowed

### TC-3.5.4.2 Cloud Shadow Moves

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Animate cloud coverage over 60 frames
   - **Expected:** Shadow pattern position changes between frames

### TC-3.5.5.1 TOD Sun Arc

| # | Requirement |
|---|-------------|
| 1 | R-3.5.5     |

1. **#1** — Advance dawn to night in 60s
   - **Expected:** Sun position follows smooth arc (no discontinuities)

### TC-3.5.5.2 TOD Time Scale

| # | Requirement |
|---|-------------|
| 1 | R-3.5.5     |
| 2 | R-3.5.5     |

1. **#1** — time_scale=1.0; measure cycle_duration
   - **Expected:** Cycle completes in T seconds
2. **#2** — time_scale=2.0; measure cycle_duration
   - **Expected:** Cycle completes in T/2 seconds

### TC-3.5.6.1 Star Magnitude Brightness

| # | Requirement |
|---|-------------|
| 1 | R-3.5.6     |

1. **#1** — Star magnitude=1.0 vs star magnitude=5.0
   - **Expected:** Magnitude-1 star brighter than magnitude-5 star

### TC-3.5.6.2 Star Horizon Extinction

| # | Requirement |
|---|-------------|
| 1 | R-3.5.6     |
| 2 | R-3.5.6     |

1. **#1** — Star at 2 degrees above horizon
   - **Expected:** Brightness reduced by atmospheric extinction
2. **#2** — Star at zenith
   - **Expected:** Full brightness (minimal extinction)

### TC-3.5.6.3 Moon Phase Illumination

| # | Requirement |
|---|-------------|
| 1 | R-3.5.6     |
| 2 | R-3.5.6     |

1. **#1** — First quarter moon phase
   - **Expected:** Approximately half the moon disc illuminated
2. **#2** — Full moon phase
   - **Expected:** Entire disc illuminated

### TC-3.5.7.1 Cubemap Ambient Shift

| # | Requirement |
|---|-------------|
| 1 | R-3.5.7     |

1. **#1** — Change sky from clear to overcast
   - **Expected:** Cubemap ambient color shifts within update period

### Integration Tests

### TC-3.3.1.I1 Foliage 1M Instances

| # | Requirement |
|---|-------------|
| 1 | R-3.3.1     |

1. **#1** — Render 1M foliage instances, GPU culling enabled
   - **Expected:** Constant CPU draw call count (GPU-driven)

### TC-3.3.2.I1 Foliage No Disk Data

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |

1. **#1** — Procedural placement only; check disk I/O
   - **Expected:** Zero per-instance data read from disk

### TC-3.4.1.I1 Ocean Physics Coupling

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |

1. **#1** — Drop rigid body onto ocean surface
   - **Expected:** Buoyancy force from WaterSurface displacement applied

### TC-3.4.6.I1 River Ocean Seamless

| # | Requirement |
|---|-------------|
| 1 | R-3.4.6     |

1. **#1** — River at ocean estuary
   - **Expected:** Seamless mesh and flow transition at boundary

### TC-3.5.5.I1 Full Day Cycle

| # | Requirement |
|---|-------------|
| 1 | R-3.5.5     |

1. **#1** — Run 24h cycle (dawn, day, dusk, night)
   - **Expected:** All transitions smooth; no discontinuities

### TC-3.5.7.I1 Cubemap Reflects Sky

| # | Requirement |
|---|-------------|
| 1 | R-3.5.7     |

1. **#1** — Place reflective sphere; render
   - **Expected:** Reflections match current atmosphere/sky state

### TC-3.5.4.I1 Cloud Shadow on Foliage

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Forest under clouds
   - **Expected:** Foliage receives cloud shadow modulation

### TC-3.5.4.I2 Cloud Shadow on Water

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Ocean under clouds
   - **Expected:** Water surface receives cloud shadow modulation

### TC-3.3.1.I2 Tier Mobile Budget

| # | Requirement |
|---|-------------|
| 1 | R-3.3.1     |

1. **#1** — Run mobile tier, full environment scene
   - **Expected:** Frame time < 16 ms on target hardware

### TC-3.3.1.I3 Tier Desktop Quality

| # | Requirement |
|---|-------------|
| 1 | R-3.3.1     |

1. **#1** — Run desktop tier, full environment scene
   - **Expected:** All features enabled and rendering correctly

### Benchmarks

### TC-3.3.1.B1 Foliage Cull 1M Instances

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M foliage instances, compute cull pass | GPU time | < 1 ms | R-3.3.1 |

### TC-3.3.2.B1 Foliage Placement Per Tile

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single terrain tile, procedural placement | GPU time | < 2 ms | R-3.3.2 |

### TC-3.4.1.B1 Ocean FFT

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256x256 resolution, 3 cascades | GPU time | < 1 ms | R-3.4.1 |

### TC-3.5.3.B1 Cloud Ray March

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Half-resolution, 96 ray march steps | GPU time | < 3 ms | R-3.5.3 |

### TC-3.5.2.B1 Atmosphere LUT Rebuild

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full atmosphere LUT recompute | GPU time | < 2 ms | R-3.5.2 |

### TC-3.5.7.B1 Cubemap Capture Per Face

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single cubemap face render | GPU time | < 0.5 ms | R-3.5.7 |

### TC-3.5.4.B1 Cloud Shadow Map

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2048x2048 cloud shadow map generation | GPU time | < 0.5 ms | R-3.5.4 |

### TC-3.3.6.B1 Grass Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200K grass blades, mesh shader generation | GPU time | < 1.5 ms | R-3.3.6 |
