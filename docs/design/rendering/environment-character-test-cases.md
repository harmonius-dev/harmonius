# Environment and Character Rendering Test Cases

Companion test cases for [environment-character.md](environment-character.md).

## Unit Tests

### TC-2.6.1.1 TAA Jitter Halton

| # | Requirement |
|---|-------------|
| 1 | R-2.6.1     |

1. **#1** — Halton(2,3) sequence, 16 frames
   - **Expected:** 16 distinct sub-pixel offsets, no repeats

### TC-2.6.1.2 TAA Clamp MinMax

| # | Requirement |
|---|-------------|
| 1 | R-2.6.1     |

1. **#1** — 3x3 neighborhood, history sample outside min/max
   - **Expected:** History sample rejected (clamped)

### TC-2.6.1.3 TAA Clamp Variance

| # | Requirement |
|---|-------------|
| 1 | R-2.6.1     |

1. **#1** — High-variance 3x3 neighborhood
   - **Expected:** Variance clamp box tighter than MinMax clamp

### TC-2.6.3.1 FXAA Edge Detect

| # | Requirement |
|---|-------------|
| 1 | R-2.6.3     |

1. **#1** — Black/white boundary
   - **Expected:** FXAA detects and blends the edge

### TC-2.6.3.2 SMAA Three Pass

| # | Requirement |
|---|-------------|
| 1 | R-2.6.3     |

1. **#1** — High-contrast scene
   - **Expected:** Edge, blend weight, resolve passes execute in sequence; anti-aliased output

### TC-2.6.4.1 MSAA Forward Only

| # | Requirement |
|---|-------------|
| 1 | R-2.6.4     |
| 2 | R-2.6.4     |

1. **#1** — Forward path, MSAA 2x/4x
   - **Expected:** MSAA activates
2. **#2** — Deferred path, MSAA 2x
   - **Expected:** MSAA rejected

### TC-2.6.2.1 TSR Internal Res

| # | Requirement |
|---|-------------|
| 1 | R-2.6.2     |

1. **#1** — Each quality mode (Performance, Balanced, Quality, Ultra)
   - **Expected:** Correct internal dimensions per mode

### TC-2.6.6.1 Upscaler Fallback

| # | Requirement |
|---|-------------|
| 1 | R-2.6.6     |

1. **#1** — No vendor SDK available
   - **Expected:** `VendorUpscaler::detect` returns `Tsr` variant

### TC-2.6.5.1 Checkerboard Pattern

| # | Requirement |
|---|-------------|
| 1 | R-2.6.5     |

1. **#1** — Two consecutive frames
   - **Expected:** Alternating pixel pattern covers all pixels

### TC-2.6.7.1 Frame Gen Doubling

| # | Requirement |
|---|-------------|
| 1 | R-2.6.7     |

1. **#1** — `max_generated_frames=1`
   - **Expected:** 2x output frames per rendered frame

### TC-2.7.1.1 Sky LUT Dimensions

| # | Requirement |
|---|-------------|
| 1 | R-2.7.1     |

1. **#1** — Each platform tier
   - **Expected:** Transmittance, multi-scatter, sky-view, aerial LUTs have correct dimensions

### TC-2.7.2.1 Froxel Grid Setup

| # | Requirement |
|---|-------------|
| 1 | R-2.7.2     |

1. **#1** — `froxel_resolution = (160, 90, 64)`
   - **Expected:** Grid dimensions match 160x90x64

### TC-2.7.5.1 Fog Model Exp

| # | Requirement |
|---|-------------|
| 1 | R-2.7.5     |

1. **#1** — density=0.1, distance=10.0
   - **Expected:** Attenuation = exp(-0.1 * 10) = 0.3679

### TC-2.7.5.2 Fog Model Exp2

| # | Requirement |
|---|-------------|
| 1 | R-2.7.5     |

1. **#1** — density=0.1, distance=10.0
   - **Expected:** Attenuation = exp(-(0.1 * 10)^2) = 0.3679

### TC-11.2.1.1 Decal Channel Mask

| # | Requirement |
|---|-------------|
| 1 | R-11.2.1    |

1. **#1** — Decal writes albedo only
   - **Expected:** Normal, roughness, metallic unchanged in G-buffer

