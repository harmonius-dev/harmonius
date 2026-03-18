# 1.1 — Entity Component System

## Storage

| ID      | Feature                          | Requirements |
|---------|----------------------------------|--------------|
| F-1.1.1 | Archetype-Based Dense Storage    | R-1.1.1      |
| F-1.1.2 | Sparse Component Storage         | R-1.1.2      |
| F-1.1.3 | Archetype Graph and Edge Caching | R-1.1.3      |

1. **F-1.1.1** — Store components in contiguous, cache-friendly archetype tables where each unique
   combination of component types defines an archetype. Components within an archetype use
   structure-of-arrays layout — one contiguous array per component type per table — maximizing
   iteration throughput and SIMD potential. Fixed-size chunks (configurable, default 16 KiB)
   subdivide tables for cache-line-aligned access.
   - **Deps:** F-1.7.1 (Arena Allocators), F-1.3.1 (Type Registry)
   - **Platform:** Mobile: 8 KiB default chunk size to fit L1 cache. Switch: 8 KiB handheld, 16 KiB
     docked. Desktop: 16 KiB default, configurable up to 64 KiB.
2. **F-1.1.2** — Allow components to opt into sparse-set storage via a `#[sparse]` attribute,
   bypassing archetype tables entirely. Sparse components are stored in a separate per-type sparse
   set keyed by entity ID. When an entity gains or loses a sparse component, it does not change
   archetype — eliminating table fragmentation for rarely-queried or high-churn components (e.g.,
   debug markers, temporary status effects).
   - **Deps:** F-1.1.1, F-1.1.4
3. **F-1.1.3** — Maintain a directed graph of archetypes where edges represent single-component
   additions or removals. Cache edge lookups so that structural changes resolve the target archetype
   in O(1). Edges can encode invariants (e.g., adding component A always brings component B). This
   avoids linear scans during entity migration, keeping bulk spawn and despawn operations fast at
   scale.
   - **Deps:** F-1.1.1

## Components

| ID       | Feature                                   | Requirements |
|----------|-------------------------------------------|--------------|
| F-1.1.4  | Static and Dynamic Component Registration | R-1.1.4      |
| F-1.1.5  | Tag Components (Zero-Size)                | R-1.1.5      |
| F-1.1.6  | Shared Components                         | R-1.1.6      |
| F-1.1.7  | Buffer Components (Dynamic Arrays)        | R-1.1.7      |
| F-1.1.8  | Enableable Components                     | R-1.1.8      |
| F-1.1.9  | Component Hooks                           | R-1.1.9      |
| F-1.1.10 | Component Bundles and Required Components | R-1.1.10     |

1. **F-1.1.4** — Support both compile-time (derive macro) and runtime component registration. Static
   registration enables zero-cost access patterns and monomorphized queries. Dynamic registration
   allows server-side scripting and hot-reloaded gameplay logic to introduce new component types
   without recompilation. Each component records size, alignment, drop function, and storage mode.
   - **Deps:** F-1.3.1 (Type Registry)
2. **F-1.1.5** — Components with no data fields occupy zero bytes in storage. Tags alter archetype
   identity (enabling query filtering) without consuming memory per entity. Used for markers like
   `Player`, `Enemy`, `Static`, `CcdEnabled`, `Sleeping`. The archetype table stores no column for
   tag components.
   - **Deps:** F-1.1.1, F-1.1.4
3. **F-1.1.6** — Components whose value is shared by all entities in a chunk. Stored once per chunk
   rather than per entity — zero per-entity memory cost. Changing a shared component value is a
   structural change that moves the entity to a different chunk. Ideal for material IDs, LOD groups,
   and render layer assignments where many entities share the same value.
   - **Deps:** F-1.1.1
4. **F-1.1.7** — Variable-length, resizable arrays associated with an entity. Small buffers stored
   inline in the archetype chunk; large buffers spill to a heap allocation. Used for child entity
   lists, inventory slots, waypoint sequences, and any per-entity collection. Accessed through a
   typed `DynamicBuffer<T>` handle.
   - **Deps:** F-1.1.1, F-1.7.2 (Pool Allocators)
   - **Platform:** Mobile: inline threshold 128 bytes, max buffer 4 KiB before spill. Switch: inline
     threshold 256 bytes. Desktop: inline threshold 512 bytes, no cap.
