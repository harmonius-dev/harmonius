# Entity Component System User Stories

## Storage

## US-1.1.1 Store Components in Cache-Friendly Archetype Tables

**As an** engine developer (P-26), **I want** components stored in contiguous archetype tables with
structure-of-arrays layout and configurable chunk sizes (8-64 KiB), **so that** iteration over large
entity populations is cache-friendly and SIMD-optimized for maximum throughput.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Contiguous SoA layout per component type per archetype | F-1.1.1 | R-1.1.1 |
| Cache-line-aligned chunks (16 KiB default desktop, 8 KiB mobile) | F-1.1.1 | R-1.1.1 |
| Iteration throughput scales with entity count | F-1.1.1 | R-1.1.1 |

## US-1.1.2 Verify Archetype Storage Cache Performance

**As an** engine tester (P-27), **I want** to benchmark archetype iteration throughput at various
entity counts and chunk sizes to verify cache-friendly access patterns, **so that** I can confirm
the SoA layout delivers expected speedup over AoS alternatives.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Iteration benchmark at 100K, 500K, 1M entities | F-1.1.1 | R-1.1.1 |
| Cache miss rate measured and within target | F-1.1.1 | R-1.1.1 |
| SIMD utilization verified for numeric component types | F-1.1.1 | R-1.1.1 |

## US-1.1.3 Mark Rarely-Queried Components as Sparse

**As a** game developer (P-15), **I want** to mark components as sparse so they bypass archetype
tables and are stored in per-type sparse sets, **so that** adding or removing debug markers and
temporary status effects does not trigger expensive archetype migrations.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| `#[sparse]` attribute opts component into sparse storage | F-1.1.2 | R-1.1.2 |
| Sparse component changes do not change archetype | F-1.1.2 | R-1.1.2 |
| Sparse components keyed by entity ID | F-1.1.2 | R-1.1.2 |

## US-1.1.4 Understand Which Components Are Sparse in the Visual Editor

**As a** designer (P-5), **I want** the visual editor to indicate which components are sparse versus
dense stored, **so that** I understand why some components cause migrations and others do not when
editing entity templates.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Editor displays storage mode (dense/sparse) per component | F-1.1.2 | R-1.1.2 |
| Visual indication when a component change causes migration | F-1.1.2 | R-1.1.2 |

## US-1.1.5 Resolve Archetype Transitions in O(1) via Cached Edges

**As an** engine developer (P-26), **I want** a directed archetype graph with cached edge lookups so
structural changes resolve target archetypes in O(1), **so that** bulk spawning and despawning
thousands of entities per frame remains fast regardless of archetype count.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Directed graph of archetypes with edge caching | F-1.1.3 | R-1.1.3 |
| O(1) archetype resolution for add/remove operations | F-1.1.3 | R-1.1.3 |
| Edges encode invariants (adding A always brings B) | F-1.1.3 | R-1.1.3 |

## US-1.1.6 Stress-Test Archetype Migration Under Bulk Spawn/Despawn

**As an** engine tester (P-27), **I want** to stress-test archetype graph traversal under rapid bulk
spawn and despawn of thousands of entities with varying component sets, **so that** I can verify
O(1) edge lookup holds and no linear scans occur.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 10K entities spawned/despawned per frame without degradation | F-1.1.3 | R-1.1.3 |
| Edge cache hit rate > 99% after warmup | F-1.1.3 | R-1.1.3 |
| No linear archetype scan during structural changes | F-1.1.3 | R-1.1.3 |

## Components

## US-1.1.7 Register Components at Compile Time and Runtime

**As a** game developer (P-15), **I want** to register components both at compile time via derive
macros and at runtime for hot-reloaded gameplay logic, **so that** I get zero-cost access for known
types while supporting dynamic scripting.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Compile-time registration via derive macro | F-1.1.4 | R-1.1.4 |
| Runtime registration for dynamic component types | F-1.1.4 | R-1.1.4 |
| Each component records size, alignment, drop, storage mode | F-1.1.4 | R-1.1.4 |

## US-1.1.8 Use Zero-Size Tag Components for Entity Markers

