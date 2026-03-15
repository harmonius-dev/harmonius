# Scene & Transforms User Stories

## Scene Hierarchy

## US-1.2.1 Represent Scene Hierarchy as ECS Relationships

**As a** game developer (P-15), **I want** scene hierarchy represented as ECS parent-child
relationships with batched modifications via command buffers, **so that** I can build and modify
scene graphs without iterator invalidation during parallel iteration.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Each entity has at most one parent and ordered children | F-1.2.1 | R-1.2.1 |
| Hierarchy modifications batched through command buffers | F-1.2.1 | R-1.2.1 |
| Consistent state during parallel iteration | F-1.2.1 | R-1.2.1 |

## US-1.2.2 Browse Scene Hierarchy in the Visual Editor

**As a** designer (P-5), **I want** to browse, expand, and collapse the scene hierarchy in the
visual editor with drag-and-drop reparenting, **so that** I can organize entities into logical
groups and adjust parent-child relationships visually.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Hierarchical tree view in editor | F-1.2.1 | R-1.2.1 |
| Drag-and-drop reparenting | F-1.2.1 | R-1.2.1 |
| Expand/collapse subtrees | F-1.2.1 | R-1.2.1 |

## US-1.2.3 Traverse Hierarchies Efficiently for Culling and LOD

**As an** engine developer (P-26), **I want** allocation-free depth-first and breadth-first
traversal iterators with early termination and subtree skipping, **so that** per-frame culling and
LOD selection in large open worlds incur no heap allocation overhead.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Depth-first and breadth-first iterators | F-1.2.2 | R-1.2.2 |
| Allocation-free for trees within fixed stack depth | F-1.2.2 | R-1.2.2 |
| Early termination and subtree skipping | F-1.2.2 | R-1.2.2 |

## US-1.2.4 Benchmark Hierarchy Traversal at Scale

**As an** engine tester (P-27), **I want** to benchmark hierarchy traversal over trees with 100K+
entities and varying depths, **so that** I can verify allocation-free iteration holds and traversal
performance meets frame budgets.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 100K entity tree traversed within budget | F-1.2.2 | R-1.2.2 |
| Zero heap allocations during traversal | F-1.2.2 | R-1.2.2 |
| Early termination reduces traversal cost proportionally | F-1.2.2 | R-1.2.2 |

## Parent-Child Relationships

## US-1.2.5 Cascade Parent Deletion to All Descendants

**As a** game developer (P-15), **I want** parent deletion to automatically despawn all descendants
with an optional orphan-on-delete mode, **so that** I never accumulate orphaned entities that leak
resources in long-running sessions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Parent despawn cascades to all descendants | F-1.2.3 | R-1.2.3 |
| Orphan-on-delete mode reparents children to world root | F-1.2.3 | R-1.2.3 |
| Cascading processed through command buffers | F-1.2.3 | R-1.2.3 |

## US-1.2.6 Verify No Orphaned Entities After Parent Deletion

**As an** engine tester (P-27), **I want** to verify that after deleting parent entities at various
depths, no orphaned entities remain in the world, **so that** long-running server sessions do not
leak entities over time.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Zero orphaned entities after cascading delete | F-1.2.3 | R-1.2.3 |
| Orphan-on-delete mode correctly reparents to root | F-1.2.3 | R-1.2.3 |
| No iterator invalidation during cascading | F-1.2.3 | R-1.2.3 |

## Transform Propagation

## US-1.2.7 Propagate Transforms in Parallel Across Independent Subtrees

**As an** engine developer (P-26), **I want** world-space transforms computed by composing local
transforms in a top-down parallel system that processes independent subtrees concurrently, **so
that** deep hierarchies (vehicles carrying players carrying equipment) propagate correctly without
stack overflow.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Top-down parallel propagation | F-1.2.4 | R-1.2.4 |
| Independent subtrees processed concurrently | F-1.2.4 | R-1.2.4 |
| Arbitrary depth without stack overflow (iterative) | F-1.2.4 | R-1.2.4 |
| Max depth configurable per platform | F-1.2.4 | R-1.2.4 |

## US-1.2.8 Verify Transform Propagation Correctness for Deep Hierarchies

**As an** engine tester (P-27), **I want** to verify that transform propagation produces correct
world-space transforms for hierarchies up to maximum platform depth, **so that** deeply nested
entities have accurate positions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Correct results at max depth (32 mobile, 256 desktop) | F-1.2.4 | R-1.2.4 |
| No stack overflow at any supported depth | F-1.2.4 | R-1.2.4 |
| Parallel results match serial reference implementation | F-1.2.4 | R-1.2.4 |

## Dirty Tracking

## US-1.2.9 Skip Static Geometry During Transform Propagation

**As an** engine developer (P-26), **I want** transform propagation to skip subtrees with no dirty
ancestors using ECS change detection, **so that** in open worlds where most entities are stationary,
propagation cost is proportional to moved entities only.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Dirty marking on local transform modification | F-1.2.5 | R-1.2.5 |
| Unmodified subtrees skipped entirely | F-1.2.5 | R-1.2.5 |
| Cost proportional to moved entities, not total count | F-1.2.5 | R-1.2.5 |

## US-1.2.10 Benchmark Dirty Tracking Savings in Static Worlds

**As an** engine tester (P-27), **I want** to benchmark transform propagation with varying ratios of
static to dynamic entities, **so that** I can measure the order-of-magnitude cost reduction from
dirty tracking when most entities are stationary.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 1M entities, 1% moving: propagation under 0.5ms | F-1.2.5 | R-1.2.5 |
| Cost scales with moved entity count, not total | F-1.2.5 | R-1.2.5 |
| No false dirty marks from read-only access | F-1.2.5 | R-1.2.5 |

## Spatial Partitioning

## US-1.2.11 Maintain Incrementally Updated Spatial Acceleration Structure

**As an** engine developer (P-26), **I want** a spatial acceleration structure indexing entity
positions and bounding volumes with incremental updates from dirty flags, **so that** frustum
culling, range queries, and nearest-neighbor lookups complete in sub-millisecond time.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| BVH, grid, or R-tree indexing world-space positions | F-1.2.6 | R-1.2.6 |
| Incremental update using transform dirty flags | F-1.2.6 | R-1.2.6 |
| Sub-millisecond query times for 1M+ entities | F-1.2.6 | R-1.2.6 |

## US-1.2.12 Use Spatial Queries to Retrieve Game Entities

**As a** game developer (P-15), **I want** a spatial query API for point containment, ray
intersection, sphere/box overlap, and k-nearest neighbors that combines spatial and ECS component
filtering, **so that** proximity triggers, line-of-sight, and AoE abilities retrieve only matching
entities.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Point, ray, sphere, box, frustum, k-NN queries | F-1.2.7 | R-1.2.7 |
| Combined spatial + ECS component filtering | F-1.2.7 | R-1.2.7 |
| Results as ECS Entity handles with hit metadata | F-1.2.7 | R-1.2.7 |

## US-1.2.13 Understand How Transforms and Hierarchy Work Together

**As a** designer (P-5), **I want** to see both local and world-space transforms in the visual
editor and understand how parent-child relationships affect position, **so that** I can place child
entities relative to their parent and predict the visual result.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Editor shows both local and world-space transforms | F-1.2.4 | R-1.2.4 |
| Moving parent visually moves children | F-1.2.4 | R-1.2.4 |
| Local transform editable relative to parent | F-1.2.4 | R-1.2.4 |
