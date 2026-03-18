# Advanced Rendering (Ray Tracing and GI) Test Cases

Companion test cases for [advanced.md](advanced.md).

## Unit Tests

### TC-2.5.1.1 BLAS Build from Meshlets

| # | Requirement |
|---|-------------|
| 1 | R-2.5.1     |

1. **#1** — 10K meshlets with valid geometry
   - **Expected:** Valid BLAS handle, non-zero GPU address

### TC-2.5.1.2 BLAS Compaction Saves Memory

| # | Requirement |
|---|-------------|
| 1 | R-2.5.1     |

1. **#1** — Built BLAS, then compact
   - **Expected:** Compacted size >= 20% smaller than original

### TC-2.5.1.3 TLAS Refit Preserves Intersections

| # | Requirement |
|---|-------------|
| 1 | R-2.5.1     |

1. **#1** — Move 50% of instances, refit TLAS
   - **Expected:** Ray hits match rebuilt TLAS results

### TC-2.5.1.4 TLAS Rebuild vs Refit Threshold

| # | Requirement |
|---|-------------|
| 1 | R-2.5.1     |
| 2 | R-2.5.1     |

1. **#1** — 31% of instances changed
   - **Expected:** Rebuild triggered (not refit)
2. **#2** — 29% of instances changed
   - **Expected:** Refit used (no rebuild)

### TC-2.5.1.5 Capability Detection

| # | Requirement |
|---|-------------|
| 1 | R-2.5.1     |
| 2 | R-2.5.1     |
| 3 | R-2.5.1     |

1. **#1** — Mock Metal backend feature queries
   - **Expected:** `RtCapabilities` fields match Metal RT support
2. **#2** — Mock D3D12 backend feature queries
   - **Expected:** `RtCapabilities` fields match DXR support
3. **#3** — Mock Vulkan backend feature queries
   - **Expected:** `RtCapabilities` fields match Vulkan RT support

### TC-2.5.14.1 Fallback No RT Selects Voxel GI

| # | Requirement |
|---|-------------|
| 1 | R-2.5.14    |

1. **#1** — `hardware_rt = false`
   - **Expected:** `GiMode::VoxelGi` selected

### TC-2.5.4.1 Fallback Desktop Selects DDGI

| # | Requirement |
|---|-------------|
| 1 | R-2.5.4     |

1. **#1** — Desktop RT caps, default budget
   - **Expected:** `GiMode::Ddgi` selected

### TC-2.5.7.1 Fallback High-End Selects Surfel

| # | Requirement |
|---|-------------|
| 1 | R-2.5.7     |

1. **#1** — High-end RT caps
   - **Expected:** Surfel GI mode available

### TC-2.5.12.1 Denoiser NRD Fallback

| # | Requirement |
|---|-------------|
| 1 | R-2.5.12    |

1. **#1** — `tensor_cores = false`
   - **Expected:** `DenoiserMode::NrdReblur` selected

### TC-2.5.10.1 OMM Skipped Without Support

| # | Requirement |
|---|-------------|
| 1 | R-2.5.10    |

1. **#1** — `opacity_micromaps = false`
   - **Expected:** OMM generation is a no-op

### TC-2.5.11.1 SER Skipped Without Support

| # | Requirement |
|---|-------------|
| 1 | R-2.5.11    |

1. **#1** — `shader_exec_reorder = false`
   - **Expected:** `ShaderExecReorder::new` returns `None`

### TC-2.5.4.2 DDGI Probe Grid Dimensions

| # | Requirement |
|---|-------------|
| 1 | R-2.5.4     |

1. **#1** — world_size=(100,50,100), spacing=2.0
   - **Expected:** Probe count = (50, 25, 50) per axis

### TC-2.5.8.1 ReSTIR Reservoir Memory

| # | Requirement |
|---|-------------|
| 1 | R-2.5.8     |

1. **#1** — 1080p resolution, 32-byte reservoirs
   - **Expected:** Total memory < 2 GB budget

### TC-2.5.5.1 Path Tracer Energy Conservation

| # | Requirement |
|---|-------------|
| 1 | R-2.5.5     |

1. **#1** — White-furnace test, 16 bounces
   - **Expected:** No energy gain or loss (output == input within tolerance)

### TC-2.5.9.1 Production PT GI Fallback

