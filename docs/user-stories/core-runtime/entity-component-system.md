# Entity Component System User Stories

## Storage

| ID       | Persona                 | Features | Requirements |
|----------|-------------------------|----------|--------------|
| US-1.1.1 | engine developer (P-26) | F-1.1.1  | R-1.1.1      |
| US-1.1.2 | engine tester (P-27)    | F-1.1.1  | R-1.1.1      |
| US-1.1.3 | game developer (P-15)   | F-1.1.2  | R-1.1.2      |
| US-1.1.4 | designer (P-5)          | F-1.1.2  | R-1.1.2      |
| US-1.1.5 | engine developer (P-26) | F-1.1.3  | R-1.1.3      |
| US-1.1.6 | engine tester (P-27)    | F-1.1.3  | R-1.1.3      |

1. **US-1.1.1** — components stored in contiguous archetype tables with structure-of-arrays layout
   and configurable chunk sizes (8-64 KiB), so that iteration over large entity populations is
   cache-friendly and SIMD-optimized for maximum throughput
   - **Acceptance:** Contiguous SoA layout per component type per archetype<br>Cache-line-aligned
     chunks (16 KiB default desktop, 8 KiB mobile)<br>Iteration throughput scales with entity count
2. **US-1.1.2** — to benchmark archetype iteration throughput at various entity counts and chunk
   sizes to verify cache-friendly access patterns, so that I can confirm the SoA layout delivers
   expected speedup over AoS alternatives
   - **Acceptance:** Iteration benchmark at 100K, 500K, 1M entities<br>Cache miss rate measured and
     within target<br>SIMD utilization verified for numeric component types
3. **US-1.1.3** — to mark components as sparse so they bypass archetype tables and are stored in
   per-type sparse sets, so that adding or removing debug markers and temporary status effects does
   not trigger expensive archetype migrations
   - **Acceptance:** `#[sparse]` attribute opts component into sparse storage<br>Sparse component
     changes do not change archetype<br>Sparse components keyed by entity ID
4. **US-1.1.4** — the visual editor to indicate which components are sparse versus dense stored, so
   that I understand why some components cause migrations and others do not when editing entity
   templates
   - **Acceptance:** Editor displays storage mode (dense/sparse) per component<br>Visual indication
     when a component change causes migration
5. **US-1.1.5** — a directed archetype graph with cached edge lookups so structural changes resolve
   target archetypes in O(1), so that bulk spawning and despawning thousands of entities per frame
   remains fast regardless of archetype count
   - **Acceptance:** Directed graph of archetypes with edge caching<br>O(1) archetype resolution for
     add/remove operations<br>Edges encode invariants (adding A always brings B)
6. **US-1.1.6** — to stress-test archetype graph traversal under rapid bulk spawn and despawn of
   thousands of entities with varying component sets, so that I can verify O(1) edge lookup holds
   and no linear scans occur
   - **Acceptance:** 10K entities spawned/despawned per frame without degradation<br>Edge cache hit
     rate > 99% after warmup<br>No linear archetype scan during structural changes

## Components

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.1.7  | game developer (P-15)   | F-1.1.4  | R-1.1.4      |
| US-1.1.8  | game developer (P-15)   | F-1.1.5  | R-1.1.5      |
| US-1.1.9  | game developer (P-15)   | F-1.1.6  | R-1.1.6      |
| US-1.1.10 | game developer (P-15)   | F-1.1.7  | R-1.1.7      |
| US-1.1.11 | game developer (P-15)   | F-1.1.8  | R-1.1.8      |
| US-1.1.12 | engine tester (P-27)    | F-1.1.8  | R-1.1.8      |
| US-1.1.13 | engine developer (P-26) | F-1.1.9  | R-1.1.9      |
| US-1.1.14 | game developer (P-15)   | F-1.1.10 | R-1.1.10     |

1. **US-1.1.7** — to register components both at compile time via derive macros and at runtime for
   hot-reloaded gameplay logic, so that I get zero-cost access for known types while supporting
   dynamic scripting
   - **Acceptance:** Compile-time registration via derive macro<br>Runtime registration for dynamic
     component types<br>Each component records size, alignment, drop, storage mode