**As a** game developer (P-15), **I want** zero-sized tag components that alter archetype identity
and enable query filtering without per-entity memory cost, **so that** I can mark entities as
`Player`, `Enemy`, or `Static` efficiently across millions of entities.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Tag components occupy zero bytes in storage | F-1.1.5 | R-1.1.5 |
| Tags alter archetype identity for query filtering | F-1.1.5 | R-1.1.5 |
| No column stored for tag components | F-1.1.5 | R-1.1.5 |

## US-1.1.9 Share Component Values Across Entities in a Chunk

**As a** game developer (P-15), **I want** shared components stored once per chunk rather than per
entity, **so that** material IDs, LOD groups, and render layer assignments do not waste memory when
thousands of entities share the same value.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Shared component stored once per chunk | F-1.1.6 | R-1.1.6 |
| Changing shared value is a structural change (moves entity) | F-1.1.6 | R-1.1.6 |
| Zero per-entity memory cost for shared components | F-1.1.6 | R-1.1.6 |

## US-1.1.10 Store Variable-Length Arrays Per Entity

**As a** game developer (P-15), **I want** variable-length arrays associated with entities via
`DynamicBuffer<T>` with inline small buffers and heap spill, **so that** I can store child lists,
inventory slots, and waypoint sequences per entity without fixed-size limits.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Small buffers inline in archetype chunk | F-1.1.7 | R-1.1.7 |
| Large buffers spill to heap | F-1.1.7 | R-1.1.7 |
| Typed `DynamicBuffer<T>` handle for access | F-1.1.7 | R-1.1.7 |
| Platform-appropriate inline thresholds | F-1.1.7 | R-1.1.7 |

## US-1.1.11 Toggle Components Without Structural Changes

**As a** game developer (P-15), **I want** to toggle enableable components on and off per entity
without structural changes, **so that** I can enable or disable behaviors like AI perception or
physics sleeping cheaply from parallel worker threads.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| `#[enableable]` attribute marks togglable components | F-1.1.8 | R-1.1.8 |
| Toggle safe from parallel worker threads | F-1.1.8 | R-1.1.8 |
| Disabled components excluded from queries by default | F-1.1.8 | R-1.1.8 |
| `WithDisabled<T>` and `WithPresent<T>` override filtering | F-1.1.8 | R-1.1.8 |

## US-1.1.12 Verify Enableable Component Thread Safety

**As an** engine tester (P-27), **I want** to stress-test concurrent toggling of enableable
components from multiple worker threads, **so that** I can verify no data races or incorrect query
results occur under parallel access.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Concurrent toggles from 8+ threads produce correct results | F-1.1.8 | R-1.1.8 |
| No data races detected by sanitizers | F-1.1.8 | R-1.1.8 |
| Query results consistent with toggle state | F-1.1.8 | R-1.1.8 |

## US-1.1.13 Keep Auxiliary Structures in Sync via Component Hooks

**As an** engine developer (P-26), **I want** per-type lifecycle callbacks (on_add, on_remove,
on_set) that fire synchronously at the point of change, **so that** auxiliary data structures like
the spatial index stay in sync with component changes.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Hooks fire synchronously on add, remove, and set | F-1.1.9 | R-1.1.9 |
| Hook receives entity ID and component reference | F-1.1.9 | R-1.1.9 |
| Spatial index updated via Transform on_add hook | F-1.1.9 | R-1.1.9 |

## US-1.1.14 Insert Related Components Atomically as Bundles

**As a** game developer (P-15), **I want** to insert groups of related components atomically as
named bundles with auto-added required companions, **so that** I never create incomplete entities
missing essential components like `CollisionLayers` when adding `Collider`.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Bundle inserts all components atomically | F-1.1.10 | R-1.1.10 |
| Required companions auto-added if not present | F-1.1.10 | R-1.1.10 |
| Bundles flattened at compile time, zero runtime overhead | F-1.1.10 | R-1.1.10 |

## Entities

## US-1.1.15 Detect Stale Entity References via Generational Indices

**As an** engine developer (P-26), **I want** entities identified by generational indices (32-bit
index + 32-bit generation) that detect stale references in O(1), **so that** recycled entity slots
cannot be mistakenly accessed through old handles after despawn.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| O(1) allocation and deallocation via free list | F-1.1.11 | R-1.1.11 |
| Generation counter incremented on deallocation | F-1.1.11 | R-1.1.11 |
| Stale handle detected without indirection | F-1.1.11 | R-1.1.11 |

