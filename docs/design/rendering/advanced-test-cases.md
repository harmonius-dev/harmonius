# Advanced Rendering (Ray Tracing and GI) Test Cases

Companion test cases for [advanced.md](advanced.md).

## Unit Tests

### TC-2.5.1.1 BLAS Build from Meshlets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K meshlets with valid geometry | Valid BLAS handle, non-zero GPU address | R-2.5.1 |

### TC-2.5.1.2 BLAS Compaction Saves Memory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Built BLAS, then compact | Compacted size >= 20% smaller than original | R-2.5.1 |

### TC-2.5.1.3 TLAS Refit Preserves Intersections

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move 50% of instances, refit TLAS | Ray hits match rebuilt TLAS results | R-2.5.1 |

### TC-2.5.1.4 TLAS Rebuild vs Refit Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 31% of instances changed | Rebuild triggered (not refit) | R-2.5.1 |
| 2 | 29% of instances changed | Refit used (no rebuild) | R-2.5.1 |

### TC-2.5.1.5 Capability Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mock Metal backend feature queries | `RtCapabilities` fields match Metal RT support | R-2.5.1 |
| 2 | Mock D3D12 backend feature queries | `RtCapabilities` fields match DXR support | R-2.5.1 |
| 3 | Mock Vulkan backend feature queries | `RtCapabilities` fields match Vulkan RT support | R-2.5.1 |

### TC-2.5.14.1 Fallback No RT Selects Voxel GI

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `hardware_rt = false` | `GiMode::VoxelGi` selected | R-2.5.14 |

### TC-2.5.4.1 Fallback Desktop Selects DDGI

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Desktop RT caps, default budget | `GiMode::Ddgi` selected | R-2.5.4 |

### TC-2.5.7.1 Fallback High-End Selects Surfel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | High-end RT caps | Surfel GI mode available | R-2.5.7 |

### TC-2.5.12.1 Denoiser NRD Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `tensor_cores = false` | `DenoiserMode::NrdReblur` selected | R-2.5.12 |

### TC-2.5.10.1 OMM Skipped Without Support

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `opacity_micromaps = false` | OMM generation is a no-op | R-2.5.10 |

### TC-2.5.11.1 SER Skipped Without Support

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `shader_exec_reorder = false` | `ShaderExecReorder::new` returns `None` | R-2.5.11 |

### TC-2.5.4.2 DDGI Probe Grid Dimensions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | world_size=(100,50,100), spacing=2.0 | Probe count = (50, 25, 50) per axis | R-2.5.4 |

### TC-2.5.8.1 ReSTIR Reservoir Memory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1080p resolution, 32-byte reservoirs | Total memory < 2 GB budget | R-2.5.8 |

### TC-2.5.5.1 Path Tracer Energy Conservation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | White-furnace test, 16 bounces | No energy gain or loss (output == input within tolerance) | R-2.5.5 |

### TC-2.5.9.1 Production PT GI Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Terminated path trace ray | Rasterized GI sampled (not black) | R-2.5.9 |

### TC-2.5.14.2 Voxel GI Relight No Revoxelize

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change light direction, static geometry | Re-lighting occurs without re-voxelization | R-2.5.14 |

### TC-2.5.16.1 Stochastic SSR Half Res

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render resolution 1920x1080 | Reflection buffer dimensions = 960x540 | R-2.5.16 |

### TC-2.5.6.1 RT Subsurface Thickness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Material thickness = 0.01 | High transmission intensity | R-2.5.6 |
| 2 | Material thickness = 1.0 | Near-zero transmission (exponential decay) | R-2.5.6 |

## Integration Tests

### TC-2.5.3.I1 Cornell Box Color Bleed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cornell box with red/green walls, white surfaces | Color bleeding visible; PSNR > 30 dB vs reference | R-2.5.3, R-2.5.4 |

### TC-2.5.2.I1 Hybrid Reflection Offscreen

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Glossy floor reflecting off-screen object | SSR fails, RT produces correct reflection | R-2.5.2 |

### TC-2.5.2.I2 Hybrid Reflection Roughness Handoff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Roughness increased past threshold | RT rays activate for high-roughness reflections | R-2.5.2 |

### TC-2.5.4.I1 DDGI Dynamic Light Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move light source at runtime | DDGI probes update within 2 seconds | R-2.5.4 |

### TC-2.5.7.I1 Surfel GI Open World Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Double environment size | GPU cost within 10% variance of original | R-2.5.7 |

### TC-2.5.8.I1 ReSTIR 5000 Lights

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene with 5K lights | All lights contribute; cost independent of count | R-2.5.8 |

### TC-2.5.5.I1 Path Tracer Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4096 spp Cornell box | < 1% relative error vs analytical reference | R-2.5.5 |

### TC-2.5.12.I1 Neural Denoise Temporal Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 120 frames of camera motion | No ghosting or flickering artifacts | R-2.5.12 |

### TC-2.5.10.I1 OMM Vegetation Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vegetation scene with OMM enabled | >= 30% RT intersection throughput improvement | R-2.5.10 |

### TC-2.5.13.I1 RT Lens Flare Ghost Positions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bright source, 5-element lens | Ghost positions within 5% angular error of analytical prediction | R-2.5.13 |

### TC-2.5.0.I1 Full Tier Cascade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk through each platform tier | Correct passes active, fallbacks engage per tier | R-2.5.1 through R-2.5.16 |

### TC-2.5.1.I1 GCD RT Command Buffer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | macOS Metal RT build commands | Commands submit and complete via GCD dispatch | R-2.5.1 |

## Benchmarks

### TC-2.5.1.B1 BLAS Build

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10K meshlets, full rebuild | GPU time | < 50 ms | NFR-2.5.1 |

### TC-2.5.1.B2 BLAS Incremental

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10% meshlets changed | GPU time | < 2 ms | NFR-2.5.1 |

### TC-2.5.1.B3 TLAS Refit

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10K instances, refit | GPU time | < 1 ms | R-2.5.1 |

### TC-2.5.0.B1 Combined RT Budget

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p, default RT settings | Combined GPU time | <= 8 ms | NFR-2.5.2 |

### TC-2.5.4.B1 DDGI Probe Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64 rays/probe | GPU time | < 2 ms | R-2.5.4 |

### TC-2.5.7.B1 Surfel GI Pipeline

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full surfel GI pipeline | GPU time | < 4 ms | R-2.5.7 |

### TC-2.5.8.B1 ReSTIR DI

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5K lights | GPU time | < 3 ms | R-2.5.8 |

### TC-2.5.12.B1 Denoiser Per Signal

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single RT signal denoising | GPU time | < 1 ms | NFR-2.5.3 |

### TC-2.5.14.B1 Voxel GI Relight

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Light direction change, static scene | GPU time | < 2 ms | R-2.5.14 |

### TC-2.5.16.B1 Stochastic SSR

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Half-res stochastic SSR | GPU time | < 1.5 ms | R-2.5.16 |

### TC-2.5.9.B1 Path Tracer 1 SPP

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1 sample per pixel | GPU time | < 16 ms | R-2.5.9 |
