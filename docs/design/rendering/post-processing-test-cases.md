# Post-Processing Test Cases

Companion test cases for [post-processing.md](post-processing.md).

## Unit Tests

### TC-2.9.1.1 Bloom Threshold Extraction

| # | Requirement |
|---|-------------|
| 1 | R-2.9.1     |
| 2 | R-2.9.1     |

1. **#1** — Pixel luminance = 2.0, threshold = 1.0
   - **Expected:** Pixel contributes to bloom
2. **#2** — Pixel luminance = 0.5, threshold = 1.0
   - **Expected:** Pixel does not contribute

### TC-2.9.1.2 Bloom Soft Knee

| # | Requirement |
|---|-------------|
| 1 | R-2.9.1     |

1. **#1** — Luminance at threshold boundary, soft_knee = 0.5
   - **Expected:** Soft transition (not hard cutoff)

### TC-2.9.1.3 Bloom Mip Chain Dimensions

| # | Requirement |
|---|-------------|
| 1 | R-2.9.1     |

1. **#1** — Input 1920x1080, 6 iterations
   - **Expected:** Mips: 960x540, 480x270, 240x135, 120x68, 60x34, 30x17

### TC-2.9.2.1 DOF CoC Calculation

| # | Requirement |
|---|-------------|
| 1 | R-2.9.2     |

1. **#1** — aperture=2.8, focal_length=50mm, focus_dist=10m, depth=20m
   - **Expected:** CoC matches physics formula

### TC-2.9.2.2 DOF Near Far Sign

| # | Requirement |
|---|-------------|
| 1 | R-2.9.2     |
| 2 | R-2.9.2     |

1. **#1** — Depth behind focus plane
   - **Expected:** Positive CoC (far field)
2. **#2** — Depth in front of focus plane
   - **Expected:** Negative CoC (near field)

### TC-2.9.2.3 DOF Bokeh Shape Samples

| # | Requirement |
|---|-------------|
| 1 | R-2.9.2     |
| 2 | R-2.9.2     |
| 3 | R-2.9.2     |

1. **#1** — BokehShape::Circle, sample_count=48
   - **Expected:** Sample positions on unit disc
2. **#2** — BokehShape::Hexagon, sample_count=48
   - **Expected:** Sample positions within hexagonal kernel
3. **#3** — BokehShape::Octagon, sample_count=48
   - **Expected:** Sample positions within octagonal kernel

### TC-2.9.3.1 Motion Blur Tile Max

| # | Requirement |
|---|-------------|
| 1 | R-2.9.3     |

1. **#1** — Velocity buffer with known max per 16x16 tile
   - **Expected:** Per-tile max matches expected values

### TC-2.9.3.2 Motion Blur Skip Static

| # | Requirement |
|---|-------------|
| 1 | R-2.9.3     |

1. **#1** — Tile with zero velocity
   - **Expected:** No blur applied to that tile

### TC-2.9.4.1 Histogram Bin Count

| # | Requirement |
|---|-------------|
| 1 | R-2.9.4     |
| 2 | R-2.9.4     |

1. **#1** — QualityTier::Desktop, histogram_bins=64
   - **Expected:** 64 bins
2. **#2** — QualityTier::HighEnd, histogram_bins=128
   - **Expected:** 128 bins

### TC-2.9.4.2 Histogram Percentile Avg

| # | Requirement |
|---|-------------|
| 1 | R-2.9.4     |

1. **#1** — Known histogram, low=10%, high=90%
   - **Expected:** Weighted average matches manual calculation

### TC-2.9.4.3 Exposure Temporal Adapt

| # | Requirement |
|---|-------------|
| 1 | R-2.9.4     |

1. **#1** — prev_ev=0, target_ev=10, speed_up=3.0, dt=0.016
   - **Expected:** EV moves toward target via exponential smoothing

### TC-2.9.4.4 Exposure Min Max Clamp

| # | Requirement |
|---|-------------|
| 1 | R-2.9.4     |
| 2 | R-2.9.4     |

