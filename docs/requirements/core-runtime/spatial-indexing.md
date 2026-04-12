# R-1.9 — Spatial Indexing Requirements

## Acceleration Structures

1. **R-1.9.1** — The engine **SHALL** maintain a single BVH as an ECS resource, updated once per
   frame by a dedicated system, shared across physics broadphase, rendering frustum culling, network
   interest management, AI perception, and gameplay spatial queries.
   - **Rationale:** A shared spatial index eliminates redundant structures and ensures all
     subsystems agree on entity positions within the same frame.
   - **Verification:** Verify BVH accessible from physics, rendering, and gameplay within same
     frame. Move one entity; verify updated position visible to all consumers after BVH update.
2. **R-1.9.2** — BVH memory overhead **SHALL NOT** exceed 64 bytes per indexed entity. The BVH
   **SHALL** maintain SAH quality within 2x of a full rebuild after incremental updates. The engine
   **SHALL** trigger a background full rebuild when quality degrades beyond a configurable
   threshold.
   - **Rationale:** Incremental updates degrade quality over time; automatic rebuilds prevent query
     performance decay.
   - **Verification:** Insert 1M entities; incrementally update 10% per frame for 1,000 frames;
     verify SAH within 2x of fresh build. Verify under 64 bytes per entity.
3. **R-1.9.3** — The engine **SHALL** update the BVH incrementally using ECS change detection on
   Transform components, refitting moved entities in-place and batch-inserting newly spawned
   entities, with cost proportional to moved entity count.
   - **Rationale:** Full rebuilds are infeasible at scale; incremental updates keep cost
     proportional to dynamic entities.
   - **Verification:** 1M entities, move 1%; verify update under 1 ms. Move 10%; verify time scales
     linearly (~10x, not constant or exploding).
4. **R-1.9.4** — The engine **SHALL** provide an optional coarse-grained spatial index (uniform grid
   or octree) as an ECS resource alongside the BVH, with cell membership updated by the same spatial
   index system using transform change detection.
   - **Rationale:** Cell-based indices are more efficient for network area-of-interest and AI crowd
     density queries.
   - **Verification:** Insert 100,000 entities; query a cell; verify all entities within bounds
     returned. Move across boundaries; verify membership updates next frame.

## Query Interface

1. **R-1.9.5** — The engine **SHALL** provide a single spatial query API supporting ray casts, shape
   casts, overlap tests, nearest-neighbor, and frustum queries, returning ECS Entity handles with
   hit metadata (position, normal, distance) and accepting ECS filters (With, Without). Queries
   against an empty index **SHALL** return an empty result set.
   - **Rationale:** A unified API avoids subsystem-specific implementations and enables combined
     spatial + archetype filtering.
   - **Verification:** Populate 10,000 entities; perform each query type; verify correct results
     against brute-force reference. Apply Without<Static> filter; verify excluded. Query empty
     index; verify empty set.
2. **R-1.9.6** — Single ray casts **SHALL** complete in under 10 us against 1M entities. Frustum
   queries **SHALL** complete in under 500 us for 1M entities.
   - **Rationale:** Spatial queries run thousands of times per frame; predictable latency is
     essential.
   - **Verification:** Benchmark 10,000 ray casts against 1M entities; verify average under 10 us.
     Frustum query under 500 us.
3. **R-1.9.7** — The engine **SHALL** support batch spatial queries executing in parallel across
   worker threads, amortizing traversal overhead and exploiting SIMD for ray-BVH intersection.
   - **Rationale:** Parallel batch execution is needed for server ticks with hundreds of concurrent
     queries.
   - **Verification:** Submit 1,000 ray casts on 1, 2, 4, 8 cores; verify near-linear speedup (3x on
     4 cores). Verify batch results match sequential exactly.

## Consumer Integration

1. **R-1.9.8** — The engine **SHALL** use the shared BVH for physics broadphase, querying
   overlapping AABB pairs filtered by CollisionLayers components, without a separate broadphase
   structure.
   - **Rationale:** Eliminates redundant broadphase and ensures physics/rendering agree on
     positions.
   - **Verification:** Create 1,000 entities in known overlapping configs; verify physics detects
     all pairs via shared BVH. Verify no separate structure allocated.
2. **R-1.9.9** — The engine **SHALL** use the shared BVH for rendering frustum culling across all
   views (main camera, shadow cascades, reflection probes), outputting visibility as marker
   components or per-view bitsets.
   - **Rationale:** Sharing BVH with physics avoids rebuilding a separate culling hierarchy.
   - **Verification:** Place 10,000 entities; camera covers half; verify ~5,000 marked visible.
     Verify shadow cascade produces different visibility set.
3. **R-1.9.10** — The engine **SHALL** use the shared spatial index (grid or octree) to compute
   area-of-interest sets per player, replicating only entities within the relevancy radius.
   - **Rationale:** Area-of-interest filtering is essential for network bandwidth in large
     multiplayer worlds.
   - **Verification:** 10,000 entities; 2 players at different positions with 100-unit radius;
     verify each interest set contains only entities within radius. Move player; verify update next
     frame.
4. **R-1.9.11** — The engine **SHALL** route AI perception (sight cones, hearing radii) and gameplay
   spatial queries (AoE, trigger volumes) through the shared index via the unified query API.
   - **Rationale:** Using the shared index avoids AI and gameplay maintaining independent lookups.
   - **Verification:** 100 AI agents with sight cones and 500 targets; verify sight cone returns
     only entities within angular and distance bounds. Verify AoE sphere returns all entities within
     radius, none outside.

