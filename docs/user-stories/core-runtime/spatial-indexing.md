# Spatial Indexing User Stories

## Acceleration Structures

| ID       | Persona                 | Features                  | Requirements              |
|----------|-------------------------|---------------------------|---------------------------|
| US-1.9.1 | engine developer (P-26) | F-1.9.1                   | R-1.9.1                   |
| US-1.9.2 | designer (P-5)          | F-1.9.1, F-1.9.3, F-1.9.4 | R-1.9.1, R-1.9.3, R-1.9.4 |
| US-1.9.3 | engine tester (P-27)    | F-1.9.1                   | R-1.9.1                   |
| US-1.9.4 | engine developer (P-26) | F-1.9.2                   | R-1.9.2                   |
| US-1.9.5 | engine tester (P-27)    | F-1.9.2                   | R-1.9.2                   |
| US-1.9.6 | game developer (P-15)   | F-1.9.3                   | R-1.9.3                   |

1. **US-1.9.1** — a single BVH maintained as an ECS resource shared across physics, rendering,
   networking, AI, and gameplay, so that all subsystems use one spatial structure instead of
   maintaining redundant copies
   - **Acceptance:** Single BVH updated by dedicated system<br>All subsystems read from shared
     BVH<br>Platform-appropriate entity limits and memory budgets
2. **US-1.9.2** — to visualize the shared spatial index in the editor via debug overlays showing BVH
   bounding boxes and grid cells, so that I can understand how entities are spatially organized and
   debug query behavior
   - **Acceptance:** BVH bounding boxes visualized as debug overlay<br>Grid/octree cells visualized
     when enabled<br>Query results highlighted in viewport
3. **US-1.9.3** — to benchmark BVH memory usage and query latency at platform entity limits (50K
   mobile to 5M+ high-end PC), so that I can verify the spatial index meets its memory and
   performance budgets
   - **Acceptance:** Mobile: 50K entities in ~2 MB<br>Desktop: 1M+ entities in ~40
     MB<br>Sub-millisecond query times at platform limits
4. **US-1.9.4** — the BVH updated incrementally using ECS change detection on Transform components,
   so that update cost is proportional to moved entities rather than total entity count
   - **Acceptance:** Moved entities refitted in-place or reinserted<br>Newly spawned entities
     batch-inserted<br>Update cost proportional to moved entities
5. **US-1.9.5** — to benchmark incremental BVH update cost with varying ratios of stationary to
   moving entities, so that I can verify update time scales with moved entity count, not total count
   - **Acceptance:** 1M entities, 1% moving: update under 0.5ms<br>1M entities, 100% moving: update
     within refit budget<br>No full rebuild unless entity distribution changes radically
6. **US-1.9.6** — an optional coarse-grained spatial index (uniform grid or octree) alongside the
   BVH, so that network area-of-interest filtering and AI crowd density queries use the most
   efficient structure for their access pattern
   - **Acceptance:** Grid or octree stored as ECS resource alongside BVH<br>Cell membership updated
     by spatial index system<br>Platform-appropriate grid size and depth limits

## Query Interface

| ID       | Persona                 | Features | Requirements |
|----------|-------------------------|----------|--------------|
| US-1.9.7 | game developer (P-15)   | F-1.9.4  | R-1.9.4      |
| US-1.9.8 | engine developer (P-26) | F-1.9.5  | R-1.9.5      |
| US-1.9.9 | engine tester (P-27)    | F-1.9.5  | R-1.9.5      |

1. **US-1.9.7** — a single API for ray casts, shape casts, overlap tests, nearest-neighbor, and
   frustum queries that accepts ECS component filters, so that all subsystems use a consistent query
   interface
   - **Acceptance:** Ray cast, shape cast, overlap, k-NN, frustum queries<br>Results as ECS Entity
     handles with hit metadata<br>ECS filters (With<T>, Without<T>) accepted
