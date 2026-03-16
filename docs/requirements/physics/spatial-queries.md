# R-4.4 — Spatial Queries Requirements

## F-4.4.1 Ray Casting

### R-4.4.1 Ray Cast via Shared Spatial Index

The engine **SHALL** provide a `RayCast` system parameter that queries the shared BVH spatial index
against entities with `Collider` and `Transform` components. Results **SHALL** include hit entity
ID, position, surface normal, distance, and `CollisionLayers` value.

- **Derived from:**
  [F-4.4.1](../../features/physics/spatial-queries.md)
- **Rationale:** Ray casts are foundational for line-of-sight, weapon targeting, and AI perception;
  they must use the shared index to avoid redundancy.
- **Verification:** Cast rays against sphere, box, and capsule colliders at known positions. Assert
  hit position error is below 0.1 mm.

### R-4.4.1a Ray Cast Normal Accuracy

Ray cast surface normals **SHALL** be within 0.001 radians of the analytical solution for all
primitive shape types.

- **Derived from:**
  [F-4.4.1](../../features/physics/spatial-queries.md)
- **Rationale:** Accurate normals are required for reflection, decal placement, and slope
  calculations.
- **Verification:** Cast rays against analytically defined surfaces. Assert normals differ by less
  than 0.001 radians from expected values.

### R-4.4.1b Single Ray Cast Latency

A single ray cast against 50,000 collider entities **SHALL** complete within 5 microseconds on
minimum-spec hardware, including BVH traversal and narrowphase intersection.

- **Derived from:**
  [F-4.4.1](../../features/physics/spatial-queries.md)
- **Rationale:** Gameplay code issues ray casts on the main thread; they must be near-instantaneous
  to avoid blocking frame completion.
- **Verification:** Benchmark: populate 50,000 collider entities. Issue a single ray cast. Assert
  completion within 5 microseconds.

---

## US-4.4.1.1 Perform Ray Casts for Line-of-Sight

**As a** game developer (P-15), **I want to** use the `RayCast` system parameter to check
line-of-sight between two positions, **so that** AI perception and weapon targeting use a unified
spatial query API.

## US-4.4.1.2 Read Ray Cast Hit Results

**As a** game developer (P-15), **I want** ray cast results to include hit entity ID, position,
surface normal, distance, and `CollisionLayers`, **so that** I have full context about what was hit.

## US-4.4.1.3 Verify Ray Cast Position Accuracy

**As an** engine tester (P-27), **I want to** cast rays against sphere, box, and capsule colliders
at known positions and assert hit position error is below 0.1mm, **so that** geometric accuracy is
confirmed.

## US-4.4.1.4 Verify Ray Cast Normal Accuracy

**As an** engine tester (P-27), **I want to** verify ray cast surface normals are within 0.001
radians of the analytical solution, **so that** normal accuracy is validated.

## US-4.4.1.5 Benchmark Single Ray Cast Latency

**As an** engine tester (P-27), **I want to** cast a single ray against 50000 collider entities and
assert completion within 5 microseconds, **so that** per-query latency meets the performance
requirement.

## US-4.4.1.6 Implement Ray Cast via Shared Spatial Index

**As an** engine developer (P-26), **I want to** implement `RayCast` using the shared BVH spatial
index, traversing `Collider` and `Transform` components, **so that** ray casts share the same
acceleration structure as rendering and AI.

## US-4.4.1.7 Set Up Ray-Based Interactions in Editor

**As a** designer (P-5), **I want to** configure ray cast-based interactions (laser beams, gaze
targeting) in the visual editor, **so that** ray-driven gameplay is data-driven.

## US-4.4.1.8 Experience Accurate Targeting

**As a** player (P-23), **I want** weapon targeting and crosshairs to hit exactly what I aim at,
**so that** combat aiming feels precise and fair.

## US-4.4.1.9 Place Laser Traps Using Ray Casts

**As a** level designer (P-6), **I want to** set up laser trap lines using ray casts between emitter
and receiver entities, **so that** environmental puzzles use physics queries.

---

## F-4.4.2 Shape Casting (Sweep Tests)

### R-4.4.2 Shape Cast System Parameter

