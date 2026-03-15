# R-4.2 — Collision Detection Requirements

## R-4.2.1 Broadphase via Shared Spatial Index

The engine **SHALL** perform broadphase collision pair culling by querying the shared BVH
spatial index (F-1.9.1) rather than maintaining a physics-private acceleration structure, and
produce identical candidate pairs to an exhaustive AABB overlap test.

- **Derived from:** [F-4.2.1](../../features/physics/collision-detection.md)
- **Rationale:** Sharing the spatial index with rendering, networking, and AI avoids redundant
  structure rebuilds and reduces memory overhead.
- **Verification:** Integration test — populate 1,000 entities with random `Collider` and
  `Transform` components; compare `BroadphasePairs` output against a brute-force AABB overlap
  reference; assert zero false negatives.

## R-4.2.2 Narrowphase Contact Generation

The engine **SHALL** generate contact manifolds with position error (penetration depth) within
1 mm of the analytical solution for all supported primitive-vs-primitive shape pairs.

- **Derived from:** [F-4.2.2](../../features/physics/collision-detection.md)
- **Rationale:** Accurate contact points and normals are the foundation for stable impulse
  resolution and constraint solving.
- **Verification:** Unit test — for each pair of supported primitives (sphere-sphere,
  sphere-box, capsule-box, etc.), position shapes at known overlapping configurations with
  analytically computable penetration; assert generated contact depth differs from the
  analytical value by less than 1 mm.

## R-4.2.3 Primitive and Convex Collision Shapes

The engine **SHALL** support `Box`, `Sphere`, `Capsule`, and `ConvexHull` variants in the
`ColliderShape` enum, and dispatch narrowphase evaluation for primitive-vs-primitive pairs
through specialized fast-path routines rather than generic GJK.

- **Derived from:** [F-4.2.3](../../features/physics/collision-detection.md)
- **Rationale:** Specialized routines for common primitive pairs are significantly faster than
  generic algorithms and dominate real-world collision workloads.
- **Verification:** Benchmark — measure narrowphase evaluation time for 10,000
  sphere-vs-sphere pairs using the specialized path vs. generic GJK; assert the specialized
  path is at least 2x faster. Unit test — verify all four shape types produce correct contact
  manifolds.

## R-4.2.4 Triangle Mesh and Heightfield Shapes

The engine **SHALL** support `TriMesh` and `Heightfield` collider shape variants that resolve
per-triangle material indices to `PhysicsMaterial` entries for surface-specific friction and
restitution.

- **Derived from:** [F-4.2.4](../../features/physics/collision-detection.md)
- **Rationale:** Terrain and static environment geometry require concave collision support with
  per-surface material variation for correct physical response and gameplay effects.
- **Verification:** Integration test — create a heightfield with two regions assigned different
  `PhysicsMaterial` entries; drop a sphere onto each region; assert friction and restitution
  values applied during contact resolution match the respective material.

## R-4.2.5 Compound Shapes

The engine **SHALL** support a `CompoundCollider` component that combines multiple primitive or
convex child shapes with independent `CollisionLayers` and `PhysicsMaterial` data per child.

- **Derived from:** [F-4.2.5](../../features/physics/collision-detection.md)
- **Rationale:** Complex object silhouettes (vehicles, furniture, characters) need multi-shape
  colliders without expensive concave decomposition.
- **Verification:** Unit test — create a compound collider with three child shapes on different
  collision layers; fire a ray that intersects only one child; assert the hit reports the
  correct child's `CollisionLayers` and `PhysicsMaterial` values.

## R-4.2.6 Collision Filtering and Layers

The engine **SHALL** reject broadphase collision pairs whose `CollisionLayers` membership and
mask bitsets have no overlapping bits, preventing those pairs from reaching narrowphase.

- **Derived from:** [F-4.2.6](../../features/physics/collision-detection.md)
- **Rationale:** Layer-based filtering is the primary mechanism for separating collision domains
  (player, enemy, projectile, trigger) and reducing narrowphase workload.
