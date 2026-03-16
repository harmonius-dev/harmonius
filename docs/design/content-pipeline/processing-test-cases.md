# Asset Processing Test Cases

Companion test cases for [processing.md](processing.md).

## Unit Tests

### TC-12.2.1.1 BC7 Compression Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1024x1024 RGBA test texture | BC7 compressed output | R-12.2.1 |
| 2 | Decompress BC7 output | PSNR > 40 dB vs original | R-12.2.1 |

### TC-12.2.1.2 ASTC Compression Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1024x1024 RGBA test texture | ASTC 4x4 compressed output | R-12.2.1 |
| 2 | Decompress ASTC output | PSNR > 38 dB vs original | R-12.2.1 |

### TC-12.2.1.3 ETC2 Compression Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1024x1024 RGBA test texture | ETC2 compressed output | R-12.2.1 |
| 2 | Decompress ETC2 output | PSNR > 35 dB vs original | R-12.2.1 |

### TC-12.2.1.4 Platform Format Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PlatformTarget::Windows | BC7 selected | R-12.2.1 |
| 2 | PlatformTarget::iOS | ASTC selected | R-12.2.1 |
| 3 | PlatformTarget::Android | ETC2 selected | R-12.2.1 |

### TC-12.2.2.1 LOD Triangle Ratios

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100K triangle mesh; generate 4-level LOD chain | Each level meets target ratio within 5% tolerance | R-12.2.2 |

### TC-12.2.2.2 LOD Silhouette Preservation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100K triangle mesh; generate LOD chain | Hausdorff distance below error threshold per LOD level | R-12.2.2 |

### TC-12.2.3.1 Meshlet Size Limits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50K triangle mesh; build meshlets | No meshlet exceeds 64 vertices or 124 triangles | R-12.2.3 |

### TC-12.2.3.2 Meshlet Bounds Validity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50K triangle mesh; build meshlets | Every meshlet has non-degenerate AABB and valid normal cone | R-12.2.3 |

### TC-12.2.4.1 Vertex Cache ACMR Improvement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unoptimized 100K triangle mesh | ACMR after optimization > 20% improvement over before | R-12.2.4 |

### TC-12.2.4.2 Overdraw Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unoptimized 100K triangle mesh | Overdraw ratio after < overdraw ratio before | R-12.2.4 |

### TC-12.2.5.1 Lightmap UV No Overlap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mesh with 500 faces; generate lightmap UVs | Zero chart overlaps via rasterization test | R-12.2.5 |

### TC-12.2.5.2 Lightmap Texel Density Uniformity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mesh with 500 faces; generate lightmap UVs | Texel density variance < 10% across charts | R-12.2.5 |

### TC-12.2.6.1 Opus Encode Quality

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 s WAV file; encode to Opus at 128 kbps | Decoded SNR > 30 dB vs original | R-12.2.6 |

### TC-12.2.6.2 ADPCM Decode Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 ms WAV clip; encode to ADPCM | Decode latency < 1 ms | R-12.2.6 |

### TC-12.2.6.3 PCM Passthrough Bit-Exact

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 ms WAV clip; encode as PCM | Output samples bit-exact match input samples | R-12.2.6 |

### TC-12.2.7.1 HLSL Code Generation Valid Syntax

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20-node shader graph | Generated HLSL passes syntax validation | R-12.2.7 |

### TC-12.2.7.2 HLSL No Template Markers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20-node shader graph | Output contains no `{{`, `%%`, or `<%>` markers | R-12.2.7 |

### TC-12.2.7.3 HLSL Source Map Coverage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20-node shader graph | Every HLSL line maps to a graph node via source map | R-12.2.7 |

### TC-12.2.7.4 Variant Pruning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shader graph with 8 boolean parameters (256 possible) | Only reachable variants generated; count < 256 | R-12.2.7 |

### TC-12.2.8.1 Dependency Graph Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add circular reference A->B->C->A | CycleDetected error | R-12.2.8 |

### TC-12.2.8.2 Incremental Cache Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Process asset X; re-process without changes | Cache hit; processing skipped | R-12.2.8 |

### TC-12.2.8.3 Incremental Invalidation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change shared texture T1 used by materials M1, M2 | M1 and M2 reprocessed; unrelated assets skipped | R-12.2.8 |

### TC-12.2.9.1 DXC HLSL to DXIL

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid HLSL test shader | Valid DXIL bytecode | R-12.2.9 |

### TC-12.2.9.2 DXC HLSL to SPIR-V

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid HLSL test shader | Valid SPIR-V bytecode | R-12.2.9 |

