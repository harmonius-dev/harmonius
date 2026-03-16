# R-4.2 — Collision Detection Requirements

## F-4.2.1 Broadphase Acceleration Structures

### R-4.2.1 Shared BVH Broadphase

The engine **SHALL** cull collision pairs using the shared BVH spatial index rather than a
physics-private acceleration structure. The `BroadphaseQuerySystem` **SHALL** read the shared BVH
resource and write overlapping AABB pairs filtered by `CollisionLayers` to a `BroadphasePairs`
resource.

- **Derived from:**
  [F-4.2.1](../../features/physics/collision-detection.md)
- **Rationale:** A shared BVH eliminates redundant spatial structures across physics, rendering, and
  AI, reducing memory and rebuild cost.
- **Verification:** Populate 1000 entities with random colliders. Compare `BroadphasePairs` output
  against brute-force AABB overlap. Assert zero false negatives.

### R-4.2.1a Broadphase Throughput

Broadphase pair culling **SHALL** process at least 50,000 AABB-bearing entities within 1 ms on
minimum-spec hardware, including BVH traversal and collision layer filtering.

- **Derived from:**
  [F-4.2.1](../../features/physics/collision-detection.md)
- **Rationale:** Broadphase is the first stage of every physics tick; its cost must remain a small
  fraction of the total physics budget.
- **Verification:** Benchmark: populate 50,000 entities with random AABB extents. Measure broadphase
  query time. Assert completion within 1 ms.

---

## US-4.2.1.1 Query Shared BVH for Collision Pairs

**As a** game developer (P-15), **I want** broadphase collision culling to query the shared BVH
spatial index rather than a physics-private structure, **so that** physics, rendering, and AI share
one acceleration structure without redundancy.

## US-4.2.1.2 Verify Broadphase Pair Accuracy

**As an** engine tester (P-27), **I want to** populate 1000 entities with random colliders and
compare `BroadphasePairs` output against brute-force AABB overlap, asserting zero false negatives,
**so that** broadphase correctness is confirmed.

## US-4.2.1.3 Benchmark Broadphase Throughput

**As an** engine tester (P-27), **I want to** benchmark broadphase pair culling with 50000
AABB-bearing entities and assert it completes within 1ms, **so that** broadphase performance meets
the throughput requirement.

## US-4.2.1.4 Implement Shared BVH Integration for Broadphase

**As an** engine developer (P-26), **I want to** implement the `BroadphaseQuerySystem` that reads
the shared BVH resource and writes overlapping AABB pairs filtered by `CollisionLayers` to the
`BroadphasePairs` resource, **so that** broadphase runs through the shared spatial index.

## US-4.2.1.5 Experience Physics Not Causing Frame Drops in Dense Scenes

**As a** player (P-23), **I want** physics collision detection to run efficiently even in scenes
with hundreds of objects, **so that** frame rates remain smooth during chaotic gameplay.

## US-4.2.1.6 Configure Broadphase Pair Budget Per Platform

**As a** designer (P-5), **I want to** see the platform-specific broadphase pair budget in the
editor (2048 mobile, 32K desktop), **so that** I can design scenes within performance limits.

---

## F-4.2.2 Narrowphase Contact Generation

### R-4.2.2 GJK/EPA/SAT Contact Generation

The engine **SHALL** generate precise contact points and penetration depths using GJK for distance,
EPA for penetration, and SAT for feature-based contacts. The `NarrowphaseSystem` **SHALL** read
candidate pairs from `BroadphasePairs` and produce `ContactManifold` components containing contact
points, normals, and penetration depths.

- **Derived from:**
  [F-4.2.2](../../features/physics/collision-detection.md)
- **Rationale:** Multiple algorithms are needed: GJK for general convex pairs, SAT for fast
  primitive cases, and EPA when penetration depth is required.
- **Verification:** Position known primitive pairs at analytically computable overlaps. Assert
  penetration depth accuracy within 1 mm.

### R-4.2.2a Narrowphase Determinism