1. **#1** — min_ev=-4, max_ev=16, target below min
   - **Expected:** EV clamped to -4
2. **#2** — min_ev=-4, max_ev=16, target above max
   - **Expected:** EV clamped to 16

### TC-2.9.5.1 ACES Tonemap Known Values

| # | Requirement |
|---|-------------|
| 1 | R-2.9.5     |

1. **#1** — HDR input (0.0, 0.18, 1.0, 10.0)
   - **Expected:** Output matches ACES reference curve

### TC-2.9.5.2 LUT Identity

| # | Requirement |
|---|-------------|
| 1 | R-2.9.5     |

1. **#1** — Identity 3D LUT
   - **Expected:** No color change in output

### TC-2.9.5.3 LUT Size Per Tier

| # | Requirement |
|---|-------------|
| 1 | R-2.9.5     |
| 2 | R-2.9.5     |
| 3 | R-2.9.5     |

1. **#1** — Mobile tier
   - **Expected:** 16x16x16 LUT
2. **#2** — Desktop tier
   - **Expected:** 32x32x32 LUT
3. **#3** — HighEnd tier
   - **Expected:** 64x64x64 LUT

### TC-2.9.5.4 Lift Gamma Gain Neutral

| # | Requirement |
|---|-------------|
| 1 | R-2.9.5     |

1. **#1** — Default lift/gamma/gain values
   - **Expected:** Identity color transform

### TC-2.9.6.1 Film Grain Luminance Response

| # | Requirement |
|---|-------------|
| 1 | R-2.9.6     |
| 2 | R-2.9.6     |

1. **#1** — luminance_response=1.0, midtone pixel
   - **Expected:** Maximum grain density
2. **#2** — luminance_response=1.0, shadow pixel
   - **Expected:** Reduced grain density

### TC-2.9.7.1 Chromatic Aberr Center Zero

| # | Requirement |
|---|-------------|
| 1 | R-2.9.7     |

1. **#1** — Screen center pixel
   - **Expected:** Zero UV offset for all channels

### TC-2.9.7.2 Chromatic Aberr Radial

| # | Requirement |
|---|-------------|
| 1 | R-2.9.7     |

1. **#1** — Pixel at screen edge vs center
   - **Expected:** Edge has larger UV offset

### TC-2.9.8.1 Lens Flare Threshold

| # | Requirement |
|---|-------------|
| 1 | R-2.9.8     |
| 2 | R-2.9.8     |

1. **#1** — Pixel luminance=3.0, threshold=2.0
   - **Expected:** Generates flare
2. **#2** — Pixel luminance=1.0, threshold=2.0
   - **Expected:** No flare generated

### TC-2.9.8.2 Lens Flare Ghost Count

| # | Requirement |
|---|-------------|
| 1 | R-2.9.8     |

1. **#1** — ghost_count=4
   - **Expected:** Exactly 4 ghost elements in output

### TC-2.9.9.1 Vignette Center Unaffected

| # | Requirement |
|---|-------------|
| 1 | R-2.9.9     |

1. **#1** — Center pixel, center=(0,0)
   - **Expected:** No darkening (multiplier = 1.0)

### TC-2.9.9.2 Vignette Power Curve

| # | Requirement |
|---|-------------|
| 1 | R-2.9.9     |
| 2 | R-2.9.9     |

1. **#1** — Edge pixel, power=2.0
   - **Expected:** Falloff matches x^2 power curve
2. **#2** — Edge pixel, power=4.0
   - **Expected:** Falloff matches x^4 power curve (sharper)

### TC-2.9.11.1 Local Exposure Tile Count

| # | Requirement |
|---|-------------|
| 1 | R-2.9.11    |

1. **#1** — tile_count=16
   - **Expected:** 16x16 tile grid

### TC-2.9.11.2 Local Exposure Max Compensation

| # | Requirement |
|---|-------------|
| 1 | R-2.9.11    |

