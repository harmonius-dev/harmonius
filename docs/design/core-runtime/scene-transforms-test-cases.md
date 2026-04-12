# Scene & Transforms Test Cases

Companion test cases for [scene-transforms.md](scene-transforms.md).

## Unit Tests

### TC-1.2.4.1 Transform Identity

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |

1. **#1** — `Transform::IDENTITY.local_matrix()`
   - **Expected:** `Mat4::IDENTITY`

### TC-1.2.4.2 Transform Compose TRS

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |

1. **#1** — T=(1,2,3), R=45 deg Y, S=(2,2,2)
   - **Expected:** `local_matrix()` matches T * R * S reference

### TC-1.2.4.3 Global Transform Decompose

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |
| 2 | R-1.2.4     |

1. **#1** — Compose T=(5,0,0), R=90 deg Z, S=(1,1,1)
   - **Expected:** GlobalTransform set
2. **#2** — Decompose back to T, R, S
   - **Expected:** T=(5,0,0), R=90 deg Z, S=(1,1,1)

### TC-1.2.1.1 Hierarchy Single Parent

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |

1. **#1** — Set parent of child to A
   - **Expected:** `child.parent() == A`
2. **#2** — Set parent of child to B
   - **Expected:** `child.parent() == B`, A's children excludes child

### TC-1.2.1.2 Children Ordering

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |
| 3 | R-1.2.1     |

1. **#1** — Add children C1, C2, C3 to parent
   - **Expected:** Order: [C1, C2, C3]
2. **#2** — Remove C2
   - **Expected:** Order: [C1, C3]
3. **#3** — Add C4
   - **Expected:** Order: [C1, C3, C4]

### TC-1.2.1.3 Set Parent Command

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |
| 3 | R-1.2.1     |

1. **#1** — `set_parent(child, parent)` via command buffer
   - **Expected:** Parent component added to child
2. **#2** — Check parent's Children
   - **Expected:** Child present in list
3. **#3** — Check events
   - **Expected:** HierarchyEvent::ChildAdded emitted

### TC-1.2.1.4 Remove Parent Command

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |
| 3 | R-1.2.1     |

1. **#1** — `remove_parent(child)` via command buffer
   - **Expected:** Parent component removed from child
2. **#2** — Check old parent's Children
   - **Expected:** Child absent
3. **#3** — Check events
   - **Expected:** HierarchyEvent::ChildRemoved emitted

### TC-1.2.1.5 Reparent Child

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |

1. **#1** — Child under parent A, reparent to B
   - **Expected:** A's children excludes child
2. **#2** — Check B's children
   - **Expected:** Child present in B's list

### TC-1.2.3.1 Cascade Despawn

| # | Requirement |
|---|-------------|
| 1 | R-1.2.3     |
| 2 | R-1.2.3     |

1. **#1** — 3-level hierarchy: root -> mid -> leaf
   - **Expected:** All 3 entities alive
2. **#2** — Despawn root
   - **Expected:** All 3 entities destroyed

### TC-1.2.3.2 Orphan On Delete

| # | Requirement |
|---|-------------|
| 1 | R-1.2.3     |
| 2 | R-1.2.3     |

1. **#1** — Parent with 2 children, `despawn_orphaning(parent)`
   - **Expected:** Parent destroyed
2. **#2** — Check children
   - **Expected:** Both alive, reparented to root (no Parent)

### TC-1.2.3.3 No Orphaned Entities

| # | Requirement |
|---|-------------|
| 1 | R-1.2.3     |
| 2 | R-1.2.3     |

1. **#1** — Cascade-delete 5-level hierarchy
   - **Expected:** All descendants destroyed
2. **#2** — Query entities with Parent pointing to dead entity
   - **Expected:** Zero results

### TC-1.2.2.1 DFS Traversal Order

| # | Requirement |
|---|-------------|
| 1 | R-1.2.2     |

1. **#1** — 5-level tree: R->[A->[D,E], B->[F], C]
   - **Expected:** DFS order: R, A, D, E, B, F, C