## US-1.1.16 Verify Generational Index Prevents Use-After-Free

**As an** engine tester (P-27), **I want** to verify that accessing a despawned entity via a stale
generational handle fails safely rather than returning incorrect data, **so that** use-after-free
bugs are impossible in the entity system.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Stale handle access returns error, not stale data | F-1.1.11 | R-1.1.11 |
| Recycled slot with incremented generation rejects old handle | F-1.1.11 | R-1.1.11 |
| Fuzz test with random spawn/despawn/access patterns passes | F-1.1.11 | R-1.1.11 |

## US-1.1.17 Defer Entity Destruction for Resource Cleanup

**As an** engine developer (P-26), **I want** cleanup components that persist after entity
destruction so dedicated systems can tear down GPU buffers and network registrations, **so that**
external resources are cleaned up gracefully rather than leaked.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| `#[cleanup]` components persist after destroy | F-1.1.12 | R-1.1.12 |
| Non-cleanup components removed on destroy | F-1.1.12 | R-1.1.12 |
| Cleanup system detects lingering entities | F-1.1.12 | R-1.1.12 |
| Final removal of cleanup components completes destruction | F-1.1.12 | R-1.1.12 |

## US-1.1.18 Reference Entities by Name and Hierarchical Path

**As a** game developer (P-15), **I want** to assign human-readable names to entities and look them
up by hierarchical path (e.g., `World/Zone3/NPC_Guard_17`), **so that** I can reference entities in
visual scripts and debug the entity tree without memorizing raw IDs.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Names assigned as sparse components | F-1.1.13 | R-1.1.13 |
| Path lookup in O(log n) using parent-child hierarchy | F-1.1.13 | R-1.1.13 |
| Names usable in visual editor entity tree | F-1.1.13 | R-1.1.13 |

## US-1.1.19 Browse Named Entities in the Visual Editor

**As a** designer (P-5), **I want** to browse and search entities by name in the visual editor's
entity tree, **so that** I can find and select specific entities without scrolling through raw
numeric IDs.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Entity tree shows names and hierarchical paths | F-1.1.13 | R-1.1.13 |
| Search by name or path substring | F-1.1.13 | R-1.1.13 |
| Double-click navigates to entity in viewport | F-1.1.13 | R-1.1.13 |

## Relationships and Hierarchies

## US-1.1.20 Model Relationships as Entity Pairs With Wildcard Queries

**As a** game developer (P-15), **I want** relationships encoded as (Relationship, Target) pairs
with wildcard queries, **so that** I can model complex data like "all entities that like any food"
or "anything targeting this NPC" in a single query.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Relationships packed into 64-bit component IDs | F-1.1.14 | R-1.1.14 |
| Wildcard queries on relationship or target | F-1.1.14 | R-1.1.14 |
| Relationships are the foundation for hierarchies | F-1.1.14 | R-1.1.14 |

## US-1.1.21 Enforce Domain Invariants With Relationship Properties

**As a** game developer (P-15), **I want** relationships annotated with properties like Exclusive,
Symmetric, Transitive, Acyclic, and cascading delete policies, **so that** domain invariants are
enforced automatically without hand-written validation logic.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Exclusive: only one target per relationship | F-1.1.15 | R-1.1.15 |
| Symmetric: auto-add reverse relationship | F-1.1.15 | R-1.1.15 |
| OnDelete/OnDeleteTarget cascading policies | F-1.1.15 | R-1.1.15 |
| Acyclic constraint prevents cycles | F-1.1.15 | R-1.1.15 |

## US-1.1.22 Use Built-In Parent-Child Hierarchy With Cascading Delete

**As a** game developer (P-15), **I want** a built-in `ChildOf` relationship with automatic
cascading deletion and traversal support, **so that** deleting a parent entity automatically cleans
up all descendants and I can walk the hierarchy in either direction.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| `ChildOf` with Acyclic, Traversable, OnDeleteTarget(Delete) | F-1.1.16 | R-1.1.16 |
| Up (parent chain) and cascade (top-down) traversal | F-1.1.16 | R-1.1.16 |
| Parent deletion cascades to all descendants | F-1.1.16 | R-1.1.16 |

## US-1.1.23 Verify Cascading Delete Cleans Up Deep Hierarchies

