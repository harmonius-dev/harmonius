# User Stories -- 4.4 Spatial Queries

## Ray and Shape Casts

| ID         | Persona                 |
|------------|-------------------------|
| US-4.4.1.1 | game developer (P-15)   |
| US-4.4.1.2 | engine developer (P-26) |
| US-4.4.1.3 | game designer (P-5)     |
| US-4.4.2.1 | game developer (P-15)   |
| US-4.4.2.2 | engine tester (P-27)    |

1. **US-4.4.1.1** -- **As a** game developer (P-15), **I want** ray casts via a RayCast system
   parameter that queries the shared spatial index, **so that** line-of-sight and targeting checks
   are fast.
2. **US-4.4.1.2** -- **As a** engine developer (P-26), **I want** ray cast results to include hit
   entity, position, normal, distance, and collision layers, **so that** downstream systems have
   full hit context.
3. **US-4.4.1.3** -- **As a** game designer (P-5), **I want** ray casts filtered by CollisionLayers
   and query predicates, **so that** weapon targeting ignores irrelevant entities.
4. **US-4.4.2.1** -- **As a** game developer (P-15), **I want** shape casts sweeping sphere,
   capsule, box, or convex hull along a direction, **so that** character ground detection and
   projectile hit detection are accurate.
5. **US-4.4.2.2** -- **As a** engine tester (P-27), **I want** to verify shape casts report correct
   contact points and normals, **so that** swept collision results are geometrically precise.

## Overlap and Proximity

| ID         | Persona                 |
|------------|-------------------------|
| US-4.4.3.1 | game developer (P-15)   |
| US-4.4.3.2 | game designer (P-5)     |
| US-4.4.4.1 | game developer (P-15)   |
| US-4.4.4.2 | engine developer (P-26) |

1. **US-4.4.3.1** -- **As a** game developer (P-15), **I want** overlap queries testing a shape
   against all colliders, **so that** area-of-effect abilities find all affected entities.
2. **US-4.4.3.2** -- **As a** game designer (P-5), **I want** overlap results to include entity IDs
   and collision layers, **so that** I filter results by gameplay relevance.
3. **US-4.4.4.1** -- **As a** game developer (P-15), **I want** closest-point queries returning the
   nearest point on any collider surface, **so that** distance-based triggers and attachment
   calculations work precisely.
4. **US-4.4.4.2** -- **As a** engine developer (P-26), **I want** closest-point results to include
   signed distance and surface normal, **so that** proximity logic uses accurate geometric data.

## Batching and Performance

| ID         | Persona                 |
|------------|-------------------------|
| US-4.4.5.1 | game developer (P-15)   |
| US-4.4.5.2 | engine developer (P-26) |
| US-4.4.5.3 | engine tester (P-27)    |
| US-4.4.6.1 | game developer (P-15)   |
| US-4.4.6.2 | engine developer (P-26) |

1. **US-4.4.5.1** -- **As a** game developer (P-15), **I want** batch queries submitting multiple
   casts for parallel execution, **so that** AI agents issuing many queries per tick stay within
   budget.
2. **US-4.4.5.2** -- **As a** engine developer (P-26), **I want** batch queries to amortize BVH
   traversal cost and exploit SIMD parallelism, **so that** server-side simulation scales to
   hundreds of agents.
3. **US-4.4.5.3** -- **As a** engine tester (P-27), **I want** to verify batch query results match
   individual query results, **so that** batching produces identical answers.
4. **US-4.4.6.1** -- **As a** game developer (P-15), **I want** query filters combining
   CollisionLayers masks with ECS component filters and custom predicates, **so that** queries like
   "nearest enemy not behind cover" work without post-filtering.
5. **US-4.4.6.2** -- **As a** engine developer (P-26), **I want** custom predicates to receive an
   EntityRef during traversal, **so that** filters access any component on candidate entities.