5. **F-1.1.8** — Components marked `#[enableable]` can be toggled on/off per entity without
   structural changes. Disabled components remain in the archetype table but are excluded from
   queries by default. Toggling is safe from parallel worker threads without command buffers.
   Queries can explicitly request `WithDisabled<T>` or `WithPresent<T>` to override default
   filtering.
   - **Deps:** F-1.1.1, F-1.1.4
6. **F-1.1.9** — Register per-type lifecycle callbacks: `on_add` (component added to entity),
   `on_remove` (component removed), `on_set` (value written). Hooks run synchronously at the point
   of change and receive the entity ID and component reference. Used for maintaining auxiliary data
   structures (e.g., updating the spatial index when a `Transform` is added).
   - **Deps:** F-1.1.4
7. **F-1.1.10** — Group multiple components into a named bundle for atomic insertion (e.g.,
   `RigidBodyBundle` inserts `RigidBody`, `Velocity`, `Mass`, `Collider` together). Components can
   declare required companions — adding `Collider` automatically adds `CollisionLayers` if not
   present. Bundles are flattened at compile time with no runtime overhead.
   - **Deps:** F-1.1.4

## Entities

| ID       | Feature                                     | Requirements |
|----------|---------------------------------------------|--------------|
| F-1.1.11 | Entity Lifecycle with Generational Indices  | R-1.1.11     |
| F-1.1.12 | Cleanup Components and Deferred Destruction | R-1.1.12     |
| F-1.1.13 | Entity Names and Path Lookup                | R-1.1.13     |

1. **F-1.1.11** — Use generational indices as entity identifiers — a 32-bit index plus a 32-bit
   generation counter. Stale references are detected without indirection through a free list.
   Allocation and deallocation are O(1) amortized. The generation counter prevents use-after-free
   bugs when entities are recycled across simulation ticks.
   - **Deps:** F-1.7.4 (Generational Indices)
2. **F-1.1.12** — Components marked `#[cleanup]` persist after their entity is "destroyed" — all
   non-cleanup components are removed but the entity remains alive with its cleanup components
   intact. A system detects these lingering entities, performs resource teardown (GPU buffer
   release, network deregistration), and then explicitly removes cleanup components to finalize
   destruction.
   - **Deps:** F-1.1.11
3. **F-1.1.13** — Assign human-readable names to entities. Names combine with parent-child
   hierarchies to form path names (e.g., `World/Zone3/NPC_Guard_17`). Entities can be looked up by
   path in O(log n). Used for debugging, scripting references, and editor entity trees. Names are
   stored as sparse components to avoid archetype fragmentation.
   - **Deps:** F-1.1.2, F-1.1.11

## Relationships and Hierarchies

| ID       | Feature                         | Requirements |
|----------|---------------------------------|--------------|
| F-1.1.14 | Entity Relationships (Pairs)    | R-1.1.14     |
| F-1.1.15 | Relationship Properties         | R-1.1.15     |
| F-1.1.16 | Built-In Parent-Child Hierarchy | R-1.1.16     |

1. **F-1.1.14** — Encode relationships as pairs of two entities — (Relationship, Target) — packed
   into a single 64-bit component ID. Adding `(Likes, Apple)` to an entity is equivalent to adding a
   component. Wildcards enable queries like "all entities that `Likes` anything" or "anything that
   targets `Apple`." Relationships are the foundation for hierarchies, prefabs, and link-based data
   modeling.
   - **Deps:** F-1.1.1, F-1.1.11
2. **F-1.1.15** — Annotate relationships with properties that control behavior: `Exclusive` (only
   one target per relationship), `Symmetric` (auto-add reverse), `Transitive` (A→B→C implies A→C),
   `Reflexive`, `Acyclic`, `Traversable`, `With` (auto-add companion). Cleanup policies (`OnDelete`,
   `OnDeleteTarget`) control cascading behavior — e.g., deleting a parent cascades to children.
   - **Deps:** F-1.1.14
