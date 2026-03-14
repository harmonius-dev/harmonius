# 4.2 — Collision Detection

## Broadphase

### F-4.2.1 Broadphase Acceleration Structures

Cull collision pairs using the shared spatial index (F-1.9.1) rather than a physics-private
acceleration structure. The `BroadphaseQuerySystem` reads the shared BVH ECS resource and
queries overlapping AABB pairs, filtered by `CollisionLayers` components. Candidate pairs are
written to a `BroadphasePairs` ECS resource for consumption by narrowphase. Because the BVH
is shared with rendering culling, network AOI, and AI perception, physics avoids rebuilding
a redundant spatial structure.

- **Requirements:** R-4.2.1
- **Dependencies:** F-1.9.1 (Shared BVH), F-1.9.6 (Physics Integration), F-1.1.1
- **Platform notes:** None

## Narrowphase

### F-4.2.2 Narrowphase Contact Generation

Generate precise contact points and penetration depths using GJK (distance), EPA (penetration),
and SAT (feature-based contacts) algorithms. The `NarrowphaseSystem` reads candidate pairs from
the `BroadphasePairs` resource and `Collider` components, then produces `ContactManifold`
components on collision pair entities containing contact points, normals, and penetration depths.
Contact generation is deterministic across platforms for server-authoritative simulation.

- **Requirements:** R-4.2.2
- **Dependencies:** F-4.2.1, F-1.1.2
- **Platform notes:** None

## Collision Shapes

### F-4.2.3 Primitive and Convex Collision Shapes

Support box, sphere, capsule, and convex hull shapes via the `Collider` ECS component, which
holds a `ColliderShape` enum (`Box`, `Sphere`, `Capsule`, `ConvexHull`). Convex hulls are
generated from arbitrary meshes at build time with configurable vertex limits. The
`NarrowphaseSystem` dispatches to specialized fast-path routines for primitive-vs-primitive
pairs rather than falling through to generic GJK.

- **Requirements:** R-4.2.3
- **Dependencies:** F-4.2.2, F-1.1.2
- **Platform notes:** None

### F-4.2.4 Triangle Mesh and Heightfield Shapes

Support concave triangle mesh and heightfield shapes as `ColliderShape::TriMesh` and
`ColliderShape::Heightfield` variants within the `Collider` component. Both integrate with the
BVH inside the `Broadphase` resource for efficient culling. Triangle meshes support per-triangle
material indices that map to `PhysicsMaterial` entries for surface-specific friction and effects.

- **Requirements:** R-4.2.4
- **Dependencies:** F-4.2.1, F-4.2.2, F-1.1.2
- **Platform notes:** None

### F-4.2.5 Compound Shapes

Combine multiple primitive or convex shapes into a single entity via the `CompoundCollider`
component, which stores a list of child shapes with local-space offsets. This avoids concave
decomposition while supporting complex silhouettes. Each child shape within a `CompoundCollider`
carries its own `CollisionLayers` and `PhysicsMaterial` data for independent filtering and
surface properties.

- **Requirements:** R-4.2.5
- **Dependencies:** F-4.2.3, F-1.1.2
- **Platform notes:** None

## Filtering

### F-4.2.6 Collision Filtering and Layers

Filter collisions using the `CollisionLayers` ECS component, which defines layer membership
(a bitset of layers the entity belongs to) and a collision mask (a bitset of layers the entity
can interact with). The `BroadphaseUpdateSystem` evaluates layer compatibility at broadphase to
reject pairs before narrowphase. An optional `CollisionFilterCallback` system parameter allows
fine-grained runtime filtering (team-based rules, ignore-owner logic) as a registered ECS
system.

- **Requirements:** R-4.2.6
- **Dependencies:** F-4.2.1, F-1.1.2
- **Platform notes:** None

### F-4.2.7 Collision Events

Emit typed ECS events — `CollisionStarted`, `CollisionPersisted`, `CollisionEnded` — through
the ECS event system. Each event carries contact points, normals, impulse magnitudes, and the
participating entity IDs. The `CollisionEventSystem` compares current-frame `ContactManifold`
components against the previous frame's `ActiveCollisions` resource to determine event type.
Events are batched per-frame to reduce overhead in scenes with many simultaneous contacts.

- **Requirements:** R-4.2.7
- **Dependencies:** F-4.2.2, F-4.2.6, F-1.1.5
- **Platform notes:** None
