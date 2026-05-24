# Grids and Volumes

Spatial partitioning: fog of war, tactical maps, flow fields, and height fields.

## What it covers

- Uniform grids: generic 2D/3D spatial containers backing fog of war, height fields, tactical
  maps, scent grids, and influence maps.
- Grid operations: propagation (floodfill, Dijkstra), line-of-sight, and influence decay.
- Voxel volumes: 3D-only palette-compressed chunks for efficient storage.
- Dirty-region optimization: only recompute changed cells; GPU uploads for on-demand
  computation.
- Hierarchical grids: variable LOD support for large worlds.
- Network relevancy: dedicated area-of-interest grid avoiding full-world spatial queries.
- Performance budgets: 256² grids propagate in <1 ms; 128-cell line-of-sight in <0.01 ms.

## Concepts

### Uniform Grids

A 2D or 3D grid stores typed values at each cell. Operations like floodfill, Dijkstra shortest path,
and decay propagate values across cells. Line-of-sight checks ray through cells, answering "what is
visible from this position?". Influence maps decay over distance; obstacles block propagation.

### Dirty Regions

Grid modifications update only affected cells. Dirty regions track changed cells; recomputation
stays on the fast path. When CPU budget is exceeded, the engine automatically offloads computation
to the GPU via compute shaders.

### Relevancy and Hierarchies

A dedicated area-of-interest grid answers "who cares about this entity?" without querying the global
spatial index per observer. Hierarchical grids subdivide on demand, enabling consistent queries at
multiple levels of detail for arbitrarily large worlds.

## How it fits

- See [spatial-awareness](./spatial-awareness.md) for sense queries using grid data.
- See [timelines](./timelines.md) for cutscene triggers based on grid state.
- Integrates with [physics](../physics/queries-and-vehicles.md) for collision voxel volumes.
- Integrates with [rendering](../rendering/pipeline.md) for GPU grid uploads and compute
  dispatch.
