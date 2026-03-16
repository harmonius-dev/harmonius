# Spatial Index Test Cases

Companion test cases for [spatial-index.md](spatial-index.md).

## Unit Tests

### TC-1.9.4.1 AABB Intersection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AABB(0,0,0)-(1,1,1) vs AABB(0.5,0.5,0.5)-(1.5,1.5,1.5) | Intersects = true | R-1.9.4 |
| 2 | AABB(0,0,0)-(1,1,1) vs AABB(2,2,2)-(3,3,3) | Intersects = false | R-1.9.4 |
| 3 | Ray origin=(0,0,0) dir=(1,0,0) vs AABB(5,−1,−1)-(6,1,1) | Hit at t=5.0 | R-1.9.4 |
| 4 | Sphere center=(0,0,0) r=1 vs AABB(0.5,0,0)-(2,1,1) | Intersects = true | R-1.9.4 |
| 5 | Frustum covering (−10,−10,0)-(10,10,100) vs AABB inside | Intersects = true | R-1.9.4 |
| 6 | Frustum vs AABB completely outside | Intersects = false | R-1.9.4 |

### TC-1.9.1.1 BVH Insert Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 1,000 entities with random AABBs | All 1,000 queryable | R-1.9.1 |
| 2 | Remove 500 entities | 500 remaining, all queryable | R-1.9.1 |
| 3 | Verify BVH invariants | All leaves reachable, parent AABBs enclose children | R-1.9.1 |

### TC-1.9.1.2 BVH SAH Quality

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build BVH from 10K random AABBs | BVH constructed | R-1.9.1a |
| 2 | Compute SAH cost | Within expected bounds for random distribution | R-1.9.1a |

### TC-1.9.2.1 BVH Incremental Refit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 10K entities, move 100 | Refit completes | R-1.9.2 |
| 2 | Verify update scope | Only moved nodes and ancestors updated | R-1.9.2 |

### TC-1.9.2.2 BVH Fat AABB Skip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move entity by delta < fat AABB margin | No tree structure change | R-1.9.2 |

### TC-1.9.2.3 BVH Batch Insert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Batch-insert 1,000 entities | All 1,000 queryable | R-1.9.2 |
| 2 | Measure SAH quality | Comparable to 1,000 incremental single-inserts | R-1.9.2 |

### TC-1.9.1.3 BVH Rebuild Quality

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Degrade BVH via 1,000 frames of random movement | SAH quality degraded | R-1.9.1a |
| 2 | Full rebuild | SAH quality within 1.1x of fresh build | R-1.9.1a |

### TC-1.9.1.4 BVH Stale Handle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove entity, attempt update with old handle | `StaleHandle` error | R-1.9.1 |

### TC-1.9.1.5 BVH Memory Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 1M entities | All inserted | R-1.9.1a |
| 2 | Measure total BVH memory | Under 64 bytes per entity | R-1.9.1a |

### TC-1.9.3.1 Octree Insert Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 10K entities into octree | All inserted | R-1.9.3 |
| 2 | Query cells and ranges | Results match brute-force reference | R-1.9.3 |

### TC-1.9.3.2 Octree Cross-Cell Move

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move entity across cell boundary | Old cell loses entity | R-1.9.3 |
| 2 | Query new cell | New cell contains entity | R-1.9.3 |

### TC-1.9.3.3 Grid Insert Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 10K entities into 2D grid | All inserted | R-1.9.3 |
| 2 | Radius query | Results match brute-force reference | R-1.9.3 |

### TC-1.9.3.4 Grid Boundary

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert entity at exact grid cell boundary | Correct cell assignment (no ambiguity) | R-1.9.3 |

### TC-1.9.4.2 Ray Cast Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1,000 random rays against 10K entities | All hits recorded | R-1.9.4 |
| 2 | Compare to brute-force ray-AABB test | Zero false negatives, zero false positives | R-1.9.4 |

### TC-1.9.4.3 Frustum Cull Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Frustum query against 10K entities | Result set computed | R-1.9.4 |
| 2 | Compare to brute-force frustum-AABB | Zero false negatives, zero false positives | R-1.9.4 |

### TC-1.9.4.4 KNN Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | k-NN query k=10 against 10K entities | 10 entities returned | R-1.9.4 |
| 2 | Compare to brute-force distance sort | Returned entities are the 10 closest | R-1.9.4 |

### TC-1.9.4.5 Layer Filtering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 100 entities on layer 1, 100 on layer 2 | All inserted | R-1.9.4 |
| 2 | Query with layer 1 mask only | Returns exactly 100 layer-1 entities | R-1.9.4 |

### TC-1.9.4.6 Empty Index Queries

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray cast against empty BVH | Empty result set (no panic) | R-1.9.4a |
| 2 | Frustum cull against empty BVH | Empty result set (no panic) | R-1.9.4a |
| 3 | k-NN against empty BVH | Empty result set (no panic) | R-1.9.4a |
| 4 | Overlap against empty BVH | Empty result set (no panic) | R-1.9.4a |

### TC-1.9.4.7 Query Sort Nearest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray cast with `QuerySort::Nearest` against 100 entities | Results ordered by ascending distance | R-1.9.4 |

## Integration Tests

### TC-1.9.1.I1 Shared BVH Cross-Subsystem

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move entity via Transform system | Position updated | R-1.9.1 |
| 2 | Query from physics broadphase | Sees updated position | R-1.9.1 |
| 3 | Query from rendering culling | Sees updated position | R-1.9.1 |
| 4 | Query from AI perception | Sees updated position (same frame) | R-1.9.1 |

### TC-1.9.6.I1 Physics Broadphase Shared

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1,000 overlapping collider pairs | All pairs created | R-1.9.6 |
| 2 | Physics broadphase via shared BVH | All 1,000 pairs detected | R-1.9.6 |
| 3 | Verify memory | No separate broadphase allocation | R-1.9.6 |

### TC-1.9.7.I1 Rendering Frustum Shared

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K entities, camera covering ~50% | ~5K marked visible | R-1.9.7 |
| 2 | Shadow cascade frustum | Different visible set than main camera | R-1.9.7 |

### TC-1.9.8.I1 Network Relevancy Grid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K entities, 2 players at different positions | Interest sets computed | R-1.9.8 |
| 2 | Verify each player's set | Contains only entities within radius | R-1.9.8 |

### TC-1.9.9.I1 AI Perception Sight Cone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 AI agents with sight cones, 500 targets | Sight cone queries run | R-1.9.9 |
| 2 | Verify results | Only entities within angular + distance bounds | R-1.9.9 |

### TC-1.9.2.I1 Incremental vs Full Build

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1M entities, move 1% per frame, 100 frames | Incremental updates applied | R-1.9.2 |
| 2 | Compare queries to full-rebuild reference | Identical result sets | R-1.9.2 |

### TC-1.9.1.I2 Parallel Consumer Safety

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Physics, rendering, AI query in parallel | All queries complete | R-1.9.1 |
| 2 | Run under ThreadSanitizer | Zero data races | R-1.9.1 |

### TC-1.9.5.I1 Batch Matches Sequential

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 1,000 queries as batch | Batch results computed | R-1.9.5 |
| 2 | Submit same 1,000 queries sequentially | Sequential results computed | R-1.9.5 |
| 3 | Compare result sets | Identical | R-1.9.5 |

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