1. **#1** — max_compensation=2.0, tile needing 3.0 EV
   - **Expected:** Compensation clamped to 2.0 EV

### TC-2.9.12.1 Panini Center Identity

| # | Requirement |
|---|-------------|
| 1 | R-2.9.12    |

1. **#1** — Center pixel, any distance param
   - **Expected:** Pixel unmoved

### TC-2.9.12.2 Panini Edge Correction

| # | Requirement |
|---|-------------|
| 1 | R-2.9.12    |

1. **#1** — Edge pixels, distance=1.0
   - **Expected:** Pixels displaced inward

### TC-2.9.13.1 Cavity Ridge Positive

| # | Requirement |
|---|-------------|
| 1 | R-2.9.13    |

1. **#1** — Convex surface normals
   - **Expected:** Ridge output > 0.5

### TC-2.9.13.2 Cavity Valley Negative

| # | Requirement |
|---|-------------|
| 1 | R-2.9.13    |

1. **#1** — Concave surface normals
   - **Expected:** Valley output < 0.5

### TC-2.9.13.3 Cavity Distance Fade

| # | Requirement |
|---|-------------|
| 1 | R-2.9.13    |

1. **#1** — Surface beyond fade_start
   - **Expected:** Effect attenuated

### TC-2.9.13.4 Cavity Depth Fallback

| # | Requirement |
|---|-------------|
| 1 | R-2.9.13    |

1. **#1** — Depth-reconstructed normals (no G-buffer normals)
   - **Expected:** Comparable cavity output

## Integration Tests

### TC-2.9.1.I1 Full Pipeline 1080p

| # | Requirement |
|---|-------------|
| 1 | NFR-2.9.1   |

1. **#1** — All effects enabled, 1080p
   - **Expected:** Combined GPU time < 3.0 ms

### TC-2.9.2.I1 Per Effect Budget

| # | Requirement |
|---|-------------|
| 1 | NFR-2.9.2   |

1. **#1** — Each effect profiled individually
   - **Expected:** Heavy effects < 1.0 ms; lightweight < 0.1 ms

### TC-2.9.1.I2 Volume Blend Two Overlapping

| # | Requirement |
|---|-------------|
| 1 | R-2.9.1     |

1. **#1** — Two overlapping volumes, different bloom params
   - **Expected:** Correctly blended bloom parameters

### TC-2.9.1.I3 Volume Priority Override

| # | Requirement |
|---|-------------|
| 1 | R-2.9.1     |

1. **#1** — Higher priority volume, weight=1.0
   - **Expected:** Fully overrides lower priority

### TC-2.9.1.I4 Volume Blend Distance

| # | Requirement |
|---|-------------|
| 1 | R-2.9.1     |

1. **#1** — Camera at blend boundary
   - **Expected:** 50% interpolated parameters

### TC-2.9.6.I1 Mobile Composite Merge

| # | Requirement |
|---|-------------|
| 1 | NFR-2.9.2   |

1. **#1** — Mobile tier, grain+CA+vignette
   - **Expected:** Single compute dispatch

### TC-2.9.5.I1 HDR10 Output PQ Curve

| # | Requirement |
|---|-------------|
| 1 | NFR-2.9.3   |

1. **#1** — HdrOutputMode::Hdr10
   - **Expected:** Correct PQ EOTF curve encoding

### TC-2.9.5.I2 HDR10 Output BT2020

| # | Requirement |
|---|-------------|
| 1 | NFR-2.9.3   |

1. **#1** — HdrOutputMode::Hdr10
   - **Expected:** BT.2020 primaries in output

### TC-2.9.10.I1 Custom Material Depth Read

| # | Requirement |
|---|-------------|
| 1 | R-2.9.10    |

1. **#1** — Custom material requesting depth input
   - **Expected:** Depth buffer read correctly

### TC-2.9.10.I2 Custom Material Limit Mobile

| # | Requirement |
|---|-------------|
| 1 | R-2.9.10    |