Narrowphase contact generation **SHALL** produce bit-identical `ContactManifold` values when run
twice with identical inputs.

- **Derived from:**
  [F-4.2.2](../../features/physics/collision-detection.md)
- **Rationale:** Deterministic contacts are required for server-authoritative simulation with
  client-side prediction.
- **Verification:** Run the same narrowphase scenario twice. Assert bit-identical contact manifolds.

### R-4.2.2b Narrowphase Throughput

Narrowphase contact generation **SHALL** process at least 10,000 primitive-vs-primitive candidate
pairs within 2 ms on minimum-spec hardware.

- **Derived from:**
  [F-4.2.2](../../features/physics/collision-detection.md)
- **Rationale:** Narrowphase dominates collision detection cost; it must handle peak loads during
  explosions and dense combat.
- **Verification:** Benchmark: generate 10,000 overlapping primitive pairs. Measure narrowphase
  time. Assert completion within 2 ms.

---

## US-4.2.2.1 Set Up Colliders for Narrowphase

**As a** game developer (P-15), **I want to** attach `Collider` components to entities and have the
narrowphase system automatically generate `ContactManifold` components for overlapping pairs,
**so that** contact data is available for resolution without manual wiring.

## US-4.2.2.2 Verify Contact Point Accuracy

**As an** engine tester (P-27), **I want to** position known primitive pairs at analytically
computable overlaps and assert penetration depth accuracy within 1mm, **so that** narrowphase
geometric precision is confirmed.

## US-4.2.2.3 Verify Deterministic Contact Generation

**As an** engine tester (P-27), **I want to** run the same narrowphase scenario twice and assert
bit-identical contact manifolds, **so that** narrowphase determinism is verified for
server-authoritative simulation.

## US-4.2.2.4 Benchmark Narrowphase Throughput

**As an** engine tester (P-27), **I want to** process 10000 primitive-vs-primitive candidate pairs
and assert completion within 2ms, **so that** narrowphase throughput meets the performance
requirement.

## US-4.2.2.5 Implement GJK/EPA/SAT Contact Generation

**As an** engine developer (P-26), **I want to** implement the `NarrowphaseSystem` using GJK for
distance, EPA for penetration, and SAT for feature-based contacts, **so that** precise contact
manifolds are generated for all shape combinations.

## US-4.2.2.6 Experience Accurate Collision Responses

**As a** player (P-23), **I want** objects to collide at their visible surfaces without clipping or
ghosting through each other, **so that** interactions look physically correct.

---

## F-4.2.3 Primitive and Convex Collision Shapes

### R-4.2.3 Collider Shape Types

The engine **SHALL** support `Box`, `Sphere`, `Capsule`, and `ConvexHull` shape variants within the
`Collider` ECS component. Convex hulls **SHALL** be generated from meshes at build time with
configurable vertex limits.

- **Derived from:**
  [F-4.2.3](../../features/physics/collision-detection.md)
- **Rationale:** These four primitives cover the vast majority of game object silhouettes;
  build-time hull generation avoids runtime cost.
- **Verification:** Create entities with each of the four shape types. Assert correct contact
  manifolds are generated for all shape-pair combinations.

### R-4.2.3a Fast-Path Primitive Dispatch

The `NarrowphaseSystem` **SHALL** dispatch to specialized routines for common primitive pairs
(sphere-sphere, sphere-box, capsule-box) that bypass generic GJK, achieving at least 2x speedup over
the generic path.

- **Derived from:**
  [F-4.2.3](../../features/physics/collision-detection.md)
- **Rationale:** Primitive-vs-primitive pairs are the most frequent collision type; specialized
  routines avoid the overhead of iterative GJK.
- **Verification:** Benchmark: process 10,000 sphere-vs-sphere pairs via fast path and generic GJK.
  Assert at least 2x speedup for the fast path.

---

## US-4.2.3.1 Assign Primitive Collider Shapes

**As a** game developer (P-15), **I want to** set a `Collider` component to `Box`, `Sphere`,
`Capsule`, or `ConvexHull` shape, **so that** entities have appropriate collision geometry.