2. **US-1.9.8** — to submit batches of spatial queries that execute in parallel across worker
   threads with SIMD acceleration, so that server ticks with hundreds of AI and ability checks
   complete within frame budgets
   - **Acceptance:** Batch queries execute in parallel via job system<br>SIMD-accelerated ray-BVH
     intersection<br>Platform-appropriate batch and worker limits
3. **US-1.9.9** — to benchmark batch spatial query throughput at platform limits (64 mobile to 4096+
   high-end PC), so that I can verify server tick budgets are met during worst-case AI and ability
   processing
   - **Acceptance:** Mobile: 64 queries per batch within budget<br>Desktop: 1024+ queries per batch
     within budget<br>SIMD acceleration measurably improves throughput

## Consumer Integration

| ID        | Persona                 | Features         | Requirements     |
|-----------|-------------------------|------------------|------------------|
| US-1.9.10 | engine developer (P-26) | F-1.9.6          | R-1.9.6          |
| US-1.9.11 | engine tester (P-27)    | F-1.9.6, F-1.9.7 | R-1.9.6, R-1.9.7 |
| US-1.9.12 | engine developer (P-26) | F-1.9.7          | R-1.9.7          |
| US-1.9.13 | game developer (P-15)   | F-1.9.8          | R-1.9.8          |
| US-1.9.14 | engine tester (P-27)    | F-1.9.8          | R-1.9.8          |
| US-1.9.15 | game developer (P-15)   | F-1.9.9          | R-1.9.9          |
| US-1.9.16 | engine tester (P-27)    | F-1.9.9          | R-1.9.9          |

1. **US-1.9.10** — the physics broadphase to read from the shared BVH filtered by collision layers,
   so that collision detection uses the same spatial data as rendering and no separate broadphase
   structure is allocated
   - **Acceptance:** Physics queries shared BVH for overlapping AABB pairs<br>Results filtered by
     CollisionLayers components<br>No separate broadphase data structure
2. **US-1.9.11** — to verify that physics broadphase and rendering culling produce consistent
   results from the shared BVH within the same frame, so that no entity is visible but
   non-collidable or vice versa
   - **Acceptance:** Same entity positions for physics and rendering in same frame<br>No frame-lag
     between spatial index and subsystem queries
3. **US-1.9.12** — frustum culling to read from the shared BVH for all views (main camera, shadow
   cascades, reflection probes), so that the renderer shares incremental updates and does not
   rebuild a separate culling hierarchy
   - **Acceptance:** Main camera, shadow cascade, and reflection probe culling<br>Visibility results
     as marker components or bitsets<br>Platform-appropriate view count limits
4. **US-1.9.13** — the network relevancy system to use the shared spatial index for area-of-interest
   filtering, so that only entities within each player's relevancy radius are replicated, saving
   bandwidth in large multiplayer worlds
   - **Acceptance:** Range queries compute per-player area-of-interest sets<br>Grid or octree used
     for cell-based filtering<br>Consistent spatial reasoning with physics and rendering
5. **US-1.9.14** — to verify that network relevancy area-of-interest sets are spatially consistent
   with physics and rendering, so that replicated entities match what the player sees and collides
   with
   - **Acceptance:** Replicated entities match visible entities within radius<br>No entities
     replicated outside relevancy radius<br>Radius changes reflected within one tick
6. **US-1.9.15** — AI perception systems (sight cones, hearing radii) and gameplay spatial queries
   (AoE abilities, trigger volumes) to read from the shared index, so that AI and gameplay systems
   agree with physics on entity positions
   - **Acceptance:** Sight cone queries via frustum test on BVH<br>Hearing radius queries via sphere
     overlap<br>AoE and trigger volumes use unified query API
7. **US-1.9.16** — to verify that AI sight cone and hearing radius queries return exactly the
   entities within the perception volume, so that AI does not perceive entities it should not see or
   miss entities it should detect
   - **Acceptance:** Sight cone includes only entities within frustum<br>Hearing radius includes
     only entities within sphere<br>No false positives or negatives in perception results
