# 11.5 — Destruction VFX

## Debris and Fragments

| ID       | Feature                      |
|----------|------------------------------|
| F-11.5.1 | Debris Spawning              |
| F-11.5.2 | Dust Clouds and Smoke Plumes |
| F-11.5.3 | Sparks and Embers            |

1. **F-11.5.1** — Event-driven debris particle and mesh fragment emission triggered by destruction
   events. Debris meshes are produced by the physics fracture system (F-4.6.1). Debris type, count,
   velocity cone, and material are selected from the destroyed object's debris table. Fragments
   inherit the source object's material and are rendered as mesh particles with physics-driven
   trajectories. A global debris budget caps concurrent fragments to keep frame time bounded during
   large siege battles.
   - **Deps:** F-11.1.1, F-11.1.6, F-4.6.1 (Voronoi Fracture)
   - **Platform:** Mobile debris budget is 32 fragments (vs. 256 on desktop). Mobile uses sprite
     imposters instead of mesh fragments for distant destruction.
2. **F-11.5.2** — Volumetric dust and smoke effects spawned at destruction sites. Dust clouds use
   fast-expanding low-opacity particle bursts for the initial impact, transitioning to slower
   drifting smoke plumes that persist and dissipate over time. Particle color and density are
   derived from the destroyed material (stone produces grey dust, wood produces brown). Smoke plumes
   interact with the wind field for natural drift across the battlefield.
   - **Deps:** F-11.1.1, F-11.1.3
   - **Platform:** Mobile uses reduced particle count and skips wind-field interaction for smoke
     drift. Plume persistence is shorter on mobile (5 s vs. 15 s).
3. **F-11.5.3** — Short-lived bright particle bursts for metallic impacts, electrical failures, and
   fire-related destruction. Sparks use high-velocity point emitters with gravity, bounce collision,
   and rapid color fade from white-hot to orange to dark. Embers are slower, longer-lived particles
   that drift upward from burning wreckage with flickering emissive intensity. Both contribute to
   the clustered light buffer via F-11.1.6.
   - **Deps:** F-11.1.1, F-11.1.6 (Particle Light Emission)
   - **Platform:** Mobile disables particle-emitted lights for sparks/embers. Spark count reduced to
     25% of desktop. Ember bounce collision disabled on mobile.

## Surface Damage

| ID       | Feature                      |
|----------|------------------------------|
| F-11.5.4 | Structural Cracking Overlays |
| F-11.5.5 | Persistent Scorch Marks      |

1. **F-11.5.4** — Animated crack decals that spread outward from impact points over time,
   visualizing the physics stress propagation system (F-4.6.5). Crack patterns are generated from
   pre-authored directional crack atlases selected by surface material. Crack growth speed and
   branching density scale with accumulated damage, providing visual warning before full structural
   collapse during siege events.
   - **Deps:** F-11.2.1, F-11.2.5, F-4.6.5 (Stress Propagation)
   - **Platform:** Mobile uses static crack decals (no animated growth) to avoid per-frame decal
     updates. Fewer branching variants on mobile atlas.
2. **F-11.5.5** — Long-lived burn decals placed at explosion and fire damage sites. Scorch marks
   modify albedo (darkened), roughness (increased), and normal (flattened) channels of the
   underlying surface. Marks persist across the decal lifecycle budget and are prioritized above
   transient combat decals to maintain battlefield readability. Intensity fades gradually over a
   configurable world-time duration.
   - **Deps:** F-11.2.1, F-11.2.6
   - **Platform:** Mobile modifies albedo only (skips roughness and normal channels) to reduce
     G-buffer writes. Persistence duration shorter on mobile.

## Explosive and Fire Effects

| ID       | Feature                  |
|----------|--------------------------|
| F-11.5.6 | Explosion Shockwaves     |
| F-11.5.7 | Fire Propagation Visuals |

1. **F-11.5.6** — Expanding spherical distortion wave rendered as a screen-space refraction ring
   that propagates outward from the detonation point. The shockwave displaces the color buffer
   radially, kicks up a ring of dust particles at ground contact, and applies a brief camera shake
   scaled by distance. Multiple overlapping shockwaves composite their distortion additively, capped
   to prevent excessive screen warping.
   - **Deps:** F-11.3.1, F-11.3.5
   - **Platform:** Mobile skips screen-space distortion ring; uses camera shake and dust particles
     only. Concurrent shockwave cap is 1 on mobile (vs. 4 on desktop).
