# Spatial Indexing User Stories

## Acceleration Structures

## US-1.9.1 Share a Single BVH Across All Subsystems

**As an** engine developer (P-26), **I want** a single BVH maintained as an ECS resource shared
across physics, rendering, networking, AI, and gameplay, **so that** all subsystems use one spatial
structure instead of maintaining redundant copies.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Single BVH updated by dedicated system | F-1.9.1 | R-1.9.1 |
| All subsystems read from shared BVH | F-1.9.1 | R-1.9.1 |
| Platform-appropriate entity limits and memory budgets | F-1.9.1 | R-1.9.1 |

## US-1.9.2 Understand the Shared Spatial Index in the Editor

**As a** designer (P-5), **I want** to visualize the shared spatial index in the editor via debug
overlays showing BVH bounding boxes and grid cells, **so that** I can understand how entities are
spatially organized and debug query behavior.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| BVH bounding boxes visualized as debug overlay | F-1.9.1 | R-1.9.1 |
| Grid/octree cells visualized when enabled | F-1.9.3 | R-1.9.3 |
| Query results highlighted in viewport | F-1.9.4 | R-1.9.4 |

## US-1.9.3 Benchmark BVH Memory and Query Performance at Scale

**As an** engine tester (P-27), **I want** to benchmark BVH memory usage and query latency at
platform entity limits (50K mobile to 5M+ high-end PC), **so that** I can verify the spatial index
meets its memory and performance budgets.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Mobile: 50K entities in ~2 MB | F-1.9.1 | R-1.9.1 |
| Desktop: 1M+ entities in ~40 MB | F-1.9.1 | R-1.9.1 |
| Sub-millisecond query times at platform limits | F-1.9.1 | R-1.9.1 |

## US-1.9.4 Update BVH Incrementally Using Change Detection

**As an** engine developer (P-26), **I want** the BVH updated incrementally using ECS change
detection on Transform components, **so that** update cost is proportional to moved entities rather
than total entity count.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Moved entities refitted in-place or reinserted | F-1.9.2 | R-1.9.2 |
| Newly spawned entities batch-inserted | F-1.9.2 | R-1.9.2 |
| Update cost proportional to moved entities | F-1.9.2 | R-1.9.2 |

## US-1.9.5 Benchmark Incremental BVH Update Cost

**As an** engine tester (P-27), **I want** to benchmark incremental BVH update cost with varying
ratios of stationary to moving entities, **so that** I can verify update time scales with moved
entity count, not total count.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 1M entities, 1% moving: update under 0.5ms | F-1.9.2 | R-1.9.2 |
| 1M entities, 100% moving: update within refit budget | F-1.9.2 | R-1.9.2 |
| No full rebuild unless entity distribution changes radically | F-1.9.2 | R-1.9.2 |

## US-1.9.6 Use Coarse Grid or Octree for Cell-Based Queries

**As a** game developer (P-15), **I want** an optional coarse-grained spatial index (uniform grid or
octree) alongside the BVH, **so that** network area-of-interest filtering and AI crowd density
queries use the most efficient structure for their access pattern.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Grid or octree stored as ECS resource alongside BVH | F-1.9.3 | R-1.9.3 |
| Cell membership updated by spatial index system | F-1.9.3 | R-1.9.3 |
| Platform-appropriate grid size and depth limits | F-1.9.3 | R-1.9.3 |

## Query Interface

## US-1.9.7 Query the Spatial Index With a Unified API

**As a** game developer (P-15), **I want** a single API for ray casts, shape casts, overlap tests,
nearest-neighbor, and frustum queries that accepts ECS component filters, **so that** all subsystems
use a consistent query interface.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Ray cast, shape cast, overlap, k-NN, frustum queries | F-1.9.4 | R-1.9.4 |
| Results as ECS Entity handles with hit metadata | F-1.9.4 | R-1.9.4 |
| ECS filters (With<T>, Without<T>) accepted | F-1.9.4 | R-1.9.4 |

## US-1.9.8 Submit Batch Spatial Queries in Parallel

**As an** engine developer (P-26), **I want** to submit batches of spatial queries that execute in
parallel across worker threads with SIMD acceleration, **so that** server ticks with hundreds of AI
and ability checks complete within frame budgets.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Batch queries execute in parallel via job system | F-1.9.5 | R-1.9.5 |
| SIMD-accelerated ray-BVH intersection | F-1.9.5 | R-1.9.5 |
| Platform-appropriate batch and worker limits | F-1.9.5 | R-1.9.5 |

