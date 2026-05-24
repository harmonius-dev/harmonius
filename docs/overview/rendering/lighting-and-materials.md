# Lighting and Materials

How surfaces respond to light and how thousands of lights evaluate efficiently at fragment time.

## What it covers

- Tiled and clustered light culling via compute passes bounding per-tile lights.
- Deferred and forward lighting paths with G-buffer support (albedo, normal, motion, depth).
- Cook-Torrance PBR with GGX normal distribution, Smith visibility, and Schlick Fresnel.
- Extended BRDF layers for skin, clearcoat, anisotropy, sheen, degrading by quality tier.
- Per-object transparent sorting and per-instance material parameter buffers.
- Multi-layer material compositing with blend masks and configurable layer counts.
- Projected and mesh-based decals with deferred decal buffer writes.
- Specialized shading model variants for hair, cloth, eye, foliage, thin translucent, skin, and
  water.
- Importance-sampled stochastic area lighting with temporal accumulation denoising.
- Cascaded shadow maps with temporal stabilization and depth bias.
- Percentage-closer filtering for soft shadow edges.
- Contact shadows via ray marching.
- Volumetric fog and atmospheric scattering in participating media.

## Concepts

### Light Culling and Rendering Cost

Tiled or clustered light culling runs as a compute pass per view, evaluating only tile-local lights
during fragment shading. This bounds per-fragment cost and enables rendering hundreds of dynamic
lights without quadratic frame-time scaling. Deferred rendering via a G-buffer decouples geometry
complexity from lighting cost, supporting hundreds of lights on mobile; forward shading works best
with few lights and is used selectively.

### Physical Shading Foundation

Cook-Torrance PBR forms the base shading model, using GGX normal distribution, Smith joint
visibility, and Schlick Fresnel approximation. All textures bind via bindless indices for
consistency. Material instances share compiled shaders and differ only in per-instance parameter
buffers, enabling thousands of variants at minimal cost.

### Extended Materials and Specialization

Extended BRDF lobes layer on top of PBR base: subsurface scattering for skin and fabric, clearcoat
for automotive paint and lacquer, anisotropy for brushed metals and hair, and sheen for cloth.
Each quality tier caps the lobe count: mobile may support only PBR; desktop enables all. Dedicated
shading models exist for hair (Marschner), cloth (fiber scattering), foliage (translucency), and
water (reflection/refraction compositing) to optimize code paths for distinct categories.

### Shadows and Contact

Cascaded shadow maps with temporal stabilization cover directional lights (sun), providing crisp
near-field detail across the full scene range with configurable cascades and no seam artifacts.
Percentage-closer filtering softens shadow edges. Contact shadows via ray marching catch local
contacts missed by shadow maps. Temporal techniques reuse samples across frames to reduce per-frame
cost.

### Advanced Area Lighting

Importance-sampled stochastic lighting replaces per-light shadow map evaluation at scale, enabling
thousands of shadowed area lights. Samples accumulate over frames with temporal denoising, producing
soft shadows that converge within 8 frames while staying within frame budget.

## How it fits

- See [pipeline.md](./pipeline.md) for draw list compilation and bindless indexing.
- See [effects-and-styles.md](./effects-and-styles.md) for post-processing effects and bloom.
- See [environment-and-characters.md](./environment-and-characters.md) for reflection probes and
  baked lighting.
- Integrates with [../data-systems/attributes-and-effects.md](../data-systems/attributes-and-effects.md)
  for entity properties.
