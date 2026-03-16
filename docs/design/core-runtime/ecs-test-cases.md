# Entity Component System Test Cases

Companion test cases for [ecs.md](ecs.md).

## Unit Tests

### TC-1.1.11.1 Entity Allocate and Deallocate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `allocate()` returns Entity(idx=0, gen=0) | Entity valid, `is_alive()` true | R-1.1.11 |
| 2 | `deallocate(Entity(0, 0))` | Entity(0, 0) invalid, `is_alive()` false | R-1.1.11 |
| 3 | `allocate()` reuses idx=0 | Entity(idx=0, gen=1) returned | R-1.1.11 |
| 4 | Lookup Entity(idx=0, gen=0) after reuse | Returns `None` (stale generation) | R-1.1.11 |

### TC-1.1.11.2 Entity 1M Alloc/Dealloc Performance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate 1,000,000 entities | All valid, unique handles | R-1.1.11a |
| 2 | Deallocate all 1,000,000 | All invalid | R-1.1.11a |
| 3 | Measure per-op cost | < 100 ns amortized | R-1.1.11a |

### TC-1.1.1.1 Archetype SoA Layout Verification

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn 1000 entities with (Position, Velocity) | Position[] contiguous in memory | R-1.1.1 |
| 2 | Verify via pointer arithmetic | Velocity[] contiguous, separate from Position[] | R-1.1.1 |
| 3 | Iterate Position column | All 1000 values accessible sequentially | R-1.1.1 |

### TC-1.1.1.2 Archetype Chunk Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate chunk for archetype (A, B) | `chunk.base_ptr() % 64 == 0` | R-1.1.1a |
| 2 | Allocate second chunk | `chunk.base_ptr() % 64 == 0` | R-1.1.1a |

### TC-1.1.2.1 Sparse Component No Migration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record archetype ID before `#[sparse]` add | Archetype ID captured | R-1.1.2 |
| 2 | Add and remove `#[sparse]` component 10,000 times | Archetype ID identical after each op | R-1.1.2 |

### TC-1.1.2.2 Sparse Component O(1) Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Lookup sparse component on 100,000 entities | All lookups succeed | R-1.1.2a |
| 2 | Measure per-lookup time | < 200 ns per operation | R-1.1.2a |

### TC-1.1.3.1 Archetype Graph Edge Cache

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn 100,000 entities with same component set | Single archetype used | R-1.1.3 |
| 2 | Measure per-entity archetype resolution | No degradation with entity count | R-1.1.3 |

### TC-1.1.3.2 Archetype Graph 10K Archetypes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create 10,000 distinct archetypes | All created without error | R-1.1.3a |
| 2 | Lookup edges between arbitrary pairs | O(1) per lookup | R-1.1.3a |

### TC-1.1.4.1 Component Static Registration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `#[derive(Component)]` on struct Position | TypeId registered in world | R-1.1.4 |
| 2 | Spawn entity with Position, query by TypeId | Returns correct Position data | R-1.1.4 |

### TC-1.1.4.2 Component Dynamic Registration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `world.register_dynamic("Health", layout)` | Component ID assigned | R-1.1.4 |
| 2 | Attach dynamic component, query | Returns correct bytes | R-1.1.4 |

### TC-1.1.5.1 Tag Component Zero Memory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add zero-size tag to 100,000 entities | Tag column size == 0 bytes | R-1.1.5 |
| 2 | Query `With<Tag>` | Returns all 100,000 entities | R-1.1.5 |

### TC-1.1.6.1 Shared Component One Per Chunk

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Assign same shared value to 10,000 entities | Stored once (not 10,000 copies) | R-1.1.6 |
| 2 | Change one entity's shared value | Entity migrates to new chunk | R-1.1.6 |

### TC-1.1.7.1 Buffer Component Inline and Spill

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create buffer component, append < inline threshold | Data stored inline | R-1.1.7 |
| 2 | Append past inline threshold | Data spills to heap | R-1.1.7 |
| 3 | Remove elements below threshold | Returns to inline storage | R-1.1.7 |

### TC-1.1.8.1 Enableable Component Toggle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disable enableable component | Default query excludes entity | R-1.1.8 |
| 2 | Query with `WithDisabled<T>` | Entity included | R-1.1.8 |
| 3 | Re-enable component | Default query includes entity again | R-1.1.8 |

### TC-1.1.8.2 Enableable Parallel Toggle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle from 8 threads concurrently, 10K ops each | No data races (ThreadSanitizer clean) | R-1.1.8 |

### TC-1.1.9.1 Component Hooks Fire Correctly

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register on_add hook, add component | Hook fires with (entity, &component) | R-1.1.9 |
| 2 | Register on_remove hook, remove component | Hook fires with entity | R-1.1.9 |
| 3 | Register on_set hook, mutate component | Hook fires with (entity, &old, &new) | R-1.1.9 |

### TC-1.1.9.2 Component Hook Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register 16 hooks for one component type | All 16 registered successfully | R-1.1.9a |
| 2 | Register 17th hook | Error returned | R-1.1.9a |

