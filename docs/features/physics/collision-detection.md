# 4.2 — Collision Detection

## Broadphase

| ID      | Feature |
|---------|------------------------------------ |
| F-4.2.1 | Broadphase Acceleration Structures |

1. **F-4.2.1** — Cull collision pairs using the shared spatial index (F-1.9.1) rather than a
   physics-private acceleration structure. The `BroadphaseQuerySystem` reads the shared BVH ECS
   resource and queries overlapping AABB pairs, filtered by `CollisionLayers` components. Candidate
   pairs are written to a `BroadphasePairs` ECS resource for consumption by narrowphase. Because the
   BVH is shared with rendering culling, network AOI, and AI perception, physics avoids rebuilding a
   redundant spatial structure.
   - **Deps:** F-1.9.1 (Shared BVH), F-1.9.6 (Physics Integration), F-1.1.1
   - **Platform:** Mobile: max 2048 broadphase pairs per frame. Switch: max 4096 pairs. Desktop: max
     32K pairs. High-end PC: max 128K pairs. Pair budget enforced by distance culling radius that
     shrinks on constrained platforms.

## Narrowphase

| ID      | Feature |
|---------|-------------------------------- |
| F-4.2.2 | Narrowphase Contact Generation |

1. **F-4.2.2** — Generate precise contact points and penetration depths using GJK (distance), EPA
   (penetration), and SAT (feature-based contacts) algorithms. The `NarrowphaseSystem` reads
   candidate pairs from the `BroadphasePairs` resource and `Collider` components, then produces
   `ContactManifold` components on collision pair entities containing contact points, normals, and
   penetration depths. Contact generation is deterministic across platforms for server-authoritative
   simulation.
   - **Deps:** F-4.2.1, F-1.1.2
   - **Platform:** Mobile: max 4 contact points per manifold, EPA max 16 iterations. Switch: max 4
     contacts, EPA max 32 iterations. Desktop: max 8 contacts, EPA max 64 iterations. GJK/EPA
     budgets reduced on mobile to stay within 2ms narrowphase target.

## Collision Shapes

| ID      | Feature |
|---------|--------------------------------------- |
| F-4.2.3 | Primitive and Convex Collision Shapes |
| F-4.2.4 | Triangle Mesh and Heightfield Shapes |
| F-4.2.5 | Compound Shapes |

1. **F-4.2.3** — Support box, sphere, capsule, and convex hull shapes via the `Collider` ECS
   component, which holds a `ColliderShape` enum (`Box`, `Sphere`, `Capsule`, `ConvexHull`). Convex
   hulls are generated from arbitrary meshes at build time with configurable vertex limits. The
   `NarrowphaseSystem` dispatches to specialized fast-path routines for primitive-vs-primitive pairs
   rather than falling through to generic GJK.
   - **Deps:** F-4.2.2, F-1.1.2
   - **Platform:** Mobile: convex hulls max 16 vertices, prefer primitives. Switch: convex hulls max
     32 vertices. Desktop: convex hulls max 64 vertices. High-end PC: convex hulls max 256 vertices.
2. **F-4.2.4** — Support concave triangle mesh and heightfield shapes as `ColliderShape::TriMesh`
   and `ColliderShape::Heightfield` variants within the `Collider` component. Both integrate with
   the BVH inside the `Broadphase` resource for efficient culling. Triangle meshes support
   per-triangle material indices that map to `PhysicsMaterial` entries for surface-specific friction
   and effects.
   - **Deps:** F-4.2.1, F-4.2.2, F-1.1.2
   - **Platform:** Mobile: trimesh max 8K triangles per collider, heightfield max 128x128. Switch:
     trimesh max 32K, heightfield max 256x256. Desktop: trimesh max 256K, heightfield max 1024x1024.
     Coarser LODs loaded on constrained platforms.
3. **F-4.2.5** — Combine multiple primitive or convex shapes into a single entity via the
   `CompoundCollider` component, which stores a list of child shapes with local-space offsets. This
   avoids concave decomposition while supporting complex silhouettes. Each child shape within a
   `CompoundCollider` carries its own `CollisionLayers` and `PhysicsMaterial` data for independent
   filtering and surface properties.
   - **Deps:** F-4.2.3, F-1.1.2
   - **Platform:** Mobile: max 4 child shapes per compound. Switch: max 8 children. Desktop: max 32
     children. High-end PC: max 64 children. Fewer children reduce broadphase overlap test cost per
     entity.

## Filtering

| ID      | Feature |
|---------|-------------------------------- |
| F-4.2.6 | Collision Filtering and Layers |
| F-4.2.7 | Collision Events |

1. **F-4.2.6** — Filter collisions using the `CollisionLayers` ECS component, which defines layer
   membership (a bitset of layers the entity belongs to) and a collision mask (a bitset of layers
   the entity can interact with). The `BroadphaseUpdateSystem` evaluates layer compatibility at
   broadphase to reject pairs before narrowphase. An optional `CollisionFilterCallback` system
   parameter allows fine-grained runtime filtering (team-based rules, ignore-owner logic) as a
   registered ECS system.
   - **Deps:** F-4.2.1, F-1.1.2