| # | Requirement |
|---|-------------|
| 1 | R-2.5.9     |

1. **#1** — Terminated path trace ray
   - **Expected:** Rasterized GI sampled (not black)

### TC-2.5.14.2 Voxel GI Relight No Revoxelize

| # | Requirement |
|---|-------------|
| 1 | R-2.5.14    |

1. **#1** — Change light direction, static geometry
   - **Expected:** Re-lighting occurs without re-voxelization

### TC-2.5.16.1 Stochastic SSR Half Res

| # | Requirement |
|---|-------------|
| 1 | R-2.5.16    |

1. **#1** — Render resolution 1920x1080
   - **Expected:** Reflection buffer dimensions = 960x540

### TC-2.5.6.1 RT Subsurface Thickness

| # | Requirement |
|---|-------------|
| 1 | R-2.5.6     |
| 2 | R-2.5.6     |

1. **#1** — Material thickness = 0.01
   - **Expected:** High transmission intensity
2. **#2** — Material thickness = 1.0
   - **Expected:** Near-zero transmission (exponential decay)

## Integration Tests

### TC-2.5.3.I1 Cornell Box Color Bleed

| # | Requirement      |
|---|------------------|
| 1 | R-2.5.3, R-2.5.4 |

1. **#1** — Cornell box with red/green walls, white surfaces
   - **Expected:** Color bleeding visible; PSNR > 30 dB vs reference

### TC-2.5.2.I1 Hybrid Reflection Offscreen

| # | Requirement |
|---|-------------|
| 1 | R-2.5.2     |

1. **#1** — Glossy floor reflecting off-screen object
   - **Expected:** SSR fails, RT produces correct reflection

### TC-2.5.2.I2 Hybrid Reflection Roughness Handoff

| # | Requirement |
|---|-------------|
| 1 | R-2.5.2     |

1. **#1** — Roughness increased past threshold
   - **Expected:** RT rays activate for high-roughness reflections

### TC-2.5.4.I1 DDGI Dynamic Light Response

| # | Requirement |
|---|-------------|
| 1 | R-2.5.4     |

1. **#1** — Move light source at runtime
   - **Expected:** DDGI probes update within 2 seconds

### TC-2.5.7.I1 Surfel GI Open World Scaling

| # | Requirement |
|---|-------------|
| 1 | R-2.5.7     |

1. **#1** — Double environment size
   - **Expected:** GPU cost within 10% variance of original

### TC-2.5.8.I1 ReSTIR 5000 Lights

| # | Requirement |
|---|-------------|
| 1 | R-2.5.8     |

1. **#1** — Scene with 5K lights
   - **Expected:** All lights contribute; cost independent of count

### TC-2.5.5.I1 Path Tracer Convergence

| # | Requirement |
|---|-------------|
| 1 | R-2.5.5     |

1. **#1** — 4096 spp Cornell box
   - **Expected:** < 1% relative error vs analytical reference

### TC-2.5.12.I1 Neural Denoise Temporal Stability

| # | Requirement |
|---|-------------|
| 1 | R-2.5.12    |

1. **#1** — 120 frames of camera motion
   - **Expected:** No ghosting or flickering artifacts

### TC-2.5.10.I1 OMM Vegetation Throughput

| # | Requirement |
|---|-------------|
| 1 | R-2.5.10    |

1. **#1** — Vegetation scene with OMM enabled
   - **Expected:** >= 30% RT intersection throughput improvement

### TC-2.5.13.I1 RT Lens Flare Ghost Positions

| # | Requirement |
|---|-------------|
| 1 | R-2.5.13    |

1. **#1** — Bright source, 5-element lens
   - **Expected:** Ghost positions within 5% angular error of analytical prediction

### TC-2.5.0.I1 Full Tier Cascade

| # | Requirement              |
|---|--------------------------|
| 1 | R-2.5.1 through R-2.5.16 |

1. **#1** — Walk through each platform tier
   - **Expected:** Correct passes active, fallbacks engage per tier

### TC-2.5.1.I1 GCD RT Command Buffer

| # | Requirement |
|---|-------------|
| 1 | R-2.5.1     |

1. **#1** — macOS Metal RT build commands
   - **Expected:** Commands submit and complete via GCD dispatch

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
