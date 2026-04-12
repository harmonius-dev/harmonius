# 4.4 — Spatial Queries

## Ray and Shape Casts

| ID      | Feature |
|---------|----------------------------- |
| F-4.4.1 | Ray Casting |
| F-4.4.2 | Shape Casting (Sweep Tests) |

1. **F-4.4.1** — `RayCast` is a system parameter that accepts a ray origin, direction, maximum
   distance, and `QueryFilter`, then queries the shared spatial index (F-1.9.1) against ECS entities
   with `Collider` and `Transform` components. Results include the hit `Entity` ID, hit position,
   surface normal, distance, and `CollisionLayers` component value. Ray casts use the unified
   spatial query API (F-1.9.4) and are foundational for line-of-sight checks, weapon targeting, and
   AI perception in server-authoritative simulation.
   - **Deps:** F-1.9.4 (Unified Spatial Query API), F-4.2.6, F-1.1.17 (Composable Archetype Queries)
2. **F-4.4.2** — `ShapeCast` is a system parameter that sweeps a collision shape (sphere, capsule,
   box, convex hull) along a direction against ECS entities with `Collider` components. The sweep
   consults the broadphase ECS resource and narrowphase geometry to report the first or all contacts
   encountered. Results include the hit `Entity` ID, contact point, normal, penetration depth, and
   `CollisionLayers`. Shape casts power character controller ground detection, projectile hit
   detection, and safe movement queries.
   - **Deps:** F-4.2.1, F-4.2.2, F-4.2.6, F-1.1.17 (Composable Archetype Queries)

## Overlap and Proximity

| ID      | Feature |
|---------|----------------------- |
| F-4.4.3 | Overlap Tests |
| F-4.4.4 | Closest Point Queries |

1. **F-4.4.3** — `OverlapQuery` is a system parameter that tests whether a given shape placed at a
   position overlaps any ECS entities carrying `Collider` components. It traverses the broadphase
   ECS resource and returns all overlapping `Entity` IDs along with their `CollisionLayers` values.
   Supported query shapes are sphere, box, capsule, and convex hull. Overlap tests power
   area-of-effect abilities, trigger volumes, and proximity-based gameplay logic.
   - **Deps:** F-4.2.1, F-4.2.6, F-1.1.17 (Composable Archetype Queries)
2. **F-4.4.4** — `ClosestPointQuery` is a system parameter that computes the closest point on any
   `Collider` surface to a given world-space point by traversing the broadphase ECS resource and
   evaluating narrowphase geometry. Results include the closest `Entity` ID, closest point position,
   surface normal, and signed distance. Useful for distance-based triggers, attachment point
   calculation, and AI navigation queries requiring precise geometric proximity.
   - **Deps:** F-4.2.2, F-4.2.1, F-1.1.17 (Composable Archetype Queries)

## Batching and Performance

| ID      | Feature |
|---------|--------------------------------------- |
| F-4.4.5 | Batch Query Execution |
| F-4.4.6 | Query Filtering and Custom Predicates |

1. **F-4.4.5** — `BatchSpatialQuery` accepts a slice of query descriptors (ray casts, shape casts,
   overlaps) and submits them to the ECS job system for parallel execution across worker threads.
   Results are written to a caller-provided buffer or returned via a channel. Batch queries amortize
   broadphase traversal cost and exploit SIMD parallelism, which is critical for server-side
   simulation where hundreds of AI agents issue spatial queries every tick.
   - **Deps:** F-4.4.1, F-4.4.2, F-4.4.3, F-1.1.20 (Automatic Parallel Iteration)
   - **Platform:** Mobile: max 32 queries per batch, single-threaded fallback. Switch: max 64
     queries, 2 worker threads. Desktop: max 512 queries, workers scale with cores. High-end PC: max
     4096 queries with SIMD ray-BVH acceleration.
2. **F-4.4.6** — All spatial query parameters accept a `QueryFilter` that combines `CollisionLayers`
   bitmask filtering with ECS query filters (`With<T>`, `Without<T>`) to include or exclude entities
   by component presence. Beyond structural filters, queries accept an optional custom predicate
   closure that receives an `&EntityRef`, granting full read access to any component on the
   candidate entity during broadphase and narrowphase traversal. This enables queries like "find the
   nearest enemy not behind cover" without post-filtering large result sets.
   - **Deps:** F-4.4.1, F-4.2.6, F-1.1.17 (Composable Archetype Queries)

## Public Query Resource and Oriented Shapes

| ID      | Feature |
|---------|----------------------------------- |
| F-4.4.7 | PhysicsQueries ECS Resource API |
| F-4.4.8 | Oriented Shape Cast and Overlap |

1. **F-4.4.7** — A `PhysicsQueries` ECS resource exposes ray cast, shape cast, point overlap, and
   AABB overlap APIs that any ECS system can call via `Res<PhysicsQueries>`. The resource holds a
   read-only handle into the physics BVH and dispatches to the same broadphase/narrowphase routines
   used by the system-parameter query APIs. Callers supply collision filter function pointers (not
   boxed closures) so batch calls have bounded per-query dispatch cost.
   - **Deps:** F-4.4.1, F-4.4.2, F-4.4.3, F-1.1.23 (World Resources)
2. **F-4.4.8** — Shape cast and overlap APIs accept an optional `Quat` rotation parameter that
   orients capsule, box, and convex hull query shapes. The default identity rotation preserves
   existing call sites. Narrowphase dispatch uses the rotated shape when computing contacts,
   enabling non-axis-aligned sweeps for weapon hitboxes, rotated capsule projectiles, and oriented
   AI perception volumes.
   - **Deps:** F-4.4.2, F-4.4.3, F-4.2.3 (Primitive and Convex Collision Shapes)
