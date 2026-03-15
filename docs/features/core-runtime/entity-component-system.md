# 1.1 — Entity Component System

## Storage

### F-1.1.1 Archetype-Based Dense Storage

Store components in contiguous, cache-friendly archetype tables where each unique combination of
component types defines an archetype. Components within an archetype use structure-of-arrays layout
— one contiguous array per component type per table — maximizing iteration throughput and SIMD
potential. Fixed-size chunks (configurable, default 16 KiB) subdivide tables for cache-line-aligned
access.

- **Requirements:** R-1.1.1
- **Dependencies:** F-1.7.1 (Arena Allocators), F-1.3.1 (Type Registry)
- **Platform notes:** Mobile: 8 KiB default chunk size to fit L1 cache. Switch: 8 KiB handheld, 16
  KiB docked. Desktop: 16 KiB default, configurable up to 64 KiB.

### F-1.1.2 Sparse Component Storage

Allow components to opt into sparse-set storage via a `#[sparse]` attribute, bypassing archetype
tables entirely. Sparse components are stored in a separate per-type sparse set keyed by entity ID.
When an entity gains or loses a sparse component, it does not change archetype — eliminating table
fragmentation for rarely-queried or high-churn components (e.g., debug markers, temporary status
effects).

- **Requirements:** R-1.1.2
- **Dependencies:** F-1.1.1, F-1.1.4
- **Platform notes:** None

### F-1.1.3 Archetype Graph and Edge Caching

Maintain a directed graph of archetypes where edges represent single-component additions or
removals. Cache edge lookups so that structural changes resolve the target archetype in O(1). Edges
can encode invariants (e.g., adding component A always brings component B). This avoids linear scans
during entity migration, keeping bulk spawn and despawn operations fast at scale.

- **Requirements:** R-1.1.3
- **Dependencies:** F-1.1.1
- **Platform notes:** None

## Components

### F-1.1.4 Static and Dynamic Component Registration

Support both compile-time (derive macro) and runtime component registration. Static registration
enables zero-cost access patterns and monomorphized queries. Dynamic registration allows server-side
scripting and hot-reloaded gameplay logic to introduce new component types without recompilation.
Each component records size, alignment, drop function, and storage mode.

- **Requirements:** R-1.1.4
- **Dependencies:** F-1.3.1 (Type Registry)
- **Platform notes:** None

### F-1.1.5 Tag Components (Zero-Size)

Components with no data fields occupy zero bytes in storage. Tags alter archetype identity (enabling
query filtering) without consuming memory per entity. Used for markers like `Player`, `Enemy`,
`Static`, `CcdEnabled`, `Sleeping`. The archetype table stores no column for tag components.

- **Requirements:** R-1.1.5
- **Dependencies:** F-1.1.1, F-1.1.4
- **Platform notes:** None

### F-1.1.6 Shared Components

Components whose value is shared by all entities in a chunk. Stored once per chunk rather than per
entity — zero per-entity memory cost. Changing a shared component value is a structural change that
moves the entity to a different chunk. Ideal for material IDs, LOD groups, and render layer
assignments where many entities share the same value.

- **Requirements:** R-1.1.6
- **Dependencies:** F-1.1.1
- **Platform notes:** None

### F-1.1.7 Buffer Components (Dynamic Arrays)

Variable-length, resizable arrays associated with an entity. Small buffers stored inline in the
archetype chunk; large buffers spill to a heap allocation. Used for child entity lists, inventory
slots, waypoint sequences, and any per-entity collection. Accessed through a typed
`DynamicBuffer<T>` handle.

- **Requirements:** R-1.1.7
- **Dependencies:** F-1.1.1, F-1.7.2 (Pool Allocators)
- **Platform notes:** Mobile: inline threshold 128 bytes, max buffer 4 KiB before spill. Switch:
  inline threshold 256 bytes. Desktop: inline threshold 512 bytes, no cap.

### F-1.1.8 Enableable Components

Components marked `#[enableable]` can be toggled on/off per entity without structural changes.
Disabled components remain in the archetype table but are excluded from queries by default. Toggling
is safe from parallel worker threads without command buffers. Queries can explicitly request
`WithDisabled<T>` or `WithPresent<T>` to override default filtering.

- **Requirements:** R-1.1.8
- **Dependencies:** F-1.1.1, F-1.1.4
- **Platform notes:** None

### F-1.1.9 Component Hooks

Register per-type lifecycle callbacks: `on_add` (component added to entity), `on_remove` (component
removed), `on_set` (value written). Hooks run synchronously at the point of change and receive the
entity ID and component reference. Used for maintaining auxiliary data structures (e.g., updating
the spatial index when a `Transform` is added).

- **Requirements:** R-1.1.9
- **Dependencies:** F-1.1.4
- **Platform notes:** None

