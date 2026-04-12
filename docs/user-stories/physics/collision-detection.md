# User Stories -- 4.2 Collision Detection

## Broadphase

| ID         | Persona                 |
|------------|-------------------------|
| US-4.2.1.1 | game developer (P-15)   |
| US-4.2.1.2 | engine developer (P-26) |
| US-4.2.1.3 | engine tester (P-27)    |

1. **US-4.2.1.1** -- **As a** game developer (P-15), **I want** broadphase to query the shared BVH
   filtered by CollisionLayers, **so that** physics reuses the spatial index without a redundant
   structure.
2. **US-4.2.1.2** -- **As a** engine developer (P-26), **I want** candidate pairs written to a
   BroadphasePairs ECS resource, **so that** narrowphase consumes them without allocation.
3. **US-4.2.1.3** -- **As a** engine tester (P-27), **I want** to verify broadphase pair counts
   respect per-platform caps, **so that** frame budgets hold on constrained devices.

## Narrowphase

| ID         | Persona                 |
|------------|-------------------------|
| US-4.2.2.1 | engine developer (P-26) |
| US-4.2.2.2 | engine tester (P-27)    |
| US-4.2.2.3 | game developer (P-15)   |

1. **US-4.2.2.1** -- **As a** engine developer (P-26), **I want** GJK, EPA, and SAT algorithms
   producing ContactManifold components, **so that** contacts have precise points, normals, and
   penetration depths.
2. **US-4.2.2.2** -- **As a** engine tester (P-27), **I want** to verify contact generation is
   bit-identical across platforms, **so that** deterministic simulation is maintained.
3. **US-4.2.2.3** -- **As a** game developer (P-15), **I want** to query ContactManifold components
   from gameplay systems, **so that** I can react to collision contact details.

## Collision Shapes

| ID         | Persona                 |
|------------|-------------------------|
| US-4.2.3.1 | game developer (P-15)   |
| US-4.2.3.2 | engine developer (P-26) |
| US-4.2.3.3 | game designer (P-5)     |
| US-4.2.4.1 | game developer (P-15)   |
| US-4.2.4.2 | engine developer (P-26) |
| US-4.2.5.1 | game developer (P-15)   |
| US-4.2.5.2 | game designer (P-5)     |

1. **US-4.2.3.1** -- **As a** game developer (P-15), **I want** box, sphere, capsule, and convex
   hull shapes via a Collider component, **so that** common object silhouettes have efficient
   collision.
2. **US-4.2.3.2** -- **As a** engine developer (P-26), **I want** specialized fast paths for
   primitive pairs, **so that** common collisions bypass generic GJK.
3. **US-4.2.3.3** -- **As a** game designer (P-5), **I want** to assign collision shapes in the
   visual editor, **so that** I configure colliders without code.
4. **US-4.2.4.1** -- **As a** game developer (P-15), **I want** triangle mesh and heightfield
   collider shapes, **so that** static environments and terrain have accurate collision.
5. **US-4.2.4.2** -- **As a** engine developer (P-26), **I want** per-triangle material indices on
   mesh colliders, **so that** different surfaces have distinct friction and effects.
6. **US-4.2.5.1** -- **As a** game developer (P-15), **I want** compound colliders combining
   multiple primitives, **so that** complex objects collide accurately without concave
   decomposition.
7. **US-4.2.5.2** -- **As a** game designer (P-5), **I want** each child shape to carry its own
   CollisionLayers and PhysicsMaterial, **so that** parts filter and respond independently.

## Filtering and Events

| ID         | Persona                 |
|------------|-------------------------|
| US-4.2.6.1 | game developer (P-15)   |
| US-4.2.6.2 | game designer (P-5)     |
| US-4.2.6.3 | engine tester (P-27)    |
| US-4.2.7.1 | game developer (P-15)   |
| US-4.2.7.2 | game developer (P-15)   |
| US-4.2.7.3 | engine developer (P-26) |

1. **US-4.2.6.1** -- **As a** game developer (P-15), **I want** layer membership and collision mask
   via CollisionLayers components, **so that** entities only collide with relevant layers.
2. **US-4.2.6.2** -- **As a** game designer (P-5), **I want** to configure collision layers in the
   visual editor, **so that** layer setup is code-free.
