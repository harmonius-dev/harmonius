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

### TC-11.1.1.1 GPU Spawn Shape Coverage

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |
| 2 | R-11.1.1    |
| 3 | R-11.1.1    |

1. **#1** — Compile spawn kernel for each `SpawnShape` variant (Point, Sphere, Box, Cone,
   MeshSurface)
   - **Expected:** Codegen emits distinct HLSL per variant, compile succeeds
2. **#2** — Dispatch 10,000-particle spawn with Sphere radius=1.0
   - **Expected:** All particle positions satisfy `length(pos - origin) <= 1.0`
3. **#3** — Measure CPU readback during spawn frame
   - **Expected:** Zero CPU readback of particle buffer

### TC-11.1.3.1 Sprite Billboard Blend Modes

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |
| 2 | R-11.1.3    |
| 3 | R-11.1.3    |

1. **#1** — `SpriteOutputConfig { blend: Additive }`, quad centered on camera axis
   - **Expected:** Draw call uses SrcAlpha/One blend state
2. **#2** — `SpriteOutputConfig { blend: Alpha }`
   - **Expected:** Draw call uses SrcAlpha/InvSrcAlpha blend state
3. **#3** — `SpriteOutputConfig { soft_particle: true, depth_fade: 0.5 }`
   - **Expected:** Shader samples depth buffer, fades alpha within 0.5m of scene depth

<!-- THIN: design section lacks detail -->
### TC-11.2.2.1 Mesh Decal Tangent Space Normals

| # | Requirement |
|---|-------------|
| 1 | R-11.2.2    |
| 2 | R-11.2.2    |

1. **#1** — Generate mesh decal with `ProjectionKind::Mesh` on curved source mesh with known tangent
   frame
   - **Expected:** Decal vertex tangents match source tangents within 0.001
2. **#2** — Sample decal normal map at UV (0.5, 0.5)
   - **Expected:** World-space normal reconstructed via TBN matches expected vector within 1 degree

<!-- THIN: design section lacks detail -->
### TC-11.2.5.1 Damage Decal Weapon Variation

| # | Requirement |
|---|-------------|
| 1 | R-11.2.5    |
| 2 | R-11.2.5    |

1. **#1** — Spawn damage decal from `HitEvent { weapon: Sword, angle: 45 }`
   - **Expected:** Atlas entry selected matches sword-slash variant, rotation set to 45 degrees
2. **#2** — Spawn 10 decals from identical events
   - **Expected:** At least 3 distinct atlas variants used (non-repetitive)

<!-- THIN: design section lacks detail -->
### TC-11.3.4.1 Chromatic Aberration Radial Offset

| # | Requirement |
|---|-------------|
| 1 | R-11.3.4    |
| 2 | R-11.3.4    |

1. **#1** — Enable chromatic aberration intensity=1.0, sample at screen edge UV=(1,0.5)
   - **Expected:** Red and blue channels offset radially by configured pixel delta, green channel
     unchanged
2. **#2** — Sample at screen center UV=(0.5,0.5)
   - **Expected:** Offset magnitude = 0 (no aberration at center)

<!-- THIN: design section lacks detail -->
### TC-11.4.6.1 Wind Debris Velocity From WindField

| # | Requirement |
|---|-------------|
| 1 | R-11.4.6    |
| 2 | R-11.4.6    |

1. **#1** — Sample WindField at (0,0,0) returns (5,0,0), spawn 100 leaf particles at origin
   - **Expected:** Per-particle velocity initial component x within [4.5, 5.5] (wind + small
     variance)
2. **#2** — Set WindField to zero, advance 1s
   - **Expected:** Particle velocities damped toward zero via drag term

<!-- THIN: design section lacks detail -->
### TC-11.4.7.1 Underwater Caustic Pattern Update

| # | Requirement |
|---|-------------|
| 1 | R-11.4.7    |
| 2 | R-11.4.7    |

1. **#1** — Enable underwater mode at depth=5m, advance simulation 1s
   - **Expected:** Caustic texture scroll offset advanced by `scroll_speed * 1.0`
2. **#2** — Sample depth fog at view distance 20m
   - **Expected:** Blue channel attenuation matches exp(-0.1 * 20) within 1%

### TC-11.6.1.1 Effect Graph Validation Dangling Pin

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |
| 2 | R-11.6.1    |

1. **#1** — Build graph with `Spawn -> Update` but `Update.velocity` pin unconnected and required
   - **Expected:** `GraphValidator::validate()` returns
     `Err(ValidationError::DanglingRequired { node, pin })`
2. **#2** — Connect pin and revalidate
   - **Expected:** `Ok(ValidatedGraph { .. })`