**As an** engine tester (P-27), **I want** to verify that deleting a parent entity at the root of a
deep hierarchy cascades correctly through all descendants, **so that** no orphaned entities remain
after hierarchical deletion.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Delete root of 100-deep hierarchy: all descendants removed | F-1.1.16 | R-1.1.16 |
| No orphaned entities after cascading delete | F-1.1.16 | R-1.1.16 |
| Cleanup components on descendants processed correctly | F-1.1.16, F-1.1.12 | R-1.1.16, R-1.1.12 |

## Queries

## US-1.1.24 Compose Queries With Cached Archetype Filters

**As a** game developer (P-15), **I want** to compose queries with `With`, `Without`, `Option`,
`Changed`, and `Added` filters that cache after first evaluation, **so that** I can precisely select
entities without repeated archetype matching overhead.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All filter types composable in a single query | F-1.1.17 | R-1.1.17 |
| Cached queries skip archetype matching on repeat | F-1.1.17 | R-1.1.17 |
| Incremental update when new archetypes created | F-1.1.17 | R-1.1.17 |

## US-1.1.25 Sort and Group Query Results for Batching

**As a** game developer (P-15), **I want** to sort query results by component value and group by
relationship target, **so that** I can batch draw calls by material ID and partition entities
spatially without external sorting.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Stable sort by component value with custom comparator | F-1.1.18 | R-1.1.18 |
| Group by relationship target | F-1.1.18 | R-1.1.18 |
| Sort cached between frames, re-sorted on change detection | F-1.1.18 | R-1.1.18 |

## US-1.1.26 Match Graph Patterns Across Relationships

**As a** game developer (P-15), **I want** query variables enabling graph pattern matching across
entity relationships, **so that** I can find all children of boss entities or all entities connected
by specific relationship chains in a single query pass.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Variables ($parent, $target) in query terms | F-1.1.19 | R-1.1.19 |
| Joins across relationship edges | F-1.1.19 | R-1.1.19 |
| Pattern matching without nested loops | F-1.1.19 | R-1.1.19 |

## US-1.1.27 Iterate Queries in Parallel Across Worker Threads

**As an** engine developer (P-26), **I want** query results automatically partitioned across worker
threads with borrow safety guarantees, **so that** systems processing large entity populations scale
linearly with core count.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Archetype-level or chunk-level partitioning | F-1.1.20 | R-1.1.20 |
| No conflicting mutable/immutable borrows across threads | F-1.1.20 | R-1.1.20 |
| Linear scaling with core count for large populations | F-1.1.20 | R-1.1.20 |
| Mobile: 2-4 workers, desktop: physical cores - 1 | F-1.1.20 | R-1.1.20 |

## US-1.1.28 Benchmark Parallel Iteration Scaling Across Core Counts

**As an** engine tester (P-27), **I want** to benchmark parallel query iteration at various entity
counts and core counts, **so that** I can verify linear scaling and identify any contention or false
sharing bottlenecks.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Linear speedup measured from 1 to N workers | F-1.1.20 | R-1.1.20 |
| No false sharing detected via perf counters | F-1.1.20 | R-1.1.20 |
| Borrow safety verified under sanitizers | F-1.1.20 | R-1.1.20 |

## Aspects

## US-1.1.29 Group Components Into Named Aspect Structs

**As a** game developer (P-15), **I want** to group related components into named aspect structs
like `PhysicsAspect`, **so that** I reduce boilerplate when multiple systems need the same component
grouping.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Aspect struct groups subset of entity components | F-1.1.21 | R-1.1.21 |
| Aspects usable as single type parameter in queries | F-1.1.21 | R-1.1.21 |
| Nested aspects supported | F-1.1.21 | R-1.1.21 |
| Per-field access modes (&T vs &mut T) | F-1.1.21 | R-1.1.21 |

## Change Detection

## US-1.1.30 Skip Unmodified Chunks With Change Detection

**As a** game developer (P-15), **I want** to query only components modified since the last tick
using `Changed<T>`, **so that** network delta compression, spatial index updates, and dirty-flag
propagation skip unmodified chunks.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Per-component tick counter bumped on mutable access | F-1.1.22 | R-1.1.22 |
| Chunk-granularity change tracking | F-1.1.22 | R-1.1.22 |
| `Changed<T>` filter skips unmodified chunks | F-1.1.22 | R-1.1.22 |

