# 11.4 — Weather & Environmental FX

## Precipitation

### F-11.4.1 Rain Particle System and Screen Droplets

Multi-layered rain rendering combining GPU particle streaks for world-space rainfall, screen-space
droplet simulation for camera-near effects, and ripple normal maps on wet surfaces. Particle
density scales with weather intensity and camera exposure to the sky. Screen droplets flow
downward with gravity, merge on contact, and evaporate after the storm passes.

- **Requirements:** R-11.4.1
- **Dependencies:** F-11.1.1
- **Platform notes:** None

### F-11.4.2 Rain Puddles and Wet Surfaces

Dynamic puddle formation driven by a heightfield accumulation buffer and terrain concavity.
Puddles modify surface roughness toward mirror-smooth, darken albedo, and add animated ripple
normals during active rainfall. Wet surface response is material-driven: stone darkens, metal
reflects, and dirt turns to mud. Puddle depth drains over time after rain stops.

- **Requirements:** R-11.4.2
- **Dependencies:** F-11.4.1
- **Platform notes:** None

### F-11.4.3 Snow Accumulation and Interaction

Vertex-displacement snow layer that accumulates on upward-facing surfaces over time. Accumulation
depth is stored in a world-space height texture updated by weather state. Character and vehicle
movement carve deformation trails into the snow layer using depth stamps. Footprints reveal the
underlying surface material and fade gradually under continued snowfall.

- **Requirements:** R-11.4.3
- **Dependencies:** None
- **Platform notes:** None

## Atmospheric Effects

### F-11.4.4 Fog Volumes

Localized volumetric fog regions defined by oriented boxes or convex hulls with per-volume density,
color, height falloff, and animation parameters. Fog volumes inject density into the global
volumetric fog froxel grid and participate in temporal reprojection. Used for swamp haze, dungeon
mist, and spell-area fog across open-world zones without affecting the global atmosphere.

- **Requirements:** R-11.4.4
- **Dependencies:** None
- **Platform notes:** None

### F-11.4.5 Lightning

Procedural branching lightning bolt rendering using L-system subdivision with randomized branching
angle and segment length. Each bolt emits a burst of light into the scene's lighting system,
illuminating terrain and clouds for a single frame followed by exponential decay. Thunder audio
cues are distance-delayed. Multiple simultaneous bolts are supported for intense storm sequences.

- **Requirements:** R-11.4.5
- **Dependencies:** None
- **Platform notes:** None

### F-11.4.6 Wind Visualization and Dust Storms

GPU particle-driven wind visualization rendering leaves, dust, and debris carried by the global
wind field. Particle velocity is sampled from the shared wind vector volume so vegetation sway and
particle motion stay coherent. Dust storms reduce visibility by injecting scattering density into
the atmospheric fog and tinting the sky toward the storm color, creating dramatic weather
transitions across open-world zones.

- **Requirements:** R-11.4.6
- **Dependencies:** F-11.1.1
- **Platform notes:** None

## Underwater Effects

### F-11.4.7 Underwater Caustics and Depth Fog

Underwater rendering pass applying animated caustic light patterns projected onto submerged
geometry, exponential depth fog with wavelength-dependent absorption (reds fade before blues),
and particle-driven bubble streams. A refraction distortion layer on the water surface boundary
simulates the view through a rippling interface. Light shafts are rendered as screen-space
god rays from the surface.

- **Requirements:** R-11.4.7
- **Dependencies:** None
- **Platform notes:** None