2. **US-1.1.8** — zero-sized tag components that alter archetype identity and enable query filtering
   without per-entity memory cost, so that I can mark entities as `Player`, `Enemy`, or `Static`
   efficiently across millions of entities
   - **Acceptance:** Tag components occupy zero bytes in storage<br>Tags alter archetype identity
     for query filtering<br>No column stored for tag components
3. **US-1.1.9** — shared components stored once per chunk rather than per entity, so that material
   IDs, LOD groups, and render layer assignments do not waste memory when thousands of entities
   share the same value
   - **Acceptance:** Shared component stored once per chunk<br>Changing shared value is a structural
     change (moves entity)<br>Zero per-entity memory cost for shared components
4. **US-1.1.10** — variable-length arrays associated with entities via `DynamicBuffer<T>` with
   inline small buffers and heap spill, so that I can store child lists, inventory slots, and
   waypoint sequences per entity without fixed-size limits
   - **Acceptance:** Small buffers inline in archetype chunk<br>Large buffers spill to heap<br>Typed
     `DynamicBuffer<T>` handle for access<br>Platform-appropriate inline thresholds
5. **US-1.1.11** — to toggle enableable components on and off per entity without structural changes,
   so that I can enable or disable behaviors like AI perception or physics sleeping cheaply from
   parallel worker threads
   - **Acceptance:** `#[enableable]` attribute marks togglable components<br>Toggle safe from
     parallel worker threads<br>Disabled components excluded from queries by
     default<br>`WithDisabled<T>` and `WithPresent<T>` override filtering
6. **US-1.1.12** — to stress-test concurrent toggling of enableable components from multiple worker
   threads, so that I can verify no data races or incorrect query results occur under parallel
   access
   - **Acceptance:** Concurrent toggles from 8+ threads produce correct results<br>No data races
     detected by sanitizers<br>Query results consistent with toggle state
7. **US-1.1.13** — per-type lifecycle callbacks (on_add, on_remove, on_set) that fire synchronously
   at the point of change, so that auxiliary data structures like the spatial index stay in sync
   with component changes
   - **Acceptance:** Hooks fire synchronously on add, remove, and set<br>Hook receives entity ID and
     component reference<br>Spatial index updated via Transform on_add hook
8. **US-1.1.14** — to insert groups of related components atomically as named bundles with
   auto-added required companions, so that I never create incomplete entities missing essential
   components like `CollisionLayers` when adding `Collider`
   - **Acceptance:** Bundle inserts all components atomically<br>Required companions auto-added if
     not present<br>Bundles flattened at compile time, zero runtime overhead

## Entities

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.1.15 | engine developer (P-26) | F-1.1.11 | R-1.1.11     |
| US-1.1.16 | engine tester (P-27)    | F-1.1.11 | R-1.1.11     |
| US-1.1.17 | engine developer (P-26) | F-1.1.12 | R-1.1.12     |
| US-1.1.18 | game developer (P-15)   | F-1.1.13 | R-1.1.13     |
| US-1.1.19 | designer (P-5)          | F-1.1.13 | R-1.1.13     |

1. **US-1.1.15** — entities identified by generational indices (32-bit index + 32-bit generation)
   that detect stale references in O(1), so that recycled entity slots cannot be mistakenly accessed
   through old handles after despawn
   - **Acceptance:** O(1) allocation and deallocation via free list<br>Generation counter
     incremented on deallocation<br>Stale handle detected without indirection
2. **US-1.1.16** — to verify that accessing a despawned entity via a stale generational handle fails
   safely rather than returning incorrect data, so that use-after-free bugs are impossible in the
   entity system
   - **Acceptance:** Stale handle access returns error, not stale data<br>Recycled slot with
     incremented generation rejects old handle<br>Fuzz test with random spawn/despawn/access
     patterns passes
3. **US-1.1.17** — cleanup components that persist after entity destruction so dedicated systems can
   tear down GPU buffers and network registrations, so that external resources are cleaned up
   gracefully rather than leaked
   - **Acceptance:** `#[cleanup]` components persist after destroy<br>Non-cleanup components removed
     on destroy<br>Cleanup system detects lingering entities<br>Final removal of cleanup components
     completes destruction
