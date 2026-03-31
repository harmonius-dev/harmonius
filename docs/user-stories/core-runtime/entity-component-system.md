# Entity Component System User Stories

## Storage

| ID       | Persona                 |
|----------|-------------------------|
| US-1.1.1 | engine developer (P-26) |
| US-1.1.2 | game developer (P-15)   |
| US-1.1.3 | engine developer (P-26) |

1. **US-1.1.1** — **As an** engine developer (P-26), **I want** components stored in contiguous
   archetype tables with structure-of-arrays layout and configurable chunk sizes, **so that**
   iteration over large entity populations is cache-friendly and SIMD-optimized.
2. **US-1.1.2** — **As a** game developer (P-15), **I want** to mark components as sparse so they
   bypass archetype tables and use per-type sparse sets, **so that** adding or removing debug
   markers and temporary status effects does not trigger archetype migrations.
3. **US-1.1.3** — **As an** engine developer (P-26), **I want** a directed archetype graph with
   cached edge lookups resolving target archetypes in O(1), **so that** bulk spawning and despawning
   thousands of entities per frame remains fast regardless of archetype count.

## Components

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.1.4  | game developer (P-15)   |
| US-1.1.5  | game developer (P-15)   |
| US-1.1.6  | game developer (P-15)   |
| US-1.1.7  | game developer (P-15)   |
| US-1.1.8  | game developer (P-15)   |
| US-1.1.9  | engine developer (P-26) |
| US-1.1.10 | game developer (P-15)   |

1. **US-1.1.4** — **As a** game developer (P-15), **I want** to register components at compile time
   via derive macros and at runtime for hot-reloaded gameplay logic, **so that** I get zero-cost
   access for known types while supporting dynamic scripting.
2. **US-1.1.5** — **As a** game developer (P-15), **I want** zero-sized tag components that alter
   archetype identity and enable query filtering without per-entity memory cost, **so that** I can
   mark entities as Player, Enemy, or Static efficiently across millions of entities.
3. **US-1.1.6** — **As a** game developer (P-15), **I want** shared components stored once per chunk
   rather than per entity, **so that** material IDs, LOD groups, and render layer assignments do not
   waste memory when thousands of entities share the same value.
4. **US-1.1.7** — **As a** game developer (P-15), **I want** variable-length buffer components
   accessed via DynamicBuffer with inline small buffers and heap spill, **so that** I can store
   child lists, inventory slots, and waypoint sequences per entity without fixed-size limits.
5. **US-1.1.8** — **As a** game developer (P-15), **I want** to toggle enableable components on and
   off per entity without structural changes, **so that** I can enable or disable behaviors like AI
   perception cheaply from parallel worker threads.
6. **US-1.1.9** — **As an** engine developer (P-26), **I want** per-type lifecycle callbacks
   (on_add, on_remove, on_set) that fire synchronously at the point of change, **so that** auxiliary
   data structures like the spatial index stay in sync with component changes.
7. **US-1.1.10** — **As a** game developer (P-15), **I want** to insert groups of related components
   atomically as named bundles with auto-added required companions, **so that** I never create
   incomplete entities missing essential components.

## Entities

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.1.11 | engine developer (P-26) |
| US-1.1.12 | engine developer (P-26) |
| US-1.1.13 | game developer (P-15)   |

1. **US-1.1.11** — **As an** engine developer (P-26), **I want** entities identified by generational
   indices (32-bit index + 32-bit generation) that detect stale references in O(1), **so that**
   recycled entity slots cannot be mistakenly accessed through old handles.
2. **US-1.1.12** — **As an** engine developer (P-26), **I want** cleanup components that persist
   after entity destruction so dedicated systems can tear down GPU buffers and network
   registrations, **so that** external resources are cleaned up gracefully rather than leaked.
3. **US-1.1.13** — **As a** game developer (P-15), **I want** to assign human-readable names to
   entities and look them up by hierarchical path, **so that** I can reference entities in Logic
   Graphs and debug the entity tree without memorizing raw IDs.

## Relationships and Hierarchies

| ID        | Persona               |
|-----------|-----------------------|
| US-1.1.14 | game developer (P-15) |
| US-1.1.15 | game developer (P-15) |
| US-1.1.16 | game developer (P-15) |
| US-1.1.17 | engine tester (P-27)  |

1. **US-1.1.14** — **As a** game developer (P-15), **I want** relationships encoded as
   (Relationship, Target) pairs with wildcard queries, **so that** I can model complex data like
   "all entities that like any food" or "anything targeting this NPC" in a single query.
