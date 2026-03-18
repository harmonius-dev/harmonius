# Asset Processing Test Cases

Companion test cases for [processing.md](processing.md).

## Unit Tests

### TC-12.2.1.1 BC7 Compression Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.2.1    |
| 2 | R-12.2.1    |

1. **#1** — 1024x1024 RGBA test texture
   - **Expected:** BC7 compressed output
2. **#2** — Decompress BC7 output
   - **Expected:** PSNR > 40 dB vs original

### TC-12.2.1.2 ASTC Compression Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.2.1    |
| 2 | R-12.2.1    |

1. **#1** — 1024x1024 RGBA test texture
   - **Expected:** ASTC 4x4 compressed output
2. **#2** — Decompress ASTC output
   - **Expected:** PSNR > 38 dB vs original

### TC-12.2.1.3 ETC2 Compression Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.2.1    |
| 2 | R-12.2.1    |

1. **#1** — 1024x1024 RGBA test texture
   - **Expected:** ETC2 compressed output
2. **#2** — Decompress ETC2 output
   - **Expected:** PSNR > 35 dB vs original

### TC-12.2.1.4 Platform Format Selection

| # | Requirement |
|---|-------------|
| 1 | R-12.2.1    |
| 2 | R-12.2.1    |
| 3 | R-12.2.1    |

1. **#1** — PlatformTarget::Windows
   - **Expected:** BC7 selected
2. **#2** — PlatformTarget::iOS
   - **Expected:** ASTC selected
3. **#3** — PlatformTarget::Android
   - **Expected:** ETC2 selected

### TC-12.2.2.1 LOD Triangle Ratios

| # | Requirement |
|---|-------------|
| 1 | R-12.2.2    |

1. **#1** — 100K triangle mesh; generate 4-level LOD chain
   - **Expected:** Each level meets target ratio within 5% tolerance

### TC-12.2.2.2 LOD Silhouette Preservation

| # | Requirement |
|---|-------------|
| 1 | R-12.2.2    |

1. **#1** — 100K triangle mesh; generate LOD chain
   - **Expected:** Hausdorff distance below error threshold per LOD level

### TC-12.2.3.1 Meshlet Size Limits

| # | Requirement |
|---|-------------|
| 1 | R-12.2.3    |

1. **#1** — 50K triangle mesh; build meshlets
   - **Expected:** No meshlet exceeds 64 vertices or 124 triangles

### TC-12.2.3.2 Meshlet Bounds Validity

| # | Requirement |
|---|-------------|
| 1 | R-12.2.3    |

1. **#1** — 50K triangle mesh; build meshlets
   - **Expected:** Every meshlet has non-degenerate AABB and valid normal cone

### TC-12.2.4.1 Vertex Cache ACMR Improvement

| # | Requirement |
|---|-------------|
| 1 | R-12.2.4    |

1. **#1** — Unoptimized 100K triangle mesh
   - **Expected:** ACMR after optimization > 20% improvement over before

### TC-12.2.4.2 Overdraw Reduction

| # | Requirement |
|---|-------------|
| 1 | R-12.2.4    |

1. **#1** — Unoptimized 100K triangle mesh
   - **Expected:** Overdraw ratio after < overdraw ratio before

### TC-12.2.5.1 Lightmap UV No Overlap

| # | Requirement |
|---|-------------|
| 1 | R-12.2.5    |

1. **#1** — Mesh with 500 faces; generate lightmap UVs
   - **Expected:** Zero chart overlaps via rasterization test

### TC-12.2.5.2 Lightmap Texel Density Uniformity

| # | Requirement |
|---|-------------|
| 1 | R-12.2.5    |

1. **#1** — Mesh with 500 faces; generate lightmap UVs
   - **Expected:** Texel density variance < 10% across charts

### TC-12.2.6.1 Opus Encode Quality

| # | Requirement |
|---|-------------|
| 1 | R-12.2.6    |

1. **#1** — 10 s WAV file; encode to Opus at 128 kbps
   - **Expected:** Decoded SNR > 30 dB vs original

### TC-12.2.6.2 ADPCM Decode Latency

| # | Requirement |
|---|-------------|
| 1 | R-12.2.6    |

1. **#1** — 100 ms WAV clip; encode to ADPCM
   - **Expected:** Decode latency < 1 ms

### TC-12.2.6.3 PCM Passthrough Bit-Exact

| # | Requirement |
|---|-------------|
| 1 | R-12.2.6    |

1. **#1** — 100 ms WAV clip; encode as PCM
   - **Expected:** Output samples bit-exact match input samples

### TC-12.2.7.1 HLSL Code Generation Valid Syntax

| # | Requirement |
|---|-------------|
| 1 | R-12.2.7    |

1. **#1** — 20-node shader graph
   - **Expected:** Generated HLSL passes syntax validation

### TC-12.2.7.2 HLSL No Template Markers

| # | Requirement |
|---|-------------|
| 1 | R-12.2.7    |

