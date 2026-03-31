# R-1.1 — Entity Component System Requirements

## Storage

1. **R-1.1.1** — The engine **SHALL** store components in contiguous archetype tables using
   structure-of-arrays layout, where each unique combination of component types defines a distinct
   archetype, subdivided into fixed-size chunks (default 16 KiB, configurable 8-64 KiB).
   - **Rationale:** SoA layout maximizes iteration throughput and SIMD potential for large entity
     populations.
   - **Verification:** Benchmark iterating 1M entities with 3 component types; verify throughput
     exceeds 500M components/second per core. Validate contiguous per-type arrays via pointer
     arithmetic assertions.
2. **R-1.1.2** — The engine **SHALL** achieve at least 500 million component reads/second per core
   when iterating dense archetype tables. Chunk alignment **SHALL** match the platform cache line
   size (64 bytes). Per-archetype metadata overhead **SHALL NOT** exceed 256 bytes.
   - **Rationale:** Hard performance bounds ensure the storage layer meets frame-time budgets across
     platforms.
   - **Verification:** Benchmark: iterate 100K, 500K, 1M, and 5M entities; verify constant
     per-entity throughput (within 5% variance). Verify 64-byte chunk alignment.
3. **R-1.1.3** — The engine **SHALL** support sparse-set storage for components annotated with
   `#[sparse]`, such that adding or removing a sparse component does not change the entity's
   archetype.
   - **Rationale:** High-churn components like debug markers cause excessive archetype fragmentation
     in dense tables.
   - **Verification:** Unit test: add and remove a sparse component 10,000 times; assert the
     entity's archetype ID remains unchanged. Benchmark lookup at O(1).
4. **R-1.1.4** — The engine **SHALL** maintain a directed archetype graph where edges represent
   single-component additions or removals, resolving the target archetype in O(1) amortized time via
   cached edge lookups.
   - **Rationale:** Linear archetype scans during structural changes become a bottleneck when
     thousands of entities are spawned per frame.
   - **Verification:** Benchmark: spawn 100,000 entities; verify per-entity resolution time does not
     increase with archetype count. Support at least 10,000 distinct archetypes per world without
     degradation.

## Components

1. **R-1.1.5** — The engine **SHALL** support both compile-time component registration via derive
   macros and runtime registration via a dynamic API, recording size, alignment, drop function, and
   storage mode per type.
   - **Rationale:** Compile-time registration enables zero-cost access; runtime registration
     supports hot-reloaded gameplay logic.
   - **Verification:** Integration test: register a component dynamically, attach it, query it, and
     verify correct data. Verify static registration produces monomorphized query code.
2. **R-1.1.6** — The engine **SHALL** treat components with no data fields as zero-sized tags that
   alter archetype identity for query filtering but consume zero bytes of per-entity storage.
   - **Rationale:** Tags enable filtering without wasting memory across millions of entities.
   - **Verification:** Add a tag to 100,000 entities; assert zero bytes memory increase for the tag
     column. Verify With query correctly filters.
3. **R-1.1.7** — The engine **SHALL** support shared components stored once per chunk, where
   changing a shared value triggers a structural change moving the entity to a different chunk.
   - **Rationale:** Components like material IDs are identical across many entities; per-entity
     storage wastes memory.
   - **Verification:** Assign same value to 10,000 entities; assert stored once. Modify on one
     entity; assert it migrates to new chunk.
4. **R-1.1.8** — The engine **SHALL** provide variable-length buffer components via
   `DynamicBuffer<T>`, with inline small buffers in archetype chunks and heap spill for large
   buffers.
   - **Rationale:** Per-entity collections require dynamic sizing without archetype changes on
     resize.
   - **Verification:** Append elements past inline capacity; verify heap spill. Shrink below inline;
     verify return to inline. Assert correct iteration throughout.
5. **R-1.1.9** — The engine **SHALL** support components marked `#[enableable]` that can be toggled
   on/off per entity without structural changes, excluded from queries by default but accessible via
   `WithDisabled<T>` or `WithPresent<T>` filters.
   - **Rationale:** Toggling without archetype changes avoids costly entity migrations for frequent
     toggling.
   - **Verification:** Toggle off; verify default query excludes. Toggle from parallel worker
     thread; verify immediate visibility in subsequent queries.
6. **R-1.1.10** — The engine **SHALL** support per-type lifecycle hooks (`on_add`, `on_remove`,
   `on_set`) that execute synchronously at the point of change, receiving entity ID and component
   reference. Dispatch overhead **SHALL NOT** exceed 50 ns per invocation beyond the hook body.
   - **Rationale:** Hooks enable maintaining auxiliary data structures in lockstep with component
     changes.
   - **Verification:** Register hooks; perform add, set, remove; assert each fires exactly once with
     correct data. Benchmark no-op hook overhead under 50 ns.