### F-1.1.10 Component Bundles and Required Components

Group multiple components into a named bundle for atomic insertion (e.g., `RigidBodyBundle` inserts
`RigidBody`, `Velocity`, `Mass`, `Collider` together). Components can declare required companions —
adding `Collider` automatically adds `CollisionLayers` if not present. Bundles are flattened at
compile time with no runtime overhead.

- **Requirements:** R-1.1.10
- **Dependencies:** F-1.1.4
- **Platform notes:** None

## Entities

### F-1.1.11 Entity Lifecycle with Generational Indices

Use generational indices as entity identifiers — a 32-bit index plus a 32-bit generation counter.
Stale references are detected without indirection through a free list. Allocation and deallocation
are O(1) amortized. The generation counter prevents use-after-free bugs when entities are recycled
across simulation ticks.

- **Requirements:** R-1.1.11
- **Dependencies:** F-1.7.4 (Generational Indices)
- **Platform notes:** None

### F-1.1.12 Cleanup Components and Deferred Destruction

Components marked `#[cleanup]` persist after their entity is "destroyed" — all non-cleanup
components are removed but the entity remains alive with its cleanup components intact. A system
detects these lingering entities, performs resource teardown (GPU buffer release, network
deregistration), and then explicitly removes cleanup components to finalize destruction.

- **Requirements:** R-1.1.12
- **Dependencies:** F-1.1.11
- **Platform notes:** None

### F-1.1.13 Entity Names and Path Lookup

Assign human-readable names to entities. Names combine with parent-child hierarchies to form path
names (e.g., `World/Zone3/NPC_Guard_17`). Entities can be looked up by path in O(log n). Used for
debugging, scripting references, and editor entity trees. Names are stored as sparse components to
avoid archetype fragmentation.

- **Requirements:** R-1.1.13
- **Dependencies:** F-1.1.2, F-1.1.11
- **Platform notes:** None

## Relationships and Hierarchies

### F-1.1.14 Entity Relationships (Pairs)

Encode relationships as pairs of two entities — (Relationship, Target) — packed into a single 64-bit
component ID. Adding `(Likes, Apple)` to an entity is equivalent to adding a component. Wildcards
enable queries like "all entities that `Likes` anything" or "anything that targets `Apple`."
Relationships are the foundation for hierarchies, prefabs, and link-based data modeling.

- **Requirements:** R-1.1.14
- **Dependencies:** F-1.1.1, F-1.1.11
- **Platform notes:** None

### F-1.1.15 Relationship Properties

Annotate relationships with properties that control behavior: `Exclusive` (only one target per
relationship), `Symmetric` (auto-add reverse), `Transitive` (A→B→C implies A→C), `Reflexive`,
`Acyclic`, `Traversable`, `With` (auto-add companion). Cleanup policies (`OnDelete`,
`OnDeleteTarget`) control cascading behavior — e.g., deleting a parent cascades to children.

- **Requirements:** R-1.1.15
- **Dependencies:** F-1.1.14
- **Platform notes:** None

### F-1.1.16 Built-In Parent-Child Hierarchy

A `ChildOf` relationship with `Acyclic`, `Traversable`, and `OnDeleteTarget(Delete)` provides
built-in parent-child hierarchies. Queries support `up` (search parent chain) and `cascade`
(breadth-first top-down) traversal. Path-based entity lookup uses the hierarchy. Deleting a parent
cascades destruction to all descendants.

- **Requirements:** R-1.1.16
- **Dependencies:** F-1.1.14, F-1.1.15
- **Platform notes:** None

## Queries

### F-1.1.17 Composable Archetype Queries

Filter archetypes by `With<T>` (required), `Without<T>` (excluded), `Option<T>` (optional),
`Changed<T>` (mutated since last tick), `Added<T>` (newly added). Queries are cached after first
evaluation — repeated execution within a frame incurs no archetype-matching overhead. Cached queries
incrementally update when new archetypes are created.

- **Requirements:** R-1.1.17
- **Dependencies:** F-1.1.1, F-1.1.4
- **Platform notes:** None

### F-1.1.18 Query Sorting and Grouping

Sort query results by a component value using a user-provided comparator — e.g., sort `RenderBatch`
entities by material ID for draw call batching. Group results by relationship target for spatial
partitioning or category-based iteration. Sorting is stable and cached between frames, re-sorted
only when change detection indicates modifications.

- **Requirements:** R-1.1.18
- **Dependencies:** F-1.1.17, F-1.1.7 (Change Detection via F-1.1.22)
- **Platform notes:** None

### F-1.1.19 Query Variables and Pattern Matching

Variables in query terms (e.g., `$parent`, `$target`) enable graph pattern matching across entity
relationships. A query like `(ChildOf, $parent), $parent.Has<Boss>` finds all children of boss
entities in a single query pass. Variables support joins and are essential for relationship-heavy
data models.

