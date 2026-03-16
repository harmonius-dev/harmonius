# 11.5 — Destruction VFX

## Debris and Fragments

### F-11.5.1 Debris Spawning

Event-driven debris particle and mesh fragment emission triggered by destruction events. Debris
meshes are produced by the physics fracture system (F-4.6.1). Debris type, count, velocity cone, and
material are selected from the destroyed object's debris table. Fragments inherit the source
object's material and are rendered as mesh particles with physics-driven trajectories. A global
debris budget caps concurrent fragments to keep frame time bounded during large siege battles.

- **Requirements:** R-11.5.1
- **Dependencies:** F-11.1.1, F-11.1.6, F-4.6.1 (Voronoi Fracture)
- **Platform notes:** Mobile debris budget is 32 fragments (vs. 256 on desktop). Mobile uses sprite
  imposters instead of mesh fragments for distant destruction.

### F-11.5.2 Dust Clouds and Smoke Plumes

Volumetric dust and smoke effects spawned at destruction sites. Dust clouds use fast-expanding
low-opacity particle bursts for the initial impact, transitioning to slower drifting smoke plumes
that persist and dissipate over time. Particle color and density are derived from the destroyed
material (stone produces grey dust, wood produces brown). Smoke plumes interact with the wind field
for natural drift across the battlefield.

- **Requirements:** R-11.5.2
- **Dependencies:** F-11.1.1, F-11.1.3
- **Platform notes:** Mobile uses reduced particle count and skips wind-field interaction for smoke
  drift. Plume persistence is shorter on mobile (5 s vs. 15 s).

### F-11.5.3 Sparks and Embers

Short-lived bright particle bursts for metallic impacts, electrical failures, and fire-related
destruction. Sparks use high-velocity point emitters with gravity, bounce collision, and rapid color
fade from white-hot to orange to dark. Embers are slower, longer-lived particles that drift upward
from burning wreckage with flickering emissive intensity. Both contribute to the clustered light
buffer via F-11.1.6.

- **Requirements:** R-11.5.3
- **Dependencies:** F-11.1.1, F-11.1.6 (Particle Light Emission)
- **Platform notes:** Mobile disables particle-emitted lights for sparks/embers. Spark count reduced
  to 25% of desktop. Ember bounce collision disabled on mobile.

## Surface Damage

### F-11.5.4 Structural Cracking Overlays

Animated crack decals that spread outward from impact points over time, visualizing the physics
stress propagation system (F-4.6.5). Crack patterns are generated from pre-authored directional
crack atlases selected by surface material. Crack growth speed and branching density scale with
accumulated damage, providing visual warning before full structural collapse during siege events.

- **Requirements:** R-11.5.4
- **Dependencies:** F-11.2.1, F-11.2.5, F-4.6.5 (Stress Propagation)
- **Platform notes:** Mobile uses static crack decals (no animated growth) to avoid per-frame decal
  updates. Fewer branching variants on mobile atlas.

### F-11.5.5 Persistent Scorch Marks

Long-lived burn decals placed at explosion and fire damage sites. Scorch marks modify albedo
(darkened), roughness (increased), and normal (flattened) channels of the underlying surface. Marks
persist across the decal lifecycle budget and are prioritized above transient combat decals to
maintain battlefield readability. Intensity fades gradually over a configurable world-time duration.

- **Requirements:** R-11.5.5
- **Dependencies:** F-11.2.1, F-11.2.6
- **Platform notes:** Mobile modifies albedo only (skips roughness and normal channels) to reduce
  G-buffer writes. Persistence duration shorter on mobile.

## Explosive and Fire Effects

### F-11.5.6 Explosion Shockwaves

Expanding spherical distortion wave rendered as a screen-space refraction ring that propagates
outward from the detonation point. The shockwave displaces the color buffer radially, kicks up a
ring of dust particles at ground contact, and applies a brief camera shake scaled by distance.
Multiple overlapping shockwaves composite their distortion additively, capped to prevent excessive
screen warping.

- **Requirements:** R-11.5.6
- **Dependencies:** F-11.3.1, F-11.3.5
- **Platform notes:** Mobile skips screen-space distortion ring; uses camera shake and dust
  particles only. Concurrent shockwave cap is 1 on mobile (vs. 4 on desktop).

### F-11.5.7 Fire Propagation Visuals

Surface-spreading fire effect driven by a propagation map that tracks burn state per texel across
destructible surfaces. Active fire regions spawn flame particles, emit light, and overlay emissive
burn textures on the surface. Fire spreads along flammable material connections at a rate influenced
by wind and material type, producing dynamic burn lines across wooden structures and vegetation
during siege warfare.