### TC-11.6.2.1 Custom Node Library Load

| # | Requirement |
|---|-------------|
| 1 | R-11.6.2    |

1. **#1** — Load middleman .dylib containing custom node with float input `strength` and vector
   output `force`, reference via `CustomNodeRef(7)`
   - **Expected:** Node palette lists node, compile produces kernel calling custom HLSL function at
     index 7

### TC-11.6.3.1 Parameter Override Per Instance

| # | Requirement |
|---|-------------|
| 1 | R-11.6.3    |
| 2 | R-11.6.3    |

1. **#1** — Compile graph with color parameter default=red, spawn instance A with no override
   - **Expected:** Instance A `param_buffer.color == (1,0,0,1)`
2. **#2** — Spawn instance B with `ParamOverride::Color((0,0,1,1))`
   - **Expected:** Instance B `param_buffer.color == (0,0,1,1)`, instance A unchanged

### TC-11.6.4.1 Observer Event Spawn Parameterization

| # | Requirement |
|---|-------------|
| 1 | R-11.6.4    |
| 2 | R-11.6.4    |

1. **#1** — Register `VfxSpawnEvent` observer for `CollisionEvent`, emit collision with
   `position=(3,0,0)`, `normal=(0,1,0)`
   - **Expected:** One `EffectInstanceComponent` created with transform origin=(3,0,0), orientation
     aligned to normal
2. **#2** — Emit collision with `material=Stone`
   - **Expected:** Spawned instance selects stone-variant param override

### TC-11.6.5.1 LOD Tier Selection By Distance

| # | Requirement |
|---|-------------|
| 1 | R-11.6.5    |
| 2 | R-11.6.5    |
| 3 | R-11.6.5    |

1. **#1** — Place emitter at distance=5m from camera, LOD thresholds `[10, 30, 80]`
   - **Expected:** `EmitterLodComponent.current_tier == LodTier::Full`
2. **#2** — Move emitter to distance=50m
   - **Expected:** `current_tier == LodTier::Impostor`
3. **#3** — Move emitter to distance=100m
   - **Expected:** `current_tier == LodTier::Culled`

### TC-15.8.1.1 Logic Graph Native Compile

| # | Requirement |
|---|-------------|
| 1 | R-15.8.1    |
| 2 | R-15.8.1    |

1. **#1** — Compile graph `add(a, b) -> result` to native code for VFX parameter evaluation
   - **Expected:** Compiled kernel produces bit-identical output to hand Rust `a + b` over 1000
     random float pairs
2. **#2** — Benchmark compiled graph vs hand Rust
   - **Expected:** Overhead under 5%

### TC-15.8.5b.1 Shader Graph HLSL Compile

| # | Requirement |
|---|-------------|
| 1 | R-15.8.5b   |
| 2 | R-15.8.5b   |

1. **#1** — Author shader graph with PBR inputs (albedo, normal, roughness), invoke
   `EffectGraphCompiler` with `PlatformTier::Desktop`
   - **Expected:** DXC produces DXIL bytecode, no errors
2. **#2** — Same graph with `PlatformTier::Apple`
   - **Expected:** DXC -> MSC produces MSL bytecode, no errors

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

### TC-11.2.1.I3 Decal Forward Path Fallback

| # | Requirement  |
|---|--------------|
| 1 | US-11.2.1.3  |

1. **#1** — Disable deferred renderer, spawn decal with `ProjectionKind::Deferred`
   - **Expected:** System substitutes `ProjectionKind::Mesh`, decal visible on target mesh with
     correct UVs

### TC-11.2.2.I1 Mesh Decal On Curved Surface

| # | Requirement  |
|---|--------------|
| 1 | US-11.2.2.1  |
| 2 | US-11.2.2.2  |

1. **#1** — Place mesh decal on curved cylinder mesh, desktop tier
   - **Expected:** Generated mesh conforms to cylinder, tangent normals produce correct lighting at
     4 sample points
2. **#2** — Same spawn on mobile tier
   - **Expected:** Falls back to deferred decal, no mesh generation occurs

### TC-11.2.3.I2 Atlas LRU Under Memory Pressure

| # | Requirement  |
|---|--------------|
| 1 | US-11.2.3.2  |

1. **#1** — Pack atlas to 90% capacity, request 20 new textures that collide with existing LRU
   entries
   - **Expected:** Oldest LRU entries evicted, new textures packed, total atlas bytes stays under
     configured limit

### TC-11.2.4.I1 Decal Priority Blend Stack

| # | Requirement  |
|---|--------------|
| 1 | US-11.2.4.1  |
| 2 | US-11.2.4.2  |

