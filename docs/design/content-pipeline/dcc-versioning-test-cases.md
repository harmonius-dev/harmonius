# DCC Plugins and Asset Versioning Test Cases

Companion test cases for [dcc-versioning.md](dcc-versioning.md).

## Unit Tests

### TC-12.7.1.1 Asset Header Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write AssetHeader{version=2, sections=5, hash=H1} | Byte buffer | R-12.7.1 |
| 2 | Read back from byte buffer | All fields match original | R-12.7.1 |

### TC-12.7.1.2 Section O(1) Access

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | mmap multi-section asset (10 sections); access section 10 | Data returned without reading sections 1-9 | R-12.7.1 |

### TC-12.7.1.3 Content Hash BLAKE3

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write asset with known payload | Content hash is valid BLAKE3 digest of payload | R-12.7.1 |

### TC-12.7.2.1 Bundle Partial Decompress

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create bundle with 10 assets; extract asset #5 | Asset #5 extracted without decompressing others | R-12.7.2 |

### TC-12.7.2.2 Bundle LZ4 Runtime

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create runtime bundle | Compression format == LZ4 | R-12.7.2 |

### TC-12.7.2.3 Bundle Zstd Distribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create distribution bundle | Compression format == Zstd | R-12.7.2 |

### TC-12.7.2.4 Bundle Manifest Sorted

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create bundle with 100 assets | Manifest entries sorted for O(log n) lookup | R-12.7.2 |

### TC-12.7.3.1 Diff Graph Added Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Diff logic graph V1 (5 nodes) vs V2 (6 nodes) | Added node appears in changes list | R-12.7.3 |

### TC-12.7.3.2 Diff Mesh Vertex Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Diff mesh V1 (1000 verts) vs V2 (1200 verts) | Vertex count delta = +200 reported | R-12.7.3 |

### TC-12.7.3.3 Diff Table Row Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Diff data table V1 vs V2 (row 5 modified) | Changed row 5 identified in diff | R-12.7.3 |

### TC-12.7.4.1 Merge Non-Overlapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Branch A changes section 1; branch B changes section 3 | Auto-merge succeeds; both changes present | R-12.7.4 |

### TC-12.7.4.2 Merge Conflict Detected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Branch A and B both change section 2 | Conflict reported for section 2 | R-12.7.4 |

### TC-12.7.5.1 Auto Merge LWW

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Branch A sets color=red at T1; B sets size=10 at T2 | LWW applies; both properties merged | R-12.7.5 |

### TC-12.7.5.2 Auto Merge Union

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Branch A adds node X; branch B adds node Y | Union resolution; both X and Y present | R-12.7.5 |

### TC-12.7.5.3 Auto Merge Deterministic Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reordered elements in both branches | Deterministic ordering applied | R-12.7.5 |

### TC-12.7.4.3 Git Merge Driver Exit Codes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Invoke Git merge driver with clean merge inputs | Exit code 0 | R-12.7.4 |
| 2 | Invoke Git merge driver with conflicting inputs | Exit code non-zero | R-12.7.4 |

### TC-12.6.1.1 SDK C API Mesh Submit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load SDK .so; call har_export_submit_mesh with 1 triangle | Valid asset file produced | R-12.6.1 |

### TC-12.6.1.2 SDK Version Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call har_sdk_version() | major/minor/patch match build config | R-12.6.1 |

### TC-12.6.1.3 SDK Session Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin session; submit mesh; commit | Session cleaned up; asset file written | R-12.6.1 |

### TC-12.6.1.4 SDK Session Cancel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin session; submit mesh; cancel | No file written; session cleaned up | R-12.6.1 |

### TC-12.6.2.1 Live Link Connect and Disconnect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect mock DCC client | Session created | R-12.6.2 |
| 2 | Disconnect client | Session cleaned up; no resource leaks | R-12.6.2 |

### TC-12.6.2.2 Live Link Delta Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encode mesh delta; send over live link; decode | Decoded data matches original delta | R-12.6.2 |