7. **R-1.1.11** — The engine **SHALL** support named component bundles for atomic multi-component
   insertion, automatically adding required companion components declared via `#[require]` if not
   already present.
   - **Rationale:** Bundles reduce boilerplate and prevent incomplete entity construction.
   - **Verification:** Insert a 4-component bundle; verify all present. Insert a component with a
     required companion; verify auto-added. Verify no duplication if companion already exists.

## Entities

1. **R-1.1.12** — The engine **SHALL** use 64-bit generational indices (32-bit index + 32-bit
   generation) as entity identifiers, providing O(1) allocation, O(1) deallocation, and O(1)
   stale-reference detection. The engine **SHALL** support at least 4 million live entities per
   world.
   - **Rationale:** Generational indices prevent use-after-free bugs when entities are recycled.
   - **Verification:** Allocate an entity, despawn it, allocate at same index; verify old ID fails
     validation. Benchmark 4M entities; verify under 100 ns per alloc/dealloc.
2. **R-1.1.13** — The engine **SHALL** support `#[cleanup]` components that persist after entity
   destruction, enabling resource teardown systems to detect lingering entities and finalize
   destruction by removing cleanup components.
   - **Rationale:** GPU buffers and network registrations need graceful teardown that cannot happen
     synchronously.
   - **Verification:** Attach cleanup + normal components. Despawn entity. Assert entity alive with
     only cleanup component. Remove cleanup; assert fully destroyed.
3. **R-1.1.14** — The engine **SHALL** allow assigning human-readable names to entities, forming
   hierarchical path names via parent-child relationships, with O(log n) lookup by path. Names
   **SHALL** be stored as sparse components.
   - **Rationale:** Named entities support debugging, Logic Graph references, and editor entity
     trees.
   - **Verification:** Build 3-level named hierarchy; look up leaf by full path; verify correct
     entity returned. Benchmark 10,000 lookups in 100,000 named entities.

## Relationships and Hierarchies

1. **R-1.1.15** — The engine **SHALL** encode entity relationships as (Relationship, Target) pairs
   packed into 64-bit component IDs, supporting wildcard queries on either element.
   - **Rationale:** Pair-based relationships are the foundation for hierarchies, Entity Templates,
     and graph-based data modeling.
   - **Verification:** Add (Likes, Apple) and (Likes, Banana); query (Likes,
     *) and verify both returned; query (*, Apple) and verify only apple-liking entity.
2. **R-1.1.16** — The engine **SHALL** support relationship properties including Exclusive,
   Symmetric, Transitive, Acyclic, and cleanup policies (OnDelete, OnDeleteTarget), enforcing
   semantics automatically.
   - **Rationale:** Properties encode domain invariants that would otherwise require hand-written
     logic.
   - **Verification:** Test each property: Exclusive removes previous target; Symmetric auto-adds
     reverse; OnDeleteTarget(Delete) cascades deletion.
3. **R-1.1.17** — The engine **SHALL** provide a built-in ChildOf relationship with Acyclic,
   Traversable, and OnDeleteTarget(Delete) properties, supporting up and cascade traversal.
   Hierarchies **SHALL** support at least 256 levels without stack overflow.
   - **Rationale:** Parent-child hierarchy is fundamental for scene graphs and transform
     propagation.
   - **Verification:** Build 4-level hierarchy; delete root; assert all descendants destroyed.
     Verify up traversal from leaf to root. Verify no stack overflow at 256 levels. Cascade-delete
     100K subtree within 10 ms.

## Queries

1. **R-1.1.18** — The engine **SHALL** support composable archetype queries with With, Without,
   Option, Changed, and Added filters, caching after first evaluation so repeated execution within a
   frame incurs zero additional archetype matching.
   - **Rationale:** Cached queries are essential when hundreds of systems re-evaluate per frame.
   - **Verification:** Construct query with all five filters; verify correct inclusion/exclusion.
     Run cached query 1,000 times; verify zero additional matching cost. Cache metadata under 1 KiB
     per query.
2. **R-1.1.19** — The engine **SHALL** support sorting query results by component value and grouping
   by relationship target, with stable cached sort re-sorted only on change detection.
   - **Rationale:** Sorted iteration enables draw call batching and spatial partitioning without
     external sorting.
   - **Verification:** Create 1,000 entities; sort by value; verify order. Modify 10; re-sort;
     verify only modified subset triggers re-sort. Verify sort stability.
