# Environment Systems Test Cases

Companion test cases for [environment.md](environment.md).

## Unit Tests

### TC-3.3.2.1 Placement Respects Slope

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |
| 2 | R-3.3.2     |

1. **#1** — Slope range [0, 30], terrain with 35-degree face
   - **Expected:** Zero instances placed on 35-degree face
2. **#2** — Slope range [0, 30], terrain with 25-degree face
   - **Expected:** Instances placed on 25-degree face

### TC-3.3.2.2 Placement Respects Altitude

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |
| 2 | R-3.3.2     |
| 3 | R-3.3.2     |

1. **#1** — Altitude range [100, 500], point at y=50
   - **Expected:** No instance placed
2. **#2** — Altitude range [100, 500], point at y=300
   - **Expected:** Instance placed
3. **#3** — Altitude range [100, 500], point at y=600
   - **Expected:** No instance placed

### TC-3.3.2.3 Placement Density Correlation

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |
| 2 | R-3.3.2     |

1. **#1** — Density map at 100%, area=100m^2, density=10/m^2
   - **Expected:** Instance count approximately 1000 (within 5%)
2. **#2** — Density map at 50%, area=100m^2, density=10/m^2
   - **Expected:** Instance count approximately 500 (within 5%)

### TC-3.3.3.1 LOD Distance Thresholds

| # | Requirement |
|---|-------------|
| 1 | R-3.3.3     |
| 2 | R-3.3.3     |
| 3 | R-3.3.3     |

1. **#1** — Camera at 5m, thresholds=[10, 50, 200]
   - **Expected:** LOD index = 0 (full detail)
2. **#2** — Camera at 30m, thresholds=[10, 50, 200]
   - **Expected:** LOD index = 1 (mid detail)
3. **#3** — Camera at 100m, thresholds=[10, 50, 200]
   - **Expected:** LOD index = 2 (billboard)

### TC-3.3.3.2 Crossfade Active in Range

| # | Requirement |
|---|-------------|
| 1 | R-3.3.3     |
| 2 | R-3.3.3     |

1. **#1** — Camera at LOD boundary (10m), crossfade_width=2m
   - **Expected:** Dither factor in (0.0, 1.0) (crossfade active)
2. **#2** — Camera at 5m, LOD boundary=10m, crossfade_width=2m
   - **Expected:** Dither factor = 0.0 (no crossfade)

### TC-3.3.4.1 Wind Three Layers

| # | Requirement |
|---|-------------|
| 1 | R-3.3.4     |
| 2 | R-3.3.4     |

1. **#1** — All 3 wind layers enabled (trunk, branch, leaf)
   - **Expected:** Vertex displacement has 3 distinct frequency components
2. **#2** — Only trunk layer enabled
   - **Expected:** Vertex displacement has 1 frequency component

### TC-3.3.4.2 Wind Reads Shared Field

| # | Requirement |
|---|-------------|
| 1 | R-3.3.4     |

1. **#1** — Shared wind field texture (F-4.7.5) updated
   - **Expected:** Foliage wind system reads updated wind values

### TC-3.3.5.1 Interaction Decay

| # | Requirement |
|---|-------------|
| 1 | R-3.3.5     |
| 2 | R-3.3.5     |

1. **#1** — Write impulse magnitude=1.0 to interaction buffer; wait 2s, decay_time=2s
   - **Expected:** Magnitude approximately 0.0
2. **#2** — Write impulse magnitude=1.0; wait 0.5s, decay_time=2s
   - **Expected:** Magnitude approximately 0.75

### TC-3.3.6.1 Grass Density Scales

| # | Requirement |
|---|-------------|
| 1 | R-3.3.6     |
| 2 | R-3.3.6     |

1. **#1** — Camera at 10m; grass blades in view = N1
   - **Expected:** N1 > 0
2. **#2** — Camera at 50m; grass blades in view = N2
   - **Expected:** N2 < N1 (density decreases)