4. **US-1.1.18** — to assign human-readable names to entities and look them up by hierarchical path
   (e.g., `World/Zone3/NPC_Guard_17`), so that I can reference entities in visual scripts and debug
   the entity tree without memorizing raw IDs
   - **Acceptance:** Names assigned as sparse components<br>Path lookup in O(log n) using
     parent-child hierarchy<br>Names usable in visual editor entity tree
5. **US-1.1.19** — to browse and search entities by name in the visual editor's entity tree, so that
   I can find and select specific entities without scrolling through raw numeric IDs
   - **Acceptance:** Entity tree shows names and hierarchical paths<br>Search by name or path
     substring<br>Double-click navigates to entity in viewport

## Relationships and Hierarchies

| ID        | Persona               | Features                     | Requirements                 |
|-----------|-----------------------|------------------------------|------------------------------|
| US-1.1.20 | game developer (P-15) | F-1.1.14                     | R-1.1.14                     |
| US-1.1.21 | game developer (P-15) | F-1.1.15                     | R-1.1.15                     |
| US-1.1.22 | game developer (P-15) | F-1.1.16                     | R-1.1.16                     |
| US-1.1.23 | engine tester (P-27)  | F-1.1.16, F-1.1.16, F-1.1.12 | R-1.1.16, R-1.1.16, R-1.1.12 |

1. **US-1.1.20** — relationships encoded as (Relationship, Target) pairs with wildcard queries, so
   that I can model complex data like "all entities that like any food" or "anything targeting this
   NPC" in a single query
   - **Acceptance:** Relationships packed into 64-bit component IDs<br>Wildcard queries on
     relationship or target<br>Relationships are the foundation for hierarchies
2. **US-1.1.21** — relationships annotated with properties like Exclusive, Symmetric, Transitive,
   Acyclic, and cascading delete policies, so that domain invariants are enforced automatically
   without hand-written validation logic
   - **Acceptance:** Exclusive: only one target per relationship<br>Symmetric: auto-add reverse
     relationship<br>OnDelete/OnDeleteTarget cascading policies<br>Acyclic constraint prevents
     cycles
3. **US-1.1.22** — a built-in `ChildOf` relationship with automatic cascading deletion and traversal
   support, so that deleting a parent entity automatically cleans up all descendants and I can walk
   the hierarchy in either direction
   - **Acceptance:** `ChildOf` with Acyclic, Traversable, OnDeleteTarget(Delete)<br>Up (parent
     chain) and cascade (top-down) traversal<br>Parent deletion cascades to all descendants
4. **US-1.1.23** — to verify that deleting a parent entity at the root of a deep hierarchy cascades
   correctly through all descendants, so that no orphaned entities remain after hierarchical
   deletion
   - **Acceptance:** Delete root of 100-deep hierarchy: all descendants removed<br>No orphaned
     entities after cascading delete<br>Cleanup components on descendants processed correctly

## Queries

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.1.24 | game developer (P-15)   | F-1.1.17 | R-1.1.17     |
| US-1.1.25 | game developer (P-15)   | F-1.1.18 | R-1.1.18     |
| US-1.1.26 | game developer (P-15)   | F-1.1.19 | R-1.1.19     |
| US-1.1.27 | engine developer (P-26) | F-1.1.20 | R-1.1.20     |
| US-1.1.28 | engine tester (P-27)    | F-1.1.20 | R-1.1.20     |

1. **US-1.1.24** — to compose queries with `With`, `Without`, `Option`, `Changed`, and `Added`
   filters that cache after first evaluation, so that I can precisely select entities without
   repeated archetype matching overhead
   - **Acceptance:** All filter types composable in a single query<br>Cached queries skip archetype
     matching on repeat<br>Incremental update when new archetypes created
2. **US-1.1.25** — to sort query results by component value and group by relationship target, so
   that I can batch draw calls by material ID and partition entities spatially without external
   sorting
   - **Acceptance:** Stable sort by component value with custom comparator<br>Group by relationship
     target<br>Sort cached between frames, re-sorted on change detection
3. **US-1.1.26** — query variables enabling graph pattern matching across entity relationships, so
   that I can find all children of boss entities or all entities connected by specific relationship
   chains in a single query pass
   - **Acceptance:** Variables ($parent, $target) in query terms<br>Joins across relationship
     edges<br>Pattern matching without nested loops
