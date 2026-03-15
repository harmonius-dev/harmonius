# User Stories — 4.2 Collision Detection

## US-4.2.1.1 Query Shared BVH for Broadphase Pairs
**As a** game developer (P-15), **I want** physics to query the shared BVH for collision pairs,
**so that** no redundant spatial structure is maintained.

## US-4.2.1.2 Filter Broadphase by CollisionLayers
**As a** game developer (P-15), **I want** broadphase queries to filter by CollisionLayers
components, **so that** irrelevant pairs are rejected early.

## US-4.2.1.3 Implement BroadphaseQuerySystem
**As an** engine developer (P-26), **I want to** implement a BroadphaseQuerySystem that reads
the shared BVH and writes candidate pairs to BroadphasePairs, **so that** narrowphase has
valid input.

## US-4.2.1.4 Verify Broadphase Pair Budget on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile caps at 2048 broadphase pairs
per frame, **so that** the frame budget is respected.

## US-4.2.1.5 Experience Accurate Collision in Dense Scenes
**As a** player (P-23), **I want** collisions to detect correctly even in dense battle scenes,
**so that** objects and characters interact properly.

## US-4.2.1.6 Share BVH with Rendering and Network Systems
**As an** engine developer (P-26), **I want** physics to share the spatial index with rendering
culling, network AOI, and AI perception, **so that** no redundant structures exist.

## US-4.2.1.7 Configure Distance Culling Radius Per Platform
**As a** designer (P-5), **I want to** configure distance culling radius per platform tier,
**so that** broadphase pair count stays within budget on constrained devices.

## US-4.2.1.8 Test Broadphase Under Maximum Entity Count
**As an** engine tester (P-27), **I want to** stress test broadphase with the maximum entity
count per platform, **so that** performance degrades gracefully.

## US-4.2.1.9 Write Candidate Pairs to BroadphasePairs Resource
**As an** engine developer (P-26), **I want** candidate pairs written to a BroadphasePairs
ECS resource, **so that** narrowphase systems consume them without allocation.

## US-4.2.1.10 Configure Pair Budget Per Platform
**As a** designer (P-5), **I want to** set maximum broadphase pair counts per platform tier,
**so that** each device has appropriate collision detection budgets.

## US-4.2.1.11 Verify AABB Overlap Correctness
**As an** engine tester (P-27), **I want to** test AABB overlap queries against known scenarios,
**so that** broadphase produces correct candidate pairs.

## US-4.2.1.12 Scale Pair Budget on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to support 128K broadphase pairs,
**so that** large-scale battles have full collision fidelity.

## US-4.2.2.1 Generate Contact Points with GJK/EPA
**As an** engine developer (P-26), **I want to** generate contact points and penetration depths
using GJK and EPA algorithms, **so that** narrow-phase contacts are precise.

## US-4.2.2.2 Produce ContactManifold Components
**As an** engine developer (P-26), **I want to** produce ContactManifold components on collision
pair entities, **so that** the solver has contact data to resolve.

## US-4.2.2.3 Ensure Deterministic Contact Generation
**As an** engine tester (P-27), **I want to** verify contact generation produces identical
results across platforms, **so that** server-authoritative simulation is consistent.

## US-4.2.2.4 Experience Correct Object Interactions
**As a** player (P-23), **I want** objects to interact precisely at contact surfaces, **so that**
stacking, sliding, and bouncing look correct.

## US-4.2.2.5 Implement SAT for Feature-Based Contacts
**As an** engine developer (P-26), **I want to** implement SAT for feature-based contact
generation, **so that** box-box contacts produce edge contacts efficiently.

## US-4.2.2.6 Verify EPA Iteration Limits on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile caps EPA at 16 iterations,
**so that** narrowphase stays within 2ms target.

## US-4.2.2.7 Configure Contact Points Per Platform
**As a** designer (P-5), **I want** mobile to cap at 4 contact points per manifold, **so that**
solver cost is controlled on constrained devices.

## US-4.2.2.8 Test Edge-Edge and Face-Face Contacts
**As an** engine tester (P-27), **I want to** test edge-edge and face-face contact scenarios,
**so that** all contact types generate correct manifolds.

## US-4.2.2.9 Read Contact Data from Gameplay Systems
**As a** game developer (P-15), **I want to** query ContactManifold components from gameplay
systems, **so that** I can react to collision contact details.