3. **F-1.1.16** — A `ChildOf` relationship with `Acyclic`, `Traversable`, and
   `OnDeleteTarget(Delete)` provides built-in parent-child hierarchies. Queries support `up` (search
   parent chain) and `cascade` (breadth-first top-down) traversal. Path-based entity lookup uses the
   hierarchy. Deleting a parent cascades destruction to all descendants.
   - **Deps:** F-1.1.14, F-1.1.15

## Queries

| ID       | Feature                              | Requirements |
|----------|--------------------------------------|--------------|
| F-1.1.17 | Composable Archetype Queries         | R-1.1.17     |
| F-1.1.18 | Query Sorting and Grouping           | R-1.1.18     |
| F-1.1.19 | Query Variables and Pattern Matching | R-1.1.19     |
| F-1.1.20 | Automatic Parallel Iteration         | R-1.1.20     |

1. **F-1.1.17** — Filter archetypes by `With<T>` (required), `Without<T>` (excluded), `Option<T>`
   (optional), `Changed<T>` (mutated since last tick), `Added<T>` (newly added). Queries are cached
   after first evaluation — repeated execution within a frame incurs no archetype-matching overhead.
   Cached queries incrementally update when new archetypes are created.
   - **Deps:** F-1.1.1, F-1.1.4
2. **F-1.1.18** — Sort query results by a component value using a user-provided comparator — e.g.,
   sort `RenderBatch` entities by material ID for draw call batching. Group results by relationship
   target for spatial partitioning or category-based iteration. Sorting is stable and cached between
   frames, re-sorted only when change detection indicates modifications.
   - **Deps:** F-1.1.17, F-1.1.7 (Change Detection via F-1.1.22)
3. **F-1.1.19** — Variables in query terms (e.g., `$parent`, `$target`) enable graph pattern
   matching across entity relationships. A query like `(ChildOf, $parent), $parent.Has<Boss>` finds
   all children of boss entities in a single query pass. Variables support joins and are essential
   for relationship-heavy data models.
   - **Deps:** F-1.1.17, F-1.1.14
4. **F-1.1.20** — Partition query results across worker threads using archetype-level or chunk-level
   granularity. The scheduler guarantees no two parallel tasks hold conflicting mutable and
   immutable borrows to the same component storage. Scales linearly with core count for large entity
   populations.
   - **Deps:** F-1.1.17, F-1.1.25 (System Scheduling)
   - **Platform:** Mobile: 2-4 worker threads, chunk-level partitioning only. Switch: 3 workers
     handheld, 3 workers docked. Desktop: worker count = physical cores - 1. High-end PC: up to 15+
     workers on 16-core CPUs with archetype-level partitioning.

## Aspects

| ID       | Feature           | Requirements |
|----------|-------------------|--------------|
| F-1.1.21 | Component Aspects | R-1.1.21     |

1. **F-1.1.21** — Group a subset of an entity's components into a named accessor struct — e.g.,
   `PhysicsAspect` groups `RigidBody`, `Velocity`, `Mass`, `Transform`. Aspects are used in queries
   and jobs as a single type parameter, reducing boilerplate. Nested aspects (an aspect containing
   another) are supported. Access modes (`&T` vs `&mut T`) are declared per field.
   - **Deps:** F-1.1.17, F-1.1.4

## Change Detection

| ID       | Feature                     | Requirements |
|----------|-----------------------------|--------------|
| F-1.1.22 | Tick-Based Change Detection | R-1.1.22     |

1. **F-1.1.22** — Track per-component mutation using a tick counter bumped on each mutable access.
   Change detection operates at chunk granularity — if any entity's component in a chunk was
   written, the entire chunk is marked changed. Systems query `Changed<T>` to skip unmodified
   chunks, enabling reactive patterns like dirty-flag propagation, network delta compression, and
   incremental spatial index updates.
   - **Deps:** F-1.1.1, F-1.1.17