4. **US-1.1.27** — query results automatically partitioned across worker threads with borrow safety
   guarantees, so that systems processing large entity populations scale linearly with core count
   - **Acceptance:** Archetype-level or chunk-level partitioning<br>No conflicting mutable/immutable
     borrows across threads<br>Linear scaling with core count for large populations<br>Mobile: 2-4
     workers, desktop: physical cores - 1
5. **US-1.1.28** — to benchmark parallel query iteration at various entity counts and core counts,
   so that I can verify linear scaling and identify any contention or false sharing bottlenecks
   - **Acceptance:** Linear speedup measured from 1 to N workers<br>No false sharing detected via
     perf counters<br>Borrow safety verified under sanitizers

## Aspects

| ID        | Persona               | Features | Requirements |
|-----------|-----------------------|----------|--------------|
| US-1.1.29 | game developer (P-15) | F-1.1.21 | R-1.1.21     |

1. **US-1.1.29** — to group related components into named aspect structs like `PhysicsAspect`, so
   that I reduce boilerplate when multiple systems need the same component grouping
   - **Acceptance:** Aspect struct groups subset of entity components<br>Aspects usable as single
     type parameter in queries<br>Nested aspects supported<br>Per-field access modes (&T vs &mut T)

## Change Detection

| ID        | Persona               | Features | Requirements |
|-----------|-----------------------|----------|--------------|
| US-1.1.30 | game developer (P-15) | F-1.1.22 | R-1.1.22     |
| US-1.1.31 | engine tester (P-27)  | F-1.1.22 | R-1.1.22     |

1. **US-1.1.30** — to query only components modified since the last tick using `Changed<T>`, so that
   network delta compression, spatial index updates, and dirty-flag propagation skip unmodified
   chunks
   - **Acceptance:** Per-component tick counter bumped on mutable access<br>Chunk-granularity change
     tracking<br>`Changed<T>` filter skips unmodified chunks
2. **US-1.1.31** — to verify that change detection correctly marks only chunks containing modified
   entities and does not produce false positives or negatives, so that reactive systems process
   exactly the correct set of changes
   - **Acceptance:** Modified entity's chunk marked changed<br>Unmodified chunks in same archetype
     not marked<br>No false positives from read-only access

## Resources and Singletons

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.1.32 | game developer (P-15)   | F-1.1.23 | R-1.1.23     |
| US-1.1.33 | engine developer (P-26) | F-1.1.24 | R-1.1.24     |

1. **US-1.1.32** — typed singleton resources accessible through scheduler-aware `Res<T>` and
   `ResMut<T>` parameters with change detection, so that global state like `Time` and `InputState`
   is type-safe and participates in dependency analysis
   - **Acceptance:** One instance per type per world<br>Res<T> (shared) and ResMut<T> (exclusive)
     access<br>Resources participate in change detection
2. **US-1.1.33** — resources marked non-send to be automatically pinned to the main thread by the
   scheduler, so that GPU device handles and window handles are never accessed from worker threads
   - **Acceptance:** Non-send resources pinned to main thread<br>Systems accessing non-send
     resources run on main thread<br>Scheduler never moves non-send access to workers

## System Scheduling

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.1.34 | engine developer (P-26) | F-1.1.25 | R-1.1.25     |
| US-1.1.35 | game developer (P-15)   | F-1.1.26 | R-1.1.26     |
| US-1.1.36 | designer (P-5)          | F-1.1.26 | R-1.1.26     |
| US-1.1.37 | game developer (P-15)   | F-1.1.27 | R-1.1.27     |
| US-1.1.38 | QA engineer (P-19)      | F-1.1.28 | R-1.1.28     |
| US-1.1.39 | engine developer (P-26) | F-1.1.28 | R-1.1.28     |
| US-1.1.40 | engine developer (P-26) | F-1.1.29 | R-1.1.29     |

1. **US-1.1.34** — system execution order resolved automatically from declared read/write access
   sets with DAG construction and cycle detection, so that I do not manually order hundreds of
   systems while guaranteeing correctness
   - **Acceptance:** DAG built from declared access sets<br>Topological ordering maximizes
     parallelism<br>Cycles detected and reported at schedule-build time
