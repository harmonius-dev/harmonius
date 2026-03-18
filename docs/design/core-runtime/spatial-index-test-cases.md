# Spatial Index Test Cases

Companion test cases for [spatial-index.md](spatial-index.md).

## Unit Tests

### TC-1.9.4.1 AABB Intersection

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4     |
| 2 | R-1.9.4     |
| 3 | R-1.9.4     |
| 4 | R-1.9.4     |
| 5 | R-1.9.4     |
| 6 | R-1.9.4     |

1. **#1** — AABB(0,0,0)-(1,1,1) vs AABB(0.5,0.5,0.5)-(1.5,1.5,1.5)
   - **Expected:** Intersects = true
2. **#2** — AABB(0,0,0)-(1,1,1) vs AABB(2,2,2)-(3,3,3)
   - **Expected:** Intersects = false
3. **#3** — Ray origin=(0,0,0) dir=(1,0,0) vs AABB(5,−1,−1)-(6,1,1)
   - **Expected:** Hit at t=5.0
4. **#4** — Sphere center=(0,0,0) r=1 vs AABB(0.5,0,0)-(2,1,1)
   - **Expected:** Intersects = true
5. **#5** — Frustum covering (−10,−10,0)-(10,10,100) vs AABB inside
   - **Expected:** Intersects = true
6. **#6** — Frustum vs AABB completely outside
   - **Expected:** Intersects = false

### TC-1.9.1.1 BVH Insert Remove

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1     |
| 2 | R-1.9.1     |
| 3 | R-1.9.1     |

1. **#1** — Insert 1,000 entities with random AABBs
   - **Expected:** All 1,000 queryable
2. **#2** — Remove 500 entities
   - **Expected:** 500 remaining, all queryable
3. **#3** — Verify BVH invariants
   - **Expected:** All leaves reachable, parent AABBs enclose children

### TC-1.9.1.2 BVH SAH Quality

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1a    |
| 2 | R-1.9.1a    |

1. **#1** — Build BVH from 10K random AABBs
   - **Expected:** BVH constructed
2. **#2** — Compute SAH cost
   - **Expected:** Within expected bounds for random distribution

### TC-1.9.2.1 BVH Incremental Refit

| # | Requirement |
|---|-------------|
| 1 | R-1.9.2     |
| 2 | R-1.9.2     |

1. **#1** — Insert 10K entities, move 100
   - **Expected:** Refit completes
2. **#2** — Verify update scope
   - **Expected:** Only moved nodes and ancestors updated

### TC-1.9.2.2 BVH Fat AABB Skip

| # | Requirement |
|---|-------------|
| 1 | R-1.9.2     |

1. **#1** — Move entity by delta < fat AABB margin
   - **Expected:** No tree structure change

### TC-1.9.2.3 BVH Batch Insert

| # | Requirement |
|---|-------------|
| 1 | R-1.9.2     |
| 2 | R-1.9.2     |

1. **#1** — Batch-insert 1,000 entities
   - **Expected:** All 1,000 queryable
2. **#2** — Measure SAH quality
   - **Expected:** Comparable to 1,000 incremental single-inserts

### TC-1.9.1.3 BVH Rebuild Quality

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1a    |
| 2 | R-1.9.1a    |

1. **#1** — Degrade BVH via 1,000 frames of random movement
   - **Expected:** SAH quality degraded
2. **#2** — Full rebuild
   - **Expected:** SAH quality within 1.1x of fresh build

### TC-1.9.1.4 BVH Stale Handle

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1     |

1. **#1** — Remove entity, attempt update with old handle
   - **Expected:** `StaleHandle` error

### TC-1.9.1.5 BVH Memory Budget

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1a    |
| 2 | R-1.9.1a    |

1. **#1** — Insert 1M entities
   - **Expected:** All inserted
2. **#2** — Measure total BVH memory
   - **Expected:** Under 64 bytes per entity

### TC-1.9.3.1 Octree Insert Query

| # | Requirement |
|---|-------------|
| 1 | R-1.9.3     |
| 2 | R-1.9.3     |