### TC-3.3.7.1 Tree Subsurface Transmission

| # | Requirement |
|---|-------------|
| 1 | R-3.3.7     |
| 2 | R-3.3.7     |

1. **#1** — Render tree with backlight (sun behind tree)
   - **Expected:** Leaf pixels have subsurface contribution > 0
2. **#2** — Render tree with frontlight (sun in front of tree)
   - **Expected:** Leaf pixels have zero or minimal subsurface contribution

### TC-3.4.1.1 FFT Tile Continuity

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |

1. **#1** — Sample displacement at tile boundary, 60 frames
   - **Expected:** Delta < 0.001m across all frames

### TC-3.4.1.2 FFT Cascade Count

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |
| 2 | R-3.4.1     |

1. **#1** — Configure 3 cascades
   - **Expected:** 3 IFFT dispatches with distinct resolutions
2. **#2** — Configure 1 cascade
   - **Expected:** 1 IFFT dispatch

### TC-3.4.1.3 Spectrum Types

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |
| 2 | R-3.4.1     |
| 3 | R-3.4.1     |

1. **#1** — Phillips spectrum initialization
   - **Expected:** Distinct frequency distribution from JONSWAP
2. **#2** — JONSWAP spectrum initialization
   - **Expected:** Distinct frequency distribution from TMA
3. **#3** — TMA spectrum initialization
   - **Expected:** Distinct frequency distribution from Phillips

### TC-3.4.2.1 Shoreline Opacity Fade

| # | Requirement |
|---|-------------|
| 1 | R-3.4.2     |
| 2 | R-3.4.2     |

1. **#1** — Water at depth > 2m from terrain surface
   - **Expected:** Opacity = 1.0
2. **#2** — Water at depth = 0 (terrain intersection)
   - **Expected:** Opacity = 0.0

### TC-3.4.2.2 Shoreline Foam Mask

| # | Requirement |
|---|-------------|
| 1 | R-3.4.2     |
| 2 | R-3.4.2     |

1. **#1** — Pixel within configured shoreline band
   - **Expected:** Foam mask > 0
2. **#2** — Pixel far from shore (10m+ depth)
   - **Expected:** Foam mask = 0

### TC-3.4.3.1 Underwater Fog Density

| # | Requirement |
|---|-------------|
| 1 | R-3.4.3     |
| 2 | R-3.4.3     |

1. **#1** — Camera at 5m depth
   - **Expected:** Fog density > fog at 1m depth (Beer-Lambert)
2. **#2** — Camera at 1m depth
   - **Expected:** Fog density > 0

### TC-3.4.3.2 Underwater Absorption

| # | Requirement |
|---|-------------|
| 1 | R-3.4.3     |

1. **#1** — Scene color at 10m depth, absorption spectrum configured
   - **Expected:** Color shifted toward absorption spectrum

### TC-3.4.4.1 Caustics Depth Scaling

| # | Requirement |
|---|-------------|
| 1 | R-3.4.4     |
| 2 | R-3.4.4     |

1. **#1** — Seabed at 2m depth, wave amplitude=1.0
   - **Expected:** Caustic intensity = I1
2. **#2** — Seabed at 10m depth, wave amplitude=1.0
   - **Expected:** Caustic intensity < I1 (decreases with depth)

### TC-3.4.5.1 Fresnel Grazing Angle

| # | Requirement |
|---|-------------|
| 1 | R-3.4.5     |
| 2 | R-3.4.5     |

1. **#1** — Water surface viewed at grazing angle (> 80 deg from normal)
   - **Expected:** Reflection > refraction
2. **#2** — Water surface viewed at steep angle (< 20 deg from normal)
   - **Expected:** Refraction > reflection

### TC-3.4.6.1 Flow Map UV Advance

| # | Requirement |
|---|-------------|
| 1 | R-3.4.6     |

1. **#1** — Uniform rightward flow (1, 0); advance 10 frames
   - **Expected:** Normal UV offset advances rightward each frame