2. **US-1.1.15** — **As a** game developer (P-15), **I want** relationships annotated with
   properties like Exclusive, Symmetric, Transitive, Acyclic, and cascading delete policies,
   **so that** domain invariants are enforced automatically without hand-written validation.
3. **US-1.1.16** — **As a** game developer (P-15), **I want** a built-in ChildOf relationship with
   automatic cascading deletion and traversal support, **so that** deleting a parent entity
   automatically cleans up all descendants.
4. **US-1.1.17** — **As an** engine tester (P-27), **I want** to verify that deleting a parent at
   the root of a deep hierarchy cascades correctly through all descendants, **so that** no orphaned
   entities remain after hierarchical deletion.

## Queries

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.1.18 | game developer (P-15)   |
| US-1.1.19 | game developer (P-15)   |
| US-1.1.20 | game developer (P-15)   |
| US-1.1.21 | engine developer (P-26) |
| US-1.1.22 | engine tester (P-27)    |

1. **US-1.1.18** — **As a** game developer (P-15), **I want** to compose queries with With, Without,
   Option, Changed, and Added filters that cache after first evaluation, **so that** I can precisely
   select entities without repeated archetype matching overhead.
2. **US-1.1.19** — **As a** game developer (P-15), **I want** to sort query results by component
   value and group by relationship target, **so that** I can batch draw calls by material ID and
   partition entities spatially without external sorting.
3. **US-1.1.20** — **As a** game developer (P-15), **I want** query variables enabling graph pattern
   matching across entity relationships, **so that** I can find all children of boss entities in a
   single query pass.
4. **US-1.1.21** — **As an** engine developer (P-26), **I want** query results automatically
   partitioned across worker threads with borrow safety guarantees, **so that** systems processing
   large entity populations scale linearly with core count.
5. **US-1.1.22** — **As an** engine tester (P-27), **I want** to benchmark parallel query iteration
   at various entity counts and core counts, **so that** I can verify linear scaling and identify
   contention.

## Aspects

| ID        | Persona               |
|-----------|-----------------------|
| US-1.1.23 | game developer (P-15) |

1. **US-1.1.23** — **As a** game developer (P-15), **I want** to group related components into named
   aspect structs like PhysicsAspect, **so that** I reduce boilerplate when multiple systems need
   the same component grouping.

## Change Detection

| ID        | Persona               |
|-----------|-----------------------|
| US-1.1.24 | game developer (P-15) |
| US-1.1.25 | engine tester (P-27)  |

1. **US-1.1.24** — **As a** game developer (P-15), **I want** to query only components modified
   since the last tick using Changed, **so that** network delta compression, spatial index updates,
   and dirty-flag propagation skip unmodified chunks.
2. **US-1.1.25** — **As an** engine tester (P-27), **I want** to verify that change detection
   correctly marks only chunks containing modified entities, **so that** reactive systems process
   exactly the correct set of changes with no false positives.

## Resources and Singletons

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.1.26 | game developer (P-15)   |
| US-1.1.27 | engine developer (P-26) |

1. **US-1.1.26** — **As a** game developer (P-15), **I want** typed singleton resources accessible
   through scheduler-aware Res and ResMut parameters with change detection, **so that** global state
   like Time and InputState is type-safe and participates in dependency analysis.
2. **US-1.1.27** — **As an** engine developer (P-26), **I want** resources marked non-send to be
   automatically pinned to the game loop thread by the scheduler, **so that** GPU device handles and
   window handles are never accessed from worker threads.

## System Scheduling

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.1.28 | engine developer (P-26) |
| US-1.1.29 | game developer (P-15)   |
| US-1.1.30 | game developer (P-15)   |
| US-1.1.31 | engine developer (P-26) |
| US-1.1.32 | engine developer (P-26) |

1. **US-1.1.28** — **As an** engine developer (P-26), **I want** system execution order resolved
   automatically from declared read/write access sets with DAG construction and cycle detection,
   **so that** I do not manually order hundreds of systems while guaranteeing correctness.
2. **US-1.1.29** — **As a** game developer (P-15), **I want** systems organized into hierarchical
   groups and phases (PreUpdate, Update, PostUpdate, FixedUpdate, PreRender, Render) with
   before/after ordering, **so that** I can cleanly separate gameplay, physics, and rendering
   execution.