1. **#1** — Mobile tier, 3 custom passes
   - **Expected:** 2 execute, 3rd rejected with warning

### TC-2.9.0.I1 Pass Order Stability

| # | Requirement              |
|---|--------------------------|
| 1 | R-2.9.1 through R-2.9.14 |

1. **#1** — Full pipeline over 100 frames
   - **Expected:** Effects execute in consistent order

### TC-2.9.0.I2 Disabled Effect Skip

| # | Requirement              |
|---|--------------------------|
| 1 | R-2.9.1 through R-2.9.14 |

1. **#1** — Bloom disabled
   - **Expected:** Bloom not dispatched; pipeline chains around it

### TC-2.9.0.I3 Quality Tier Adaptation

| # | Requirement              |
|---|--------------------------|
| 1 | R-2.9.1 through R-2.9.14 |

1. **#1** — Each platform tier
   - **Expected:** Effects select correct quality settings

### TC-2.9.4.I1 Temporal State Persistence

| # | Requirement |
|---|-------------|
| 1 | R-2.9.4     |

1. **#1** — Exposure and histogram over multiple frames
   - **Expected:** State persists across frames

### TC-2.9.14.I1 Graph Editor Compile

| # | Requirement |
|---|-------------|
| 1 | R-2.9.14    |

1. **#1** — Node graph with valid connections
   - **Expected:** Compiles to valid compute dispatch sequence

### TC-2.9.14.I2 Graph Editor Hot Reload

| # | Requirement |
|---|-------------|
| 1 | R-2.9.14    |

1. **#1** — Modified graph asset
   - **Expected:** Updates within one frame

### TC-2.9.14.I3 Graph Editor Mobile Limit

| # | Requirement |
|---|-------------|
| 1 | R-2.9.14    |

1. **#1** — Mobile tier, 5 graph nodes
   - **Expected:** Caps at 4, logs warning on 5th

## Benchmarks

### TC-2.9.1.B1 Bloom

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 6 iterations, 1080p | GPU time | < 0.8 ms | NFR-2.9.2 |

### TC-2.9.2.B1 DOF

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Gather bokeh, 1080p | GPU time | < 1.0 ms | NFR-2.9.2 |

### TC-2.9.3.B1 Motion Blur

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 samples, 1080p | GPU time | < 0.5 ms | NFR-2.9.2 |

### TC-2.9.4.B1 Auto Exposure

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 64-bin histogram | GPU time | < 0.2 ms | NFR-2.9.2 |

### TC-2.9.5.B1 Tonemap and Color Grade

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Tonemap + LUT | GPU time | < 0.3 ms | NFR-2.9.2 |

### TC-2.9.6.B1 Film Grain

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Procedural grain, 1080p | GPU time | < 0.05 ms | NFR-2.9.2 |

### TC-2.9.7.B1 Chromatic Aberration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 3-sample, 1080p | GPU time | < 0.05 ms | NFR-2.9.2 |

### TC-2.9.8.B1 Lens Flare

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full ghost/halo chain | GPU time | < 0.4 ms | NFR-2.9.2 |

### TC-2.9.9.B1 Vignette

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.03 ms | NFR-2.9.2 |

### TC-2.9.11.B1 Local Exposure

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16x16 tile grid | GPU time | < 0.3 ms | NFR-2.9.2 |

### TC-2.9.12.B1 Panini Projection

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1080p | GPU time | < 0.05 ms | NFR-2.9.2 |

### TC-2.9.13.B1 Cavity

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16 samples, 1080p | GPU time | < 0.5 ms | NFR-2.9.2, US-2.9.13.6 |

### TC-2.9.0.B1 Full Pipeline Combined

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | All effects enabled, 1080p | GPU time | < 3.0 ms | NFR-2.9.1 |

### TC-2.9.6.B2 Mobile Composite Merge

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Mobile tier, merged composite | GPU time | < 0.1 ms | US-2.9.6.2 |