### TC-11.2.4.1 Decal Priority Order

| # | Requirement |
|---|-------------|
| 1 | R-11.2.4    |

1. **#1** — Two decals at same position, different priorities
   - **Expected:** Higher-priority decal overwrites lower

### TC-11.2.1.2 Decal Angle Fade

| # | Requirement |
|---|-------------|
| 1 | R-11.2.1    |

1. **#1** — Decal projected at 80-degree angle (beyond threshold)
   - **Expected:** Opacity attenuated to near-zero

### TC-11.2.4.2 Decal Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-11.2.4    |

1. **#1** — Spawn decal, advance time
   - **Expected:** Fade-in, sustain, dissolve-out match configured durations

### TC-2.7.2.2 Detail Poisson

| # | Requirement |
|---|-------------|
| 1 | R-2.7.2     |

1. **#1** — Poisson disk distribution
   - **Expected:** Minimum spacing guaranteed between all instances

### TC-2.8.3.1 LOD Crossfade

| # | Requirement |
|---|-------------|
| 1 | R-2.8.3     |

1. **#1** — `LodTransitionComponent::blend_factor` over `dither_duration`
   - **Expected:** Transitions linearly from 0.0 to 1.0

### TC-2.7.10.1 Weather Transition

| # | Requirement |
|---|-------------|
| 1 | R-2.7.10    |

1. **#1** — Transition Clear to Rain
   - **Expected:** Fog density, precipitation, wetness interpolate over configured duration

### TC-2.7.10.2 Weather States

| # | Requirement |
|---|-------------|
| 1 | R-2.7.10    |

1. **#1** — All six weather states
   - **Expected:** Each reachable and produces distinct parameter sets

### TC-2.8.1.1 Strand Count Limit

| # | Requirement |
|---|-------------|
| 1 | R-2.8.1     |

1. **#1** — 100K strands, desktop tier
   - **Expected:** `StrandParams::strand_count` clamped to platform maximum

### TC-2.8.1.2 Marschner Energy

| # | Requirement |
|---|-------------|
| 1 | R-2.8.1     |

1. **#1** — R + TT + TRT lobe intensities
   - **Expected:** Sum <= 1.0 (energy conservation)

### TC-2.8.1.3 Deep Opacity Res

| # | Requirement |
|---|-------------|
| 1 | R-2.8.1     |

1. **#1** — `StrandParams` with specified resolution
   - **Expected:** Deep opacity map allocated at that resolution

### TC-2.8.2.1 Card Layer Clamp

| # | Requirement |
|---|-------------|
| 1 | R-2.8.2     |

1. **#1** — `CardParams::layer_count=10`, mobile tier
   - **Expected:** Clamped to 3

### TC-2.8.3.2 Hair LOD Tiers

| # | Requirement |
|---|-------------|
| 1 | R-2.8.3     |

1. **#1** — Camera near to far
   - **Expected:** Transitions Strand -> Card -> MeshProxy at thresholds

### TC-2.8.4.1 Eye Cornea IOR

| # | Requirement |
|---|-------------|
| 1 | R-2.8.4     |

1. **#1** — `cornea_ior=1.376`, 45-degree incident ray
   - **Expected:** Refraction angle matches Snell's law

### TC-2.8.5.1 Cloth Fuzz Energy

| # | Requirement |
|---|-------------|
| 1 | R-2.8.5     |

1. **#1** — Fabric specular lobe evaluation
   - **Expected:** Outgoing energy <= incoming energy

### TC-2.8.6.1 Skin Burley Profile

| # | Requirement |
|---|-------------|
| 1 | R-2.8.6     |

1. **#1** — Burley diffusion kernel at known radii
   - **Expected:** Output matches Burley 2015 reference values

### TC-2.8.9.1 Biometric Melanin

| # | Requirement |
|---|-------------|
| 1 | R-2.8.9     |
| 2 | R-2.8.9     |

1. **#1** — melanin=0.0 (lightest)
   - **Expected:** Scatter color at light end of range
2. **#2** — melanin=1.0 (darkest)
   - **Expected:** Scatter color at dark end of range

### TC-2.8.8.1 Peach Fuzz Threshold

| # | Requirement |
|---|-------------|
| 1 | R-2.8.8     |
| 2 | R-2.8.8     |

