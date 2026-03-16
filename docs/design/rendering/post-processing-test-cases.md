# Post-Processing Test Cases

Companion test cases for [post-processing.md](post-processing.md).

## Unit Tests

### TC-2.9.1.1 Bloom Threshold Extraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pixel luminance = 2.0, threshold = 1.0 | Pixel contributes to bloom | R-2.9.1 |
| 2 | Pixel luminance = 0.5, threshold = 1.0 | Pixel does not contribute | R-2.9.1 |

### TC-2.9.1.2 Bloom Soft Knee

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Luminance at threshold boundary, soft_knee = 0.5 | Soft transition (not hard cutoff) | R-2.9.1 |

### TC-2.9.1.3 Bloom Mip Chain Dimensions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Input 1920x1080, 6 iterations | Mips: 960x540, 480x270, 240x135, 120x68, 60x34, 30x17 | R-2.9.1 |

### TC-2.9.2.1 DOF CoC Calculation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | aperture=2.8, focal_length=50mm, focus_dist=10m, depth=20m | CoC matches physics formula | R-2.9.2 |

### TC-2.9.2.2 DOF Near Far Sign

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Depth behind focus plane | Positive CoC (far field) | R-2.9.2 |
| 2 | Depth in front of focus plane | Negative CoC (near field) | R-2.9.2 |

### TC-2.9.2.3 DOF Bokeh Shape Samples

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | BokehShape::Circle, sample_count=48 | Sample positions on unit disc | R-2.9.2 |
| 2 | BokehShape::Hexagon, sample_count=48 | Sample positions within hexagonal kernel | R-2.9.2 |
| 3 | BokehShape::Octagon, sample_count=48 | Sample positions within octagonal kernel | R-2.9.2 |

### TC-2.9.3.1 Motion Blur Tile Max

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Velocity buffer with known max per 16x16 tile | Per-tile max matches expected values | R-2.9.3 |

### TC-2.9.3.2 Motion Blur Skip Static

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tile with zero velocity | No blur applied to that tile | R-2.9.3 |

### TC-2.9.4.1 Histogram Bin Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | QualityTier::Desktop, histogram_bins=64 | 64 bins | R-2.9.4 |
| 2 | QualityTier::HighEnd, histogram_bins=128 | 128 bins | R-2.9.4 |

### TC-2.9.4.2 Histogram Percentile Avg

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known histogram, low=10%, high=90% | Weighted average matches manual calculation | R-2.9.4 |

### TC-2.9.4.3 Exposure Temporal Adapt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | prev_ev=0, target_ev=10, speed_up=3.0, dt=0.016 | EV moves toward target via exponential smoothing | R-2.9.4 |

### TC-2.9.4.4 Exposure Min Max Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | min_ev=-4, max_ev=16, target below min | EV clamped to -4 | R-2.9.4 |
| 2 | min_ev=-4, max_ev=16, target above max | EV clamped to 16 | R-2.9.4 |

### TC-2.9.5.1 ACES Tonemap Known Values

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HDR input (0.0, 0.18, 1.0, 10.0) | Output matches ACES reference curve | R-2.9.5 |

### TC-2.9.5.2 LUT Identity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Identity 3D LUT | No color change in output | R-2.9.5 |

### TC-2.9.5.3 LUT Size Per Tier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile tier | 16x16x16 LUT | R-2.9.5 |
| 2 | Desktop tier | 32x32x32 LUT | R-2.9.5 |
| 3 | HighEnd tier | 64x64x64 LUT | R-2.9.5 |

### TC-2.9.5.4 Lift Gamma Gain Neutral

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Default lift/gamma/gain values | Identity color transform | R-2.9.5 |

### TC-2.9.6.1 Film Grain Luminance Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | luminance_response=1.0, midtone pixel | Maximum grain density | R-2.9.6 |
| 2 | luminance_response=1.0, shadow pixel | Reduced grain density | R-2.9.6 |

### TC-2.9.7.1 Chromatic Aberr Center Zero

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Screen center pixel | Zero UV offset for all channels | R-2.9.7 |

### TC-2.9.7.2 Chromatic Aberr Radial

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pixel at screen edge vs center | Edge has larger UV offset | R-2.9.7 |

### TC-2.9.8.1 Lens Flare Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pixel luminance=3.0, threshold=2.0 | Generates flare | R-2.9.8 |
| 2 | Pixel luminance=1.0, threshold=2.0 | No flare generated | R-2.9.8 |