### TC-12.6.2.3 Live Link Camera Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Broadcast engine camera state{pos=(1,2,3), rot=(0,0,0,1)} | DCC client receives matching camera state | R-12.6.2 |

### TC-12.6.25.1 Material Mapper Blender PBR

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Blender Principled BSDF{base_color=tex1, metallic=0.8, roughness=0.3} | Engine PBR{albedo=tex1, metallic=0.8, roughness=0.3} | R-12.6.25 |

### TC-12.6.25.2 Material Mapper Maya Phong

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Maya Phong{specular_color=white, specular_power=50} | Specular-to-metallic conversion applied | R-12.6.25 |

### TC-12.6.25.3 Material Mapper Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Material missing normal map slot | Fallback default flat normal applied | R-12.6.25 |

### TC-12.6.25.4 Material Mapper Warning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Material with unmappable custom shader node | Warning emitted; node skipped | R-12.6.25 |

### TC-12.7.1.4 Dependency Graph Topological Sort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build graph A->B, B->C, A->C | Sort order: A before B before C | R-12.7.1 |

### TC-12.7.1.5 Dependency Graph Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert cycle A->B->C->A | Error returned with cycle path | R-12.7.1 |

### TC-12.7.1.6 Dependency Graph Invalidation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mark texture T1 changed (T1->material M1->prefab P1) | M1 and P1 invalidated | R-12.7.1 |

### TC-12.7.2.5 Binary Diff Copy and Insert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Diff two similar 1 MB files (90% identical) | Patch uses Copy for unchanged regions, Insert for new | R-12.7.2 |

### TC-12.7.2.6 Binary Diff Apply Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Encode diff between file A and B; apply patch to A | Result matches B exactly | R-12.7.2 |

### TC-12.7.2.7 Binary Diff Hash Verify

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply patch with wrong source hash | Error returned | R-12.7.2 |

## Integration Tests

### TC-12.6.3.I1 Houdini HDA Cook

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load reference HDA; cook with changed params | Output geometry matches golden reference | R-12.6.3 |

### TC-12.6.4.I1 Houdini Mesh Export

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Export Houdini scene with packed prims, curves, volumes | All primitive types import correctly | R-12.6.4 |

### TC-12.6.6.I1 Maya Incremental Export

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Export Maya scene; modify one node; re-export | Only the changed asset updates | R-12.6.6 |

### TC-12.6.7.I1 Maya Animation Curves

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Export Bezier animation curves from Maya | Tangent data matches source within 0.001 tolerance | R-12.6.7 |

### TC-12.6.8.I1 Blender Collection Prefab

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Export Blender collections (3 nested levels) | Engine prefab hierarchy matches collection nesting | R-12.6.8 |

### TC-12.6.9.I1 Blender BSDF Conversion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Export Principled BSDF materials | Engine PBR material has correct texture bindings | R-12.6.9 |

### TC-12.6.11.I1 Garment Fitting Two Bodies

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fit garment to 2 characters with different morphs | Constraints computed for both characters | R-12.6.11 |

### TC-12.6.12.I1 Substance Parameter Tweakable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import .sbsar; change exposed parameter in engine | Textures regenerate with new parameter value | R-12.6.12 |

### TC-12.6.2.I1 Live Link Latency Under 2s

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify mesh in mock DCC client; push via live link | Engine viewport updates within 2 s | R-12.6.2 |

### TC-12.6.22.I1 Git LFS Lock Multi-User

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two users lock the same file | Conflict notification for second user | R-12.6.22 |

### TC-12.6.26.I1 Headless Batch Identical Output

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compare headless batch export vs interactive export | Byte-identical output | R-12.6.26 |

### TC-12.7.2.I1 Bundle Stream Single Asset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stream single asset from 100-asset bundle | Decompression completes; data correct | R-12.7.2 |

### TC-12.7.4.I1 Full Merge Workflow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Branch; edit on both branches; merge via Git driver | Correct merged result; no data loss | R-12.7.4 |

### TC-12.7.1.I1 Dependency Graph Migration Plan

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create assets with outdated versions | Migration plan covers all assets in dependency order | R-12.7.1 |

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
