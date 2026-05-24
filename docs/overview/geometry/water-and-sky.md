# Water and Sky

Procedural water surfaces, oceans, sky rendering, and atmospheric effects.

## What it covers

- Gerstner waves: analytical wave simulation without solvers.
- FFT-based water: frequency-domain wave generation.
- Wave animation: per-vertex displacement.
- Ocean LOD: mesh density scaling with distance.
- Water reflections and refractions: environment mapping compositing.
- Underwater effects: blue tint and fog distance.
- Caustics: animated light projection simulating underwater light.
- Sky rendering: dome or procedural skyboxes.
- Cloud layers: procedural or texture-based clouds.
- Atmospheric scattering: Rayleigh and Mie effects.
- Sun and moon positioning: time-of-day sky variation.

## Concepts

### Water Simulation

Gerstner waves model ocean surface analytically: ocean height = sum of sinusoidal waves at different
frequencies and directions. Each wave component applies XZ displacement and height offset, producing
natural-looking ocean surface. Multiple waves superimpose, creating complex surface detail. Vertex
shader applies wave displacement in real-time; no solver needed.

### Ocean LOD and Rendering

Ocean meshes use LOD: dense geometry near the player, coarser geometry at distance. Multiple LOD
rings render concurrently. At very far distance, ocean renders as a horizon plane. Distant ocean
uses simplified vertex displacement. Water renders via reflection and refraction: reflections bounce
light from sky and terrain; refractions show underwater detail. Fresnel blending transitions from
reflections (shallow angles) to refraction (grazing angles).

### Sky and Atmosphere

Sky domes render as geometry or procedurally via ray tracing. Procedural sky computes color based on
sun position, altitude, and atmospheric parameters. Rayleigh scattering (short wavelengths scatter
more) produces blue sky and red sunsets. Mie scattering (particles) adds haze. Clouds layer on top:
procedural clouds via 3D noise, or texture-based clouds with parallax scrolling.

### Underwater Effects

Underwater shifts the view color blue and applies distance fog. Caustics (animated light patterns)
project onto surfaces, simulating light refraction through water. Caustic animation applies U/V
scrolling to a caustic texture. Particles rise as bubbles. Audio muffles (low-pass filter) and
changes reverberation properties.

## How it fits

- See [terrain-and-foliage.md](./terrain-and-foliage.md) for terrain and ground-level detail.
- See [meshes-and-detail.md](./meshes-and-detail.md) for LOD mesh systems.
- See [../rendering/lighting-and-materials.md](../rendering/lighting-and-materials.md) for
  reflection and refraction rendering.
