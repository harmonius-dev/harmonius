# Terrain Test Cases

Companion test cases for [terrain.md](terrain.md).

## Unit Tests

### TC-3.2.1.1 Tile Height Sample Bilinear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known 4x4 grid, sample at UV (0.5, 0.5) | Bilinear interpolation matches expected value | R-3.2.1 |
| 2 | Known 4x4 grid, sample at UV (0.0, 0.0) | Returns corner height exactly | R-3.2.1 |

### TC-3.2.1.2 Tile Height at World

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | World position (50, 0, 50), tile covers [0..100] | Correct height returned | R-3.2.1 |
| 2 | World position (150, 0, 150), tile covers [0..100] | Returns None (out-of-bounds) | R-3.2.1 |

### TC-3.2.1.3 Tile Normal Flat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flat tile (all heights equal at y=10) | Normal = (0, 1, 0) | R-3.2.1 |

### TC-3.2.1.4 Tile Normal Slope

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 45-degree slope tile | Normal is correct and normalized (magnitude = 1.0) | R-3.2.1 |

### TC-3.2.3.1 Clipmap Ring Spacing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8 rings, base_spacing=1.0m | Ring N spacing = 2^N meters (1, 2, 4, 8, 16, 32, 64, 128) | R-3.2.3 |

### TC-3.2.3.2 Clipmap Morph Boundaries

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vertex at ring boundary | morph_factor = 1.0 | R-3.2.3 |
| 2 | Vertex at ring center | morph_factor = 0.0 | R-3.2.3 |

### TC-3.2.3.3 Clipmap Screen Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1m spacing at 100m distance, 90-deg FOV, 1080p | Screen error < pixel threshold | R-3.2.3 |

### TC-3.2.4.1 Hole Mask Bit Operations

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set hole at (5,7); query is_hole(5,7) | true | R-3.2.4 |
| 2 | Clear hole at (5,7); query is_hole(5,7) | false | R-3.2.4 |
| 3 | Set hole at (5,7); query is_hole(5,8) | false (adjacent unaffected) | R-3.2.4 |

### TC-3.2.4.2 Hole Mask Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set 100 random holes; read all back | All 100 holes read as true; all others read as false | R-3.2.4 |

### TC-3.2.5.1 Splatmap Weight Sum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 layers with random weights per vertex | Per-vertex weights sum to 255 | R-3.2.5 |

### TC-3.2.5.2 Splatmap 16 Layers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tile with 16 active layers | All 16 layer indices valid; weight lookup returns > 0 for each | R-3.2.5 |

### TC-3.2.6.1 Heightfield Collider Ray Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray from (50, 100, 50) downward (-Y) | Hit at terrain surface; normal is upward (Y > 0) | R-3.2.6 |

### TC-3.2.6.2 Heightfield Collider Ray Miss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray parallel to terrain surface (direction = (1, 0, 0)) | No hit | R-3.2.6 |

### TC-3.2.6.3 Heightfield Collider Hole Passthrough

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray through hole region (downward) | No hit (ray passes through hole) | R-3.2.6 |

### TC-3.2.7.1 World Position Camera Relative

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Position=(100000.5, 0, 100000.5), camera=(100000, 0, 100000) | Relative = (0.5, 0, 0.5) | R-3.2.7 |

### TC-3.2.7.2 World Position No Jitter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 positions at 50km from origin; convert to camera-relative f32 | Sub-millimeter precision (error < 0.001m) | R-3.2.7 |

### TC-3.2.9.1 Voxel SDF Sample

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known SDF grid; sample at cell centers | Values match expected SDF within 1e-5 tolerance | R-3.2.9 |

### TC-3.2.9.2 Voxel Homogeneous Zero Memory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 homogeneous air nodes | Total memory = node metadata only; no per-cell arrays | R-3.2.9 |

### TC-3.2.10.1 Hybrid Representation Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flat region (no vertical complexity) | RepresentationTag = Heightfield | R-3.2.10 |
| 2 | Region with cave | RepresentationTag = Voxel | R-3.2.10 |

### TC-3.2.12.1 Marching Cubes Sphere

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SDF sphere, radius=5 | Mesh is closed; vertex count > 0; all normals point outward | R-3.2.12 |

### TC-3.2.12.2 Dual Contouring Cube

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SDF axis-aligned cube | Sharp edges preserved (adjacent face normal angle approximately 90 deg) | R-3.2.12 |

### TC-3.2.12.3 Transvoxel No Cracks

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two adjacent chunks at different LODs | Shared edge vertices match exactly (no T-junctions) | R-3.2.12 |

### TC-3.2.12.4 Meshing Incremental

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify 1 voxel cell | Only containing chunk re-meshed; neighbors unchanged | R-3.2.12 |

### TC-3.2.13.1 SDF Edit Add

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add brush at center, radius=3m | SDF values decreased within radius; unchanged outside | R-3.2.13 |