### TC-12.2.9.3 MSC DXIL to MSL

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid DXIL bytecode | Valid MSL source output | R-12.2.9 |

### TC-12.2.9.4 Shader Reflection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HLSL with 3 texture bindings and 2 CBVs | Reflected bindings match source declarations | R-12.2.9 |

### TC-12.2.9.5 Shader Dead Code Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HLSL with unreachable branch | Dead code eliminated in output bytecode | R-12.2.9 |

### TC-12.2.9.6 Shader Error Source Tracing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HLSL with type mismatch error | Error maps to originating graph node via source map | R-12.2.9 |

### TC-12.2.8.4 Content Hash Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same source + config; run twice | Identical BLAKE3 hash both runs | R-12.2.8 |

### TC-12.2.8.5 Processing Graph Topological Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build graph with edges A->B, B->C, A->C | Topological sort: A before B, B before C | R-12.2.8 |

## Integration Tests

### TC-12.2.1.I1 Full Pipeline Texture

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import PNG; process for Windows, iOS, Android | 3 distinct compressed artifacts in CAS (BC7, ASTC, ETC2) | R-12.2.1 |

### TC-12.2.2.I1 Full Pipeline Mesh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import mesh via DCC plugin | LOD chain, meshlets, and optimized index buffer in output | R-12.2.2 |

### TC-12.2.7.I1 Full Pipeline Shader

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shader graph to HLSL to all backends | Valid DXIL, SPIR-V, and MSL outputs | R-12.2.7 |

### TC-12.2.6.I1 Full Pipeline Audio

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import WAV; process as Opus, ADPCM, PCM | 3 artifacts in CAS, one per encoding | R-12.2.6 |

### TC-12.2.1.I2 Multi-Platform Build

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Process 100 assets for 3 platforms | Correct format per platform for all 100 assets | R-12.2.1 |

### TC-12.2.8.I1 Incremental Rebuild End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full build of 50 assets; modify 1 texture; rebuild | Only dependents reprocessed; others use cache | R-12.2.8 |

### TC-12.2.8.I2 Parallel Thread Utilization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Process 1000 assets on 8-core machine | Thread utilization >= 90% | R-12.2.8 |

### TC-12.2.9.I1 Shader Error to Graph Node Navigation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Introduce error in graph node N5 | Editor can navigate to node N5 from error message | R-12.2.9 |

## Benchmarks

### TC-12.2.1.B1 BC7 Compression 1K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | BC7 compress 1024x1024 texture | Wall time | < 10 ms | R-12.2.1 |

### TC-12.2.1.B2 BC7 Compression 4K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | BC7 compress 4096x4096 texture | Wall time | < 100 ms | R-12.2.1 |

### TC-12.2.1.B3 ASTC 4x4 Compression 1K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | ASTC 4x4 compress 1024x1024 texture | Wall time | < 15 ms | R-12.2.1 |

### TC-12.2.2.B1 LOD Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Generate LOD chain for 100K triangle mesh | Wall time | < 50 ms | R-12.2.2 |

### TC-12.2.3.B1 Meshlet Build

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Build meshlets for 100K triangle mesh | Wall time | < 20 ms | R-12.2.3 |

### TC-12.2.4.B1 Vertex Cache Optimization

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Optimize vertex cache for 100K triangle mesh | Wall time | < 10 ms | R-12.2.4 |

### TC-12.2.9.B1 DXC HLSL to DXIL Cold

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compile HLSL to DXIL (cold start) | Wall time | < 500 ms | R-12.2.9 |

### TC-12.2.9.B2 DXC HLSL to SPIR-V Cold

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compile HLSL to SPIR-V (cold start) | Wall time | < 500 ms | R-12.2.9 |

### TC-12.2.9.B3 MSC DXIL to MSL

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Translate DXIL to MSL | Wall time | < 200 ms | R-12.2.9 |

### TC-12.2.8.B1 Incremental Single-Asset Rebuild

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Rebuild 1 changed asset and dependents | Wall time | < 2 s | R-12.2.8 |

### TC-12.2.8.B2 Full Build 10K Assets

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full build of 10K assets on 8 cores | Wall time | < 5 min | R-12.2.8 |

### TC-12.2.8.B3 BLAKE3 Hash 1 MB

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hash 1 MB file | Wall time | < 0.5 ms | R-12.2.8 |

### TC-12.2.8.B4 CAS Lookup Cached

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | CAS lookup for cached entry | Wall time | < 1 ms | R-12.2.8 |
