# Entity Component System Test Cases

Companion test cases for [ecs.md](ecs.md).

## Unit Tests

### TC-1.1.11.1 Entity Allocate and Deallocate

| # | Requirement |
|---|-------------|
| 1 | R-1.1.11    |
| 2 | R-1.1.11    |
| 3 | R-1.1.11    |
| 4 | R-1.1.11    |

1. **#1** — `allocate()` returns Entity(idx=0, gen=0)
   - **Expected:** Entity valid, `is_alive()` true
2. **#2** — `deallocate(Entity(0, 0))`
   - **Expected:** Entity(0, 0) invalid, `is_alive()` false
3. **#3** — `allocate()` reuses idx=0
   - **Expected:** Entity(idx=0, gen=1) returned
4. **#4** — Lookup Entity(idx=0, gen=0) after reuse
   - **Expected:** Returns `None` (stale generation)

### TC-1.1.11.2 Entity 1M Alloc/Dealloc Performance

| # | Requirement |
|---|-------------|
| 1 | R-1.1.11a   |
| 2 | R-1.1.11a   |
| 3 | R-1.1.11a   |

1. **#1** — Allocate 1,000,000 entities
   - **Expected:** All valid, unique handles
2. **#2** — Deallocate all 1,000,000
   - **Expected:** All invalid
3. **#3** — Measure per-op cost
   - **Expected:** < 100 ns amortized

### TC-1.1.1.1 Archetype SoA Layout Verification

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |
| 2 | R-1.1.1     |
| 3 | R-1.1.1     |

1. **#1** — Spawn 1000 entities with (Position, Velocity)
   - **Expected:** Position[] contiguous in memory
2. **#2** — Verify via pointer arithmetic
   - **Expected:** Velocity[] contiguous, separate from Position[]
3. **#3** — Iterate Position column
   - **Expected:** All 1000 values accessible sequentially

### TC-1.1.1.2 Archetype Chunk Alignment

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1a    |
| 2 | R-1.1.1a    |

1. **#1** — Allocate chunk for archetype (A, B)
   - **Expected:** `chunk.base_ptr() % 64 == 0`
2. **#2** — Allocate second chunk
   - **Expected:** `chunk.base_ptr() % 64 == 0`

### TC-1.1.2.1 Sparse Component No Migration

| # | Requirement |
|---|-------------|
| 1 | R-1.1.2     |
| 2 | R-1.1.2     |

1. **#1** — Record archetype ID before `#[sparse]` add
   - **Expected:** Archetype ID captured
2. **#2** — Add and remove `#[sparse]` component 10,000 times
   - **Expected:** Archetype ID identical after each op

### TC-1.1.2.2 Sparse Component O(1) Lookup

| # | Requirement |
|---|-------------|
| 1 | R-1.1.2a    |
| 2 | R-1.1.2a    |

1. **#1** — Lookup sparse component on 100,000 entities
   - **Expected:** All lookups succeed
2. **#2** — Measure per-lookup time
   - **Expected:** < 200 ns per operation

### TC-1.1.3.1 Archetype Graph Edge Cache

| # | Requirement |
|---|-------------|
| 1 | R-1.1.3     |
| 2 | R-1.1.3     |

1. **#1** — Spawn 100,000 entities with same component set
   - **Expected:** Single archetype used
2. **#2** — Measure per-entity archetype resolution
   - **Expected:** No degradation with entity count

### TC-1.1.3.2 Archetype Graph 10K Archetypes

| # | Requirement |
|---|-------------|
| 1 | R-1.1.3a    |
| 2 | R-1.1.3a    |

1. **#1** — Create 10,000 distinct archetypes
   - **Expected:** All created without error
2. **#2** — Lookup edges between arbitrary pairs
   - **Expected:** O(1) per lookup

### TC-1.1.4.1 Component Static Registration

| # | Requirement |
|---|-------------|
| 1 | R-1.1.4     |
| 2 | R-1.1.4     |

1. **#1** — `#[derive(Component)]` on struct Position
   - **Expected:** TypeId registered in world
2. **#2** — Spawn entity with Position, query by TypeId
   - **Expected:** Returns correct Position data

### TC-1.1.4.2 Component Dynamic Registration