1. **#1** — Place blood decal priority=2 alpha-blend, scorch priority=4 multiply, footprint
   priority=1 alpha on same surface patch
   - **Expected:** G-buffer albedo shows footprint first, blood over, scorch darkening on top; no
     z-fighting observed
2. **#2** — Advance lifecycle to dissolve phase
   - **Expected:** Opacity attenuated via noise mask, decals fade without pop

### TC-11.2.5.I1 Blood Decal From Hit Event

| # | Requirement  |
|---|--------------|
| 1 | US-11.2.5.1  |
| 2 | US-11.2.5.2  |

1. **#1** — Emit `HitEvent { weapon: Sword, impact, surface: Flesh }`
   - **Expected:** Blood decal spawned at impact with sword-slash atlas variant, priority set from
     material table
2. **#2** — Set content rating profile to `restricted`, emit identical event
   - **Expected:** No blood decal spawned, non-blood hit decal substitute used

### TC-11.2.6.I2 Tire Tracks Along Vehicle Path

| # | Requirement  |
|---|--------------|
| 1 | US-11.2.6.2  |
| 2 | US-11.2.6.3  |

1. **#1** — Drive vehicle 20 meters on mud surface, desktop tier
   - **Expected:** `DecalStrip` produces continuous ribbon of quads matching wheel contact patch
     width
2. **#2** — Walk character 10 meters on mobile tier
   - **Expected:** Footprint decals spawn every 4th step (not every step)

### TC-11.3.1.I1 Shake Additive From Multiple Sources

| # | Requirement  |
|---|--------------|
| 1 | US-11.3.1.1  |
| 2 | US-11.3.1.2  |
| 3 | US-11.3.1.3  |

1. **#1** — Trigger 3 shake sources amplitudes 0.5, 0.8, 0.3 with Perlin driver
   - **Expected:** Combined `CameraShakeOffset.magnitude <= max_amp` each frame
2. **#2** — Enable reduced-motion setting
   - **Expected:** Effective amplitude scaled to zero
3. **#3** — Advance 3 seconds, decay_rate=1.0
   - **Expected:** All sources expired, offset returns to zero

### TC-11.3.2.I2 Motion Blur Per Object And Camera

| # | Requirement  |
|---|--------------|
| 1 | US-11.3.2.1  |
| 2 | US-11.3.2.3  |

1. **#1** — Sword swing at 30 m/s in front of stationary camera, desktop tier
   - **Expected:** Per-object motion vectors produce blur aligned to swing direction in velocity
     buffer
2. **#2** — Same swing on mobile tier
   - **Expected:** Motion blur pass disabled, no blur applied

### TC-11.3.3.I2 Lens Flare Artist Template

| # | Requirement  |
|---|--------------|
| 1 | US-11.3.3.1  |
| 2 | US-11.3.3.2  |

1. **#1** — Apply `LensFlareTemplate` with 3 elements: ghost, halo, starburst facing a bright
   directional light
   - **Expected:** Rendered flare shows 3 distinct elements aligned along light-screen axis
2. **#2** — Partially occlude light source
   - **Expected:** Element intensities scaled by occlusion factor with temporal smoothing (no pop)

### TC-11.3.4.I1 Chromatic Grain Vignette Composite

| # | Requirement  |
|---|--------------|
| 1 | US-11.3.4.1  |
| 2 | US-11.3.4.2  |

1. **#1** — Enable chromatic aberration pulse triggered by damage event, film grain intensity=0.5,
   vignette intensity=0.8
   - **Expected:** Post-process output shows radial RGB offset at screen edges, animated grain
     pattern, and darkened corners
2. **#2** — Disable pulse after 0.3s decay
   - **Expected:** Aberration intensity returns to baseline, grain and vignette persist

### TC-11.3.5.I1 Heat Haze Accumulated Distortion

| # | Requirement  |
|---|--------------|
| 1 | US-11.3.5.1  |
| 2 | US-11.3.5.2  |

1. **#1** — Place 5 `DistortionSource` heat haze effects within half-res buffer region
   - **Expected:** Accumulated distortion vector buffer shows summed scrolling normal contributions,
     cost bounded by buffer resolution
2. **#2** — Sample distortion at viewport center
   - **Expected:** Vector magnitude within configured max displacement

### TC-11.3.6.I1 Damage Overlay Per Type Lifecycle

| # | Requirement  |
|---|--------------|
| 1 | US-11.3.6.1  |
| 2 | US-11.3.6.2  |
| 3 | US-11.3.6.3  |