### TC-1.2.2.2 BFS Traversal Order

| # | Requirement |
|---|-------------|
| 1 | R-1.2.2     |

1. **#1** — 5-level tree: R->[A->[D,E], B->[F], C]
   - **Expected:** BFS order: R, A, B, C, D, E, F

### TC-1.2.2.3 Traversal Skip Subtree

| # | Requirement |
|---|-------------|
| 1 | R-1.2.2     |

1. **#1** — DFS on R->[A->[D,E], B], skip subtree at A
   - **Expected:** Visited: R, A, B (D, E skipped)

### TC-1.2.2.4 Traversal Early Termination

| # | Requirement |
|---|-------------|
| 1 | R-1.2.2     |

1. **#1** — DFS on 10-node tree, break after 3
   - **Expected:** Exactly 3 nodes visited

### TC-1.2.2.5 Traversal 256 Depth

| # | Requirement |
|---|-------------|
| 1 | R-1.2.2a    |
| 2 | R-1.2.2a    |

1. **#1** — 256-level chain
   - **Expected:** Traversal completes
2. **#2** — Check allocations
   - **Expected:** Zero heap allocations

### TC-1.2.2.6 Traversal 300 Depth Fallback

| # | Requirement |
|---|-------------|
| 1 | R-1.2.2a    |
| 2 | R-1.2.2a    |

1. **#1** — 300-level chain
   - **Expected:** Traversal completes (heap fallback)
2. **#2** — Verify correctness
   - **Expected:** All 300 nodes visited in order

### TC-1.2.2.7 Traversal Depth Warning

| # | Requirement |
|---|-------------|
| 1 | R-1.2.2a    |

1. **#1** — 129-level tree
   - **Expected:** Diagnostic warning fires at depth > 128

### TC-1.2.4.4 Propagation Root Only

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |

1. **#1** — Root entity with T=(1,0,0), no parent
   - **Expected:** `GlobalTransform == Transform.local_matrix()`

### TC-1.2.4.5 Propagation Two Levels

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |
| 2 | R-1.2.4     |

1. **#1** — Parent T=(10,0,0), Child T=(0,5,0)
   - **Expected:** Child GlobalTransform = parent.global * child.local
2. **#2** — Extract child world translation
   - **Expected:** (10, 5, 0)

### TC-1.2.4.6 Propagation Deep Chain

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |
| 2 | R-1.2.4     |

1. **#1** — 50-level chain, each T=(1,0,0)
   - **Expected:** Leaf world translation = (50, 0, 0)
2. **#2** — Compare to serial reference computation
   - **Expected:** Bit-exact match

### TC-1.2.4.7 Propagation No Stack Overflow

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |

1. **#1** — 1000-level chain, run propagation
   - **Expected:** Completes without stack overflow

### TC-1.2.5.1 Dirty Tracking Unchanged

| # | Requirement |
|---|-------------|
| 1 | R-1.2.5     |

1. **#1** — Entity with unmodified Transform
   - **Expected:** GlobalTransform not recomputed this frame

### TC-1.2.5.2 Dirty Leaf Only

| # | Requirement |
|---|-------------|
| 1 | R-1.2.5     |

1. **#1** — 5-level tree, modify leaf's Transform only
   - **Expected:** Only leaf's GlobalTransform recomputed

### TC-1.2.5.3 Dirty Root Propagates

| # | Requirement |
|---|-------------|
| 1 | R-1.2.5     |

1. **#1** — 5-level tree, modify root's Transform
   - **Expected:** All descendants' GlobalTransforms recomputed

### TC-1.2.5.4 Dirty No False Marks

| # | Requirement |
|---|-------------|
| 1 | R-1.2.5     |

1. **#1** — Read Transform via `&Transform` (immutable)
   - **Expected:** Dirty flag not set

### TC-1.2.1.6 Scene Serialize Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |

1. **#1** — Scene with hierarchy and transforms, serialize
   - **Expected:** Bytes produced