1. **#1** — 20-node shader graph
   - **Expected:** Output contains no `{{`, `%%`, or `<%>` markers

### TC-12.2.7.3 HLSL Source Map Coverage

| # | Requirement |
|---|-------------|
| 1 | R-12.2.7    |

1. **#1** — 20-node shader graph
   - **Expected:** Every HLSL line maps to a graph node via source map

### TC-12.2.7.4 Variant Pruning

| # | Requirement |
|---|-------------|
| 1 | R-12.2.7    |

1. **#1** — Shader graph with 8 boolean parameters (256 possible)
   - **Expected:** Only reachable variants generated; count < 256

### TC-12.2.8.1 Dependency Graph Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Add circular reference A->B->C->A
   - **Expected:** CycleDetected error

### TC-12.2.8.2 Incremental Cache Hit

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Process asset X; re-process without changes
   - **Expected:** Cache hit; processing skipped

### TC-12.2.8.3 Incremental Invalidation

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Change shared texture T1 used by materials M1, M2
   - **Expected:** M1 and M2 reprocessed; unrelated assets skipped

### TC-12.2.9.1 DXC HLSL to DXIL

| # | Requirement |
|---|-------------|
| 1 | R-12.2.9    |

1. **#1** — Valid HLSL test shader
   - **Expected:** Valid DXIL bytecode

### TC-12.2.9.2 DXC HLSL to SPIR-V

| # | Requirement |
|---|-------------|
| 1 | R-12.2.9    |

1. **#1** — Valid HLSL test shader
   - **Expected:** Valid SPIR-V bytecode

### TC-12.2.9.3 MSC DXIL to MSL

| # | Requirement |
|---|-------------|
| 1 | R-12.2.9    |

1. **#1** — Valid DXIL bytecode
   - **Expected:** Valid MSL source output

### TC-12.2.9.4 Shader Reflection

| # | Requirement |
|---|-------------|
| 1 | R-12.2.9    |

1. **#1** — HLSL with 3 texture bindings and 2 CBVs
   - **Expected:** Reflected bindings match source declarations

### TC-12.2.9.5 Shader Dead Code Elimination

| # | Requirement |
|---|-------------|
| 1 | R-12.2.9    |

1. **#1** — HLSL with unreachable branch
   - **Expected:** Dead code eliminated in output bytecode

### TC-12.2.9.6 Shader Error Source Tracing

| # | Requirement |
|---|-------------|
| 1 | R-12.2.9    |

1. **#1** — HLSL with type mismatch error
   - **Expected:** Error maps to originating graph node via source map

### TC-12.2.8.4 Content Hash Determinism

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Same source + config; run twice
   - **Expected:** Identical BLAKE3 hash both runs

### TC-12.2.8.5 Processing Graph Topological Order

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Build graph with edges A->B, B->C, A->C
   - **Expected:** Topological sort: A before B, B before C

## Integration Tests

### TC-12.2.1.I1 Full Pipeline Texture

| # | Requirement |
|---|-------------|
| 1 | R-12.2.1    |

1. **#1** — Import PNG; process for Windows, iOS, Android
   - **Expected:** 3 distinct compressed artifacts in CAS (BC7, ASTC, ETC2)

### TC-12.2.2.I1 Full Pipeline Mesh

| # | Requirement |
|---|-------------|
| 1 | R-12.2.2    |

1. **#1** — Import mesh via DCC plugin
   - **Expected:** LOD chain, meshlets, and optimized index buffer in output

### TC-12.2.7.I1 Full Pipeline Shader

| # | Requirement |
|---|-------------|
| 1 | R-12.2.7    |

1. **#1** — Shader graph to HLSL to all backends
   - **Expected:** Valid DXIL, SPIR-V, and MSL outputs

### TC-12.2.6.I1 Full Pipeline Audio

| # | Requirement |
|---|-------------|
| 1 | R-12.2.6    |

1. **#1** — Import WAV; process as Opus, ADPCM, PCM
   - **Expected:** 3 artifacts in CAS, one per encoding

### TC-12.2.1.I2 Multi-Platform Build

| # | Requirement |
|---|-------------|
| 1 | R-12.2.1    |

1. **#1** — Process 100 assets for 3 platforms
   - **Expected:** Correct format per platform for all 100 assets

### TC-12.2.8.I1 Incremental Rebuild End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Full build of 50 assets; modify 1 texture; rebuild
   - **Expected:** Only dependents reprocessed; others use cache

### TC-12.2.8.I2 Parallel Thread Utilization

| # | Requirement |
|---|-------------|
| 1 | R-12.2.8    |

1. **#1** — Process 1000 assets on 8-core machine
   - **Expected:** Thread utilization >= 90%

### TC-12.2.9.I1 Shader Error to Graph Node Navigation

| # | Requirement |
|---|-------------|
| 1 | R-12.2.9    |

1. **#1** — Introduce error in graph node N5
   - **Expected:** Editor can navigate to node N5 from error message

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
