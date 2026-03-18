# VFX Effects Test Cases

Companion test cases for [effects.md](effects.md).

## Unit Tests

### TC-11.2.4.1 Decal Lifecycle Phases

| # | Requirement |
|---|-------------|
| 1 | R-11.2.4    |
| 2 | R-11.2.4    |
| 3 | R-11.2.4    |

1. **#1** — Decal: 1s fade-in, 2s sustain, 1s dissolve; advance to t=0.5
   - **Expected:** Phase=FadeIn, opacity=0.5
2. **#2** — Advance to t=1.5
   - **Expected:** Phase=Sustain, opacity=1.0
3. **#3** — Advance to t=3.5
   - **Expected:** Phase=Dissolve, opacity=0.5

### TC-11.2.4.2 Decal Priority Sorting

| # | Requirement |
|---|-------------|
| 1 | R-11.2.4    |

1. **#1** — Decals with priorities 1, 3, 5 on same surface
   - **Expected:** Render order: 1, 3, 5 (ascending)

### TC-11.2.4.3 Decal Pool Reclaim

| # | Requirement |
|---|-------------|
| 1 | R-11.2.4    |

1. **#1** — Exhaust pool, spawn high-priority decal
   - **Expected:** Oldest low-priority decal reclaimed, high-priority succeeds

### TC-11.2.3.1 Atlas Pack and Lookup

| # | Requirement |
|---|-------------|
| 1 | R-11.2.3    |

1. **#1** — Pack 50 textures into atlas
   - **Expected:** Each returns valid region; lookup by AssetId matches

### TC-11.2.3.2 Atlas LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | R-11.2.3    |

1. **#1** — Pack textures exceeding atlas budget
   - **Expected:** LRU entries evicted, new entries succeed

### TC-11.3.1.1 Shake Decay

| # | Requirement |
|---|-------------|
| 1 | R-11.3.1    |

1. **#1** — ShakeSource decay_rate=2.0, advance 3 seconds
   - **Expected:** Amplitude < 0.01

### TC-11.3.1.2 Shake Additive Clamping

| # | Requirement |
|---|-------------|
| 1 | R-11.3.1    |

1. **#1** — 10 simultaneous shake sources
   - **Expected:** Total offset <= max_amp

### TC-11.3.1.3 Shake Reduced Motion

| # | Requirement |
|---|-------------|
| 1 | R-11.3.1    |

1. **#1** — Enable reduced_motion, attenuation=0.0
   - **Expected:** CameraShakeOffset = zero

### TC-11.3.6.1 Overlay Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-11.3.6    |
| 2 | R-11.3.6    |

1. **#1** — Overlay: 0.5s fade-in, 1s sustain, 0.5s fade-out; sample at t=0.25
   - **Expected:** Opacity = 0.5 (fade-in)
2. **#2** — Sample at t=1.0
   - **Expected:** Opacity = 1.0 (sustain)

### TC-11.3.6.2 Overlay Count Cap

| # | Requirement |
|---|-------------|
| 1 | R-11.3.6    |

1. **#1** — Allocate overlays beyond platform max
   - **Expected:** Allocation fails for excess overlays

### TC-11.4.1.1 Weather State Transition

| # | Requirement |
|---|-------------|
| 1 | R-11.4.1    |

1. **#1** — Transition from Clear to Rain
   - **Expected:** intensity ramps up, rain systems activate

### TC-11.4.2.1 Puddle Accumulate Drain

| # | Requirement |
|---|-------------|
| 1 | R-11.4.2    |

1. **#1** — Accumulate puddle for 5s at intensity=1.0, then stop rain
   - **Expected:** Depth increases during rain, drains to 0 after

### TC-11.4.2.2 Wet Surface Material

| # | Requirement |
|---|-------------|
| 1 | R-11.4.2    |
| 2 | R-11.4.2    |

1. **#1** — Stone, wetness=1.0
   - **Expected:** roughness_override = 0.2 (1.0 - 0.8)
2. **#2** — Metal, wetness=1.0
   - **Expected:** roughness_override = 0.05 (1.0 - 0.95)

### TC-11.4.3.1 Snow Deformation Fade