3. **US-4.2.6.3** -- **As a** engine tester (P-27), **I want** to verify all layer combinations
   produce correct filtering, **so that** no unintended collisions occur.
4. **US-4.2.7.1** -- **As a** game developer (P-15), **I want** CollisionStarted,
   CollisionPersisted, and CollisionEnded events with contact data, **so that** gameplay systems
   react to collision lifecycle.
5. **US-4.2.7.2** -- **As a** game developer (P-15), **I want** collision events to carry entity
   IDs, contact points, normals, and impulse magnitudes, **so that** I spawn VFX at impact
   locations.
6. **US-4.2.7.3** -- **As a** engine developer (P-26), **I want** events batched per frame and
   available the same frame they are detected, **so that** gameplay feedback has zero delivery
   delay.

## Triggers and Materials

| ID         | Persona                 |
|------------|-------------------------|
| US-4.2.8.1 | game developer (P-15)   |
| US-4.2.8.2 | game designer (P-5)     |
| US-4.2.8.3 | engine tester (P-27)    |
| US-4.2.9.1 | game designer (P-5)     |
| US-4.2.9.2 | game developer (P-15)   |
| US-4.2.9.3 | engine tester (P-27)    |

1. **US-4.2.8.1** -- **As a** game developer (P-15), **I want** trigger volumes that detect overlap
   without contact response, **so that** area-of-effect zones and quest regions work without
   physical forces.
2. **US-4.2.8.2** -- **As a** game designer (P-5), **I want** one-shot, persistent, and filtered
   trigger modes, **so that** I configure trigger behavior per gameplay need.
3. **US-4.2.8.3** -- **As a** engine tester (P-27), **I want** to verify TriggerEnter, TriggerStay,
   and TriggerExit fire correctly, **so that** trigger lifecycle is reliable.
4. **US-4.2.9.1** -- **As a** game designer (P-5), **I want** PhysicsMaterial assets with friction,
   restitution, density, and surface type tags, **so that** surface properties are reusable and
   data-driven.
5. **US-4.2.9.2** -- **As a** game developer (P-15), **I want** surface type tags to drive audio
   footsteps and impact VFX, **so that** materials produce matching sensory feedback.
6. **US-4.2.9.3** -- **As a** engine tester (P-27), **I want** to verify material combination is
   symmetric, **so that** collision order does not affect behavior.

## Voxel Terrain and Convex Decomposition

| ID            | Persona                 |
|---------------|-------------------------|
| US-4.2.10.1   | game developer (P-15)   |
| US-4.2.10.2   | engine developer (P-26) |
| US-4.2.11.1   | technical artist (P-13) |
| US-4.2.12.1   | engine developer (P-26) |

1. **US-4.2.10.1** -- **As a** game developer (P-15), **I want** modified voxel chunks to rebuild
   their trimesh collider on a job worker and atomically swap at the next FixedUpdate, **so that**
   players digging or building see updated physics without frame hitches.
2. **US-4.2.10.2** -- **As an** engine developer (P-26), **I want** a per-frame chunk rebuild budget
   enforced by platform tier, **so that** heavy voxel editing never exceeds the physics frame
   budget.
3. **US-4.2.11.1** -- **As a** technical artist (P-13), **I want** non-convex source meshes
   automatically decomposed into compound convex hulls at import time via V-HACD, **so that**
   complex props have accurate collision without manual decomposition in a DCC tool.
4. **US-4.2.12.1** -- **As an** engine developer (P-26), **I want** runtime quickhull to generate a
   single convex hull from an arbitrary vertex set, **so that** destruction fragments and procedural
   geometry gain collision without an offline bake step.

## Collision Layer Interaction Matrix

| ID           | Persona                 |
|--------------|-------------------------|
| US-4.2.13.1  | game designer (P-5)     |
| US-4.2.13.2  | game developer (P-15)   |

1. **US-4.2.13.1** -- **As a** game designer (P-5), **I want** an editor-configurable 32x32
   collision layer interaction matrix, **so that** I can toggle which layer pairs interact without
   writing filter code.
2. **US-4.2.13.2** -- **As a** game developer (P-15), **I want** matrix entries that select overlap
   only without contact response, **so that** hitbox and hurtbox combat layers report events without
   generating collision impulses.
