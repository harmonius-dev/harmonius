# Environment Systems Test Cases

Companion test cases for [environment.md](environment.md).

## Unit Tests

### TC-3.3.2.1 Placement Respects Slope

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Slope range [0, 30], terrain with 35-degree face | Zero instances placed on 35-degree face | R-3.3.2 |
| 2 | Slope range [0, 30], terrain with 25-degree face | Instances placed on 25-degree face | R-3.3.2 |

### TC-3.3.2.2 Placement Respects Altitude

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Altitude range [100, 500], point at y=50 | No instance placed | R-3.3.2 |
| 2 | Altitude range [100, 500], point at y=300 | Instance placed | R-3.3.2 |
| 3 | Altitude range [100, 500], point at y=600 | No instance placed | R-3.3.2 |

### TC-3.3.2.3 Placement Density Correlation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Density map at 100%, area=100m^2, density=10/m^2 | Instance count approximately 1000 (within 5%) | R-3.3.2 |
| 2 | Density map at 50%, area=100m^2, density=10/m^2 | Instance count approximately 500 (within 5%) | R-3.3.2 |

### TC-3.3.3.1 LOD Distance Thresholds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at 5m, thresholds=[10, 50, 200] | LOD index = 0 (full detail) | R-3.3.3 |
| 2 | Camera at 30m, thresholds=[10, 50, 200] | LOD index = 1 (mid detail) | R-3.3.3 |
| 3 | Camera at 100m, thresholds=[10, 50, 200] | LOD index = 2 (billboard) | R-3.3.3 |

### TC-3.3.3.2 Crossfade Active in Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at LOD boundary (10m), crossfade_width=2m | Dither factor in (0.0, 1.0) (crossfade active) | R-3.3.3 |
| 2 | Camera at 5m, LOD boundary=10m, crossfade_width=2m | Dither factor = 0.0 (no crossfade) | R-3.3.3 |

### TC-3.3.4.1 Wind Three Layers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All 3 wind layers enabled (trunk, branch, leaf) | Vertex displacement has 3 distinct frequency components | R-3.3.4 |
| 2 | Only trunk layer enabled | Vertex displacement has 1 frequency component | R-3.3.4 |

### TC-3.3.4.2 Wind Reads Shared Field

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shared wind field texture (F-4.7.5) updated | Foliage wind system reads updated wind values | R-3.3.4 |

### TC-3.3.5.1 Interaction Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write impulse magnitude=1.0 to interaction buffer; wait 2s, decay_time=2s | Magnitude approximately 0.0 | R-3.3.5 |
| 2 | Write impulse magnitude=1.0; wait 0.5s, decay_time=2s | Magnitude approximately 0.75 | R-3.3.5 |

### TC-3.3.6.1 Grass Density Scales

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at 10m; grass blades in view = N1 | N1 > 0 | R-3.3.6 |
| 2 | Camera at 50m; grass blades in view = N2 | N2 < N1 (density decreases) | R-3.3.6 |

### TC-3.3.7.1 Tree Subsurface Transmission

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render tree with backlight (sun behind tree) | Leaf pixels have subsurface contribution > 0 | R-3.3.7 |
| 2 | Render tree with frontlight (sun in front of tree) | Leaf pixels have zero or minimal subsurface contribution | R-3.3.7 |

### TC-3.4.1.1 FFT Tile Continuity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sample displacement at tile boundary, 60 frames | Delta < 0.001m across all frames | R-3.4.1 |

### TC-3.4.1.2 FFT Cascade Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Configure 3 cascades | 3 IFFT dispatches with distinct resolutions | R-3.4.1 |
| 2 | Configure 1 cascade | 1 IFFT dispatch | R-3.4.1 |

### TC-3.4.1.3 Spectrum Types

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Phillips spectrum initialization | Distinct frequency distribution from JONSWAP | R-3.4.1 |
| 2 | JONSWAP spectrum initialization | Distinct frequency distribution from TMA | R-3.4.1 |
| 3 | TMA spectrum initialization | Distinct frequency distribution from Phillips | R-3.4.1 |