- **Requirements:** R-1.1.19
- **Dependencies:** F-1.1.17, F-1.1.14
- **Platform notes:** None

### F-1.1.20 Automatic Parallel Iteration

Partition query results across worker threads using archetype-level or chunk-level granularity. The
scheduler guarantees no two parallel tasks hold conflicting mutable and immutable borrows to the
same component storage. Scales linearly with core count for large entity populations.

- **Requirements:** R-1.1.20
- **Dependencies:** F-1.1.17, F-1.1.25 (System Scheduling)
- **Platform notes:** Mobile: 2-4 worker threads, chunk-level partitioning only. Switch: 3 workers
  handheld, 3 workers docked. Desktop: worker count = physical cores - 1. High-end PC: up to 15+
  workers on 16-core CPUs with archetype-level partitioning.

## Aspects

### F-1.1.21 Component Aspects

Group a subset of an entity's components into a named accessor struct — e.g., `PhysicsAspect` groups
`RigidBody`, `Velocity`, `Mass`, `Transform`. Aspects are used in queries and jobs as a single type
parameter, reducing boilerplate. Nested aspects (an aspect containing another) are supported. Access
modes (`&T` vs `&mut T`) are declared per field.

- **Requirements:** R-1.1.21
- **Dependencies:** F-1.1.17, F-1.1.4
- **Platform notes:** None

## Change Detection

### F-1.1.22 Tick-Based Change Detection

Track per-component mutation using a tick counter bumped on each mutable access. Change detection
operates at chunk granularity — if any entity's component in a chunk was written, the entire chunk
is marked changed. Systems query `Changed<T>` to skip unmodified chunks, enabling reactive patterns
like dirty-flag propagation, network delta compression, and incremental spatial index updates.

- **Requirements:** R-1.1.22
- **Dependencies:** F-1.1.1, F-1.1.17
- **Platform notes:** None

## Resources and Singletons

### F-1.1.23 World Resources

Typed singletons stored in the world — one instance per type. Accessed via `Res<T>` (shared) and
`ResMut<T>` (exclusive) system parameters. Resources participate in the dependency graph and change
detection. Used for global state like `Time`, `PhysicsConfig`, `Broadphase`, `NavMeshTileMap`, and
`InputState`.

- **Requirements:** R-1.1.23
- **Dependencies:** F-1.1.4
- **Platform notes:** None

### F-1.1.24 Non-Send Resources

Resources that must only be accessed from the main thread (e.g., GPU device handles, windowing
handles). The scheduler never moves non-send resource access to worker threads. Systems that access
non-send resources are automatically pinned to the main thread.

- **Requirements:** R-1.1.24
- **Dependencies:** F-1.1.23
- **Platform notes:** None

## System Scheduling

### F-1.1.25 Dependency Resolution and Topological Ordering

Automatically resolve system execution order from declared read/write access sets. Build a DAG of
system dependencies and produce a topological ordering that maximizes parallelism while respecting
data dependencies. Cycles are detected at schedule-build time and reported as errors.

- **Requirements:** R-1.1.25
- **Dependencies:** F-1.1.17
- **Platform notes:** None

### F-1.1.26 System Groups and Phases

Organize systems into hierarchical groups with defined execution order. Built-in phases:
`PreUpdate`, `Update`, `PostUpdate`, `FixedUpdate`, `PreRender`, `Render`. Custom phases can be
inserted with `before`/`after` ordering. Groups can nest — a `PhysicsGroup` inside `FixedUpdate`
contains all physics systems. Disabling a group disables all contained systems.

- **Requirements:** R-1.1.26
- **Dependencies:** F-1.1.25
- **Platform notes:** None

### F-1.1.27 System Run Criteria and Conditions

Systems declare run criteria — predicates evaluated each frame that gate execution. Criteria include
fixed-timestep accumulators, state machine transitions, resource existence checks, and user-defined
predicates. Multiple criteria compose with AND logic. Run criteria avoid branching inside system
bodies and enable clean phase separation.

- **Requirements:** R-1.1.27
- **Dependencies:** F-1.1.25
- **Platform notes:** None

### F-1.1.28 Ambiguity Detection

Detect pairs of systems that access the same components with conflicting access modes (one reads,
one writes) and have no explicit ordering constraint. Report ambiguities at schedule- build time as
warnings. Ambiguity detection prevents subtle nondeterminism from unordered parallel systems —
critical for deterministic MMO simulation.

- **Requirements:** R-1.1.28
- **Dependencies:** F-1.1.25
- **Platform notes:** None

### F-1.1.29 Exclusive Systems

