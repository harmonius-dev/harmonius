# Entity Component System Design

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are defined in
> [features/core-runtime/](../../features/core-runtime/),
> [requirements/core-runtime/](../../requirements/core-runtime/), and
> [user-stories/core-runtime/](../../user-stories/core-runtime/). The table below traces design
> elements to those definitions.

| Feature  | Requirement         |
|----------|---------------------|
| F-1.1.1  | R-1.1.1, R-1.1.1a   |
| F-1.1.2  | R-1.1.2, R-1.1.2a   |
| F-1.1.3  | R-1.1.3, R-1.1.3a   |
| F-1.1.4  | R-1.1.4             |
| F-1.1.5  | R-1.1.5             |
| F-1.1.6  | R-1.1.6             |
| F-1.1.7  | R-1.1.7             |
| F-1.1.8  | R-1.1.8             |
| F-1.1.9  | R-1.1.9, R-1.1.9a   |
| F-1.1.10 | R-1.1.10            |
| F-1.1.11 | R-1.1.11, R-1.1.11a |
| F-1.1.12 | R-1.1.12            |
| F-1.1.13 | R-1.1.13            |
| F-1.1.14 | R-1.1.14            |
| F-1.1.15 | R-1.1.15            |
| F-1.1.16 | R-1.1.16, R-1.1.16a |
| F-1.1.17 | R-1.1.17, R-1.1.17a |
| F-1.1.18 | R-1.1.18            |
| F-1.1.19 | R-1.1.19            |
| F-1.1.20 | R-1.1.20            |
| F-1.1.21 | R-1.1.21            |
| F-1.1.22 | R-1.1.22, R-1.1.22a |
| F-1.1.23 | R-1.1.23            |
| F-1.1.24 | R-1.1.24            |
| F-1.1.25 | R-1.1.25, R-1.1.25a |
| F-1.1.26 | R-1.1.26            |
| F-1.1.27 | R-1.1.27            |
| F-1.1.28 | R-1.1.28            |
| F-1.1.29 | R-1.1.29            |
| F-1.1.30 | R-1.1.30, R-1.1.30a |
| F-1.1.31 | R-1.1.31            |
| F-1.1.32 | R-1.1.32, R-1.1.32a |
| F-1.1.33 | R-1.1.33            |
| F-1.1.34 | R-1.1.34            |
| F-1.1.35 | R-1.1.35, R-1.1.35a |
| F-1.1.36 | R-1.1.36            |
| F-1.1.37 | R-1.1.37            |
| F-1.1.38 | R-1.1.38            |

1. **F-1.1.1** — Archetype-based dense storage, SoA layout, chunked
2. **F-1.1.2** — Sparse-set storage for `#[sparse]` components
3. **F-1.1.3** — Archetype graph with O(1) cached edge transitions
4. **F-1.1.4** — Static (derive) and dynamic component registration
5. **F-1.1.5** — Zero-size tag components
6. **F-1.1.6** — Shared components (one value per chunk)
7. **F-1.1.7** — Buffer components (dynamic arrays per entity)
8. **F-1.1.8** — Enableable components (toggle without migration)
9. **F-1.1.9** — Component hooks (on_add, on_remove, on_set)
10. **F-1.1.10** — Component bundles and required companions
11. **F-1.1.11** — Entity lifecycle with generational indices
12. **F-1.1.12** — Cleanup components and deferred destruction
13. **F-1.1.13** — Entity names and hierarchical path lookup
14. **F-1.1.14** — Entity relationships (pairs)
15. **F-1.1.15** — Relationship properties (Exclusive, Symmetric, ...)
16. **F-1.1.16** — Built-in ChildOf hierarchy with cascading delete
17. **F-1.1.17** — Composable archetype queries with caching
18. **F-1.1.18** — Query sorting and grouping
19. **F-1.1.19** — Query variables and pattern matching
20. **F-1.1.20** — Automatic parallel iteration
21. **F-1.1.21** — Component aspects
22. **F-1.1.22** — Tick-based change detection at chunk granularity
23. **F-1.1.23** — World resources (typed singletons)
24. **F-1.1.24** — Non-send resources (main-thread pinned)
25. **F-1.1.25** — Dependency resolution and topological ordering
26. **F-1.1.26** — System groups and phases
27. **F-1.1.27** — System run criteria and conditions
28. **F-1.1.28** — Ambiguity detection
29. **F-1.1.29** — Exclusive systems (full barriers)
30. **F-1.1.30** — Event-triggered observers
31. **F-1.1.31** — Custom entity events with propagation
32. **F-1.1.32** — Deferred structural changes via command buffers
33. **F-1.1.33** — Parallel command recording with sort keys
34. **F-1.1.34** — Multiple independent worlds
35. **F-1.1.35** — Entity migration between worlds
36. **F-1.1.36** — Prefab entities with inheritance
37. **F-1.1.37** — Prefab children and nested prefabs
38. **F-1.1.38** — ECS-integrated state machine

## Overview

The ECS is the foundational data model and execution framework for every domain in the Harmonius
engine. All simulation data lives as components, all logic runs as systems, and all state is owned
by a `World`. There are no parallel data stores, no separate physics world, no renderer scene graph
diverging from the ECS. The ECS defines the interoperability contracts consumed by every downstream
design: `Component`, `Entity`, `System`, and `World`.

Key architectural choices:

1. **Archetype table storage** as the default. Components grouped by archetype in contiguous SoA
   arrays within fixed-size chunks (16 KiB default) for cache-friendly iteration.
2. **Sparse-set storage** as an opt-in alternative for high-churn or rarely-queried components,
   avoiding archetype fragmentation.
3. **Archetype graph** with cached edges for O(1) structural transitions when adding or removing
   components.
4. **Generational entity indices** (32-bit index + 32-bit generation) for O(1) allocation,
   deallocation, and stale-reference detection.
5. **Relationship pairs** encoded as 64-bit component IDs for hierarchies, prefabs, and graph-based
   data modeling.
6. **Tick-based change detection** at chunk granularity for reactive patterns like dirty-flag
   propagation and network delta compression.
7. **Automatic system scheduling** via dependency analysis on declared access sets, producing a
   `TaskGraph` (from `threading.md`) for parallel execution on the `ThreadPool`.
8. **Command buffers** for deferred structural changes, flushed at sync points in deterministic
   order.
9. **Observers** for reactive callbacks on component add/remove/change, evaluated during command
   buffer flush.

**Audio runtime exception.** Per constraints.md, the audio mixer runs on a dedicated real-time
thread with a < 0.5 ms latency budget. ECS components (`AudioSource`, `AudioListener`) bridge game
state to the audio runtime via a lock-free SPSC command queue. The audio thread owns its own mix
buffers and effect chains outside the ECS.

## Architecture

### Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_ecs"
        W[World]
        EA[EntityAllocator]
        AS[ArchetypeStorage]
        AG[ArchetypeGraph]
        SS[SparseSetStorage]
        CR[ComponentRegistry]
        QC[QueryCache]
        QE[QueryEngine]
        SC[Scheduler]
        CB[CommandBuffer]
        OB[ObserverRegistry]
        CD[ChangeDetector]
        SM[StateMachine]
    end

    subgraph "harmonius_ecs::system"
        SY[System trait]
        SP[SystemParam trait]
        SG[SystemGroup]
        PH[Phase]
        RC[RunCriteria]
    end

    subgraph "harmonius_platform::threading"
        TP[ThreadPool]
        TG[TaskGraph]
        GLG[GameLoopGraph]
        CF[CompiledFrame]
    end

    subgraph "harmonius_core::reflect"
        RF[Reflect trait]
        TR[TypeRegistry]
    end

    W --> EA
    W --> AS
    W --> SS
    W --> CR
    W --> QC
    W --> OB
    W --> CD

    AS --> AG
    CR --> RF
    CR --> TR
    QE --> QC
    QE --> AS
    QE --> SS
    QE --> CD

    SC --> GLG
    GLG --> CF
    CF --> TG
    CF --> TP
    SC --> SG
    SC --> QE
    SG --> PH
    SG --> RC

    CB --> W
    OB --> CB
    SM --> OB
    SY --> SP
```

### File Layout

```text
harmonius_ecs/
├── entity/
│   ├── allocator.rs   # EntityAllocator, free list,
│   │                  # generational index
│   ├── entity.rs      # Entity, EntityMeta
│   └── entity_ref.rs  # EntityRef, EntityMut,
│                      # EntityWorldMut
├── component/
│   ├── component.rs   # Component trait, ComponentId,
│   │                  # StorageMode
│   ├── registry.rs    # ComponentRegistry,
│   │                  # ComponentDescriptor
│   ├── bundle.rs      # Bundle trait, derive macro
│   ├── hooks.rs       # ComponentHooks, HookFn
│   └── info.rs        # ComponentInfo, layout, drop
├── storage/
│   ├── archetype.rs   # Archetype, ArchetypeId,
│   │                  # ArchetypeStorage
│   ├── graph.rs       # ArchetypeGraph, EdgeMap
│   ├── column.rs      # Column, BlobVec
│   ├── chunk.rs       # Chunk, chunk capacity calc
│   ├── sparse_set.rs  # SparseSet, SparseSetMap
│   ├── table.rs       # Table row operations
│   └── shared.rs      # SharedComponentStorage
├── query/
│   ├── state.rs       # QueryState, AccessSet
│   ├── iter.rs        # QueryIter, ParQueryIter
│   ├── filter.rs      # With, Without, Changed,
│   │                  # Added, Option
│   ├── fetch.rs       # WorldQuery, ReadOnlyQuery
│   ├── cache.rs       # QueryCache, incremental
│   │                  # archetype matching
│   └── variable.rs    # QueryVariable,
│                      # PatternMatcher
├── relationship/
│   ├── pair.rs        # RelationPair, pair encoding
│   ├── properties.rs  # Exclusive, Symmetric,
│   │                  # Transitive, Acyclic
│   ├── hierarchy.rs   # ChildOf, parent-child ops
│   └── traversal.rs   # Up, Cascade traversal
├── system/
│   ├── system.rs      # System trait, ErasedSystem
│   ├── param.rs       # SystemParam trait, Res,
│   │                  # ResMut, Query, Commands
│   ├── function.rs    # IntoSystem, function
│   │                  # systems
│   ├── exclusive.rs   # ExclusiveSystem
│   └── criteria.rs    # RunCriterion, conditions
├── schedule/
│   ├── schedule.rs    # Schedule, build, run
│   ├── graph.rs       # SystemGraph, dep analysis,
│   │                  # topo sort
│   ├── phase.rs       # Phase enum, custom phases
│   ├── group.rs       # SystemGroup, nesting
│   └── ambiguity.rs   # AmbiguityDetector
├── observer/
│   ├── observer.rs    # Observer, ObserverRegistry
│   ├── trigger.rs     # Trigger, OnAdd, OnRemove,
│   │                  # OnSet
│   └── event.rs       # EntityEvent, propagation
├── command/
│   ├── buffer.rs      # CommandBuffer, Command enum
│   └── parallel.rs    # ParallelCommandWriter,
│                      # sort keys
├── world/
│   ├── world.rs       # World, world flags
│   ├── resource.rs    # ResourceMap, Res, ResMut,
│   │                  # NonSend
│   ├── migration.rs   # entity migration between
│   │                  # worlds
│   └── change.rs      # ChangeTick, ChangeDetector
├── prefab/
│   ├── prefab.rs      # Prefab tag, IsA
│   │                  # relationship
│   ├── instance.rs    # instantiation, overrides
│   └── slot.rs        # SlotRef, named child
│                      # access
├── state/
│   ├── state.rs       # State trait, NextState
│   └── machine.rs     # OnEnter, OnExit,
│                      # OnTransition
└── aspect/
    └── aspect.rs      # Aspect derive, nested
                       # aspects