## US-4.2.3.2 Configure Convex Hull Vertex Limits

**As a** designer (P-5), **I want to** set the maximum vertex count for convex hull generation in
the build pipeline settings, **so that** hull complexity matches platform budgets.

## US-4.2.3.3 Verify Fast-Path Primitive Dispatch

**As an** engine tester (P-27), **I want to** benchmark 10000 sphere-vs-sphere pairs using the
specialized path versus generic GJK and assert at least 2x speedup, **so that** fast-path dispatch
effectiveness is quantified.

## US-4.2.3.4 Verify All Four Shape Types Generate Correct Contacts

**As an** engine tester (P-27), **I want to** verify that box, sphere, capsule, and convex hull
shapes all produce correct contact manifolds, **so that** all primitive types are validated.

## US-4.2.3.5 Implement Specialized Primitive-vs-Primitive Routines

**As an** engine developer (P-26), **I want to** implement specialized narrowphase routines for
common primitive pairs (sphere-sphere, sphere-box, capsule-box) that bypass generic GJK, **so that**
the most frequent collision pairs are evaluated with minimal cost.

## US-4.2.3.6 Select Collider Shapes in Editor

**As a** level designer (P-6), **I want to** select and preview collider shapes (box, sphere,
capsule) on entities in the editor, **so that** I can visually verify collision geometry matches the
visual mesh.

---

## F-4.2.4 Triangle Mesh and Heightfield Shapes

### R-4.2.4 Concave Shape Support

The engine **SHALL** support `ColliderShape::TriMesh` and `ColliderShape::Heightfield` variants
within the `Collider` component, integrated with the shared BVH for efficient culling.

- **Derived from:**
  [F-4.2.4](../../features/physics/collision-detection.md)
- **Rationale:** Static environment geometry is typically concave; triangle meshes and heightfields
  represent terrain and architecture accurately.
- **Verification:** Assign `TriMesh` to a static entity and `Heightfield` to terrain. Drop rigid
  bodies onto both. Assert correct contact manifolds are generated.

### R-4.2.4a Per-Triangle Material Indices

Triangle mesh and heightfield shapes **SHALL** support per-triangle material indices that map to
`PhysicsMaterial` entries for surface-specific friction and restitution.

- **Derived from:**
  [F-4.2.4](../../features/physics/collision-detection.md)
- **Rationale:** Terrain surfaces vary (dirt, stone, mud); per-triangle materials enable
  surface-specific responses from a single collider.
- **Verification:** Create a heightfield with two regions assigned different physics materials. Drop
  a sphere onto each. Assert correct friction and restitution values.

---

## US-4.2.4.1 Set Up Triangle Mesh Colliders

**As a** game developer (P-15), **I want to** assign `ColliderShape::TriMesh` to static environment
entities, **so that** complex concave geometry has accurate collision.

## US-4.2.4.2 Set Up Heightfield Colliders for Terrain

**As a** game developer (P-15), **I want to** assign `ColliderShape::Heightfield` to terrain
entities, **so that** terrain collision uses the optimized heightfield path.

## US-4.2.4.3 Verify Per-Triangle Material Indices

**As an** engine tester (P-27), **I want to** create a heightfield with two regions assigned
different physics materials, drop a sphere onto each, and assert correct friction and restitution,
**so that** per-triangle material mapping is verified.

## US-4.2.4.4 Benchmark Triangle Mesh Collision at Platform Limits

**As an** engine tester (P-27), **I want to** benchmark collision against a 256K triangle mesh on
desktop and verify acceptable frame time, **so that** mesh collision scales to platform limits.

## US-4.2.4.5 Implement TriMesh and Heightfield Narrowphase

**As an** engine developer (P-26), **I want to** implement narrowphase contact generation for
`TriMesh` and `Heightfield` shapes integrated with the shared BVH, **so that** concave environment
geometry is fully supported.

## US-4.2.4.6 Assign Per-Surface Materials to Terrain