### TC-3.4.2.1 Shoreline Opacity Fade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Water at depth > 2m from terrain surface | Opacity = 1.0 | R-3.4.2 |
| 2 | Water at depth = 0 (terrain intersection) | Opacity = 0.0 | R-3.4.2 |

### TC-3.4.2.2 Shoreline Foam Mask

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pixel within configured shoreline band | Foam mask > 0 | R-3.4.2 |
| 2 | Pixel far from shore (10m+ depth) | Foam mask = 0 | R-3.4.2 |

### TC-3.4.3.1 Underwater Fog Density

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at 5m depth | Fog density > fog at 1m depth (Beer-Lambert) | R-3.4.3 |
| 2 | Camera at 1m depth | Fog density > 0 | R-3.4.3 |

### TC-3.4.3.2 Underwater Absorption

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene color at 10m depth, absorption spectrum configured | Color shifted toward absorption spectrum | R-3.4.3 |

### TC-3.4.4.1 Caustics Depth Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Seabed at 2m depth, wave amplitude=1.0 | Caustic intensity = I1 | R-3.4.4 |
| 2 | Seabed at 10m depth, wave amplitude=1.0 | Caustic intensity < I1 (decreases with depth) | R-3.4.4 |

### TC-3.4.5.1 Fresnel Grazing Angle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Water surface viewed at grazing angle (> 80 deg from normal) | Reflection > refraction | R-3.4.5 |
| 2 | Water surface viewed at steep angle (< 20 deg from normal) | Refraction > reflection | R-3.4.5 |

### TC-3.4.6.1 Flow Map UV Advance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Uniform rightward flow (1, 0); advance 10 frames | Normal UV offset advances rightward each frame | R-3.4.6 |

### TC-3.4.7.1 Foam Coverage Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All foam sources active; foam_lifetime=2s; wait 3s | Foam coverage = 0 (fully decayed) | R-3.4.7 |
| 2 | All foam sources active; foam_lifetime=2s; wait 1s | Foam coverage > 0 (still decaying) | R-3.4.7 |

### TC-3.4.7.2 Foam Jacobian Whitecaps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Jacobian value < whitecap threshold | Foam present at that pixel | R-3.4.7 |
| 2 | Jacobian value > whitecap threshold | No foam at that pixel | R-3.4.7 |

### TC-3.5.1.1 Sky Luminance Monotonic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Evaluate sky for zenith angles 0, 30, 60, 90 deg (opposite sun) | Luminance decreases monotonically | R-3.5.1 |

### TC-3.5.1.2 Sky Warm at Sunset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sun at 5 degrees above horizon | Chromaticity shifts warm (higher red/orange) | R-3.5.1 |
| 2 | Sun at 60 degrees above horizon | Chromaticity neutral/blue | R-3.5.1 |

### TC-3.5.2.1 Aerial Perspective Depth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render objects at 1km, 10km, 50km from camera | 50km pixel color closer to horizon color than 1km pixel | R-3.5.2 |

### TC-3.5.2.2 LUT Recompute on Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change AtmosphereConfig parameters | LUTs are recomputed (dirty flag set and cleared) | R-3.5.2 |

### TC-3.5.2.3 Mie Sun Halo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sample sky luminance near sun direction | Bright halo from Mie scattering (luminance peak) | R-3.5.2 |

### TC-3.5.3.1 Cloud Coverage Correlation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Weather map at 50% coverage | Cloud pixels non-transparent where coverage > 0 | R-3.5.3 |
| 2 | Weather map at 0% coverage | No cloud pixels visible | R-3.5.3 |

### TC-3.5.3.2 Cloud Temporal Savings

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Temporal reprojection enabled vs disabled | Per-frame sample count reduced by >= 50% | R-3.5.3 |

### TC-3.5.4.1 Cloud Shadow Modulation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Terrain under cloud shadow | Shadowed pixels receive less direct light than unshadowed | R-3.5.4 |

### TC-3.5.4.2 Cloud Shadow Moves

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Animate cloud coverage over 60 frames | Shadow pattern position changes between frames | R-3.5.4 |