```

### Core Data Structures

```mermaid
classDiagram
    class Entity {
        +index: u32
        +generation: u32
        +from_raw(index, generation) Entity
        +id() u64
        +is_placeholder() bool
    }

    class EntityAllocator {
        -entities: Vec~EntityMeta~
        -free_list: Vec~u32~
        -len: u32
        +allocate() Entity
        +deallocate(entity) bool
        +is_alive(entity) bool
        +meta(entity) Option~EntityMeta~
        +len() u32
    }

    class EntityMeta {
        +generation: u32
        +archetype_id: ArchetypeId
        +archetype_row: u32
    }

    class World {
        -allocator: EntityAllocator
        -archetypes: ArchetypeStorage
        -sparse_sets: SparseSetMap
        -components: ComponentRegistry
        -resources: ResourceMap
        -observers: ObserverRegistry
        -change_tick: AtomicU32
        +new() World
        +spawn(bundle) Entity
        +despawn(entity) bool
        +entity(entity) EntityRef
        +entity_mut(entity) EntityMut
        +query~Q~() QueryState~Q~
        +resource~T~() Res~T~
        +resource_mut~T~() ResMut~T~
        +send_event~E~(event)
    }

    class ArchetypeStorage {
        -archetypes: Vec~Archetype~
        -graph: ArchetypeGraph
        +get(id) Archetype
        +get_or_create(components) ArchetypeId
        +archetype_count() u32
    }

    class Archetype {
        -id: ArchetypeId
        -columns: HashMap~ComponentId, Column~
        -entities: Vec~Entity~
        -shared_values: HashMap~ComponentId, SharedValue~
        -chunk_capacity: u32
        +len() u32
        +has_component(id) bool
        +column(id) Option~Column~
    }

    class Column {
        -data: BlobVec
        -change_ticks: Vec~ChangeTick~
        -component_id: ComponentId
        +len() u32
        +get_unchecked(row) Ptr
        +get_mut_unchecked(row) MutPtr
    }

    class ArchetypeGraph {
        -edges: HashMap~ArchetypeId, EdgeMap~
        +traverse_add(from, comp) ArchetypeId
        +traverse_remove(from, comp) ArchetypeId
    }

    EntityAllocator *-- EntityMeta
    World *-- EntityAllocator
    World *-- ArchetypeStorage
    World --> ComponentRegistry
    ArchetypeStorage *-- Archetype
    ArchetypeStorage *-- ArchetypeGraph
    Archetype *-- Column
    Entity ..> EntityMeta : validated via
```

### Component and Query Data Structures

```mermaid
classDiagram
    class ComponentId {
        +index: u32
    }

    class ComponentDescriptor {
        +id: ComponentId
        +name: str
        +layout: Layout
        +drop_fn: Option~DropFn~
        +storage_mode: StorageMode
        +is_send: bool
        +type_id: Option~TypeId~
    }

    class StorageMode {
        &lt;&lt;enumeration&gt;&gt;
        Table
        Sparse
    }

    class ComponentRegistry {
        -descriptors: Vec~ComponentDescriptor~
        -type_id_map: HashMap~TypeId, ComponentId~
        -name_map: HashMap~String, ComponentId~
        +register~T: Component~() ComponentId
        +register_dynamic(desc) ComponentId
        +get(id) ComponentDescriptor
        +lookup~T~() Option~ComponentId~
    }

    class QueryState~Q~ {
        -matched_archetypes: SmallVec~ArchetypeId 8~
        -component_access: AccessSet
        -last_check_archetype_count: u32
        +new(world) QueryState~Q~
        +iter(world) QueryIter~Q~
        +par_iter(world, pool) ParQueryIter~Q~
        +get(world, entity) Option~Q::Item~
        +single(world) Q::Item
    }

    class AccessSet {
        -reads: FixedBitSet
        -writes: FixedBitSet
        +reads_component(id) bool
        +writes_component(id) bool
        +is_compatible(other) bool
    }

    class ChangeTick {
        +last_changed: u32
        +last_run: u32
        +is_changed(current_tick) bool
    }

    ComponentRegistry *-- ComponentDescriptor
    ComponentDescriptor --> StorageMode
    ComponentDescriptor --> ComponentId
    QueryState --> AccessSet
    Column --> ChangeTick
```

### System Scheduling Data Structures

```mermaid
classDiagram
    class Schedule {
        -systems: Vec~SystemNode~
        -groups: Vec~SystemGroup~
        -phases: Vec~Phase~
        -graph: SystemGraph
        -ambiguities: Vec~Ambiguity~
        +add_system(system, phase)
        +add_group(group)
        +build_game_loop(world) Result~GameLoopGraph~
    }

    class SystemNode {
        -system: ErasedSystem
        -access: AccessSet
        -run_criteria: Vec~RunCriterion~
        -phase: PhaseId
        -is_exclusive: bool
        -is_main_thread: bool
    }

    class SystemGraph {
        -nodes: Vec~SystemNode~
        -edges: Vec~Edge~
        -topo_order: Vec~SystemNodeId~
        +detect_cycles() Result
        +detect_ambiguities() Vec~Ambiguity~
        +build_system_phase(phase) SystemPhase
    }

    class GameLoopGraph {
        -phases: Vec~PhaseNode~
        -edges: Vec~PhaseId PhaseId~
        +compile(world, pool) Result~CompiledFrame~
    }

    class CompiledFrame {
        -task_graph: TaskGraph
        -render_submissions: Vec~RenderSubmission~
        +execute(world, pool, reactor)
    }

    class Phase {
        &lt;&lt;enumeration&gt;&gt;
        PreUpdate
        Update
        PostUpdate
        FixedUpdate
        PreRender
        Render
        Custom
    }

    class RunCriterion {
        &lt;&lt;trait&gt;&gt;
        +should_run(world) bool
    }

    class CommandBuffer {
        -commands: Vec~Command~
        -sort_keys: Vec~u64~
        +spawn(bundle) Entity
        +despawn(entity)
        +insert~T~(entity, component)
        +remove~T~(entity)
        +flush(world)
    }

    class ObserverRegistry {
        -observers: HashMap~EventType, Vec~Observer~~
        +register(event, query, callback)
        +dispatch(events, world)
    }

    Schedule *-- SystemNode
    Schedule *-- SystemGraph
    Schedule --> Phase
    SystemNode --> AccessSet
    SystemNode --> RunCriterion
    Schedule ..> GameLoopGraph : builds
    GameLoopGraph ..> CompiledFrame : compiles
    CompiledFrame --> TaskGraph
    CommandBuffer ..> World : flush into
    ObserverRegistry ..> CommandBuffer : may trigger
```

### Relationship Pair Encoding

```mermaid
graph TD
    subgraph Encoding["Pair: 64-bit ComponentId"]
        HI["High 32 bits: Relationship"]
        LO["Low 32 bits: Target Entity"]
    end

    subgraph Hierarchy["ChildOf Example"]
        E1["Entity 1 - Parent"]
        E2["Entity 2 - Child"]
        E3["Entity 3 - Child"]
        E2 -->|ChildOf| E1
        E3 -->|ChildOf| E1
    end
```

## API Design

### Entity

```rust
/// An entity identifier. 32-bit index + 32-bit
/// generation counter. Total size: 8 bytes.
/// Implements Copy, Clone, Debug, PartialEq, Eq,
/// Hash, Reflect.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    /// Sentinel value for "no entity."
    pub const PLACEHOLDER: Entity = Entity {
        index: u32::MAX,
        generation: 0,
    };

    /// Construct from raw parts. Used for
    /// deserialization and tests.
    pub fn from_raw(
        index: u32,
        generation: u32,
    ) -> Self;

    /// Pack into a single u64 for hashing and
    /// storage. Layout: [generation:32][index:32].
    pub fn to_bits(self) -> u64;

    /// Unpack from u64.
    pub fn from_bits(bits: u64) -> Self;

    pub fn index(self) -> u32;
    pub fn generation(self) -> u32;

    pub fn is_placeholder(self) -> bool;
}
```

### Entity Allocator

```rust
/// Metadata for a single entity slot.
pub(crate) struct EntityMeta {
    pub generation: u32,
    pub archetype_id: ArchetypeId,
    pub archetype_row: u32,
}

/// Manages entity allocation and deallocation
/// with generational indices and a free list.
pub(crate) struct EntityAllocator {
    entities: Vec<EntityMeta>,
    free_list: Vec<u32>,
    len: u32,
}

impl EntityAllocator {
    pub fn new() -> Self;

    /// Allocate a new entity. O(1) amortized.
    /// Reuses slots from the free list with an
    /// incremented generation counter.
    pub fn allocate(&mut self) -> Entity;

    /// Deallocate an entity. O(1). Pushes the
    /// slot onto the free list with incremented
    /// generation.
    pub fn deallocate(
        &mut self,
        entity: Entity,
    ) -> bool;

    /// Check whether the entity handle is still
    /// valid. O(1) — compares generation counters.
    pub fn is_alive(&self, entity: Entity) -> bool;

    /// Get metadata for a live entity.
    pub fn meta(
        &self,
        entity: Entity,
    ) -> Option<&EntityMeta>;

    /// Mutable metadata access (for archetype
    /// migration).
    pub fn meta_mut(
        &mut self,
        entity: Entity,
    ) -> Option<&mut EntityMeta>;

    /// Number of live entities.
    pub fn len(&self) -> u32;
}
```

### Component

```rust
/// Identifies a component type. Internally a u32
/// index into the ComponentRegistry.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct ComponentId(pub(crate) u32);

/// Storage strategy for a component type.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
)]
pub enum StorageMode {
    /// Dense archetype table storage (default).
    /// Cache-friendly SoA layout in fixed chunks.
    Table,
    /// Sparse-set storage. Component changes do
    /// not trigger archetype migration.
    Sparse,
}

/// Trait implemented by all component types.
/// Derive via `#[derive(Component)]`.
pub trait Component: Send + Sync + 'static {
    /// Storage mode. Overridden by `#[sparse]`.
    const STORAGE_MODE: StorageMode =
        StorageMode::Table;
}

/// Full descriptor for a registered component.
pub struct ComponentDescriptor {
    pub id: ComponentId,
    pub name: &'static str,
    pub layout: Layout,
    pub drop_fn: Option<unsafe fn(*mut u8)>,
    pub storage_mode: StorageMode,
    pub is_send: bool,
    pub type_id: Option<TypeId>,
}

/// Central registry for all component types.
pub struct ComponentRegistry {
    descriptors: Vec<ComponentDescriptor>,
    type_id_map: HashMap<TypeId, ComponentId>,
    name_map: HashMap<String, ComponentId>,
}

impl ComponentRegistry {
    pub fn new() -> Self;

    /// Register a statically-known component type.
    /// Called once per type; subsequent calls return
    /// the existing ComponentId.
    pub fn register<T: Component>(
        &mut self,
    ) -> ComponentId;

    /// Register a component at runtime (dynamic
    /// scripting, hot-reload). Caller provides
    /// size, alignment, drop fn, and storage mode.
    pub fn register_dynamic(
        &mut self,
        desc: ComponentDescriptor,
    ) -> ComponentId;

    /// Look up by Rust TypeId.
    pub fn lookup<T: Component>(
        &self,
    ) -> Option<ComponentId>;

    /// Look up by string name (for scripting).
    pub fn lookup_by_name(
        &self,
        name: &str,
    ) -> Option<ComponentId>;

    /// Get the descriptor for a registered
    /// component.
    pub fn get(
        &self,
        id: ComponentId,
    ) -> &ComponentDescriptor;

