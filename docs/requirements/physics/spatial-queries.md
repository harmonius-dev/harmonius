# R-4.4 — Spatial Queries Requirements

## F-4.4.1 Ray Casting

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.4.1 | Ray Cast via Shared Spatial Index: The engine **SHALL** provide a `RayCast` system parameter that queries the shared BVH spatial index against entities with `Collider` and `Transform` components. Results **SHALL** include hit entity ID, position, surface normal, distance, and `CollisionLayers` value. |  [F-4.4.1](../../features/physics/spatial-queries.md) | Ray casts are foundational for line-of-sight, weapon targeting, and AI perception; they must use the shared index to avoid redundancy. | Cast rays against sphere, box, and capsule colliders at known positions. Assert hit position error is below 0.1 mm. |
| R-4.4.1a | Ray Cast Normal Accuracy: Ray cast surface normals **SHALL** be within 0.001 radians of the analytical solution for all primitive shape types. |  [F-4.4.1](../../features/physics/spatial-queries.md) | Accurate normals are required for reflection, decal placement, and slope calculations. | Cast rays against analytically defined surfaces. Assert normals differ by less than 0.001 radians from expected values. |
| R-4.4.1b | Single Ray Cast Latency: A single ray cast against 50,000 collider entities **SHALL** complete within 5 microseconds on minimum-spec hardware, including BVH traversal and narrowphase intersection. |  [F-4.4.1](../../features/physics/spatial-queries.md) | Gameplay code issues ray casts on the main thread; they must be near-instantaneous to avoid blocking frame completion. | Benchmark: populate 50,000 collider entities. Issue a single ray cast. Assert completion within 5 microseconds. |

## F-4.4.2 Shape Casting (Sweep Tests)

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.4.2 | Shape Cast System Parameter: The engine **SHALL** provide a `ShapeCast` system parameter that sweeps a collision shape (sphere, capsule, box, convex hull) along a direction against entities with `Collider` components, reporting first or all contacts with entity ID, contact point, normal, penetration depth, and `CollisionLayers`. |  [F-4.4.2](../../features/physics/spatial-queries.md) | Shape casts power character ground detection, projectile hit detection, and safe movement queries requiring swept-volume tests. | Sweep a sphere and capsule toward known colliders. Assert contact points are within 0.1 mm of the analytical first-contact point. |
| R-4.4.2a | No-Hit Correctness: Shape casts in directions with no obstacles **SHALL** return a "no hit" result with no false positives. |  [F-4.4.2](../../features/physics/spatial-queries.md) | False positives cause characters to get stuck or projectiles to stop in empty space. | Sweep in a direction with no obstacles. Assert "no hit" is returned. |

## F-4.4.3 Overlap Tests

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.4.3 | Overlap Query System Parameter: The engine **SHALL** provide an `OverlapQuery` system parameter that tests a shape (sphere, box, capsule, convex hull) against entities with `Collider` components, returning all overlapping entity IDs with `CollisionLayers` values. |  [F-4.4.3](../../features/physics/spatial-queries.md) | AoE abilities, trigger volumes, and proximity logic all require efficient spatial overlap tests. | Place 100 colliders at known positions. Perform sphere overlap queries of varying radii. Assert zero false negatives and zero false positives. |

## F-4.4.4 Closest Point Queries

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.4.4 | Closest Point Query System Parameter: The engine **SHALL** provide a `ClosestPointQuery` system parameter that computes the closest point on any `Collider` surface to a given world-space position, returning closest entity ID, closest point, surface normal, and signed distance. |  [F-4.4.4](../../features/physics/spatial-queries.md) | Distance-based triggers, attachment calculations, and AI navigation require precise geometric proximity queries. | Query closest points against sphere, box, and capsule colliders. Assert results are within 0.1 mm of the analytical closest point. |
| R-4.4.4a | Signed Distance Classification: Signed distance values **SHALL** be negative for points inside collider shapes and positive for points outside. |  [F-4.4.4](../../features/physics/spatial-queries.md) | Inside/outside classification is required for containment tests and penetration resolution. | Query signed distance for points known to be inside and outside colliders. Assert correct sign. |

## F-4.4.5 Batch Query Execution

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.4.5 | Parallel Batch Queries: The engine **SHALL** provide a `BatchSpatialQuery` that accepts a slice of query descriptors (ray casts, shape casts, overlaps) and distributes them across worker threads via the ECS job system for parallel execution. |  [F-4.4.5](../../features/physics/spatial-queries.md) | Server-side AI issues hundreds of queries per tick; parallel execution is required to meet frame budgets. | Submit 1000 ray casts on an 8-core system. Assert completion is at least 4x faster than sequential execution. |
| R-4.4.5a | Batch Query Scalability: Batch query throughput **SHALL** scale linearly with core count up to 16 cores, achieving at least 75% parallel efficiency for batches of 1,000 or more queries. |  [F-4.4.5](../../features/physics/spatial-queries.md) | Dedicated server hardware has many cores; batch queries must scale to utilize them. | Benchmark: submit 10,000 ray casts on 4, 8, and 16 core systems. Assert parallel efficiency is at least 75%. |

## F-4.4.6 Query Filtering and Custom Predicates

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.4.6 | Composable Query Filters: All spatial queries **SHALL** accept a `QueryFilter` that combines `CollisionLayers` bitmask filtering with ECS structural filters (`With<T>`, `Without<T>`) and optional custom predicate closures. Filtering **SHALL** be evaluated at broadphase traversal time to reject entities before narrowphase. |  [F-4.4.6](../../features/physics/spatial-queries.md) | Early rejection at broadphase eliminates unnecessary narrowphase evaluations, improving query throughput for filtered searches. | Issue a ray cast excluding 2 layers and requiring `With<MarkerA>`. Instrument narrowphase. Assert excluded entities are never evaluated. |

## Non-Functional Requirements

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.4.NF1 | Single Ray Cast Latency: A single ray cast against a scene of 50,000 collider entities **SHALL** complete within 0.005 ms (5 microseconds) on minimum-spec hardware, including BVH traversal and narrowphase intersection. | R-4.4.1 | Individual ray casts are issued by gameplay code on the main thread; they must be near-instantaneous to avoid blocking frame completion. | Benchmark -- populate 50,000 collider entities; issue a single ray cast; measure wall-clock time; assert it completes within 5 microseconds. |
| R-4.4.NF2 | Batch Query Scalability: Batch query throughput **SHALL** scale linearly with core count up to 16 cores, achieving at least 75% parallel efficiency (speedup / core count) for batches of 1,000 or more queries. | R-4.4.5 | Server workloads issue thousands of queries per tick; scaling with available cores is critical for dedicated server hardware utilization. | Benchmark -- submit 10,000 ray casts on systems with 4, 8, and 16 cores; measure throughput at each core count; assert parallel efficiency is at least 75%. |