| # | Requirement |
|---|-------------|
| 1 | R-1.1.4     |
| 2 | R-1.1.4     |

1. **#1** — `world.register_dynamic("Health", layout)`
   - **Expected:** Component ID assigned
2. **#2** — Attach dynamic component, query
   - **Expected:** Returns correct bytes

### TC-1.1.5.1 Tag Component Zero Memory

| # | Requirement |
|---|-------------|
| 1 | R-1.1.5     |
| 2 | R-1.1.5     |

1. **#1** — Add zero-size tag to 100,000 entities
   - **Expected:** Tag column size == 0 bytes
2. **#2** — Query `With<Tag>`
   - **Expected:** Returns all 100,000 entities

### TC-1.1.6.1 Shared Component One Per Chunk

| # | Requirement |
|---|-------------|
| 1 | R-1.1.6     |
| 2 | R-1.1.6     |

1. **#1** — Assign same shared value to 10,000 entities
   - **Expected:** Stored once (not 10,000 copies)
2. **#2** — Change one entity's shared value
   - **Expected:** Entity migrates to new chunk

### TC-1.1.7.1 Buffer Component Inline and Spill

| # | Requirement |
|---|-------------|
| 1 | R-1.1.7     |
| 2 | R-1.1.7     |
| 3 | R-1.1.7     |

1. **#1** — Create buffer component, append < inline threshold
   - **Expected:** Data stored inline
2. **#2** — Append past inline threshold
   - **Expected:** Data spills to heap
3. **#3** — Remove elements below threshold
   - **Expected:** Returns to inline storage

### TC-1.1.8.1 Enableable Component Toggle

| # | Requirement |
|---|-------------|
| 1 | R-1.1.8     |
| 2 | R-1.1.8     |
| 3 | R-1.1.8     |

1. **#1** — Disable enableable component
   - **Expected:** Default query excludes entity
2. **#2** — Query with `WithDisabled<T>`
   - **Expected:** Entity included
3. **#3** — Re-enable component
   - **Expected:** Default query includes entity again

### TC-1.1.8.2 Enableable Parallel Toggle

| # | Requirement |
|---|-------------|
| 1 | R-1.1.8     |

1. **#1** — Toggle from 8 threads concurrently, 10K ops each
   - **Expected:** No data races (ThreadSanitizer clean)

### TC-1.1.9.1 Component Hooks Fire Correctly

| # | Requirement |
|---|-------------|
| 1 | R-1.1.9     |
| 2 | R-1.1.9     |
| 3 | R-1.1.9     |

1. **#1** — Register on_add hook, add component
   - **Expected:** Hook fires with (entity, &component)
2. **#2** — Register on_remove hook, remove component
   - **Expected:** Hook fires with entity
3. **#3** — Register on_set hook, mutate component
   - **Expected:** Hook fires with (entity, &old, &new)

### TC-1.1.9.2 Component Hook Limit

| # | Requirement |
|---|-------------|
| 1 | R-1.1.9a    |
| 2 | R-1.1.9a    |

1. **#1** — Register 16 hooks for one component type
   - **Expected:** All 16 registered successfully
2. **#2** — Register 17th hook
   - **Expected:** Error returned

### TC-1.1.10.1 Bundle Atomic Insert

| # | Requirement |
|---|-------------|
| 1 | R-1.1.10    |
| 2 | R-1.1.10    |

1. **#1** — Insert bundle of (A, B, C, D)
   - **Expected:** Single archetype transition
2. **#2** — Query entity
   - **Expected:** All 4 components present

### TC-1.1.10.2 Required Component Auto-Add

| # | Requirement |
|---|-------------|
| 1 | R-1.1.10    |
| 2 | R-1.1.10    |

1. **#1** — Insert Collider (requires CollisionLayers)
   - **Expected:** Both Collider and CollisionLayers present
2. **#2** — Query CollisionLayers
   - **Expected:** Default value present

### TC-1.1.12.1 Cleanup Component Persistence

| # | Requirement |
|---|-------------|
| 1 | R-1.1.12    |
| 2 | R-1.1.12    |

1. **#1** — Despawn entity with cleanup + normal components
   - **Expected:** Entity alive
2. **#2** — Query entity's components
   - **Expected:** Only cleanup components remain