1. **#1** — `screen_size_threshold=200`, face size=150 px
   - **Expected:** Fuzz disabled
2. **#2** — `screen_size_threshold=200`, face size=250 px
   - **Expected:** Fuzz enabled

### TC-2.8.7.1 Compute Raster Classify

| # | Requirement |
|---|-------------|
| 1 | R-2.8.7     |
| 2 | R-2.8.7     |

1. **#1** — Sub-pixel projected strand area
   - **Expected:** Routed to compute rasterizer path
2. **#2** — Large projected strand area
   - **Expected:** Routed to hardware rasterizer path

## Integration Tests

### TC-2.6.1.I1 TAA Ghosting Rejection

| # | Requirement |
|---|-------------|
| 1 | R-2.6.1     |

1. **#1** — Rapid camera pan disoccluding surfaces
   - **Expected:** Ghosting resolves within 1 frame

### TC-2.6.2.I1 TSR PSNR Quality

| # | Requirement |
|---|-------------|
| 1 | R-2.6.2     |

1. **#1** — Native 4K vs 1080p + TSR to 4K
   - **Expected:** PSNR within 1 dB

### TC-2.6.6.I1 Vendor Upscaler Init

| # | Requirement |
|---|-------------|
| 1 | R-2.6.6     |

1. **#1** — Initialize DLSS, FSR, XeSS
   - **Expected:** Correct output, no crash

### TC-2.6.6.I2 Vendor Fallback Chain

| # | Requirement |
|---|-------------|
| 1 | R-2.6.6     |

1. **#1** — Remove vendor SDKs one by one
   - **Expected:** Fallback chain: vendor -> TSR -> none

### TC-2.6.8.I1 Latency With Frame Gen

| # | Requirement |
|---|-------------|
| 1 | R-2.6.8     |

1. **#1** — Frame gen on + latency reduction
   - **Expected:** Input-to-display latency <= no-gen baseline

### TC-2.7.1.I1 Sky TOD Sweep

| # | Requirement |
|---|-------------|
| 1 | R-2.7.1     |

1. **#1** — Sun sweep from sunrise to sunset
   - **Expected:** Smooth transitions, no banding

### TC-2.7.2.I1 Volumetric Fog Light

| # | Requirement |
|---|-------------|
| 1 | R-2.7.2     |

1. **#1** — Directional light in fog
   - **Expected:** Visible scattering with correct depth attenuation

### TC-2.7.3.I1 Cloud Flythrough

| # | Requirement |
|---|-------------|
| 1 | R-2.7.3     |

1. **#1** — Camera flying through cloud layer
   - **Expected:** No visual artifacts at close range

### TC-2.7.4.I1 God Rays Both Modes

| # | Requirement |
|---|-------------|
| 1 | R-2.7.4     |
| 2 | R-2.7.4     |

1. **#1** — Occluded sun, screen-space mode
   - **Expected:** Visible shafts
2. **#2** — Occluded sun, volumetric mode
   - **Expected:** Visible shafts

### TC-2.7.5.I1 Fog Composite

| # | Requirement |
|---|-------------|
| 1 | R-2.7.5     |

1. **#1** — Analytical fog + froxel volumetrics
   - **Expected:** No double-fogging

### TC-2.7.6.I1 Ocean LOD

| # | Requirement |
|---|-------------|
| 1 | R-2.7.6     |

1. **#1** — Camera from shore to horizon
   - **Expected:** LOD transitions with no popping

### TC-11.2.1.I1 Decal Cross Mesh

| # | Requirement |
|---|-------------|
| 1 | R-11.2.1    |

1. **#1** — Decal across mesh/terrain boundary
   - **Expected:** Seamless blending

### TC-11.2.3.I1 Decal Atlas Eviction

| # | Requirement |
|---|-------------|
| 1 | R-11.2.3    |

1. **#1** — 500+ decals exceeding atlas budget
   - **Expected:** LRU eviction without corruption

### TC-2.7.10.I1 Weather All States

| # | Requirement |
|---|-------------|
| 1 | R-2.7.10    |

1. **#1** — Cycle through all 6 weather states
   - **Expected:** Fog, clouds, particles, lighting, wetness, vegetation respond

### TC-2.8.3.I1 Hair LOD No Pop