2. **#2** — Deserialize
   - **Expected:** Identical hierarchy and transform values

### TC-1.2.1.7 Scene Entity Remap

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |

1. **#1** — Spawn scene into populated world
   - **Expected:** Scene entities created
2. **#2** — Verify entity references in components
   - **Expected:** All remapped to new IDs

### TC-1.2.1.8 Scene Spawn As Child

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |

1. **#1** — Spawn scene under target parent
   - **Expected:** Scene root becomes child of target

### TC-1.2.1.9 Scene Cyclic Detection

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |

1. **#1** — Attempt to serialize cyclic hierarchy
   - **Expected:** `CyclicHierarchy` error returned

### TC-1.1.28.1 Transform Propagation Run Criteria Ordering

| # | Requirement |
|---|-------------|
| 1 | R-1.1.28    |
| 2 | R-1.1.28    |

1. **#1** — Schedule transform propagation with `after(hierarchy_apply)` run criterion
   - **Expected:** Propagation system runs strictly after hierarchy command application
2. **#2** — Record system execution order over 100 frames
   - **Expected:** Hierarchy apply precedes propagation every frame, zero inversions

### TC-1.2.7.1 Delegate To Shared BVH Resource

| # | Requirement |
|---|-------------|
| 1 | R-1.2.7     |
| 2 | R-1.2.7     |

1. **#1** — Query scene for entities in AABB via `world.spatial_query_aabb(aabb)`
   - **Expected:** Call resolves through shared `BvhResource` (verified via resource borrow log)
2. **#2** — Swap `BvhResource` with instrumented fake and re-run query
   - **Expected:** Fake records the query, scene module does not maintain its own index

### TC-1.9.1.1 Single BVH ECS Resource Uniqueness

| # | Requirement |
|---|-------------|
| 1 | R-1.9.1     |
| 2 | R-1.9.1     |

1. **#1** — Inspect world after startup
   - **Expected:** Exactly one `BvhResource` instance registered
2. **#2** — Run propagation, physics, and rendering extract in sequence
   - **Expected:** All three subsystems borrow the same `BvhResource` handle

## Integration Tests

### TC-1.2.4.I1 Parallel Propagation Correctness

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4     |
| 2 | R-1.2.4     |

1. **#1** — 100K entities, mixed depths 1-50, parallel propagation
   - **Expected:** All GlobalTransforms computed
2. **#2** — Compare to serial reference
   - **Expected:** Bit-exact match for all entities

### TC-1.2.4.I2 Propagation Same Frame

| # | Requirement |
|---|-------------|
| 1 | R-1.2.4a    |
| 2 | R-1.2.4a    |

1. **#1** — Modify Transform in Update phase
   - **Expected:** Change applied
2. **#2** — Read GlobalTransform in PreRender phase
   - **Expected:** Reflects Update modification (no 1-frame lag)

### TC-1.2.1.I1 Hierarchy During Parallel

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |

1. **#1** — Hierarchy modifications via commands during parallel iteration
   - **Expected:** All commands applied after flush
2. **#2** — Run under ThreadSanitizer
   - **Expected:** Zero data races

### TC-1.2.6.I1 Spatial Index After Propagation

| # | Requirement |
|---|-------------|
| 1 | R-1.2.6     |

1. **#1** — Move entities, run propagation, then spatial query
   - **Expected:** Query returns entities at correct world positions

### TC-1.2.1.I2 Scene With Entity Templates

| # | Requirement |
|---|-------------|
| 1 | R-1.2.1     |
| 2 | R-1.2.1     |

1. **#1** — Scene containing entity template instances
   - **Expected:** Hierarchy preserved
2. **#2** — Verify overrides
   - **Expected:** Entity template overrides intact

### TC-1.2.1.I3 Game Developer Builds Scene Hierarchy Via ECS

| # | Requirement |
|---|-------------|
| 1 | US-1.2.1    |
| 2 | US-1.2.1    |

