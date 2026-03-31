# 1.9 — Spatial Indexing

## Acceleration Structures

| ID      | Feature                         |
|---------|----------------------------------|
| F-1.9.1 | Shared BVH Spatial Index        |
| F-1.9.2 | Incremental BVH Updates         |
| F-1.9.3 | Hierarchical Grid / Octree Index|

1. **F-1.9.1** — A single bounding volume hierarchy (BVH) maintained as an ECS resource, updated
   once per frame by a dedicated system that queries all entities with spatial components
   (Transform, Collider, BoundingVolume). All subsystems — physics broadphase, rendering frustum
   culling, network interest management, AI perception, and gameplay spatial queries — read from
   this shared structure rather than maintaining independent copies.
   - **Deps:** F-1.1.1 (Archetype Storage), F-1.1.17 (Composable Archetype Queries), F-1.2.1
     (Transforms)
   - **Platform:** Mobile: max 50K entities in BVH, ~2 MB memory. Switch: max 200K entities, ~8 MB.
     Desktop: 1M+ entities, ~40 MB. High-end PC: 5M+ entities with SIMD-optimized traversal. BVH
     node size tuned per platform cache line.
2. **F-1.9.2** — Update the BVH incrementally using ECS change detection (F-1.1.22) on Transform
   components rather than rebuilding from scratch each frame. Moved entities are refitted in-place;
   large movements trigger node reinsertion. Newly spawned entities are batch-inserted. This keeps
   update cost proportional to the number of moved entities, not total entity count — critical for
   MMO worlds where most entities are stationary.
   - **Deps:** F-1.9.1, F-1.1.22 (Tick-Based Change Detection)
3. **F-1.9.3** — An optional coarse-grained spatial index (uniform grid or octree) for queries that
   benefit from cell-based locality — network area-of-interest filtering, AI crowd density queries,
   and world-partition zone assignment. Stored as an ECS resource alongside the BVH. Entities
   register in cells based on their Transform; cell membership is updated by the same spatial index
   system.
   - **Deps:** F-1.9.1, F-1.1.22 (Tick-Based Change Detection)
   - **Platform:** Mobile: max grid 64x64, octree depth 4. Switch: grid 128x128, octree depth 5.
     Desktop: grid 256x256, octree depth 8. Cell size adjusts per platform to keep total cell count
     within memory budget.

## Query Interface

| ID      | Feature                           |
|---------|------------------------------------|
| F-1.9.4 | Unified Spatial Query API         |
| F-1.9.5 | Batch and Parallel Spatial Queries|

1. **F-1.9.4** — A single query API that all subsystems use to perform ray casts, shape casts,
   overlap tests, nearest-neighbor searches, and frustum queries against the shared spatial index.
   Queries return ECS Entity handles with hit metadata (position, normal, distance). The API accepts
   ECS query filters (With<T>, Without<T>) so callers can restrict results by component presence
   without post-filtering.
   - **Deps:** F-1.9.1, F-1.1.17 (Composable Archetype Queries)
2. **F-1.9.5** — Submit multiple spatial queries as a batch that executes in parallel across worker
   threads via the job system (F-14.3.3). Batch queries amortize tree traversal overhead and exploit
   SIMD for ray-BVH intersection. This is essential for server-side MMO ticks where hundreds of AI
   agents, ability checks, and network relevancy scans issue spatial queries simultaneously.
   - **Deps:** F-1.9.4, F-14.3.3 (Job System)
   - **Platform:** Mobile: max 64 queries per batch, 2 worker threads. Switch: max 128 queries, 3
     workers. Desktop: max 1024 queries, workers scale with core count. High-end PC: 4096+ queries
     with full SIMD ray-BVH acceleration.

## Consumer Integration

| ID      | Feature                                |
|---------|-----------------------------------------|
| F-1.9.6 | Physics Broadphase Integration         |
| F-1.9.7 | Rendering Culling Integration          |
| F-1.9.8 | Network Interest Management Integration|
| F-1.9.9 | AI Perception and Gameplay Integration |

1. **F-1.9.6** — The physics broadphase reads from the shared BVH rather than maintaining a separate
   sweep-and-prune or BVH. The collision detection system queries the shared index for overlapping
   AABB pairs, filtered by CollisionLayers components. This eliminates redundant spatial data and
   ensures physics and rendering agree on entity positions within the same frame.
   - **Deps:** F-1.9.1, F-4.2.1 (Broadphase), F-4.2.6 (Collision Layers)
2. **F-1.9.7** — The rendering frustum culling system reads from the shared BVH to determine visible
   entities per view (main camera, shadow cascades, reflection probes). Culling results are written
   as transient Visible marker components or into per-view visibility bitsets. Sharing the BVH with
   physics avoids rebuilding a separate culling hierarchy for the renderer.
   - **Deps:** F-1.9.1, F-2.10.4 (View Setup)
   - **Platform:** Mobile: 1 view + 1 shadow cascade culled per frame. Switch: 1 view - 2 shadow
     cascades. Desktop: main view + 4 shadow cascades + reflection probes. High-end PC: unlimited
     views with parallel culling passes.
3. **F-1.9.8** — The network relevancy system uses the shared spatial index (grid or octree,
   F-1.9.3) to compute area-of-interest sets for each connected player. Only entities within a
   player's relevancy radius are replicated, and the spatial index provides efficient range queries.
   Sharing the index with physics and rendering ensures consistent spatial reasoning across all
   subsystems.
   - **Deps:** F-1.9.3, F-8.2.3 (Area-of-Interest Filtering)
4. **F-1.9.9** — AI perception systems (sight cones, hearing radii) and gameplay spatial queries
   (area-of-effect abilities, trigger volumes) read from the shared spatial index. Perception
   queries use frustum and sphere overlap tests on the BVH; gameplay queries use the unified query
   API (F-1.9.4). This avoids AI and gameplay systems maintaining their own spatial lookups.
   - **Deps:** F-1.9.4, F-7.6.1 (Sight), F-7.6.2 (Hearing)