The engine **SHALL** provide a `ShapeCast` system parameter that sweeps a collision shape (sphere,
capsule, box, convex hull) along a direction against entities with `Collider` components, reporting
first or all contacts with entity ID, contact point, normal, penetration depth, and
`CollisionLayers`.

- **Derived from:**
  [F-4.4.2](../../features/physics/spatial-queries.md)
- **Rationale:** Shape casts power character ground detection, projectile hit detection, and safe
  movement queries requiring swept-volume tests.
- **Verification:** Sweep a sphere and capsule toward known colliders. Assert contact points are
  within 0.1 mm of the analytical first-contact point.

### R-4.4.2a No-Hit Correctness

Shape casts in directions with no obstacles **SHALL** return a "no hit" result with no false
positives.

- **Derived from:**
  [F-4.4.2](../../features/physics/spatial-queries.md)
- **Rationale:** False positives cause characters to get stuck or projectiles to stop in empty
  space.
- **Verification:** Sweep in a direction with no obstacles. Assert "no hit" is returned.

---

## US-4.4.2.1 Perform Shape Casts for Ground Detection

**As a** game developer (P-15), **I want to** use `ShapeCast` to sweep a capsule downward for ground
detection, **so that** character controllers find the ground surface accurately.

## US-4.4.2.2 Perform Shape Casts for Safe Movement

**As a** game developer (P-15), **I want to** sweep a collision shape along a movement direction to
check for obstructions before moving an entity, **so that** movement validation prevents clipping.

## US-4.4.2.3 Verify Shape Cast Contact Accuracy

**As an** engine tester (P-27), **I want to** sweep a sphere and capsule toward known colliders and
assert contact points are within 0.1mm of the analytical first-contact point, **so that** shape cast
precision is confirmed.

## US-4.4.2.4 Verify No-Hit Returns

**As an** engine tester (P-27), **I want to** sweep in a direction with no obstacles and verify "no
hit" is correctly returned, **so that** false positives are ruled out.

## US-4.4.2.5 Implement Shape Cast with Broadphase Integration

**As an** engine developer (P-26), **I want to** implement `ShapeCast` that sweeps collision shapes
through the broadphase and narrowphase, reporting first or all contacts, **so that** swept spatial
queries are available.

## US-4.4.2.6 Experience Projectile Hit Volumes

**As a** player (P-23), **I want** projectiles with physical size to hit targets they visually
contact, **so that** ranged attacks feel accurate.

---

## F-4.4.3 Overlap Tests

### R-4.4.3 Overlap Query System Parameter

The engine **SHALL** provide an `OverlapQuery` system parameter that tests a shape (sphere, box,
capsule, convex hull) against entities with `Collider` components, returning all overlapping entity
IDs with `CollisionLayers` values.

- **Derived from:**
  [F-4.4.3](../../features/physics/spatial-queries.md)
- **Rationale:** AoE abilities, trigger volumes, and proximity logic all require efficient spatial
  overlap tests.
- **Verification:** Place 100 colliders at known positions. Perform sphere overlap queries of
  varying radii. Assert zero false negatives and zero false positives.

---

## US-4.4.3.1 Perform Overlap Queries for AoE

**As a** game developer (P-15), **I want to** use `OverlapQuery` to find all entities within a
sphere for area-of-effect abilities, **so that** AoE damage targets are identified efficiently.

## US-4.4.3.2 Perform Overlap Queries with Query Shapes

**As a** game developer (P-15), **I want to** test overlap using sphere, box, capsule, and convex
hull query shapes, **so that** different AoE geometries are supported.

## US-4.4.3.3 Verify Overlap Test Accuracy

**As an** engine tester (P-27), **I want to** place 100 colliders at known positions, perform sphere
overlap queries of varying radii, and assert zero false negatives and zero false positives,
**so that** overlap accuracy is confirmed.

## US-4.4.3.4 Implement Overlap Query System

**As an** engine developer (P-26), **I want to** implement `OverlapQuery` that traverses the
broadphase resource and returns all overlapping entity IDs with `CollisionLayers`, **so that**
spatial overlap detection is available as a system parameter.

## US-4.4.3.5 Configure AoE Zones in Editor

**As a** designer (P-5), **I want to** define area-of-effect shapes and radii in the editor for
abilities, **so that** AoE gameplay is configured visually.

