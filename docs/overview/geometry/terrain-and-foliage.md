# Terrain and Foliage

Large-scale terrain rendering, vegetation systems, and environmental detail.

## What it covers

- Height fields: elevation-based terrain representation.
- Chunked terrain: dividing terrain into manageable chunks for streaming and LOD.
- Terrain materials: layered textures (grass, rock, snow) blended by slope and height.
- Triplanar mapping: preventing texture stretching on vertical cliffs.
- Grass and vegetation systems: instanced plant meshes.
- Foliage clustering: reducing draw calls by batching similar vegetation.
- Vegetation LOD: detailed leaves at close range, billboards at distance.
- Wind zones: procedural animation of grass and trees.
- Destructible terrain: deformation and decals from explosions.
- Terrain baking: pre-baking shadows and ambient occlusion.

## Concepts

### Terrain Representation

Height fields store elevation per grid point. Terrain meshes interpolate between grid points,
producing smooth surfaces. Terrain chunks divide the world into manageable pieces; only nearby
chunks render. Streaming loads and unloads chunks as the character moves. Height field resolution
varies: high detail near the player, low detail at distance (LOD).

### Terrain Materials and Blending

Terrain surfaces use multiple materials (grass, rock, dirt) blended by slope and altitude. Grass
appears on gentle slopes; rock on steep cliffs. Snow appears at high altitudes. Triplanar mapping
applies textures from three directions (X, Y, Z), preventing stretching on vertical walls. Blend
masks indicate material transitions: sharp transitions for cliffs, soft gradients for rolling hills.

### Vegetation and Foliage

Grass and trees render via instanced meshes: a single mesh definition with thousands of transform
instances placed across terrain. Foliage clustering combines nearby plants into single draw calls,
reducing overhead. LOD systems render full detail at close range and billboards or lower-poly meshes
at distance. Wind zones apply procedural vertex animation: grass and tree branches sway based on a
wind parameter varying over time and space.

### Destructible Terrain

Explosions deform terrain height fields, creating craters. Blast force computes deformation radius
and depth. Real-time height field updates enable dynamic craters. Decals mark blast impact sites.
Baked lightmaps update in regions with deformation, recomputing shadows and ambient occlusion.

## How it fits

- See [water-and-sky.md](./water-and-sky.md) for procedural water and atmospheric effects.
- See [meshes-and-detail.md](./meshes-and-detail.md) for mesh hierarchies and LOD systems.
- See [../rendering/environment-and-characters.md](../rendering/environment-and-characters.md)
  for foliage rendering and wind-driven animation.