## Resources and Singletons

| ID       | Feature            | Requirements |
|----------|--------------------|--------------|
| F-1.1.23 | World Resources    | R-1.1.23     |
| F-1.1.24 | Non-Send Resources | R-1.1.24     |

1. **F-1.1.23** — Typed singletons stored in the world — one instance per type. Accessed via
   `Res<T>` (shared) and `ResMut<T>` (exclusive) system parameters. Resources participate in the
   dependency graph and change detection. Used for global state like `Time`, `PhysicsConfig`,
   `Broadphase`, `NavMeshTileMap`, and `InputState`.
   - **Deps:** F-1.1.4
2. **F-1.1.24** — Resources that must only be accessed from the main thread (e.g., GPU device
   handles, windowing handles). The scheduler never moves non-send resource access to worker
   threads. Systems that access non-send resources are automatically pinned to the main thread.
   - **Deps:** F-1.1.23

## System Scheduling

| ID       | Feature                                        | Requirements |
|----------|------------------------------------------------|--------------|
| F-1.1.25 | Dependency Resolution and Topological Ordering | R-1.1.25     |
| F-1.1.26 | System Groups and Phases                       | R-1.1.26     |
| F-1.1.27 | System Run Criteria and Conditions             | R-1.1.27     |
| F-1.1.28 | Ambiguity Detection                            | R-1.1.28     |
| F-1.1.29 | Exclusive Systems                              | R-1.1.29     |

1. **F-1.1.25** — Automatically resolve system execution order from declared read/write access sets.
   Build a DAG of system dependencies and produce a topological ordering that maximizes parallelism
   while respecting data dependencies. Cycles are detected at schedule-build time and reported as
   errors.
   - **Deps:** F-1.1.17
2. **F-1.1.26** — Organize systems into hierarchical groups with defined execution order. Built-in
   phases: `PreUpdate`, `Update`, `PostUpdate`, `FixedUpdate`, `PreRender`, `Render`. Custom phases
   can be inserted with `before`/`after` ordering. Groups can nest — a `PhysicsGroup` inside
   `FixedUpdate` contains all physics systems. Disabling a group disables all contained systems.
   - **Deps:** F-1.1.25
3. **F-1.1.27** — Systems declare run criteria — predicates evaluated each frame that gate
   execution. Criteria include fixed-timestep accumulators, state machine transitions, resource
   existence checks, and user-defined predicates. Multiple criteria compose with AND logic. Run
   criteria avoid branching inside system bodies and enable clean phase separation.
   - **Deps:** F-1.1.25
4. **F-1.1.28** — Detect pairs of systems that access the same components with conflicting access
   modes (one reads, one writes) and have no explicit ordering constraint. Report ambiguities at
   schedule- build time as warnings. Ambiguity detection prevents subtle nondeterminism from
   unordered parallel systems — critical for deterministic MMO simulation.
   - **Deps:** F-1.1.25
5. **F-1.1.29** — Systems that require exclusive `&mut World` access, preventing all other systems
   from running concurrently. Used for operations that cannot be expressed through component queries
   — e.g., archetype migrations, world serialization, or scene loading. The scheduler treats
   exclusive systems as full barriers in the execution graph.
   - **Deps:** F-1.1.25

## Observers

| ID       | Feature                   | Requirements |
|----------|---------------------------|--------------|
| F-1.1.30 | Event-Triggered Observers | R-1.1.30     |
| F-1.1.31 | Custom Entity Events      | R-1.1.31     |

1. **F-1.1.30** — Register callbacks that fire when specific events occur on entities matching a
   query. Built-in events: `OnAdd`, `OnRemove`, `OnSet`, `OnTableCreate`, `OnTableEmpty`. Observers
   differ from hooks in that they match multi-term queries and are evaluated at sync points
   (deferred), making them safe for structural changes.
   - **Deps:** F-1.1.17, F-1.5.1 (Events)