    pub fn count(&self) -> u32;
}
```

### Component Hooks

```rust
/// Function signature for component lifecycle
/// hooks. Receives the world, entity, and a
/// pointer to the component data.
pub type HookFn = fn(
    &mut World,
    Entity,
    ComponentId,
);

/// Per-component-type lifecycle hooks.
pub struct ComponentHooks {
    pub on_add: SmallVec<[HookFn; 4]>,
    pub on_remove: SmallVec<[HookFn; 4]>,
    pub on_set: SmallVec<[HookFn; 4]>,
}

impl ComponentHooks {
    pub fn new() -> Self;

    /// Register an on_add hook. Max 16 per type.
    pub fn add_on_add(
        &mut self,
        f: HookFn,
    ) -> Result<(), HookError>;

    /// Register an on_remove hook. Max 16 per
    /// type.
    pub fn add_on_remove(
        &mut self,
        f: HookFn,
    ) -> Result<(), HookError>;

    /// Register an on_set hook. Max 16 per type.
    pub fn add_on_set(
        &mut self,
        f: HookFn,
    ) -> Result<(), HookError>;
}

pub enum HookError {
    /// Exceeded the 16-hook-per-type limit.
    LimitExceeded { component: ComponentId },
}
```

### Bundle

```rust
/// A group of components inserted atomically.
/// Derive via `#[derive(Bundle)]`.
///
/// Bundles are flattened at compile time. The
/// derive macro generates code that inserts each
/// field's component in a single archetype
/// transition.
pub trait Bundle: Send + Sync + 'static {
    /// Component IDs in this bundle, in
    /// declaration order.
    fn component_ids(
        registry: &mut ComponentRegistry,
    ) -> Vec<ComponentId>;

    /// Write all component values into a raw
    /// table row.
    unsafe fn write_components(
        self,
        table_row: &mut TableRow,
        registry: &ComponentRegistry,
    );
}

/// Required companion components. When component
/// A is inserted and its #[require] list includes
/// B, B is auto-inserted with its Default value
/// if not already present.
///
/// Declared via `#[derive(Component)]` attribute:
///   #[require(CollisionLayers)]
///   struct Collider { ... }
```

### Archetype Storage

```rust
/// Identifies an archetype. Index into
/// ArchetypeStorage::archetypes.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct ArchetypeId(pub(crate) u32);

/// An archetype: a unique combination of component
/// types with contiguous SoA storage.
pub struct Archetype {
    id: ArchetypeId,
    /// One column per non-tag, non-shared component
    /// in this archetype.
    columns: HashMap<ComponentId, Column>,
    /// Entity list. Index = row in columns.
    entities: Vec<Entity>,
    /// Shared component values (one per chunk).
    shared_values:
        HashMap<ComponentId, Vec<SharedValue>>,
    /// Tag component IDs (no column needed).
    tags: HashSet<ComponentId>,
    /// Enableable component bit arrays. One bit
    /// per entity per enableable component.
    enable_bits:
        HashMap<ComponentId, FixedBitSet>,
    /// Chunk capacity computed from component
    /// sizes and platform cache line.
    chunk_capacity: u32,
}

impl Archetype {
    pub fn id(&self) -> ArchetypeId;
    pub fn len(&self) -> u32;
    pub fn is_empty(&self) -> bool;
    pub fn has_component(
        &self,
        id: ComponentId,
    ) -> bool;
    pub fn column(
        &self,
        id: ComponentId,
    ) -> Option<&Column>;
    pub fn column_mut(
        &mut self,
        id: ComponentId,
    ) -> Option<&mut Column>;
    pub fn entities(&self) -> &[Entity];
    pub fn chunk_capacity(&self) -> u32;
    pub fn chunk_count(&self) -> u32;
}

/// Type-erased contiguous column of component
/// data within an archetype.
pub struct Column {
    /// Raw byte storage. Elements are laid out
    /// contiguously with proper alignment.
    data: BlobVec,
    /// Per-chunk change tick. One entry per chunk.
    change_ticks: Vec<ChangeTick>,
    /// Per-entity added tick. One entry per entity.
    added_ticks: Vec<u32>,
    component_id: ComponentId,
    item_layout: Layout,
}

impl Column {
    pub fn len(&self) -> u32;

    /// Get a read pointer to row `index`.
    /// Caller must ensure index is in bounds and
    /// the type matches.
    pub unsafe fn get_unchecked(
        &self,
        index: u32,
    ) -> *const u8;

    /// Get a write pointer to row `index`.
    /// Also marks the containing chunk as changed
    /// for the given tick.
    pub unsafe fn get_mut_unchecked(
        &mut self,
        index: u32,
        change_tick: u32,
    ) -> *mut u8;

    /// Returns the change tick for the chunk
    /// containing `row`.
    pub fn chunk_change_tick(
        &self,
        row: u32,
    ) -> &ChangeTick;
}

/// Manages all archetypes and the archetype graph.
pub struct ArchetypeStorage {
    archetypes: Vec<Archetype>,
    graph: ArchetypeGraph,
}

impl ArchetypeStorage {
    pub fn new() -> Self;

    pub fn get(
        &self,
        id: ArchetypeId,
    ) -> &Archetype;

    pub fn get_mut(
        &mut self,
        id: ArchetypeId,
    ) -> &mut Archetype;

    /// Find or create the archetype for a given
    /// component set.
    pub fn get_or_create(
        &mut self,
        component_ids: &[ComponentId],
        registry: &ComponentRegistry,
    ) -> ArchetypeId;

    pub fn archetype_count(&self) -> u32;
}
```

### Archetype Graph

```rust
/// Directed graph of archetypes. Edges represent
/// single-component add/remove transitions.
/// Edge lookups are O(1) via HashMap caching.
pub struct ArchetypeGraph {
    /// Per-archetype edge map. Key = source
    /// archetype, value = edge map.
    edges: HashMap<ArchetypeId, EdgeMap>,
}

/// Edges from a single archetype node.
struct EdgeMap {
    /// component_id -> target archetype when
    /// adding that component.
    add_edges: HashMap<ComponentId, ArchetypeId>,
    /// component_id -> target archetype when
    /// removing that component.
    remove_edges:
        HashMap<ComponentId, ArchetypeId>,
}

impl ArchetypeGraph {
    pub fn new() -> Self;

    /// Traverse the "add component" edge from
    /// `from`. If the edge is not yet cached,
    /// compute the target archetype, create it
    /// if needed, and cache the edge. O(1)
    /// amortized.
    pub fn traverse_add(
        &mut self,
        from: ArchetypeId,
        component: ComponentId,
        storage: &mut ArchetypeStorage,
        registry: &ComponentRegistry,
    ) -> ArchetypeId;

    /// Traverse the "remove component" edge. Same
    /// caching behavior as traverse_add.
    pub fn traverse_remove(
        &mut self,
        from: ArchetypeId,
        component: ComponentId,
        storage: &mut ArchetypeStorage,
        registry: &ComponentRegistry,
    ) -> ArchetypeId;
}
```

### Sparse-Set Storage

```rust
/// A sparse set for a single component type.
/// Provides O(1) lookup, insert, and remove by
/// entity index. Does not affect archetype
/// identity.
pub struct SparseSet<T: Component> {
    /// Dense array of (entity_index, T) pairs.
    dense: Vec<(u32, T)>,
    /// Sparse array: entity_index -> dense_index.
    /// Uses a paged sparse array to avoid
    /// allocating MAX_ENTITIES entries.
    sparse: PagedSparseArray,
}

impl<T: Component> SparseSet<T> {
    pub fn new() -> Self;

    pub fn get(
        &self,
        entity: Entity,
    ) -> Option<&T>;

    pub fn get_mut(
        &mut self,
        entity: Entity,
    ) -> Option<&mut T>;

    pub fn insert(
        &mut self,
        entity: Entity,
        value: T,
    );

    pub fn remove(
        &mut self,
        entity: Entity,
    ) -> Option<T>;

    pub fn contains(
        &self,
        entity: Entity,
    ) -> bool;

    pub fn len(&self) -> u32;
}

/// Type-erased sparse set for dynamic components.
pub struct ErasedSparseSet { /* ... */ }

/// Map of ComponentId -> ErasedSparseSet for all
/// sparse components in a world.
pub struct SparseSetMap {
    sets: HashMap<ComponentId, ErasedSparseSet>,
}
```

### Change Detection

```rust
/// Per-chunk change tracking. Stores the tick at
/// which the chunk was last mutated and the tick
/// at which the observing system last ran.
/// Size: 8 bytes per component type per chunk.
#[derive(Clone, Copy, Debug)]
pub struct ChangeTick {
    pub last_changed: u32,
}

impl ChangeTick {
    /// Returns true if this chunk was modified
    /// after `last_run` and before or at
    /// `current_tick`.
    pub fn is_changed(
        &self,
        last_run: u32,
        current_tick: u32,
    ) -> bool;
}

/// Global tick counter. Incremented once per
/// system run. Stored in World as AtomicU32.
pub struct ChangeDetector {
    tick: AtomicU32,
}

impl ChangeDetector {
    pub fn new() -> Self;

    /// Increment and return the new tick value.
    /// Called by the scheduler before each system.
    pub fn increment(&self) -> u32;

    /// Current tick value.
    pub fn current(&self) -> u32;
}
```

### Enableable Components

```rust
/// Enableable components can be toggled per entity
/// without structural changes. The component data
/// stays in the archetype table; only a bitmask
/// controls visibility.
///
/// Toggling is an atomic bit flip -- safe from
/// parallel worker threads without command buffers.
///
/// Derive via:
///   #[derive(Component)]
///   #[enableable]
///   struct AiPerception { ... }

impl Archetype {
    /// Check if an enableable component is
    /// currently enabled for the given entity
    /// row.
    pub fn is_enabled(
        &self,
        component: ComponentId,
        row: u32,
    ) -> bool;

    /// Toggle an enableable component. Atomic
    /// operation -- safe from any thread.
    pub fn set_enabled(
        &self,
        component: ComponentId,
        row: u32,
        enabled: bool,
    );
}
```

### Relationships

```rust
/// A relationship pair. Encodes a (Relationship,
/// Target) pair as a single 64-bit ComponentId.
///
/// Layout:
///   High 32 bits: Relationship ComponentId.index
///   Low 32 bits:  Target Entity.index
///
/// Adding (ChildOf, parent_entity) to an entity
/// is equivalent to adding a component whose
/// ComponentId encodes both the relationship type
/// and the target.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct RelationPair {
    pub relationship: ComponentId,
    pub target: Entity,
}

impl RelationPair {
    /// Encode as a 64-bit component ID.
    pub fn to_component_id(self) -> ComponentId;

    /// Decode from a 64-bit component ID.
    pub fn from_component_id(
        id: ComponentId,
    ) -> Option<Self>;

    /// Wildcard: match any target for this
    /// relationship.
    pub fn wildcard_target(
        relationship: ComponentId,
    ) -> RelationPair;

    /// Wildcard: match any relationship to this
    /// target.
    pub fn wildcard_relationship(
        target: Entity,
    ) -> RelationPair;
}

/// Properties that control relationship behavior.
#[derive(Clone, Debug)]
pub struct RelationshipProperties {
    /// Only one target per relationship per entity.
    pub exclusive: bool,
    /// Auto-add reverse relationship.
    pub symmetric: bool,
    /// A->B->C implies A->C for queries.
    pub transitive: bool,
    /// Prevents cycles. Validated on add.
    pub acyclic: bool,
    /// Enables Up/Cascade traversal.
    pub traversable: bool,
    /// Cleanup policy when the source is deleted.
    pub on_delete: CleanupPolicy,
    /// Cleanup policy when the target is deleted.
    pub on_delete_target: CleanupPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CleanupPolicy {
    /// No action.
    None,
    /// Remove the relationship component.
    Remove,
    /// Delete the entity.
    Delete,
}
```

### Built-In ChildOf Hierarchy

```rust
/// Marker component for the parent-child
/// relationship. Registered with:
///   Acyclic, Traversable,
///   OnDeleteTarget(Delete)
pub struct ChildOf;

impl World {
    /// Set entity as child of parent. Adds
    /// (ChildOf, parent) relationship pair.
    pub fn set_parent(
        &mut self,
        child: Entity,
        parent: Entity,
    ) -> Result<(), HierarchyError>;

