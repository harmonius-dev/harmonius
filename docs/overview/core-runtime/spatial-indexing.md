# Spatial Indexing

Accelerating location-based queries across all systems.

## What it covers

- Single shared bounding-volume-hierarchy (BVH) used by all subsystems for location lookups.
- Incremental change-detection-driven BVH updates following entity transforms.
- Unified query API: raycasts, shape casts, overlaps, k-nearest-neighbor, frustum culling.
- Optional coarse grid or octree for cell-based queries (interest areas, density).
- Filtering by spatial layers (built-in and user-defined masks).
- Batch parallel queries with near-linear scaling.
- 2D and 3D BVH variants sharing the same interface.

## Concepts

### Shared Index

A single BVH stores axis-aligned bounding boxes for all spatial entities. Physics broadphase,
rendering culling, network area-of-interest, AI perception, and gameplay all query this index. The
index updates incrementally as entities move; change detection skips static entities to avoid wasted
updates.

### Query Types

Raycasts and frustum culling answer "what is in this region?". k-NN queries find the nearest K
objects. Overlaps test pairs for intersection. All queries are optionally ECS-filtered — constrain
results to entities with specific components.

### Fattened AABBs and Background Rebuilds

Small movements do not immediately rebuild the BVH; fattened bounding boxes absorb them. At frame
boundaries, the engine builds an improved BVH in the background and swaps it in, balancing query
latency with tree quality.

## How it fits

- See [world-and-entities](./world-and-entities.md) for how entities obtain spatial bounds.
- See [simulation-loop](./simulation-loop.md) for when updates are safe.
- Integrates with [physics](../physics/dynamics.md) for broadphase collision detection.
- Integrates with [ai](../ai/perception.md) for agent awareness.