## US-4.4.3.6 Experience AoE Abilities Hitting Correct Targets

**As a** player (P-23), **I want** area-of-effect abilities to affect exactly the targets within the
visual effect radius, **so that** AoE feels accurate.

---

## F-4.4.4 Closest Point Queries

### R-4.4.4 Closest Point Query System Parameter

The engine **SHALL** provide a `ClosestPointQuery` system parameter that computes the closest point
on any `Collider` surface to a given world-space position, returning closest entity ID, closest
point, surface normal, and signed distance.

- **Derived from:**
  [F-4.4.4](../../features/physics/spatial-queries.md)
- **Rationale:** Distance-based triggers, attachment calculations, and AI navigation require precise
  geometric proximity queries.
- **Verification:** Query closest points against sphere, box, and capsule colliders. Assert results
  are within 0.1 mm of the analytical closest point.

### R-4.4.4a Signed Distance Classification

Signed distance values **SHALL** be negative for points inside collider shapes and positive for
points outside.

- **Derived from:**
  [F-4.4.4](../../features/physics/spatial-queries.md)
- **Rationale:** Inside/outside classification is required for containment tests and penetration
  resolution.
- **Verification:** Query signed distance for points known to be inside and outside colliders.
  Assert correct sign.

---

## US-4.4.4.1 Perform Closest Point Queries

**As a** game developer (P-15), **I want to** use `ClosestPointQuery` to find the nearest point on
any collider surface to a world-space position, **so that** distance-based triggers and attachment
calculations use precise geometry.

## US-4.4.4.2 Verify Closest Point Accuracy

**As an** engine tester (P-27), **I want to** query closest points against sphere, box, and capsule
colliders and assert the result is within 0.1mm of the analytical closest point, **so that**
geometric precision is confirmed.

## US-4.4.4.3 Verify Signed Distance Values

**As an** engine tester (P-27), **I want to** verify that signed distance values match expected
values (negative inside, positive outside), **so that** inside/outside classification is correct.

## US-4.4.4.4 Implement Closest Point Query

**As an** engine developer (P-26), **I want to** implement `ClosestPointQuery` that traverses the
broadphase and evaluates narrowphase geometry for the nearest surface point, **so that** precise
proximity queries are available.

## US-4.4.4.5 Experience Magnetic Attachment to Surfaces

**As a** player (P-23), **I want** objects that snap to surfaces (climbing holds, magnetic items) to
find the correct attachment point on any surface shape, **so that** attachment feels precise.

---

## F-4.4.5 Batch Query Execution

### R-4.4.5 Parallel Batch Queries

The engine **SHALL** provide a `BatchSpatialQuery` that accepts a slice of query descriptors (ray
casts, shape casts, overlaps) and distributes them across worker threads via the ECS job system for
parallel execution.

- **Derived from:**
  [F-4.4.5](../../features/physics/spatial-queries.md)
- **Rationale:** Server-side AI issues hundreds of queries per tick; parallel execution is required
  to meet frame budgets.
- **Verification:** Submit 1000 ray casts on an 8-core system. Assert completion is at least 4x
  faster than sequential execution.

### R-4.4.5a Batch Query Scalability

Batch query throughput **SHALL** scale linearly with core count up to 16 cores, achieving at least
75% parallel efficiency for batches of 1,000 or more queries.

- **Derived from:**
  [F-4.4.5](../../features/physics/spatial-queries.md)
- **Rationale:** Dedicated server hardware has many cores; batch queries must scale to utilize them.
- **Verification:** Benchmark: submit 10,000 ray casts on 4, 8, and 16 core systems. Assert parallel
  efficiency is at least 75%.

---

## US-4.4.5.1 Submit Batch Spatial Queries

**As a** game developer (P-15), **I want to** submit a batch of ray casts, shape casts, and overlaps
via `BatchSpatialQuery` for parallel execution, **so that** server-side AI issuing hundreds of
queries per tick benefits from parallelism.

## US-4.4.5.2 Verify Batch Query Parallel Speedup

**As an** engine tester (P-27), **I want to** submit 1000 ray casts on an 8-core system and assert
completion within 2x the time of a single ray cast and at least 4x faster than sequential,
**so that** parallel efficiency is confirmed.

## US-4.4.5.3 Verify Batch Query Scalability