1. **#1** — Game developer spawns a vehicle root with 6 child wheel entities using
   `Commands::spawn_with_children`
   - **Expected:** Six `Parent` components and one `Children` component created via ECS API
2. **#2** — Query `Children(vehicle)` after command flush
   - **Expected:** All six wheels returned in insertion order, accessible through standard ECS
     queries

### TC-1.2.2.I1 Engine Developer Traverses Hierarchy Allocation Free

| # | Requirement |
|---|-------------|
| 1 | US-1.2.2    |
| 2 | US-1.2.2    |

1. **#1** — Build 1K-entity tree (depth 10) and run `traverse_dfs` and `traverse_bfs` closures
   - **Expected:** Both visit every entity exactly once
2. **#2** — Wrap calls in allocation counter (bumpalo probe or `#[global_allocator]` counter)
   - **Expected:** Zero heap allocations recorded during traversal

### TC-1.2.3.I1 Engine Tester Benchmarks Traversal Over 10K Hierarchy

| # | Requirement |
|---|-------------|
| 1 | US-1.2.3    |
| 2 | US-1.2.3    |

1. **#1** — Construct 10,000-entity hierarchy with branching factor 4 and invoke criterion bench
   over DFS traversal
   - **Expected:** Benchmark harness runs and records samples, producing a stable timing report
2. **#2** — Re-run benchmark five times on same fixture
   - **Expected:** Sample variance < 5 %, confirming benchmark is deterministic for CI tracking

### TC-1.2.4.I3 Cascade Despawn From Parent Deletion

| # | Requirement |
|---|-------------|
| 1 | US-1.2.4    |
| 2 | US-1.2.4    |

1. **#1** — Game developer calls `world.despawn(parent)` on a 3-level vehicle (chassis → wheels →
   bolts)
   - **Expected:** Parent, all children, and all grand-children removed in one command flush
2. **#2** — Query world for any entity referencing the despawned parent
   - **Expected:** Zero entities remain, no dangling `Parent` components

### TC-1.2.5.I1 Zero Orphans After Parent Delete

| # | Requirement |
|---|-------------|
| 1 | US-1.2.5    |
| 2 | US-1.2.5    |

1. **#1** — Engine tester deletes parent entity in 5-level chain and runs post-condition sweep
   - **Expected:** Sweep query `Q<&Parent>` yields no entity whose parent is dead
2. **#2** — Re-run sweep after next frame's command flush
   - **Expected:** Still zero orphans, confirming deterministic cascade

### TC-1.2.6.I2 Engine Developer Reads Propagated World Transforms

| # | Requirement |
|---|-------------|
| 1 | US-1.2.6    |
| 2 | US-1.2.6    |

1. **#1** — Engine developer writes a physics system that reads `&GlobalTransform` after propagation
   phase
   - **Expected:** Read returns world-space matrix computed that frame by the propagation system
2. **#2** — Mutate parent `Transform`, re-run propagation, re-read child `GlobalTransform`
   - **Expected:** Child world position reflects parent's new local transform within the same frame

### TC-1.2.7.I1 Tester Verifies Propagation Matches Reference

| # | Requirement |
|---|-------------|
| 1 | US-1.2.7    |
| 2 | US-1.2.7    |

1. **#1** — Engine tester runs propagation on 10K-entity fixture and captures all `GlobalTransform`
   values
   - **Expected:** Every value matches a naive serial recursive reference implementation bit-exact
2. **#2** — Re-run with randomised parallel scheduling seed
   - **Expected:** Results remain bit-identical across seeds, confirming determinism

### TC-1.2.8.I1 Propagation Skips Clean Subtrees

| # | Requirement |
|---|-------------|
| 1 | US-1.2.8    |
| 2 | US-1.2.8    |

1. **#1** — Engine developer instruments propagation with per-entity recompute counter and runs one
   frame with only 1 % of transforms dirty
   - **Expected:** Recompute counter ≤ 1 % of entities plus their descendants