| # | Requirement |
|---|-------------|
| 1 | R-11.4.3    |

1. **#1** — Stamp deformation, continue snowfall for 10s
   - **Expected:** Trail depth decreases toward zero

### TC-11.4.5.1 Lightning Branch Depth

| # | Requirement |
|---|-------------|
| 1 | R-11.4.5    |
| 2 | R-11.4.5    |

1. **#1** — Generate bolt on mobile tier
   - **Expected:** Max branch depth = 2
2. **#2** — Generate bolt on desktop tier
   - **Expected:** Max branch depth = 4

### TC-11.5.1.1 Debris Budget Cap

| # | Requirement |
|---|-------------|
| 1 | R-11.5.1    |

1. **#1** — Trigger destructions exceeding global budget
   - **Expected:** Concurrent fragments never exceed platform cap

### TC-11.5.2.1 Dust Color by Material

| # | Requirement |
|---|-------------|
| 1 | R-11.5.2    |
| 2 | R-11.5.2    |

1. **#1** — Destroy Stone object
   - **Expected:** Dust color = (0.6, 0.6, 0.6)
2. **#2** — Destroy Wood object
   - **Expected:** Dust color = (0.5, 0.35, 0.2)

### TC-11.5.3.1 Spark Color Fade

| # | Requirement |
|---|-------------|
| 1 | R-11.5.3    |
| 2 | R-11.5.3    |
| 3 | R-11.5.3    |

1. **#1** — Spark at t=0
   - **Expected:** Color = white
2. **#2** — Spark at t=0.5*lifetime
   - **Expected:** Color = orange
3. **#3** — Spark at t=lifetime
   - **Expected:** Color = dark/faded

### TC-11.5.4.1 Crack Growth Rate

| # | Requirement |
|---|-------------|
| 1 | R-11.5.4    |

1. **#1** — accumulated_damage=50, growth_speed=1.0, dt=1.0
   - **Expected:** radius increases by 0.5 (50/100 * 1.0 * 1.0)

### TC-11.5.5.1 Scorch Persistence

| # | Requirement |
|---|-------------|
| 1 | R-11.5.5    |

1. **#1** — Scorch mark placed alongside transient decal
   - **Expected:** Scorch renders above transient in priority

### TC-11.5.6.1 Shockwave Expansion

| # | Requirement |
|---|-------------|
| 1 | R-11.5.6    |

1. **#1** — Shockwave at origin, speed=50, after 1 second
   - **Expected:** current_radius approximately 50

### TC-11.5.7.1 Fire Material Blocking

| # | Requirement |
|---|-------------|
| 1 | R-11.5.7    |

1. **#1** — Ignite wood zone adjacent to stone zone
   - **Expected:** Fire spreads on wood, does not spread to stone

### TC-11.5.0.1 Budget Per Platform

| # | Requirement |
|---|-------------|
| 1 | F-11.5.1    |
| 2 | F-11.5.1    |

1. **#1** — EffectBudget for Mobile tier
   - **Expected:** Debris cap=32, shockwave cap=1
2. **#2** — EffectBudget for Desktop tier
   - **Expected:** Debris cap=256, shockwave cap=4

## Integration Tests

### TC-11.2.1.I1 Decal GBuffer Blend

| # | Requirement |
|---|-------------|
| 1 | R-11.2.1    |

1. **#1** — Deferred decal across mesh/terrain boundary
   - **Expected:** G-buffer modified within OBB, unmodified outside

### TC-11.2.1.I2 Decal Angle Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-11.2.1    |

1. **#1** — Decal on 80-degree surface
   - **Expected:** Opacity attenuated to near zero

### TC-11.2.3.I1 Atlas 500 Decals

| # | Requirement |
|---|-------------|
| 1 | R-11.2.3    |

1. **#1** — 500 decals, 50 unique textures
   - **Expected:** Draw calls proportional to atlas pages, not decal count

### TC-11.2.6.I1 Footprint Per Material

| # | Requirement |
|---|-------------|
| 1 | R-11.2.6    |

1. **#1** — Walk across mud, snow, sand
   - **Expected:** Footprint shape and material response differ per surface