## US-4.2.2.10 Support 8 Contact Points on Desktop
**As an** engine developer (P-26), **I want** desktop to support up to 8 contact points per
manifold, **so that** large flat contacts resolve correctly.

## US-4.2.2.11 Dispatch to Fast-Path Routines for Primitives
**As an** engine developer (P-26), **I want** narrowphase to dispatch to specialized fast-path
routines for primitive-vs-primitive pairs, **so that** common cases are optimized.

## US-4.2.2.12 Verify Penetration Depth Accuracy
**As an** engine tester (P-27), **I want to** compare penetration depths against analytic
solutions for known shapes, **so that** EPA accuracy is validated.

## US-4.2.3.1 Create Box Colliders on Entities
**As a** game developer (P-15), **I want to** add a Collider component with a Box shape to
entities, **so that** they participate in collision detection.

## US-4.2.3.2 Create Sphere Colliders on Entities
**As a** game developer (P-15), **I want to** add Sphere colliders, **so that** round objects
have efficient collision shapes.

## US-4.2.3.3 Create Capsule Colliders on Entities
**As a** game developer (P-15), **I want to** add Capsule colliders, **so that** characters
and cylindrical objects have appropriate shapes.

## US-4.2.3.4 Generate Convex Hulls at Build Time
**As an** engine developer (P-26), **I want to** generate convex hull colliders from meshes at
build time with configurable vertex limits, **so that** arbitrary shapes are supported.

## US-4.2.3.5 Verify Convex Hull Vertex Limits Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile convex hulls cap at 16 vertices,
**so that** performance budgets are respected.

## US-4.2.3.6 Experience Accurate Object Shapes
**As a** player (P-23), **I want** objects to collide matching their visible shape, **so that**
interactions feel physically correct.

## US-4.2.3.7 Assign Primitive Shapes in Visual Editor
**As a** designer (P-5), **I want to** assign box, sphere, and capsule shapes in the visual
editor, **so that** I can set up colliders without code.

## US-4.2.3.8 Dispatch Primitive Fast Paths in Narrowphase
**As an** engine developer (P-26), **I want** primitive-vs-primitive pairs to use specialized
fast paths, **so that** common collisions are cheaper than generic GJK.

## US-4.2.3.9 Test Convex Hull vs Primitive Contacts
**As an** engine tester (P-27), **I want to** test convex hull vs each primitive type, **so
that** all shape pair combinations generate correct contacts.

## US-4.2.3.10 Use Capsule Colliders for Characters
**As a** level designer (P-6), **I want to** use capsule colliders for characters, **so that**
they slide along walls and fit through doorways naturally.

## US-4.2.3.11 Store ColliderShape as Enum Variant
**As an** engine developer (P-26), **I want** ColliderShape to be an enum with Box, Sphere,
Capsule, and ConvexHull variants, **so that** shape type is a compile-time dispatch.

## US-4.2.3.12 Test Sphere-Sphere and Box-Box Fast Paths
**As an** engine tester (P-27), **I want to** profile primitive fast paths vs generic GJK,
**so that** I can verify the performance benefit.

## US-4.2.4.1 Add Triangle Mesh Colliders
**As a** game developer (P-15), **I want to** add TriMesh colliders to terrain and static
geometry, **so that** complex environments have accurate collision.

## US-4.2.4.2 Add Heightfield Colliders for Terrain
**As a** level designer (P-6), **I want to** use Heightfield colliders for terrain, **so that**
landscapes have efficient collision without full triangle meshes.

## US-4.2.4.3 Support Per-Triangle Material Indices
**As a** game developer (P-15), **I want** triangle meshes to support per-triangle material
indices, **so that** different surfaces have distinct friction and effects.

## US-4.2.4.4 Implement BVH for TriMesh Queries
**As an** engine developer (P-26), **I want** triangle meshes to integrate with BVH for
efficient culling, **so that** large meshes do not bottleneck broadphase.

## US-4.2.4.5 Verify TriMesh Limits on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile caps trimesh at 8K triangles
per collider, **so that** memory and CPU budgets are respected.

## US-4.2.4.6 Walk on Terrain Without Falling Through
**As a** player (P-23), **I want** terrain collision to be accurate, **so that** I never fall
through the ground.

## US-4.2.4.7 Load Coarser LOD Colliders on Mobile
**As an** engine developer (P-26), **I want** constrained platforms to load coarser collision
LODs, **so that** trimesh and heightfield complexity scales with capability.