### TC-2.9.8.2 Lens Flare Ghost Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ghost_count=4 | Exactly 4 ghost elements in output | R-2.9.8 |

### TC-2.9.9.1 Vignette Center Unaffected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Center pixel, center=(0,0) | No darkening (multiplier = 1.0) | R-2.9.9 |

### TC-2.9.9.2 Vignette Power Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Edge pixel, power=2.0 | Falloff matches x^2 power curve | R-2.9.9 |
| 2 | Edge pixel, power=4.0 | Falloff matches x^4 power curve (sharper) | R-2.9.9 |

### TC-2.9.11.1 Local Exposure Tile Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | tile_count=16 | 16x16 tile grid | R-2.9.11 |

### TC-2.9.11.2 Local Exposure Max Compensation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | max_compensation=2.0, tile needing 3.0 EV | Compensation clamped to 2.0 EV | R-2.9.11 |

### TC-2.9.12.1 Panini Center Identity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Center pixel, any distance param | Pixel unmoved | R-2.9.12 |

### TC-2.9.12.2 Panini Edge Correction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Edge pixels, distance=1.0 | Pixels displaced inward | R-2.9.12 |

### TC-2.9.13.1 Cavity Ridge Positive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Convex surface normals | Ridge output > 0.5 | R-2.9.13 |

### TC-2.9.13.2 Cavity Valley Negative

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Concave surface normals | Valley output < 0.5 | R-2.9.13 |

### TC-2.9.13.3 Cavity Distance Fade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Surface beyond fade_start | Effect attenuated | R-2.9.13 |

### TC-2.9.13.4 Cavity Depth Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Depth-reconstructed normals (no G-buffer normals) | Comparable cavity output | R-2.9.13 |

## Integration Tests

### TC-2.9.1.I1 Full Pipeline 1080p

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All effects enabled, 1080p | Combined GPU time < 3.0 ms | NFR-2.9.1 |

### TC-2.9.2.I1 Per Effect Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Each effect profiled individually | Heavy effects < 1.0 ms; lightweight < 0.1 ms | NFR-2.9.2 |

### TC-2.9.1.I2 Volume Blend Two Overlapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two overlapping volumes, different bloom params | Correctly blended bloom parameters | R-2.9.1 |

### TC-2.9.1.I3 Volume Priority Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Higher priority volume, weight=1.0 | Fully overrides lower priority | R-2.9.1 |

### TC-2.9.1.I4 Volume Blend Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at blend boundary | 50% interpolated parameters | R-2.9.1 |

### TC-2.9.6.I1 Mobile Composite Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile tier, grain+CA+vignette | Single compute dispatch | NFR-2.9.2 |

### TC-2.9.5.I1 HDR10 Output PQ Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HdrOutputMode::Hdr10 | Correct PQ EOTF curve encoding | NFR-2.9.3 |

### TC-2.9.5.I2 HDR10 Output BT2020

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | HdrOutputMode::Hdr10 | BT.2020 primaries in output | NFR-2.9.3 |

### TC-2.9.10.I1 Custom Material Depth Read

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Custom material requesting depth input | Depth buffer read correctly | R-2.9.10 |

### TC-2.9.10.I2 Custom Material Limit Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile tier, 3 custom passes | 2 execute, 3rd rejected with warning | R-2.9.10 |

### TC-2.9.0.I1 Pass Order Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full pipeline over 100 frames | Effects execute in consistent order | R-2.9.1 through R-2.9.14 |

### TC-2.9.0.I2 Disabled Effect Skip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bloom disabled | Bloom not dispatched; pipeline chains around it | R-2.9.1 through R-2.9.14 |

### TC-2.9.0.I3 Quality Tier Adaptation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Each platform tier | Effects select correct quality settings | R-2.9.1 through R-2.9.14 |

### TC-2.9.4.I1 Temporal State Persistence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exposure and histogram over multiple frames | State persists across frames | R-2.9.4 |

### TC-2.9.14.I1 Graph Editor Compile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Node graph with valid connections | Compiles to valid compute dispatch sequence | R-2.9.14 |

### TC-2.9.14.I2 Graph Editor Hot Reload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modified graph asset | Updates within one frame | R-2.9.14 |

### TC-2.9.14.I3 Graph Editor Mobile Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile tier, 5 graph nodes | Caps at 4, logs warning on 5th | R-2.9.14 |

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