2. **US-1.1.35** — systems organized into hierarchical groups and phases (PreUpdate, Update,
   PostUpdate, FixedUpdate, PreRender, Render) with before/after ordering, so that I can cleanly
   separate gameplay, physics, and rendering execution
   - **Acceptance:** Built-in phases with defined execution order<br>Custom phases insertable with
     before/after ordering<br>Nested groups (PhysicsGroup inside FixedUpdate)<br>Disabling a group
     disables all contained systems
3. **US-1.1.36** — to see which execution phase each system belongs to in the visual editor, so that
   I understand when gameplay logic runs relative to physics and rendering when debugging timing
   issues
   - **Acceptance:** Visual editor shows system-to-phase mapping<br>Phase execution order visible in
     editor<br>System enable/disable toggleable per group
4. **US-1.1.37** — systems gated by predicates like fixed-timestep accumulators, state machine
   transitions, and resource existence checks, so that systems only run when their conditions are
   met without branching inside system bodies
   - **Acceptance:** Fixed-timestep accumulator criteria<br>State machine transition
     criteria<br>Multiple criteria compose with AND logic
5. **US-1.1.38** — the scheduler to detect pairs of systems with conflicting access and no ordering
   constraint, so that I identify potential nondeterminism before it causes hard-to-reproduce bugs
   in deterministic simulation
   - **Acceptance:** Conflicting access pairs detected<br>Ambiguities reported as warnings at
     schedule-build time<br>Warning includes system names and conflicting components
6. **US-1.1.39** — ambiguity detection reports to include system names, conflicting component types,
   and suggested resolution, so that I can quickly fix ordering issues without manually tracing the
   dependency graph
   - **Acceptance:** Report includes both system names<br>Report lists conflicting component
     types<br>Suggested resolution (add ordering or access change)
7. **US-1.1.40** — systems that acquire exclusive `&mut World` access and act as full barriers, so
   that operations like world serialization and scene loading that cannot be decomposed into queries
   still integrate with the scheduler
   - **Acceptance:** Exclusive system prevents all concurrent execution<br>Scheduler treats
     exclusive systems as barriers<br>World serialization and scene loading use exclusive access

## Observers

| ID        | Persona               | Features | Requirements |
|-----------|-----------------------|----------|--------------|
| US-1.1.41 | game developer (P-15) | F-1.1.30 | R-1.1.30     |
| US-1.1.42 | game developer (P-15) | F-1.1.31 | R-1.1.31     |

1. **US-1.1.41** — to register callbacks that fire when specific events (OnAdd, OnRemove, OnSet)
   occur on entities matching a query at sync points, so that I can implement reactive patterns like
   spatial index updates without polling
   - **Acceptance:** Observers match multi-term queries<br>Evaluated at sync points (deferred), safe
     for structural changes<br>Built-in events: OnAdd, OnRemove, OnSet, OnTableCreate, OnTableEmpty
2. **US-1.1.42** — to define custom event types and emit them at specific entities with propagation
   along relationship edges, so that gameplay events like DamageEvent and PickupEvent bubble up
   parent chains of composite entities
   - **Acceptance:** Custom event types with data payloads<br>Events emitted at specific
     entities<br>Propagation along relationship edges (parent chain)

## Command Buffers

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.1.43 | engine developer (P-26) | F-1.1.32 | R-1.1.32     |
| US-1.1.44 | engine developer (P-26) | F-1.1.33 | R-1.1.33     |
| US-1.1.45 | engine tester (P-27)    | F-1.1.33 | R-1.1.33     |

1. **US-1.1.43** — per-system command buffers that record spawn, despawn, and component operations
   applied in deterministic order at sync points, so that parallel systems can record mutations
   without borrow conflicts
   - **Acceptance:** Per-system command buffer records all structural operations<br>Commands applied
     in deterministic order at sync points<br>No runtime borrow conflicts during parallel execution
2. **US-1.1.44** — multiple worker threads to record commands into the same buffer with sort keys
   for deterministic playback, so that parallel systems share a single command buffer without
   per-thread merge overhead
   - **Acceptance:** Concurrent recording from multiple workers<br>Sort keys ensure deterministic
     playback order<br>No per-thread command buffer merge needed
