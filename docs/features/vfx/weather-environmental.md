# 11.4 — Weather & Environmental FX

## Precipitation

| ID       | Feature                                  | Requirements |
|----------|------------------------------------------|--------------|
| F-11.4.1 | Rain Particle System and Screen Droplets | R-11.4.1     |
| F-11.4.2 | Rain Puddles and Wet Surfaces            | R-11.4.2     |
| F-11.4.3 | Snow Accumulation and Interaction        | R-11.4.3     |

1. **F-11.4.1** — Multi-layered rain rendering combining GPU particle streaks for world-space
   rainfall, screen-space droplet simulation for camera-near effects, and ripple normal maps on wet
   surfaces. Particle density scales with weather intensity and camera exposure to the sky. Screen
   droplets flow downward with gravity, merge on contact, and evaporate after the storm passes.
   - **Deps:** F-11.1.1
   - **Platform:** Mobile uses a single particle layer (no screen droplets). Rain particle density
     reduced to 25% of desktop. Ripple normals use half-res on mobile.
2. **F-11.4.2** — Dynamic puddle formation driven by a heightfield accumulation buffer and terrain
   concavity. Puddles modify surface roughness toward mirror-smooth, darken albedo, and add animated
   ripple normals during active rainfall. Wet surface response is material-driven: stone darkens,
   metal reflects, and dirt turns to mud. Puddle depth drains over time after rain stops.
   - **Deps:** F-11.4.1
   - **Platform:** Mobile uses pre-placed puddle decals (no dynamic heightfield accumulation). Wet
     surface effect is albedo-darken only (no roughness modification).
3. **F-11.4.3** — Vertex-displacement snow layer that accumulates on upward-facing surfaces over
   time. Accumulation depth is stored in a world-space height texture updated by weather state.
   Character and vehicle movement carve deformation trails into the snow layer using depth stamps.
   Footprints reveal the underlying surface material and fade gradually under continued snowfall.
   - **Platform:** Mobile uses texture-blend snow (no vertex displacement). Deformation trails use
     decals instead of depth stamps on mobile. Lower height texture resolution.

## Atmospheric Effects

| ID       | Feature                            | Requirements |
|----------|------------------------------------|--------------|
| F-11.4.4 | Fog Volumes                        | R-11.4.4     |
| F-11.4.5 | Lightning                          | R-11.4.5     |
| F-11.4.6 | Wind Visualization and Dust Storms | R-11.4.6     |

1. **F-11.4.4** — Localized volumetric fog regions defined by oriented boxes or convex hulls with
   per-volume density, color, height falloff, and animation parameters. Fog volumes inject density
   into the global volumetric fog froxel grid and participate in temporal reprojection. Used for
   swamp haze, dungeon mist, and spell-area fog across open-world zones without affecting the global
   atmosphere.
   - **Deps:** F-2.7.2 (Ray-Marched Volumetric Fog)
   - **Platform:** Mobile uses screen-space height fog (no volumetric froxel grid). Switch uses a
     lower-resolution froxel grid. Concurrent fog volumes limited on mobile.
2. **F-11.4.5** — Procedural branching lightning bolt rendering using L-system subdivision with
   randomized branching angle and segment length. Each bolt emits a burst of light into the scene's
   lighting system, illuminating terrain and clouds for a single frame followed by exponential
   decay. Thunder audio cues are distance-delayed. Multiple simultaneous bolts are supported for
   intense storm sequences.
   - **Deps:** F-11.1.1 (GPU Particle Simulation)
   - **Platform:** Mobile limits branching depth to 2 (vs. 4 on desktop) and caps simultaneous bolts
     to 1. Light emission uses a single directional flash on mobile.
3. **F-11.4.6** — GPU particle-driven wind visualization rendering leaves, dust, and debris carried
   by the global wind field. Particle velocity is sampled from the shared wind vector volume so
   vegetation sway and particle motion stay coherent. Dust storms reduce visibility by injecting
   scattering density into the atmospheric fog and tinting the sky toward the storm color, creating
   dramatic weather transitions across open-world zones.
   - **Deps:** F-11.1.1
   - **Platform:** Mobile uses reduced wind particle count (10% of desktop). Dust storm visibility
     reduction uses distance fog only (no volumetric scattering) on mobile.

## Underwater Effects

| ID       | Feature                           | Requirements |
|----------|-----------------------------------|--------------|
| F-11.4.7 | Underwater Caustics and Depth Fog | R-11.4.7     |

1. **F-11.4.7** — Underwater rendering pass applying animated caustic light patterns projected onto
   submerged geometry, exponential depth fog with wavelength-dependent absorption (reds fade before
   blues), and particle-driven bubble streams. A refraction distortion layer on the water surface
   boundary simulates the view through a rippling interface. Light shafts are rendered as
   screen-space god rays from the surface.
   - **Deps:** F-11.3.3 (Full-Screen Distortion)
   - **Platform:** Mobile skips caustic projection and god rays; uses depth fog and simplified blue
     tint only. Bubble particles reduced to 25% of desktop count.
