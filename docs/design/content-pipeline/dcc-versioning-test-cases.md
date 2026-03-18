# DCC Plugins and Asset Versioning Test Cases

Companion test cases for [dcc-versioning.md](dcc-versioning.md).

## Unit Tests

### TC-12.7.1.1 Asset Header Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.7.1    |
| 2 | R-12.7.1    |

1. **#1** — Write AssetHeader{version=2, sections=5, hash=H1}
   - **Expected:** Byte buffer
2. **#2** — Read back from byte buffer
   - **Expected:** All fields match original

### TC-12.7.1.2 Section O(1) Access

| # | Requirement |
|---|-------------|
| 1 | R-12.7.1    |

1. **#1** — mmap multi-section asset (10 sections); access section 10
   - **Expected:** Data returned without reading sections 1-9

### TC-12.7.1.3 Content Hash BLAKE3

| # | Requirement |
|---|-------------|
| 1 | R-12.7.1    |

1. **#1** — Write asset with known payload
   - **Expected:** Content hash is valid BLAKE3 digest of payload

### TC-12.7.2.1 Bundle Partial Decompress

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Create bundle with 10 assets; extract asset #5
   - **Expected:** Asset #5 extracted without decompressing others

### TC-12.7.2.2 Bundle LZ4 Runtime

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Create runtime bundle
   - **Expected:** Compression format == LZ4

### TC-12.7.2.3 Bundle Zstd Distribution

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Create distribution bundle
   - **Expected:** Compression format == Zstd

### TC-12.7.2.4 Bundle Manifest Sorted

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Create bundle with 100 assets
   - **Expected:** Manifest entries sorted for O(log n) lookup

### TC-12.7.3.1 Diff Graph Added Node

| # | Requirement |
|---|-------------|
| 1 | R-12.7.3    |

1. **#1** — Diff logic graph V1 (5 nodes) vs V2 (6 nodes)
   - **Expected:** Added node appears in changes list

### TC-12.7.3.2 Diff Mesh Vertex Count

| # | Requirement |
|---|-------------|
| 1 | R-12.7.3    |

1. **#1** — Diff mesh V1 (1000 verts) vs V2 (1200 verts)
   - **Expected:** Vertex count delta = +200 reported

### TC-12.7.3.3 Diff Table Row Change

| # | Requirement |
|---|-------------|
| 1 | R-12.7.3    |

1. **#1** — Diff data table V1 vs V2 (row 5 modified)
   - **Expected:** Changed row 5 identified in diff

### TC-12.7.4.1 Merge Non-Overlapping

| # | Requirement |
|---|-------------|
| 1 | R-12.7.4    |

1. **#1** — Branch A changes section 1; branch B changes section 3
   - **Expected:** Auto-merge succeeds; both changes present

### TC-12.7.4.2 Merge Conflict Detected

| # | Requirement |
|---|-------------|
| 1 | R-12.7.4    |

1. **#1** — Branch A and B both change section 2
   - **Expected:** Conflict reported for section 2

### TC-12.7.5.1 Auto Merge LWW

| # | Requirement |
|---|-------------|
| 1 | R-12.7.5    |

1. **#1** — Branch A sets color=red at T1; B sets size=10 at T2
   - **Expected:** LWW applies; both properties merged

### TC-12.7.5.2 Auto Merge Union

| # | Requirement |
|---|-------------|
| 1 | R-12.7.5    |

1. **#1** — Branch A adds node X; branch B adds node Y
   - **Expected:** Union resolution; both X and Y present

### TC-12.7.5.3 Auto Merge Deterministic Order

| # | Requirement |
|---|-------------|
| 1 | R-12.7.5    |

1. **#1** — Reordered elements in both branches
   - **Expected:** Deterministic ordering applied

### TC-12.7.4.3 Git Merge Driver Exit Codes

| # | Requirement |
|---|-------------|
| 1 | R-12.7.4    |
| 2 | R-12.7.4    |

1. **#1** — Invoke Git merge driver with clean merge inputs
   - **Expected:** Exit code 0
2. **#2** — Invoke Git merge driver with conflicting inputs
   - **Expected:** Exit code non-zero

### TC-12.6.1.1 SDK C API Mesh Submit

| # | Requirement |
|---|-------------|
| 1 | R-12.6.1    |

1. **#1** — Load SDK .so; call har_export_submit_mesh with 1 triangle
   - **Expected:** Valid asset file produced

### TC-12.6.1.2 SDK Version Query

| # | Requirement |
|---|-------------|
| 1 | R-12.6.1    |

