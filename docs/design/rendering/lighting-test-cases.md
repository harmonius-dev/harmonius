# Lighting System Test Cases

Companion test cases for [lighting.md](lighting.md).

## Unit Tests

### TC-2.4.1.1 Cluster Grid Dimensions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Viewport 1920x1080, tile_size=64, 24 depth slices | Tile count = 30x17, total clusters = 30x17x24 | R-2.4.1 |

### TC-2.4.1.2 Cluster Assignment 500 Lights

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 point lights, random positions | Every fragment reads correct cluster light list | NFR-2.4.1 |

### TC-2.4.1.3 Cluster Depth Slice Distribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Logarithmic depth slicing formula | Slice boundaries match formula output | R-2.4.1 |

### TC-2.4.11.1 Shadow Atlas Alloc Release

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate and release tiles, 10K cycles | No memory leaks | R-2.4.11 |

### TC-2.4.11.2 Shadow Atlas LRU Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill atlas to capacity | LRU eviction frees oldest tiles | NFR-2.4.2 |

### TC-2.4.11.3 Shadow Atlas Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Total shadow atlas allocation | Stays under 256 MB budget | NFR-2.4.2 |

### TC-2.4.11.4 CSM Split Computation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | lambda=0 (linear) | Cascade splits match linear formula | R-2.4.11 |
| 2 | lambda=0.5 (blend) | Cascade splits match blended formula | R-2.4.11 |
| 3 | lambda=1.0 (logarithmic) | Cascade splits match logarithmic formula | R-2.4.11 |

### TC-2.4.11.5 CSM Temporal Stabilization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rotate light 360 degrees | Shadow texel movement < 1 texel | R-2.4.11 |

### TC-2.4.12.1 PCF Filter 4 Tap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shadow with 4-tap PCF | Soft edge width matches kernel radius | R-2.4.12 |

### TC-2.4.12.2 PCSS Penumbra Scales

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vary blocker distance | Penumbra width proportional to blocker distance | R-2.4.12 |

### TC-2.4.13.1 SSAO Half Res Output

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Viewport 1920x1080 | SSAO texture = 960x540 | R-2.4.13 |

### TC-2.4.13.2 GTAO Bent Normals

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | GTAO output | Valid bent normal vectors (unit length) | R-2.4.13 |

### TC-2.4.22.1 IES Parse Valid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reference IES file | Correct angles and candela values | R-2.4.22 |

### TC-2.4.22.2 IES Parse Invalid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Malformed IES data | `IesParseError` returned | R-2.4.22 |

### TC-2.4.20.1 Area Light LTC Energy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | LTC integration for area light | Energy conserved (output <= input) | R-2.4.20 |

### TC-2.4.21.1 Sky Light Irradiance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Irradiance cubemap from sky | Dominant color matches sky color | R-2.4.21 |

### TC-2.4.23.1 Light Function Scroll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Animate UV offset over 60 frames | Light pattern moves | R-2.4.23 |

### TC-2.4.1.4 GPU Light Pack Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `GpuLight` struct | Size = 64 bytes (cache line) | R-2.4.1 |

### TC-2.4.1.5 Directional Light Component

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity with DirectionalLight | ECS query finds it | R-2.4.1 |

### TC-2.4.1.6 Point Light Radius Cull

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Point light outside frustum | Culled | R-2.4.1 |
| 2 | Point light inside frustum | Retained | R-2.4.1 |

### TC-2.4.14.1 VSM Page Alloc Evict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate beyond page pool capacity | LRU eviction of oldest pages | R-2.4.14 |

### TC-2.4.18.1 OIT Moment Precision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 overlapping transparent planes | PSNR >= 30 dB vs sorted reference | R-2.4.18 |

### TC-2.4.15.1 Contact Shadow Step Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Each platform tier | Step count matches tier config | R-2.4.15 |

### TC-2.4.17.1 Capsule Shadow Pose Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Animate skeleton over 60 frames | Capsule shadows track pose | R-2.4.17 |

### TC-2.4.10.1 Stochastic Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2000 lights, 16 frames temporal accumulation | Noise below threshold | R-2.4.10 |

## Integration Tests

### TC-2.4.1.I1 Forward Plus 500 Lights

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene with 500 lights | Sub-linear frame time scaling | NFR-2.4.1 |

### TC-2.4.2.I1 Deferred GBuffer Layout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deferred G-buffer render | Targets match spec (albedo, normal, motion, depth) | R-2.4.2 |

### TC-2.4.2.I2 Deferred Matches Forward

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same scene, deferred vs forward | PSNR within 40 dB | R-2.4.2 |

### TC-2.4.11.I1 Shadow Atlas 20 Lights

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 CSM cascades + 20 shadow lights | VRAM < 256 MB | NFR-2.4.2 |

### TC-2.4.0.I1 Tier Fallback Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile tier active | Area lights fall back to point; IES is 1D 64; OIT disabled | R-2.4.1 through R-2.4.23 |

### TC-2.4.0.I2 Tier Fallback Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch tier, docked mode | PCSS active | R-2.4.1 through R-2.4.23 |
| 2 | Switch tier, handheld mode | PCF active (not PCSS) | R-2.4.1 through R-2.4.23 |

### TC-2.4.0.I3 Tier Fallback Desktop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Desktop tier active | GTAO, PCSS, VSM all enabled | R-2.4.1 through R-2.4.23 |

### TC-2.4.21.I1 Sky Refilter On TOD

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change sun position | Irradiance map updates | R-2.4.21 |

### TC-2.4.14.I1 VSM Consistent Detail

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shadow texel density near vs far | Approximately constant texel density | R-2.4.14 |

### TC-2.4.16.I1 SDF CSM Blend

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shadow transition between CSM and SDF ranges | Seamless transition | R-2.4.16 |

### TC-2.4.1.I2 Light Extraction Parallel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 lights extracted in parallel | No data races (ThreadSanitizer clean) | R-2.4.1 |

### TC-2.4.11.I2 Cross Backend Shadow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same scene on Metal, D3D12, Vulkan | Shadow output within PSNR 35 dB | R-2.4.11 |

## Benchmarks

### TC-2.4.1.B1 Cluster Assignment

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p, 500 lights | GPU time | < 0.5 ms | NFR-2.4.1 |

### TC-2.4.3.B1 BRDF Eval

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M fragments | GPU time | < 0.1 ms | NFR-2.4.3 |

### TC-2.4.11.B1 CSM 4-Cascade Render

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4 cascades, 2048 each | GPU time | < 2.0 ms | R-2.4.11 |

### TC-2.4.13.B1 GTAO Full Res

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 1.0 ms | R-2.4.13 |

### TC-2.4.13.B2 SSAO Half Res

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.3 ms | R-2.4.13 |

### TC-2.4.15.B1 Contact Shadows

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16-step, 1080p | GPU time | < 0.3 ms | R-2.4.15 |

### TC-2.4.11.B2 Shadow Atlas Allocation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 lights | CPU time | < 0.05 ms | NFR-2.4.2 |

### TC-2.4.1.B2 Light Extraction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 lights | CPU time | < 0.1 ms | NFR-2.4.1 |

### TC-2.4.20.B1 LTC Area Light Eval

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 area lights | GPU time | < 0.2 ms | R-2.4.20 |

### TC-2.4.21.B1 Sky IBL Refilter

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256x256 cubemap | GPU time | < 5.0 ms | R-2.4.21 |