## US-1.1.31 Verify Change Detection Chunk Granularity

**As an** engine tester (P-27), **I want** to verify that change detection correctly marks only
chunks containing modified entities and does not produce false positives or negatives, **so that**
reactive systems process exactly the correct set of changes.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Modified entity's chunk marked changed | F-1.1.22 | R-1.1.22 |
| Unmodified chunks in same archetype not marked | F-1.1.22 | R-1.1.22 |
| No false positives from read-only access | F-1.1.22 | R-1.1.22 |

## Resources and Singletons

## US-1.1.32 Access Global State Through Typed World Resources

**As a** game developer (P-15), **I want** typed singleton resources accessible through
scheduler-aware `Res<T>` and `ResMut<T>` parameters with change detection, **so that** global state
like `Time` and `InputState` is type-safe and participates in dependency analysis.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| One instance per type per world | F-1.1.23 | R-1.1.23 |
| Res<T> (shared) and ResMut<T> (exclusive) access | F-1.1.23 | R-1.1.23 |
| Resources participate in change detection | F-1.1.23 | R-1.1.23 |

## US-1.1.33 Pin Non-Send Resources to the Main Thread

**As an** engine developer (P-26), **I want** resources marked non-send to be automatically pinned
to the main thread by the scheduler, **so that** GPU device handles and window handles are never
accessed from worker threads.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Non-send resources pinned to main thread | F-1.1.24 | R-1.1.24 |
| Systems accessing non-send resources run on main thread | F-1.1.24 | R-1.1.24 |
| Scheduler never moves non-send access to workers | F-1.1.24 | R-1.1.24 |

## System Scheduling

## US-1.1.34 Resolve System Execution Order Automatically

**As an** engine developer (P-26), **I want** system execution order resolved automatically from
declared read/write access sets with DAG construction and cycle detection, **so that** I do not
manually order hundreds of systems while guaranteeing correctness.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| DAG built from declared access sets | F-1.1.25 | R-1.1.25 |
| Topological ordering maximizes parallelism | F-1.1.25 | R-1.1.25 |
| Cycles detected and reported at schedule-build time | F-1.1.25 | R-1.1.25 |

## US-1.1.35 Organize Systems Into Groups and Phases

**As a** game developer (P-15), **I want** systems organized into hierarchical groups and phases
(PreUpdate, Update, PostUpdate, FixedUpdate, PreRender, Render) with before/after ordering,
**so that** I can cleanly separate gameplay, physics, and rendering execution.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Built-in phases with defined execution order | F-1.1.26 | R-1.1.26 |
| Custom phases insertable with before/after ordering | F-1.1.26 | R-1.1.26 |
| Nested groups (PhysicsGroup inside FixedUpdate) | F-1.1.26 | R-1.1.26 |
| Disabling a group disables all contained systems | F-1.1.26 | R-1.1.26 |

## US-1.1.36 Understand How Systems Map to Visual Editor Phases

**As a** designer (P-5), **I want** to see which execution phase each system belongs to in the
visual editor, **so that** I understand when gameplay logic runs relative to physics and rendering
when debugging timing issues.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Visual editor shows system-to-phase mapping | F-1.1.26 | R-1.1.26 |
| Phase execution order visible in editor | F-1.1.26 | R-1.1.26 |
| System enable/disable toggleable per group | F-1.1.26 | R-1.1.26 |

## US-1.1.37 Gate System Execution on Run Criteria

**As a** game developer (P-15), **I want** systems gated by predicates like fixed-timestep
accumulators, state machine transitions, and resource existence checks, **so that** systems only run
when their conditions are met without branching inside system bodies.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Fixed-timestep accumulator criteria | F-1.1.27 | R-1.1.27 |
| State machine transition criteria | F-1.1.27 | R-1.1.27 |
| Multiple criteria compose with AND logic | F-1.1.27 | R-1.1.27 |

## US-1.1.38 Detect Scheduling Ambiguities at Build Time