3. **R-1.1.20** — The engine **SHALL** support query variables (e.g., $parent, $target) enabling
   graph pattern matching across relationships in a single query pass.
   - **Rationale:** Variable-based matching avoids nested loops for relationship-heavy queries.
   - **Verification:** Create parent-child pairs where some parents have Boss component. Query
     (ChildOf, $parent), $parent.Has<Boss>; verify only children of boss entities returned.
4. **R-1.1.21** — The engine **SHALL** partition query results across worker threads at archetype or
   chunk granularity, guaranteeing no conflicting mutable and immutable borrows to the same
   component storage.
   - **Rationale:** Parallel iteration must scale linearly with core count for large entity
     populations.
   - **Verification:** Benchmark 1M entities on 1, 2, 4, 8 cores; verify near-linear speedup (3.5x
     on 4 cores). Stress test under ThreadSanitizer for no data races.

## Aspects

1. **R-1.1.22** — The engine **SHALL** support named aspect structs grouping a subset of an entity's
   components into a single type parameter for queries, with per-field access mode declarations and
   nested aspect support.
   - **Rationale:** Aspects reduce boilerplate and enforce consistent component groupings.
   - **Verification:** Define PhysicsAspect with 4 components; query using aspect; verify correct
     access modes. Nest a TransformAspect inside; verify nested access.

## Change Detection

1. **R-1.1.23** — The engine **SHALL** track per-component mutation at chunk granularity using a
   tick counter incremented on each mutable access. Per-chunk metadata **SHALL NOT** exceed 8 bytes
   per component type per chunk.
   - **Rationale:** Skipping unchanged chunks enables reactive patterns and reduces work for network
     delta compression and spatial index updates.
   - **Verification:** Mutate one entity's component in a chunk; verify chunk marked changed. Next
     tick without mutations; verify Changed returns no results. Verify metadata matches 8 bytes per
     component type per chunk.

## Resources and Singletons

1. **R-1.1.24** — The engine **SHALL** support typed singleton resources accessed via Res (shared)
   and ResMut (exclusive) system parameters, participating in the scheduler's dependency graph and
   change detection.
   - **Rationale:** Global state like Time and InputState needs type-safe, scheduler-aware access.
   - **Verification:** Insert resource; read via Res, write via ResMut; verify scheduler orders
     correctly. Verify Changed detects resource mutations.
2. **R-1.1.25** — The engine **SHALL** ensure non-send resources are only accessed from the game
   loop thread. Systems accessing non-send resources **SHALL** be automatically pinned to the game
   loop thread.
   - **Rationale:** GPU device handles and windowing handles are not thread-safe.
   - **Verification:** Register non-send resource; verify system executes on game loop thread.
     Verify scheduler prevents worker thread access.

## System Scheduling

1. **R-1.1.26** — The engine **SHALL** automatically resolve system execution order from declared
   read/write access sets by building a DAG and producing a topological ordering. Cycles **SHALL**
   be detected and reported at schedule-build time.
   - **Rationale:** Manual ordering is error-prone and does not scale; automatic resolution
     maximizes parallelism.
   - **Verification:** Register systems with dependencies; verify correct order. Register cycle;
     verify error reported. Schedule build under 50 ms for 500 systems.
2. **R-1.1.27** — The engine **SHALL** organize systems into hierarchical groups with built-in
   phases (PreUpdate, Update, PostUpdate, FixedUpdate, PreRender, Render), support custom phases
   with before/after ordering, and disable all systems when a group is disabled.
   - **Rationale:** Phases provide coarse execution structure; group disable controls subsystem
     activation.
   - **Verification:** Register systems in phases; verify order. Disable group; verify no systems
     execute. Insert custom phase between built-ins; verify order.
3. **R-1.1.28** — The engine **SHALL** support system run criteria including fixed-timestep
   accumulators, state machine transitions, resource existence checks, and user-defined predicates,
   composable with AND logic.
   - **Rationale:** Run criteria prevent unnecessary execution without branching inside system
     bodies.
   - **Verification:** Gate on boolean resource; toggle; verify system runs only when met. Compose
     two criteria with AND; verify both must be satisfied.
4. **R-1.1.29** — The engine **SHALL** detect pairs of systems with conflicting access modes and no
   ordering constraint, reporting ambiguities as warnings at schedule-build time.
   - **Rationale:** Unordered conflicting access causes nondeterminism harmful for deterministic
     simulation.
   - **Verification:** Register read + write systems on same component with no ordering; verify
     warning emitted naming both systems and conflicting component.
5. **R-1.1.30** — The engine **SHALL** support exclusive systems acquiring &mut World access,
   preventing all concurrent execution. The scheduler **SHALL** treat exclusive systems as full
   barriers.
   - **Rationale:** World serialization and scene loading require full world access.
   - **Verification:** Register exclusive system alongside parallel systems; verify it runs alone.
     Verify systems before/after still run in parallel with each other.