### TC-3.5.5.1 TOD Sun Arc

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Advance dawn to night in 60s | Sun position follows smooth arc (no discontinuities) | R-3.5.5 |

### TC-3.5.5.2 TOD Time Scale

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | time_scale=1.0; measure cycle_duration | Cycle completes in T seconds | R-3.5.5 |
| 2 | time_scale=2.0; measure cycle_duration | Cycle completes in T/2 seconds | R-3.5.5 |

### TC-3.5.6.1 Star Magnitude Brightness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Star magnitude=1.0 vs star magnitude=5.0 | Magnitude-1 star brighter than magnitude-5 star | R-3.5.6 |

### TC-3.5.6.2 Star Horizon Extinction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Star at 2 degrees above horizon | Brightness reduced by atmospheric extinction | R-3.5.6 |
| 2 | Star at zenith | Full brightness (minimal extinction) | R-3.5.6 |

### TC-3.5.6.3 Moon Phase Illumination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | First quarter moon phase | Approximately half the moon disc illuminated | R-3.5.6 |
| 2 | Full moon phase | Entire disc illuminated | R-3.5.6 |

### TC-3.5.7.1 Cubemap Ambient Shift

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change sky from clear to overcast | Cubemap ambient color shifts within update period | R-3.5.7 |

## Integration Tests

### TC-3.3.1.I1 Foliage 1M Instances

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render 1M foliage instances, GPU culling enabled | Constant CPU draw call count (GPU-driven) | R-3.3.1 |

### TC-3.3.2.I1 Foliage No Disk Data

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Procedural placement only; check disk I/O | Zero per-instance data read from disk | R-3.3.2 |

### TC-3.4.1.I1 Ocean Physics Coupling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drop rigid body onto ocean surface | Buoyancy force from WaterSurface displacement applied | R-3.4.1 |

### TC-3.4.6.I1 River Ocean Seamless

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | River at ocean estuary | Seamless mesh and flow transition at boundary | R-3.4.6 |

### TC-3.5.5.I1 Full Day Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run 24h cycle (dawn, day, dusk, night) | All transitions smooth; no discontinuities | R-3.5.5 |

### TC-3.5.7.I1 Cubemap Reflects Sky

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place reflective sphere; render | Reflections match current atmosphere/sky state | R-3.5.7 |

### TC-3.5.4.I1 Cloud Shadow on Foliage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Forest under clouds | Foliage receives cloud shadow modulation | R-3.5.4 |

### TC-3.5.4.I2 Cloud Shadow on Water

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ocean under clouds | Water surface receives cloud shadow modulation | R-3.5.4 |

### TC-3.3.1.I2 Tier Mobile Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run mobile tier, full environment scene | Frame time < 16 ms on target hardware | R-3.3.1 |

### TC-3.3.1.I3 Tier Desktop Quality

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run desktop tier, full environment scene | All features enabled and rendering correctly | R-3.3.1 |

## Benchmarks

### TC-3.3.1.B1 Foliage Cull 1M Instances

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M foliage instances, compute cull pass | GPU time | < 1 ms | R-3.3.1 |

### TC-3.3.2.B1 Foliage Placement Per Tile

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single terrain tile, procedural placement | GPU time | < 2 ms | R-3.3.2 |

### TC-3.4.1.B1 Ocean FFT

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256x256 resolution, 3 cascades | GPU time | < 1 ms | R-3.4.1 |

### TC-3.5.3.B1 Cloud Ray March

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Half-resolution, 96 ray march steps | GPU time | < 3 ms | R-3.5.3 |

### TC-3.5.2.B1 Atmosphere LUT Rebuild

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full atmosphere LUT recompute | GPU time | < 2 ms | R-3.5.2 |

### TC-3.5.7.B1 Cubemap Capture Per Face

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single cubemap face render | GPU time | < 0.5 ms | R-3.5.7 |

### TC-3.5.4.B1 Cloud Shadow Map

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2048x2048 cloud shadow map generation | GPU time | < 0.5 ms | R-3.5.4 |

### TC-3.3.6.B1 Grass Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200K grass blades, mesh shader generation | GPU time | < 1.5 ms | R-3.3.6 |
