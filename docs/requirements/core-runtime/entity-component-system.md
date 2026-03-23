# R-1.1 — Entity Component System Requirements

## Storage

| ID       | Derived From                                                      |
|----------|-------------------------------------------------------------------|
| R-1.1.1  | [F-1.1.1](../../features/core-runtime/entity-component-system.md) |
| R-1.1.1a | [F-1.1.1](../../features/core-runtime/entity-component-system.md) |
| R-1.1.2  | [F-1.1.2](../../features/core-runtime/entity-component-system.md) |
| R-1.1.2a | [F-1.1.2](../../features/core-runtime/entity-component-system.md) |
| R-1.1.3  | [F-1.1.3](../../features/core-runtime/entity-component-system.md) |
| R-1.1.3a | [F-1.1.3](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.1** — The engine **SHALL** store components in contiguous archetype tables using
   structure-of-arrays layout, where each unique combination of component types defines a distinct
   archetype, and each component type occupies a separate contiguous array within the table
   subdivided into fixed-size chunks (default 16 KiB).
   - **Rationale:** Cache-friendly SoA layout maximizes iteration throughput and SIMD potential for
     systems processing large entity populations.
   - **Verification:** Benchmark iterating 1 million entities with 3 component types; verify
     throughput exceeds 500 million components/second on a single core. Validate memory layout with
     pointer arithmetic assertions confirming contiguous per-type arrays.
2. **R-1.1.1a** — The engine **SHALL** achieve at least 500 million component reads/second per core
   when iterating dense archetype tables. Per-archetype metadata overhead **SHALL NOT** exceed 256
   bytes. Chunk alignment **SHALL** match the platform cache line size (64 bytes). The system
   **SHALL** support at least 1 million entities per world without degradation in per-entity
   iteration throughput.
   - **Rationale:** Hard performance bounds ensure the storage layer meets frame-time budgets for
     large entity populations across all supported platforms.
   - **Verification:** Benchmark: iterate 100K, 500K, 1M, and 5M entities and verify per-entity
     throughput remains constant (within 5% variance). Verify chunk base addresses are 64-byte
     aligned. Verify per-archetype metadata footprint via allocator instrumentation.
3. **R-1.1.2** — The engine **SHALL** support sparse-set storage for components annotated with
   `#[sparse]`, such that adding or removing a sparse component does not change the entity's
   archetype.
   - **Rationale:** High-churn components like debug markers or temporary status effects cause
     excessive archetype fragmentation when stored in dense tables.
   - **Verification:** Unit test: add and remove a `#[sparse]` component 10,000 times on an entity
     and assert the entity's archetype ID remains unchanged throughout.
4. **R-1.1.2a** — The engine **SHALL** complete sparse component lookup in O(1) time per entity.
   Sparse storage memory overhead **SHALL NOT** exceed 16 bytes per entity per sparse component type
   (index + generation + padding). Adding or removing a sparse component **SHALL** complete in under
   200 nanoseconds.
   - **Rationale:** Sparse storage must remain efficient for high-churn components accessed
     frequently during simulation ticks.
   - **Verification:** Benchmark: lookup, add, and remove sparse components on 100,000 entities.
     Verify per-operation latency. Measure memory overhead per sparse component type and verify it
     stays within the 16-byte-per-entity bound.
5. **R-1.1.3** — The engine **SHALL** maintain a directed archetype graph where edges represent
   single-component additions or removals, and **SHALL** resolve the target archetype for any
   structural change in O(1) amortized time via cached edge lookups.
   - **Rationale:** Linear archetype scans during structural changes become a bottleneck when
     thousands of entities are spawned or migrated per frame.
   - **Verification:** Benchmark: spawn 100,000 entities with identical component sets and measure
     that per-entity archetype resolution time does not increase with archetype count. Profile
     confirms no linear search in the hot path.
6. **R-1.1.3a** — The engine **SHALL** support at least 10,000 distinct archetypes per world without
   degrading edge lookup performance below O(1) amortized. Graph memory overhead **SHALL NOT**
   exceed 128 bytes per edge. The system **SHALL** report a diagnostic error if the archetype count
   exceeds a configurable warning threshold (default 50,000).
   - **Rationale:** Large games with many component combinations can produce thousands of
     archetypes; the graph must scale without hidden performance cliffs.
   - **Verification:** Stress test: create 10,000 archetypes by varying component combinations.
     Verify edge lookup remains O(1). Verify the warning threshold triggers a diagnostic message at
     the configured limit.

## Components

| ID       | Derived From                                                       |
|----------|--------------------------------------------------------------------|
| R-1.1.4  | [F-1.1.4](../../features/core-runtime/entity-component-system.md)  |
| R-1.1.5  | [F-1.1.5](../../features/core-runtime/entity-component-system.md)  |
| R-1.1.6  | [F-1.1.6](../../features/core-runtime/entity-component-system.md)  |
| R-1.1.7  | [F-1.1.7](../../features/core-runtime/entity-component-system.md)  |
| R-1.1.8  | [F-1.1.8](../../features/core-runtime/entity-component-system.md)  |
| R-1.1.9  | [F-1.1.9](../../features/core-runtime/entity-component-system.md)  |
| R-1.1.10 | [F-1.1.10](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.4** — The engine **SHALL** support both compile-time component registration via derive
   macros and runtime component registration via a dynamic API, recording size, alignment, drop
   function, and storage mode for each component type.
   - **Rationale:** Compile-time registration enables zero-cost access, while runtime registration
     supports hot-reloaded gameplay logic without recompilation.
   - **Verification:** Integration test: register a component dynamically at runtime, attach it to
     an entity, query it, and verify correct data retrieval. Verify statically registered components
     produce monomorphized query code (inspect generated assembly or compilation output).
2. **R-1.1.5** — The engine **SHALL** treat components with no data fields as zero-sized tags that
   alter archetype identity for query filtering but consume zero bytes of per-entity storage.
   - **Rationale:** Tags like `Player`, `Enemy`, or `Static` must enable query filtering without
     wasting memory across potentially millions of entities.
   - **Verification:** Unit test: add a zero-sized tag to 100,000 entities. Assert total archetype
     table memory increase is zero bytes for the tag column. Verify `With<Tag>` query correctly
     includes tagged entities and excludes untagged ones.
3. **R-1.1.6** — The engine **SHALL** support shared components that are stored once per chunk
   rather than per entity, where changing a shared component value triggers a structural change
   moving the entity to a different chunk.
   - **Rationale:** Components like material IDs and LOD groups are identical across many entities;
     storing them per-entity wastes memory.
   - **Verification:** Unit test: assign the same shared component value to 10,000 entities in one
     chunk. Assert the shared value is stored exactly once. Modify the value on one entity and
     assert it migrates to a new chunk while the original 9,999 remain unchanged.
4. **R-1.1.7** — The engine **SHALL** provide variable-length buffer components accessed via
   `DynamicBuffer<T>`, with small buffers stored inline in archetype chunks and large buffers
   spilling to heap allocations.
   - **Rationale:** Per-entity collections (child lists, inventory slots, waypoints) require dynamic
     sizing without archetype changes on resize.
   - **Verification:** Unit test: create a buffer component, append elements until it exceeds inline
     capacity, verify spill to heap, then shrink below inline capacity and verify return to inline
     storage. Assert iteration over buffer elements yields correct values throughout.
5. **R-1.1.8** — The engine **SHALL** support components marked `#[enableable]` that can be toggled
   on/off per entity without structural changes, where disabled components are excluded from queries
   by default but accessible via explicit `WithDisabled<T>` or `WithPresent<T>` filters.
   - **Rationale:** Toggling component participation without archetype changes avoids costly entity
     migrations for frequently toggled behaviors.
   - **Verification:** Unit test: toggle an enableable component off, verify default query excludes
     the entity, verify `WithDisabled<T>` query includes it, toggle back on from a parallel worker
     thread without command buffers and verify immediate visibility in default queries.
6. **R-1.1.9** — The engine **SHALL** support per-type lifecycle hooks (`on_add`, `on_remove`,
   `on_set`) that execute synchronously at the point of change, receiving the entity ID and a
   reference to the component value.
   - **Rationale:** Hooks enable maintaining auxiliary data structures (e.g., spatial index updates)
     in lockstep with component changes.
   - **Verification:** Unit test: register `on_add`, `on_remove`, and `on_set` hooks on a component
     type. Perform add, set, and remove operations on an entity. Assert each hook fires exactly once
     with the correct entity ID and component value, in the correct order.
7. **R-1.1.10** — The engine **SHALL** support named component bundles for atomic multi-component
   insertion, and **SHALL** automatically add required companion components (declared via
   `#[require]`) when a component is inserted if the companion is not already present.
   - **Rationale:** Bundles reduce boilerplate and prevent incomplete entity construction; required
     components enforce invariants automatically.
   - **Verification:** Unit test: insert a bundle containing 4 components and verify all are
     present. Insert a component with a required companion and verify the companion is auto-added.
     Insert the same component on an entity that already has the companion and verify the companion
     is not duplicated or overwritten.

## Entities

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.9a  | [F-1.1.9](../../features/core-runtime/entity-component-system.md)  |
| R-1.1.11  | [F-1.1.11](../../features/core-runtime/entity-component-system.md) |
| R-1.1.12  | [F-1.1.12](../../features/core-runtime/entity-component-system.md) |
| R-1.1.13  | [F-1.1.13](../../features/core-runtime/entity-component-system.md) |
| R-1.1.11a | [F-1.1.11](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.9a** — Component lifecycle hooks (`on_add`, `on_remove`, `on_set`) **SHALL** add no more
   than 50 nanoseconds of dispatch overhead per invocation beyond the hook body's own execution
   time. The engine **SHALL** limit the total number of registered hooks per component type to 16
   and report a diagnostic error if this limit is exceeded.
   - **Rationale:** Hooks execute synchronously in the hot path; excessive dispatch overhead or
     unbounded hook chains degrade frame times.
   - **Verification:** Benchmark: register a no-op hook and measure per-invocation overhead on
     100,000 component additions. Verify overhead is under 50 ns. Attempt to register a 17th hook
     and verify an error is reported.
2. **R-1.1.11** — The engine **SHALL** use 64-bit generational indices (32-bit index + 32-bit
   generation) as entity identifiers, providing O(1) allocation, O(1) deallocation, and O(1)
   stale-reference detection without indirection through a free list.
   - **Rationale:** Generational indices prevent use-after-free bugs when entities are recycled,
     which is critical for a long-running server.
   - **Verification:** Unit test: allocate an entity, record its ID, despawn it, allocate a new
     entity at the same index, and verify the old ID fails validation due to generation mismatch.
     Benchmark: allocate and despawn 1 million entities and verify O(1) per-operation cost.
3. **R-1.1.12** — The engine **SHALL** support `#[cleanup]` components that persist after entity
   destruction (removal of all non-cleanup components), enabling resource teardown systems to detect
   lingering entities and finalize destruction by removing cleanup components.
   - **Rationale:** GPU buffers, network registrations, and other external resources need graceful
     teardown that cannot happen synchronously during entity despawn.
   - **Verification:** Unit test: attach a `#[cleanup]` component and a normal component to an
     entity. Despawn the entity. Assert the entity is still alive with only the cleanup component.
     Remove the cleanup component and assert the entity is fully destroyed.
4. **R-1.1.13** — The engine **SHALL** allow assigning human-readable names to entities, forming
   hierarchical path names via parent-child relationships (e.g., `World/Zone3/NPC_Guard_17`), with
   O(log n) lookup by path.
   - **Rationale:** Named entities support debugging, scripting references, and editor entity trees
     without raw entity ID manipulation.
   - **Verification:** Unit test: build a 3-level hierarchy with named entities, look up a leaf
     entity by full path, and verify correct entity is returned. Benchmark: perform 10,000 path
     lookups in a world with 100,000 named entities and verify O(log n) scaling.
5. **R-1.1.11a** — The engine **SHALL** support at least 4 million live entities per world. Entity
   allocation and deallocation **SHALL** each complete in under 100 nanoseconds amortized. The free
   list **SHALL NOT** cause generation counter overflow for at least 2^32 allocate/deallocate cycles
   per index slot.
   - **Rationale:** MMO servers and large open worlds may host millions of entities; the allocator
     must scale without performance cliffs or generation exhaustion.
   - **Verification:** Stress test: allocate and deallocate 4 million entities. Verify per-operation
     latency remains under 100 ns. Verify generation counter does not wrap during the test lifetime.

## Relationships and Hierarchies

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.14  | [F-1.1.14](../../features/core-runtime/entity-component-system.md) |
| R-1.1.15  | [F-1.1.15](../../features/core-runtime/entity-component-system.md) |
| R-1.1.16  | [F-1.1.16](../../features/core-runtime/entity-component-system.md) |
| R-1.1.16a | [F-1.1.16](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.14** — The engine **SHALL** encode entity relationships as (Relationship, Target) pairs
   packed into a single 64-bit component ID, supporting wildcard queries such as "all entities that
   have relationship R with any target" or "all entities targeting entity E."
   - **Rationale:** Pair-based relationships are the foundation for hierarchies, prefabs, and
     graph-based data modeling without custom data structures.
   - **Verification:** Unit test: add relationships `(Likes, Apple)` and `(Likes, Banana)` to
     entities. Query with wildcard `(Likes, *)` and verify both are returned. Query with
     `(*, Apple)` and verify only the apple-liking entity is returned.
2. **R-1.1.15** — The engine **SHALL** support relationship properties including `Exclusive`,
   `Symmetric`, `Transitive`, `Acyclic`, and cleanup policies (`OnDelete`, `OnDeleteTarget`),
   enforcing each property's semantics automatically when relationships are added or targets are
   deleted.
   - **Rationale:** Relationship properties encode domain invariants (e.g., exactly one parent,
     symmetric friendship) that would otherwise require hand-written enforcement logic.
   - **Verification:** Unit test per property: (1) `Exclusive` — add a second target and verify the
     first is removed. (2) `Symmetric` — add A->B and verify B->A exists. (3)
     `OnDeleteTarget(Delete)` — delete the target and verify the source entity is also deleted.
3. **R-1.1.16** — The engine **SHALL** provide a built-in `ChildOf` relationship with `Acyclic`,
   `Traversable`, and `OnDeleteTarget(Delete)` properties, supporting `up` (parent chain) and
   `cascade` (breadth-first top-down) traversal, with parent deletion cascading destruction to all
   descendants.
   - **Rationale:** Parent-child hierarchy is fundamental for scene graphs, transform propagation,
     and entity ownership.
   - **Verification:** Unit test: build a 4-level hierarchy, delete the root, and assert all
     descendants are destroyed. Verify `up` traversal from a leaf reaches the root. Verify `cascade`
     traversal from root visits all descendants in breadth-first order.
4. **R-1.1.16a** — The engine **SHALL** support parent-child hierarchies of at least 256 levels deep
   without stack overflow. Attempts to create a cycle in the `ChildOf` relationship **SHALL** be
   detected and rejected with a diagnostic error. Cascading deletion of a subtree with more than
   100,000 descendants **SHALL** complete within 10 milliseconds.
   - **Rationale:** Deep hierarchies arise from complex vehicles, skeletal rigs, and nested prefabs;
     cycles must be prevented to avoid infinite loops in traversal.
   - **Verification:** Unit test: build a 256-level hierarchy and verify traversal completes without
     stack overflow. Attempt to create a cycle and verify the error is reported. Benchmark: cascade-
     delete a 100,000-entity subtree and verify completion within 10 ms.

## Queries

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.17  | [F-1.1.17](../../features/core-runtime/entity-component-system.md) |
| R-1.1.18  | [F-1.1.18](../../features/core-runtime/entity-component-system.md) |
| R-1.1.19  | [F-1.1.19](../../features/core-runtime/entity-component-system.md) |
| R-1.1.17a | [F-1.1.17](../../features/core-runtime/entity-component-system.md) |
| R-1.1.20  | [F-1.1.20](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.17** — The engine **SHALL** support composable archetype queries with `With<T>`,
   `Without<T>`, `Option<T>`, `Changed<T>`, and `Added<T>` filters, caching query results after
   first evaluation so that repeated execution within a frame incurs zero archetype-matching
   overhead.
   - **Rationale:** Cached queries are essential for performance when hundreds of systems execute
     per frame, each re-evaluating queries against potentially thousands of archetypes.
   - **Verification:** Unit test: construct a query with all five filter types and verify correct
     entity inclusion/exclusion. Benchmark: run the same cached query 1,000 times in a frame and
     verify zero additional archetype matching cost beyond the first run.
2. **R-1.1.18** — The engine **SHALL** support sorting query results by a component value and
   grouping by relationship target, with stable sort order cached between frames and re-sorted only
   when change detection indicates modifications.
   - **Rationale:** Sorted iteration enables draw call batching by material ID and spatial
     partitioning without external sorting passes.
   - **Verification:** Unit test: create 1,000 entities with integer component values, sort by
     value, verify ascending order. Modify 10 entities, re-sort, and verify only the modified subset
     triggers re-sorting. Verify sort stability across frames for equal keys.
3. **R-1.1.19** — The engine **SHALL** support query variables (e.g., `$parent`, `$target`) that
   enable graph pattern matching across entity relationships, joining results across multiple
   relationship hops in a single query pass.
   - **Rationale:** Variable-based pattern matching avoids nested loops and manual joins for
     relationship-heavy queries.
   - **Verification:** Unit test: create parent-child pairs where some parents have a `Boss`
     component. Query `(ChildOf, $parent), $parent.Has<Boss>` and verify only children of boss
     entities are returned.
4. **R-1.1.17a** — Cached query metadata **SHALL NOT** exceed 1 KiB per cached query. The cache
   **SHALL** support at least 2,000 active cached queries per world. Cache invalidation on new
   archetype creation **SHALL** complete in O(n) where n is the number of cached queries, not the
   number of archetypes.
   - **Rationale:** Large games may register hundreds of systems, each with multiple queries;
     unbounded cache growth or expensive invalidation degrades startup and frame times.
   - **Verification:** Unit test: create 2,000 cached queries, verify total memory overhead is under
     2 MiB. Create a new archetype and measure invalidation time; verify it scales linearly with
     query count.
5. **R-1.1.20** — The engine **SHALL** partition query results across worker threads at archetype or
   chunk granularity, guaranteeing no two parallel tasks hold conflicting mutable and immutable
   borrows to the same component storage.
   - **Rationale:** Parallel iteration must scale linearly with core count for large entity
     populations to meet frame-time budgets.
   - **Verification:** Benchmark: iterate 1 million entities with a write query on 1, 2, 4, and 8
     cores. Verify near-linear speedup (at least 3.5x on 4 cores). Stress test: run parallel
     iteration with conflicting access patterns and verify no data races via Miri or
     ThreadSanitizer.

## Aspects

| ID       | Derived From                                                       |
|----------|--------------------------------------------------------------------|
| R-1.1.21 | [F-1.1.21](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.21** — The engine **SHALL** support named aspect structs that group a subset of an
   entity's components into a single type parameter for use in queries, with per-field access mode
   declarations (`&T` vs `&mut T`) and support for nested aspects.
   - **Rationale:** Aspects reduce query boilerplate and enforce consistent component groupings
     across related systems.
   - **Verification:** Unit test: define a `PhysicsAspect` grouping 4 components, query using the
     aspect, and verify all component fields are accessible with correct access modes. Nest a
     `TransformAspect` inside `PhysicsAspect` and verify nested field access works.

## Change Detection

| ID       | Derived From                                                       |
|----------|--------------------------------------------------------------------|
| R-1.1.22 | [F-1.1.22](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.22** — The engine **SHALL** track per-component mutation at chunk granularity using a
   tick counter incremented on each mutable access, enabling `Changed<T>` queries to skip unmodified
   chunks.
   - **Rationale:** Skipping unchanged chunks enables reactive patterns and dramatically reduces
     work for systems like network delta compression and spatial index updates.
   - **Verification:** Unit test: mutate one entity's component in a chunk of 100. Verify the chunk
     is marked changed. Query `Changed<T>` and verify the chunk is included. On the next tick
     without mutations, verify `Changed<T>` returns no results.

## Resources and Singletons

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.23  | [F-1.1.23](../../features/core-runtime/entity-component-system.md) |
| R-1.1.24  | [F-1.1.24](../../features/core-runtime/entity-component-system.md) |
| R-1.1.22a | [F-1.1.22](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.23** — The engine **SHALL** support typed singleton resources stored in the ECS world,
   accessed via `Res<T>` (shared) and `ResMut<T>` (exclusive) system parameters, participating in
   the scheduler's dependency graph and change detection.
   - **Rationale:** Global state like `Time`, `PhysicsConfig`, and `InputState` needs a type-safe,
     scheduler-aware access mechanism.
   - **Verification:** Unit test: insert a resource, read it via `Res<T>` from one system and write
     it via `ResMut<T>` from another. Verify the scheduler orders them correctly. Verify
     `Changed<T>` detects resource mutations.
2. **R-1.1.24** — The engine **SHALL** ensure resources marked as non-send are only accessed from
   the game loop thread, and **SHALL** automatically pin systems that access non-send resources to
   the game loop thread.
   - **Rationale:** GPU device handles and windowing handles are not thread-safe and must never be
     moved to worker threads.
   - **Verification:** Unit test: register a non-send resource and a system that accesses it. Verify
     the system executes on the game loop thread. Attempt to access the resource from a worker
     thread and verify the scheduler prevents it (compile-time or runtime error).
3. **R-1.1.22a** — Per-chunk change tracking metadata **SHALL NOT** exceed 8 bytes per component
   type per chunk (one tick counter). False-positive change detection rate **SHALL** be bounded by
   chunk granularity — at most chunk_capacity entities are falsely reported as changed when only one
   entity in the chunk was mutated.
   - **Rationale:** Change detection metadata scales with archetype and chunk count; unbounded
     overhead reduces effective memory for entity data.
   - **Verification:** Unit test: measure change detection metadata size per chunk across 1,000
     archetypes. Verify it matches expected 8 bytes per component type per chunk. Verify
     false-positive bound by mutating one entity in a chunk and counting entities reported as
     changed.

## System Scheduling

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.25  | [F-1.1.25](../../features/core-runtime/entity-component-system.md) |
| R-1.1.26  | [F-1.1.26](../../features/core-runtime/entity-component-system.md) |
| R-1.1.27  | [F-1.1.27](../../features/core-runtime/entity-component-system.md) |
| R-1.1.28  | [F-1.1.28](../../features/core-runtime/entity-component-system.md) |
| R-1.1.29  | [F-1.1.29](../../features/core-runtime/entity-component-system.md) |
| R-1.1.25a | [F-1.1.25](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.25** — The engine **SHALL** automatically resolve system execution order from declared
   read/write access sets by building a DAG and producing a topological ordering, detecting cycles
   at schedule-build time and reporting them as errors.
   - **Rationale:** Manual ordering is error-prone and does not scale; automatic dependency
     resolution maximizes parallelism while guaranteeing correctness.
   - **Verification:** Unit test: register systems with known read/write dependencies, build the
     schedule, and verify the resulting order respects all dependencies. Register systems forming a
     cycle and verify a diagnostic error is reported at build time.
2. **R-1.1.26** — The engine **SHALL** organize systems into hierarchical groups with built-in
   phases (`PreUpdate`, `Update`, `PostUpdate`, `FixedUpdate`, `PreRender`, `Render`), support
   custom phases with `before`/`after` ordering, and disable all contained systems when a group is
   disabled.
   - **Rationale:** Phases provide a coarse execution structure, and group disable/enable controls
     subsystem activation (e.g., disabling physics during pause).
   - **Verification:** Unit test: register systems in `Update` and `FixedUpdate`, verify execution
     order. Disable a group and verify none of its systems execute. Insert a custom phase `between`
     two built-in phases and verify correct ordering.
3. **R-1.1.27** — The engine **SHALL** support system run criteria — predicates evaluated each frame
   that gate system execution — including fixed-timestep accumulators, state machine transitions,
   resource existence checks, and user-defined predicates, composable with AND logic.
   - **Rationale:** Run criteria prevent unnecessary system execution and enable clean phase
     separation without runtime branching inside system bodies.
   - **Verification:** Unit test: attach a run criterion that gates on a boolean resource. Toggle
     the resource and verify the system runs only when the criterion is met. Compose two criteria
     with AND and verify the system runs only when both are satisfied.
4. **R-1.1.28** — The engine **SHALL** detect pairs of systems that access the same components with
   conflicting access modes and have no explicit ordering constraint, reporting ambiguities as
   warnings at schedule-build time.
   - **Rationale:** Unordered parallel systems with conflicting access cause nondeterminism that is
     especially harmful for deterministic simulation.
   - **Verification:** Unit test: register two systems where one reads component A and the other
     writes component A with no ordering constraint. Build the schedule and verify a warning is
     emitted naming both systems and the conflicting component.
5. **R-1.1.29** — The engine **SHALL** support exclusive systems that acquire `&mut World` access,
   preventing all other systems from running concurrently, with the scheduler treating exclusive
   systems as full barriers in the execution graph.
   - **Rationale:** Operations like world serialization and scene loading require full world access
     that cannot be decomposed into component queries.
   - **Verification:** Unit test: register an exclusive system alongside parallel systems. Verify
     the exclusive system runs alone with no concurrent system execution. Measure that systems
     before and after the exclusive system still run in parallel with each other.
6. **R-1.1.25a** — Schedule construction (dependency resolution, topological sort, ambiguity
   detection) **SHALL** complete in under 50 milliseconds for up to 500 systems. The schedule
   **SHALL** be rebuilt only when systems are added, removed, or when run criteria change the active
   system set — not on every frame.
   - **Rationale:** Schedule build occurs at startup and on system set changes; excessive build time
     delays level loads and hot-reload cycles.
   - **Verification:** Benchmark: build a schedule with 500 systems and measure construction time.
     Verify it is under 50 ms. Verify the schedule is not rebuilt on frames where no system set
     changes occur.

## Observers

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.30  | [F-1.1.30](../../features/core-runtime/entity-component-system.md) |
| R-1.1.31  | [F-1.1.31](../../features/core-runtime/entity-component-system.md) |
| R-1.1.30a | [F-1.1.30](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.30** — The engine **SHALL** support observers that fire callbacks when built-in events
   (`OnAdd`, `OnRemove`, `OnSet`, `OnTableCreate`, `OnTableEmpty`) occur on entities matching a
   multi-term query, evaluated at sync points to allow safe structural changes.
   - **Rationale:** Deferred observer evaluation at sync points prevents iterator invalidation while
     enabling reactive patterns.
   - **Verification:** Unit test: register an observer for `OnAdd` of component A on entities that
     also have component B. Add component A to entities with and without B. Verify the observer
     fires only for matching entities at the next sync point.
2. **R-1.1.31** — The engine **SHALL** support user-defined event types emitted at specific
   entities, with observers firing for matching entities and event propagation along relationship
   edges (e.g., bubbling up parent chains).
   - **Rationale:** Gameplay events like `DamageEvent` or `PickupEvent` need entity-targeted
     delivery with hierarchy propagation for composite entities.
   - **Verification:** Unit test: define a `DamageEvent` type, emit it at a child entity, register
     observers on both child and parent. Verify the observer fires on the child first, then bubbles
     to the parent along the `ChildOf` relationship edge.
3. **R-1.1.30a** — The engine **SHALL** support at least 1,000 registered observers per world.
   Observer dispatch at sync points **SHALL** complete in O(e * m) where e is the number of pending
   events and m is the number of matching observers, not the total observer count. The system
   **SHALL** report a warning if observer dispatch exceeds 1 millisecond in a single sync point.
   - **Rationale:** Games with complex reactive logic may register hundreds of observers; dispatch
     must scale with event volume, not total observer count.
   - **Verification:** Register 1,000 observers matching different event types. Fire 100 events
     matching 10 observers each. Verify dispatch time scales with the 100 * 10 = 1,000 invocations,
     not with the total 1,000 observer registrations.

## Command Buffers

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.32  | [F-1.1.32](../../features/core-runtime/entity-component-system.md) |
| R-1.1.33  | [F-1.1.33](../../features/core-runtime/entity-component-system.md) |
| R-1.1.32a | [F-1.1.32](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.32** — The engine **SHALL** provide per-system command buffers that record entity spawn,
   despawn, and component add/remove operations, applying them in deterministic order at designated
   sync points.
   - **Rationale:** Deferred application eliminates borrow conflicts during parallel system
     execution and ensures reproducible simulation state.
   - **Verification:** Unit test: record spawn and component-add commands from two parallel systems.
     Flush at a sync point. Verify entities and components are applied in the same deterministic
     order across repeated runs with the same inputs.
2. **R-1.1.33** — The engine **SHALL** allow multiple worker threads to record commands into the
   same command buffer concurrently using a parallel writer, with each command carrying a sort key
   to ensure deterministic playback order regardless of recording order.
   - **Rationale:** Per-thread command buffers add merge complexity; a shared buffer with sort keys
     simplifies parallel system authoring.
   - **Verification:** Stress test: record 100,000 commands from 8 threads with random timing. Flush
     and verify playback order matches the sort-key order exactly. Run 100 iterations and verify
     identical results every time.
3. **R-1.1.32a** — Per-system command buffer memory **SHALL NOT** exceed 64 KiB per frame under
   typical usage. Command flush at sync points **SHALL** process at least 100,000 commands per
   millisecond. The engine **SHALL** report a diagnostic warning if any single command buffer
   exceeds 1 MiB, indicating potential unbounded command generation.
   - **Rationale:** Command buffers are allocated per system per frame; unbounded growth degrades
     frame arena utilization and flush times.
   - **Verification:** Benchmark: record 100,000 spawn commands and flush. Verify flush completes in
     under 1 ms. Verify per-system buffer size under typical workloads. Verify the warning triggers
     when a buffer exceeds 1 MiB.

## Worlds

| ID        | Derived From                                                       |
|-----------|--------------------------------------------------------------------|
| R-1.1.34  | [F-1.1.34](../../features/core-runtime/entity-component-system.md) |
| R-1.1.35  | [F-1.1.35](../../features/core-runtime/entity-component-system.md) |
| R-1.1.35a | [F-1.1.35](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.34** — The engine **SHALL** support multiple independent ECS worlds within a single
   process, each owning its own archetype storage, entity allocator, and resource map, tagged with
   flags (Game, Editor, Server, Shadow) that control system instantiation.
   - **Rationale:** Multiple worlds enable rollback, streaming staging, and instanced zone isolation
     without cross-contamination.
   - **Verification:** Integration test: create two worlds with different flags, register systems
     gated per flag. Verify each world runs only its flagged systems. Verify entities in one world
     are invisible to queries in the other.
2. **R-1.1.35** — The engine **SHALL** support transferring entities with all their components from
   one world to another, remapping entity IDs during migration to avoid collisions.
   - **Rationale:** Zone transitions in open worlds require moving entities between server worlds
     while preserving full component state.
   - **Verification:** Integration test: create an entity with 5 components and relationships in
     world A. Migrate it to world B. Verify all components and relationship data are intact in world
     B. Verify the original entity no longer exists in world A. Verify no entity ID collisions in
     world B.
3. **R-1.1.35a** — Entity migration between worlds **SHALL** complete in under 10 microseconds per
   entity (including all components and relationship remapping). The engine **SHALL** return a
   diagnostic error if migration fails due to type incompatibility (target world missing a required
   component type registration) rather than silently dropping data.
   - **Rationale:** Zone transitions must be fast enough to avoid frame-time spikes; data loss
     during migration is unacceptable in a persistent world.
   - **Verification:** Benchmark: migrate 10,000 entities with 5 components each and verify
     per-entity cost. Attempt migration of an entity with an unregistered component type and verify
     a diagnostic error is returned with the missing type name.

## Prefabs and Prototypes

| ID       | Derived From                                                       |
|----------|--------------------------------------------------------------------|
| R-1.1.36 | [F-1.1.36](../../features/core-runtime/entity-component-system.md) |
| R-1.1.37 | [F-1.1.37](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.36** — The engine **SHALL** support prefab entities that serve as templates via an `IsA`
   relationship, where instances inherit shared components from the prefab and automatically
   override (copy-on-write) inherited components when an instance writes to them.
   - **Rationale:** Prefab inheritance avoids duplicating shared data across thousands of instances
     while enabling per-instance customization.
   - **Verification:** Unit test: create a prefab with 3 components, instantiate 100 instances via
     `IsA`. Verify instances share prefab component data. Modify one component on a single instance
     and verify only that instance has an overridden copy while the other 99 still share the
     prefab's value.
2. **R-1.1.37** — The engine **SHALL** instantiate prefab child hierarchies as complete subtrees,
   support nested prefab composition where inner prefab changes propagate to all outer instances,
   and provide slot references for named access to instantiated children.
   - **Rationale:** Complex game objects (vehicles, characters with equipment) are composed of
     nested prefab hierarchies that must stay synchronized with their templates.
   - **Verification:** Integration test: create a nested prefab (outer prefab containing an inner
     prefab child). Instantiate 10 outer instances. Modify a component on the inner prefab. Verify
     all 10 outer instances reflect the inner prefab change. Verify slot references resolve to the
     correct child entities.

## State Machines

| ID       | Derived From                                                       |
|----------|--------------------------------------------------------------------|
| R-1.1.38 | [F-1.1.38](../../features/core-runtime/entity-component-system.md) |

1. **R-1.1.38** — The engine **SHALL** support typed state components with `OnEnter`, `OnExit`, and
   `OnTransition` observer hooks, where state transitions replace one state component with another,
   and systems can declare run criteria gated on the active state.
   - **Rationale:** Game state management (menu, loading, gameplay, paused) must integrate with the
     ECS scheduling system to activate/deactivate systems based on state.
   - **Verification:** Unit test: define states `Menu`, `Playing`, `Paused`. Transition from `Menu`
     to `Playing` and verify `OnExit(Menu)` and `OnEnter(Playing)` observers fire. Verify a system
     with run criterion `in_state(Playing)` runs during `Playing` and does not run during `Menu`.
