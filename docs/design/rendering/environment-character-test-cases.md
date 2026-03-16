# Environment and Character Rendering Test Cases

Companion test cases for [environment-character.md](environment-character.md).

## Unit Tests

### TC-2.6.1.1 TAA Jitter Halton

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Halton(2,3) sequence, 16 frames | 16 distinct sub-pixel offsets, no repeats | R-2.6.1 |

### TC-2.6.1.2 TAA Clamp MinMax

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3x3 neighborhood, history sample outside min/max | History sample rejected (clamped) | R-2.6.1 |

### TC-2.6.1.3 TAA Clamp Variance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | High-variance 3x3 neighborhood | Variance clamp box tighter than MinMax clamp | R-2.6.1 |

### TC-2.6.3.1 FXAA Edge Detect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Black/white boundary | FXAA detects and blends the edge | R-2.6.3 |

### TC-2.6.3.2 SMAA Three Pass

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | High-contrast scene | Edge, blend weight, resolve passes execute in sequence; anti-aliased output | R-2.6.3 |

### TC-2.6.4.1 MSAA Forward Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Forward path, MSAA 2x/4x | MSAA activates | R-2.6.4 |
| 2 | Deferred path, MSAA 2x | MSAA rejected | R-2.6.4 |

### TC-2.6.2.1 TSR Internal Res

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Each quality mode (Performance, Balanced, Quality, Ultra) | Correct internal dimensions per mode | R-2.6.2 |

### TC-2.6.6.1 Upscaler Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | No vendor SDK available | `VendorUpscaler::detect` returns `Tsr` variant | R-2.6.6 |

### TC-2.6.5.1 Checkerboard Pattern

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two consecutive frames | Alternating pixel pattern covers all pixels | R-2.6.5 |

### TC-2.6.7.1 Frame Gen Doubling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `max_generated_frames=1` | 2x output frames per rendered frame | R-2.6.7 |

### TC-2.7.1.1 Sky LUT Dimensions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Each platform tier | Transmittance, multi-scatter, sky-view, aerial LUTs have correct dimensions | R-2.7.1 |

### TC-2.7.2.1 Froxel Grid Setup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `froxel_resolution = (160, 90, 64)` | Grid dimensions match 160x90x64 | R-2.7.2 |

### TC-2.7.5.1 Fog Model Exp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | density=0.1, distance=10.0 | Attenuation = exp(-0.1 * 10) = 0.3679 | R-2.7.5 |

### TC-2.7.5.2 Fog Model Exp2

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | density=0.1, distance=10.0 | Attenuation = exp(-(0.1 * 10)^2) = 0.3679 | R-2.7.5 |

### TC-11.2.1.1 Decal Channel Mask

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decal writes albedo only | Normal, roughness, metallic unchanged in G-buffer | R-11.2.1 |

### TC-11.2.4.1 Decal Priority Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two decals at same position, different priorities | Higher-priority decal overwrites lower | R-11.2.4 |

### TC-11.2.1.2 Decal Angle Fade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decal projected at 80-degree angle (beyond threshold) | Opacity attenuated to near-zero | R-11.2.1 |

### TC-11.2.4.2 Decal Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn decal, advance time | Fade-in, sustain, dissolve-out match configured durations | R-11.2.4 |

### TC-2.7.2.2 Detail Poisson

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Poisson disk distribution | Minimum spacing guaranteed between all instances | R-2.7.2 |

### TC-2.8.3.1 LOD Crossfade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `LodTransitionComponent::blend_factor` over `dither_duration` | Transitions linearly from 0.0 to 1.0 | R-2.8.3 |

### TC-2.7.10.1 Weather Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transition Clear to Rain | Fog density, precipitation, wetness interpolate over configured duration | R-2.7.10 |

### TC-2.7.10.2 Weather States

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All six weather states | Each reachable and produces distinct parameter sets | R-2.7.10 |

### TC-2.8.1.1 Strand Count Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100K strands, desktop tier | `StrandParams::strand_count` clamped to platform maximum | R-2.8.1 |

### TC-2.8.1.2 Marschner Energy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | R + TT + TRT lobe intensities | Sum <= 1.0 (energy conservation) | R-2.8.1 |

### TC-2.8.1.3 Deep Opacity Res

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `StrandParams` with specified resolution | Deep opacity map allocated at that resolution | R-2.8.1 |

### TC-2.8.2.1 Card Layer Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `CardParams::layer_count=10`, mobile tier | Clamped to 3 | R-2.8.2 |

### TC-2.8.3.2 Hair LOD Tiers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera near to far | Transitions Strand -> Card -> MeshProxy at thresholds | R-2.8.3 |