**As a** QA engineer (P-19), **I want** the scheduler to detect pairs of systems with conflicting
access and no ordering constraint, **so that** I identify potential nondeterminism before it causes
hard-to-reproduce bugs in deterministic simulation.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Conflicting access pairs detected | F-1.1.28 | R-1.1.28 |
| Ambiguities reported as warnings at schedule-build time | F-1.1.28 | R-1.1.28 |
| Warning includes system names and conflicting components | F-1.1.28 | R-1.1.28 |

## US-1.1.39 Debug Scheduling Ambiguities in Development

**As an** engine developer (P-26), **I want** ambiguity detection reports to include system names,
conflicting component types, and suggested resolution, **so that** I can quickly fix ordering issues
without manually tracing the dependency graph.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Report includes both system names | F-1.1.28 | R-1.1.28 |
| Report lists conflicting component types | F-1.1.28 | R-1.1.28 |
| Suggested resolution (add ordering or access change) | F-1.1.28 | R-1.1.28 |

## US-1.1.40 Use Exclusive Systems as Full Barriers

**As an** engine developer (P-26), **I want** systems that acquire exclusive `&mut World` access and
act as full barriers, **so that** operations like world serialization and scene loading that cannot
be decomposed into queries still integrate with the scheduler.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Exclusive system prevents all concurrent execution | F-1.1.29 | R-1.1.29 |
| Scheduler treats exclusive systems as barriers | F-1.1.29 | R-1.1.29 |
| World serialization and scene loading use exclusive access | F-1.1.29 | R-1.1.29 |

## Observers

## US-1.1.41 React to Entity Events at Sync Points

**As a** game developer (P-15), **I want** to register callbacks that fire when specific events
(OnAdd, OnRemove, OnSet) occur on entities matching a query at sync points, **so that** I can
implement reactive patterns like spatial index updates without polling.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Observers match multi-term queries | F-1.1.30 | R-1.1.30 |
| Evaluated at sync points (deferred), safe for structural changes | F-1.1.30 | R-1.1.30 |
| Built-in events: OnAdd, OnRemove, OnSet, OnTableCreate, OnTableEmpty | F-1.1.30 | R-1.1.30 |

## US-1.1.42 Emit Custom Events That Propagate Along Relationships

**As a** game developer (P-15), **I want** to define custom event types and emit them at specific
entities with propagation along relationship edges, **so that** gameplay events like DamageEvent and
PickupEvent bubble up parent chains of composite entities.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Custom event types with data payloads | F-1.1.31 | R-1.1.31 |
| Events emitted at specific entities | F-1.1.31 | R-1.1.31 |
| Propagation along relationship edges (parent chain) | F-1.1.31 | R-1.1.31 |

## Command Buffers

## US-1.1.43 Record Structural Changes in Deferred Command Buffers

**As an** engine developer (P-26), **I want** per-system command buffers that record spawn, despawn,
and component operations applied in deterministic order at sync points, **so that** parallel systems
can record mutations without borrow conflicts.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Per-system command buffer records all structural operations | F-1.1.32 | R-1.1.32 |
| Commands applied in deterministic order at sync points | F-1.1.32 | R-1.1.32 |
| No runtime borrow conflicts during parallel execution | F-1.1.32 | R-1.1.32 |

## US-1.1.44 Record Commands in Parallel From Multiple Workers

**As an** engine developer (P-26), **I want** multiple worker threads to record commands into the
same buffer with sort keys for deterministic playback, **so that** parallel systems share a single
command buffer without per-thread merge overhead.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Concurrent recording from multiple workers | F-1.1.33 | R-1.1.33 |
| Sort keys ensure deterministic playback order | F-1.1.33 | R-1.1.33 |
| No per-thread command buffer merge needed | F-1.1.33 | R-1.1.33 |

## US-1.1.45 Verify Deterministic Command Buffer Playback

**As an** engine tester (P-27), **I want** to verify that parallel command recording produces
identical playback order regardless of thread scheduling, **so that** deterministic simulation is
maintained even under varying thread timing.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Same commands recorded in different thread orders produce same result | F-1.1.33 | R-1.1.33 |
| Sort key ordering verified across 100+ runs | F-1.1.33 | R-1.1.33 |
| World state identical after playback regardless of recording order | F-1.1.33 | R-1.1.33 |

## Worlds

## US-1.1.46 Run Multiple Independent ECS Worlds