1. **#1** — Insert 10K entities into octree
   - **Expected:** All inserted
2. **#2** — Query cells and ranges
   - **Expected:** Results match brute-force reference

### TC-1.9.3.2 Octree Cross-Cell Move

| # | Requirement |
|---|-------------|
| 1 | R-1.9.3     |
| 2 | R-1.9.3     |

1. **#1** — Move entity across cell boundary
   - **Expected:** Old cell loses entity
2. **#2** — Query new cell
   - **Expected:** New cell contains entity

### TC-1.9.3.3 Grid Insert Query

| # | Requirement |
|---|-------------|
| 1 | R-1.9.3     |
| 2 | R-1.9.3     |

1. **#1** — Insert 10K entities into 2D grid
   - **Expected:** All inserted
2. **#2** — Radius query
   - **Expected:** Results match brute-force reference

### TC-1.9.3.4 Grid Boundary

| # | Requirement |
|---|-------------|
| 1 | R-1.9.3     |

1. **#1** — Insert entity at exact grid cell boundary
   - **Expected:** Correct cell assignment (no ambiguity)

### TC-1.9.4.2 Ray Cast Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4     |
| 2 | R-1.9.4     |

1. **#1** — 1,000 random rays against 10K entities
   - **Expected:** All hits recorded
2. **#2** — Compare to brute-force ray-AABB test
   - **Expected:** Zero false negatives, zero false positives

### TC-1.9.4.3 Frustum Cull Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4     |
| 2 | R-1.9.4     |

1. **#1** — Frustum query against 10K entities
   - **Expected:** Result set computed
2. **#2** — Compare to brute-force frustum-AABB
   - **Expected:** Zero false negatives, zero false positives

### TC-1.9.4.4 KNN Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4     |
| 2 | R-1.9.4     |

1. **#1** — k-NN query k=10 against 10K entities
   - **Expected:** 10 entities returned
2. **#2** — Compare to brute-force distance sort
   - **Expected:** Returned entities are the 10 closest

### TC-1.9.4.5 Layer Filtering

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4     |
| 2 | R-1.9.4     |

1. **#1** — Insert 100 entities on layer 1, 100 on layer 2
   - **Expected:** All inserted
2. **#2** — Query with layer 1 mask only
   - **Expected:** Returns exactly 100 layer-1 entities

### TC-1.9.4.6 Empty Index Queries

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4a    |
| 2 | R-1.9.4a    |
| 3 | R-1.9.4a    |
| 4 | R-1.9.4a    |

1. **#1** — Ray cast against empty BVH
   - **Expected:** Empty result set (no panic)
2. **#2** — Frustum cull against empty BVH
   - **Expected:** Empty result set (no panic)
3. **#3** — k-NN against empty BVH
   - **Expected:** Empty result set (no panic)
4. **#4** — Overlap against empty BVH
   - **Expected:** Empty result set (no panic)

### TC-1.9.4.7 Query Sort Nearest

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4     |

1. **#1** — Ray cast with `QuerySort::Nearest` against 100 entities
   - **Expected:** Results ordered by ascending distance

## Integration Tests

### TC-1.9.1.I1 Shared BVH Cross-Subsystem

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1     |
| 2 | R-1.9.1     |
| 3 | R-1.9.1     |
| 4 | R-1.9.1     |

1. **#1** — Move entity via Transform system
   - **Expected:** Position updated
2. **#2** — Query from physics broadphase
   - **Expected:** Sees updated position
3. **#3** — Query from rendering culling
   - **Expected:** Sees updated position
4. **#4** — Query from AI perception
   - **Expected:** Sees updated position (same frame)

### TC-1.9.6.I1 Physics Broadphase Shared

| # | Requirement |
|---|-------------|
| 1 | R-1.9.6     |
| 2 | R-1.9.6     |
| 3 | R-1.9.6     |

1. **#1** — 1,000 overlapping collider pairs
   - **Expected:** All pairs created
2. **#2** — Physics broadphase via shared BVH
   - **Expected:** All 1,000 pairs detected
