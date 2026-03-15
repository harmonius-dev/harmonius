# Entity Component System User Stories

## Storage

### US-1.1.1 Archetype-Based Dense Storage

**As an** engine developer, **I want** components stored in contiguous archetype tables with
structure-of-arrays layout, **so that** iteration over large entity populations is cache-friendly
and SIMD-optimized for maximum throughput.

### US-1.1.2 Sparse Component Storage

**As a** game developer, **I want** to mark rarely-queried components as sparse so they bypass
archetype tables, **so that** adding or removing debug markers and temporary status effects does
not cause expensive archetype migrations.

### US-1.1.3 Archetype Graph and Edge Caching

**As an** engine developer, **I want** structural changes to resolve target archetypes in O(1)
via cached graph edges, **so that** bulk spawning and despawning thousands of entities per frame
remains fast regardless of archetype count.

## Components

### US-1.1.4 Static and Dynamic Component Registration

**As a** game developer, **I want** to register components both at compile time and at runtime,
**so that** I get zero-cost access for known types while still supporting hot-reloaded gameplay
logic and server-side scripting.

### US-1.1.5 Tag Components (Zero-Size)

**As a** game developer, **I want** zero-sized tag components that enable query filtering without
consuming per-entity memory, **so that** I can mark entities as `Player`, `Enemy`, or `Static`
efficiently across millions of entities.

### US-1.1.6 Shared Components

**As a** game developer, **I want** components shared across all entities in a chunk stored only
once, **so that** material IDs, LOD groups, and render layer assignments do not waste memory when
thousands of entities share the same value.

### US-1.1.7 Buffer Components (Dynamic Arrays)

**As a** game developer, **I want** variable-length arrays associated with entities via
`DynamicBuffer<T>`, **so that** I can store child lists, inventory slots, and waypoint sequences
per entity without fixed-size limitations.

### US-1.1.8 Enableable Components

**As a** game developer, **I want** to toggle components on and off per entity without structural
changes, **so that** I can enable or disable behaviors like AI perception or physics sleeping
cheaply from parallel worker threads.

### US-1.1.9 Component Hooks

**As an** engine developer, **I want** per-type lifecycle callbacks that fire synchronously on
component add, remove, and set, **so that** I can keep auxiliary data structures like spatial
indices in sync with component changes.

### US-1.1.10 Component Bundles and Required Components

**As a** game developer, **I want** to insert groups of related components atomically as bundles
with auto-added required companions, **so that** I never create incomplete entities missing
essential components like `CollisionLayers` when adding `Collider`.

## Entities

### US-1.1.11 Entity Lifecycle with Generational Indices

**As an** engine developer, **I want** entities identified by generational indices that detect
stale references in O(1), **so that** recycled entity slots cannot be mistakenly accessed through
old handles after despawn.

### US-1.1.12 Cleanup Components and Deferred Destruction

**As an** engine developer, **I want** cleanup components that persist after entity destruction,
**so that** GPU buffers, network registrations, and other external resources can be torn down
gracefully by dedicated cleanup systems.

### US-1.1.13 Entity Names and Path Lookup

**As a** game developer, **I want** to assign human-readable names to entities and look them up
by hierarchical path, **so that** I can reference entities in visual scripts and debug the entity
tree without memorizing raw IDs.

## Relationships and Hierarchies

### US-1.1.14 Entity Relationships (Pairs)

**As a** game developer, **I want** to encode relationships as (Relationship, Target) pairs with
wildcard queries, **so that** I can model complex data like "all entities that like any food" or
"anything targeting this NPC" in a single query.

### US-1.1.15 Relationship Properties

**As a** game developer, **I want** relationships annotated with properties like Exclusive,
Symmetric, and cascading delete policies, **so that** domain invariants are enforced automatically
without hand-written validation logic.

### US-1.1.16 Built-In Parent-Child Hierarchy

**As a** game developer, **I want** a built-in `ChildOf` relationship with automatic cascading
deletion and traversal support, **so that** deleting a parent entity automatically cleans up all
descendants and I can walk the hierarchy in either direction.

## Queries

### US-1.1.17 Composable Archetype Queries

**As a** game developer, **I want** to compose queries with `With`, `Without`, `Option`,
`Changed`, and `Added` filters that are cached after first evaluation, **so that** I can
precisely select entities without repeated archetype matching overhead.

### US-1.1.18 Query Sorting and Grouping

**As a** game developer, **I want** to sort query results by component value and group by
relationship target, **so that** I can batch draw calls by material ID and partition entities
spatially without external sorting.

### US-1.1.19 Query Variables and Pattern Matching

**As a** game developer, **I want** query variables that enable graph pattern matching across
relationships, **so that** I can find all children of boss entities in a single query pass
without nested loops.