### TC-1.1.10.1 Bundle Atomic Insert

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert bundle of (A, B, C, D) | Single archetype transition | R-1.1.10 |
| 2 | Query entity | All 4 components present | R-1.1.10 |

### TC-1.1.10.2 Required Component Auto-Add

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert Collider (requires CollisionLayers) | Both Collider and CollisionLayers present | R-1.1.10 |
| 2 | Query CollisionLayers | Default value present | R-1.1.10 |

### TC-1.1.12.1 Cleanup Component Persistence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Despawn entity with cleanup + normal components | Entity alive | R-1.1.12 |
| 2 | Query entity's components | Only cleanup components remain | R-1.1.12 |

### TC-1.1.13.1 Entity Name Path Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build 3-level hierarchy: "root/child/leaf" | Path lookup returns leaf entity | R-1.1.13 |
| 2 | Lookup at 100,000 entities | O(log n) scaling verified | R-1.1.13 |

### TC-1.1.14.1 Relationship Pair Encoding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add (Likes, Apple) and (Likes, Banana) to entity | Both relationships stored | R-1.1.14 |
| 2 | Query (Likes, *) | Returns {Apple, Banana} | R-1.1.14 |

### TC-1.1.15.1 Exclusive Relationship

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add exclusive (HasTarget, A) | Relationship present | R-1.1.15 |
| 2 | Add exclusive (HasTarget, B) | (HasTarget, A) removed, (HasTarget, B) present | R-1.1.15 |

### TC-1.1.15.2 Symmetric Relationship

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add symmetric (FriendOf, B) to entity A | A has (FriendOf, B) | R-1.1.15 |
| 2 | Query entity B | B has (FriendOf, A) auto-added | R-1.1.15 |

### TC-1.1.16.1 ChildOf Cascade Delete

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build 4-level hierarchy (root, c1, c2, c3) | All 4 entities alive | R-1.1.16 |
| 2 | Delete root | All 4 entities destroyed | R-1.1.16 |

### TC-1.1.16.2 ChildOf Cycle Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A parent of B, attempt B parent of A | Error returned (cycle detected) | R-1.1.16a |

### TC-1.1.16.3 ChildOf 256 Depth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build 256-level hierarchy | All entities created | R-1.1.16a |
| 2 | Traverse full depth | Completes without stack overflow | R-1.1.16a |

### TC-1.1.17.1 Query All Filters

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 entities: 5 with A, 3 with B, 2 with both | `Query<&A, With<B>>` returns 2 | R-1.1.17 |
| 2 | `Query<&A, Without<B>>` | Returns 3 | R-1.1.17 |
| 3 | `Query<Option<&A>>` on entity without A | Returns `None` | R-1.1.17 |
| 4 | `Query<&A, Changed<A>>` after mutation | Returns mutated entity only | R-1.1.17 |
| 5 | `Query<&A, Added<A>>` after spawn | Returns newly spawned entity | R-1.1.17 |

### TC-1.1.17.2 Query Cache Zero Overhead

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run cached query 1,000 times (no new archetypes) | Zero archetype matching after first run | R-1.1.17a |

### TC-1.1.17.3 Query Cache Incremental Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build query cache, then add new archetype | Cache incrementally updates | R-1.1.17a |
| 2 | Run query | New archetype entities included | R-1.1.17a |

### TC-1.1.18.1 Query Sort Stable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sort 1,000 entities by value field | Ascending order verified | R-1.1.18 |
| 2 | Modify 10 values, re-sort | Stable sort (equal elements preserve order) | R-1.1.18 |

### TC-1.1.19.1 Query Variable Pattern

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create parent-child pairs, some parents have Boss tag | Query children of Boss parents | R-1.1.19 |
| 2 | Verify results | Only children of Boss-tagged parents returned | R-1.1.19 |

### TC-1.1.22.1 Change Detection Chunk Level

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mutate 1 entity in chunk of 100 | Chunk marked changed | R-1.1.22 |
| 2 | Next tick, no mutations | `Changed<T>` query returns 0 results | R-1.1.22 |

### TC-1.1.22.2 Change Detection Metadata Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inspect change detection metadata per chunk | 8 bytes per component type per chunk | R-1.1.22a |

### TC-1.1.23.1 Resource Res and ResMut

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert resource value 42 | `Res<T>` reads 42 | R-1.1.23 |
| 2 | Write 99 via `ResMut<T>` | `Res<T>` reads 99 | R-1.1.23 |
| 3 | Scheduler with read + write systems | Write ordered before read | R-1.1.23 |

### TC-1.1.24.1 Non-Send Main Thread

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register non-send resource | System using it runs on main thread | R-1.1.24 |
| 2 | Verify thread ID | Matches main thread ID | R-1.1.24 |

### TC-1.1.25.1 Schedule Dependency Resolution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | System A before B, B before C | Execution order: A, B, C | R-1.1.25 |

### TC-1.1.25.2 Schedule Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A before B, B before A | Error at build time (cycle) | R-1.1.25 |

### TC-1.1.26.1 Schedule Phases Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Systems in Update and FixedUpdate | FixedUpdate before Update | R-1.1.26 |
| 2 | Disable FixedUpdate group | FixedUpdate systems do not execute | R-1.1.26 |

