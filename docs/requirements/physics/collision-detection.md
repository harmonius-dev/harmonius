# R-4.2 — Collision Detection Requirements

## F-4.2.1 Broadphase Acceleration Structures

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.1  | [F-4.2.1](../../features/physics/collision-detection.md) |
| R-4.2.1a | [F-4.2.1](../../features/physics/collision-detection.md) |

1. **R-4.2.1** — Shared BVH Broadphase: The engine **SHALL** cull collision pairs using the shared
   BVH spatial index rather than a physics-private acceleration structure. The
   `BroadphaseQuerySystem` **SHALL** read the shared BVH resource and write overlapping AABB pairs
   filtered by `CollisionLayers` to a `BroadphasePairs` resource.
   - **Rationale:** A shared BVH eliminates redundant spatial structures across physics, rendering,
     and AI, reducing memory and rebuild cost.
   - **Verification:** Populate 1000 entities with random colliders. Compare `BroadphasePairs`
     output against brute-force AABB overlap. Assert zero false negatives.
2. **R-4.2.1a** — Broadphase Throughput: Broadphase pair culling **SHALL** process at least 50,000
   AABB-bearing entities within 1 ms on minimum-spec hardware, including BVH traversal and collision
   layer filtering.
   - **Rationale:** Broadphase is the first stage of every physics tick; its cost must remain a
     small fraction of the total physics budget.
   - **Verification:** Benchmark: populate 50,000 entities with random AABB extents. Measure
     broadphase query time. Assert completion within 1 ms.

## F-4.2.2 Narrowphase Contact Generation

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.2  | [F-4.2.2](../../features/physics/collision-detection.md) |
| R-4.2.2a | [F-4.2.2](../../features/physics/collision-detection.md) |
| R-4.2.2b | [F-4.2.2](../../features/physics/collision-detection.md) |

1. **R-4.2.2** — GJK/EPA/SAT Contact Generation: The engine **SHALL** generate precise contact
   points and penetration depths using GJK for distance, EPA for penetration, and SAT for
   feature-based contacts. The `NarrowphaseSystem` **SHALL** read candidate pairs from
   `BroadphasePairs` and produce `ContactManifold` components containing contact points, normals,
   and penetration depths.
   - **Rationale:** Multiple algorithms are needed: GJK for general convex pairs, SAT for fast
     primitive cases, and EPA when penetration depth is required.
   - **Verification:** Position known primitive pairs at analytically computable overlaps. Assert
     penetration depth accuracy within 1 mm.
2. **R-4.2.2a** — Narrowphase Determinism: Narrowphase contact generation **SHALL** produce
   bit-identical `ContactManifold` values when run twice with identical inputs.
   - **Rationale:** Deterministic contacts are required for server-authoritative simulation with
     client-side prediction.
   - **Verification:** Run the same narrowphase scenario twice. Assert bit-identical contact
     manifolds.
3. **R-4.2.2b** — Narrowphase Throughput: Narrowphase contact generation **SHALL** process at least
   10,000 primitive-vs-primitive candidate pairs within 2 ms on minimum-spec hardware.
   - **Rationale:** Narrowphase dominates collision detection cost; it must handle peak loads during
     explosions and dense combat.
   - **Verification:** Benchmark: generate 10,000 overlapping primitive pairs. Measure narrowphase
     time. Assert completion within 2 ms.

## F-4.2.3 Primitive and Convex Collision Shapes

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.3  | [F-4.2.3](../../features/physics/collision-detection.md) |
| R-4.2.3a | [F-4.2.3](../../features/physics/collision-detection.md) |

1. **R-4.2.3** — Collider Shape Types: The engine **SHALL** support `Box`, `Sphere`, `Capsule`, and
   `ConvexHull` shape variants within the `Collider` ECS component. Convex hulls **SHALL** be
   generated from meshes at build time with configurable vertex limits.
   - **Rationale:** These four primitives cover the vast majority of game object silhouettes;
     build-time hull generation avoids runtime cost.
   - **Verification:** Create entities with each of the four shape types. Assert correct contact
     manifolds are generated for all shape-pair combinations.
2. **R-4.2.3a** — Fast-Path Primitive Dispatch: The `NarrowphaseSystem` **SHALL** dispatch to
   specialized routines for common primitive pairs (sphere-sphere, sphere-box, capsule-box) that
   bypass generic GJK, achieving at least 2x speedup over the generic path.
   - **Rationale:** Primitive-vs-primitive pairs are the most frequent collision type; specialized
     routines avoid the overhead of iterative GJK.
   - **Verification:** Benchmark: process 10,000 sphere-vs-sphere pairs via fast path and generic
     GJK. Assert at least 2x speedup for the fast path.

