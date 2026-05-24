# Procedural Generation

Runtime geometry generation, noise functions, and constraint-based placement.

## What it covers

- Noise functions: Perlin, Simplex, Worley for procedural textures and placement.
- Terrain generation: sculpting height fields from noise.
- Dungeon generation: procedural layout via constraint satisfaction.
- Building generation: floors, walls, windows, doors from templates.
- Road networks: automatic road path generation and mesh.
- Asset placement: distributing trees, rocks, vegetation via noise sampling.
- Mesh generation: runtime creation of geometry (fractals, L-systems).
- Prefab variation: procedural variants of base geometry.
- Seed-based reproduction: consistent generation from seed values.
- Streaming integration: generating and unloading chunks as needed.

## Concepts

### Procedural Noise and Sampling

Noise functions (Perlin, Simplex) produce pseudo-random values smoothly varying over space. Sampling
noise at positions determines terrain height, vegetation density, or material type. Noise layers
(octaves) combine multiple frequencies for natural variation: single-octave noise is too regular;
multiple octaves produce natural fractal-like features. Seed-based generation reproduces identical
results from the same seed, enabling deterministic procedural worlds.

### Terrain and Dungeon Generation

Terrain generation samples noise at each grid point, producing elevation maps. Water pooling in
valleys creates rivers. Dungeon generation uses constraint satisfaction: place room A, then room B
with connectivity constraints (doors, corridors). Constraint solvers find valid placements. Building
generation composes modular pieces: base layout defines floor plan, procedural variation adds unique
details.

### Asset and Road Networks

Asset placement samples noise: high noise values place trees, rocks, vegetation. Noise clustering
creates forests and clearings. Road networks generate via path-finding or L-systems: recursive
branching rules produce realistic road layouts with main highways and branching streets. Road meshes
generate by tessellating paths into geometry.

### Mesh Generation at Runtime

L-systems encode procedural rules: "draw a branch, then recursively draw two smaller branches".
These produce tree-like structures. Fractals generate self-similar geometry. Marching cubes converts
signed distance fields to meshes. All mesh generation uses memory efficiently: streaming generates
chunks on-demand.

## How it fits

- See [meshes-and-detail.md](./meshes-and-detail.md) for mesh hierarchies and LOD systems.
- See [terrain-and-foliage.md](./terrain-and-foliage.md) for terrain integration.
- See [../data-systems/composition.md](../data-systems/composition.md) for templating and
  prefab variation.
