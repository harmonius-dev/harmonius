# Scene & Transforms Test Cases

Companion test cases for [scene-transforms.md](scene-transforms.md).

## Unit Tests

### TC-1.2.4.1 Transform Identity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `Transform::IDENTITY.local_matrix()` | `Mat4::IDENTITY` | R-1.2.4 |

### TC-1.2.4.2 Transform Compose TRS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | T=(1,2,3), R=45 deg Y, S=(2,2,2) | `local_matrix()` matches T * R * S reference | R-1.2.4 |

### TC-1.2.4.3 Global Transform Decompose

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compose T=(5,0,0), R=90 deg Z, S=(1,1,1) | GlobalTransform set | R-1.2.4 |
| 2 | Decompose back to T, R, S | T=(5,0,0), R=90 deg Z, S=(1,1,1) | R-1.2.4 |

### TC-1.2.1.1 Hierarchy Single Parent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set parent of child to A | `child.parent() == A` | R-1.2.1 |
| 2 | Set parent of child to B | `child.parent() == B`, A's children excludes child | R-1.2.1 |

### TC-1.2.1.2 Children Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add children C1, C2, C3 to parent | Order: [C1, C2, C3] | R-1.2.1 |
| 2 | Remove C2 | Order: [C1, C3] | R-1.2.1 |
| 3 | Add C4 | Order: [C1, C3, C4] | R-1.2.1 |

### TC-1.2.1.3 Set Parent Command

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_parent(child, parent)` via command buffer | Parent component added to child | R-1.2.1 |
| 2 | Check parent's Children | Child present in list | R-1.2.1 |
| 3 | Check events | HierarchyEvent::ChildAdded emitted | R-1.2.1 |

### TC-1.2.1.4 Remove Parent Command

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `remove_parent(child)` via command buffer | Parent component removed from child | R-1.2.1 |
| 2 | Check old parent's Children | Child absent | R-1.2.1 |
| 3 | Check events | HierarchyEvent::ChildRemoved emitted | R-1.2.1 |

### TC-1.2.1.5 Reparent Child

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Child under parent A, reparent to B | A's children excludes child | R-1.2.1 |
| 2 | Check B's children | Child present in B's list | R-1.2.1 |

### TC-1.2.3.1 Cascade Despawn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-level hierarchy: root -> mid -> leaf | All 3 entities alive | R-1.2.3 |
| 2 | Despawn root | All 3 entities destroyed | R-1.2.3 |

### TC-1.2.3.2 Orphan On Delete

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent with 2 children, `despawn_orphaning(parent)` | Parent destroyed | R-1.2.3 |
| 2 | Check children | Both alive, reparented to root (no Parent) | R-1.2.3 |

### TC-1.2.3.3 No Orphaned Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cascade-delete 5-level hierarchy | All descendants destroyed | R-1.2.3 |
| 2 | Query entities with Parent pointing to dead entity | Zero results | R-1.2.3 |

### TC-1.2.2.1 DFS Traversal Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-level tree: R->[A->[D,E], B->[F], C] | DFS order: R, A, D, E, B, F, C | R-1.2.2 |

### TC-1.2.2.2 BFS Traversal Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-level tree: R->[A->[D,E], B->[F], C] | BFS order: R, A, B, C, D, E, F | R-1.2.2 |

### TC-1.2.2.3 Traversal Skip Subtree

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DFS on R->[A->[D,E], B], skip subtree at A | Visited: R, A, B (D, E skipped) | R-1.2.2 |

### TC-1.2.2.4 Traversal Early Termination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DFS on 10-node tree, break after 3 | Exactly 3 nodes visited | R-1.2.2 |

### TC-1.2.2.5 Traversal 256 Depth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 256-level chain | Traversal completes | R-1.2.2a |
| 2 | Check allocations | Zero heap allocations | R-1.2.2a |

### TC-1.2.2.6 Traversal 300 Depth Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 300-level chain | Traversal completes (heap fallback) | R-1.2.2a |
| 2 | Verify correctness | All 300 nodes visited in order | R-1.2.2a |

### TC-1.2.2.7 Traversal Depth Warning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 129-level tree | Diagnostic warning fires at depth > 128 | R-1.2.2a |

### TC-1.2.4.4 Propagation Root Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Root entity with T=(1,0,0), no parent | `GlobalTransform == Transform.local_matrix()` | R-1.2.4 |

### TC-1.2.4.5 Propagation Two Levels

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent T=(10,0,0), Child T=(0,5,0) | Child GlobalTransform = parent.global * child.local | R-1.2.4 |
| 2 | Extract child world translation | (10, 5, 0) | R-1.2.4 |

### TC-1.2.4.6 Propagation Deep Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50-level chain, each T=(1,0,0) | Leaf world translation = (50, 0, 0) | R-1.2.4 |
| 2 | Compare to serial reference computation | Bit-exact match | R-1.2.4 |

### TC-1.2.4.7 Propagation No Stack Overflow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000-level chain, run propagation | Completes without stack overflow | R-1.2.4 |

### TC-1.2.5.1 Dirty Tracking Unchanged

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity with unmodified Transform | GlobalTransform not recomputed this frame | R-1.2.5 |

### TC-1.2.5.2 Dirty Leaf Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-level tree, modify leaf's Transform only | Only leaf's GlobalTransform recomputed | R-1.2.5 |

### TC-1.2.5.3 Dirty Root Propagates

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-level tree, modify root's Transform | All descendants' GlobalTransforms recomputed | R-1.2.5 |

### TC-1.2.5.4 Dirty No False Marks

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Read Transform via `&Transform` (immutable) | Dirty flag not set | R-1.2.5 |

### TC-1.2.1.6 Scene Serialize Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene with hierarchy and transforms, serialize | Bytes produced | R-1.2.1 |
| 2 | Deserialize | Identical hierarchy and transform values | R-1.2.1 |

### TC-1.2.1.7 Scene Entity Remap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn scene into populated world | Scene entities created | R-1.2.1 |
| 2 | Verify entity references in components | All remapped to new IDs | R-1.2.1 |

### TC-1.2.1.8 Scene Spawn As Child

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn scene under target parent | Scene root becomes child of target | R-1.2.1 |

### TC-1.2.1.9 Scene Cyclic Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attempt to serialize cyclic hierarchy | `CyclicHierarchy` error returned | R-1.2.1 |

## Integration Tests

### TC-1.2.4.I1 Parallel Propagation Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100K entities, mixed depths 1-50, parallel propagation | All GlobalTransforms computed | R-1.2.4 |
| 2 | Compare to serial reference | Bit-exact match for all entities | R-1.2.4 |

### TC-1.2.4.I2 Propagation Same Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify Transform in Update phase | Change applied | R-1.2.4a |
| 2 | Read GlobalTransform in PreRender phase | Reflects Update modification (no 1-frame lag) | R-1.2.4a |

### TC-1.2.1.I1 Hierarchy During Parallel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hierarchy modifications via commands during parallel iteration | All commands applied after flush | R-1.2.1 |
| 2 | Run under ThreadSanitizer | Zero data races | R-1.2.1 |

### TC-1.2.6.I1 Spatial Index After Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move entities, run propagation, then spatial query | Query returns entities at correct world positions | R-1.2.6 |

### TC-1.2.1.I2 Scene With Prefabs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene containing prefab instances | Hierarchy preserved | R-1.2.1 |
| 2 | Verify overrides | Prefab overrides intact | R-1.2.1 |

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