    /// Remove parent-child relationship.
    pub fn remove_parent(
        &mut self,
        child: Entity,
    );

    /// Get the parent of an entity, if any.
    pub fn parent(
        &self,
        entity: Entity,
    ) -> Option<Entity>;

    /// Iterate children of an entity.
    pub fn children(
        &self,
        parent: Entity,
    ) -> ChildIter;

    /// Traverse up the parent chain.
    pub fn ancestors(
        &self,
        entity: Entity,
    ) -> AncestorIter;

    /// Traverse down breadth-first from root.
    pub fn descendants(
        &self,
        root: Entity,
    ) -> DescendantIter;
}

pub enum HierarchyError {
    /// Adding this parent would create a cycle.
    CycleDetected {
        child: Entity,
        parent: Entity,
    },
    /// Entity does not exist.
    EntityNotFound(Entity),
    /// Maximum hierarchy depth (256) exceeded.
    DepthExceeded {
        entity: Entity,
        depth: u32,
    },
}
```

### Queries

```rust
/// Read-only component access in a query.
/// Written as `&T` in query tuples.
pub struct Ref<'w, T: Component> {
    value: &'w T,
    ticks: &'w ChangeTick,
}

impl<'w, T: Component> Ref<'w, T> {
    pub fn is_changed(
        &self,
        last_run: u32,
        current_tick: u32,
    ) -> bool;

    pub fn is_added(
        &self,
        last_run: u32,
        current_tick: u32,
    ) -> bool;
}

impl<'w, T: Component> Deref for Ref<'w, T> {
    type Target = T;
    fn deref(&self) -> &T;
}

/// Mutable component access in a query.
/// Written as `&mut T` in query tuples.
pub struct Mut<'w, T: Component> {
    value: &'w mut T,
    ticks: &'w mut ChangeTick,
    current_tick: u32,
}

impl<'w, T: Component> Deref for Mut<'w, T> {
    type Target = T;
    fn deref(&self) -> &T;
}

impl<'w, T: Component> DerefMut for Mut<'w, T> {
    /// Marks the chunk as changed when the value
    /// is mutably accessed.
    fn deref_mut(&mut self) -> &mut T;
}

/// Tracks the set of components read and written
/// by a query or system. Used for borrow safety
/// and dependency analysis.
pub struct AccessSet {
    reads: FixedBitSet,
    writes: FixedBitSet,
}

impl AccessSet {
    pub fn new() -> Self;
    pub fn add_read(&mut self, id: ComponentId);
    pub fn add_write(&mut self, id: ComponentId);
    pub fn reads_component(
        &self,
        id: ComponentId,
    ) -> bool;
    pub fn writes_component(
        &self,
        id: ComponentId,
    ) -> bool;

    /// True if `self` and `other` have no
    /// conflicting accesses (two writes, or read
    /// + write to the same component).
    pub fn is_compatible(
        &self,
        other: &AccessSet,
    ) -> bool;

    /// Merge another AccessSet into this one.
    pub fn extend(&mut self, other: &AccessSet);
}

/// Cached query state. Built once, reused across
/// frames. Incrementally updated when new
/// archetypes are created.
pub struct QueryState<Q: WorldQuery> {
    matched_archetypes:
        SmallVec<[ArchetypeId; 8]>,
    component_access: AccessSet,
    last_check_archetype_count: u32,
    _marker: PhantomData<Q>,
}

impl<Q: WorldQuery> QueryState<Q> {
    /// Build a new cached query.
    pub fn new(world: &World) -> Self;

    /// Sequential iteration over all matching
    /// entities.
    pub fn iter<'w>(
        &mut self,
        world: &'w World,
    ) -> QueryIter<'w, Q>;

    /// Parallel iteration. Partitions work across
    /// ThreadPool workers at chunk granularity.
    pub fn par_iter<'w>(
        &mut self,
        world: &'w World,
        pool: &ThreadPool,
        batch_size: u32,
    ) -> ParQueryIter<'w, Q>;

    /// Look up a single entity.
    pub fn get<'w>(
        &mut self,
        world: &'w World,
        entity: Entity,
    ) -> Option<Q::Item<'w>>;

    /// Assert exactly one matching entity.
    pub fn single<'w>(
        &mut self,
        world: &'w World,
    ) -> Q::Item<'w>;

    /// Access set for dependency analysis.
    pub fn access(&self) -> &AccessSet;

    /// Update the archetype match cache if new
    /// archetypes have been created since the last
    /// check.
    pub fn update_archetypes(
        &mut self,
        world: &World,
    );
}
```

### Query Filters

```rust
/// Include only entities that have component T.
/// Does not fetch T's data.
pub struct With<T: Component>(PhantomData<T>);

/// Exclude entities that have component T.
pub struct Without<T: Component>(
    PhantomData<T>,
);

/// Fetch T if present; yields None otherwise.
/// Does not exclude entities missing T.
pub struct QueryOption<T: Component>(
    PhantomData<T>,
);

/// Include only entities whose component T was
/// mutated since this system last ran.
pub struct Changed<T: Component>(
    PhantomData<T>,
);

/// Include only entities where component T was
/// added since this system last ran.
pub struct Added<T: Component>(
    PhantomData<T>,
);

/// Include disabled enableable components.
pub struct WithDisabled<T: Component>(
    PhantomData<T>,
);

/// Include both enabled and disabled.
pub struct WithPresent<T: Component>(
    PhantomData<T>,
);

// Query tuple example:
// Query<(&Position, &mut Velocity), (
//     With<Enemy>,
//     Without<Dead>,
//     Changed<Health>,
// )>
```

### World Query Trait

```rust
/// Trait for types that can be fetched from a
/// World. Implemented for component references,
/// tuples, filters, Option, and Aspects.
///
/// Safety: implementors must correctly report
/// their AccessSet so the scheduler can prevent
/// conflicting parallel access.
pub unsafe trait WorldQuery {
    /// The item yielded per entity.
    type Item<'w>;

    /// Report read/write access to the AccessSet.
    fn update_access(
        access: &mut AccessSet,
        registry: &ComponentRegistry,
    );

    /// Test whether an archetype matches this
    /// query.
    fn matches_archetype(
        archetype: &Archetype,
        registry: &ComponentRegistry,
    ) -> bool;

    /// Fetch the item for a single entity row.
    /// Safety: caller must ensure row is valid
    /// and access is sound.
    unsafe fn fetch<'w>(
        archetype: &'w Archetype,
        row: u32,
        change_tick: u32,
        last_run: u32,
    ) -> Self::Item<'w>;
}

/// Marker trait for queries that only read.
/// Enables shared parallel access.
pub unsafe trait ReadOnlyWorldQuery:
    WorldQuery {}
```

### Aspect

```rust
/// Derive macro for component aspects. Groups
/// multiple component accesses into a single
/// type parameter.
///
/// ```
/// #[derive(Aspect)]
/// pub struct PhysicsAspect<'w> {
///     pub transform: &'w mut Transform,
///     pub velocity: &'w Velocity,
///     pub mass: &'w Mass,
///     pub rigid_body: &'w RigidBody,
/// }
///
/// // Nested aspect:
/// #[derive(Aspect)]
/// pub struct CharacterAspect<'w> {
///     pub physics: PhysicsAspect<'w>,
///     pub health: &'w Health,
/// }
/// ```
///
/// The derive macro implements WorldQuery for
/// the aspect struct, aggregating the access sets
/// of all fields.
```

### Resources

```rust
/// Type-safe singleton resource stored in World.
/// Accessed via Res<T> (shared) or ResMut<T>
/// (exclusive) system parameters.
pub struct ResourceMap {
    resources:
        HashMap<TypeId, ErasedResourceSlot>,
    non_send:
        HashMap<TypeId, ErasedResourceSlot>,
}

impl ResourceMap {
    pub fn new() -> Self;

    /// Insert a resource. If already present,
    /// replaces and returns the old value.
    pub fn insert<T: Resource>(
        &mut self,
        value: T,
    );

    /// Insert a non-send resource. Must only be
    /// accessed from the game loop thread.
    pub fn insert_non_send<T: Resource>(
        &mut self,
        value: T,
    );

    pub fn get<T: Resource>(
        &self,
    ) -> Option<&T>;

    pub fn get_mut<T: Resource>(
        &mut self,
    ) -> Option<&mut T>;

    pub fn contains<T: Resource>(&self) -> bool;
    pub fn remove<T: Resource>(
        &mut self,
    ) -> Option<T>;
}

/// Marker trait for world resources.
/// **Canonical definition.** `Resource`,
/// `Res<T>`, and `ResMut<T>` are defined here.
/// Other documents cross-reference this section.
pub trait Resource: Send + Sync + 'static {}

/// Shared resource access in a system parameter.
pub struct Res<'w, T: Resource> {
    value: &'w T,
    ticks: &'w ChangeTick,
}

/// Exclusive resource access in a system
/// parameter. Marks the resource as changed when
/// dereferenced mutably.
pub struct ResMut<'w, T: Resource> {
    value: &'w mut T,
    ticks: &'w mut ChangeTick,
    change_tick: u32,
}
```

### System

```rust
/// The core system trait. Systems are the only
/// way to execute logic in the ECS.
pub trait System: Send + Sync + 'static {
    /// Run the system with world access derived
    /// from SystemParam.
    fn run(
        &mut self,
        world: &World,
    );

    /// Report the access set for scheduling.
    fn access(&self) -> &AccessSet;

    /// System name for diagnostics.
    fn name(&self) -> &'static str;
}

/// Trait for types that can be used as system
/// parameters. Implementations exist for:
///   Query<Q, F>, Res<T>, ResMut<T>,
///   Commands, EventReader<E>,
///   EventWriter<E>, Local<T>
pub trait SystemParam {
    /// State that persists across system runs.
    type State: Send + Sync + 'static;

    /// The fetched item type for a given world
    /// borrow lifetime.
    type Item<'w>;

    /// Initialize param state.
    fn init_state(
        world: &World,
    ) -> Self::State;

    /// Report access to the AccessSet.
    fn update_access(
        state: &Self::State,
        access: &mut AccessSet,
    );

    /// Fetch the param for this run.
    /// Safety: caller (scheduler) must guarantee
    /// no conflicting access.
    unsafe fn fetch<'w>(
        state: &mut Self::State,
        world: &'w World,
    ) -> Self::Item<'w>;
}

/// Convert a function into a System. Supports
/// functions with up to 16 SystemParam arguments.
///
/// ```
/// fn movement_system(
///     mut query: Query<(
///         &mut Transform,
///         &Velocity,
///     )>,
///     time: Res<Time>,
/// ) {
///     for (mut transform, velocity) in
///         query.iter_mut()
///     {
///         transform.position +=
///             velocity.linear * time.delta();
///     }
/// }
/// ```
pub trait IntoSystem<Params> {
    type System: System;
    fn into_system(self) -> Self::System;
}

/// Systems that require exclusive &mut World
/// access. Run as full barriers in the schedule.
pub trait ExclusiveSystem:
    Send + Sync + 'static
{
    fn run(&mut self, world: &mut World);
    fn name(&self) -> &'static str;
}
```

### Command Buffer

```rust
/// Deferred structural change commands. Each
/// system receives its own Commands handle for
/// recording. Commands are flushed at sync points
/// in deterministic order.
pub struct Commands<'w> {
    buffer: &'w CommandBuffer,
    entities: &'w EntityAllocator,
}