## 2D Spatial Index

1. **R-1.9.12** — The engine **SHALL** provide a 2D BVH variant using Aabb2D leaf nodes for 2D/2.5D
   games, implementing the same SpatialQuery trait as the 3D BVH for generic consumer code.
   - **Rationale:** 2D games benefit from a BVH that operates on 2D primitives, avoiding the
     overhead of a third axis in all intersection tests.
   - **Verification:** Insert 100,000 2D entities; perform ray cast, overlap, and nearest-neighbor
     queries; verify results match brute-force reference. Verify SpatialQuery trait is identical to
     3D BVH.
2. **R-1.9.13** — The engine **SHALL** provide a 2D axis-aligned bounding box (Aabb2D) with area,
   merge, point containment, ray intersection, and circle intersection operations.
   - **Rationale:** 2D AABB is the leaf primitive for the 2D BVH and 2D overlap queries; it must
     support the same operation set as 3D AABB.
   - **Verification:** Construct Aabb2D from min/max; verify area calculation. Merge two AABBs;
     verify enclosing bounds. Test point containment, ray hit, and circle overlap against known
     geometry.

## BVH Configuration

1. **R-1.9.14** — The engine **SHALL** expose BVH construction parameters (SAH bin count, traversal
   cost ratio, fat AABB margin, rebuild quality threshold) as configurable values per platform tier.
   - **Rationale:** Different platforms have different memory and CPU budgets; per-tier BVH
     parameters enable quality/cost tradeoffs.
   - **Verification:** Set SAH bins to 8 vs 32; verify build produces different tree shapes. Set fat
     margin to 0 vs 0.1; verify refit frequency differs. Verify each platform tier has distinct
     default values.

## Fattened AABBs

1. **R-1.9.15** — The engine **SHALL** maintain fattened AABBs per BVH leaf so that entities moving
   within the fat margin require no tree modification, reducing refit cost for jittering entities.
   - **Rationale:** Fattened AABBs absorb small movements (jitter, idle animations) without
     triggering costly tree updates, significantly reducing refit work for mostly-stationary scenes.
   - **Verification:** Move an entity within the fat margin 100 times; verify zero BVH node
     modifications. Move beyond margin; verify refit triggers. Benchmark jittering 10,000 entities;
     verify update cost under 0.5 ms.

## Double-Buffered BVH Rebuild

1. **R-1.9.16** — The engine **SHALL** perform BVH full rebuilds on a background worker thread into
   a shadow buffer, swapping the result into the active BVH at frame boundary without blocking
   queries.
   - **Rationale:** Full rebuilds restore optimal SAH quality but are too expensive for the main
     frame budget; background rebuilding hides the cost.
   - **Verification:** Trigger a full rebuild with 1M entities; verify queries continue against the
     active BVH during rebuild. Verify swap occurs at frame boundary. Verify SAH quality improves
     after swap.

## Any-Hit Ray Test

1. **R-1.9.17** — The engine **SHALL** provide an any-hit ray test returning bool that stops at the
   first intersection, more efficient than nearest-hit ray cast for line-of-sight checks.
   - **Rationale:** Line-of-sight and occlusion checks only need to know if anything is hit, not
     which entity is nearest; early termination reduces traversal cost.
   - **Verification:** Cast ray through 1,000 entities; verify any-hit returns true and terminates
     before visiting all nodes. Benchmark any-hit vs nearest-hit; verify at least 2x faster for
     dense scenes.

## Spatial Layer Masks

1. **R-1.9.18** — The engine **SHALL** provide a 32-bit spatial layer mask with 8 built-in layers
   (physics, rendering, navigation, network, AI, audio, gameplay, trigger) and 24 user-definable
   layers, filtering all spatial queries by layer intersection.
   - **Rationale:** Layer masks enable subsystems to query only relevant entities without
     post-filtering, reducing query cost for multi-purpose spatial indices.
   - **Verification:** Assign entities to physics and rendering layers; query with physics-only
     mask; verify rendering-only entities excluded. Define a custom layer; assign and query; verify
     correct filtering.

## Multiple Bounding Volume Types

1. **R-1.9.19** — The engine **SHALL** support multiple bounding volume types (AABB, sphere, OBB) as
   ECS components, converting to AABB for BVH storage.
   - **Rationale:** Different entity shapes are best approximated by different bounding volumes;
     conversion to AABB for BVH storage maintains a uniform tree structure.
   - **Verification:** Attach sphere bounding volume to an entity; verify BVH stores its enclosing
     AABB. Attach OBB; verify enclosing AABB. Query via BVH; verify correct broad-phase results for
     each volume type.

## Multi-View Batch Frustum Culling

1. **R-1.9.20** — The engine **SHALL** support batch frustum culling for multiple views (main
   camera, shadow cascades, reflection probes) in a single parallel dispatch.
   - **Rationale:** Culling each view separately repeats tree traversal; batch dispatch amortizes
     traversal cost across all views.
   - **Verification:** Submit 6 frustum queries (1 main + 4 shadow cascades + 1 reflection probe) as
     a batch; verify all results correct. Benchmark batch vs sequential; verify at least 2x faster
     for 1M entities.
