# 1.2 — Scene & Transforms

## Scene Hierarchy

| ID      | Feature                       | Requirements |
|---------|-------------------------------|--------------|
| F-1.2.1 | Entity-Based Scene Hierarchy  | R-1.2.1      |
| F-1.2.2 | Hierarchy Traversal Iterators | R-1.2.2      |

1. **F-1.2.1** — Represent scene hierarchy as parent-child relationships stored directly in ECS
   components. Each entity may have at most one parent and an ordered list of children. Hierarchy
   modifications are batched through command buffers to maintain consistency during parallel
   iteration. This enables efficient traversal for transform propagation, culling, and
   serialization.
   - **Deps:** F-1.1.11 (Entity Lifecycle with Generational Indices), F-1.1.32 (Deferred Structural
     Changes via Command Buffers)
2. **F-1.2.2** — Provide depth-first and breadth-first traversal iterators over the scene hierarchy.
   Iterators are lazy and allocation-free for trees that fit within a fixed stack depth. Traversal
   supports early termination and subtree skipping, which is essential for culling and LOD selection
   in large open worlds.
   - **Deps:** F-1.2.1

## Parent-Child Relationships

| ID      | Feature                         | Requirements |
|---------|---------------------------------|--------------|
| F-1.2.3 | Cascading Lifecycle Propagation | R-1.2.3      |

1. **F-1.2.3** — When a parent entity is despawned, recursively despawn all descendants. Optionally
   allow orphan-on-delete semantics where children are reparented to the world root instead.
   Lifecycle cascading is processed through command buffers to avoid iterator invalidation during
   hierarchy walks.
   - **Deps:** F-1.2.1, F-1.1.32 (Deferred Structural Changes via Command Buffers)

## Transform Propagation

| ID      | Feature                            | Requirements |
|---------|------------------------------------|--------------|
| F-1.2.4 | Hierarchical Transform Propagation | R-1.2.4      |

1. **F-1.2.4** — Compute world-space transforms by composing local transforms along the parent
   chain. Propagation runs as a top-down parallel system that processes independent subtrees
   concurrently. For MMO worlds with deep hierarchies (e.g., vehicles carrying players carrying
   equipment), propagation must handle chains of arbitrary depth without stack overflow.
   - **Deps:** F-1.2.1, F-1.1.20 (Automatic Parallel Iteration)
   - **Platform:** Mobile: max hierarchy depth 32, parallel threshold 512 entities. Switch: max
     depth 64, parallel threshold 256. Desktop: max depth 256, parallel threshold 128. All platforms
     use iterative (not recursive) propagation to avoid stack overflow.

## Dirty Tracking

| ID      | Feature                  | Requirements |
|---------|--------------------------|--------------|
| F-1.2.5 | Transform Dirty Tracking | R-1.2.5      |

1. **F-1.2.5** — Use the ECS change detection system to mark transforms as dirty when their local
   transform is modified. Propagation only recomputes world-space transforms for dirty subtrees,
   skipping static geometry entirely. In open worlds where most entities are stationary each frame,
   this reduces propagation cost by orders of magnitude.
   - **Deps:** F-1.2.4, F-1.1.22 (Tick-Based Change Detection)

## Spatial Partitioning

| ID      | Feature                    | Requirements |
|---------|----------------------------|--------------|
| F-1.2.6 | Spatial Partitioning Index | R-1.2.6      |

1. **F-1.2.6** — Maintain a spatial acceleration structure (e.g., BVH, grid, or R-tree) that indexes
   entity world-space positions and bounding volumes. The index is updated incrementally using dirty
   flags from the transform system. It must support millions of entities and provide sub-millisecond
   query times for frustum culling, range queries, and nearest-neighbor lookups. This feature
   delegates to the shared spatial index (F-1.9.1) for the underlying acceleration structure.
   - **Deps:** F-1.2.4, F-1.2.5, F-1.9.1 (Shared BVH Spatial Index)
   - **Platform:** Mobile: max 50K indexed entities, BVH refit budget 0.5ms. Switch: max 200K
     entities, 0.8ms budget. Desktop: 1M+ entities, 1ms budget. High-end PC: 5M+ entities with
     parallel BVH refit.

## Scene Queries

| ID      | Feature               | Requirements |
|---------|-----------------------|--------------|
| F-1.2.7 | Spatial Scene Queries | R-1.2.7      |

1. **F-1.2.7** — Provide a query API for spatial operations: point containment, ray intersection,
   sphere/box overlap, and k-nearest neighbors. Queries combine spatial filtering from the
   acceleration structure with ECS component filtering to retrieve only entities matching both
   spatial and archetype criteria. This powers gameplay systems like proximity triggers,
   line-of-sight checks, and area-of-effect abilities. This feature delegates to the shared spatial
   index (F-1.9.1) for the underlying acceleration structure.
   - **Deps:** F-1.2.6, F-1.1.17 (Composable Archetype Queries), F-1.9.4 (Unified Spatial Query API)