impl<'w> Commands<'w> {
    /// Reserve an entity ID (immediately
    /// available for recording further commands
    /// against). The entity is not truly spawned
    /// until flush.
    pub fn spawn(
        &mut self,
        bundle: impl Bundle,
    ) -> EntityCommands;

    pub fn despawn(&mut self, entity: Entity);

    pub fn entity(
        &mut self,
        entity: Entity,
    ) -> EntityCommands;
}

/// Commands scoped to a specific entity.
pub struct EntityCommands<'a> {
    entity: Entity,
    commands: &'a mut Commands<'a>,
}

impl<'a> EntityCommands<'a> {
    pub fn insert(
        &mut self,
        bundle: impl Bundle,
    ) -> &mut Self;

    pub fn remove<T: Component>(
        &mut self,
    ) -> &mut Self;

    pub fn despawn(&mut self);

    /// Set this entity as child of parent.
    pub fn set_parent(
        &mut self,
        parent: Entity,
    ) -> &mut Self;
}

/// The underlying command buffer storage.
pub struct CommandBuffer {
    commands: Vec<Command>,
}

/// A single deferred command.
enum Command {
    Spawn {
        entity: Entity,
        components: ErasedBundle,
    },
    Despawn {
        entity: Entity,
    },
    Insert {
        entity: Entity,
        components: ErasedBundle,
    },
    Remove {
        entity: Entity,
        component_ids: SmallVec<[ComponentId; 4]>,
    },
    SetParent {
        child: Entity,
        parent: Entity,
    },
    Custom(Box<dyn FnOnce(&mut World) + Send>),
}

impl CommandBuffer {
    pub fn new() -> Self;

    /// Flush all recorded commands into the world
    /// in recording order. Triggers component
    /// hooks and observer dispatch.
    pub fn flush(&mut self, world: &mut World);

    /// Number of pending commands.
    pub fn len(&self) -> usize;

    /// Memory footprint in bytes.
    pub fn byte_size(&self) -> usize;
}

/// Thread-safe parallel command writer. Multiple
/// workers record into the same logical buffer.
/// Each command carries a sort key for
/// deterministic playback regardless of recording
/// order.
pub struct ParallelCommandWriter {
    /// Per-thread command segments. Merged and
    /// sorted by key at flush time.
    segments: Vec<CommandSegment>,
}

struct CommandSegment {
    commands: Vec<(u64, Command)>,
}

impl ParallelCommandWriter {
    pub fn new(worker_count: u32) -> Self;

    /// Get the writer for a specific worker
    /// thread. Each worker has exclusive access
    /// to its own segment.
    pub fn writer(
        &mut self,
        worker_index: u32,
    ) -> &mut CommandSegment;

    /// Merge all segments, sort by key, and flush.
    pub fn flush(&mut self, world: &mut World);
}
```

### Observer System

```rust
/// Built-in observer trigger events.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub enum ObserverTrigger {
    OnAdd(ComponentId),
    OnRemove(ComponentId),
    OnSet(ComponentId),
    OnTableCreate(ArchetypeId),
    OnTableEmpty(ArchetypeId),
    /// Custom user-defined event type.
    Custom(TypeId),
}

/// An observer registration.
pub struct Observer {
    /// What triggers this observer.
    trigger: ObserverTrigger,
    /// Query filter -- observer only fires for
    /// entities matching this query.
    filter: ErasedQueryFilter,
    /// The callback to execute.
    callback: Box<
        dyn FnMut(&mut World, Entity, &dyn Any)
            + Send
    >,
}

// Observer dispatch is defined in
// [events-plugins.md](events-plugins.md). The
// `ObserverRegistry`, trigger types, and callback
// dispatch are canonical there.

/// Custom entity event. Emitted at a specific
/// entity and optionally propagated along
/// relationship edges.
pub trait EntityEvent:
    Send + Sync + 'static
{
    /// Relationship along which to propagate.
    /// None = no propagation.
    fn propagation_relationship(
    ) -> Option<ComponentId> {
        None
    }
}

impl World {
    /// Emit a custom event targeted at an entity.
    /// The event is queued and dispatched at the
    /// next sync point.
    pub fn emit_event<E: EntityEvent>(
        &mut self,
        entity: Entity,
        event: E,
    );
}
```

### Schedule and Phases

```rust
/// Identifies a system within the schedule.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct SystemNodeId(pub(crate) u32);

/// Built-in execution phases. Systems are
/// assigned to exactly one phase. Phases execute
/// in order; systems within a phase execute in
/// parallel subject to dependency constraints.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, PartialOrd, Ord,
)]
pub enum Phase {
    /// Before gameplay logic. Input processing,
    /// time update.
    PreUpdate,
    /// Main gameplay logic.
    Update,
    /// After gameplay. Transform propagation,
    /// event cleanup.
    PostUpdate,
    /// Fixed-timestep simulation. Physics,
    /// networking.
    FixedUpdate,
    /// Before rendering. Visibility, culling,
    /// render proxy extraction.
    PreRender,
    /// Rendering. Command buffer building, draw
    /// submission.
    Render,
    /// User-defined custom phase with explicit
    /// ordering.
    Custom(u32),
}

/// A group of systems that share ordering
/// constraints and can be enabled/disabled as
/// a unit.
pub struct SystemGroup {
    name: &'static str,
    phase: Phase,
    systems: Vec<SystemNodeId>,
    children: Vec<SystemGroup>,
    enabled: bool,
}

/// Run criterion -- a predicate evaluated each
/// frame to gate system execution.
pub trait RunCriterion:
    Send + Sync + 'static
{
    fn should_run(&self, world: &World) -> bool;
}

/// Fixed timestep accumulator criterion.
pub struct FixedTimestep {
    pub step: f64,
    accumulator: f64,
}

impl RunCriterion for FixedTimestep {
    fn should_run(
        &self,
        world: &World,
    ) -> bool;
}

/// Run only when in a specific state.
pub struct InState<S: StateComponent> {
    pub state: S,
}

/// The schedule: owns all systems, groups,
/// phases, and the dependency graph.
pub struct Schedule {
    systems: Vec<ErasedSystem>,
    groups: Vec<SystemGroup>,
    graph: Option<SystemGraph>,
    ambiguities: Vec<Ambiguity>,
}

impl Schedule {
    pub fn new() -> Self;

    /// Add a system to a phase.
    pub fn add_system<S, Params>(
        &mut self,
        system: S,
        phase: Phase,
    ) -> SystemNodeId
    where
        S: IntoSystem<Params>;

    /// Add a system with run criteria.
    pub fn add_system_with_criteria<S, Params>(
        &mut self,
        system: S,
        phase: Phase,
        criteria: Vec<Box<dyn RunCriterion>>,
    ) -> SystemNodeId
    where
        S: IntoSystem<Params>;

    /// Add an exclusive system (full barrier).
    pub fn add_exclusive_system(
        &mut self,
        system: impl ExclusiveSystem,
        phase: Phase,
    ) -> SystemNodeId;

    /// Declare explicit ordering between two
    /// systems.
    pub fn order(
        &mut self,
        before: SystemNodeId,
        after: SystemNodeId,
    );

    /// Add a system group.
    pub fn add_group(
        &mut self,
        group: SystemGroup,
    );

    /// Build the game loop graph. Performs:
    /// 1. Dependency resolution from access sets
    /// 2. Topological sort
    /// 3. Cycle detection
    /// 4. Ambiguity detection
    /// 5. Phase node construction
    /// Returns a GameLoopGraph that can be compiled
    /// into a CompiledFrame for execution.
    pub fn build_game_loop(
        &self,
        world: &World,
    ) -> Result<GameLoopGraph, ScheduleError>;
}

pub struct Ambiguity {
    pub system_a: SystemNodeId,
    pub system_b: SystemNodeId,
    pub conflicting_components: Vec<ComponentId>,
}

pub enum ScheduleError {
    CycleDetected {
        cycle: Vec<SystemNodeId>,
    },
    /// A non-send system was assigned to a
    /// parallel phase without main-thread
    /// pinning.
    NonSendConflict {
        system: SystemNodeId,
    },
}
```

### World

```rust
/// Flags controlling which systems a world
/// instantiates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldFlag {
    Game,
    Editor,
    Server,
    Shadow,
    Custom(u32),
}

/// The top-level ECS container. Owns all entity,
/// component, and resource data. Each world is
/// independent -- entities in one world are
/// invisible to queries in another.
pub struct World {
    allocator: EntityAllocator,
    archetypes: ArchetypeStorage,
    sparse_sets: SparseSetMap,
    components: ComponentRegistry,
    resources: ResourceMap,
    observers: ObserverRegistry,
    change_detector: ChangeDetector,
    flags: Vec<WorldFlag>,
    id: WorldId,
}

/// **Canonical definition.** `WorldId` is defined
/// here. Other documents cross-reference this.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct WorldId(pub u32);

impl World {
    pub fn new(
        flags: Vec<WorldFlag>,
    ) -> Self;

    pub fn id(&self) -> WorldId;
    pub fn flags(&self) -> &[WorldFlag];

    // --- Entity operations ---

    /// Spawn an entity with a bundle of
    /// components.
    pub fn spawn(
        &mut self,
        bundle: impl Bundle,
    ) -> Entity;

    /// Spawn an entity with no components.
    pub fn spawn_empty(&mut self) -> Entity;

    /// Despawn an entity. If it has cleanup
    /// components, only non-cleanup components
    /// are removed. Otherwise the entity is fully
    /// destroyed.
    pub fn despawn(
        &mut self,
        entity: Entity,
    ) -> bool;

    /// Read-only entity access.
    pub fn entity(
        &self,
        entity: Entity,
    ) -> Option<EntityRef>;

    /// Mutable entity access.
    pub fn entity_mut(
        &mut self,
        entity: Entity,
    ) -> Option<EntityMut>;

    /// Check if an entity handle is still valid.
    pub fn is_alive(
        &self,
        entity: Entity,
    ) -> bool;

    /// Number of live entities.
    pub fn entity_count(&self) -> u32;

    // --- Query ---

    /// Create a cached query. Call once, reuse
    /// across frames.
    pub fn query<Q: WorldQuery>(
        &self,
    ) -> QueryState<Q>;

    // --- Resources ---

    pub fn insert_resource<T: Resource>(
        &mut self,
        value: T,
    );

    pub fn resource<T: Resource>(
        &self,
    ) -> Option<Res<T>>;

    pub fn resource_mut<T: Resource>(
        &mut self,
    ) -> Option<ResMut<T>>;

    pub fn contains_resource<T: Resource>(
        &self,
    ) -> bool;

    // --- Components ---

    pub fn component_registry(
        &self,
    ) -> &ComponentRegistry;

    pub fn component_registry_mut(
        &mut self,
    ) -> &mut ComponentRegistry;

    // --- Change detection ---

    pub fn change_tick(&self) -> u32;
    pub fn increment_change_tick(&self) -> u32;

    // --- Archetypes ---

    pub fn archetype_count(&self) -> u32;
}
```

### Entity Migration

```rust
/// Transfer entities between worlds. Used for
/// zone transitions in open worlds.
pub struct EntityMigration;

impl EntityMigration {
    /// Migrate a single entity with all its
    /// components from `source` to `target`.
    /// Entity IDs are remapped to avoid
    /// collisions in the target world. Returns
    /// the new Entity handle in the target world.
    ///
    /// Errors if the target world is missing a
    /// component type registration for any of the
    /// entity's components.
    pub fn migrate(
        source: &mut World,
        target: &mut World,
        entity: Entity,
    ) -> Result<Entity, MigrationError>;