1. **#1** — Call har_sdk_version()
   - **Expected:** major/minor/patch match build config

### TC-12.6.1.3 SDK Session Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-12.6.1    |

1. **#1** — Begin session; submit mesh; commit
   - **Expected:** Session cleaned up; asset file written

### TC-12.6.1.4 SDK Session Cancel

| # | Requirement |
|---|-------------|
| 1 | R-12.6.1    |

1. **#1** — Begin session; submit mesh; cancel
   - **Expected:** No file written; session cleaned up

### TC-12.6.2.1 Live Link Connect and Disconnect

| # | Requirement |
|---|-------------|
| 1 | R-12.6.2    |
| 2 | R-12.6.2    |

1. **#1** — Connect mock DCC client
   - **Expected:** Session created
2. **#2** — Disconnect client
   - **Expected:** Session cleaned up; no resource leaks

### TC-12.6.2.2 Live Link Delta Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.6.2    |

1. **#1** — Encode mesh delta; send over live link; decode
   - **Expected:** Decoded data matches original delta

### TC-12.6.2.3 Live Link Camera Sync

| # | Requirement |
|---|-------------|
| 1 | R-12.6.2    |

1. **#1** — Broadcast engine camera state{pos=(1,2,3), rot=(0,0,0,1)}
   - **Expected:** DCC client receives matching camera state

### TC-12.6.25.1 Material Mapper Blender PBR

| # | Requirement |
|---|-------------|
| 1 | R-12.6.25   |

1. **#1** — Blender Principled BSDF{base_color=tex1, metallic=0.8, roughness=0.3}
   - **Expected:** Engine PBR{albedo=tex1, metallic=0.8, roughness=0.3}

### TC-12.6.25.2 Material Mapper Maya Phong

| # | Requirement |
|---|-------------|
| 1 | R-12.6.25   |

1. **#1** — Maya Phong{specular_color=white, specular_power=50}
   - **Expected:** Specular-to-metallic conversion applied

### TC-12.6.25.3 Material Mapper Fallback

| # | Requirement |
|---|-------------|
| 1 | R-12.6.25   |

1. **#1** — Material missing normal map slot
   - **Expected:** Fallback default flat normal applied

### TC-12.6.25.4 Material Mapper Warning

| # | Requirement |
|---|-------------|
| 1 | R-12.6.25   |

1. **#1** — Material with unmappable custom shader node
   - **Expected:** Warning emitted; node skipped

### TC-12.7.1.4 Dependency Graph Topological Sort

| # | Requirement |
|---|-------------|
| 1 | R-12.7.1    |

1. **#1** — Build graph A->B, B->C, A->C
   - **Expected:** Sort order: A before B before C

### TC-12.7.1.5 Dependency Graph Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-12.7.1    |

1. **#1** — Insert cycle A->B->C->A
   - **Expected:** Error returned with cycle path

### TC-12.7.1.6 Dependency Graph Invalidation

| # | Requirement |
|---|-------------|
| 1 | R-12.7.1    |

1. **#1** — Mark texture T1 changed (T1->material M1->prefab P1)
   - **Expected:** M1 and P1 invalidated

### TC-12.7.2.5 Binary Diff Copy and Insert

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Diff two similar 1 MB files (90% identical)
   - **Expected:** Patch uses Copy for unchanged regions, Insert for new

### TC-12.7.2.6 Binary Diff Apply Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Encode diff between file A and B; apply patch to A
   - **Expected:** Result matches B exactly

### TC-12.7.2.7 Binary Diff Hash Verify

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Apply patch with wrong source hash
   - **Expected:** Error returned

## Integration Tests

### TC-12.6.3.I1 Houdini HDA Cook

| # | Requirement |
|---|-------------|
| 1 | R-12.6.3    |

1. **#1** — Load reference HDA; cook with changed params
   - **Expected:** Output geometry matches golden reference

### TC-12.6.4.I1 Houdini Mesh Export

| # | Requirement |
|---|-------------|
| 1 | R-12.6.4    |

1. **#1** — Export Houdini scene with packed prims, curves, volumes
   - **Expected:** All primitive types import correctly

### TC-12.6.6.I1 Maya Incremental Export

| # | Requirement |
|---|-------------|
| 1 | R-12.6.6    |

1. **#1** — Export Maya scene; modify one node; re-export
   - **Expected:** Only the changed asset updates

### TC-12.6.7.I1 Maya Animation Curves

| # | Requirement |
|---|-------------|
| 1 | R-12.6.7    |

1. **#1** — Export Bezier animation curves from Maya
   - **Expected:** Tangent data matches source within 0.001 tolerance

