# R-4.4 — Spatial Queries Requirements

## R-4.4.1 Ray Casting

The engine **SHALL** provide a `RayCast` system parameter that queries the shared spatial index
(F-1.9.1) and returns hit entity ID, position, normal, distance, and `CollisionLayers` for the
nearest intersected `Collider` entity, with hit position accurate to within 0.1 mm of the
analytical intersection point.

- **Derived from:** [F-4.4.1](../../features/physics/spatial-queries.md)
- **Rationale:** Ray casts are the most frequently used spatial query for line-of-sight, weapon
  targeting, ground detection, and AI perception.
- **Verification:** Unit test — cast rays against sphere, box, and capsule colliders at known
  positions; compare returned hit position and normal against analytical solutions; assert
  position error is below 0.1 mm and normal error is below 0.001 radians.

## R-4.4.2 Shape Casting (Sweep Tests)

The engine **SHALL** provide a `ShapeCast` system parameter that sweeps a collision shape along
a direction and reports the first contact with a `Collider` entity, including hit entity ID,
contact point, normal, and penetration depth.

- **Derived from:** [F-4.4.2](../../features/physics/spatial-queries.md)
- **Rationale:** Shape casts are essential for character controller ground detection, projectile
  hit volumes, and safe movement validation.
- **Verification:** Unit test — sweep a sphere and a capsule toward known collider positions;
  assert the reported contact point is within 0.1 mm of the analytical first-contact point;
  verify that "no hit" is correctly returned when sweeping in a direction with no obstacles.

## R-4.4.3 Overlap Tests

The engine **SHALL** provide an `OverlapQuery` system parameter that returns all entity IDs
whose `Collider` shapes overlap a given query shape at a given position, with zero false
negatives for shapes whose AABB overlap is non-empty.

- **Derived from:** [F-4.4.3](../../features/physics/spatial-queries.md)
- **Rationale:** Overlap tests power area-of-effect abilities, trigger volumes, and proximity
  gameplay logic.
- **Verification:** Unit test — place 100 colliders at known positions; perform sphere overlap
  queries of varying radii; compare returned entity sets against a brute-force reference; assert
  zero false negatives and zero false positives.

## R-4.4.4 Closest Point Queries

The engine **SHALL** provide a `ClosestPointQuery` system parameter that returns the nearest
point on any `Collider` surface to a given world-space point, with the result accurate to
within 0.1 mm of the analytical closest point.

- **Derived from:** [F-4.4.4](../../features/physics/spatial-queries.md)
- **Rationale:** Distance-based triggers, attachment point calculation, and AI navigation
  require precise geometric proximity queries.
- **Verification:** Unit test — query closest points against sphere, box, and capsule colliders
  at known positions; assert returned point differs from the analytical closest point by less
  than 0.1 mm; verify signed distance matches expected value.

## R-4.4.5 Batch Query Execution

The engine **SHALL** execute batches of 1,000 or more spatial queries in parallel across worker
threads, completing within 2x the wall-clock time of a single query for batches of 1,000 on a
system with 8 or more logical cores.

- **Derived from:** [F-4.4.5](../../features/physics/spatial-queries.md)
- **Rationale:** Server-side AI and gameplay systems issue hundreds of queries per tick; batch
  parallelism is required to meet frame budgets.
- **Verification:** Benchmark — submit a batch of 1,000 ray casts on an 8-core system; measure
  wall-clock completion time; assert it is no more than 2x the time of a single ray cast.
  Compare against sequential execution and assert at least 4x speedup.

## R-4.4.6 Query Filtering and Custom Predicates

The engine **SHALL** support `QueryFilter` parameters on all spatial queries that combine
`CollisionLayers` bitmask filtering with ECS structural filters (`With<T>`, `Without<T>`) and
an optional custom predicate closure, excluding filtered entities before narrowphase evaluation.

- **Derived from:** [F-4.4.6](../../features/physics/spatial-queries.md)
- **Rationale:** Pre-filtering at broadphase avoids expensive narrowphase work on irrelevant
  entities and eliminates post-filter passes over large result sets.
- **Verification:** Integration test — populate 500 entities across 4 collision layers; issue a
  ray cast with a `QueryFilter` that excludes 2 layers and requires `With<MarkerA>`;
  assert the result only contains entities matching the filter; verify excluded entities are
  never evaluated in narrowphase (instrument narrowphase to count evaluations).

---

## Non-Functional Requirements

### R-4.4.NF1 Single Ray Cast Latency

A single ray cast against a scene of 50,000 collider entities **SHALL** complete within
0.005 ms (5 microseconds) on minimum-spec hardware, including BVH traversal and narrowphase
intersection.

- **Derived from:** R-4.4.1
- **Rationale:** Individual ray casts are issued by gameplay code on the main thread; they
  must be near-instantaneous to avoid blocking frame completion.
- **Verification:** Benchmark — populate 50,000 collider entities; issue a single ray cast;
  measure wall-clock time; assert it completes within 5 microseconds.

### R-4.4.NF2 Batch Query Scalability

Batch query throughput **SHALL** scale linearly with core count up to 16 cores, achieving at
least 75% parallel efficiency (speedup / core count) for batches of 1,000 or more queries.

- **Derived from:** R-4.4.5
- **Rationale:** Server workloads issue thousands of queries per tick; scaling with available
  cores is critical for dedicated server hardware utilization.
- **Verification:** Benchmark — submit 10,000 ray casts on systems with 4, 8, and 16 cores;
  measure throughput at each core count; assert parallel efficiency is at least 75%.