### TC-1.1.27.1 Run Criteria Gate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Boolean criterion = false | System does not run | R-1.1.27 |
| 2 | Boolean criterion = true | System runs | R-1.1.27 |
| 3 | AND compose two criteria (true, false) | System does not run | R-1.1.27 |

### TC-1.1.28.1 Ambiguity Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | System X reads A, System Y writes A, no ordering | Warning emitted naming X, Y, and component A | R-1.1.28 |

### TC-1.1.29.1 Exclusive System Barrier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register exclusive system | No other systems run concurrently | R-1.1.29 |

### TC-1.1.30.1 Observer OnAdd

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register OnAdd observer with `With<Tag>` filter | Observer registered | R-1.1.30 |
| 2 | Add component to entity with Tag | Observer fires | R-1.1.30 |
| 3 | Add component to entity without Tag | Observer does not fire | R-1.1.30 |

### TC-1.1.31.1 Custom Event Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emit DamageEvent at child entity | Observer fires on child | R-1.1.31 |
| 2 | Event bubbles to parent | Observer fires on parent | R-1.1.31 |

### TC-1.1.32.1 Command Buffer Deterministic Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2 systems record commands, flush | Deterministic order | R-1.1.32 |
| 2 | Repeat 100 times | Identical results each run | R-1.1.32 |

### TC-1.1.33.1 Parallel Command Writer

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8 threads record 100,000 commands total | All recorded | R-1.1.33 |
| 2 | Flush, verify sort-key order | Commands in deterministic order | R-1.1.33 |
| 3 | Repeat 100 iterations | Identical results | R-1.1.33 |

### TC-1.1.34.1 Multiple Worlds Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create World A and World B | Both empty | R-1.1.34 |
| 2 | Spawn 100 entities in A, 50 in B | A has 100, B has 50 | R-1.1.34 |
| 3 | Query in A | Returns only A's entities | R-1.1.34 |

### TC-1.1.35.1 Entity Migration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity with 5 components + 2 relationships in World A | All data present | R-1.1.35 |
| 2 | Migrate to World B | All data intact in B, no ID collisions | R-1.1.35 |

### TC-1.1.35.2 Migration Missing Type Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Migrate entity with unregistered component type | `MigrationError` with type name in message | R-1.1.35a |

### TC-1.1.36.1 Prefab Inheritance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Prefab with (A=1, B=2, C=3), instantiate 100 | All 100 share prefab data | R-1.1.36 |
| 2 | Override A=99 on instance 0 | Instance 0 has A=99, others have A=1 | R-1.1.36 |

### TC-1.1.37.1 Nested Prefab

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Outer prefab contains inner prefab, instantiate 10 | 10 outer + 10 inner instances | R-1.1.37 |
| 2 | Modify inner prefab value | Change propagates to all inner instances | R-1.1.37 |

### TC-1.1.38.1 State Transition Observers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transition Menu -> Playing | OnExit(Menu) fires | R-1.1.38 |
| 2 | After transition | OnEnter(Playing) fires | R-1.1.38 |
| 3 | `in_state(Playing)` criterion | Returns true | R-1.1.38 |

## Integration Tests

### TC-1.1.25.I1 Full Frame Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build schedule, run full frame | All phases execute in order | R-1.1.25 |
| 2 | Verify command flush and observer dispatch | World state consistent | R-1.1.29 |

### TC-1.1.20.I1 Parallel Iteration Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Iterate 1M entities on 1 core | Baseline time recorded | R-1.1.20 |
| 2 | Iterate 1M entities on 4 cores | >= 3.5x speedup over baseline | R-1.1.20 |
| 3 | Run under ThreadSanitizer | Zero data races | R-1.1.20 |

### TC-1.1.16.I1 Cascade Delete 100K

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build 100,000-entity subtree | All entities alive | R-1.1.16a |
| 2 | Delete root | All 100,000 destroyed within 10 ms | R-1.1.16a |

### TC-1.1.35.I1 Bulk Migration 500

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Migrate 500 entities simultaneously | All data intact | R-1.1.35a |
| 2 | Verify IDs | No ID collisions in target world | R-1.1.35a |

### TC-1.1.25.I2 Schedule 500 Systems

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build schedule with 500 systems | Construction under 50 ms | R-1.1.25a |
| 2 | Run second frame (no system changes) | No rebuild triggered | R-1.1.25a |

### TC-1.1.30.I1 Observer Dispatch 1000

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1,000 observers, fire 100 events matching 10 each | 1,000 callbacks total | R-1.1.30a |
| 2 | Verify scaling | O(events * matches) | R-1.1.30a |

### TC-1.1.32.I1 Command Buffer 100K Flush

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 100,000 commands | All recorded | R-1.1.32a |
| 2 | Flush | Completes under 1 ms | R-1.1.32a |
| 3 | Verify per-system buffer size | Under 64 KiB typical | R-1.1.32a |

### TC-1.1.1.I1 Mixed Storage Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entities with table component A and sparse component B | Both present | R-1.1.1 |
| 2 | Query `(&A, &B)` | Correct results across both storage modes | R-1.1.2 |

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
