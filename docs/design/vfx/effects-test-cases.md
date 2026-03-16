# VFX Effects Test Cases

Companion test cases for [effects.md](effects.md).

## Unit Tests

### TC-11.2.4.1 Decal Lifecycle Phases

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decal: 1s fade-in, 2s sustain, 1s dissolve; advance to t=0.5 | Phase=FadeIn, opacity=0.5 | R-11.2.4 |
| 2 | Advance to t=1.5 | Phase=Sustain, opacity=1.0 | R-11.2.4 |
| 3 | Advance to t=3.5 | Phase=Dissolve, opacity=0.5 | R-11.2.4 |

### TC-11.2.4.2 Decal Priority Sorting

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decals with priorities 1, 3, 5 on same surface | Render order: 1, 3, 5 (ascending) | R-11.2.4 |

### TC-11.2.4.3 Decal Pool Reclaim

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exhaust pool, spawn high-priority decal | Oldest low-priority decal reclaimed, high-priority succeeds | R-11.2.4 |

### TC-11.2.3.1 Atlas Pack and Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pack 50 textures into atlas | Each returns valid region; lookup by AssetId matches | R-11.2.3 |

### TC-11.2.3.2 Atlas LRU Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pack textures exceeding atlas budget | LRU entries evicted, new entries succeed | R-11.2.3 |

### TC-11.3.1.1 Shake Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ShakeSource decay_rate=2.0, advance 3 seconds | Amplitude < 0.01 | R-11.3.1 |

### TC-11.3.1.2 Shake Additive Clamping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 simultaneous shake sources | Total offset <= max_amp | R-11.3.1 |

### TC-11.3.1.3 Shake Reduced Motion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enable reduced_motion, attenuation=0.0 | CameraShakeOffset = zero | R-11.3.1 |

### TC-11.3.6.1 Overlay Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Overlay: 0.5s fade-in, 1s sustain, 0.5s fade-out; sample at t=0.25 | Opacity = 0.5 (fade-in) | R-11.3.6 |
| 2 | Sample at t=1.0 | Opacity = 1.0 (sustain) | R-11.3.6 |

### TC-11.3.6.2 Overlay Count Cap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate overlays beyond platform max | Allocation fails for excess overlays | R-11.3.6 |

### TC-11.4.1.1 Weather State Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transition from Clear to Rain | intensity ramps up, rain systems activate | R-11.4.1 |

### TC-11.4.2.1 Puddle Accumulate Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Accumulate puddle for 5s at intensity=1.0, then stop rain | Depth increases during rain, drains to 0 after | R-11.4.2 |

### TC-11.4.2.2 Wet Surface Material

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stone, wetness=1.0 | roughness_override = 0.2 (1.0 - 0.8) | R-11.4.2 |
| 2 | Metal, wetness=1.0 | roughness_override = 0.05 (1.0 - 0.95) | R-11.4.2 |

### TC-11.4.3.1 Snow Deformation Fade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stamp deformation, continue snowfall for 10s | Trail depth decreases toward zero | R-11.4.3 |

### TC-11.4.5.1 Lightning Branch Depth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate bolt on mobile tier | Max branch depth = 2 | R-11.4.5 |
| 2 | Generate bolt on desktop tier | Max branch depth = 4 | R-11.4.5 |

### TC-11.5.1.1 Debris Budget Cap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger destructions exceeding global budget | Concurrent fragments never exceed platform cap | R-11.5.1 |

### TC-11.5.2.1 Dust Color by Material

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Destroy Stone object | Dust color = (0.6, 0.6, 0.6) | R-11.5.2 |
| 2 | Destroy Wood object | Dust color = (0.5, 0.35, 0.2) | R-11.5.2 |

### TC-11.5.3.1 Spark Color Fade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spark at t=0 | Color = white | R-11.5.3 |
| 2 | Spark at t=0.5*lifetime | Color = orange | R-11.5.3 |
| 3 | Spark at t=lifetime | Color = dark/faded | R-11.5.3 |

### TC-11.5.4.1 Crack Growth Rate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | accumulated_damage=50, growth_speed=1.0, dt=1.0 | radius increases by 0.5 (50/100 * 1.0 * 1.0) | R-11.5.4 |

### TC-11.5.5.1 Scorch Persistence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scorch mark placed alongside transient decal | Scorch renders above transient in priority | R-11.5.5 |

### TC-11.5.6.1 Shockwave Expansion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shockwave at origin, speed=50, after 1 second | current_radius approximately 50 | R-11.5.6 |

### TC-11.5.7.1 Fire Material Blocking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ignite wood zone adjacent to stone zone | Fire spreads on wood, does not spread to stone | R-11.5.7 |

### TC-11.5.0.1 Budget Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | EffectBudget for Mobile tier | Debris cap=32, shockwave cap=1 | F-11.5.1 |
| 2 | EffectBudget for Desktop tier | Debris cap=256, shockwave cap=4 | F-11.5.1 |

## Integration Tests

### TC-11.2.1.I1 Decal GBuffer Blend

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deferred decal across mesh/terrain boundary | G-buffer modified within OBB, unmodified outside | R-11.2.1 |

### TC-11.2.1.I2 Decal Angle Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Decal on 80-degree surface | Opacity attenuated to near zero | R-11.2.1 |

### TC-11.2.3.I1 Atlas 500 Decals

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 decals, 50 unique textures | Draw calls proportional to atlas pages, not decal count | R-11.2.3 |

### TC-11.2.6.I1 Footprint Per Material

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk across mud, snow, sand | Footprint shape and material response differ per surface | R-11.2.6 |

### TC-11.3.2.I1 Motion Blur Framerate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Render at 30 and 60 fps | Blur width consistent (scaled by frame rate) | R-11.3.2 |

### TC-11.3.3.I1 Flare Occlusion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Partially occlude light with flare | Temporal smoothing prevents popping | R-11.3.3 |

### TC-11.4.1.I1 Rain Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rain at full intensity | Streaks, droplets, ripples all active | R-11.4.1 |
| 2 | Move under shelter | Droplets cease | R-11.4.1 |

### TC-11.4.2.I1 Puddle Dynamic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rain over varied terrain for 30s | Puddles in concavities, dry on ridges | R-11.4.2 |

### TC-11.4.3.I1 Snow Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Snow for 60s, walk through | Accumulation on upward surfaces, deformation trails | R-11.4.3 |

### TC-11.4.4.I1 Fog Volume Bounds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Place box fog volume | Fog confined to bounds, outside is clear | R-11.4.4 |

### TC-11.5.1.I1 Debris Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Destroy object with 10-entry debris table | Fragments spawn with correct material and velocity | R-11.5.1 |

### TC-11.5.2.I1 Smoke Wind

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Destroy structure with active wind field | Smoke drift aligns with wind within 15 degrees | R-11.5.2 |

### TC-11.5.6.I1 Shockwave Composite

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two overlapping explosions | Combined distortion does not exceed configured max | R-11.5.6 |

### TC-11.5.7.I1 Fire Spread Wind

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ignite surface with wind blowing east | Spread rate increases eastward | R-11.5.7 |

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