### TC-3.2.13.2 SDF Edit Subtract

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Subtract brush at center, radius=3m | SDF values increased within radius (material removed) | R-3.2.13 |

### TC-3.2.13.3 SDF Edit Smooth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Smooth brush on noisy SDF region | SDF variance decreased within brush radius | R-3.2.13 |

### TC-3.2.13.4 SDF Edit Flatten

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Flatten brush with target_height=10.0 | Surface converges to height=10.0 within brush radius | R-3.2.13 |

### TC-3.2.13.5 Edit Delta Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Single-cell edit; serialize EditDelta | Serialized size < 1 KB | R-3.2.13 |

### TC-3.2.13.6 Edit Log Undo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 5 edits; undo 3 | Volume matches state after edit 2 exactly | R-3.2.13 |

### TC-3.2.13.7 Edit Log Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply edits; serialize log; deserialize; replay on fresh volume | Identical result to original | R-3.2.13 |

### TC-3.2.13.8 Edit Throttle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queue 100 edits, max_per_frame=10 | Flush returns exactly 10; 90 remain queued | R-3.2.13 |

### TC-3.2.14.1 Voxel RLE Compression

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Homogeneous air chunk, RLE compress | Compression ratio > 100:1 | R-3.2.14 |

### TC-3.2.14.2 Voxel LZ4 Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Surface chunk, RLE+LZ4 compress then decompress | Bit-exact match with original | R-3.2.14 |

### TC-3.2.14.3 Voxel Compression Ratio

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mixed terrain chunk (air + surface + solid) | Overall compression ratio >= 10:1 | R-3.2.14 |

### TC-3.2.1.5 Residency Load Nearest First

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at (0,0); 20 tiles at varying distances | to_load sorted by distance ascending | R-3.2.1 |

### TC-3.2.1.6 Residency Evict Farthest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move camera 10 tiles away | Old distant tiles appear in to_evict list | R-3.2.1 |

### TC-3.2.14.4 Residency Budget Enforcement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget=10 tiles; load 15 | Eviction kicks in at 10; only 10 resident | R-3.2.14 |

### TC-3.2.2.1 Virtual Page Allocate Release

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate 100 pages; release 50; allocate 50 | No memory leak; all allocations succeed | R-3.2.2 |

### TC-3.2.2.2 Virtual Indirection Resolve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate page; resolve VirtualPageId | Returns correct PhysicalSlot | R-3.2.2 |

### TC-3.2.11.1 Planet Spherical Round-Trip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | World position -> spherical coords -> world position | Sub-meter precision (error < 1.0m) | R-3.2.11 |

### TC-3.2.11.2 Planet Gravity Toward Center

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Position on planet surface | Gravity vector points toward (0, 0, 0) | R-3.2.11 |

### TC-3.2.11.3 Planet Cube Face Projection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 6 axis-aligned positions (+X, -X, +Y, -Y, +Z, -Z) | Each maps to correct CubeFace | R-3.2.11 |

## Integration Tests

### TC-3.2.1.I1 Stream 100 Tiles No Holes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream 100 tiles via async I/O | All load; no placeholder tiles remain after settling | R-3.2.1 |

### TC-3.2.1.I2 Stream Teleport Cancel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start loading 50 tiles; teleport camera 1km away | Old loads cancelled; new loads issued | R-3.2.1 |

### TC-3.2.1.I3 Stream Budget Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream 200 tiles, budget for 100 | 100 resident, 100 evicted, no memory leak | R-3.2.1 |

### TC-3.2.3.I1 Clipmap Visual No Seams

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render 8-ring clipmap | Adjacent ring vertex heights match within epsilon at morph boundaries | R-3.2.3 |

### TC-3.2.4.I1 Collision Matches Visual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Paint holes on terrain | Visual mesh discards same triangles as collision rejects | R-3.2.4 |

### TC-3.2.10.I1 Hybrid Boundary Stitch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Heightfield tile adjacent to voxel chunk | Shared edge vertices produce watertight mesh | R-3.2.10 |

### TC-3.2.13.I1 Voxel Edit Mesh Collision Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dig tunnel through terrain | Mesh has hole; collision allows passage; NavMesh path exists | R-3.2.13 |

### TC-3.2.13.I2 Multiplayer Edit Replication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client applies edit; server validates and replicates | Second client receives edit; terrain identical | R-3.2.13 |

### TC-3.2.13.I3 Multiplayer Restricted Zone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client attempts edit in restricted zone | EditRejected error; terrain unchanged | R-3.2.13 |

### TC-3.2.2.I1 Virtual Texture Feedback Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render terrain; read feedback; load pages; render again | Requested pages now resident | R-3.2.2 |

### TC-3.2.1.I4 Platform Async IO Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same streaming test on Windows, macOS, Linux | Identical tile data loaded on all platforms | R-3.2.1 |

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