### TC-1.1.13.1 Entity Name Path Lookup

| # | Requirement |
|---|-------------|
| 1 | R-1.1.13    |
| 2 | R-1.1.13    |

1. **#1** — Build 3-level hierarchy: "root/child/leaf"
   - **Expected:** Path lookup returns leaf entity
2. **#2** — Lookup at 100,000 entities
   - **Expected:** O(log n) scaling verified

### TC-1.1.14.1 Relationship Pair Encoding

| # | Requirement |
|---|-------------|
| 1 | R-1.1.14    |
| 2 | R-1.1.14    |

1. **#1** — Add (Likes, Apple) and (Likes, Banana) to entity
   - **Expected:** Both relationships stored
2. **#2** — Query (Likes, *)
   - **Expected:** Returns {Apple, Banana}

### TC-1.1.15.1 Exclusive Relationship

| # | Requirement |
|---|-------------|
| 1 | R-1.1.15    |
| 2 | R-1.1.15    |

1. **#1** — Add exclusive (HasTarget, A)
   - **Expected:** Relationship present
2. **#2** — Add exclusive (HasTarget, B)
   - **Expected:** (HasTarget, A) removed, (HasTarget, B) present

### TC-1.1.15.2 Symmetric Relationship

| # | Requirement |
|---|-------------|
| 1 | R-1.1.15    |
| 2 | R-1.1.15    |

1. **#1** — Add symmetric (FriendOf, B) to entity A
   - **Expected:** A has (FriendOf, B)
2. **#2** — Query entity B
   - **Expected:** B has (FriendOf, A) auto-added

### TC-1.1.16.1 ChildOf Cascade Delete

| # | Requirement |
|---|-------------|
| 1 | R-1.1.16    |
| 2 | R-1.1.16    |

1. **#1** — Build 4-level hierarchy (root, c1, c2, c3)
   - **Expected:** All 4 entities alive
2. **#2** — Delete root
   - **Expected:** All 4 entities destroyed

### TC-1.1.16.2 ChildOf Cycle Rejection

| # | Requirement |
|---|-------------|
| 1 | R-1.1.16a   |

1. **#1** — A parent of B, attempt B parent of A
   - **Expected:** Error returned (cycle detected)

### TC-1.1.16.3 ChildOf 256 Depth

| # | Requirement |
|---|-------------|
| 1 | R-1.1.16a   |
| 2 | R-1.1.16a   |

1. **#1** — Build 256-level hierarchy
   - **Expected:** All entities created
2. **#2** — Traverse full depth
   - **Expected:** Completes without stack overflow

### TC-1.1.17.1 Query All Filters

| # | Requirement |
|---|-------------|
| 1 | R-1.1.17    |
| 2 | R-1.1.17    |
| 3 | R-1.1.17    |
| 4 | R-1.1.17    |
| 5 | R-1.1.17    |

1. **#1** — 10 entities: 5 with A, 3 with B, 2 with both
   - **Expected:** `Query<&A, With<B>>` returns 2
2. **#2** — `Query<&A, Without<B>>`
   - **Expected:** Returns 3
3. **#3** — `Query<Option<&A>>` on entity without A
   - **Expected:** Returns `None`
4. **#4** — `Query<&A, Changed<A>>` after mutation
   - **Expected:** Returns mutated entity only
5. **#5** — `Query<&A, Added<A>>` after spawn
   - **Expected:** Returns newly spawned entity

### TC-1.1.17.2 Query Cache Zero Overhead

| # | Requirement |
|---|-------------|
| 1 | R-1.1.17a   |

1. **#1** — Run cached query 1,000 times (no new archetypes)
   - **Expected:** Zero archetype matching after first run

### TC-1.1.17.3 Query Cache Incremental Update

| # | Requirement |
|---|-------------|
| 1 | R-1.1.17a   |
| 2 | R-1.1.17a   |

1. **#1** — Build query cache, then add new archetype
   - **Expected:** Cache incrementally updates
2. **#2** — Run query
   - **Expected:** New archetype entities included

### TC-1.1.18.1 Query Sort Stable

| # | Requirement |
|---|-------------|
| 1 | R-1.1.18    |
| 2 | R-1.1.18    |