### TC-12.6.8.I1 Blender Collection Prefab

| # | Requirement |
|---|-------------|
| 1 | R-12.6.8    |

1. **#1** — Export Blender collections (3 nested levels)
   - **Expected:** Engine prefab hierarchy matches collection nesting

### TC-12.6.9.I1 Blender BSDF Conversion

| # | Requirement |
|---|-------------|
| 1 | R-12.6.9    |

1. **#1** — Export Principled BSDF materials
   - **Expected:** Engine PBR material has correct texture bindings

### TC-12.6.11.I1 Garment Fitting Two Bodies

| # | Requirement |
|---|-------------|
| 1 | R-12.6.11   |

1. **#1** — Fit garment to 2 characters with different morphs
   - **Expected:** Constraints computed for both characters

### TC-12.6.12.I1 Substance Parameter Tweakable

| # | Requirement |
|---|-------------|
| 1 | R-12.6.12   |

1. **#1** — Import .sbsar; change exposed parameter in engine
   - **Expected:** Textures regenerate with new parameter value

### TC-12.6.2.I1 Live Link Latency Under 2s

| # | Requirement |
|---|-------------|
| 1 | R-12.6.2    |

1. **#1** — Modify mesh in mock DCC client; push via live link
   - **Expected:** Engine viewport updates within 2 s

### TC-12.6.22.I1 Git LFS Lock Multi-User

| # | Requirement |
|---|-------------|
| 1 | R-12.6.22   |

1. **#1** — Two users lock the same file
   - **Expected:** Conflict notification for second user

### TC-12.6.26.I1 Headless Batch Identical Output

| # | Requirement |
|---|-------------|
| 1 | R-12.6.26   |

1. **#1** — Compare headless batch export vs interactive export
   - **Expected:** Byte-identical output

### TC-12.7.2.I1 Bundle Stream Single Asset

| # | Requirement |
|---|-------------|
| 1 | R-12.7.2    |

1. **#1** — Stream single asset from 100-asset bundle
   - **Expected:** Decompression completes; data correct

### TC-12.7.4.I1 Full Merge Workflow

| # | Requirement |
|---|-------------|
| 1 | R-12.7.4    |

1. **#1** — Branch; edit on both branches; merge via Git driver
   - **Expected:** Correct merged result; no data loss

### TC-12.7.1.I1 Dependency Graph Migration Plan

| # | Requirement |
|---|-------------|
| 1 | R-12.7.1    |

1. **#1** — Create assets with outdated versions
   - **Expected:** Migration plan covers all assets in dependency order

## Benchmarks

### TC-12.7.1.B1 Asset Write Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Write assets sequentially | Throughput | >= 500 MB/s | R-12.7.1 |

### TC-12.7.1.B2 Asset mmap Section Access

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Access single section via mmap | Wall time | < 1 us per section | R-12.7.1 |

### TC-12.7.1.B3 BLAKE3 Hash 100 MB Asset

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hash 100 MB asset payload | Wall time | < 50 ms | R-12.7.1 |

### TC-12.7.2.B1 LZ4 Decompress 10 MB Section

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Decompress 10 MB LZ4-compressed section | Wall time | < 3 ms | R-12.7.2 |

### TC-12.7.2.B2 Zstd Compress 100 MB Bundle

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compress 100 MB bundle with Zstd | Wall time | < 500 ms | R-12.7.2 |

### TC-12.7.3.B1 Structural Diff 1000-Node Graph

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Diff two versions of 1000-node logic graph | Wall time | < 100 ms | R-12.7.3 |

### TC-12.7.4.B1 Three-Way Merge Non-Conflicting

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Three-way merge of non-conflicting changes | Wall time | < 200 ms | R-12.7.4 |

### TC-12.7.2.B3 Binary Diff 10 MB Asset

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Binary diff of two 10 MB asset versions | Wall time | < 50 ms | R-12.7.2 |

### TC-12.6.2.B1 Live Link Delta Encode

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Encode mesh delta for live link | Wall time | < 1 ms | R-12.6.2 |

### TC-12.6.1.B1 SDK Mesh Submit 100K Vertices

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Submit 100K-vertex mesh via SDK C API | Wall time | < 10 ms | R-12.6.1 |

### TC-12.6.25.B1 Material Mapping Translation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Translate single DCC material to engine PBR | Wall time | < 1 ms | R-12.6.25 |

### TC-12.7.1.B4 Dependency Graph 10K Assets Sort

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Topological sort of 10K-asset dependency graph | Wall time | < 50 ms | R-12.7.1 |
