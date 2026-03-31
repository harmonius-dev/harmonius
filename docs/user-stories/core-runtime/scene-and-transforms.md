# Scene & Transforms User Stories

## Scene Hierarchy

| ID       | Persona                 |
|----------|-------------------------|
| US-1.2.1 | game developer (P-15)   |
| US-1.2.2 | engine developer (P-26) |
| US-1.2.3 | engine tester (P-27)    |

1. **US-1.2.1** — **As a** game developer (P-15), **I want** scene hierarchy represented as ECS
   parent-child relationships with batched modifications via command buffers, **so that** I can
   build and modify scene graphs without iterator invalidation during parallel iteration.
2. **US-1.2.2** — **As an** engine developer (P-26), **I want** allocation-free depth-first and
   breadth-first traversal iterators with early termination and subtree skipping, **so that**
   per-frame culling and LOD selection in large open worlds incur no heap allocation overhead.
3. **US-1.2.3** — **As an** engine tester (P-27), **I want** to benchmark hierarchy traversal over
   trees with 100K+ entities and varying depths, **so that** I can verify allocation-free iteration
   holds and traversal performance meets frame budgets.

## Parent-Child Relationships

| ID       | Persona               |
|----------|-----------------------|
| US-1.2.4 | game developer (P-15) |
| US-1.2.5 | engine tester (P-27)  |

1. **US-1.2.4** — **As a** game developer (P-15), **I want** parent deletion to automatically
   despawn all descendants with an optional orphan-on-delete mode, **so that** I never accumulate
   orphaned entities that leak resources in long-running sessions.
2. **US-1.2.5** — **As an** engine tester (P-27), **I want** to verify that after deleting parent
   entities at various depths no orphaned entities remain, **so that** long-running server sessions
   do not leak entities.

## Transform Propagation

| ID       | Persona                 |
|----------|-------------------------|
| US-1.2.6 | engine developer (P-26) |
| US-1.2.7 | engine tester (P-27)    |

1. **US-1.2.6** — **As an** engine developer (P-26), **I want** world-space transforms computed by
   composing local transforms in a top-down parallel system that processes independent subtrees
   concurrently, **so that** deep hierarchies propagate correctly without stack overflow.
2. **US-1.2.7** — **As an** engine tester (P-27), **I want** to verify that transform propagation
   produces correct world-space transforms for hierarchies up to maximum platform depth, **so that**
   deeply nested entities have accurate positions.

## Dirty Tracking

| ID       | Persona                 |
|----------|-------------------------|
| US-1.2.8 | engine developer (P-26) |
| US-1.2.9 | engine tester (P-27)    |

1. **US-1.2.8** — **As an** engine developer (P-26), **I want** transform propagation to skip
   subtrees with no dirty ancestors using ECS change detection, **so that** in open worlds where
   most entities are stationary, propagation cost is proportional to moved entities only.
2. **US-1.2.9** — **As an** engine tester (P-27), **I want** to benchmark transform propagation with
   varying ratios of static to dynamic entities, **so that** I can measure the cost reduction from
   dirty tracking when most entities are stationary.

## Spatial Partitioning

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.2.10 | engine developer (P-26) |
| US-1.2.11 | game developer (P-15)   |
| US-1.2.12 | technical artist (P-13) |

1. **US-1.2.10** — **As an** engine developer (P-26), **I want** a spatial acceleration structure
   indexing entity positions and bounding volumes with incremental updates from dirty flags,
   **so that** frustum culling and range queries complete in sub-millisecond time.
2. **US-1.2.11** — **As a** game developer (P-15), **I want** a spatial query API for point
   containment, ray intersection, sphere/box overlap, and k-nearest neighbors that combines spatial
   and ECS component filtering, **so that** proximity triggers and AoE abilities retrieve only
   matching entities.
3. **US-1.2.12** — **As a** technical artist (P-13), **I want** to see both local and world-space
   transforms in the editor and understand how parent-child relationships affect position,
   **so that** I can place child entities relative to their parent.