### US-1.1.20 Automatic Parallel Iteration

**As an** engine developer, **I want** query results automatically partitioned across worker
threads with borrow safety guarantees, **so that** systems processing large entity populations
scale linearly with core count.

## Aspects

### US-1.1.21 Component Aspects

**As a** game developer, **I want** to group related components into named aspect structs for use
in queries, **so that** I can reduce boilerplate when multiple systems need the same component
grouping like `PhysicsAspect` or `TransformAspect`.

## Change Detection

### US-1.1.22 Tick-Based Change Detection

**As a** game developer, **I want** to query only components that changed since the last tick,
**so that** systems like network delta compression and spatial index updates skip unmodified
chunks and run proportionally to the amount of change.

## Resources and Singletons

### US-1.1.23 World Resources

**As a** game developer, **I want** typed singleton resources in the world accessible through
scheduler-aware `Res<T>` and `ResMut<T>` parameters, **so that** global state like `Time` and
`InputState` is type-safe and participates in dependency analysis.

### US-1.1.24 Non-Send Resources

**As an** engine developer, **I want** resources marked as non-send to be automatically pinned to
the main thread, **so that** GPU device handles and window handles are never accessed from worker
threads.

## System Scheduling

### US-1.1.25 Dependency Resolution and Topological Ordering

**As an** engine developer, **I want** system execution order resolved automatically from declared
access sets with cycle detection, **so that** I do not have to manually order hundreds of systems
while guaranteeing correctness.

### US-1.1.26 System Groups and Phases

**As a** game developer, **I want** systems organized into hierarchical groups and phases like
`Update`, `FixedUpdate`, and `Render` that I can enable or disable as a unit, **so that** I can
cleanly separate gameplay, physics, and rendering execution.

### US-1.1.27 System Run Criteria and Conditions

**As a** game developer, **I want** to gate system execution on predicates like fixed timestep
accumulators and state machine transitions, **so that** systems only run when their conditions
are met without branching inside system bodies.

### US-1.1.28 Ambiguity Detection

**As a** QA engineer, **I want** the scheduler to detect and warn about systems with conflicting
access and no ordering constraint, **so that** I can identify potential nondeterminism before it
causes hard-to-reproduce bugs in deterministic simulation.

### US-1.1.29 Exclusive Systems

**As an** engine developer, **I want** systems that acquire exclusive world access and act as full
barriers, **so that** operations like world serialization and scene loading that cannot be
decomposed into queries still integrate with the scheduler.

## Observers

### US-1.1.30 Event-Triggered Observers

**As a** game developer, **I want** to register callbacks that fire when built-in events occur on
entities matching a query at sync points, **so that** I can implement reactive patterns like
spatial index updates without polling every frame.

### US-1.1.31 Custom Entity Events

**As a** game developer, **I want** to define custom event types and emit them at specific
entities with propagation along relationship edges, **so that** gameplay events like damage and
pickup bubble up parent chains of composite entities.

## Command Buffers

### US-1.1.32 Deferred Structural Changes via Command Buffers

**As an** engine developer, **I want** per-system command buffers that apply structural changes at
sync points in deterministic order, **so that** parallel systems can record entity mutations
without borrow conflicts or nondeterminism.

### US-1.1.33 Parallel Command Recording

**As an** engine developer, **I want** multiple worker threads to record commands into the same
buffer with sort keys ensuring deterministic playback, **so that** parallel systems share a single
command buffer without per-thread merge complexity.

## Worlds

### US-1.1.34 Multiple World Support

**As a** game developer, **I want** multiple independent ECS worlds in a single process with
per-world system instantiation controlled by flags, **so that** I can isolate rollback worlds,
streaming staging, and instanced zones.

### US-1.1.35 Entity Migration Between Worlds

**As a** game developer, **I want** to transfer entities with all components between worlds with
automatic ID remapping, **so that** zone transitions in open worlds preserve full entity state
without ID collisions.

## Prefabs and Prototypes

### US-1.1.36 Prefab Entities with Inheritance

**As a** designer, **I want** prefab templates where instances inherit shared components and
automatically override on write, **so that** I can create thousands of similar entities from a
single template while customizing individual instances.

### US-1.1.37 Prefab Children and Nested Prefabs

**As a** designer, **I want** nested prefab hierarchies where inner prefab changes propagate to
all outer instances, **so that** I can compose complex game objects from reusable prefab parts
and update them centrally.

## State Machines

### US-1.1.38 ECS-Integrated State Machine

**As a** game developer, **I want** typed state components with observer hooks for enter, exit,
and transition that gate system execution, **so that** game state management (menu, loading,
gameplay, paused) integrates naturally with the ECS scheduler.