## US-4.2.4.8 Configure Heightfield Resolution Per Platform
**As a** designer (P-5), **I want to** set heightfield resolution per platform tier, **so that**
terrain collision detail matches device capability.

## US-4.2.4.9 Test Heightfield Collision at Boundaries
**As an** engine tester (P-27), **I want to** test collision at heightfield tile boundaries,
**so that** seams between terrain tiles do not create gaps.

## US-4.2.4.10 Design Terrain with Caves Using TriMesh
**As a** level designer (P-6), **I want to** use triangle mesh colliders for caves and
overhangs, **so that** non-heightfield terrain has accurate collision.

## US-4.2.4.11 Support 256K Triangles on Desktop
**As an** engine developer (P-26), **I want** desktop to support trimesh colliders up to 256K
triangles, **so that** detailed static geometry has full-fidelity collision.

## US-4.2.4.12 Verify Heightfield Normal Accuracy
**As an** engine tester (P-27), **I want to** verify that heightfield contacts produce correct
surface normals, **so that** objects resting on terrain align properly.

## US-4.2.5.1 Combine Multiple Shapes into Compound Collider
**As a** game developer (P-15), **I want to** create CompoundCollider components combining
multiple primitive shapes, **so that** complex objects have accurate collision.

## US-4.2.5.2 Assign Per-Child Collision Layers
**As a** game developer (P-15), **I want** each child shape in a compound to carry its own
CollisionLayers, **so that** parts can be filtered independently.

## US-4.2.5.3 Assign Per-Child Physics Materials
**As a** designer (P-5), **I want** each child shape to carry its own PhysicsMaterial, **so
that** different parts of an object have distinct surface properties.

## US-4.2.5.4 Implement CompoundCollider Broadphase Integration
**As an** engine developer (P-26), **I want** CompoundCollider to integrate with broadphase
using a single entity AABB encompassing all children, **so that** overlap tests are efficient.

## US-4.2.5.5 Verify Child Shape Limits on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile caps at 4 child shapes per
compound, **so that** broadphase cost stays controlled.

## US-4.2.5.6 Experience Accurate Collision on Complex Objects
**As a** player (P-23), **I want** complex-shaped objects to collide accurately, **so that**
furniture, vehicles, and structures interact correctly.

## US-4.2.5.7 Author Compound Shapes in Visual Editor
**As a** designer (P-5), **I want to** compose compound colliders from primitives in the
visual editor, **so that** I can build complex shapes without code.

## US-4.2.5.8 Store Child Offsets in Local Space
**As an** engine developer (P-26), **I want** child shapes stored with local-space offsets,
**so that** the compound collider transforms with the parent entity.

## US-4.2.5.9 Test Compound vs Compound Contacts
**As an** engine tester (P-27), **I want to** test compound collider vs compound collider
interactions, **so that** multi-shape pairs generate correct contacts.

## US-4.2.5.10 Build Vehicle Collider from Multiple Shapes
**As a** level designer (P-6), **I want to** build vehicle collision from box and capsule
shapes, **so that** the vehicle silhouette matches visuals.

## US-4.2.5.11 Support 64 Children on High-End PC
**As an** engine developer (P-26), **I want** high-end PC to support up to 64 children per
compound, **so that** detailed objects have full collision fidelity.

## US-4.2.5.12 Profile Compound Collider Performance
**As an** engine tester (P-27), **I want to** profile compound colliders with increasing child
counts, **so that** I can validate per-platform limits.

## US-4.2.6.1 Set Collision Layer Membership on Entities
**As a** game developer (P-15), **I want to** set layer membership and collision mask via
CollisionLayers, **so that** entities only collide with relevant layers.

## US-4.2.6.2 Reject Non-Matching Pairs at Broadphase
**As an** engine developer (P-26), **I want** the BroadphaseUpdateSystem to reject pairs that
fail layer compatibility, **so that** narrowphase never processes irrelevant pairs.

## US-4.2.6.3 Register Custom Collision Filter Callbacks
**As a** game developer (P-15), **I want to** register a CollisionFilterCallback system for
fine-grained filtering, **so that** team-based or owner-ignore rules work at runtime.

## US-4.2.6.4 Separate Player and Environment Layers
**As a** designer (P-5), **I want to** configure collision layers for players, NPCs,
projectiles, and environment, **so that** interactions are controlled visually.