- **Verification:** Unit test — create two overlapping entities on non-interacting layers;
  assert no `ContactManifold` is generated. Create two overlapping entities on interacting
  layers; assert a `ContactManifold` is generated.

## R-4.2.7 Collision Events

The engine **SHALL** emit `CollisionStarted`, `CollisionPersisted`, and `CollisionEnded` ECS
events with correct transition semantics: `CollisionStarted` fires on the first frame of
contact, `CollisionPersisted` fires on each subsequent frame, and `CollisionEnded` fires on the
first frame after separation.

- **Derived from:** [F-4.2.7](../../features/physics/collision-detection.md)
- **Rationale:** Gameplay systems (damage, sound, VFX) rely on precise collision lifecycle
  events to trigger effects at the correct moment.
- **Verification:** Integration test — move two spheres into contact, hold for 5 frames, then
  separate; assert exactly one `CollisionStarted`, exactly 4 `CollisionPersisted`, and exactly
  one `CollisionEnded` event are emitted in the correct frame order.

## R-4.2.8 Trigger Volumes

The engine **SHALL** emit `TriggerEnter`, `TriggerStay`, and `TriggerExit` events for entities
overlapping a `TriggerVolume` entity without generating contact response forces.

- **Derived from:** [F-4.2.8](../../features/physics/collision-detection.md)
- **Rationale:** Non-physical overlap detection is essential for quest zones, area-of-effect
  abilities, loading zone transitions, and trap activation.
- **Verification:** Integration test — move a dynamic rigid body through a trigger volume;
  assert `TriggerEnter` fires on entry, `TriggerStay` fires each frame while overlapping,
  `TriggerExit` fires on exit, and the rigid body's velocity is unaffected by the volume.

## R-4.2.9 Physics Material Assets

The engine **SHALL** resolve effective friction and restitution for a collision pair by
combining the `PhysicsMaterial` of both colliders using the configured combine mode (average,
minimum, maximum, multiply) and produce combined values within floating-point epsilon of the
expected result.

- **Derived from:** [F-4.2.9](../../features/physics/collision-detection.md)
- **Rationale:** Material combination rules let designers control surface interactions (ice,
  rubber, metal) without per-pair authoring.
- **Verification:** Unit test — for each combine mode, create two colliders with known friction
  and restitution values; assert the combined effective values equal the expected result (e.g.,
  average of 0.2 and 0.8 = 0.5) within floating-point epsilon.

---

## Non-Functional Requirements

### R-4.2.NF1 Broadphase Throughput

Broadphase pair culling **SHALL** process at least 50,000 AABB-bearing entities within 1 ms
on minimum-spec hardware, including shared BVH traversal and collision layer filtering.

- **Derived from:** R-4.2.1, R-4.2.6
- **Rationale:** Broadphase is the first stage of every physics tick; its cost scales with
  total entity count and must remain a small fraction of the total physics budget.
- **Verification:** Benchmark — populate 50,000 entities with random AABB extents; measure
  broadphase query time; assert it completes within 1 ms.

### R-4.2.NF2 Narrowphase Throughput

Narrowphase contact generation **SHALL** process at least 10,000 candidate pairs within 2 ms
on minimum-spec hardware for primitive-vs-primitive pair types.

- **Derived from:** R-4.2.2, R-4.2.3
- **Rationale:** Narrowphase dominates collision detection cost; it must handle peak contact
  loads during explosions, collapses, and dense combat without exceeding the physics budget.
- **Verification:** Benchmark — generate 10,000 overlapping primitive pairs; measure
  narrowphase contact generation time; assert it completes within 2 ms.

### R-4.2.NF3 Collision Event Latency

Collision events (`CollisionStarted`, `CollisionPersisted`, `CollisionEnded`) **SHALL** be
available to gameplay systems within the same frame they are detected, with zero frames of
event delivery delay.

- **Derived from:** R-4.2.7, R-4.2.8
- **Rationale:** Gameplay systems (damage application, sound triggers, VFX spawning) rely on
  same-frame event delivery to avoid visible desyncs between physics and effects.
- **Verification:** Integration test — trigger a collision; assert the event is readable by
  a system scheduled after the collision event system in the same frame.
