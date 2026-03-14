# 11.5 — Destruction VFX

## Debris and Fragments

### F-11.5.1 Debris Spawning

Event-driven debris particle and mesh fragment emission triggered by destruction events. Debris
type, count, velocity cone, and material are selected from the destroyed object's debris table.
Fragments inherit the source object's material and are rendered as mesh particles with physics-
driven trajectories. A global debris budget caps concurrent fragments to keep frame time bounded
during large siege battles.

- **Requirements:** R-11.5.1
- **Dependencies:** F-11.1.1, F-11.1.6
- **Platform notes:** None

### F-11.5.2 Dust Clouds and Smoke Plumes

Volumetric dust and smoke effects spawned at destruction sites. Dust clouds use fast-expanding
low-opacity particle bursts for the initial impact, transitioning to slower drifting smoke plumes
that persist and dissipate over time. Particle color and density are derived from the destroyed
material (stone produces grey dust, wood produces brown). Smoke plumes interact with the wind
field for natural drift across the battlefield.

- **Requirements:** R-11.5.2
- **Dependencies:** F-11.1.1, F-11.1.3
- **Platform notes:** None

### F-11.5.3 Sparks and Embers

Short-lived bright particle bursts for metallic impacts, electrical failures, and fire-related
destruction. Sparks use high-velocity point emitters with gravity, bounce collision, and rapid
color fade from white-hot to orange to dark. Embers are slower, longer-lived particles that drift
upward from burning wreckage with flickering emissive intensity. Both contribute to the clustered
light buffer via F-11.1.10.

- **Requirements:** R-11.5.3
- **Dependencies:** F-11.1.1, F-11.1.10, F-11.1.11
- **Platform notes:** None

## Surface Damage

### F-11.5.4 Structural Cracking Overlays

Animated crack decals that spread outward from impact points over time, simulating progressive
structural failure. Crack patterns are generated from pre-authored directional crack atlases
selected by surface material. Crack growth speed and branching density scale with accumulated
damage, providing visual warning before full structural collapse during siege events.

- **Requirements:** R-11.5.4
- **Dependencies:** F-11.2.1, F-11.2.5
- **Platform notes:** None

### F-11.5.5 Persistent Scorch Marks

Long-lived burn decals placed at explosion and fire damage sites. Scorch marks modify albedo
(darkened), roughness (increased), and normal (flattened) channels of the underlying surface.
Marks persist across the decal lifecycle budget and are prioritized above transient combat decals
to maintain battlefield readability. Intensity fades gradually over a configurable world-time
duration.

- **Requirements:** R-11.5.5
- **Dependencies:** F-11.2.1, F-11.2.6
- **Platform notes:** None

## Explosive and Fire Effects

### F-11.5.6 Explosion Shockwaves

Expanding spherical distortion wave rendered as a screen-space refraction ring that propagates
outward from the detonation point. The shockwave displaces the color buffer radially, kicks up a
ring of dust particles at ground contact, and applies a brief camera shake scaled by distance.
Multiple overlapping shockwaves composite their distortion additively, capped to prevent excessive
screen warping.

- **Requirements:** R-11.5.6
- **Dependencies:** F-11.3.1, F-11.3.5
- **Platform notes:** None

### F-11.5.7 Fire Propagation Visuals

Surface-spreading fire effect driven by a propagation map that tracks burn state per texel across
destructible surfaces. Active fire regions spawn flame particles, emit light, and overlay emissive
burn textures on the surface. Fire spreads along flammable material connections at a rate
influenced by wind and material type, producing dynamic burn lines across wooden structures and
vegetation during siege warfare.

- **Requirements:** R-11.5.7
- **Dependencies:** F-11.1.1, F-11.1.10, F-11.2.1
- **Platform notes:** None