## F-4.2.4 Triangle Mesh and Heightfield Shapes

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.4  | [F-4.2.4](../../features/physics/collision-detection.md) |
| R-4.2.4a | [F-4.2.4](../../features/physics/collision-detection.md) |

1. **R-4.2.4** — Concave Shape Support: The engine **SHALL** support `ColliderShape::TriMesh` and
   `ColliderShape::Heightfield` variants within the `Collider` component, integrated with the shared
   BVH for efficient culling.
   - **Rationale:** Static environment geometry is typically concave; triangle meshes and
     heightfields represent terrain and architecture accurately.
   - **Verification:** Assign `TriMesh` to a static entity and `Heightfield` to terrain. Drop rigid
     bodies onto both. Assert correct contact manifolds are generated.
2. **R-4.2.4a** — Per-Triangle Material Indices: Triangle mesh and heightfield shapes **SHALL**
   support per-triangle material indices that map to `PhysicsMaterial` entries for surface-specific
   friction and restitution.
   - **Rationale:** Terrain surfaces vary (dirt, stone, mud); per-triangle materials enable
     surface-specific responses from a single collider.
   - **Verification:** Create a heightfield with two regions assigned different physics materials.
     Drop a sphere onto each. Assert correct friction and restitution values.

## F-4.2.5 Compound Shapes

| ID      | Derived From                                             |
|---------|----------------------------------------------------------|
| R-4.2.5 | [F-4.2.5](../../features/physics/collision-detection.md) |

1. **R-4.2.5** — Compound Collider Component: The engine **SHALL** support a `CompoundCollider`
   component that combines multiple primitive or convex child shapes with local-space offsets. Each
   child shape **SHALL** carry its own `CollisionLayers` and `PhysicsMaterial` data.
   - **Rationale:** Complex objects need accurate collision without concave decomposition; per-child
     materials enable different surface properties on one entity.
   - **Verification:** Create a compound collider with 3 child shapes, each with different layers
     and materials. Fire a ray that hits only one child. Assert the hit reports that child's layer
     and material.

## F-4.2.6 Collision Filtering and Layers

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.6  | [F-4.2.6](../../features/physics/collision-detection.md) |
| R-4.2.6a | [F-4.2.6](../../features/physics/collision-detection.md) |

1. **R-4.2.6** — Layer-Based Collision Filtering: The engine **SHALL** filter collisions using a
   `CollisionLayers` component with membership and mask bitsets. The `BroadphaseUpdateSystem`
   **SHALL** evaluate layer compatibility at broadphase to reject pairs before narrowphase.
   - **Rationale:** Layer-based filtering is cheap (bitwise AND) and prevents wasted narrowphase
     evaluations on non-interacting entity groups.
   - **Verification:** Create two overlapping entities on non-interacting layers. Assert no
     `ContactManifold` is generated. Place them on interacting layers. Assert a `ContactManifold` is
     generated.
2. **R-4.2.6a** — Custom Filter Callbacks: The engine **SHALL** support registering a
   `CollisionFilterCallback` system for fine-grained runtime filtering (team-based rules,
   ignore-owner logic) beyond layer-based filtering.
   - **Rationale:** Gameplay rules like "don't collide with own projectiles" cannot be expressed
     with static layers.
   - **Verification:** Register a callback that rejects pairs sharing the same `TeamId` component.
     Create overlapping same-team entities. Assert no contact is generated.

## F-4.2.7 Collision Events

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.7  | [F-4.2.7](../../features/physics/collision-detection.md) |
| R-4.2.7a | [F-4.2.7](../../features/physics/collision-detection.md) |

1. **R-4.2.7** — Typed Collision Event Lifecycle: The engine **SHALL** emit typed
   `CollisionStarted`, `CollisionPersisted`, and `CollisionEnded` events through the ECS event
   system. Each event **SHALL** carry contact points, normals, impulse magnitudes, and participating
   entity IDs.
   - **Rationale:** Gameplay systems rely on precise collision lifecycle events for damage, sound,
     and VFX triggers.
   - **Verification:** Move two spheres into contact for 5 frames then separate. Assert exactly one
     `CollisionStarted`, four `CollisionPersisted`, and one `CollisionEnded` event in correct order.
2. **R-4.2.7a** — Zero-Frame Event Delivery: Collision events **SHALL** be available to gameplay
   systems within the same frame they are detected, with zero frames of delivery delay.
   - **Rationale:** Delayed events cause visible desyncs between physics impacts and gameplay
     feedback.
   - **Verification:** Trigger a collision. Assert the event is readable by a system scheduled after
     the collision event system in the same frame.