1. **#1** — Trigger fire overlay with 0.5s fade-in
   - **Expected:** Fire overlay opacity ramps from 0 to 1 over 0.5s
2. **#2** — Trigger ice overlay concurrently on desktop
   - **Expected:** Both overlays active, independent timers
3. **#3** — Trigger 5 overlays on mobile (cap=2)
   - **Expected:** Only 2 concurrent overlays active, total opacity clamped

### TC-11.4.1.I2 Rain Density Scales By Intensity

| # | Requirement  |
|---|--------------|
| 1 | US-11.4.1.2  |
| 2 | US-11.4.1.3  |

1. **#1** — Set WeatherState intensity=0.2 (drizzle), desktop tier
   - **Expected:** Particle count scales to 20% of max, screen droplet frequency reduced
     proportionally
2. **#2** — Set intensity=1.0 on mobile tier
   - **Expected:** Single particle layer active, 25% density, no screen droplets

### TC-11.4.2.I2 Puddle Per Material Wet Response

| # | Requirement  |
|---|--------------|
| 1 | US-11.4.2.1  |
| 2 | US-11.4.2.2  |
| 3 | US-11.4.2.3  |

1. **#1** — Accumulate puddles on stone patch, desktop tier
   - **Expected:** Heightfield grows, stone roughness override drops to 0.2
2. **#2** — Same rainfall on metal patch
   - **Expected:** Roughness override drops to 0.05, reflection visible
3. **#3** — Mobile tier same scene
   - **Expected:** Pre-placed puddle decals with albedo darken only, no heightfield update

### TC-11.4.3.I2 Snow Accumulation And Deformation

| # | Requirement  |
|---|--------------|
| 1 | US-11.4.3.1  |
| 2 | US-11.4.3.2  |
| 3 | US-11.4.3.3  |

1. **#1** — Enable snowfall for 60s on mixed upward/vertical terrain
   - **Expected:** Height texture grows on upward faces only, max_depth clamped
2. **#2** — Walk character across snow
   - **Expected:** Deformation trail written into height texture, fades via `deformation_fade`
3. **#3** — Mobile tier
   - **Expected:** Texture-blend snow with decal-based deformation, no vertex displacement

### TC-11.4.4.I2 Fog Volume Froxel Injection

| # | Requirement  |
|---|--------------|
| 1 | US-11.4.4.1  |
| 2 | US-11.4.4.2  |
| 3 | US-11.4.4.3  |

1. **#1** — Place box `FogVolume` density=0.5 color=green, desktop tier
   - **Expected:** Froxel grid cells within box contain density=0.5 color=green
2. **#2** — Verify temporal reprojection blends with prior frame
   - **Expected:** Frame-over-frame variance under 5%
3. **#3** — Mobile tier
   - **Expected:** Screen-space height fog substituted, no froxel writes

### TC-11.4.5.I1 Lightning Multi Bolt Storm

| # | Requirement  |
|---|--------------|
| 1 | US-11.4.5.1  |
| 2 | US-11.4.5.2  |
| 3 | US-11.4.5.3  |

1. **#1** — Trigger thunderstorm with max_bolts=4, desktop
   - **Expected:** Up to 4 `LightningBolt` instances active, each with L-system branching,
     single-frame illumination, exponential decay
2. **#2** — Mobile tier same trigger
   - **Expected:** max_bolts=1, branch_depth=2, directional flash only
3. **#3** — Distance-delayed thunder event fired
   - **Expected:** Audio delayed by `distance / speed_of_sound`

### TC-11.4.6.I1 Dust Storm And Wind Debris

| # | Requirement  |
|---|--------------|
| 1 | US-11.4.6.1  |
| 2 | US-11.4.6.2  |
| 3 | US-11.4.6.3  |

1. **#1** — Enable dust storm with wind field (5,0,0), desktop
   - **Expected:** Leaf/debris particles sample WindField for velocity, density injected into
     atmospheric scattering with sky tint
2. **#2** — Mobile tier
   - **Expected:** 10% particle count, distance fog substitution, no density injection

### TC-11.4.7.I1 Underwater Full Pipeline

| # | Requirement  |
|---|--------------|
| 1 | US-11.4.7.1  |
| 2 | US-11.4.7.2  |
| 3 | US-11.4.7.3  |

1. **#1** — Submerge camera, desktop tier
   - **Expected:** Caustic light pattern on submerged geometry, bubble streams active,
     wavelength-dependent depth fog, god rays from surface
2. **#2** — Sample pixel at water surface boundary
   - **Expected:** Refraction distortion applied to underwater view
3. **#3** — Mobile tier
   - **Expected:** Caustics + god rays skipped, depth fog + blue tint only, 25% bubble count

