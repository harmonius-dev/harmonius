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