### TC-3.4.7.1 Foam Coverage Decay

| # | Requirement |
|---|-------------|
| 1 | R-3.4.7     |
| 2 | R-3.4.7     |

1. **#1** — All foam sources active; foam_lifetime=2s; wait 3s
   - **Expected:** Foam coverage = 0 (fully decayed)
2. **#2** — All foam sources active; foam_lifetime=2s; wait 1s
   - **Expected:** Foam coverage > 0 (still decaying)

### TC-3.4.7.2 Foam Jacobian Whitecaps

| # | Requirement |
|---|-------------|
| 1 | R-3.4.7     |
| 2 | R-3.4.7     |

1. **#1** — Jacobian value < whitecap threshold
   - **Expected:** Foam present at that pixel
2. **#2** — Jacobian value > whitecap threshold
   - **Expected:** No foam at that pixel

### TC-3.5.1.1 Sky Luminance Monotonic

| # | Requirement |
|---|-------------|
| 1 | R-3.5.1     |

1. **#1** — Evaluate sky for zenith angles 0, 30, 60, 90 deg (opposite sun)
   - **Expected:** Luminance decreases monotonically

### TC-3.5.1.2 Sky Warm at Sunset

| # | Requirement |
|---|-------------|
| 1 | R-3.5.1     |
| 2 | R-3.5.1     |

1. **#1** — Sun at 5 degrees above horizon
   - **Expected:** Chromaticity shifts warm (higher red/orange)
2. **#2** — Sun at 60 degrees above horizon
   - **Expected:** Chromaticity neutral/blue

### TC-3.5.2.1 Aerial Perspective Depth

| # | Requirement |
|---|-------------|
| 1 | R-3.5.2     |

1. **#1** — Render objects at 1km, 10km, 50km from camera
   - **Expected:** 50km pixel color closer to horizon color than 1km pixel

### TC-3.5.2.2 LUT Recompute on Change

| # | Requirement |
|---|-------------|
| 1 | R-3.5.2     |

1. **#1** — Change AtmosphereConfig parameters
   - **Expected:** LUTs are recomputed (dirty flag set and cleared)

### TC-3.5.2.3 Mie Sun Halo

| # | Requirement |
|---|-------------|
| 1 | R-3.5.2     |

1. **#1** — Sample sky luminance near sun direction
   - **Expected:** Bright halo from Mie scattering (luminance peak)

### TC-3.5.3.1 Cloud Coverage Correlation

| # | Requirement |
|---|-------------|
| 1 | R-3.5.3     |
| 2 | R-3.5.3     |

1. **#1** — Weather map at 50% coverage
   - **Expected:** Cloud pixels non-transparent where coverage > 0
2. **#2** — Weather map at 0% coverage
   - **Expected:** No cloud pixels visible

### TC-3.5.3.2 Cloud Temporal Savings

| # | Requirement |
|---|-------------|
| 1 | R-3.5.3     |

1. **#1** — Temporal reprojection enabled vs disabled
   - **Expected:** Per-frame sample count reduced by >= 50%

### TC-3.5.4.1 Cloud Shadow Modulation

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Terrain under cloud shadow
   - **Expected:** Shadowed pixels receive less direct light than unshadowed

### TC-3.5.4.2 Cloud Shadow Moves

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Animate cloud coverage over 60 frames
   - **Expected:** Shadow pattern position changes between frames

### TC-3.5.5.1 TOD Sun Arc

| # | Requirement |
|---|-------------|
| 1 | R-3.5.5     |

1. **#1** — Advance dawn to night in 60s
   - **Expected:** Sun position follows smooth arc (no discontinuities)

### TC-3.5.5.2 TOD Time Scale

| # | Requirement |
|---|-------------|
| 1 | R-3.5.5     |
| 2 | R-3.5.5     |

1. **#1** — time_scale=1.0; measure cycle_duration
   - **Expected:** Cycle completes in T seconds