**As a** level designer (P-6), **I want to** paint different physics materials (dirt, stone, mud)
onto heightfield terrain regions in the editor, **so that** surface properties vary across the
landscape.

## US-4.2.4.7 Experience Correct Terrain Collision

**As a** player (P-23), **I want** my character and objects to collide accurately with terrain hills
and valleys, **so that** the ground feels solid and reliable.

---

## F-4.2.5 Compound Shapes

### R-4.2.5 Compound Collider Component

The engine **SHALL** support a `CompoundCollider` component that combines multiple primitive or
convex child shapes with local-space offsets. Each child shape **SHALL** carry its own
`CollisionLayers` and `PhysicsMaterial` data.

- **Derived from:**
  [F-4.2.5](../../features/physics/collision-detection.md)
- **Rationale:** Complex objects need accurate collision without concave decomposition; per-child
  materials enable different surface properties on one entity.
- **Verification:** Create a compound collider with 3 child shapes, each with different layers and
  materials. Fire a ray that hits only one child. Assert the hit reports that child's layer and
  material.

---

## US-4.2.5.1 Create Compound Colliders

**As a** game developer (P-15), **I want to** attach a `CompoundCollider` component combining
multiple primitive shapes with local-space offsets, **so that** complex objects have accurate
collision without concave decomposition.

## US-4.2.5.2 Assign Independent Materials to Compound Children

**As a** game developer (P-15), **I want** each child shape in a `CompoundCollider` to carry its own
`CollisionLayers` and `PhysicsMaterial`, **so that** different parts of the same entity have
different surface properties.

## US-4.2.5.3 Verify Compound Collider Child Filtering

**As an** engine tester (P-27), **I want to** fire a ray that intersects only one child of a
compound collider and assert the hit reports that child's layer and material, **so that** per-child
filtering is correct.

## US-4.2.5.4 Implement Compound Shape Broadphase Integration

**As an** engine developer (P-26), **I want to** implement compound shape support in broadphase and
narrowphase, testing each child shape independently, **so that** compound shapes participate
correctly in collision detection.

## US-4.2.5.5 Build Compound Colliders in Editor

**As a** level designer (P-6), **I want to** visually compose multiple primitive shapes into a
compound collider in the editor, **so that** complex object collision is easy to author.

---

## F-4.2.6 Collision Filtering and Layers

### R-4.2.6 Layer-Based Collision Filtering

The engine **SHALL** filter collisions using a `CollisionLayers` component with membership and mask
bitsets. The `BroadphaseUpdateSystem` **SHALL** evaluate layer compatibility at broadphase to reject
pairs before narrowphase.

- **Derived from:**
  [F-4.2.6](../../features/physics/collision-detection.md)
- **Rationale:** Layer-based filtering is cheap (bitwise AND) and prevents wasted narrowphase
  evaluations on non-interacting entity groups.
- **Verification:** Create two overlapping entities on non-interacting layers. Assert no
  `ContactManifold` is generated. Place them on interacting layers. Assert a `ContactManifold` is
  generated.

### R-4.2.6a Custom Filter Callbacks

The engine **SHALL** support registering a `CollisionFilterCallback` system for fine-grained runtime
filtering (team-based rules, ignore-owner logic) beyond layer-based filtering.

- **Derived from:**
  [F-4.2.6](../../features/physics/collision-detection.md)
- **Rationale:** Gameplay rules like "don't collide with own projectiles" cannot be expressed with
  static layers.
- **Verification:** Register a callback that rejects pairs sharing the same `TeamId` component.
  Create overlapping same-team entities. Assert no contact is generated.

---

## US-4.2.6.1 Configure Collision Layers

**As a** game developer (P-15), **I want to** assign `CollisionLayers` components with membership
and mask bitsets to entities, **so that** I can control which entities collide with each other.

## US-4.2.6.2 Set Up Layer Interaction Matrix in Editor

**As a** designer (P-5), **I want to** configure the collision layer interaction matrix in the
editor, **so that** I can visually define which layers interact without editing code.