2. **F-11.5.7** — Surface-spreading fire effect driven by a propagation map that tracks burn state
   per texel across destructible surfaces. Active fire regions spawn flame particles, emit light,
   and overlay emissive burn textures on the surface. Fire spreads along flammable material
   connections at a rate influenced by wind and material type, producing dynamic burn lines across
   wooden structures and vegetation during siege warfare.
   - **Deps:** F-11.1.1, F-11.1.6 (Particle Light Emission), F-11.2.1
   - **Platform:** Mobile uses lower-resolution propagation maps (quarter texel density) and skips
     wind-influenced spread. Fire light emission capped to 2 sources on mobile.

## Voxel and SDF Integration

| ID        | Feature                       |
|-----------|-------------------------------|
| F-11.5.8  | Voxel Destruction VFX         |
| F-11.5.9  | SDF-Driven Particle Collision |
| F-11.5.10 | Debris Material Matching      |
| F-11.5.11 | Destruction Audio-VFX Sync    |

1. **F-11.5.8** — Particle emission triggered by voxel edit operations (dig, blast, sculpt) via the
   `VoxelVfxSystem`. When the voxel volume (F-3.2.13) is modified, the system reads the voxel
   material ID at the edit site and selects material-appropriate particle color, texture, and
   behavior from a configurable material-to-VFX lookup table. Dirt edits produce brown dust, stone
   produces grey chips and sparks, ice produces translucent shards, and wood produces splinters.
   Particle emission scales with edit volume. Emission positions are distributed across the SDF edit
   boundary surface.
   - **Deps:** F-11.1.1, F-3.2.13 (Runtime Voxel Editing), F-3.2.9 (Voxel Volume)
   - **Platform:** Mobile uses sprite particles only with reduced count (25% of desktop). Switch
     uses 50% particle count. Desktop uses full mesh particle fragments for stone and ice materials.
2. **F-11.5.9** — Particles collide with SDF volumes in world-space using sphere-traced distance
   queries rather than mesh collision. The `SdfParticleCollisionModule` evaluates the signed
   distance field at each particle position per frame. When the signed distance is negative (inside
   the surface), the particle is pushed to the surface along the SDF gradient. Cheaper than
   mesh-based collision and handles deformed terrain, carved caves, and runtime voxel edits without
   collision mesh regeneration. Configurable restitution and friction per material. Works alongside
   depth-buffer collision (F-11.1.2).
   - **Deps:** F-11.1.1, F-11.1.2, F-3.2.9 (Voxel Volume)
   - **Platform:** Mobile disables SDF particle collision (uses depth-buffer only). Switch evaluates
     SDF at half particle count (alternating frames). Desktop evaluates SDF for all colliding
     particles every frame.
3. **F-11.5.10** — Destruction debris fragments and associated VFX inherit material properties from
   the parent object's `PhysicsMaterial` surface type tag (F-4.2.9). The `DebrisMaterialSystem`
   selects material-specific VFX presets: wood produces splinters and sawdust, stone produces chips
   and grey dust, metal produces sparks and shards, glass produces translucent shards with
   refraction, ice produces frosted chunks with mist. Each preset defines fragment mesh variants,
   particle color curves, dust opacity, and secondary effects. Presets are authored as data assets.
   - **Deps:** F-11.5.1, F-4.2.9 (Physics Materials), F-4.6.3
   - **Platform:** Mobile uses a single generic debris preset per material (no secondary effects).
     Desktop uses full material-specific presets with secondary VFX layers.
4. **F-11.5.11** — Coordinated audio and VFX triggers on destruction events through a unified
   `DestructionEventBus`. The `DestructionSyncSystem` emits paired audio-VFX commands:
   material-based sound selection, fragment count scaling audio complexity, and distance-based audio
   LOD matching VFX LOD. Destruction events carry material ID, fragment count, total mass, and
   impact energy. Audio and VFX systems independently interpret these parameters for synchronized
   results without tight coupling.
   - **Deps:** F-11.5.1, F-4.6.3, F-5.1.3 (Mixer Bus)
   - **Platform:** Mobile limits concurrent destruction audio voices to 2 (vs. 8 on desktop). Audio
     complexity scaling disabled on mobile.
