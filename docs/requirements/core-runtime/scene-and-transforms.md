# R-1.2 — Scene & Transforms Requirements

## Scene Hierarchy

| ID       | Derived From                                                   |
|----------|----------------------------------------------------------------|
| R-1.2.1  | [F-1.2.1](../../features/core-runtime/scene-and-transforms.md) |
| R-1.2.2  | [F-1.2.2](../../features/core-runtime/scene-and-transforms.md) |
| R-1.2.2a | [F-1.2.2](../../features/core-runtime/scene-and-transforms.md) |

1. **R-1.2.1** — The engine **SHALL** represent scene hierarchy as parent-child relationships stored
   in ECS components, where each entity has at most one parent and an ordered list of children, with
   hierarchy modifications batched through command buffers.
   - **Rationale:** ECS-native hierarchy representation enables efficient parallel traversal for
     transform propagation, culling, and serialization.
   - **Verification:** Unit test: build a hierarchy of 1,000 entities, verify each entity has
     exactly one parent (except root), verify child ordering is preserved after insertion and
     removal. Verify hierarchy modifications during parallel iteration are deferred via command
     buffers.
2. **R-1.2.2** — The engine **SHALL** provide allocation-free depth-first and breadth-first
   traversal iterators over the scene hierarchy, supporting early termination and subtree skipping,
   for trees within a fixed stack depth.
   - **Rationale:** Allocation-free traversal is critical for per-frame culling and LOD selection in
     large open worlds.
   - **Verification:** Unit test: traverse a 5-level hierarchy with both DFS and BFS iterators and
     verify visit order. Verify early termination and subtree skipping produce correct partial
     results. Profile confirms zero heap allocations during traversal for trees under 64 levels
     deep.
3. **R-1.2.2a** — Traversal iterators **SHALL** support trees up to 256 levels deep without heap
   allocation. For trees exceeding the fixed stack depth, the iterator **SHALL** transparently fall
   back to heap allocation rather than failing. The engine **SHALL** report a diagnostic warning
   when traversal encounters a hierarchy deeper than 128 levels.
   - **Rationale:** Extremely deep hierarchies may indicate design issues; graceful fallback
     prevents crashes while the warning helps identify problematic content.
   - **Verification:** Unit test: traverse a 256-level hierarchy and verify zero heap allocations.
     Traverse a 300-level hierarchy and verify it completes correctly via fallback. Verify the
     warning fires at depth 129.

## Parent-Child Relationships

| ID      | Derived From                                                   |
|---------|----------------------------------------------------------------|
| R-1.2.3 | [F-1.2.3](../../features/core-runtime/scene-and-transforms.md) |

1. **R-1.2.3** — The engine **SHALL** recursively despawn all descendants when a parent entity is
   despawned, with an optional orphan-on-delete mode that reparents children to the world root
   instead.
   - **Rationale:** Automatic cascade prevents orphaned entities from accumulating and leaking
     resources in long-running sessions.
   - **Verification:** Unit test: build a 3-level hierarchy, despawn the root in cascade mode and
     verify all descendants are destroyed. Repeat with orphan-on-delete mode and verify children are
     reparented to the world root.

## Transform Propagation

| ID       | Derived From                                                   |
|----------|----------------------------------------------------------------|
| R-1.2.4  | [F-1.2.4](../../features/core-runtime/scene-and-transforms.md) |
| R-1.2.4a | [F-1.2.4](../../features/core-runtime/scene-and-transforms.md) |

1. **R-1.2.4** — The engine **SHALL** compute world-space transforms by composing local transforms
   along the parent chain in a top-down parallel system that processes independent subtrees
   concurrently, handling chains of arbitrary depth without stack overflow.
   - **Rationale:** Parallel propagation is essential for meeting frame-time budgets in scenes with
     deep hierarchies (vehicles carrying players carrying equipment).
   - **Verification:** Benchmark: propagate transforms for 100,000 entities in a scene with mixed
     hierarchy depths (1-50 levels). Verify correct world-space transforms against a reference
     serial implementation. Verify no stack overflow at depth 1,000.
2. **R-1.2.4a** — Transform propagation **SHALL** process at least 2 million entities per
   millisecond on a 4-core system. Propagation **SHALL** introduce no more than 1 frame of latency
   between a local transform modification and its world-space result being visible to other systems.
   - **Rationale:** Transform propagation runs every frame and is on the critical path; latency and
     throughput directly affect visual fidelity and physics accuracy.
   - **Verification:** Benchmark: propagate 2 million entities across 4 cores and verify total time
     is under 1 ms. Modify a local transform in frame N and verify the world-space result is
     available to all systems in frame N (not N+1).

## Dirty Tracking

| ID      | Derived From                                                   |
|---------|----------------------------------------------------------------|
| R-1.2.5 | [F-1.2.5](../../features/core-runtime/scene-and-transforms.md) |

1. **R-1.2.5** — The engine **SHALL** use ECS change detection to mark transforms as dirty and skip
   world-space recomputation for subtrees with no dirty ancestors in the current frame.
   - **Rationale:** In open worlds where most entities are stationary, dirty tracking reduces
     propagation cost by orders of magnitude.
   - **Verification:** Benchmark: in a scene of 100,000 entities where 1% are moving, verify
     propagation time is proportional to the dirty count (approximately 1% of full propagation
     time). Unit test: modify a leaf's local transform and verify only its subtree is recomputed.

## Spatial Partitioning

| ID      | Derived From                                                   |
|---------|----------------------------------------------------------------|
| R-1.2.6 | [F-1.2.6](../../features/core-runtime/scene-and-transforms.md) |

1. **R-1.2.6** — The engine **SHALL** delegate spatial partitioning to the shared BVH spatial index
   (R-1.9.1). The scene-and-transforms module **SHALL** register transform-bearing entities with the
   shared spatial index and provide a scene-level query API that wraps the unified spatial query API
   (R-1.9.4).
   - **Rationale:** Brute-force spatial queries are infeasible at scale; an incrementally updated
     index amortizes update costs across frames.
   - **Verification:** Benchmark: insert 2 million entities, perform 100 frustum queries per frame
     and verify all complete within 1 ms total. Verify incremental update cost is proportional to
     moved entity count, not total count.

## Scene Queries

| ID      | Derived From                                                   |
|---------|----------------------------------------------------------------|
| R-1.2.7 | [F-1.2.7](../../features/core-runtime/scene-and-transforms.md) |

1. **R-1.2.7** — The engine **SHALL** provide a spatial query API supporting point containment, ray
   intersection, sphere/box overlap, and k-nearest-neighbor queries, combining spatial filtering
   from the acceleration structure with ECS archetype filtering to restrict results by component
   presence.
   - **Rationale:** Combined spatial + archetype filtering avoids expensive post-filtering and
     powers gameplay systems like proximity triggers and area-of-effect abilities.
   - **Verification:** Integration test: populate a scene with 10,000 entities having mixed
     component sets. Perform a sphere overlap query filtered to `With<Enemy>` and verify only enemy
     entities within the sphere are returned. Verify ray intersection returns hits sorted by
     distance.