## US-4.2.6.3 Register Custom Collision Filter Callbacks

**As a** game developer (P-15), **I want to** register a `CollisionFilterCallback` system for
fine-grained runtime filtering (team-based rules, ignore-owner logic), **so that** advanced
filtering beyond layers is possible.

## US-4.2.6.4 Verify Layer-Based Pair Rejection

**As an** engine tester (P-27), **I want to** create two overlapping entities on non- interacting
layers and assert no `ContactManifold` is generated, **so that** layer filtering prevents unwanted
collisions.

## US-4.2.6.5 Verify Layer-Based Pair Acceptance

**As an** engine tester (P-27), **I want to** create two overlapping entities on interacting layers
and assert a `ContactManifold` is generated, **so that** intended collisions are not filtered out.

## US-4.2.6.6 Implement Layer Evaluation at Broadphase

**As an** engine developer (P-26), **I want to** implement layer compatibility evaluation in the
`BroadphaseUpdateSystem` to reject pairs before narrowphase, **so that** filtered pairs never incur
narrowphase cost.

## US-4.2.6.7 Assign Collision Layers to Level Objects

**As a** level designer (P-6), **I want to** assign collision layers (player, enemy, projectile,
trigger) to entities in the editor, **so that** collision domains are defined visually.

---

## F-4.2.7 Collision Events

### R-4.2.7 Typed Collision Event Lifecycle

The engine **SHALL** emit typed `CollisionStarted`, `CollisionPersisted`, and `CollisionEnded`
events through the ECS event system. Each event **SHALL** carry contact points, normals, impulse
magnitudes, and participating entity IDs.

- **Derived from:**
  [F-4.2.7](../../features/physics/collision-detection.md)
- **Rationale:** Gameplay systems rely on precise collision lifecycle events for damage, sound, and
  VFX triggers.
- **Verification:** Move two spheres into contact for 5 frames then separate. Assert exactly one
  `CollisionStarted`, four `CollisionPersisted`, and one `CollisionEnded` event in correct order.

### R-4.2.7a Zero-Frame Event Delivery

Collision events **SHALL** be available to gameplay systems within the same frame they are detected,
with zero frames of delivery delay.

- **Derived from:**
  [F-4.2.7](../../features/physics/collision-detection.md)
- **Rationale:** Delayed events cause visible desyncs between physics impacts and gameplay feedback.
- **Verification:** Trigger a collision. Assert the event is readable by a system scheduled after
  the collision event system in the same frame.

---

## US-4.2.7.1 Subscribe to Collision Events in Gameplay Systems

**As a** game developer (P-15), **I want to** query `CollisionStarted`, `CollisionPersisted`, and
`CollisionEnded` events in gameplay systems, **so that** I can trigger damage, sound, and VFX on
collision.

## US-4.2.7.2 Verify Collision Event Lifecycle

**As an** engine tester (P-27), **I want to** move two spheres into contact for 5 frames then
separate and assert exactly one `CollisionStarted`, four `CollisionPersisted`, and one
`CollisionEnded` event in the correct order, **so that** event lifecycle semantics are verified.

## US-4.2.7.3 Verify Same-Frame Event Delivery

**As an** engine tester (P-27), **I want to** trigger a collision and assert the event is readable
by a system scheduled after the collision event system in the same frame, **so that** zero-frame
delivery latency is confirmed.

## US-4.2.7.4 Implement Collision Event System

**As an** engine developer (P-26), **I want to** implement the `CollisionEventSystem` that compares
current `ContactManifold` components against the previous frame's `ActiveCollisions` resource and
emits typed events, **so that** gameplay systems receive precise collision lifecycle notifications.

## US-4.2.7.5 Experience Impact Feedback on Collision

**As a** player (P-23), **I want** visual and audio effects to trigger immediately when objects
collide, **so that** impacts feel responsive and impactful.

## US-4.2.7.6 Set Up Collision-Triggered Effects in Editor