## US-4.2.6.5 Verify Layer Filtering Correctness
**As an** engine tester (P-27), **I want to** test all layer combinations and confirm correct
filtering, **so that** no unintended collisions occur.

## US-4.2.6.6 Walk Through Friendly Projectiles
**As a** player (P-23), **I want** friendly projectiles to pass through me, **so that**
teammates do not block my movement.

## US-4.2.6.7 Assign Layers in Visual Editor
**As a** designer (P-5), **I want to** assign collision layers and masks in the visual editor,
**so that** layer configuration is code-free.

## US-4.2.6.8 Implement Layer Bitmask Operations
**As an** engine developer (P-26), **I want** layer filtering to use bitwise AND on membership
and mask bitmasks, **so that** evaluation is O(1).

## US-4.2.6.9 Test Ignore-Owner Filtering
**As an** engine tester (P-27), **I want to** verify that projectiles ignore their owner
entity via custom filter callbacks, **so that** self-hits are prevented.

## US-4.2.6.10 Configure Layers Per Game Mode
**As a** game developer (P-15), **I want to** reconfigure collision layers at runtime for
different game modes, **so that** PvP and PvE have different collision rules.

## US-4.2.6.11 Verify Layer Filtering Performance
**As an** engine tester (P-27), **I want to** profile layer filtering cost, **so that** it
remains negligible relative to narrowphase.

## US-4.2.6.12 Not Collide with Decorative Objects
**As a** player (P-23), **I want** decorative objects on a non-colliding layer to not block
my path, **so that** navigation is smooth.

## US-4.2.7.1 Receive CollisionStarted Events
**As a** game developer (P-15), **I want to** receive CollisionStarted events when two entities
first contact, **so that** I can trigger gameplay effects on impact.

## US-4.2.7.2 Receive CollisionPersisted Events
**As a** game developer (P-15), **I want to** receive CollisionPersisted events each frame
while contact continues, **so that** sustained contact effects work.

## US-4.2.7.3 Receive CollisionEnded Events
**As a** game developer (P-15), **I want to** receive CollisionEnded events when contact
breaks, **so that** I can clean up effects or trigger separation logic.

## US-4.2.7.4 Implement CollisionEventSystem
**As an** engine developer (P-26), **I want to** implement a CollisionEventSystem comparing
current ContactManifolds against previous-frame ActiveCollisions, **so that** event types
are determined correctly.

## US-4.2.7.5 Read Contact Points from Collision Events
**As a** game developer (P-15), **I want** collision events to carry contact points, normals,
and impulse magnitudes, **so that** I can spawn VFX at impact locations.

## US-4.2.7.6 Hear Impact Sounds on Collision
**As a** player (P-23), **I want to** hear appropriate impact sounds when objects collide,
**so that** the world feels responsive.

## US-4.2.7.7 Batch Events Per Frame
**As an** engine developer (P-26), **I want** collision events batched per frame, **so that**
overhead is reduced in scenes with many simultaneous contacts.

## US-4.2.7.8 Verify Event Ordering Consistency
**As an** engine tester (P-27), **I want to** verify that CollisionStarted always fires before
CollisionPersisted, **so that** event ordering is predictable.

## US-4.2.7.9 Trigger VFX from Collision Event Data
**As a** designer (P-5), **I want** collision events to carry enough data to trigger sparks,
dust, or splash VFX at the impact point, **so that** I can configure effects visually.

## US-4.2.7.10 Verify No Duplicate Events Per Frame
**As an** engine tester (P-27), **I want to** confirm that each collision pair emits exactly
one event type per frame, **so that** gameplay systems are not double-triggered.

## US-4.2.7.11 Access Participating Entity IDs in Events
**As a** game developer (P-15), **I want** collision events to contain both entity IDs, **so
that** I can identify which objects collided.

## US-4.2.7.12 See Visual Feedback on Impact
**As a** player (P-23), **I want** objects to show visual reactions (sparks, cracks, dust)
when they collide, **so that** impacts feel impactful.

## US-4.2.8.1 Create Trigger Volume Entities
**As a** game developer (P-15), **I want to** create entities with TriggerVolume and Collider
components, **so that** non-physical overlap zones detect entity presence.

## US-4.2.8.2 Receive TriggerEnter Events via Observers
**As a** game developer (P-15), **I want to** receive TriggerEnter events via observers when
entities enter a trigger volume, **so that** I can activate gameplay logic on entry.