    /// Bulk migration for multiple entities.
    /// Returns a mapping from old to new entity
    /// handles for relationship remapping.
    pub fn migrate_batch(
        source: &mut World,
        target: &mut World,
        entities: &[Entity],
    ) -> Result<
        HashMap<Entity, Entity>,
        MigrationError,
    >;
}

pub enum MigrationError {
    /// Entity does not exist in source world.
    EntityNotFound(Entity),
    /// Target world missing a component type
    /// registration.
    MissingComponentType {
        entity: Entity,
        component_name: String,
    },
}
```

### Prefabs

```rust
/// Marker tag for prefab template entities.
pub struct Prefab;

/// Relationship: "this entity is an instance of
/// that prefab." Inherits components from the
/// prefab. Unwritten components fall through to
/// the prefab's values. Writing to an inherited
/// component creates a local override
/// (copy-on-write).
pub struct IsA;

impl World {
    /// Instantiate a prefab. Creates a new entity
    /// with an IsA relationship to the prefab.
    /// Inherited components are shared (not
    /// copied) until overridden.
    pub fn instantiate_prefab(
        &mut self,
        prefab: Entity,
    ) -> Entity;

    /// Instantiate a prefab with its child
    /// hierarchy. All children are recursively
    /// instantiated.
    pub fn instantiate_prefab_hierarchy(
        &mut self,
        prefab: Entity,
    ) -> Entity;
}

/// Named access to a specific child in an
/// instantiated prefab hierarchy.
pub struct SlotRef {
    pub name: &'static str,
}
```

### State Machine

```rust
/// Trait for state components. Each state type
/// is a component. Transitioning replaces one
/// state component with another.
pub trait StateComponent:
    Component + Clone + PartialEq + Eq
{
}

/// Resource that requests a state transition.
/// Consumed at the next sync point.
pub struct NextState<S: StateComponent> {
    pub state: Option<S>,
}

/// Observer events for state transitions.
pub struct OnEnter<S: StateComponent>(
    PhantomData<S>,
);
pub struct OnExit<S: StateComponent>(
    PhantomData<S>,
);
pub struct OnTransition<S: StateComponent> {
    pub from: S,
    pub to: S,
}

/// Run criterion: system runs only when the
/// world is in the given state.
pub fn in_state<S: StateComponent>(
    state: S,
) -> impl RunCriterion;

/// Sub-state that is active only when its parent
/// state matches.
pub trait SubState: StateComponent {
    type Parent: StateComponent;
    fn should_exist(
        parent: &Self::Parent,
    ) -> bool;
}

/// Computed state derived from multiple state
/// sources.
pub trait ComputedState: StateComponent {
    type Sources;
    fn compute(sources: &Self::Sources) -> Self;
}
```

## Data Flow

### Frame Execution Sequence

The frame is driven by a `CompiledFrame` produced from the `GameLoopGraph`. The compiled frame is
reused across frames until the system set changes.

```mermaid
sequenceDiagram
    participant ML as Main Loop
    participant RE as IoReactor
    participant CF as CompiledFrame
    participant TP as ThreadPool
    participant W as Workers
    participant CB as CommandBuffers
    participant OB as Observers

    loop Every Frame
        ML->>RE: poll()
        RE->>TP: re-enqueue woken tasks

        ML->>CF: execute(world, pool, reactor)

        loop Each Phase in TaskGraph
            CF->>TP: dispatch phase tasks
            TP->>W: work-steal ready systems
            W->>W: execute systems in parallel
            Note over W: Systems read/write components
            Note over W: Systems record commands
            W-->>TP: all systems in phase complete

            CF->>CB: flush_all(world)
            Note over CB: Apply structural changes
            CB->>OB: trigger observers
            OB->>OB: fire matching callbacks
            Note over OB: May record more commands
            OB->>CB: flush observer commands
        end

        ML->>RE: poll()
    end
```

### Entity Spawn via Command Buffer

```mermaid
sequenceDiagram
    participant SYS as System
    participant CB as CommandBuffer
    participant EA as EntityAllocator
    participant AG as ArchetypeGraph
    participant AT as Archetype Table
    participant HK as Component Hooks
    participant CD as ChangeDetector

    SYS->>CB: spawn(bundle)
    Note over CB: Deferred until sync point

    Note over CB: --- Sync Point ---
    CB->>EA: allocate()
    EA-->>CB: Entity(index=42, gen=7)

    CB->>AG: get_or_create(component_set)
    AG-->>CB: ArchetypeId(15)

    CB->>AT: insert_row(entity, components)
    AT->>CD: mark_added(tick)
    AT-->>CB: row=0

    CB->>EA: set_meta(entity, arch=15, row=0)

    loop For each component in bundle
        CB->>HK: on_add(entity, component_ref)
    end
```

### Query Execution

```mermaid
sequenceDiagram
    participant SYS as System
    participant QS as QueryState
    participant QC as QueryCache
    participant AS as ArchetypeStorage
    participant CD as ChangeDetector
    participant TP as ThreadPool

    SYS->>QS: iter(world)
    QS->>QC: check_cache(archetype_count)
    alt Cache stale
        QC->>AS: match_archetypes(filters)
        AS-->>QC: matched archetype list
        QC->>QC: update cache
    end
    QC-->>QS: matched archetypes

    alt Sequential iteration
        loop Each matched archetype
            QS->>AS: get_column(component_id)
            QS->>CD: check_change_tick(chunk)
            Note over QS: Skip unchanged chunks
            QS-->>SYS: yield component refs
        end
    end

    alt Parallel iteration
        SYS->>QS: par_iter(world, pool)
        QS->>TP: partition chunks across workers
        TP->>TP: each worker iterates its chunks
        TP-->>SYS: all chunks processed
    end
```

### Schedule Build and Execution

The schedule build process runs at startup and whenever the active system set changes. The compiled
frame is reused across frames.

```rust
// --- Schedule Build (startup or system change) ---

let mut schedule = Schedule::new();

// Register systems into phases
schedule.add_system(
    input_system, Phase::PreUpdate,
);
schedule.add_system(
    movement_system, Phase::Update,
);
schedule.add_system(
    physics_step, Phase::FixedUpdate,
);
schedule.add_system(
    transform_propagation, Phase::PostUpdate,
);
schedule.add_system(
    render_extract, Phase::PreRender,
);

// Explicit ordering within a phase
schedule.order(movement_id, collision_id);

// Build game loop graph, then compile once
let graph = schedule.build_game_loop(&world)?;
let frame = graph.compile(&world, &pool)?;

// --- Frame Execution (reuses CompiledFrame) ---

loop {
    reactor.poll();
    frame.execute(
        &mut world, &pool, &mut reactor,
    );
}
```

### Archetype Migration on Component Add

When a system adds a component to an entity, the entity migrates from its current archetype to a new
archetype that includes the added component.

```rust
// Entity currently in Archetype {Position, Velocity}
// System adds Health component via Commands

// At sync point (command flush):
// 1. Resolve target archetype via graph edge:
//    ArchetypeGraph::traverse_add(
//        arch_pos_vel, health_id
//    )
//    Returns Archetype {Position, Velocity, Health}
//    Edge is cached for O(1) future lookups.

// 2. Move entity data:
//    - Copy Position from old archetype row
//      to new archetype row
//    - Copy Velocity from old archetype row
//      to new archetype row
//    - Write Health value to new archetype row

// 3. Swap-remove old row to maintain dense packing.
//    The entity that was in the last row of the
//    old archetype is moved into the vacated slot.

// 4. Update EntityMeta with new archetype_id
//    and archetype_row.

// 5. Fire on_add hook for Health.
```

### Change Detection Flow

```rust
// System A writes to Transform:
for mut transform in query.iter_mut() {
    // Mut<T>::deref_mut() marks the containing
    // chunk's ChangeTick as last_changed =
    // current_tick
    transform.position += delta;
}

// System B reads only changed Transforms:
for transform in query_changed.iter() {
    // Changed<T> filter checks:
    //   chunk.last_changed > system_b.last_run
    //   && chunk.last_changed <= current_tick
    //
    // Skips entire chunks where no Transform was
    // mutated since System B last ran.
    spatial_index.update(entity, &transform);
}
```

### Observer Dispatch During Command Flush

```rust
// During CommandBuffer::flush():
// 1. Apply all commands in order
// 2. Collect pending events:
//    - Spawn entity -> OnAdd for each component
//    - Insert component -> OnAdd
//    - Remove component -> OnRemove
//    - Set component value -> OnSet

// 3. Dispatch to ObserverRegistry:
//    For each event:
//      For each observer matching the event type:
//        If entity matches the observer's query
//          filter:
//          -> Call observer callback

// 4. Observer callbacks may record additional
//    commands via a nested CommandBuffer.
//    These are flushed recursively (with a depth
//    limit of 16 to prevent infinite loops).
```

## Platform Considerations

### Chunk Size by Platform

| Platform | Default Chunk | L1 Cache | Notes |
|----------|-------------|----------|-------|
| Mobile (iOS/Android) | 8 KiB | 32-64 KiB | Smaller L1; conservative sizing |
| Switch handheld | 8 KiB | 32 KiB | Thermal throttling reduces cache effectiveness |
| Switch docked | 16 KiB | 32 KiB | Higher clocks allow larger chunks |
| Desktop | 16 KiB | 32-64 KiB | Default; configurable up to 64 KiB |
| High-end PC | 16-64 KiB | 32-64 KiB | Larger chunks for wider SIMD |

### Parallel Iteration Worker Counts

| Platform | Workers | Partitioning |
|----------|---------|-------------|
| Mobile (4P + 4E) | 2-4 | Chunk-level only |
| Switch handheld | 3 | Chunk-level only |
| Switch docked | 3 | Chunk-level only |
| Desktop (8P + 8E) | 7 (perf cores - 1) | Archetype-level + chunk-level |
| High-end (16P + 16E) | 15 | Archetype-level + chunk-level |

### Concurrent World Limits

| Platform | Max Concurrent Worlds |
|----------|----------------------|
| Mobile | 2 (game + staging) |
| Switch | 3 |
| Desktop | 8 (configurable) |
| High-end | Unlimited |

### Buffer Component Inline Thresholds

| Platform | Inline Threshold | Spill Cap |
|----------|-----------------|-----------|
| Mobile | 128 bytes | 4 KiB before heap spill |
| Switch | 256 bytes | 8 KiB |
| Desktop | 512 bytes | No cap |

### Thread Pool Integration

The ECS scheduler no longer directly builds a `TaskGraph`. Instead, it populates a `SystemPhase`
within the `GameLoopGraph` (defined in [platform/threading.md](../platform/threading.md)). The game
loop graph is the unified execution model for the entire frame.

**`build_game_loop()` replaces `build_frame_graph()`.** The `Schedule` produces a `GameLoopGraph`
instead of a raw `TaskGraph`. Each ECS phase becomes a `PhaseNode` in the graph. The `GameLoopGraph`
is compiled once (or when systems change) and the resulting `CompiledFrame` is reused across frames.

```rust
impl Schedule {
    /// Build the game loop graph from the current
    /// system set. Each phase becomes a PhaseNode
    /// containing its systems. Dependencies between
    /// phases become graph edges.
    pub fn build_game_loop(
        &self,
        world: &World,
    ) -> Result<GameLoopGraph, ScheduleError>;
}
```

**Execution flow.** The main loop compiles the graph once, then executes the compiled frame each
tick:

```rust
// Build once (or when systems change)
let graph = schedule.build_game_loop(&world)?;
let frame = graph.compile(&world, &pool)?;