Systems that require exclusive `&mut World` access, preventing all other systems from running
concurrently. Used for operations that cannot be expressed through component queries — e.g.,
archetype migrations, world serialization, or scene loading. The scheduler treats exclusive systems
as full barriers in the execution graph.

- **Requirements:** R-1.1.29
- **Dependencies:** F-1.1.25
- **Platform notes:** None

## Observers

### F-1.1.30 Event-Triggered Observers

Register callbacks that fire when specific events occur on entities matching a query. Built-in
events: `OnAdd`, `OnRemove`, `OnSet`, `OnTableCreate`, `OnTableEmpty`. Observers differ from hooks
in that they match multi-term queries and are evaluated at sync points (deferred), making them safe
for structural changes.

- **Requirements:** R-1.1.30
- **Dependencies:** F-1.1.17, F-1.5.1 (Events)
- **Platform notes:** None

### F-1.1.31 Custom Entity Events

Define application-specific events as types and emit them targeted at specific entities. Observers
on custom events fire for matching entities, enabling gameplay patterns like `DamageEvent`,
`PickupEvent`, or `QuestCompleteEvent` that carry data payloads. Events propagate along relationship
edges (e.g., damage events bubbling up parent chains).

- **Requirements:** R-1.1.31
- **Dependencies:** F-1.1.30, F-1.1.14 (Relationships)
- **Platform notes:** None

## Command Buffers

### F-1.1.32 Deferred Structural Changes via Command Buffers

Per-system command buffers record entity spawn, despawn, and component add/remove operations.
Commands are applied in deterministic order at designated sync points, eliminating runtime borrow
conflicts during parallel system execution and ensuring reproducible simulation state.

- **Requirements:** R-1.1.32
- **Dependencies:** F-1.1.11, F-1.1.1, F-1.5.4 (Deferred Command Buffers)
- **Platform notes:** None

### F-1.1.33 Parallel Command Recording

Multiple worker threads record commands into the same command buffer concurrently using a parallel
writer. Each command carries a sort key (e.g., chunk index) so playback order is deterministic
regardless of recording order. This eliminates the need for per-thread command buffers and
simplifies parallel system authoring.

- **Requirements:** R-1.1.33
- **Dependencies:** F-1.1.32
- **Platform notes:** None

## Worlds

### F-1.1.34 Multiple World Support

Support multiple independent ECS worlds within a single process. Each world owns its own archetype
storage, entity allocator, and resource map. Worlds are tagged with flags (Game, Editor, Server,
Shadow) that control which systems are instantiated. Enables rollback worlds, streaming staging
worlds, and instanced dungeon isolation.

- **Requirements:** R-1.1.34
- **Dependencies:** F-1.1.1, F-1.1.11
- **Platform notes:** Mobile: max 2 concurrent worlds (game + staging). Switch: max 3 worlds.
  Desktop: configurable, default 8. High-end PC: unlimited concurrent worlds.

### F-1.1.35 Entity Migration Between Worlds

Transfer entities with all their components from one world to another. Entity IDs are remapped
during migration to avoid collisions. Used for zone transitions in MMO open worlds — an entity
crossing a zone boundary migrates from one server world to another while preserving full component
state.

- **Requirements:** R-1.1.35
- **Dependencies:** F-1.1.34
- **Platform notes:** None

## Prefabs and Prototypes

### F-1.1.36 Prefab Entities with Inheritance

Entities marked with a `Prefab` tag serve as templates. Instances are created via an `IsA`
relationship that inherits components from the prefab — shared components are stored once in the
prefab, not copied. When an instance writes to an inherited component, it is automatically
overridden (copied to the instance). Prefabs can inherit from other prefabs (variants).

- **Requirements:** R-1.1.36
- **Dependencies:** F-1.1.14 (Relationships), F-1.1.5 (Tags)
- **Platform notes:** None

### F-1.1.37 Prefab Children and Nested Prefabs

Prefabs with child hierarchies instantiate the entire subtree. Nested prefabs (prefab containing
other prefab instances) compose correctly — inner prefab changes propagate to all outer instances.
Slot references allow named access to instantiated children without storing entity handles in
components.

- **Requirements:** R-1.1.37
- **Dependencies:** F-1.1.36, F-1.1.16 (Hierarchy)
- **Platform notes:** None

## State Machines

### F-1.1.38 ECS-Integrated State Machine

Typed state components with `OnEnter`, `OnExit`, and `OnTransition` observer hooks. States are
components — transitioning replaces one state component with another, triggering observers.
Sub-states and computed states (derived from other state combinations) support complex game state
logic (menu, loading, gameplay, paused, cinematic). Systems declare run criteria gated on active
state.

- **Requirements:** R-1.1.38
- **Dependencies:** F-1.1.30 (Observers), F-1.1.27 (Run Criteria)
- **Platform notes:** None