### TC-2.8.4.1 Eye Cornea IOR

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `cornea_ior=1.376`, 45-degree incident ray | Refraction angle matches Snell's law | R-2.8.4 |

### TC-2.8.5.1 Cloth Fuzz Energy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fabric specular lobe evaluation | Outgoing energy <= incoming energy | R-2.8.5 |

### TC-2.8.6.1 Skin Burley Profile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Burley diffusion kernel at known radii | Output matches Burley 2015 reference values | R-2.8.6 |

### TC-2.8.9.1 Biometric Melanin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | melanin=0.0 (lightest) | Scatter color at light end of range | R-2.8.9 |
| 2 | melanin=1.0 (darkest) | Scatter color at dark end of range | R-2.8.9 |

### TC-2.8.8.1 Peach Fuzz Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `screen_size_threshold=200`, face size=150 px | Fuzz disabled | R-2.8.8 |
| 2 | `screen_size_threshold=200`, face size=250 px | Fuzz enabled | R-2.8.8 |

### TC-2.8.7.1 Compute Raster Classify

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sub-pixel projected strand area | Routed to compute rasterizer path | R-2.8.7 |
| 2 | Large projected strand area | Routed to hardware rasterizer path | R-2.8.7 |

## Integration Tests

### TC-2.6.1.I1 TAA Ghosting Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rapid camera pan disoccluding surfaces | Ghosting resolves within 1 frame | R-2.6.1 |

### TC-2.6.2.I1 TSR PSNR Quality

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Native 4K vs 1080p + TSR to 4K | PSNR within 1 dB | R-2.6.2 |

### TC-2.6.6.I1 Vendor Upscaler Init

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initialize DLSS, FSR, XeSS | Correct output, no crash | R-2.6.6 |

### TC-2.6.6.I2 Vendor Fallback Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove vendor SDKs one by one | Fallback chain: vendor -> TSR -> none | R-2.6.6 |

### TC-2.6.8.I1 Latency With Frame Gen

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Frame gen on + latency reduction | Input-to-display latency <= no-gen baseline | R-2.6.8 |

### TC-2.7.1.I1 Sky TOD Sweep

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sun sweep from sunrise to sunset | Smooth transitions, no banding | R-2.7.1 |

### TC-2.7.2.I1 Volumetric Fog Light

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Directional light in fog | Visible scattering with correct depth attenuation | R-2.7.2 |

### TC-2.7.3.I1 Cloud Flythrough

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera flying through cloud layer | No visual artifacts at close range | R-2.7.3 |

### TC-2.7.4.I1 God Rays Both Modes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Occluded sun, screen-space mode | Visible shafts | R-2.7.4 |
| 2 | Occluded sun, volumetric mode | Visible shafts | R-2.7.4 |

### TC-2.7.5.I1 Fog Composite

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Analytical fog + froxel volumetrics | No double-fogging | R-2.7.5 |

### TC-2.7.6.I1 Ocean LOD

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera from shore to horizon | LOD transitions with no popping | R-2.7.6 |

### TC-11.2.1.I1 Decal Cross Mesh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decal across mesh/terrain boundary | Seamless blending | R-11.2.1 |

### TC-11.2.3.I1 Decal Atlas Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500+ decals exceeding atlas budget | LRU eviction without corruption | R-11.2.3 |

### TC-2.7.10.I1 Weather All States

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cycle through all 6 weather states | Fog, clouds, particles, lighting, wetness, vegetation respond | R-2.7.10 |

### TC-2.8.3.I1 Hair LOD No Pop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Continuous camera zoom at 60 FPS | No abrupt quality change between LOD tiers | R-2.8.3 |

### TC-2.8.6.I1 Skin SSS Stencil

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Skin and non-skin side by side | SSS blur does not bleed onto non-skin pixels | R-2.8.6 |

### TC-2.8.4.I1 Eye Refraction Angle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Eye viewed at oblique angle | Corneal refraction distorts iris | R-2.8.4 |

### TC-2.8.5.I1 Cloth vs PBR

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth and PBR under identical lighting | Distinct highlight response | R-2.8.5 |

### TC-2.8.9.I1 Biometric Tones

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Melanin 0 to 1 in 10 steps | Each tone produces plausible SSS | R-2.8.9 |

### TC-2.8.7.I1 Compute Raster AA

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Thin strands, compute vs hardware-only | Improved anti-aliasing with compute rasterizer | R-2.8.7 |

### TC-2.8.8.I1 Peach Fuzz Closeup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dolly toward face | Fuzz activates at threshold, visible light-catching | R-2.8.8 |

### TC-2.6.3.I1 SMAA Diagonal Edge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Diagonal high-contrast edge | SMAA produces smoother edge than no-AA | R-2.6.3 |

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