- **Requirements:** R-11.5.7
- **Dependencies:** F-11.1.1, F-11.1.6 (Particle Light Emission), F-11.2.1
- **Platform notes:** Mobile uses lower-resolution propagation maps (quarter texel density) and
  skips wind-influenced spread. Fire light emission capped to 2 sources on mobile.

## Voxel and SDF Integration

### F-11.5.8 Voxel Destruction VFX

Particle emission triggered by voxel edit operations (dig, blast, sculpt) via the `VoxelVfxSystem`.
When the voxel volume (F-3.2.13) is modified, the system reads the voxel material ID at the edit
site and selects material-appropriate particle color, texture, and behavior from a configurable
material-to-VFX lookup table. Dirt edits produce brown dust, stone produces grey chips and sparks,
ice produces translucent shards, and wood produces splinters. Particle emission scales with edit
volume — larger excavations produce more particles. Emission positions are distributed across the
SDF edit boundary surface rather than at a single point, creating a natural excavation look.

- **Requirements:** R-11.5.8
- **Dependencies:** F-11.1.1, F-3.2.13 (Runtime Voxel Editing), F-3.2.9 (Voxel Volume)
- **Platform notes:** Mobile uses sprite particles only with reduced count (25% of desktop). Switch
  uses 50% particle count. Desktop uses full mesh particle fragments for stone and ice materials.

### F-11.5.9 SDF-Driven Particle Collision

Particles collide with SDF volumes in world-space using sphere-traced distance queries rather than
mesh collision. The `SdfParticleCollisionModule` evaluates the signed distance field at each
particle position per frame. When the signed distance is negative (inside the surface), the particle
is pushed to the surface along the SDF gradient (surface normal). This is cheaper than mesh-based
particle collision and automatically handles deformed terrain, carved caves, and runtime voxel edits
without requiring collision mesh regeneration. Collision response supports configurable restitution
and friction per material. SDF collision works alongside depth-buffer collision (F-11.1.2) — SDF
handles world-space volumes while depth-buffer handles screen-space geometry.

- **Requirements:** R-11.5.9
- **Dependencies:** F-11.1.1, F-11.1.2, F-3.2.9 (Voxel Volume)
- **Platform notes:** Mobile disables SDF particle collision (uses depth-buffer only). Switch
  evaluates SDF at half particle count (alternating frames). Desktop evaluates SDF for all colliding
  particles every frame.

### F-11.5.10 Debris Material Matching

Destruction debris fragments and associated VFX inherit material properties from the parent object's
`PhysicsMaterial` surface type tag (F-4.2.9). The `DebrisMaterialSystem` queries the parent entity's
material at the moment of fracture and selects material-specific VFX presets: wood produces
splinters and sawdust with brown dust trails, stone produces chips and grey dust, metal produces
sparks and metallic shards with ringing audio cue, glass produces translucent shards with glittering
refraction, and ice produces frosted chunks with mist. Each material preset defines fragment mesh
variants, particle color curves, dust opacity, and secondary effect type (sparks for metal, mist for
ice). Presets are authored as data assets and referenced from the destruction VFX profile.

- **Requirements:** R-11.5.10
- **Dependencies:** F-11.5.1, F-4.2.9 (Physics Materials), F-4.6.3
- **Platform notes:** Mobile uses a single generic debris preset per material (no secondary
  effects). Desktop uses full material-specific presets with secondary VFX layers.

### F-11.5.11 Destruction Audio-VFX Sync

Coordinated audio and VFX triggers on destruction events through a unified `DestructionEventBus`
that dispatches to both audio and VFX systems simultaneously. The `DestructionSyncSystem` reads the
destruction event (fracture activation, voxel edit, structural collapse) and emits paired audio-VFX
commands: material-based sound selection (glass shatter, wood crack, stone crumble, metal clang),
fragment count scaling audio complexity (more fragments = layered audio with more concurrent
voices), and distance-based audio LOD matching VFX LOD. Destruction events carry material ID,
fragment count, total mass, and impact energy — audio and VFX systems independently interpret these
parameters to produce synchronized results without tight coupling.

- **Requirements:** R-11.5.11
- **Dependencies:** F-11.5.1, F-4.6.3, F-5.1.3 (Mixer Bus)
- **Platform notes:** Mobile limits concurrent destruction audio voices to 2 (vs. 8 on desktop).
  Audio complexity scaling disabled on mobile — uses single-shot sounds regardless of fragment
  count.