**As an** engine tester (P-27), **I want to** submit 10000 ray casts on 4, 8, and 16 core systems
and assert at least 75% parallel efficiency, **so that** batch throughput scales linearly with
cores.

## US-4.4.5.4 Implement Batch Query via Job System

**As an** engine developer (P-26), **I want to** implement `BatchSpatialQuery` that distributes
query descriptors across worker threads via the ECS job system, **so that** batch queries exploit
multi-core parallelism.

## US-4.4.5.5 Experience Smooth Server Performance Under AI Load

**As a** player (P-23), **I want** game servers to handle hundreds of AI agents performing spatial
queries each tick without lag, **so that** multiplayer gameplay remains responsive.

---

## F-4.4.6 Query Filtering and Custom Predicates

### R-4.4.6 Composable Query Filters

All spatial queries **SHALL** accept a `QueryFilter` that combines `CollisionLayers` bitmask
filtering with ECS structural filters (`With<T>`, `Without<T>`) and optional custom predicate
closures. Filtering **SHALL** be evaluated at broadphase traversal time to reject entities before
narrowphase.

- **Derived from:**
  [F-4.4.6](../../features/physics/spatial-queries.md)
- **Rationale:** Early rejection at broadphase eliminates unnecessary narrowphase evaluations,
  improving query throughput for filtered searches.
- **Verification:** Issue a ray cast excluding 2 layers and requiring `With<MarkerA>`. Instrument
  narrowphase. Assert excluded entities are never evaluated.

---

## US-4.4.6.1 Filter Queries by Collision Layers

**As a** game developer (P-15), **I want to** pass a `QueryFilter` with `CollisionLayers` bitmask to
spatial queries, **so that** irrelevant entities are excluded before narrowphase.

## US-4.4.6.2 Filter Queries by ECS Components

**As a** game developer (P-15), **I want to** combine `CollisionLayers` filtering with `With<T>` and
`Without<T>` structural filters, **so that** queries target specific entity archetypes.

## US-4.4.6.3 Use Custom Predicate Closures

**As a** game developer (P-15), **I want to** pass a custom predicate closure to spatial queries
that reads any component on candidate entities, **so that** queries like "nearest enemy not behind
cover" work without post-filtering.

## US-4.4.6.4 Verify Filter Exclusion at Broadphase

**As an** engine tester (P-27), **I want to** issue a ray cast with a filter excluding 2 layers and
requiring `With<MarkerA>`, then instrument narrowphase to count evaluations and assert excluded
entities are never evaluated, **so that** filtering happens before narrowphase.

## US-4.4.6.5 Implement Query Filter in Spatial Query Pipeline

**As an** engine developer (P-26), **I want to** implement `QueryFilter` evaluation at broadphase
traversal time, combining layer masks, structural filters, and predicate closures, **so that**
unwanted entities are rejected as early as possible.

## US-4.4.6.6 Configure Query Filters in Editor

**As a** designer (P-5), **I want to** configure spatial query filters (target layers, required
components) in the visual editor for abilities and interactions, **so that** query targeting is
data-driven.

---

## Non-Functional Requirements

### R-4.4.NF1 Single Ray Cast Latency

A single ray cast against a scene of 50,000 collider entities **SHALL** complete within 0.005 ms (5
microseconds) on minimum-spec hardware, including BVH traversal and narrowphase intersection.

- **Derived from:** R-4.4.1
- **Rationale:** Individual ray casts are issued by gameplay code on the main thread; they must be
  near-instantaneous to avoid blocking frame completion.
- **Verification:** Benchmark -- populate 50,000 collider entities; issue a single ray cast; measure
  wall-clock time; assert it completes within 5 microseconds.

### R-4.4.NF2 Batch Query Scalability

Batch query throughput **SHALL** scale linearly with core count up to 16 cores, achieving at least
75% parallel efficiency (speedup / core count) for batches of 1,000 or more queries.

- **Derived from:** R-4.4.5
- **Rationale:** Server workloads issue thousands of queries per tick; scaling with available cores
  is critical for dedicated server hardware utilization.
- **Verification:** Benchmark -- submit 10,000 ray casts on systems with 4, 8, and 16 cores; measure
  throughput at each core count; assert parallel efficiency is at least 75%.
