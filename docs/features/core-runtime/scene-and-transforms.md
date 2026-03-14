# 1.2 — Scene & Transforms

## Scene Hierarchy

### F-1.2.1 Entity-Based Scene Hierarchy

Represent scene hierarchy as parent-child relationships stored directly in ECS components. Each entity may have at
most one parent and an ordered list of children. Hierarchy modifications are batched through command buffers to
maintain consistency during parallel iteration. This enables efficient traversal for transform propagation, culling,
and serialization.

- **Requirements:** R-1.2.1
- **Dependencies:** F-1.1.4 (Entity Lifecycle), F-1.1.11 (Deferred Structural Changes)
- **Platform notes:** None

### F-1.2.2 Hierarchy Traversal Iterators

Provide depth-first and breadth-first traversal iterators over the scene hierarchy. Iterators are lazy and
allocation-free for trees that fit within a fixed stack depth. Traversal supports early termination and subtree
skipping, which is essential for culling and LOD selection in large open worlds.

- **Requirements:** R-1.2.2
- **Dependencies:** F-1.2.1
- **Platform notes:** None

## Parent-Child Relationships

### F-1.2.3 Cascading Lifecycle Propagation

When a parent entity is despawned, recursively despawn all descendants. Optionally allow orphan-on-delete semantics
where children are reparented to the world root instead. Lifecycle cascading is processed through command buffers
to avoid iterator invalidation during hierarchy walks.

- **Requirements:** R-1.2.3
- **Dependencies:** F-1.2.1, F-1.1.11 (Deferred Structural Changes)
- **Platform notes:** None

## Transform Propagation

### F-1.2.4 Hierarchical Transform Propagation

Compute world-space transforms by composing local transforms along the parent chain. Propagation runs as a
top-down parallel system that processes independent subtrees concurrently. For MMO worlds with deep hierarchies
(e.g., vehicles carrying players carrying equipment), propagation must handle chains of arbitrary depth without
stack overflow.

- **Requirements:** R-1.2.4
- **Dependencies:** F-1.2.1, F-1.1.6 (Parallel Iteration)
- **Platform notes:** None

## Dirty Tracking

### F-1.2.5 Transform Dirty Tracking

Use the ECS change detection system to mark transforms as dirty when their local transform is modified. Propagation
only recomputes world-space transforms for dirty subtrees, skipping static geometry entirely. In open worlds where
most entities are stationary each frame, this reduces propagation cost by orders of magnitude.

- **Requirements:** R-1.2.5
- **Dependencies:** F-1.2.4, F-1.1.7 (Change Detection)
- **Platform notes:** None

## Spatial Partitioning

### F-1.2.6 Spatial Partitioning Index

Maintain a spatial acceleration structure (e.g., BVH, grid, or R-tree) that indexes entity world-space positions
and bounding volumes. The index is updated incrementally using dirty flags from the transform system. It must
support millions of entities and provide sub-millisecond query times for frustum culling, range queries, and
nearest-neighbor lookups.

- **Requirements:** R-1.2.6
- **Dependencies:** F-1.2.4, F-1.2.5
- **Platform notes:** None

## Scene Queries

### F-1.2.7 Spatial Scene Queries

Provide a query API for spatial operations: point containment, ray intersection, sphere/box overlap, and k-nearest
neighbors. Queries combine spatial filtering from the acceleration structure with ECS component filtering to
retrieve only entities matching both spatial and archetype criteria. This powers gameplay systems like proximity
triggers, line-of-sight checks, and area-of-effect abilities.

- **Requirements:** R-1.2.7
- **Dependencies:** F-1.2.6, F-1.1.5 (Composable Archetype Queries)
- **Platform notes:** None