### TC-11.3.2.I1 Motion Blur Framerate

| # | Requirement |
|---|-------------|
| 1 | R-11.3.2    |

1. **#1** — Render at 30 and 60 fps
   - **Expected:** Blur width consistent (scaled by frame rate)

### TC-11.3.3.I1 Flare Occlusion

| # | Requirement |
|---|-------------|
| 1 | R-11.3.3    |

1. **#1** — Partially occlude light with flare
   - **Expected:** Temporal smoothing prevents popping

### TC-11.4.1.I1 Rain Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-11.4.1    |
| 2 | R-11.4.1    |

1. **#1** — Rain at full intensity
   - **Expected:** Streaks, droplets, ripples all active
2. **#2** — Move under shelter
   - **Expected:** Droplets cease

### TC-11.4.2.I1 Puddle Dynamic

| # | Requirement |
|---|-------------|
| 1 | R-11.4.2    |

1. **#1** — Rain over varied terrain for 30s
   - **Expected:** Puddles in concavities, dry on ridges

### TC-11.4.3.I1 Snow Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-11.4.3    |

1. **#1** — Snow for 60s, walk through
   - **Expected:** Accumulation on upward surfaces, deformation trails

### TC-11.4.4.I1 Fog Volume Bounds

| # | Requirement |
|---|-------------|
| 1 | R-11.4.4    |

1. **#1** — Place box fog volume
   - **Expected:** Fog confined to bounds, outside is clear

### TC-11.5.1.I1 Debris Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-11.5.1    |

1. **#1** — Destroy object with 10-entry debris table
   - **Expected:** Fragments spawn with correct material and velocity

### TC-11.5.2.I1 Smoke Wind

| # | Requirement |
|---|-------------|
| 1 | R-11.5.2    |

1. **#1** — Destroy structure with active wind field
   - **Expected:** Smoke drift aligns with wind within 15 degrees

### TC-11.5.6.I1 Shockwave Composite

| # | Requirement |
|---|-------------|
| 1 | R-11.5.6    |

1. **#1** — Two overlapping explosions
   - **Expected:** Combined distortion does not exceed configured max

### TC-11.5.7.I1 Fire Spread Wind

| # | Requirement |
|---|-------------|
| 1 | R-11.5.7    |

1. **#1** — Ignite surface with wind blowing east
   - **Expected:** Spread rate increases eastward

## Benchmarks

### TC-11.2.3.B1 256 Deferred Decals Render

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Render 256 deferred decals | GPU time | < 1 ms | R-11.2.3 |

### TC-11.2.3.B2 Atlas Pack 50 Textures

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Pack 50 textures into decal atlas | CPU time | < 2 ms | R-11.2.3 |

### TC-11.3.1.B1 Shake System 20 Sources

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evaluate 20 simultaneous shake sources | CPU time | < 50 us | R-11.3.1 |

### TC-11.3.5.B1 Distortion Accumulation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Accumulate distortion from 10 sources | GPU time | < 0.5 ms | R-11.3.5 |

### TC-11.4.1.B1 Rain 100K Particles

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Simulate 100K rain particles | GPU compute time | < 1 ms | R-11.4.1 |

### TC-11.4.2.B1 Puddle Heightfield Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Update puddle heightfield per terrain patch | GPU time | < 0.3 ms | R-11.4.2 |

### TC-11.4.3.B1 Snow Height Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Update snow height texture per terrain patch | GPU time | < 0.3 ms | R-11.4.3 |

### TC-11.4.5.B1 Lightning L-System Gen

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Generate L-system bolt geometry | CPU time | < 100 us | R-11.4.5 |

### TC-11.5.1.B1 Debris Spawn 256 Fragments

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Spawn 256 debris fragments | CPU time | < 200 us | R-11.5.1 |

### TC-11.5.7.B1 Fire Propagation 256x256

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Propagate fire on 256x256 texel map | GPU time | < 0.5 ms | R-11.5.7 |

### TC-11.5.4.B1 Crack Growth Compute

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Crack growth compute dispatch | GPU time | < 0.2 ms | R-11.5.4 |