// Per-frame execution
loop {
    reactor.poll();
    frame.execute(&mut world, &pool, &mut reactor);
}
```

```mermaid
sequenceDiagram
    participant ML as Main Loop
    participant SC as Schedule
    participant GLG as GameLoopGraph
    participant CF as CompiledFrame
    participant TP as ThreadPool

    ML->>SC: build_game_loop(world)
    SC-->>ML: GameLoopGraph

    ML->>GLG: compile(world, pool)
    GLG-->>ML: CompiledFrame

    loop Every Frame
        ML->>CF: execute(world, pool, reactor)
        CF->>TP: dispatch task graph
        TP->>TP: work-steal systems in parallel
        Note over TP: Sync barriers flush commands
        TP-->>CF: frame complete
    end
```

**Phase mapping.** Each ECS phase populates a `SystemPhase` node in the game loop graph. The
schedule's topological ordering and access-set analysis are preserved within each phase node.

| ECS Phase | GameLoopGraph Node | Scheduling |
|-----------|-------------------|------------|
| PreUpdate | `PhaseNode::Systems` | Parallel within phase |
| Update | `PhaseNode::Systems` | Parallel within phase |
| FixedUpdate | `PhaseNode::Systems` | Fixed timestep gated |
| PostUpdate | `PhaseNode::Systems` | Parallel within phase |
| PreRender | `PhaseNode::Systems` | Parallel within phase |
| Render | `PhaseNode::RenderGraph` | Render pass ordering |
| Custom(n) | `PhaseNode::Systems` | User-defined |

**Non-send systems** are pinned to the game loop thread by the scheduler. They run at designated
points in the phase, never dispatched to worker threads.

**Parallel query iteration** (`par_iter`) still uses `ThreadPool::scope` internally. The outer
scheduling is driven by the compiled game loop graph, but within a system, `par_iter` splits work
across workers via scoped fork-join. Query borrows from the calling system are valid across worker
tasks without `'static` or `Arc` overhead.

### Alignment and SIMD

All chunk base addresses are 64-byte aligned (cache-line boundary). Component arrays within a chunk
are aligned to the component type's natural alignment. Numeric component types (`f32`, `Vec3`,
`Mat4`) with proper alignment can be processed with SIMD intrinsics without additional alignment
adjustments.

### Proposed Dependencies

| Crate             |
|-------------------|
| `fixedbitset`     |
| `smallvec`        |
| `crossbeam-utils` |
| `glam`            |

1. **`fixedbitset`** — Compact bitsets for AccessSet, enable bits
   - **Justification:** Efficient set operations for component access tracking
2. **`smallvec`** — Inline-allocated small vectors
   - **Justification:** Matched archetype lists, hook arrays, command component lists
3. **`crossbeam-utils`** — `CachePadded` for atomics
   - **Justification:** Prevents false sharing on change tick counters
4. **`glam`** — Math types (Vec3, Quat, Mat4)
   - **Justification:** SIMD-accelerated math implied by Transform components and spatial operations

## Safety Invariants

The following safety-critical invariants must be enforced by the implementation:

### Column Access (Critical)

`Column::get_unchecked` returns `*const u8`. Callers must cast to the correct type. Add
`debug_assert!` comparing `TypeId::of::<T>()` against the column's `ComponentDescriptor::type_id`.
The safe public API (`Column::get<T>`) performs this check unconditionally.

### WorldQuery::fetch Contract (High)

`unsafe fn fetch` requires:

1. Row index is within `archetype.len()`.
2. No structural changes between `matches_archetype` and `fetch`.
3. The `AccessSet` has been validated by the scheduler.

Aliased `&mut` access through concurrent fetch calls is undefined behavior.

### Bundle Write Ordering (High)

`Bundle::write_components` writes fields in the order returned by `component_ids()`. The derive
macro must guarantee field order matches `component_ids()` order. Add `debug_assert!` verifying each
component's offset and layout against `ComponentDescriptor`.

### RelationPair Entity Generation (High)

`RelationPair` encodes only `Entity.index` (32 bits), not the generation. Relationship queries must
validate the target entity's generation against the `EntityAllocator` before returning results.
Stale relationships to despawned-and-reused entities must be detected and cleaned up.

### ComponentDescriptor::drop_fn (High)

`drop_fn: Option<unsafe fn(*mut u8)>` must match the type at the pointer. `register_dynamic` callers
must provide a `TypeId` witness. Add `debug_assert!` comparing `TypeId` when invoking `drop_fn`.

### Enableable Components (Critical)

`set_enabled` claims atomic safety but `FixedBitSet` is not atomic. Implementation must use
`Vec<AtomicU64>` with `fetch_or`/`fetch_and` (Release/Acquire ordering) for bit toggles, or require
`&mut self` and defer through command buffers.

### ParallelCommandWriter (High)

`writer(worker_index)` returns `&mut CommandSegment`. Two threads with the same `worker_index`
create aliased `&mut` -- undefined behavior. Use a `!Copy` `WorkerToken` issued once per worker to
enforce unique access, or use `thread_local!` indexing.

### Observer Callbacks (Medium)

Observer dispatch during command buffer flush is sequential (single-threaded at sync points).
Closures need `Send` but not `Sync`. Document this invariant.

## Performance Notes

### Archetype Column Lookup

Archetype columns are stored in `HashMap<ComponentId, Column>`. For hot-path query iteration, this
hash lookup occurs per-archetype per query. Implementation should use a flat array indexed by
per-archetype column index (assigned at archetype creation), with the `HashMap` only for dynamic
lookups. This avoids hash overhead in the inner loop.

## Test Plan

### Unit Tests

| Test                                  | Req       |
|---------------------------------------|-----------|
| `test_entity_allocate_deallocate`     | R-1.1.11  |
| `test_entity_1m_alloc_dealloc`        | R-1.1.11a |
| `test_archetype_soa_layout`           | R-1.1.1   |
| `test_archetype_chunk_alignment`      | R-1.1.1a  |
| `test_sparse_no_migration`            | R-1.1.2   |
| `test_sparse_o1_lookup`               | R-1.1.2a  |
| `test_archetype_graph_edge_cache`     | R-1.1.3   |
| `test_archetype_graph_10k_archetypes` | R-1.1.3a  |
| `test_component_static_registration`  | R-1.1.4   |
| `test_component_dynamic_registration` | R-1.1.4   |
| `test_tag_zero_memory`                | R-1.1.5   |
| `test_shared_component_one_per_chunk` | R-1.1.6   |
| `test_buffer_inline_and_spill`        | R-1.1.7   |
| `test_enableable_toggle`              | R-1.1.8   |
| `test_enableable_parallel_toggle`     | R-1.1.8   |
| `test_hooks_fire_correctly`           | R-1.1.9   |
| `test_hook_limit_16`                  | R-1.1.9a  |
| `test_bundle_atomic_insert`           | R-1.1.10  |
| `test_required_component_auto_add`    | R-1.1.10  |
| `test_cleanup_component_persist`      | R-1.1.12  |
| `test_entity_name_path_lookup`        | R-1.1.13  |
| `test_relationship_pair_encoding`     | R-1.1.14  |
| `test_exclusive_relationship`         | R-1.1.15  |
| `test_symmetric_relationship`         | R-1.1.15  |
| `test_childof_cascade_delete`         | R-1.1.16  |
| `test_childof_cycle_rejection`        | R-1.1.16a |
| `test_childof_256_depth`              | R-1.1.16a |
| `test_query_all_filters`              | R-1.1.17  |
| `test_query_cache_zero_overhead`      | R-1.1.17a |
| `test_query_cache_incremental`        | R-1.1.17a |
| `test_query_sort_stable`              | R-1.1.18  |
| `test_query_variable_pattern`         | R-1.1.19  |
| `test_change_detection_chunk`         | R-1.1.22  |
| `test_change_tick_8_bytes`            | R-1.1.22a |
| `test_resource_res_resmut`            | R-1.1.23  |
| `test_non_send_main_thread`           | R-1.1.24  |
| `test_schedule_dependency_resolution` | R-1.1.25  |
| `test_schedule_cycle_detection`       | R-1.1.25  |
| `test_schedule_phases_order`          | R-1.1.26  |
| `test_run_criteria_gate`              | R-1.1.27  |
| `test_ambiguity_detection`            | R-1.1.28  |
| `test_exclusive_system_barrier`       | R-1.1.29  |
| `test_observer_on_add`                | R-1.1.30  |
| `test_custom_event_propagation`       | R-1.1.31  |
| `test_command_buffer_deterministic`   | R-1.1.32  |
| `test_parallel_command_writer`        | R-1.1.33  |
| `test_multiple_worlds_isolation`      | R-1.1.34  |
| `test_entity_migration`               | R-1.1.35  |
| `test_migration_missing_type_error`   | R-1.1.35a |
| `test_prefab_inheritance`             | R-1.1.36  |
| `test_nested_prefab`                  | R-1.1.37  |
| `test_state_transition_observers`     | R-1.1.38  |

1. **`test_entity_allocate_deallocate`** — Allocate entity, deallocate, verify generation
   increments. Reallocate at same index, verify old handle is stale.
2. **`test_entity_1m_alloc_dealloc`** — Allocate and deallocate 1M entities. Verify O(1)
   per-operation cost (under 100 ns amortized).
3. **`test_archetype_soa_layout`** — Spawn 1000 entities with (Position, Velocity). Verify
   Position[] and Velocity[] are contiguous in memory via pointer arithmetic.
4. **`test_archetype_chunk_alignment`** — Verify chunk base addresses are 64-byte aligned.
5. **`test_sparse_no_migration`** — Add and remove a `#[sparse]` component 10,000 times. Assert
   archetype ID never changes.
6. **`test_sparse_o1_lookup`** — Lookup sparse component on 100,000 entities. Verify under 200 ns
   per operation.
7. **`test_archetype_graph_edge_cache`** — Spawn 100,000 entities with same components. Verify
   per-entity archetype resolution does not degrade with archetype count.
8. **`test_archetype_graph_10k_archetypes`** — Create 10,000 distinct archetypes. Verify edge lookup
   remains O(1).
9. **`test_component_static_registration`** — Register component via derive macro. Verify zero-cost
   access via TypeId lookup.
10. **`test_component_dynamic_registration`** — Register component at runtime. Attach to entity,
    query, verify correct data.
11. **`test_tag_zero_memory`** — Add zero-size tag to 100,000 entities. Assert zero bytes per entity
    for tag column. Verify `With<Tag>` query works.
12. **`test_shared_component_one_per_chunk`** — Assign same shared value to 10,000 entities. Assert
    stored once. Modify one entity, assert migration.
13. **`test_buffer_inline_and_spill`** — Create buffer component, append past inline threshold.
    Verify spill and return to inline.
14. **`test_enableable_toggle`** — Toggle enableable component. Verify default query excludes
    disabled. Verify `WithDisabled<T>` includes it.
15. **`test_enableable_parallel_toggle`** — Toggle from 8 threads concurrently. Verify no data races
    (run under ThreadSanitizer).
16. **`test_hooks_fire_correctly`** — Register on_add, on_remove, on_set hooks. Perform operations.
    Assert each fires with correct args.
17. **`test_hook_limit_16`** — Register 17 hooks. Verify error on 17th.
18. **`test_bundle_atomic_insert`** — Insert 4-component bundle. Verify all present in single
    archetype transition.
19. **`test_required_component_auto_add`** — Insert Collider (requires CollisionLayers). Verify
    companion auto-added.
20. **`test_cleanup_component_persist`** — Despawn entity with cleanup component. Assert entity
    alive with only cleanup components.
21. **`test_entity_name_path_lookup`** — Build 3-level hierarchy with names. Look up leaf by path.
    Verify O(log n) scaling at 100,000 entities.
22. **`test_relationship_pair_encoding`** — Add (Likes, Apple) and (Likes, Banana). Query (Likes,
    *). Verify both returned.
23. **`test_exclusive_relationship`** — Add second target to exclusive relationship. Verify first
    removed.