## Observers

1. **R-1.1.31** — The engine **SHALL** support observers that fire callbacks when built-in events
   (OnAdd, OnRemove, OnSet, OnTableCreate, OnTableEmpty) occur on entities matching a multi-term
   query, evaluated at sync points. The engine **SHALL** support at least 1,000 observers per world.
   - **Rationale:** Deferred evaluation prevents iterator invalidation while enabling reactive
     patterns.
   - **Verification:** Register observer for OnAdd of A on entities with B. Add A to entities
     with/without B. Verify fires only for matching entities at sync point. Dispatch in O(e * m)
     where e = events, m = matching observers.
2. **R-1.1.32** — The engine **SHALL** support user-defined event types emitted at specific
   entities, with observers firing for matching entities and event propagation along relationship
   edges (bubbling up parent chains).
   - **Rationale:** Gameplay events like DamageEvent need entity-targeted delivery with hierarchy
     propagation.
   - **Verification:** Define DamageEvent; emit at child; register observers on child and parent.
     Verify child observer fires first, then bubbles to parent via ChildOf edge.

## Command Buffers

1. **R-1.1.33** — The engine **SHALL** provide per-system command buffers recording spawn, despawn,
   and component operations, applying them in deterministic order at sync points. Per-system buffer
   memory **SHALL NOT** exceed 64 KiB under typical usage.
   - **Rationale:** Deferred application eliminates borrow conflicts and ensures reproducible
     simulation.
   - **Verification:** Record commands from two parallel systems; flush; verify deterministic order
     across runs. Benchmark 100,000 commands; verify flush under 1 ms.
2. **R-1.1.34** — The engine **SHALL** allow multiple worker threads to record commands into the
   same command buffer concurrently with sort keys for deterministic playback.
   - **Rationale:** Shared buffer with sort keys simplifies parallel system authoring.
   - **Verification:** Record 100,000 commands from 8 threads; verify playback matches sort-key
     order exactly across 100 iterations.

## Worlds

1. **R-1.1.35** — The engine **SHALL** support multiple independent ECS worlds within a single
   process, each owning its own archetype storage, entity allocator, and resource map, tagged with
   flags (Game, Editor, Server, Shadow) controlling system instantiation.
   - **Rationale:** Multiple worlds enable rollback, streaming staging, and instanced zone
     isolation.
   - **Verification:** Create two worlds with different flags; verify each runs only its flagged
     systems. Verify entities in one world invisible to the other.
2. **R-1.1.36** — The engine **SHALL** support transferring entities with all components from one
   world to another, remapping entity IDs to avoid collisions. Migration **SHALL** complete in under
   10 us per entity.
   - **Rationale:** Zone transitions require preserving full component state across worlds.
   - **Verification:** Create entity with 5 components + relationships in world A; migrate to world
     B; verify all data intact. Verify original removed from A. No ID collisions. Return diagnostic
     error if target world lacks a required component type registration.

## Entity Templates and Prototypes

1. **R-1.1.37** — The engine **SHALL** support Entity Template entities via an IsA relationship,
   where instances inherit shared components and automatically override (copy-on-write) inherited
   components when written.
   - **Rationale:** Entity Template inheritance avoids duplicating shared data across thousands of
     instances.
   - **Verification:** Create template with 3 components; instantiate 100 via IsA. Verify shared
     data. Modify one instance; verify only that instance overrides while 99 still share template
     values.
2. **R-1.1.38** — The engine **SHALL** instantiate Entity Template child hierarchies as complete
   subtrees, support nested template composition with change propagation, and provide slot
   references for named child access.
   - **Rationale:** Complex game objects are composed of nested template hierarchies that must stay
     synchronized.
   - **Verification:** Create nested template (outer containing inner child). Instantiate 10 outer
     instances. Modify inner template component; verify all 10 outer instances reflect the change.
     Verify slot references resolve correctly.

## State Machines

1. **R-1.1.39** — The engine **SHALL** support typed state components with OnEnter, OnExit, and
   OnTransition observer hooks, where transitions replace one state component with another. Systems
   **SHALL** declare run criteria gated on active state. Sub-states and computed states **SHALL** be
   supported.
   - **Rationale:** Game state management must integrate with the ECS scheduler to
     activate/deactivate systems.
   - **Verification:** Define states Menu, Playing, Paused. Transition Menu to Playing; verify
     OnExit(Menu) and OnEnter(Playing) fire. Verify system with in_state(Playing) runs during
     Playing, not during Menu.