| # | Requirement |
|---|-------------|
| 1 | R-2.8.3     |

1. **#1** — Continuous camera zoom at 60 FPS
   - **Expected:** No abrupt quality change between LOD tiers

### TC-2.8.6.I1 Skin SSS Stencil

| # | Requirement |
|---|-------------|
| 1 | R-2.8.6     |

1. **#1** — Skin and non-skin side by side
   - **Expected:** SSS blur does not bleed onto non-skin pixels

### TC-2.8.4.I1 Eye Refraction Angle

| # | Requirement |
|---|-------------|
| 1 | R-2.8.4     |

1. **#1** — Eye viewed at oblique angle
   - **Expected:** Corneal refraction distorts iris

### TC-2.8.5.I1 Cloth vs PBR

| # | Requirement |
|---|-------------|
| 1 | R-2.8.5     |

1. **#1** — Cloth and PBR under identical lighting
   - **Expected:** Distinct highlight response

### TC-2.8.9.I1 Biometric Tones

| # | Requirement |
|---|-------------|
| 1 | R-2.8.9     |

1. **#1** — Melanin 0 to 1 in 10 steps
   - **Expected:** Each tone produces plausible SSS

### TC-2.8.7.I1 Compute Raster AA

| # | Requirement |
|---|-------------|
| 1 | R-2.8.7     |

1. **#1** — Thin strands, compute vs hardware-only
   - **Expected:** Improved anti-aliasing with compute rasterizer

### TC-2.8.8.I1 Peach Fuzz Closeup

| # | Requirement |
|---|-------------|
| 1 | R-2.8.8     |

1. **#1** — Dolly toward face
   - **Expected:** Fuzz activates at threshold, visible light-catching

### TC-2.6.3.I1 SMAA Diagonal Edge

| # | Requirement |
|---|-------------|
| 1 | R-2.6.3     |

1. **#1** — Diagonal high-contrast edge
   - **Expected:** SMAA produces smoother edge than no-AA

## Benchmarks

### TC-2.6.3.B1 FXAA Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.3 ms | NFR-2.6.2 |

### TC-2.6.1.B1 TAA Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.5 ms | NFR-2.6.2 |

### TC-2.6.3.B2 SMAA 3-Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.8 ms | R-2.6.3 |

### TC-2.6.2.B1 TSR Upscale

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p to 4K | GPU time | < 1.5 ms | R-2.6.2 |

### TC-2.6.2.B2 TSR PSNR vs Native

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | TSR output vs native | PSNR | within 1 dB | NFR-2.6.1 |

### TC-2.6.8.B1 Frame Gen Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Frame generation active | Latency | <= no-gen baseline | NFR-2.6.3 |

### TC-2.7.2.B1 Volumetric Fog

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 2.0 ms | NFR-2.7.1 |

### TC-2.7.3.B1 Cloud Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p, temporal reprojection | GPU time | < 3.0 ms | NFR-2.7.2 |

### TC-2.7.6.B1 Ocean Sim and Render

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 3.0 ms | NFR-2.7.3 |

### TC-11.2.3.B1 Decal Pass

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 decals | GPU time | < 0.5 ms | R-11.2.3 |

### TC-2.8.1.B1 Strand Hair

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100K strands, 1080p | GPU time | < 2.0 ms / char | NFR-2.8.1 |

### TC-2.8.2.B1 Card Hair

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.5 ms / char | NFR-2.8.1 |

### TC-2.8.6.B1 Skin SSS Blur

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 1.0 ms | NFR-2.8.2 |

### TC-2.8.3.B1 Hair LOD Crossfade

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | LOD transition | Duration | < 0.5 s, no pop | NFR-2.8.3 |

### TC-2.8.4.B1 Eye Layered Shading

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per character | GPU time | < 0.3 ms / char | R-2.8.4 |

### TC-2.8.5.B1 Cloth Fuzz Layer

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per character | GPU time | < 0.2 ms / char | R-2.8.5 |

### TC-2.8.1.B2 Deep Opacity Map

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100K strands | GPU time | < 0.5 ms / char | R-2.8.1 |

### TC-2.8.7.B1 Compute Hair Raster

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per character | GPU time | < 1.0 ms / char | R-2.8.7 |