24. **`test_symmetric_relationship`** — Add A->B symmetric. Verify B->A auto-added.
25. **`test_childof_cascade_delete`** — Build 4-level hierarchy. Delete root. Assert all descendants
    destroyed.
26. **`test_childof_cycle_rejection`** — Attempt to create a cycle. Verify error returned.
27. **`test_childof_256_depth`** — Build 256-level hierarchy. Verify traversal completes without
    stack overflow.
28. **`test_query_all_filters`** — Construct query with With, Without, Option, Changed, Added.
    Verify correct entity inclusion/exclusion.
29. **`test_query_cache_zero_overhead`** — Run cached query 1,000 times. Verify zero additional
    archetype matching after first.
30. **`test_query_cache_incremental`** — Add new archetype after cache built. Verify cache
    incrementally updates.
31. **`test_query_sort_stable`** — Sort 1,000 entities by value. Verify ascending. Modify 10,
    re-sort. Verify stability.
32. **`test_query_variable_pattern`** — Create parent-child pairs with Boss parents. Query children
    of bosses. Verify correct results.
33. **`test_change_detection_chunk`** — Mutate one entity in chunk of 100. Verify chunk marked
    changed. Next tick without mutations: no results.
34. **`test_change_tick_8_bytes`** — Verify change detection metadata is 8 bytes per component type
    per chunk.
35. **`test_resource_res_resmut`** — Insert resource. Read via Res, write via ResMut. Verify
    scheduler orders correctly.
36. **`test_non_send_game_loop_thread`** — Register non-send resource. Verify system runs on game
    loop thread.
37. **`test_schedule_dependency_resolution`** — Register systems with known deps. Build schedule.
    Verify topological order respects deps.
38. **`test_schedule_cycle_detection`** — Register cyclic systems. Verify error at build time.
39. **`test_schedule_phases_order`** — Register systems in Update and FixedUpdate. Verify execution
    order. Disable group, verify no execution.
40. **`test_run_criteria_gate`** — Attach boolean criterion. Toggle and verify system runs only when
    met. AND compose two criteria.
41. **`test_ambiguity_detection`** — Register read-A and write-A systems without ordering. Verify
    warning.
42. **`test_exclusive_system_barrier`** — Register exclusive system. Verify no concurrent execution.
43. **`test_observer_on_add`** — Register OnAdd observer with query filter. Add component to
    matching and non-matching entities. Verify fires only for matching.
44. **`test_custom_event_propagation`** — Emit DamageEvent at child. Verify observer fires on child,
    then parent.
45. **`test_command_buffer_deterministic`** — Record commands from two systems. Flush. Verify
    deterministic order across repeated runs.
46. **`test_parallel_command_writer`** — Record 100,000 commands from 8 threads. Flush. Verify
    sort-key order. 100 iterations, identical results.
47. **`test_multiple_worlds_isolation`** — Create two worlds. Spawn entities in each. Verify queries
    in one world do not see the other's entities.
48. **`test_entity_migration`** — Create entity with 5 components and relationships. Migrate to new
    world. Verify all data intact, no collisions.
49. **`test_migration_missing_type_error`** — Migrate entity with unregistered component type.
    Verify diagnostic error with type name.
50. **`test_prefab_inheritance`** — Create prefab with 3 components. Instantiate 100 instances.
    Verify sharing. Override one, verify copy-on-write.
51. **`test_nested_prefab`** — Create nested prefab. Instantiate 10 outer instances. Modify inner.
    Verify propagation.
52. **`test_state_transition_observers`** — Transition from Menu to Playing. Verify OnExit(Menu) and
    OnEnter(Playing) fire. Verify in_state(Playing) criterion works.

### Integration Tests

| Test                              | Req              |
|-----------------------------------|------------------|
| `test_full_frame_cycle`           | R-1.1.25-29      |
| `test_parallel_iteration_scaling` | R-1.1.20         |
| `test_cascade_delete_100k`        | R-1.1.16a        |
| `test_bulk_migration_500`         | R-1.1.35a        |
| `test_schedule_500_systems`       | R-1.1.25a        |
| `test_observer_dispatch_1000`     | R-1.1.30a        |
| `test_command_buffer_100k_flush`  | R-1.1.32a        |
| `test_mixed_storage_query`        | R-1.1.1, R-1.1.2 |

1. **`test_full_frame_cycle`** — Run a complete frame: schedule build, system execution across all
   phases, command flush, observer dispatch. Verify correct world state.
2. **`test_parallel_iteration_scaling`** — Iterate 1M entities on 1, 2, 4, 8 cores. Verify >= 3.5x
   speedup on 4 cores. Run under ThreadSanitizer.
3. **`test_cascade_delete_100k`** — Cascade-delete a 100,000-entity subtree. Verify completion
   within 10 ms.
4. **`test_bulk_migration_500`** — Migrate 500 entities simultaneously. Verify no data loss, no ID
   collisions.
5. **`test_schedule_500_systems`** — Build schedule with 500 systems. Verify construction under 50
   ms. Verify no rebuild on frames without system changes.
6. **`test_observer_dispatch_1000`** — Register 1,000 observers. Fire 100 events matching 10 each.
   Verify O(e*m) scaling.
7. **`test_command_buffer_100k_flush`** — Record 100,000 commands. Verify flush under 1 ms. Verify
   per-system buffer under 64 KiB typical.
8. **`test_mixed_storage_query`** — Query spanning both table and sparse components. Verify correct
   results across both storage modes.

### Benchmarks

| Benchmark | Target | Source |
|-----------|--------|--------|
| Archetype iteration (1M entities, 3 components, 1 core) | >= 500M components/sec | R-1.1.1a |
| Entity allocate + deallocate | < 100 ns each | R-1.1.11a |
| Sparse component lookup | O(1), < 200 ns | R-1.1.2a |
| Archetype graph edge traversal | O(1) amortized | R-1.1.3 |
| Component hook dispatch overhead | < 50 ns per invocation | R-1.1.9a |
| Query cache hit (no new archetypes) | 0 ns matching overhead | R-1.1.17a |
| Parallel iteration speedup (4 cores) | >= 3.5x | R-1.1.20 |
| Schedule build (500 systems) | < 50 ms | R-1.1.25a |
| Command buffer flush (100K commands) | < 1 ms | R-1.1.32a |
| Entity migration | < 10 us per entity | R-1.1.35a |
| Cascade delete (100K subtree) | < 10 ms | R-1.1.16a |

## Design Q & A

**Q1. What is the biggest constraint limiting this design?** What would happen if we lifted that
constraint? What is the best possible solution imaginable without those constraints? What is the
impact of removing them?

The "100% ECS-based" constraint (no separate data stores) is the single largest limiting factor.
Physics engines like PhysX and Havok maintain independent broadphase and solver structures that can
be highly optimized in isolation. Lifting this constraint would allow dedicated physics memory
layouts (SIMD-friendly solver bodies), separate spatial indices tuned per subsystem, and
vendor-optimized collision pipelines. The impact of removing it, however, would be data duplication
across subsystems, synchronization bugs between parallel stores, and higher memory footprint at MMO
scale. The ECS-only approach trades peak per-subsystem performance for global consistency and
reduced complexity.

**Q2. How can this design be improved?** Where is it weak? What potential issues will arise? What
trade-offs are we making?

Chunk-granularity change detection (R-1.1.22) introduces false positives: modifying one entity marks
the entire chunk dirty. For components with mixed hot/cold access patterns this wastes work in
downstream systems like network delta compression and spatial index updates. The archetype explosion
problem is another weakness: games with many unique component combinations (e.g., hundreds of status
effects) can create thousands of archetypes, degrading cache locality and query cache invalidation
time. The prefab inheritance model (F-1.1.36) with copy-on-write semantics adds complexity to the
query path since inherited components must be resolved through IsA chains. Adding per-entity change
bitmasks, archetype coalescing heuristics, and lazy prefab resolution would improve these areas.

**Q3. Is there a better approach?** If we are not taking it, why not?

A bitset-based ECS (like the `specs` or `shipyard` model) avoids archetype fragmentation entirely by
storing each component type in its own array indexed by entity ID. This eliminates migration costs
and archetype explosion. We chose the archetype model instead because it provides superior cache
locality during iteration: all components for matching entities are contiguous in memory, which
matters for throughput targets like 500M components/sec (R-1.1.1a). The archetype graph with O(1)
edge caching (F-1.1.3) mitigates migration costs, and sparse-set fallback (F-1.1.2) handles
high-churn components. The hybrid approach captures the best of both models.

**Q4. Does this design solve all customer problems?** Are there missing features, requirements, or
user stories? What are they? How would adding them improve the engine? What kinds of games does it
enable?

The design covers the core ECS needs for most game genres but is light on editor-side undo/redo
integration. User stories US-1.1.49 and US-1.1.50 reference prefab editing, but there is no explicit
undo stack that snapshots component state before mutations. This gap affects designers (P-5) editing
entity templates. Additionally, there is no streaming archetype paging feature for worlds exceeding
available RAM, which would be needed for truly massive persistent MMO worlds beyond the 4M entity
limit (R-1.1.11a). Adding entity streaming and undo integration would enable open-world MMOs and
improve the editor workflow for all game genres.

**Q5. Is this design cohesive with the overall engine?** Does it fit? Does it differ from other
modules, and why? How could we make it more cohesive? How can we improve it to meet engine goals?

The ECS design is the backbone and other modules (scene hierarchy F-1.2, spatial index F-1.9, events
F-1.5) depend heavily on it. It aligns well with the engine constraints: static dispatch via
generics, no singletons, composition over inheritance. One cohesion gap is between the ECS command
buffer model (F-1.1.32) and the async I/O reactor (F-1.8): command buffers are synchronous and
frame-scoped while I/O completions arrive asynchronously. Bridging these requires explicit
integration where I/O completions are batched into command buffers at poll points. Making the
command buffer API aware of async completion tokens would improve cohesion between the ECS and async
I/O subsystems.

## Open Questions

1. **Chunk capacity calculation** -- The chunk capacity depends on the sum of component sizes in the
   archetype and the target chunk byte size. Archetypes with many large components may have very
   small chunk capacities (e.g., 2-3 entities per chunk). Should there be a minimum chunk capacity
   floor (e.g., 8 entities)?

2. **Sparse set paging strategy** -- The sparse array in `SparseSet` uses paged allocation to avoid
   allocating a slot for every possible entity index. Page size (e.g., 4096 entries) trades memory
   for lookup speed. Optimal page size depends on entity density patterns.

3. **Observer recursion depth** -- Observers can record commands that trigger further observers when
   flushed. A depth limit of 16 prevents infinite loops, but some valid cascade patterns (e.g., deep
   hierarchy events) may need more depth. Should the limit be configurable?

4. **Query variable compilation** -- Pattern-matching queries with variables (`$parent`, `$target`)
   require a query planner to determine join order. The planner complexity and query planning cost
   at cache-build time need benchmarking to ensure they stay within the 50 ms schedule-build target.

5. **Shared component hash equality** -- Shared components are stored once per unique value per
   chunk. This requires hashing or equality comparison of component values. Large shared components
   (e.g., complex material descriptors) may have expensive equality checks. Should we require
   `Hash + Eq` on shared components, or use a content-addressed approach?

6. **Prefab override granularity** -- Copy-on-write overrides currently operate at the
   whole-component level. A field-level override system (like Unity's prefab property overrides)
   would be more memory efficient but significantly more complex. Is component-level granularity
   sufficient?

7. **Dynamic component performance parity** -- Runtime- registered dynamic components use
   type-erased paths that may be slower than statically-registered components. What is the
   acceptable performance gap? Should dynamic components be excluded from parallel iteration if they
   cannot guarantee Send + Sync?
