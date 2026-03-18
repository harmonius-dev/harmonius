# Scene & Transforms User Stories

## Scene Hierarchy

| ID       | Persona                 | Features | Requirements |
|----------|-------------------------|----------|--------------|
| US-1.2.1 | game developer (P-15)   | F-1.2.1  | R-1.2.1      |
| US-1.2.2 | designer (P-5)          | F-1.2.1  | R-1.2.1      |
| US-1.2.3 | engine developer (P-26) | F-1.2.2  | R-1.2.2      |
| US-1.2.4 | engine tester (P-27)    | F-1.2.2  | R-1.2.2      |

1. **US-1.2.1** — scene hierarchy represented as ECS parent-child relationships with batched
   modifications via command buffers, so that I can build and modify scene graphs without iterator
   invalidation during parallel iteration
   - **Acceptance:** Each entity has at most one parent and ordered children<br>Hierarchy
     modifications batched through command buffers<br>Consistent state during parallel iteration
2. **US-1.2.2** — to browse, expand, and collapse the scene hierarchy in the visual editor with
   drag-and-drop reparenting, so that I can organize entities into logical groups and adjust
   parent-child relationships visually
   - **Acceptance:** Hierarchical tree view in editor<br>Drag-and-drop
     reparenting<br>Expand/collapse subtrees
3. **US-1.2.3** — allocation-free depth-first and breadth-first traversal iterators with early
   termination and subtree skipping, so that per-frame culling and LOD selection in large open
   worlds incur no heap allocation overhead
   - **Acceptance:** Depth-first and breadth-first iterators<br>Allocation-free for trees within
     fixed stack depth<br>Early termination and subtree skipping
4. **US-1.2.4** — to benchmark hierarchy traversal over trees with 100K+ entities and varying
   depths, so that I can verify allocation-free iteration holds and traversal performance meets
   frame budgets
   - **Acceptance:** 100K entity tree traversed within budget<br>Zero heap allocations during
     traversal<br>Early termination reduces traversal cost proportionally

## Parent-Child Relationships

| ID       | Persona               | Features | Requirements |
|----------|-----------------------|----------|--------------|
| US-1.2.5 | game developer (P-15) | F-1.2.3  | R-1.2.3      |
| US-1.2.6 | engine tester (P-27)  | F-1.2.3  | R-1.2.3      |

1. **US-1.2.5** — parent deletion to automatically despawn all descendants with an optional
   orphan-on-delete mode, so that I never accumulate orphaned entities that leak resources in
   long-running sessions
   - **Acceptance:** Parent despawn cascades to all descendants<br>Orphan-on-delete mode reparents
     children to world root<br>Cascading processed through command buffers
2. **US-1.2.6** — to verify that after deleting parent entities at various depths, no orphaned
   entities remain in the world, so that long-running server sessions do not leak entities over time
   - **Acceptance:** Zero orphaned entities after cascading delete<br>Orphan-on-delete mode
     correctly reparents to root<br>No iterator invalidation during cascading

## Transform Propagation

| ID       | Persona                 | Features | Requirements |
|----------|-------------------------|----------|--------------|
| US-1.2.7 | engine developer (P-26) | F-1.2.4  | R-1.2.4      |
| US-1.2.8 | engine tester (P-27)    | F-1.2.4  | R-1.2.4      |

1. **US-1.2.7** — world-space transforms computed by composing local transforms in a top-down
   parallel system that processes independent subtrees concurrently, so that deep hierarchies
   (vehicles carrying players carrying equipment) propagate correctly without stack overflow
   - **Acceptance:** Top-down parallel propagation<br>Independent subtrees processed
     concurrently<br>Arbitrary depth without stack overflow (iterative)<br>Max depth configurable
     per platform
2. **US-1.2.8** — to verify that transform propagation produces correct world-space transforms for
   hierarchies up to maximum platform depth, so that deeply nested entities have accurate positions
   - **Acceptance:** Correct results at max depth (32 mobile, 256 desktop)<br>No stack overflow at
     any supported depth<br>Parallel results match serial reference implementation

## Dirty Tracking

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.2.9  | engine developer (P-26) | F-1.2.5  | R-1.2.5      |
| US-1.2.10 | engine tester (P-27)    | F-1.2.5  | R-1.2.5      |

1. **US-1.2.9** — transform propagation to skip subtrees with no dirty ancestors using ECS change
   detection, so that in open worlds where most entities are stationary, propagation cost is
   proportional to moved entities only
   - **Acceptance:** Dirty marking on local transform modification<br>Unmodified subtrees skipped
     entirely<br>Cost proportional to moved entities, not total count
2. **US-1.2.10** — to benchmark transform propagation with varying ratios of static to dynamic
   entities, so that I can measure the order-of-magnitude cost reduction from dirty tracking when
   most entities are stationary
   - **Acceptance:** 1M entities, 1% moving: propagation under 0.5ms<br>Cost scales with moved
     entity count, not total<br>No false dirty marks from read-only access

## Spatial Partitioning

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.2.11 | engine developer (P-26) | F-1.2.6  | R-1.2.6      |
| US-1.2.12 | game developer (P-15)   | F-1.2.7  | R-1.2.7      |
| US-1.2.13 | designer (P-5)          | F-1.2.4  | R-1.2.4      |

1. **US-1.2.11** — a spatial acceleration structure indexing entity positions and bounding volumes
   with incremental updates from dirty flags, so that frustum culling, range queries, and
   nearest-neighbor lookups complete in sub-millisecond time
   - **Acceptance:** BVH, grid, or R-tree indexing world-space positions<br>Incremental update using
     transform dirty flags<br>Sub-millisecond query times for 1M+ entities
2. **US-1.2.12** — a spatial query API for point containment, ray intersection, sphere/box overlap,
   and k-nearest neighbors that combines spatial and ECS component filtering, so that proximity
   triggers, line-of-sight, and AoE abilities retrieve only matching entities
   - **Acceptance:** Point, ray, sphere, box, frustum, k-NN queries<br>Combined spatial + ECS
     component filtering<br>Results as ECS Entity handles with hit metadata
3. **US-1.2.13** — to see both local and world-space transforms in the visual editor and understand
   how parent-child relationships affect position, so that I can place child entities relative to
   their parent and predict the visual result
   - **Acceptance:** Editor shows both local and world-space transforms<br>Moving parent visually
     moves children<br>Local transform editable relative to parent
