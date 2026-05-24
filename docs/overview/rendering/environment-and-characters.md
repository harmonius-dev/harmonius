# Environment and Characters

Specialized rendering techniques for large outdoor environments, indoor lighting, characters,
and skin.

## What it covers

- Environment mapping: reflection probes, planar reflections, and screen-space reflections.
- Reflection probe hierarchies: global, local, and distance-based selection.
- Irradiance and radiance baking for static geometry and vegetation.
- Global illumination at runtime on capable platforms.
- Baked shadows via lightmaps for static geometry.
- Skin shading with subsurface scattering, specular maps, and preintegrated lookup tables.
- Hair rendering with Marschner model, alpha maps, and shell-based strand simulation.
- Eye shading with refraction, specular highlights, and pupil animation.
- Character-specific material layers: cloth, armor, skin, hair rendering.
- Foliage rendering: translucency, wind animation, and level-of-detail geometry variants.
- Water rendering: reflection and refraction compositing with Fresnel weighting.
- Ocean surfaces: large-scale wave simulation, FFT-based height fields, and LOD geometry.
- Atmospheric scattering: Rayleigh and Mie effects for sky.

## Concepts

### Reflections and Probes

Reflection probes capture environment radiance in cubemaps at discrete locations. Probes blend
based on distance and view position, with parallax correction adjusting reflection direction for
non-local probes. Global probes capture the distant environment; local probes enhance nearby
surfaces. Planar reflections capture flat surfaces like mirrors or water in real-time. Screen-space
reflections (SSR) trace rays in screen space for further detail but cost more per frame; SSR and
probes combine for efficiency.

### Lighting Baking

Indirect lighting from static geometry bakes into texture atlases at authoring time, stored per
lightmap texture coordinate. Baked shadows store static shadow information on surfaces at texel
resolution, eliminating per-frame shadow evaluation. Irradiance and radiance bake with progressive
path tracing or offline renderers, with GPU bake acceleration on capable platforms. Characters and
moving geometry use probes or dynamic lighting; static geometry reads baked lightmaps.

### Skin and Hair

Skin rendering applies subsurface scattering via preintegrated lookup tables, accessed by material
thickness and light angle, producing realistic translucency. Specular maps control per-pixel
specularity variations. Hair uses the Marschner model capturing highlight shifts with hair
direction. Alpha-masked strands render in layers; longer hair adds shell layers for volume. Eye
shading refracts view-direction around a limbal ring, simulates pupil dilation, and applies specular
highlights from light sources.

### Character Materials

Character surfaces layer distinct materials: skin with subsurface scattering, cloth with anisotropic
sheen, armor with metallic properties, and hair with strand-based shading. Each layer can animate
independently (e.g., cloth flutter). Material instances allow variations per character while
sharing compiled shaders.

### Foliage and Large Scale

Foliage renders with alpha transparency and wind-driven vertex animation in the vertex stage. Leaves
and branches layer in 2D cross-sections (shells) for volumetric appearance without full 3D geometry.
Distant foliage uses billboarded imposters or reduced geometry. Ocean surfaces use Gerstner waves
or FFT-based height fields, animating vertex positions in compute passes. Distant ocean uses
simplified geometry. Atmospheric scattering adds blue and haze to the sky, simulating Rayleigh
scattering (short wavelengths) and Mie scattering (particles).

## How it fits

- See [lighting-and-materials.md](./lighting-and-materials.md) for extended BRDF lobes and
  specialized shading models.
- See [../simulation/spatial-awareness.md](../simulation/spatial-awareness.md) for wind zones
  and environmental parameters.
- See [../data-systems/attributes-and-effects.md](../data-systems/attributes-and-effects.md)
  for per-entity animation state and material variants.