## F-4.2.8 Trigger Volumes

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.8  | [F-4.2.8](../../features/physics/collision-detection.md) |
| R-4.2.8a | [F-4.2.8](../../features/physics/collision-detection.md) |

1. **R-4.2.8** — Non-Physical Overlap Detection: The engine **SHALL** support `TriggerVolume`
   entities that detect overlap without generating contact responses. The system **SHALL** emit
   `TriggerEnter`, `TriggerStay`, and `TriggerExit` events via the observer system.
   - **Rationale:** Quest regions, loading zones, and AoE hazards need spatial detection without
     physical forces.
   - **Verification:** Move a rigid body through a trigger volume. Assert `TriggerEnter`,
     `TriggerStay`, and `TriggerExit` fire correctly. Assert the body's velocity is unaffected.
2. **R-4.2.8a** — Trigger Volume Types: The engine **SHALL** support one-shot triggers (fire once
   then disable), persistent triggers (fire every frame), and filtered triggers (respond only to
   entities matching a query filter).
   - **Rationale:** Different gameplay scenarios require different trigger behaviors; one-shot for
     cutscenes, persistent for hazards, filtered for team-specific zones.
   - **Verification:** Configure a one-shot trigger. Move an entity through it twice. Assert only
     one event fires. Configure a filtered trigger. Move matching and non-matching entities through
     it. Assert only matching entities trigger events.

## F-4.2.9 Physics Material Assets

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.2.9  | [F-4.2.9](../../features/physics/collision-detection.md) |
| R-4.2.9a | [F-4.2.9](../../features/physics/collision-detection.md) |

1. **R-4.2.9** — Data-Driven Physics Materials: The engine **SHALL** support `PhysicsMaterial`
   assets with static friction, dynamic friction, restitution, density, and surface type tag.
   Materials **SHALL** be assigned per collider via a `PhysicsMaterialHandle` component.
   - **Rationale:** Data-driven materials enable designers to define surface properties without code
     changes.
   - **Verification:** Create two materials with different friction. Assign them to two colliders.
     Assert the contact solver uses the correct combined friction.
2. **R-4.2.9a** — Surface Type Tags: Physics material surface type tags (metal, wood, ice, rubber)
   **SHALL** be accessible to downstream systems for driving audio (footstep sounds) and VFX (impact
   particles).
   - **Rationale:** Surface tags unify physics, audio, and VFX so that material properties drive all
     feedback channels.
   - **Verification:** Create materials tagged "metal" and "wood". Verify a collision event carries
     the correct surface tags for both colliders.

## Non-Functional Requirements

| ID        | Derived From     |
|-----------|------------------|
| R-4.2.NF1 | R-4.2.1, R-4.2.6 |
| R-4.2.NF2 | R-4.2.2, R-4.2.3 |
| R-4.2.NF3 | R-4.2.7, R-4.2.8 |

1. **R-4.2.NF1** — Broadphase Throughput: Broadphase pair culling **SHALL** process at least 50,000
   AABB-bearing entities within 1 ms on minimum-spec hardware, including shared BVH traversal and
   collision layer filtering.
   - **Rationale:** Broadphase is the first stage of every physics tick; its cost scales with total
     entity count and must remain a small fraction of the total physics budget.
   - **Verification:** Benchmark -- populate 50,000 entities with random AABB extents; measure
     broadphase query time; assert it completes within 1 ms.
2. **R-4.2.NF2** — Narrowphase Throughput: Narrowphase contact generation **SHALL** process at least
   10,000 candidate pairs within 2 ms on minimum-spec hardware for primitive-vs-primitive pair
   types.
   - **Rationale:** Narrowphase dominates collision detection cost; it must handle peak contact
     loads during explosions, collapses, and dense combat without exceeding the physics budget.
   - **Verification:** Benchmark -- generate 10,000 overlapping primitive pairs; measure narrowphase
     contact generation time; assert it completes within 2 ms.
3. **R-4.2.NF3** — Collision Event Latency: Collision events (`CollisionStarted`,
   `CollisionPersisted`, `CollisionEnded`) **SHALL** be available to gameplay systems within the
   same frame they are detected, with zero frames of event delivery delay.
   - **Rationale:** Gameplay systems (damage application, sound triggers, VFX spawning) rely on
     same-frame event delivery to avoid visible desyncs between physics and effects.
   - **Verification:** Integration test -- trigger a collision; assert the event is readable by a
     system scheduled after the collision event system in the same frame.