**As a** designer (P-5), **I want to** bind collision events to VFX and sound effects in the visual
editor, **so that** impacts produce appropriate feedback without code.

---

## F-4.2.8 Trigger Volumes

### R-4.2.8 Non-Physical Overlap Detection

The engine **SHALL** support `TriggerVolume` entities that detect overlap without generating contact
responses. The system **SHALL** emit `TriggerEnter`, `TriggerStay`, and `TriggerExit` events via the
observer system.

- **Derived from:**
  [F-4.2.8](../../features/physics/collision-detection.md)
- **Rationale:** Quest regions, loading zones, and AoE hazards need spatial detection without
  physical forces.
- **Verification:** Move a rigid body through a trigger volume. Assert `TriggerEnter`,
  `TriggerStay`, and `TriggerExit` fire correctly. Assert the body's velocity is unaffected.

### R-4.2.8a Trigger Volume Types

The engine **SHALL** support one-shot triggers (fire once then disable), persistent triggers (fire
every frame), and filtered triggers (respond only to entities matching a query filter).

- **Derived from:**
  [F-4.2.8](../../features/physics/collision-detection.md)
- **Rationale:** Different gameplay scenarios require different trigger behaviors; one-shot for
  cutscenes, persistent for hazards, filtered for team-specific zones.
- **Verification:** Configure a one-shot trigger. Move an entity through it twice. Assert only one
  event fires. Configure a filtered trigger. Move matching and non-matching entities through it.
  Assert only matching entities trigger events.

---

## US-4.2.8.1 Set Up Trigger Volumes

**As a** game developer (P-15), **I want to** add a `TriggerVolume` component and a collider shape
to an entity, **so that** it detects overlap without generating contact responses.

## US-4.2.8.2 Configure Trigger Types

**As a** designer (P-5), **I want to** configure triggers as one-shot, persistent, or filtered in
the editor, **so that** trigger behavior matches the gameplay intent.

## US-4.2.8.3 Verify Trigger Event Lifecycle

**As an** engine tester (P-27), **I want to** move a rigid body through a trigger volume and assert
`TriggerEnter`, `TriggerStay`, and `TriggerExit` fire correctly with the body's velocity unaffected,
**so that** trigger volumes are non-physical.

## US-4.2.8.4 Verify One-Shot Trigger Fires Once

**As an** engine tester (P-27), **I want to** verify a one-shot trigger fires once then disables
itself, **so that** one-shot behavior is correct.

## US-4.2.8.5 Place Trigger Volumes for Quest Objectives

**As a** level designer (P-6), **I want to** place trigger volumes in the editor for quest objective
regions, loading zone transitions, and trap activation areas, **so that** gameplay zones are defined
spatially.

## US-4.2.8.6 Place Area-of-Effect Zones

**As a** level designer (P-6), **I want to** place trigger volumes as area-of-effect zones (poison,
fire, healing) that affect entities while they overlap, **so that** environmental hazards are easy
to author.

## US-4.2.8.7 Implement Trigger Volume Detection System

**As an** engine developer (P-26), **I want to** implement the trigger system that emits
`TriggerEnter`, `TriggerStay`, and `TriggerExit` events via the observer system without generating
contact forces, **so that** non-physical overlap detection is available.

## US-4.2.8.8 Experience Entering Areas That Trigger Events

**As a** player (P-23), **I want** walking into quest zones and hazard areas to trigger the
appropriate effects immediately, **so that** the world reacts to my presence.

---

## F-4.2.9 Physics Material Assets

### R-4.2.9 Data-Driven Physics Materials

The engine **SHALL** support `PhysicsMaterial` assets with static friction, dynamic friction,
restitution, density, and surface type tag. Materials **SHALL** be assigned per collider via a
`PhysicsMaterialHandle` component.

- **Derived from:**
  [F-4.2.9](../../features/physics/collision-detection.md)
- **Rationale:** Data-driven materials enable designers to define surface properties without code
  changes.
- **Verification:** Create two materials with different friction. Assign them to two colliders.
  Assert the contact solver uses the correct combined friction.