3. **US-1.1.30** — **As a** game developer (P-15), **I want** systems gated by predicates like
   fixed-timestep accumulators, state machine transitions, and resource existence checks,
   **so that** systems only run when their conditions are met without branching inside system
   bodies.
4. **US-1.1.31** — **As an** engine developer (P-26), **I want** the scheduler to detect pairs of
   systems with conflicting access and no ordering constraint, **so that** I identify potential
   nondeterminism before it causes hard-to-reproduce bugs in deterministic simulation.
5. **US-1.1.32** — **As an** engine developer (P-26), **I want** exclusive systems that acquire &mut
   World access and act as full barriers, **so that** operations like world serialization and scene
   loading still integrate with the scheduler.

## Observers

| ID        | Persona               |
|-----------|-----------------------|
| US-1.1.33 | game developer (P-15) |
| US-1.1.34 | game developer (P-15) |

1. **US-1.1.33** — **As a** game developer (P-15), **I want** to register callbacks that fire when
   specific events (OnAdd, OnRemove, OnSet) occur on entities matching a query at sync points,
   **so that** I can implement reactive patterns like spatial index updates without polling.
2. **US-1.1.34** — **As a** game developer (P-15), **I want** to define custom event types and emit
   them at specific entities with propagation along relationship edges, **so that** gameplay events
   like DamageEvent bubble up parent chains of composite entities.

## Command Buffers

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.1.35 | engine developer (P-26) |
| US-1.1.36 | engine developer (P-26) |
| US-1.1.37 | engine tester (P-27)    |

1. **US-1.1.35** — **As an** engine developer (P-26), **I want** per-system command buffers that
   record spawn, despawn, and component operations applied in deterministic order at sync points,
   **so that** parallel systems can record mutations without borrow conflicts.
2. **US-1.1.36** — **As an** engine developer (P-26), **I want** multiple worker threads to record
   commands into the same buffer with sort keys for deterministic playback, **so that** parallel
   systems share a single command buffer without per-thread merge overhead.
3. **US-1.1.37** — **As an** engine tester (P-27), **I want** to verify that parallel command
   recording produces identical playback order regardless of thread scheduling, **so that**
   deterministic simulation is maintained under varying thread timing.

## Worlds

| ID        | Persona               |
|-----------|-----------------------|
| US-1.1.38 | game developer (P-15) |
| US-1.1.39 | game developer (P-15) |
| US-1.1.40 | engine tester (P-27)  |

1. **US-1.1.38** — **As a** game developer (P-15), **I want** multiple independent ECS worlds in a
   single process with per-world system instantiation controlled by flags, **so that** I can isolate
   rollback worlds, streaming staging, and instanced zones.
2. **US-1.1.39** — **As a** game developer (P-15), **I want** to transfer entities with all
   components between worlds with automatic ID remapping, **so that** zone transitions in open
   worlds preserve full entity state without ID collisions.
3. **US-1.1.40** — **As an** engine tester (P-27), **I want** to stress-test entity migration with
   hundreds of entities crossing zone boundaries simultaneously, **so that** I can verify no
   component data is lost during bulk world-to-world transfer.

## Entity Templates and Prototypes

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.1.41 | technical artist (P-13) |
| US-1.1.42 | technical artist (P-13) |
| US-1.1.43 | engine developer (P-26) |

1. **US-1.1.41** — **As a** technical artist (P-13), **I want** Entity Template entities where
   instances inherit shared components and automatically override on write, **so that** I can create
   thousands of similar entities from a single template while customizing individual instances.
2. **US-1.1.42** — **As a** technical artist (P-13), **I want** nested Entity Template hierarchies
   where inner template changes propagate to all outer instances with slot references, **so that** I
   can compose complex game objects from reusable parts and update them centrally.
3. **US-1.1.43** — **As an** engine developer (P-26), **I want** Entity Template instantiation to
   share component storage between template and instances until override, **so that** memory usage
   is proportional to overrides rather than total instance count.

## State Machines

| ID        | Persona               |
|-----------|-----------------------|
| US-1.1.44 | game developer (P-15) |
| US-1.1.45 | engine tester (P-27)  |

1. **US-1.1.44** — **As a** game developer (P-15), **I want** typed state components with OnEnter,
   OnExit, and OnTransition observer hooks that gate system execution, **so that** game state
   management integrates naturally with the ECS scheduler.
2. **US-1.1.45** — **As an** engine tester (P-27), **I want** to verify that state transitions fire
   the correct OnEnter and OnExit observers and activate the correct systems, **so that**
   state-gated execution is deterministic and reliable.