2. **#2** — Same fixture with zero dirty transforms
   - **Expected:** Counter records zero recomputations

### TC-1.2.9.I1 Benchmark Propagation Across Hierarchy Depths

| # | Requirement |
|---|-------------|
| 1 | US-1.2.9    |
| 2 | US-1.2.9    |

1. **#1** — Engine tester runs criterion bench on 1M entities with depths 1, 10, 50, 100
   - **Expected:** Four bench samples produced, each reporting mean time under perf budget
2. **#2** — Compare depth-1 and depth-100 sample distributions
   - **Expected:** Both complete within the R-1.2.4a 1 ms budget for 2M entities

### TC-1.2.10.I1 Spatial Acceleration For Broadphase

| # | Requirement |
|---|-------------|
| 1 | US-1.2.10   |
| 2 | US-1.2.10   |

1. **#1** — Engine developer inserts 10K entities into BVH and queries for overlap pairs
   - **Expected:** All overlapping pairs returned; broadphase uses BVH not brute-force O(n²) scan
2. **#2** — Compare results to brute-force pair list
   - **Expected:** Set equality, zero missed or extra pairs

### TC-1.2.11.I1 Game Developer Uses Spatial Query API

| # | Requirement |
|---|-------------|
| 1 | US-1.2.11   |
| 2 | US-1.2.11   |

1. **#1** — Game developer calls `world.spatial_query_point(p)` for explosion damage
   - **Expected:** Returns all entity handles whose AABB contains `p`
2. **#2** — Game developer calls `world.spatial_query_aabb(area)` for trigger volume
   - **Expected:** Returns all entity handles whose AABB intersects `area`

### TC-1.2.12.I1 Technical Artist Views Local And World Transforms In Editor

| # | Requirement |
|---|-------------|
| 1 | US-1.2.12   |
| 2 | US-1.2.12   |

1. **#1** — Technical artist selects an entity in editor inspector and inspects Transform panel
   - **Expected:** Panel shows local T/R/S and world T/R/S side by side in real time
2. **#2** — Artist drags local translation field
   - **Expected:** Local value updates immediately, world value updates after propagation each frame

<!-- THIN: design section lacks detail -->
### TC-1.2.13.I1 Editor Transform Widgets Match Entity State

| # | Requirement |
|---|-------------|
| 1 | US-1.2.13   |
| 2 | US-1.2.13   |

1. **#1** — Editor inspector binds Transform widget to selected entity
   - **Expected:** Widget values match entity's `Transform` component on every selection change
2. **#2** — External system mutates the entity's `Transform` while selected
   - **Expected:** Widget refreshes on next frame to reflect the new values

## Benchmarks

### TC-1.2.4.B1 Propagation 2M Entities

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2M entities, 4 cores | Time | Under 1 ms | R-1.2.4a |

### TC-1.2.5.B1 Propagation 1M 1% Dirty

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M entities, 1% dirty | Time | Under 0.5 ms | R-1.2.5 |

### TC-1.2.4.B2 Propagation 100K 100% Dirty

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100K entities, 100% dirty | Time | Under 0.1 ms | R-1.2.4a |

### TC-1.2.2.B1 DFS Traversal 100K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | DFS traversal 100K entities | Allocations | Zero | R-1.2.2 |

### TC-1.2.2.B2 BFS Traversal 100K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | BFS traversal 100K entities | Allocations | Zero | R-1.2.2 |

### TC-1.2.1.B1 Set Parent Commands

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10K `set_parent` commands per frame | Time | Under 0.2 ms | R-1.2.1 |

### TC-1.2.1.B2 Scene Serialize 10K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Serialize scene with 10K entities | Time | Under 5 ms | R-1.2.1 |

### TC-1.2.1.B3 Scene Spawn 10K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Spawn scene with 10K entities | Time | Under 5 ms | R-1.2.1 |

### TC-1.2.1.B4 Entity Remap 10K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Remap 10K entity references | Time | Under 0.5 ms | R-1.2.1 |
