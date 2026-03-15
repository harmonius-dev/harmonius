# Scene & Transforms User Stories

## Scene Hierarchy

### US-1.2.1 Entity-Based Scene Hierarchy

**As a** game developer, **I want** scene hierarchy represented as ECS parent-child relationships
with batched modifications, **so that** I can build and modify scene graphs without worrying
about iterator invalidation during parallel iteration.

### US-1.2.2 Hierarchy Traversal Iterators

**As an** engine developer, **I want** allocation-free depth-first and breadth-first traversal
iterators with early termination and subtree skipping, **so that** per-frame culling and LOD
selection in large open worlds do not incur heap allocation overhead.

## Parent-Child Relationships

### US-1.2.3 Cascading Lifecycle Propagation

**As a** game developer, **I want** parent deletion to automatically despawn all descendants with
an optional orphan-on-delete mode, **so that** I never accumulate orphaned entities that leak
resources in long-running sessions.

## Transform Propagation

### US-1.2.4 Hierarchical Transform Propagation

**As an** engine developer, **I want** world-space transforms computed by composing local
transforms in a top-down parallel system, **so that** deep hierarchies like vehicles carrying
players carrying equipment propagate correctly without stack overflow.

## Dirty Tracking

### US-1.2.5 Transform Dirty Tracking

**As an** engine developer, **I want** transform propagation to skip subtrees with no dirty
ancestors, **so that** in open worlds where most entities are stationary, propagation cost is
proportional to the number of moved entities.

## Spatial Partitioning

### US-1.2.6 Spatial Partitioning Index

**As an** engine developer, **I want** an incrementally updated spatial acceleration structure
indexing entity positions and bounding volumes, **so that** frustum culling, range queries, and
nearest-neighbor lookups complete in sub-millisecond time across millions of entities.

## Scene Queries

### US-1.2.7 Spatial Scene Queries

**As a** game developer, **I want** a spatial query API combining spatial filtering with ECS
archetype filtering, **so that** proximity triggers, line-of-sight checks, and area-of-effect
abilities retrieve only entities matching both spatial and component criteria.