3. **US-1.1.45** — to verify that parallel command recording produces identical playback order
   regardless of thread scheduling, so that deterministic simulation is maintained even under
   varying thread timing
   - **Acceptance:** Same commands recorded in different thread orders produce same result<br>Sort
     key ordering verified across 100+ runs<br>World state identical after playback regardless of
     recording order

## Worlds

| ID        | Persona               | Features | Requirements |
|-----------|-----------------------|----------|--------------|
| US-1.1.46 | game developer (P-15) | F-1.1.34 | R-1.1.34     |
| US-1.1.47 | game developer (P-15) | F-1.1.35 | R-1.1.35     |
| US-1.1.48 | engine tester (P-27)  | F-1.1.35 | R-1.1.35     |

1. **US-1.1.46** — multiple independent ECS worlds in a single process with per-world system
   instantiation controlled by flags (Game, Editor, Server, Shadow), so that I can isolate rollback
   worlds, streaming staging, and instanced zones
   - **Acceptance:** Each world owns archetype storage, allocator, resource map<br>World flags
     control which systems instantiate<br>Mobile: max 2 concurrent worlds, desktop: configurable
2. **US-1.1.47** — to transfer entities with all components between worlds with automatic ID
   remapping, so that zone transitions in open worlds preserve full entity state without ID
   collisions
   - **Acceptance:** All components transferred during migration<br>Entity IDs remapped to avoid
     collisions<br>Full component state preserved across migration
3. **US-1.1.48** — to stress-test entity migration with hundreds of entities crossing zone
   boundaries simultaneously, so that I can verify no component data is lost or corrupted during
   bulk world-to-world transfer
   - **Acceptance:** 500 entities migrated simultaneously without data loss<br>ID remapping produces
     no collisions<br>All component types preserved including relationships

## Prefabs and Prototypes

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-1.1.49 | designer (P-5)          | F-1.1.36 | R-1.1.36     |
| US-1.1.50 | designer (P-5)          | F-1.1.37 | R-1.1.37     |
| US-1.1.51 | engine developer (P-26) | F-1.1.36 | R-1.1.36     |

1. **US-1.1.49** — prefab templates where instances inherit shared components and automatically
   override on write, so that I can create thousands of similar entities from a single template
   while customizing individual instances in the editor
   - **Acceptance:** Prefab tag marks template entities<br>IsA relationship inherits components from
     prefab<br>Write to inherited component auto-overrides (copy-on-write)<br>Prefab variants
     (prefabs inheriting from prefabs)
2. **US-1.1.50** — nested prefab hierarchies where inner prefab changes propagate to all outer
   instances with slot references, so that I can compose complex game objects from reusable parts
   and update them centrally
   - **Acceptance:** Prefab children instantiate entire subtree<br>Inner prefab changes propagate to
     outer instances<br>Slot references for named access to instantiated children
3. **US-1.1.51** — to implement prefab instantiation that shares component storage between prefab
   and instances until override, so that memory usage is proportional to overrides rather than total
   instance count
   - **Acceptance:** Shared components stored once in prefab<br>Memory proportional to overrides,
     not instances<br>Override detection on mutable access

## State Machines

| ID        | Persona               | Features | Requirements |
|-----------|-----------------------|----------|--------------|
| US-1.1.52 | game developer (P-15) | F-1.1.38 | R-1.1.38     |
| US-1.1.53 | designer (P-5)        | F-1.1.38 | R-1.1.38     |

1. **US-1.1.52** — typed state components with OnEnter, OnExit, and OnTransition observer hooks that
   gate system execution, so that game state management (menu, loading, gameplay, paused) integrates
   naturally with the ECS scheduler
   - **Acceptance:** States are components; transitions replace one with another<br>OnEnter, OnExit,
     OnTransition observers fire<br>Sub-states and computed states supported<br>Run criteria gated
     on active state
2. **US-1.1.53** — to see the current game state and available transitions in the visual editor, so
   that I understand which systems are active in each state and can debug state transition issues
   visually
   - **Acceptance:** Current state visible in editor<br>Available transitions shown per
     state<br>Systems gated by each state identifiable
