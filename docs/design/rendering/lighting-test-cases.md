# Lighting System Test Cases

Companion test cases for [lighting.md](lighting.md).

## Unit Tests

### TC-2.4.1.1 Cluster Grid Dimensions

| # | Requirement |
|---|-------------|
| 1 | R-2.4.1     |

1. **#1** — Viewport 1920x1080, tile_size=64, 24 depth slices
   - **Expected:** Tile count = 30x17, total clusters = 30x17x24

### TC-2.4.1.2 Cluster Assignment 500 Lights

| # | Requirement |
|---|-------------|
| 1 | NFR-2.4.1   |

1. **#1** — 500 point lights, random positions
   - **Expected:** Every fragment reads correct cluster light list

### TC-2.4.1.3 Cluster Depth Slice Distribution

| # | Requirement |
|---|-------------|
| 1 | R-2.4.1     |

1. **#1** — Logarithmic depth slicing formula
   - **Expected:** Slice boundaries match formula output

### TC-2.4.11.1 Shadow Atlas Alloc Release

| # | Requirement |
|---|-------------|
| 1 | R-2.4.11    |

1. **#1** — Allocate and release tiles, 10K cycles
   - **Expected:** No memory leaks

### TC-2.4.11.2 Shadow Atlas LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | NFR-2.4.2   |

1. **#1** — Fill atlas to capacity
   - **Expected:** LRU eviction frees oldest tiles

### TC-2.4.11.3 Shadow Atlas Budget

| # | Requirement |
|---|-------------|
| 1 | NFR-2.4.2   |

1. **#1** — Total shadow atlas allocation
   - **Expected:** Stays under 256 MB budget

### TC-2.4.11.4 CSM Split Computation

| # | Requirement |
|---|-------------|
| 1 | R-2.4.11    |
| 2 | R-2.4.11    |
| 3 | R-2.4.11    |

1. **#1** — lambda=0 (linear)
   - **Expected:** Cascade splits match linear formula
2. **#2** — lambda=0.5 (blend)
   - **Expected:** Cascade splits match blended formula
3. **#3** — lambda=1.0 (logarithmic)
   - **Expected:** Cascade splits match logarithmic formula

### TC-2.4.11.5 CSM Temporal Stabilization

| # | Requirement |
|---|-------------|
| 1 | R-2.4.11    |

1. **#1** — Rotate light 360 degrees
   - **Expected:** Shadow texel movement < 1 texel

### TC-2.4.12.1 PCF Filter 4 Tap

| # | Requirement |
|---|-------------|
| 1 | R-2.4.12    |

1. **#1** — Shadow with 4-tap PCF
   - **Expected:** Soft edge width matches kernel radius

### TC-2.4.12.2 PCSS Penumbra Scales

| # | Requirement |
|---|-------------|
| 1 | R-2.4.12    |

1. **#1** — Vary blocker distance
   - **Expected:** Penumbra width proportional to blocker distance

### TC-2.4.13.1 SSAO Half Res Output

| # | Requirement |
|---|-------------|
| 1 | R-2.4.13    |

1. **#1** — Viewport 1920x1080
   - **Expected:** SSAO texture = 960x540

### TC-2.4.13.2 GTAO Bent Normals

| # | Requirement |
|---|-------------|
| 1 | R-2.4.13    |

1. **#1** — GTAO output
   - **Expected:** Valid bent normal vectors (unit length)

### TC-2.4.22.1 IES Parse Valid

| # | Requirement |
|---|-------------|
| 1 | R-2.4.22    |

1. **#1** — Reference IES file
   - **Expected:** Correct angles and candela values

### TC-2.4.22.2 IES Parse Invalid

| # | Requirement |
|---|-------------|
| 1 | R-2.4.22    |

1. **#1** — Malformed IES data
   - **Expected:** `IesParseError` returned

### TC-2.4.20.1 Area Light LTC Energy

| # | Requirement |
|---|-------------|
| 1 | R-2.4.20    |

1. **#1** — LTC integration for area light
   - **Expected:** Energy conserved (output <= input)

### TC-2.4.21.1 Sky Light Irradiance

| # | Requirement |
|---|-------------|
| 1 | R-2.4.21    |

1. **#1** — Irradiance cubemap from sky
   - **Expected:** Dominant color matches sky color

### TC-2.4.23.1 Light Function Scroll

| # | Requirement |
|---|-------------|
| 1 | R-2.4.23    |

1. **#1** — Animate UV offset over 60 frames
   - **Expected:** Light pattern moves

### TC-2.4.1.4 GPU Light Pack Alignment

| # | Requirement |
|---|-------------|
| 1 | R-2.4.1     |

1. **#1** — `GpuLight` struct
   - **Expected:** Size = 64 bytes (cache line)

### TC-2.4.1.5 Directional Light Component

| # | Requirement |
|---|-------------|
| 1 | R-2.4.1     |