1. **#1** — Sort 1,000 entities by value field
   - **Expected:** Ascending order verified
2. **#2** — Modify 10 values, re-sort
   - **Expected:** Stable sort (equal elements preserve order)

### TC-1.1.19.1 Query Variable Pattern

| # | Requirement |
|---|-------------|
| 1 | R-1.1.19    |
| 2 | R-1.1.19    |

1. **#1** — Create parent-child pairs, some parents have Boss tag
   - **Expected:** Query children of Boss parents
2. **#2** — Verify results
   - **Expected:** Only children of Boss-tagged parents returned

### TC-1.1.22.1 Change Detection Chunk Level

| # | Requirement |
|---|-------------|
| 1 | R-1.1.22    |
| 2 | R-1.1.22    |

1. **#1** — Mutate 1 entity in chunk of 100
   - **Expected:** Chunk marked changed
2. **#2** — Next tick, no mutations
   - **Expected:** `Changed<T>` query returns 0 results

### TC-1.1.22.2 Change Detection Metadata Size

| # | Requirement |
|---|-------------|
| 1 | R-1.1.22a   |

1. **#1** — Inspect change detection metadata per chunk
   - **Expected:** 8 bytes per component type per chunk

### TC-1.1.23.1 Resource Res and ResMut

| # | Requirement |
|---|-------------|
| 1 | R-1.1.23    |
| 2 | R-1.1.23    |
| 3 | R-1.1.23    |

1. **#1** — Insert resource value 42
   - **Expected:** `Res<T>` reads 42
2. **#2** — Write 99 via `ResMut<T>`
   - **Expected:** `Res<T>` reads 99
3. **#3** — Scheduler with read + write systems
   - **Expected:** Write ordered before read

### TC-1.1.24.1 Non-Send Main Thread

| # | Requirement |
|---|-------------|
| 1 | R-1.1.24    |
| 2 | R-1.1.24    |

1. **#1** — Register non-send resource
   - **Expected:** System using it runs on game loop thread
2. **#2** — Verify thread ID
   - **Expected:** Matches game loop thread ID

### TC-1.1.25.1 Schedule Dependency Resolution

| # | Requirement |
|---|-------------|
| 1 | R-1.1.25    |

1. **#1** — System A before B, B before C
   - **Expected:** Execution order: A, B, C

### TC-1.1.25.2 Schedule Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-1.1.25    |

1. **#1** — A before B, B before A
   - **Expected:** Error at build time (cycle)

### TC-1.1.26.1 Schedule Phases Order

| # | Requirement |
|---|-------------|
| 1 | R-1.1.26    |
| 2 | R-1.1.26    |

1. **#1** — Systems in Update and FixedUpdate
   - **Expected:** FixedUpdate before Update
2. **#2** — Disable FixedUpdate group
   - **Expected:** FixedUpdate systems do not execute

### TC-1.1.27.1 Run Criteria Gate

| # | Requirement |
|---|-------------|
| 1 | R-1.1.27    |
| 2 | R-1.1.27    |
| 3 | R-1.1.27    |

1. **#1** — Boolean criterion = false
   - **Expected:** System does not run
2. **#2** — Boolean criterion = true
   - **Expected:** System runs
3. **#3** — AND compose two criteria (true, false)
   - **Expected:** System does not run

### TC-1.1.28.1 Ambiguity Detection

| # | Requirement |
|---|-------------|
| 1 | R-1.1.28    |

1. **#1** — System X reads A, System Y writes A, no ordering
   - **Expected:** Warning emitted naming X, Y, and component A

### TC-1.1.29.1 Exclusive System Barrier

| # | Requirement |
|---|-------------|
| 1 | R-1.1.29    |

1. **#1** — Register exclusive system
   - **Expected:** No other systems run concurrently

### TC-1.1.30.1 Observer OnAdd

| # | Requirement |
|---|-------------|
| 1 | R-1.1.30    |
| 2 | R-1.1.30    |
| 3 | R-1.1.30    |

1. **#1** — Register OnAdd observer with `With<Tag>` filter
   - **Expected:** Observer registered
2. **#2** — Add component to entity with Tag
   - **Expected:** Observer fires