### TC-11.5.1.I2 Debris Material Inheritance

| # | Requirement  |
|---|--------------|
| 1 | US-11.5.1.1  |
| 2 | US-11.5.1.2  |

1. **#1** — Destroy stone wall entity with `DebrisTable` containing 4 stone fragment entries
   - **Expected:** Spawned `DebrisFragment` components inherit `SurfaceMaterial::Stone`, mesh assets
     match table entries
2. **#2** — Verify velocity cone
   - **Expected:** Fragment velocities lie within configured cone angle

### TC-11.5.2.I2 Smoke Plume Material Color

| # | Requirement  |
|---|--------------|
| 1 | US-11.5.2.1  |
| 2 | US-11.5.2.3  |

1. **#1** — Destroy wood object on desktop
   - **Expected:** Smoke plume spawned with brown tint, drifts with wind field direction
2. **#2** — Mobile tier
   - **Expected:** Plume skips wind interaction, shorter persistence

### TC-11.5.3.I1 Spark Bounce With Gravity

| # | Requirement  |
|---|--------------|
| 1 | US-11.5.3.1  |
| 2 | US-11.5.3.2  |
| 3 | US-11.5.3.3  |

1. **#1** — Metal impact spawns 20 sparks with gravity=-9.8, restitution=0.5
   - **Expected:** Sparks bounce off ground plane with velocity reduced by 0.5x, color fades white
     -> orange -> dark within lifetime
2. **#2** — Ember particles rising from residual burn
   - **Expected:** Emissive intensity flickers via noise, particle lights added to cluster buffer on
     desktop
3. **#3** — Mobile tier
   - **Expected:** No particle lights, spark count 25% of desktop

### TC-11.5.4.I1 Crack Growth Damage Scaling

| # | Requirement  |
|---|--------------|
| 1 | US-11.5.4.1  |
| 2 | US-11.5.4.2  |
| 3 | US-11.5.4.3  |

1. **#1** — Apply repeated damage to stone wall, accumulated_damage grows from 0 to 100
   - **Expected:** Crack radius grows monotonically, atlas selected = stone-directional-crack
2. **#2** — Apply identical damage to wood
   - **Expected:** Atlas selected = wood-directional-crack
3. **#3** — Mobile tier
   - **Expected:** Static crack decal, no per-frame radius animation

### TC-11.5.5.I1 Scorch Persistence Material Override

| # | Requirement  |
|---|--------------|
| 1 | US-11.5.5.1  |
| 2 | US-11.5.5.2  |

1. **#1** — Place scorch decal on stone surface, desktop
   - **Expected:** G-buffer writes albedo darken, roughness += 0.3, normal flatten; persists after
     frame
2. **#2** — Mobile tier
   - **Expected:** Albedo darken only, shorter lifecycle duration

### TC-11.5.6.I2 Shockwave Refraction With Shake

| # | Requirement  |
|---|--------------|
| 1 | US-11.5.6.1  |
| 2 | US-11.5.6.2  |
| 3 | US-11.5.6.3  |

1. **#1** — Trigger shockwave origin=(0,0,0), camera 10m away, desktop
   - **Expected:** Screen-space radial refraction applied, dust ring spawned at ground contact,
     camera shake intensity scaled by distance
2. **#2** — Spawn 4 overlapping shockwaves
   - **Expected:** Combined distortion clamped to max displacement
3. **#3** — Mobile tier
   - **Expected:** Refraction skipped, max 1 concurrent shockwave

### TC-11.5.7.I2 Fire Emissive Light Spread

| # | Requirement  |
|---|--------------|
| 1 | US-11.5.7.1  |
| 2 | US-11.5.7.2  |
| 3 | US-11.5.7.3  |

1. **#1** — Ignite wooden tower, desktop tier, advance 5s
   - **Expected:** Flame particles spawned, emissive overlay on burn region, propagation map spreads
     across fuel texels guided by wind
2. **#2** — Count active point lights
   - **Expected:** Lights spawned but capped per cluster tile
3. **#3** — Mobile tier
   - **Expected:** Quarter texel density, no wind spread, max 2 fire lights

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

## Sub-story and Variant Trace

The upstream design lists the following refined sub-stories and letter-variant stories. Each is
covered by the parent-ID TC rows above; a regression in any parent TC constitutes a regression
against the listed sub-story or variant.

- US-11.2.1.1
- US-11.2.3.1
- US-11.2.4.3
- US-11.2.6.1
- US-11.3.2.2
- US-11.3.3.3
- US-11.4.1.1
- US-11.5.1.3
- US-11.5.2.2
