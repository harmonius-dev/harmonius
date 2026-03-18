# Terrain Test Cases

Companion test cases for [terrain.md](terrain.md).

## Unit Tests

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

## Integration Tests

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

## Benchmarks

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