2. **F-4.2.7** — Emit typed ECS events — `CollisionStarted`, `CollisionPersisted`, `CollisionEnded`
   — through the ECS event system. Each event carries contact points, normals, impulse magnitudes,
   and the participating entity IDs. The `CollisionEventSystem` compares current-frame
   `ContactManifold` components against the previous frame's `ActiveCollisions` resource to
   determine event type. Events are batched per-frame to reduce overhead in scenes with many
   simultaneous contacts.
   - **Deps:** F-4.2.2, F-4.2.6, F-1.1.17 (Composable Archetype Queries)

## Triggers

| ID      | Feature |
|---------|----------------- |
| F-4.2.8 | Trigger Volumes |

1. **F-4.2.8** — Non-physical collision shapes that detect overlap without generating contact
   responses. Trigger volumes are ECS entities with a `TriggerVolume` component and a
   `Collider2D`/`Collider` shape. The collision system emits `TriggerEnter`, `TriggerStay`, and
   `TriggerExit` events via the observer system (F-1.1.30) when entities enter, remain within, or
   leave the volume. Supports one-shot triggers (fire once then disable), persistent triggers (fire
   every frame while overlapping), and filtered triggers (only respond to entities matching a query
   filter). Used for area-of-effect zones, quest objective regions, loading zone transitions, and
   trap activation.
   - **Deps:** F-4.2.1, F-4.2.6 (Collision Layers), F-1.1.30 (Observers)
   - **Platform:** Mobile: max 64 active trigger volumes, prefer simple shapes (sphere, box).
     Switch: max 128 triggers. Desktop: max 1024 triggers. Persistent triggers throttled to
     every-other-frame on mobile to halve overlap test cost.

## Materials

| ID      | Feature |
|---------|------------------------- |
| F-4.2.9 | Physics Material Assets |

1. **F-4.2.9** — A `PhysicsMaterial` asset type defining surface physical properties: static
   friction, dynamic friction, restitution (bounciness), density, and surface type tag (metal, wood,
   ice, rubber). Collision pairs resolve effective friction and restitution by combining the
   materials of both bodies using configurable combine modes (average, minimum, maximum, multiply).
   Surface type tags drive audio (footstep sounds), VFX (impact particles), and gameplay (ice
   surfaces reduce traction). Materials are authored in the visual editor and assigned per-collider
   via a `PhysicsMaterialHandle` component.
   - **Deps:** F-4.2.1, F-12.7.1 (Binary Asset Format)

## Voxel Terrain Collision

| ID       | Feature |
|----------|------------------------------ |
| F-4.2.10 | Per-Chunk Voxel Trimesh Collision |

1. **F-4.2.10** — Generate a `TriMesh` collider per voxel chunk from the chunk surface mesh produced
   by the voxel meshing pipeline (F-3.2.12). When players modify voxels the affected chunk is
   re-meshed and a new collider is built on a job system worker; the old collider remains in the
   physics BVH until the next `FixedUpdate` when atomic swap installs the new collider. Heightfield
   chunks use in-place height patching without full rebuild. Chunk rebuild count per frame is
   bounded by a platform-specific budget to protect frame time.
   - **Deps:** F-4.2.4 (Triangle Mesh and Heightfield Shapes), F-4.2.1, F-3.2.12 (Voxel Meshing
     Pipeline), F-3.2.13 (Runtime Voxel Editing)
   - **Platform:** Mobile: max 1 chunk rebuild per frame, 16x16x16 chunks. Switch: max 2 rebuilds,
     16x16x16. Desktop: max 4 rebuilds, 32x32x32. High-end PC: max 8 rebuilds, 32x32x32.

## Convex Decomposition

| ID       | Feature |
|----------|-------------------------------------- |
| F-4.2.11 | Offline V-HACD Convex Decomposition |
| F-4.2.12 | Runtime Quickhull Generation |

1. **F-4.2.11** — At asset import time, non-convex source meshes are decomposed into multiple convex
   hulls via V-HACD (Volumetric Hierarchical Approximate Convex Decomposition) and stored as a baked
   `CompoundCollider`. Typical settings produce 8-16 hulls per mesh at 32-64 vertices per hull. All
   decomposition cost is paid offline. Designers can also manually place simple shapes (boxes,
   capsules, spheres) in the visual editor to override automatic decomposition.
   - **Deps:** F-4.2.5 (Compound Shapes), F-4.2.3 (Convex Hull Shapes), F-12.2.3 (Asset Processing)
2. **F-4.2.12** — At runtime, quickhull (O(n log n)) generates a single convex hull from an
   arbitrary vertex set for destructible objects, procedural geometry, and voxel chunk fragments.
   Faster than V-HACD but produces one hull rather than a decomposition — suitable for convex
   fragments spawned by destruction and procedural generation systems.
   - **Deps:** F-4.2.3 (Convex Hull Shapes), F-4.6.3 (Runtime Fracture)

## Collision Layer Interaction Matrix

| ID       | Feature |
|----------|-------------------------------------- |
| F-4.2.13 | 32x32 Collision Layer Interaction Matrix |

1. **F-4.2.13** — An editor-configurable 32x32 collision layer interaction matrix records which
   layer pairs interact and whether the interaction produces contact response or overlap only. The
   matrix is authored in the visual editor and baked at build time into layer filter bitmasks on
   every `CollisionLayers` component. Overlap-only pairs support hitbox/hurtbox combat where contact
   events fire without generating collision impulses. Per-layer budgets constrain active collider
   counts so that gameplay layers (bullets, triggers, pickups) cannot starve the physics budget.
   - **Deps:** F-4.2.6 (Collision Filtering and Layers), F-4.2.8 (Trigger Volumes)
