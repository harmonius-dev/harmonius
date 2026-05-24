# Geometry

Meshes, terrain, foliage, water, sky, and procedural world generation.

## Topics

- [terrain-and-foliage](./terrain-and-foliage.md) — heightmaps, sculpting, grass, and trees.
- [water-and-sky](./water-and-sky.md) — water surfaces, oceans, atmosphere, and stars.
- [meshes-and-detail](./meshes-and-detail.md) — mesh streaming, LOD, and detail levels.
- [procedural-generation](./procedural-generation.md) — noise, rules, and algorithmic world
  building.

## Key takeaways

- Height field terrain with chunked streaming enables large worlds; LOD reduces distant chunk
  polygon count, and triplanar mapping prevents texture stretching on cliffs.
- Gerstner waves analytically displace vertices, enabling real-time ocean waves without solver
  overhead; FFT-based waves allow larger, more complex oceans on capable platforms.
- Mesh LOD chains progressively reduce polygon count at distance; screen-space LOD selection ensures
  frame budget consistency.
- Instanced vegetation with clustering combines thousands of trees into single draw calls; wind
  animation vertex shader avoids per-tree transforms.
- Procedural generation via noise sampling, constraint satisfaction, and L-systems enables diverse
  environments from compact seed values, facilitating exploration and replay.

## Integration risks

- Terrain chunk resolution (height field grid density) directly impacts LOD transitions; mismatched
  resolutions cause visible popping at chunk boundaries. See [terrain-and-foliage.md](./terrain-and-foliage.md)
  for seamless resolution scaling.
- Wave amplitude and frequency must be coordinated across all ocean zones; mismatched wave parameters
  cause visible seams. See [water-and-sky.md](./water-and-sky.md) for wave parameter synchronization.
- LOD chain creation requires careful per-mesh tuning; incorrect LOD simplification causes
  silhouette artifacts or misaligned UV seams. See [meshes-and-detail.md](./meshes-and-detail.md)
  for LOD generation best practices.
- Procedural generation seed must be consistent across network clients; seed divergence causes
  different worlds on different clients. See [procedural-generation.md](./procedural-generation.md)
  for network seed synchronization.
- Foliage bending in wind must match vegetation LOD; distant billboards bending identically to
  near geometry prevents pop-in perception. See [terrain-and-foliage.md](./terrain-and-foliage.md)
  for LOD wind animation synchronization.