3. **#3** — Verify memory
   - **Expected:** No separate broadphase allocation

### TC-1.9.7.I1 Rendering Frustum Shared

| # | Requirement |
|---|-------------|
| 1 | R-1.9.7     |
| 2 | R-1.9.7     |

1. **#1** — 10K entities, camera covering ~50%
   - **Expected:** ~5K marked visible
2. **#2** — Shadow cascade frustum
   - **Expected:** Different visible set than main camera

### TC-1.9.8.I1 Network Relevancy Grid

| # | Requirement |
|---|-------------|
| 1 | R-1.9.8     |
| 2 | R-1.9.8     |

1. **#1** — 10K entities, 2 players at different positions
   - **Expected:** Interest sets computed
2. **#2** — Verify each player's set
   - **Expected:** Contains only entities within radius

### TC-1.9.9.I1 AI Perception Sight Cone

| # | Requirement |
|---|-------------|
| 1 | R-1.9.9     |
| 2 | R-1.9.9     |

1. **#1** — 100 AI agents with sight cones, 500 targets
   - **Expected:** Sight cone queries run
2. **#2** — Verify results
   - **Expected:** Only entities within angular + distance bounds

### TC-1.9.2.I1 Incremental vs Full Build

| # | Requirement |
|---|-------------|
| 1 | R-1.9.2     |
| 2 | R-1.9.2     |

1. **#1** — 1M entities, move 1% per frame, 100 frames
   - **Expected:** Incremental updates applied
2. **#2** — Compare queries to full-rebuild reference
   - **Expected:** Identical result sets

### TC-1.9.1.I2 Parallel Consumer Safety

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1     |
| 2 | R-1.9.1     |

1. **#1** — Physics, rendering, AI query in parallel
   - **Expected:** All queries complete
2. **#2** — Run under ThreadSanitizer
   - **Expected:** Zero data races

### TC-1.9.5.I1 Batch Matches Sequential

| # | Requirement |
|---|-------------|
| 1 | R-1.9.5     |
| 2 | R-1.9.5     |
| 3 | R-1.9.5     |

1. **#1** — Submit 1,000 queries as batch
   - **Expected:** Batch results computed
2. **#2** — Submit same 1,000 queries sequentially
   - **Expected:** Sequential results computed
3. **#3** — Compare result sets
   - **Expected:** Identical

## Benchmarks

### TC-1.9.1.B1 BVH Insert 1M

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Insert 1M entities into BVH | Time | < 500 ms | R-1.9.1 |

### TC-1.9.2.B1 Incremental Update 1% Moved

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M total entities, 1% moved | Time | < 0.5 ms | R-1.9.2 |

### TC-1.9.2.B2 Incremental Update 10% Moved

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M total entities, 10% moved | Time | < 5 ms | R-1.9.2 |

### TC-1.9.4.B1 Single Ray Cast

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single ray cast against 1M entities | Latency | < 10 us | R-1.9.4a |

### TC-1.9.4.B2 Frustum Cull

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Frustum cull against 1M entities | Time | < 500 us | R-1.9.4a |

### TC-1.9.4.B3 KNN

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | k-NN k=10 against 1M entities | Latency | < 50 us | R-1.9.4 |

### TC-1.9.4.B4 Sphere Overlap

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sphere overlap r=50 against 1M entities | Latency | < 100 us | R-1.9.4 |

### TC-1.9.5.B1 Batch Ray Cast 8 Cores

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1,000 ray casts, 1M entities, 8 cores | Speedup vs single-threaded | >= 3x | R-1.9.5 |

### TC-1.9.5.B2 Batch Ray Cast 4 Cores

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1,000 ray casts, 1M entities, 4 cores | Speedup vs single-threaded | >= 3x | R-1.9.5 |

### TC-1.9.1.B2 BVH Memory Per Entity

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M entities in BVH | Memory per entity | <= 64 bytes | R-1.9.1a |

### TC-1.9.1.B3 SAH Quality After Incremental

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | After 1,000 incremental frames | SAH cost ratio | <= 2x fresh build | R-1.9.1a |