## US-1.9.9 Benchmark Batch Spatial Query Throughput

**As an** engine tester (P-27), **I want** to benchmark batch spatial query throughput at platform
limits (64 mobile to 4096+ high-end PC), **so that** I can verify server tick budgets are met during
worst-case AI and ability processing.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Mobile: 64 queries per batch within budget | F-1.9.5 | R-1.9.5 |
| Desktop: 1024+ queries per batch within budget | F-1.9.5 | R-1.9.5 |
| SIMD acceleration measurably improves throughput | F-1.9.5 | R-1.9.5 |

## Consumer Integration

## US-1.9.10 Use Shared BVH for Physics Broadphase

**As an** engine developer (P-26), **I want** the physics broadphase to read from the shared BVH
filtered by collision layers, **so that** collision detection uses the same spatial data as
rendering and no separate broadphase structure is allocated.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Physics queries shared BVH for overlapping AABB pairs | F-1.9.6 | R-1.9.6 |
| Results filtered by CollisionLayers components | F-1.9.6 | R-1.9.6 |
| No separate broadphase data structure | F-1.9.6 | R-1.9.6 |

## US-1.9.11 Verify Physics and Rendering Agree on Entity Positions

**As an** engine tester (P-27), **I want** to verify that physics broadphase and rendering culling
produce consistent results from the shared BVH within the same frame, **so that** no entity is
visible but non-collidable or vice versa.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Same entity positions for physics and rendering in same frame | F-1.9.6, F-1.9.7 | R-1.9.6, R-1.9.7 |
| No frame-lag between spatial index and subsystem queries | F-1.9.6, F-1.9.7 | R-1.9.6, R-1.9.7 |

## US-1.9.12 Use Shared BVH for Rendering Frustum Culling

**As an** engine developer (P-26), **I want** frustum culling to read from the shared BVH for all
views (main camera, shadow cascades, reflection probes), **so that** the renderer shares incremental
updates and does not rebuild a separate culling hierarchy.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Main camera, shadow cascade, and reflection probe culling | F-1.9.7 | R-1.9.7 |
| Visibility results as marker components or bitsets | F-1.9.7 | R-1.9.7 |
| Platform-appropriate view count limits | F-1.9.7 | R-1.9.7 |

## US-1.9.13 Filter Network Replication by Spatial Relevancy

**As a** game developer (P-15), **I want** the network relevancy system to use the shared spatial
index for area-of-interest filtering, **so that** only entities within each player's relevancy
radius are replicated, saving bandwidth in large multiplayer worlds.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Range queries compute per-player area-of-interest sets | F-1.9.8 | R-1.9.8 |
| Grid or octree used for cell-based filtering | F-1.9.8 | R-1.9.8 |
| Consistent spatial reasoning with physics and rendering | F-1.9.8 | R-1.9.8 |

## US-1.9.14 Verify Network Relevancy Spatial Consistency

**As an** engine tester (P-27), **I want** to verify that network relevancy area-of-interest sets
are spatially consistent with physics and rendering, **so that** replicated entities match what the
player sees and collides with.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Replicated entities match visible entities within radius | F-1.9.8 | R-1.9.8 |
| No entities replicated outside relevancy radius | F-1.9.8 | R-1.9.8 |
| Radius changes reflected within one tick | F-1.9.8 | R-1.9.8 |

## US-1.9.15 Query Shared Index for AI Sight and Hearing

**As a** game developer (P-15), **I want** AI perception systems (sight cones, hearing radii) and
gameplay spatial queries (AoE abilities, trigger volumes) to read from the shared index, **so that**
AI and gameplay systems agree with physics on entity positions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Sight cone queries via frustum test on BVH | F-1.9.9 | R-1.9.9 |
| Hearing radius queries via sphere overlap | F-1.9.9 | R-1.9.9 |
| AoE and trigger volumes use unified query API | F-1.9.9 | R-1.9.9 |

## US-1.9.16 Verify AI Perception Queries Return Correct Entities

**As an** engine tester (P-27), **I want** to verify that AI sight cone and hearing radius queries
return exactly the entities within the perception volume, **so that** AI does not perceive entities
it should not see or miss entities it should detect.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Sight cone includes only entities within frustum | F-1.9.9 | R-1.9.9 |
| Hearing radius includes only entities within sphere | F-1.9.9 | R-1.9.9 |
| No false positives or negatives in perception results | F-1.9.9 | R-1.9.9 |