2. **#2** — time_scale=2.0; measure cycle_duration
   - **Expected:** Cycle completes in T/2 seconds

### TC-3.5.6.1 Star Magnitude Brightness

| # | Requirement |
|---|-------------|
| 1 | R-3.5.6     |

1. **#1** — Star magnitude=1.0 vs star magnitude=5.0
   - **Expected:** Magnitude-1 star brighter than magnitude-5 star

### TC-3.5.6.2 Star Horizon Extinction

| # | Requirement |
|---|-------------|
| 1 | R-3.5.6     |
| 2 | R-3.5.6     |

1. **#1** — Star at 2 degrees above horizon
   - **Expected:** Brightness reduced by atmospheric extinction
2. **#2** — Star at zenith
   - **Expected:** Full brightness (minimal extinction)

### TC-3.5.6.3 Moon Phase Illumination

| # | Requirement |
|---|-------------|
| 1 | R-3.5.6     |
| 2 | R-3.5.6     |

1. **#1** — First quarter moon phase
   - **Expected:** Approximately half the moon disc illuminated
2. **#2** — Full moon phase
   - **Expected:** Entire disc illuminated

### TC-3.5.7.1 Cubemap Ambient Shift

| # | Requirement |
|---|-------------|
| 1 | R-3.5.7     |

1. **#1** — Change sky from clear to overcast
   - **Expected:** Cubemap ambient color shifts within update period

## Integration Tests

### TC-3.3.1.I1 Foliage 1M Instances

| # | Requirement |
|---|-------------|
| 1 | R-3.3.1     |

1. **#1** — Render 1M foliage instances, GPU culling enabled
   - **Expected:** Constant CPU draw call count (GPU-driven)

### TC-3.3.2.I1 Foliage No Disk Data

| # | Requirement |
|---|-------------|
| 1 | R-3.3.2     |

1. **#1** — Procedural placement only; check disk I/O
   - **Expected:** Zero per-instance data read from disk

### TC-3.4.1.I1 Ocean Physics Coupling

| # | Requirement |
|---|-------------|
| 1 | R-3.4.1     |

1. **#1** — Drop rigid body onto ocean surface
   - **Expected:** Buoyancy force from WaterSurface displacement applied

### TC-3.4.6.I1 River Ocean Seamless

| # | Requirement |
|---|-------------|
| 1 | R-3.4.6     |

1. **#1** — River at ocean estuary
   - **Expected:** Seamless mesh and flow transition at boundary

### TC-3.5.5.I1 Full Day Cycle

| # | Requirement |
|---|-------------|
| 1 | R-3.5.5     |

1. **#1** — Run 24h cycle (dawn, day, dusk, night)
   - **Expected:** All transitions smooth; no discontinuities

### TC-3.5.7.I1 Cubemap Reflects Sky

| # | Requirement |
|---|-------------|
| 1 | R-3.5.7     |

1. **#1** — Place reflective sphere; render
   - **Expected:** Reflections match current atmosphere/sky state

### TC-3.5.4.I1 Cloud Shadow on Foliage

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Forest under clouds
   - **Expected:** Foliage receives cloud shadow modulation

### TC-3.5.4.I2 Cloud Shadow on Water

| # | Requirement |
|---|-------------|
| 1 | R-3.5.4     |

1. **#1** — Ocean under clouds
   - **Expected:** Water surface receives cloud shadow modulation

### TC-3.3.1.I2 Tier Mobile Budget

| # | Requirement |
|---|-------------|
| 1 | R-3.3.1     |

1. **#1** — Run mobile tier, full environment scene
   - **Expected:** Frame time < 16 ms on target hardware

### TC-3.3.1.I3 Tier Desktop Quality

| # | Requirement |
|---|-------------|
| 1 | R-3.3.1     |

1. **#1** — Run desktop tier, full environment scene
   - **Expected:** All features enabled and rendering correctly

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
