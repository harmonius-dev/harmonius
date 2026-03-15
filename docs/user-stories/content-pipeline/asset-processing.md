# User Stories -- 12.2 Asset Processing

## US-12.2.1 Compress Textures for Every Target Platform Automatically

**As a** designer (P-5), **I want** imported textures automatically compressed to GPU-native formats
(BC7 for desktop, ASTC for mobile/Apple Silicon, ETC2 as fallback) based on import presets and
per-platform overrides, **so that** textures are optimally compressed without manual per-platform
configuration.

## US-12.2.2 Generate LOD Chains Without Manual Artist Work

**As a** designer (P-5), **I want** automatic LOD chain generation via edge-collapse simplification
with configurable error thresholds, **so that** every mesh has performance LODs without requiring
manual decimation by artists.

## US-12.2.3 Build Meshlets for GPU-Driven Rendering

**As an** engine developer (P-26), **I want** automatic meshlet partitioning (64 vertices, 124
triangles) with precomputed AABB and normal cone bounds, **so that** the GPU-driven rendering
pipeline can perform per-meshlet culling.

## US-12.2.4 Optimize Vertex Cache and Overdraw per Meshlet

**As an** engine developer (P-26), **I want** triangle and vertex reordering within meshlets to
maximize post-transform cache hit rates and minimize overdraw, **so that** GPU rendering performance
is optimal without manual mesh optimization.

## US-12.2.5 Generate Lightmap UVs Automatically

**As a** designer (P-5), **I want** automatic non-overlapping lightmap UV atlas generation with
uniform texel density and chart packing, **so that** static geometry has correct lightmap UVs
without manual unwrapping.

## US-12.2.6 Encode Audio for Runtime with Correct Presets

**As a** designer (P-5), **I want** imported audio encoded to Opus (voice/music), ADPCM (short SFX),
or raw PCM (latency-critical) based on import presets, **so that** audio assets are compressed
appropriately for their runtime usage pattern.

## US-12.2.7 Generate Clean HLSL from Shader Graphs

**As a** game developer (P-15), **I want** the shader graph compiler to produce valid,
human-readable HLSL source files with comments mapping sections to graph nodes, **so that** I can
debug generated shaders in any IDE with full HLSL Tools support.

## US-12.2.8 Compile Shaders to All GPU Backends

**As a** DevOps engineer (P-16), **I want** the build pipeline to compile HLSL to DXIL (D3D12),
SPIR-V (Vulkan), and MSL (Metal) using DXC and Metal Shader Converter, with bytecode cached by HLSL
source hash, **so that** every platform gets optimized shader bytecode without manual per-platform
steps.

## US-12.2.9 Track Asset Dependencies for Incremental Rebuilds

**As a** DevOps engineer (P-16), **I want** directed acyclic dependency graphs tracking all asset
cross-references with circular reference detection, **so that** modifying a shared texture rebuilds
only dependent materials and CI builds complete in minutes.

## US-12.2.10 Navigate from Shader Errors to Graph Nodes

**As a** game developer (P-15), **I want** shader compilation errors to include both the HLSL line
number and the originating graph node ID, **so that** clicking an error navigates directly to the
visual graph node that caused it.

## US-12.2.11 Author Custom Shader Nodes as HLSL Snippets

**As a** game developer (P-15), **I want** to write custom shader graph nodes as HLSL function
snippets with typed inputs and outputs that appear in the graph editor palette, **so that** I can
extend the material system without modifying the engine.

## US-12.2.12 Verify Texture Compression Quality per Platform

**As an** engine tester (P-27), **I want** automated tests that compress reference textures with
BC7, ASTC, and ETC2 and compare against quality baselines (PSNR thresholds), **so that** compression
quality regressions are caught per platform in CI.

## US-12.2.13 Verify LOD Generation Preserves Silhouettes

**As an** engine tester (P-27), **I want** tests that generate LOD chains for reference meshes and
verify triangle count ratios and bounding box deviations stay within configured thresholds, **so
that** LOD generation does not produce visually broken meshes.

## US-12.2.14 Verify Shader Compilation Succeeds on All Backends

**As an** engine tester (P-27), **I want** CI tests that compile all shader graph permutations to
DXIL, SPIR-V, and MSL and verify zero compilation errors, **so that** shader compilation regressions
are caught before they reach artists.

## US-12.2.15 Benchmark Asset Processing Throughput

**As an** engine tester (P-27), **I want** benchmarks measuring texture compression, LOD generation,
and meshlet building throughput on reference hardware, **so that** processing performance
regressions are detected and build times stay within budget.

## US-12.2.16 Only Compile Reachable Shader Variants

**As a** DevOps engineer (P-16), **I want** the shader graph compiler to identify and compile only
reachable variants through static analysis of material parameter usage, **so that** permutation
explosion does not cause build time or binary size bloat.

## US-12.2.17 Configure Texture Compression Quality per Import Preset

**As a** game developer (P-15), **I want** configurable quality levels per texture import preset
with per-platform format override tables, **so that** I can balance visual quality against texture
size for each asset category (hero textures vs. background fill).

## US-12.2.18 Verify Audio Encoding Latency Meets Playback Requirements

**As an** engine tester (P-27), **I want** tests that decode Opus, ADPCM, and PCM encoded audio and
measure decode latency, verifying ADPCM decodes within 1 ms for short SFX and PCM decodes instantly,
**so that** runtime playback latency targets are met.

## US-12.2.19 Verify Meshlet Bounds Enable Correct GPU Culling

**As an** engine tester (P-27), **I want** tests that render reference scenes with meshlet culling
enabled and verify no visible meshlets are incorrectly culled, **so that** precomputed AABB and
normal cone bounds are validated against the GPU culling shader.

## US-12.2.20 Verify Lightmap UV Atlas Has No Overlapping Charts

**As an** engine tester (P-27), **I want** automated tests that generate lightmap UV atlases for
reference meshes and verify no chart overlaps exist and texel density variance is within threshold,
**so that** lightmap baking produces correct results.