3. **#3** — Add component to entity without Tag
   - **Expected:** Observer does not fire

### TC-1.1.31.1 Custom Event Propagation

| # | Requirement |
|---|-------------|
| 1 | R-1.1.31    |
| 2 | R-1.1.31    |

1. **#1** — Emit DamageEvent at child entity
   - **Expected:** Observer fires on child
2. **#2** — Event bubbles to parent
   - **Expected:** Observer fires on parent

### TC-1.1.32.1 Command Buffer Deterministic Order

| # | Requirement |
|---|-------------|
| 1 | R-1.1.32    |
| 2 | R-1.1.32    |

1. **#1** — 2 systems record commands, flush
   - **Expected:** Deterministic order
2. **#2** — Repeat 100 times
   - **Expected:** Identical results each run

### TC-1.1.33.1 Parallel Command Writer

| # | Requirement |
|---|-------------|
| 1 | R-1.1.33    |
| 2 | R-1.1.33    |
| 3 | R-1.1.33    |

1. **#1** — 8 threads record 100,000 commands total
   - **Expected:** All recorded
2. **#2** — Flush, verify sort-key order
   - **Expected:** Commands in deterministic order
3. **#3** — Repeat 100 iterations
   - **Expected:** Identical results

### TC-1.1.34.1 Multiple Worlds Isolation

| # | Requirement |
|---|-------------|
| 1 | R-1.1.34    |
| 2 | R-1.1.34    |
| 3 | R-1.1.34    |

1. **#1** — Create World A and World B
   - **Expected:** Both empty
2. **#2** — Spawn 100 entities in A, 50 in B
   - **Expected:** A has 100, B has 50
3. **#3** — Query in A
   - **Expected:** Returns only A's entities

### TC-1.1.35.1 Entity Migration

| # | Requirement |
|---|-------------|
| 1 | R-1.1.35    |
| 2 | R-1.1.35    |

1. **#1** — Entity with 5 components + 2 relationships in World A
   - **Expected:** All data present
2. **#2** — Migrate to World B
   - **Expected:** All data intact in B, no ID collisions

### TC-1.1.35.2 Migration Missing Type Error

| # | Requirement |
|---|-------------|
| 1 | R-1.1.35a   |

1. **#1** — Migrate entity with unregistered component type
   - **Expected:** `MigrationError` with type name in message

### TC-1.1.36.1 Prefab Inheritance

| # | Requirement |
|---|-------------|
| 1 | R-1.1.36    |
| 2 | R-1.1.36    |

1. **#1** — Prefab with (A=1, B=2, C=3), instantiate 100
   - **Expected:** All 100 share prefab data
2. **#2** — Override A=99 on instance 0
   - **Expected:** Instance 0 has A=99, others have A=1

### TC-1.1.37.1 Nested Prefab

| # | Requirement |
|---|-------------|
| 1 | R-1.1.37    |
| 2 | R-1.1.37    |

1. **#1** — Outer prefab contains inner prefab, instantiate 10
   - **Expected:** 10 outer + 10 inner instances
2. **#2** — Modify inner prefab value
   - **Expected:** Change propagates to all inner instances

### TC-1.1.38.1 State Transition Observers

| # | Requirement |
|---|-------------|
| 1 | R-1.1.38    |
| 2 | R-1.1.38    |
| 3 | R-1.1.38    |

1. **#1** — Transition Menu -> Playing
   - **Expected:** OnExit(Menu) fires
2. **#2** — After transition
   - **Expected:** OnEnter(Playing) fires
3. **#3** — `in_state(Playing)` criterion
   - **Expected:** Returns true

## Integration Tests

### TC-1.1.25.I1 Full Frame Cycle

| # | Requirement |
|---|-------------|
| 1 | R-1.1.25    |
| 2 | R-1.1.29    |

1. **#1** — Build schedule, run full frame
   - **Expected:** All phases execute in order
2. **#2** — Verify command flush and observer dispatch
   - **Expected:** World state consistent

### TC-1.1.20.I1 Parallel Iteration Scaling

| # | Requirement |
|---|-------------|
| 1 | R-1.1.20    |
| 2 | R-1.1.20    |
| 3 | R-1.1.20    |