**As a** game developer (P-15), **I want** multiple independent ECS worlds in a single process with
per-world system instantiation controlled by flags (Game, Editor, Server, Shadow), **so that** I can
isolate rollback worlds, streaming staging, and instanced zones.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Each world owns archetype storage, allocator, resource map | F-1.1.34 | R-1.1.34 |
| World flags control which systems instantiate | F-1.1.34 | R-1.1.34 |
| Mobile: max 2 concurrent worlds, desktop: configurable | F-1.1.34 | R-1.1.34 |

## US-1.1.47 Migrate Entities Between Worlds for Zone Transitions

**As a** game developer (P-15), **I want** to transfer entities with all components between worlds
with automatic ID remapping, **so that** zone transitions in open worlds preserve full entity state
without ID collisions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All components transferred during migration | F-1.1.35 | R-1.1.35 |
| Entity IDs remapped to avoid collisions | F-1.1.35 | R-1.1.35 |
| Full component state preserved across migration | F-1.1.35 | R-1.1.35 |

## US-1.1.48 Stress-Test Entity Migration Under Zone Transitions

**As an** engine tester (P-27), **I want** to stress-test entity migration with hundreds of entities
crossing zone boundaries simultaneously, **so that** I can verify no component data is lost or
corrupted during bulk world-to-world transfer.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 500 entities migrated simultaneously without data loss | F-1.1.35 | R-1.1.35 |
| ID remapping produces no collisions | F-1.1.35 | R-1.1.35 |
| All component types preserved including relationships | F-1.1.35 | R-1.1.35 |

## Prefabs and Prototypes

## US-1.1.49 Create Entity Templates With Prefab Inheritance

**As a** designer (P-5), **I want** prefab templates where instances inherit shared components and
automatically override on write, **so that** I can create thousands of similar entities from a
single template while customizing individual instances in the editor.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Prefab tag marks template entities | F-1.1.36 | R-1.1.36 |
| IsA relationship inherits components from prefab | F-1.1.36 | R-1.1.36 |
| Write to inherited component auto-overrides (copy-on-write) | F-1.1.36 | R-1.1.36 |
| Prefab variants (prefabs inheriting from prefabs) | F-1.1.36 | R-1.1.36 |

## US-1.1.50 Compose Nested Prefab Hierarchies

**As a** designer (P-5), **I want** nested prefab hierarchies where inner prefab changes propagate
to all outer instances with slot references, **so that** I can compose complex game objects from
reusable parts and update them centrally.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Prefab children instantiate entire subtree | F-1.1.37 | R-1.1.37 |
| Inner prefab changes propagate to outer instances | F-1.1.37 | R-1.1.37 |
| Slot references for named access to instantiated children | F-1.1.37 | R-1.1.37 |

## US-1.1.51 Implement Prefab Instantiation With Component Sharing

**As an** engine developer (P-26), **I want** to implement prefab instantiation that shares
component storage between prefab and instances until override, **so that** memory usage is
proportional to overrides rather than total instance count.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Shared components stored once in prefab | F-1.1.36 | R-1.1.36 |
| Memory proportional to overrides, not instances | F-1.1.36 | R-1.1.36 |
| Override detection on mutable access | F-1.1.36 | R-1.1.36 |

## State Machines

## US-1.1.52 Manage Game State With ECS-Integrated State Machine

**As a** game developer (P-15), **I want** typed state components with OnEnter, OnExit, and
OnTransition observer hooks that gate system execution, **so that** game state management (menu,
loading, gameplay, paused) integrates naturally with the ECS scheduler.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| States are components; transitions replace one with another | F-1.1.38 | R-1.1.38 |
| OnEnter, OnExit, OnTransition observers fire | F-1.1.38 | R-1.1.38 |
| Sub-states and computed states supported | F-1.1.38 | R-1.1.38 |
| Run criteria gated on active state | F-1.1.38 | R-1.1.38 |

## US-1.1.53 Understand State Transitions in the Visual Editor

**As a** designer (P-5), **I want** to see the current game state and available transitions in the
visual editor, **so that** I understand which systems are active in each state and can debug state
transition issues visually.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Current state visible in editor | F-1.1.38 | R-1.1.38 |
| Available transitions shown per state | F-1.1.38 | R-1.1.38 |
| Systems gated by each state identifiable | F-1.1.38 | R-1.1.38 |