1. **#1** — Entity with DirectionalLight
   - **Expected:** ECS query finds it

### TC-2.4.1.6 Point Light Radius Cull

| # | Requirement |
|---|-------------|
| 1 | R-2.4.1     |
| 2 | R-2.4.1     |

1. **#1** — Point light outside frustum
   - **Expected:** Culled
2. **#2** — Point light inside frustum
   - **Expected:** Retained

### TC-2.4.14.1 VSM Page Alloc Evict

| # | Requirement |
|---|-------------|
| 1 | R-2.4.14    |

1. **#1** — Allocate beyond page pool capacity
   - **Expected:** LRU eviction of oldest pages

### TC-2.4.18.1 OIT Moment Precision

| # | Requirement |
|---|-------------|
| 1 | R-2.4.18    |

1. **#1** — 10 overlapping transparent planes
   - **Expected:** PSNR >= 30 dB vs sorted reference

### TC-2.4.15.1 Contact Shadow Step Count

| # | Requirement |
|---|-------------|
| 1 | R-2.4.15    |

1. **#1** — Each platform tier
   - **Expected:** Step count matches tier config

### TC-2.4.17.1 Capsule Shadow Pose Update

| # | Requirement |
|---|-------------|
| 1 | R-2.4.17    |

1. **#1** — Animate skeleton over 60 frames
   - **Expected:** Capsule shadows track pose

### TC-2.4.10.1 Stochastic Convergence

| # | Requirement |
|---|-------------|
| 1 | R-2.4.10    |

1. **#1** — 2000 lights, 16 frames temporal accumulation
   - **Expected:** Noise below threshold

## Integration Tests

### TC-2.4.1.I1 Forward Plus 500 Lights

| # | Requirement |
|---|-------------|
| 1 | NFR-2.4.1   |

1. **#1** — Scene with 500 lights
   - **Expected:** Sub-linear frame time scaling

### TC-2.4.2.I1 Deferred GBuffer Layout

| # | Requirement |
|---|-------------|
| 1 | R-2.4.2     |

1. **#1** — Deferred G-buffer render
   - **Expected:** Targets match spec (albedo, normal, motion, depth)

### TC-2.4.2.I2 Deferred Matches Forward

| # | Requirement |
|---|-------------|
| 1 | R-2.4.2     |

1. **#1** — Same scene, deferred vs forward
   - **Expected:** PSNR within 40 dB

### TC-2.4.11.I1 Shadow Atlas 20 Lights

| # | Requirement |
|---|-------------|
| 1 | NFR-2.4.2   |

1. **#1** — 4 CSM cascades + 20 shadow lights
   - **Expected:** VRAM < 256 MB

### TC-2.4.0.I1 Tier Fallback Mobile

| # | Requirement              |
|---|--------------------------|
| 1 | R-2.4.1 through R-2.4.23 |

1. **#1** — Mobile tier active
   - **Expected:** Area lights fall back to point; IES is 1D 64; OIT disabled

### TC-2.4.0.I2 Tier Fallback Switch

| # | Requirement              |
|---|--------------------------|
| 1 | R-2.4.1 through R-2.4.23 |
| 2 | R-2.4.1 through R-2.4.23 |

1. **#1** — Switch tier, docked mode
   - **Expected:** PCSS active
2. **#2** — Switch tier, handheld mode
   - **Expected:** PCF active (not PCSS)

### TC-2.4.0.I3 Tier Fallback Desktop

| # | Requirement              |
|---|--------------------------|
| 1 | R-2.4.1 through R-2.4.23 |

1. **#1** — Desktop tier active
   - **Expected:** GTAO, PCSS, VSM all enabled

### TC-2.4.21.I1 Sky Refilter On TOD

| # | Requirement |
|---|-------------|
| 1 | R-2.4.21    |

1. **#1** — Change sun position
   - **Expected:** Irradiance map updates

### TC-2.4.14.I1 VSM Consistent Detail

| # | Requirement |
|---|-------------|
| 1 | R-2.4.14    |

1. **#1** — Shadow texel density near vs far
   - **Expected:** Approximately constant texel density

### TC-2.4.16.I1 SDF CSM Blend

| # | Requirement |
|---|-------------|
| 1 | R-2.4.16    |

1. **#1** — Shadow transition between CSM and SDF ranges
   - **Expected:** Seamless transition

### TC-2.4.1.I2 Light Extraction Parallel

| # | Requirement |
|---|-------------|
| 1 | R-2.4.1     |

1. **#1** — 1000 lights extracted in parallel
   - **Expected:** No data races (ThreadSanitizer clean)

### TC-2.4.11.I2 Cross Backend Shadow

| # | Requirement |
|---|-------------|
| 1 | R-2.4.11    |

1. **#1** — Same scene on Metal, D3D12, Vulkan
   - **Expected:** Shadow output within PSNR 35 dB

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