### R-4.2.9a Surface Type Tags

Physics material surface type tags (metal, wood, ice, rubber) **SHALL** be accessible to downstream
systems for driving audio (footstep sounds) and VFX (impact particles).

- **Derived from:**
  [F-4.2.9](../../features/physics/collision-detection.md)
- **Rationale:** Surface tags unify physics, audio, and VFX so that material properties drive all
  feedback channels.
- **Verification:** Create materials tagged "metal" and "wood". Verify a collision event carries the
  correct surface tags for both colliders.

---

## US-4.2.9.1 Author Physics Materials in Editor

**As a** designer (P-5), **I want to** create physics material assets with friction, restitution,
density, and surface type tag in the visual editor, **so that** surface properties are data-driven.

## US-4.2.9.2 Assign Physics Materials to Colliders

**As a** game developer (P-15), **I want to** assign a `PhysicsMaterialHandle` component to any
collider entity, **so that** surface properties are associated with collision geometry.

## US-4.2.9.3 Verify Material Combination Accuracy

**As an** engine tester (P-27), **I want to** test each combine mode (average, min, max, multiply)
with known friction and restitution values and assert combined values match expected results within
floating-point epsilon, **so that** material combination is mathematically correct.

## US-4.2.9.4 Verify Surface Tags Drive Audio and VFX

**As an** engine tester (P-27), **I want to** verify that surface type tags (metal, wood, ice) drive
footstep sound selection and impact particle selection, **so that** material tags are consumed by
downstream systems.

## US-4.2.9.5 Implement Material Combination Logic

**As an** engine developer (P-26), **I want to** implement the material combination system that
resolves effective friction and restitution from the `PhysicsMaterial` of both colliders using the
configured combine mode, **so that** surface interactions are resolved automatically.

## US-4.2.9.6 Experience Different Surface Feels

**As a** player (P-23), **I want** ice surfaces to be slippery, rubber to be bouncy, and metal to
clang, **so that** each material feels physically distinct.

## US-4.2.9.7 Paint Surface Types on Level Geometry

**As a** level designer (P-6), **I want to** assign physics materials to different surfaces in the
level, **so that** the entire environment has correct physical properties.

---

## Non-Functional Requirements

### R-4.2.NF1 Broadphase Throughput

Broadphase pair culling **SHALL** process at least 50,000 AABB-bearing entities within 1 ms on
minimum-spec hardware, including shared BVH traversal and collision layer filtering.

- **Derived from:** R-4.2.1, R-4.2.6
- **Rationale:** Broadphase is the first stage of every physics tick; its cost scales with total
  entity count and must remain a small fraction of the total physics budget.
- **Verification:** Benchmark -- populate 50,000 entities with random AABB extents; measure
  broadphase query time; assert it completes within 1 ms.

### R-4.2.NF2 Narrowphase Throughput

Narrowphase contact generation **SHALL** process at least 10,000 candidate pairs within 2 ms on
minimum-spec hardware for primitive-vs-primitive pair types.

- **Derived from:** R-4.2.2, R-4.2.3
- **Rationale:** Narrowphase dominates collision detection cost; it must handle peak contact loads
  during explosions, collapses, and dense combat without exceeding the physics budget.
- **Verification:** Benchmark -- generate 10,000 overlapping primitive pairs; measure narrowphase
  contact generation time; assert it completes within 2 ms.

### R-4.2.NF3 Collision Event Latency

Collision events (`CollisionStarted`, `CollisionPersisted`, `CollisionEnded`) **SHALL** be available
to gameplay systems within the same frame they are detected, with zero frames of event delivery
delay.

- **Derived from:** R-4.2.7, R-4.2.8
- **Rationale:** Gameplay systems (damage application, sound triggers, VFX spawning) rely on
  same-frame event delivery to avoid visible desyncs between physics and effects.
- **Verification:** Integration test -- trigger a collision; assert the event is readable by a
  system scheduled after the collision event system in the same frame.