## US-4.2.8.3 Receive TriggerExit Events
**As a** game developer (P-15), **I want to** receive TriggerExit events when entities leave
a trigger volume, **so that** I can deactivate effects on exit.

## US-4.2.8.4 Implement One-Shot Triggers
**As a** game developer (P-15), **I want** one-shot triggers that fire once then disable,
**so that** single-use traps and quest objectives work correctly.

## US-4.2.8.5 Implement Persistent Triggers
**As an** engine developer (P-26), **I want** persistent triggers that fire every frame while
overlapping, **so that** area-of-effect damage zones work continuously.

## US-4.2.8.6 Implement Filtered Triggers
**As a** game developer (P-15), **I want** filtered triggers that only respond to entities
matching a query filter, **so that** triggers can target specific entity types.

## US-4.2.8.7 Enter Area and Trigger Quest Objective
**As a** player (P-23), **I want** entering an area to trigger a quest objective, **so that**
exploration-based quests work intuitively.

## US-4.2.8.8 Design Loading Zone Transitions
**As a** level designer (P-6), **I want** trigger volumes at zone boundaries, **so that**
entering them initiates level transitions.

## US-4.2.8.9 Verify Trigger Budget on Mobile
**As an** engine tester (P-27), **I want to** confirm mobile caps at 64 active triggers with
simple shapes, **so that** performance is maintained.

## US-4.2.8.10 Use TriggerStay for Continuous Effects
**As a** game developer (P-15), **I want** TriggerStay events each frame while an entity
remains in the volume, **so that** healing zones apply effects continuously.

## US-4.2.8.11 Design Area-of-Effect Zones
**As a** level designer (P-6), **I want** trigger volumes to define damage, healing, and buff
zones, **so that** spatial gameplay effects are easy to author.

## US-4.2.8.12 Verify Throttled Triggers on Mobile
**As an** engine tester (P-27), **I want to** confirm persistent triggers run every-other-frame
on mobile, **so that** overlap test cost is halved.

## US-4.2.9.1 Create Physics Material Assets
**As a** designer (P-5), **I want to** create PhysicsMaterial assets with friction, restitution,
and density values, **so that** surface properties are reusable across objects.

## US-4.2.9.2 Assign Materials Per Collider
**As a** game developer (P-15), **I want to** assign PhysicsMaterialHandle components to
colliders, **so that** each surface has specific physical properties.

## US-4.2.9.3 Configure Material Combine Modes
**As a** designer (P-5), **I want to** set combine modes (average, min, max, multiply) for
friction and restitution, **so that** material pair interactions are predictable.

## US-4.2.9.4 Tag Materials with Surface Types
**As a** designer (P-5), **I want to** tag materials with surface type (metal, wood, ice,
rubber), **so that** audio and VFX systems produce matching effects.

## US-4.2.9.5 Drive Footstep Sounds from Surface Tags
**As an** engine developer (P-26), **I want** surface type tags to drive audio footstep sounds,
**so that** walking on wood sounds different from metal.

## US-4.2.9.6 Hear Surface-Specific Footstep Sounds
**As a** player (P-23), **I want** footstep sounds to change based on the surface I walk on,
**so that** different materials feel distinct.

## US-4.2.9.7 Author Materials in Visual Editor
**As a** designer (P-5), **I want to** author physics materials in the visual editor,
**so that** surface properties are tweaked without code.

## US-4.2.9.8 Implement Material Asset Loading
**As an** engine developer (P-26), **I want** physics materials loaded from binary asset
format, **so that** materials integrate with the content pipeline.

## US-4.2.9.9 Verify Material Combine Symmetry
**As an** engine tester (P-27), **I want to** verify that material combination is symmetric,
**so that** collision order does not affect surface behavior.

## US-4.2.9.10 Trigger Impact Particles from Surface Tags
**As a** game developer (P-15), **I want** impact VFX to vary based on surface type tags,
**so that** hitting metal produces sparks and wood produces splinters.

## US-4.2.9.11 See Surface-Appropriate Impact Effects
**As a** player (P-23), **I want** impacts on different surfaces to produce matching visual
and audio effects, **so that** the world feels rich and responsive.

## US-4.2.9.12 Test All Surface Type Combinations
**As an** engine tester (P-27), **I want to** test collisions between all surface type
combinations, **so that** every pair produces correct results and effects.