2. **F-1.1.31** — Define application-specific events as types and emit them targeted at specific
   entities. Observers on custom events fire for matching entities, enabling gameplay patterns like
   `DamageEvent`, `PickupEvent`, or `QuestCompleteEvent` that carry data payloads. Events propagate
   along relationship edges (e.g., damage events bubbling up parent chains).
   - **Deps:** F-1.1.30, F-1.1.14 (Relationships)

## Command Buffers

| ID       | Feature                                         | Requirements |
|----------|-------------------------------------------------|--------------|
| F-1.1.32 | Deferred Structural Changes via Command Buffers | R-1.1.32     |
| F-1.1.33 | Parallel Command Recording                      | R-1.1.33     |

1. **F-1.1.32** — Per-system command buffers record entity spawn, despawn, and component add/remove
   operations. Commands are applied in deterministic order at designated sync points, eliminating
   runtime borrow conflicts during parallel system execution and ensuring reproducible simulation
   state.
   - **Deps:** F-1.1.11, F-1.1.1, F-1.5.4 (Deferred Command Buffers)
2. **F-1.1.33** — Multiple worker threads record commands into the same command buffer concurrently
   using a parallel writer. Each command carries a sort key (e.g., chunk index) so playback order is
   deterministic regardless of recording order. This eliminates the need for per-thread command
   buffers and simplifies parallel system authoring.
   - **Deps:** F-1.1.32

## Worlds

| ID       | Feature                         | Requirements |
|----------|---------------------------------|--------------|
| F-1.1.34 | Multiple World Support          | R-1.1.34     |
| F-1.1.35 | Entity Migration Between Worlds | R-1.1.35     |

1. **F-1.1.34** — Support multiple independent ECS worlds within a single process. Each world owns
   its own archetype storage, entity allocator, and resource map. Worlds are tagged with flags
   (Game, Editor, Server, Shadow) that control which systems are instantiated. Enables rollback
   worlds, streaming staging worlds, and instanced dungeon isolation.
   - **Deps:** F-1.1.1, F-1.1.11
   - **Platform:** Mobile: max 2 concurrent worlds (game + staging). Switch: max 3 worlds. Desktop:
     configurable, default 8. High-end PC: unlimited concurrent worlds.
2. **F-1.1.35** — Transfer entities with all their components from one world to another. Entity IDs
   are remapped during migration to avoid collisions. Used for zone transitions in MMO open worlds —
   an entity crossing a zone boundary migrates from one server world to another while preserving
   full component state.
   - **Deps:** F-1.1.34

## Prefabs and Prototypes

| ID       | Feature                            | Requirements |
|----------|------------------------------------|--------------|
| F-1.1.36 | Prefab Entities with Inheritance   | R-1.1.36     |
| F-1.1.37 | Prefab Children and Nested Prefabs | R-1.1.37     |

1. **F-1.1.36** — Entities marked with a `Prefab` tag serve as templates. Instances are created via
   an `IsA` relationship that inherits components from the prefab — shared components are stored
   once in the prefab, not copied. When an instance writes to an inherited component, it is
   automatically overridden (copied to the instance). Prefabs can inherit from other prefabs
   (variants).
   - **Deps:** F-1.1.14 (Relationships), F-1.1.5 (Tags)
2. **F-1.1.37** — Prefabs with child hierarchies instantiate the entire subtree. Nested prefabs
   (prefab containing other prefab instances) compose correctly — inner prefab changes propagate to
   all outer instances. Slot references allow named access to instantiated children without storing
   entity handles in components.
   - **Deps:** F-1.1.36, F-1.1.16 (Hierarchy)

## State Machines

| ID       | Feature                      | Requirements |
|----------|------------------------------|--------------|
| F-1.1.38 | ECS-Integrated State Machine | R-1.1.38     |

1. **F-1.1.38** — Typed state components with `OnEnter`, `OnExit`, and `OnTransition` observer
   hooks. States are components — transitioning replaces one state component with another,
   triggering observers. Sub-states and computed states (derived from other state combinations)
   support complex game state logic (menu, loading, gameplay, paused, cinematic). Systems declare
   run criteria gated on active state.
   - **Deps:** F-1.1.30 (Observers), F-1.1.27 (Run Criteria)