1. **#1** — Iterate 1M entities on 1 core
   - **Expected:** Baseline time recorded
2. **#2** — Iterate 1M entities on 4 cores
   - **Expected:** >= 3.5x speedup over baseline
3. **#3** — Run under ThreadSanitizer
   - **Expected:** Zero data races

### TC-1.1.16.I1 Cascade Delete 100K

| # | Requirement |
|---|-------------|
| 1 | R-1.1.16a   |
| 2 | R-1.1.16a   |

1. **#1** — Build 100,000-entity subtree
   - **Expected:** All entities alive
2. **#2** — Delete root
   - **Expected:** All 100,000 destroyed within 10 ms

### TC-1.1.35.I1 Bulk Migration 500

| # | Requirement |
|---|-------------|
| 1 | R-1.1.35a   |
| 2 | R-1.1.35a   |

1. **#1** — Migrate 500 entities simultaneously
   - **Expected:** All data intact
2. **#2** — Verify IDs
   - **Expected:** No ID collisions in target world

### TC-1.1.25.I2 Schedule 500 Systems

| # | Requirement |
|---|-------------|
| 1 | R-1.1.25a   |
| 2 | R-1.1.25a   |

1. **#1** — Build schedule with 500 systems
   - **Expected:** Construction under 50 ms
2. **#2** — Run second frame (no system changes)
   - **Expected:** No rebuild triggered

### TC-1.1.30.I1 Observer Dispatch 1000

| # | Requirement |
|---|-------------|
| 1 | R-1.1.30a   |
| 2 | R-1.1.30a   |

1. **#1** — 1,000 observers, fire 100 events matching 10 each
   - **Expected:** 1,000 callbacks total
2. **#2** — Verify scaling
   - **Expected:** O(events * matches)

### TC-1.1.32.I1 Command Buffer 100K Flush

| # | Requirement |
|---|-------------|
| 1 | R-1.1.32a   |
| 2 | R-1.1.32a   |
| 3 | R-1.1.32a   |

1. **#1** — Record 100,000 commands
   - **Expected:** All recorded
2. **#2** — Flush
   - **Expected:** Completes under 1 ms
3. **#3** — Verify per-system buffer size
   - **Expected:** Under 64 KiB typical

### TC-1.1.1.I1 Mixed Storage Query

| # | Requirement |
|---|-------------|
| 1 | R-1.1.1     |
| 2 | R-1.1.2     |

1. **#1** — Entities with table component A and sparse component B
   - **Expected:** Both present
2. **#2** — Query `(&A, &B)`
   - **Expected:** Correct results across both storage modes

## Benchmarks

### TC-1.1.1.B1 Archetype Iteration Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M entities, 3 components, 1 core | Components/sec | >= 500M | R-1.1.1a |

### TC-1.1.11.B1 Entity Alloc/Dealloc Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Allocate single entity | Latency | < 100 ns | R-1.1.11a |
| 2 | Deallocate single entity | Latency | < 100 ns | R-1.1.11a |

### TC-1.1.2.B1 Sparse Component Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Lookup sparse component, 100K entities | Latency | O(1), < 200 ns | R-1.1.2a |

### TC-1.1.3.B1 Archetype Graph Edge Traversal

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Edge traversal with 10K archetypes | Complexity | O(1) amortized | R-1.1.3 |

### TC-1.1.9.B1 Component Hook Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Fire single component hook | Overhead | < 50 ns per invocation | R-1.1.9a |

### TC-1.1.17.B1 Query Cache Hit

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Cached query, no new archetypes | Matching overhead | 0 ns | R-1.1.17a |

### TC-1.1.20.B1 Parallel Iteration Speedup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1M entities, 4 cores | Speedup vs 1 core | >= 3.5x | R-1.1.20 |

### TC-1.1.25.B1 Schedule Build

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 systems, full build | Time | < 50 ms | R-1.1.25a |

### TC-1.1.32.B1 Command Buffer Flush

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100K commands, flush | Time | < 1 ms | R-1.1.32a |

### TC-1.1.35.B1 Entity Migration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single entity migration | Latency | < 10 us | R-1.1.35a |

### TC-1.1.16.B1 Cascade Delete

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Cascade delete 100K subtree | Time | < 10 ms | R-1.1.16a |
