# Spatial Awareness System Design

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are defined in
> [features/](../../features/), [requirements/](../../requirements/), and
> [user-stories/](../../user-stories/). The table below traces design elements to those definitions.

### Engine Primitives (primary trace)

| Feature   | Requirement | User Story  | Design Element                   |
|-----------|-------------|-------------|----------------------------------|
| F-17.3.1  | R-17.3.1    | US-17.3.1   | SenseDefinition primitive        |
| F-17.3.2  | R-17.3.2    | US-17.3.2   | 2D sense shapes (cone, etc.)    |
| F-17.3.3  | R-17.3.3    | US-17.3.3   | Awareness state machine          |
| F-17.3.4  | R-17.3.4    | US-17.3.4   | AwarenessTransitionEvent         |
| F-17.3.5  | R-17.3.5    | US-17.3.5   | 100 queries/1,000 tgts < 2 ms    |
| F-17.3.6  | R-17.3.6    | US-17.3.6   | Selection queries (50 < 0.5 ms)  |
| F-17.3.7  | R-17.3.7    | US-17.3.7   | GPU ultra-scale evaluation       |
| F-17.3.8  | R-17.3.8    | US-17.3.8   | 3D awareness visual indicators   |
| F-17.3.9  | R-17.3.9    | US-17.3.9   | Debug gizmo rendering of senses  |
| F-17.3.10 | R-17.3.10   | US-17.3.10  | One-frame awareness latency      |

1. **R-17.3.1** -- `SenseDefinition` primitive with shape, range, falloff, tags
2. **R-17.3.2** -- 2D sense shapes (circle, cone, rectangle) with Transform2D
3. **R-17.3.3** -- Per-entity awareness state machine (5 states)
4. **R-17.3.4** -- `AwarenessTransitionEvent` via ECS event channel
5. **R-17.3.5** -- 100 concurrent queries on 1,000 targets within 2 ms
6. **R-17.3.6** -- Selection queries (raycast, box, nearest-N) within 0.5 ms
7. **R-17.3.7** -- GPU compute spatial awareness for 1M+ entities
8. **R-17.3.8** -- 3D visual indicators driven by awareness transitions
9. **R-17.3.9** -- Debug gizmo rendering of sense volumes, layer filtered
10. **R-17.3.10** -- Awareness state change within one frame of stimulus

### Game-Framework Consumers (cross-reference)

| Feature   | Requirement | Consumer Role                                   |
|-----------|-------------|-------------------------------------------------|
| F-7.6.1   | R-7.6.1     | Sight sense (cone + line of sight)              |
| F-7.6.2   | R-7.6.2     | Hearing sense (radius + attenuation)            |
| F-7.6.5   | R-7.6.5     | Stimuli registration and expiration             |
| F-7.6.6   | R-7.6.6     | Sense aging and memory decay                    |
| F-7.6.7   | R-7.6.7     | Custom senses and perception priority           |
| F-13.18.1 | R-13.18.1   | Player visibility and stealth scoring           |
| F-13.18.2 | R-13.18.2   | AI alert state machine                          |
| F-13.18.3 | R-13.18.3   | Noise generation and distraction                |
| F-13.11.1 | R-13.11.1   | 3D world picking via shared spatial index       |
| F-13.11.2 | R-13.11.2   | 2D screen-space picking with touch slop         |
| F-1.9.1   | R-1.9.1     | Unified BVH/octree spatial index                |
| F-1.9.4   | R-1.9.4     | Unified spatial query API                       |
| F-1.9.9   | R-1.9.9     | AI perception and gameplay integration          |

### Cross-Cutting Dependencies

| Dependency           | Source  | Consumed API              |
|----------------------|---------|---------------------------|
| ECS world, queries   | F-1.1.1 | `Query`, `Entity`         |
| Shared spatial index | F-1.9.1 | Range queries, raycasts   |
| Event channels       | F-1.5.1 | `EventWriter`, `EventReader` |
| Change detection     | F-1.1.22| `Changed<T>` tracking     |
| Team / Faction       | F-13.1  | `FactionId`, `Allegiance` |

### Non-Functional Requirements

| NFR        | Target         | Description               |
|------------|----------------|---------------------------|
| NFR-SA.1   | < 2 ms/frame   | 100 entities querying     |
|            |                | 1000 targets              |
| NFR-SA.2   | < 0.5 ms/frame | Selection queries for     |
|            |                | 50 concurrent picks       |
| NFR-SA.3   | 1 frame        | Stimulus-to-awareness     |
|            |                | latency                   |

## Overview

Multi-factor spatial query system that unifies AI perception, stealth detection, target selection,
and world interaction picking. One generic system replaces four separate game-specific ones.

### Key Concepts

1. **SenseDefinition** -- a named spatial query with shape, range, falloff, and filter tags.
   "Sight", "hearing", and "smell" are data-driven sense configurations, not hardcoded systems.
2. **AwarenessState** -- per-entity state machine tracking detection level toward other entities
   (Unaware -> Suspicious -> Alert -> Tracking -> Lost).
3. **AwarenessQuery** -- run a sense against the shared spatial index, score results by
   distance/angle/occlusion/ modifiers, return ranked targets.
4. **SelectionQuery** -- simplified awareness query for player-facing picking (raycast, box select,
   nearest-N).

All queries go through the shared BVH spatial index (F-1.9.1). No separate spatial structures.

### Design Principles

1. **ECS-primary (~90%)-based.** All state lives in components. All logic runs as systems. No
   parallel data stores.
2. **Data-driven and no-code.** Sense definitions, scoring functions, thresholds, and decay rules
   are authored in the visual editor.
3. **No genre assumptions.** The same system drives stealth AI, target selection, fog of war vision,
   and interaction proximity. Configuration alone determines behavior.
4. **Shared spatial index.** All range queries and line-of-sight checks use the shared BVH (F-1.9.1)
   in 3D or the 2D BVH for 2D entities. No per-system spatial acceleration.
5. **First-class 2D.** 3D (`SenseShape`) and 2D (`SenseShape2D` via `SenseShape::Circle2D`,
   `Cone2D`, `Rect2D`) variants work identically. `Transform2D` entities use the 2D BVH.
6. **Static dispatch.** All systems are monomorphic. No trait objects on the hot path.
7. **Immutable definitions.** `SenseDefinition`, `ScoringFunction`, and `AwarenessTransition` are
   immutable data. Mutable runtime state is isolated to `AwarenessState`.

### Performance Targets

| Metric                         | Target               |
|--------------------------------|----------------------|
| 100 sources, 1000 targets      | < 2 ms (NFR-SA.1)   |
| 50 selection queries           | < 0.5 ms (NFR-SA.2) |
| Stimulus to awareness          | 1 frame (NFR-SA.3)  |
| Awareness state decay (idle)   | < 50 us/frame        |
| Single sense eval (4 factors)  | < 10 us              |

## Architecture

### Cross-Subsystem Integration

| Subsystem      | Direction    | Data                    | Mechanism                         |
|----------------|--------------|-------------------------|-----------------------------------|
| AI behavior    | consumes     | `AwarenessState`        | ECS query in Update               |
| AI navigation  | consumes     | threat positions        | ECS query on `AwarenessEntry`     |
| Physics        | consumed by  | LOS occlusion raycasts  | physics raycast API               |
| Shared BVH     | consumed by  | proximity queries       | `SpatialIndex` API                |
| Grids          | consumed by  | scent / influence maps  | `UniformGrid` read                |
| Audio          | consumes     | spatial occlusion       | shared BVH query                  |
| Networking     | produces/    | awareness replication   | relevancy grid + delta snapshot   |
|                | consumes     |                         |                                   |
| UI             | consumes     | threat indicators       | `AwarenessTransitionEvent`        |
| Rendering      | consumes     | debug sense volumes     | render layer bitmask filter       |
| VFX            | consumes     | detection-state VFX     | `AwarenessTransitionEvent`        |
| Game framework | consumes     | stealth / alert logic   | ECS query on `AwarenessState`     |
| Editor         | consumes     | sense volume gizmos     | debug overlay system              |

### Module Boundaries

```mermaid
graph TD
    subgraph sa["harmonius_game::spatial_awareness"]
        SD[SenseDefinition]
        SS[SenseShape]
        FC[FalloffCurve]
        SF[ScoringFunction]
        SR[SenseResult]
        AW[AwarenessState]
        AE[AwarenessEntry]
        AL[AwarenessLevel]
        AT[AwarenessTransition]
        AC[AwarenessConfig]
        SQ[SelectionQuery]
        SRE[SelectionResult]
        ATE[AwarenessTransitionEvent]
    end

    subgraph engine["Engine Dependencies"]
        ECS[ECS World]
        SI[Shared Spatial Index]
        EVT[Event Channels]
        TAG[TagSet]
    end

    SD --> SS
    SD --> FC
    SD --> SF
    AW --> AE
    AE --> AL
    AT --> AL
    ATE --> AL
    ATE --> EVT
    SQ --> SI
    SD --> TAG
```

### File Layout

```text
harmonius_game/
├── spatial_awareness/
│   ├── mod.rs              # Re-exports
│   ├── sense.rs            # SenseDefinitionId,
│   │                       # SenseDefinition,
│   │                       # SenseShape,
│   │                       # FalloffCurve,
│   │                       # ScoringFunction
│   ├── result.rs           # SenseResult
│   ├── awareness.rs        # AwarenessLevel,
│   │                       # AwarenessEntry,
│   │                       # AwarenessState,
│   │                       # AwarenessTransition,
│   │                       # AwarenessConfig,
│   │                       # AwarenessTransitionEvent
│   ├── selection.rs        # SelectionQuery,
│   │                       # SelectionResult
│   ├── query.rs            # query_sense(),
│   │                       # execute_selection()
│   ├── update.rs           # update_awareness()
│   └── systems/
│       ├── sense_eval.rs   # SenseEvaluatorSystem
│       ├── transition.rs   # AwarenessTransitionSys
│       ├── decay.rs        # AwarenessDecaySystem
│       └── selection.rs    # SelectionSystem
│   # No plugin.rs — system registration is
│   # codegen'd into the middleman .dylib.
```

### System Execution Order

Sense evaluation and awareness updates run in `FixedUpdate` for determinism (matching physics tick
rate). Selection queries run in `Update` for immediate responsiveness to player input.

```mermaid
flowchart LR
    subgraph FU[FixedUpdate]
        subgraph phase1[Sense Evaluation]
            EVAL[SenseEvaluator]
        end
        subgraph phase2[Awareness Update]
            TRANS[AwarenessTransition]
            DECAY[AwarenessDecay]
        end
        phase1 --> phase2
    end
    subgraph U[Update]
        subgraph phase3[Selection]
            SEL[SelectionSystem]
        end
    end
    FU --> U
```

### Awareness State Machine

```mermaid
stateDiagram-v2
    [*] --> Unaware
    Unaware --> Suspicious : score > suspicious
    Suspicious --> Alert : score > alert
    Suspicious --> Unaware : decay below threshold
    Alert --> Tracking : score > tracking
    Alert --> Suspicious : decay below alert
    Tracking --> Lost : no stimulus for N ticks
    Tracking --> Alert : decay below tracking
    Lost --> Suspicious : new stimulus
    Lost --> Unaware : timeout expires
```

### Core Data Structures

```mermaid
classDiagram
    class SenseDefinitionId {
        +u32 0
    }

    class SenseShape {
        <<enumeration>>
        Sphere
        Cone
        Box
        Cylinder
        Circle2D
        Cone2D
        Rect2D
    }

    class FalloffCurve {
        <<enumeration>>
        Linear
        InverseLinear
        Quadratic
        InverseQuadratic
        Custom
    }

    class ScoringFunction {
        +f32 distance_weight
        +f32 angle_weight
        +f32 occlusion_penalty
        +f32 modifier_bonus
    }

    class SenseDefinition {
        +SenseDefinitionId id
        +StringId name
        +SenseShape shape
        +f32 range
        +FalloffCurve falloff
        +TagSet filter_tags
        +bool occlusion_check
        +ScoringFunction scoring
        +f32 update_rate_hz
    }

    class SenseResult {
        +Entity entity
        +f32 distance
        +f32 angle
        +f32 occlusion
        +f32 raw_score
        +f32 final_score
    }

    class AwarenessLevel {
        <<enumeration>>
        Unaware
        Suspicious
        Alert
        Tracking
        Lost
    }

    class AwarenessEntry {
        +Entity target
        +AwarenessLevel level
        +f32 score
        +Vec3 last_seen_position
        +u64 last_seen_tick
    }

    class AwarenessState {
        +SmallVec entries
        +SmallVec senses
        +get_entry(Entity) Option
        +highest_threat() Option
        +entities_at_level(AwarenessLevel) Vec
        +is_aware_of(Entity) bool
    }

    class AwarenessTransition {
        +f32 suspicious_threshold
        +f32 alert_threshold
        +f32 tracking_threshold
        +f32 decay_rate
        +u32 lost_timeout_ticks
    }

    class AwarenessConfig {
        +u32 tick_rate
        +u16 max_targets_per_entity
        +f32 global_decay_rate
    }

    class SelectionQuery {
        <<enumeration>>
        Raycast
        BoxSelect
        SphereSelect
        NearestN
    }

    class SelectionResult {
        +Entity entity
        +Vec3 hit_point
        +f32 distance
    }

    class AwarenessTransitionEvent {
        +Entity source
        +Entity target
        +AwarenessLevel from
        +AwarenessLevel to
        +f32 score
    }

    SenseDefinition --> SenseDefinitionId
    SenseDefinition --> SenseShape
    SenseDefinition --> FalloffCurve
    SenseDefinition *-- ScoringFunction
    AwarenessEntry --> AwarenessLevel
    AwarenessState *-- AwarenessEntry
    AwarenessState --> SenseDefinitionId
    AwarenessTransitionEvent --> AwarenessLevel
```

## API Design

### Sense Definition and Shape

```rust
/// Unique identifier for a sense type.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct SenseDefinitionId(pub u32);

/// Geometric shape of a sense's detection volume.
#[derive(
    Clone, Debug, PartialEq,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub enum SenseShape {
    /// Omnidirectional detection (hearing, smell).
    Sphere { radius: f32 },
    /// Directional detection (sight).
    Cone { radius: f32, half_angle: f32 },
    /// Axis-aligned volume (trigger zones).
    Box { half_extents: Vec3 },
    /// Vertical cylinder (proximity detection).
    Cylinder { radius: f32, height: f32 },
    // 2D variants — used when the entity has a
    // Transform2D component.
    Circle2D { radius: f32 },
    Cone2D { radius: f32, half_angle: f32 },
    Rect2D { half_extents: Vec2 },
}

/// Falloff curve controlling how score attenuates
/// with distance from the sense origin.
#[derive(
    Clone, Debug, PartialEq,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub enum FalloffCurve {
    /// Score decreases linearly with distance.
    Linear,
    /// Score is high near origin, drops to zero
    /// at max range.
    InverseLinear,
    /// Score decreases with distance squared.
    Quadratic,
    /// Score is high near origin, quadratic drop.
    InverseQuadratic,
    /// Designer-authored curve asset.
    Custom(AssetId),
}

// Custom sense shapes, falloff curves, and scoring
// functions created in the visual editor are
// codegen'd into the middleman .dylib. Engine
// registers codegen'd variants at startup; no
// runtime reflection or dynamic dispatch.
```

### Scoring Function

```rust
/// Weights and penalties for computing a sense's
/// final score from raw spatial data.
#[derive(
    Clone, Debug,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct ScoringFunction {
    /// Weight applied to distance factor (0..1).
    /// Higher values make distance more important.
    pub distance_weight: f32,
    /// Weight applied to angle factor (0..1).
    /// Only relevant for directional senses.
    pub angle_weight: f32,
    /// Penalty subtracted when target is occluded.
    /// Range: 0.0 ..= 1.0.
    pub occlusion_penalty: f32,
    /// Bonus added from external modifiers
    /// (equipment, abilities, terrain). Range:
    /// -1.0 ..= 1.0.
    pub modifier_bonus: f32,
}
```

### Sense Definition

```rust
/// Definition of a single sense. Immutable data
/// asset authored in the visual editor. Loaded
/// from data tables at startup via zero-copy mmap
/// (rkyv). Editor metadata generated by codegen.
#[derive(
    Clone, Debug,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct SenseDefinition {
    /// Unique identifier for this sense type.
    pub id: SenseDefinitionId,
    /// Human-readable name for editor display.
    pub name: StringId,
    /// Geometric shape of the detection volume.
    pub shape: SenseShape,
    /// Maximum detection range in world units.
    pub range: f32,
    /// How score attenuates with distance.
    pub falloff: FalloffCurve,
    /// Tag filter: only entities matching these
    /// tags are considered as candidates.
    pub filter_tags: TagSet,
    /// Whether to perform occlusion raycasts
    /// against the spatial index.
    pub occlusion_check: bool,
    /// Scoring weights for this sense.
    pub scoring: ScoringFunction,
    /// How many times per second this sense
    /// re-evaluates. Lower rates save budget.
    pub update_rate_hz: f32,
}
```

### Sense Result

```rust
/// Result of evaluating a single sense against a
/// single candidate entity. Produced by
/// `query_sense`, consumed by `update_awareness`.
#[derive(Clone, Debug)]
pub struct SenseResult {
    /// The detected entity.
    pub entity: Entity,
    /// Distance from sense origin to target in
    /// world units.
    pub distance: f32,
    /// Angle between sense forward vector and
    /// direction to target in radians.
    pub angle: f32,
    /// Occlusion factor (0.0 = fully visible,
    /// 1.0 = fully occluded).
    pub occlusion: f32,
    /// Score before scoring function weights.
    pub raw_score: f32,
    /// Final weighted score after all factors.
    /// Range: 0.0 ..= 1.0.
    pub final_score: f32,
}
```

### Awareness State

```rust
/// Awareness level of a source toward a target.
/// Ordered from lowest to highest alertness.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
    PartialOrd, Ord,
)]
pub enum AwarenessLevel {
    /// No knowledge of the target.
    Unaware,
    /// Faint signal detected, not confirmed.
    Suspicious,
    /// Target confirmed, actively responding.
    Alert,
    /// Maintaining contact with a known target.
    Tracking,
    /// Contact lost, searching or timing out.
    Lost,
}

/// A single entry tracking awareness of one
/// target entity. Stored inside AwarenessState.
#[derive(Clone, Debug)]
pub struct AwarenessEntry {
    /// The entity being tracked.
    pub target: Entity,
    /// Current awareness level toward this target.
    pub level: AwarenessLevel,
    /// Current accumulated perception score.
    /// Range: 0.0 ..= 1.0.
    pub score: f32,
    /// Last known world position of the target.
    pub last_seen_position: Vec3,
    /// Tick when the target was last detected.
    pub last_seen_tick: u64,
}

/// ECS component: per-entity awareness state
/// tracking detection levels toward multiple
/// targets. Attached to entities that need
/// spatial awareness (NPCs, cameras, turrets).
#[derive(Component, Debug)]
pub struct AwarenessState {
    /// Active awareness entries, one per tracked
    /// target. Inline storage for 8 entries.
    pub entries: SmallVec<[AwarenessEntry; 8]>,
    /// Set of sense definitions this entity uses.
    /// Inline storage for 4 senses.
    pub senses: SmallVec<[SenseDefinitionId; 4]>,
}

impl AwarenessState {
    /// Find the entry for a specific target.
    pub fn get_entry(
        &self,
        target: Entity,
    ) -> Option<&AwarenessEntry>;

    /// Return the entry with the highest score.
    pub fn highest_threat(
        &self,
    ) -> Option<&AwarenessEntry>;

    /// Return all entities at a given awareness
    /// level.
    pub fn entities_at_level(
        &self,
        level: AwarenessLevel,
    ) -> Vec<Entity>;

    /// Returns true if target has any entry above
    /// Unaware.
    pub fn is_aware_of(
        &self,
        target: Entity,
    ) -> bool;
}
```

### Awareness Transition and Config

```rust
/// Thresholds and rates governing awareness level
/// transitions. Immutable data authored in the
/// visual editor. Loaded via rkyv zero-copy mmap.
#[derive(
    Clone, Debug,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct AwarenessTransition {
    /// Score threshold to enter Suspicious.
    pub suspicious_threshold: f32,
    /// Score threshold to enter Alert.
    pub alert_threshold: f32,
    /// Score threshold to enter Tracking.
    pub tracking_threshold: f32,
    /// Rate at which score decays per tick when
    /// no new stimulus is received.
    pub decay_rate: f32,
    /// Number of ticks with no stimulus before
    /// a Lost entry reverts to Unaware.
    pub lost_timeout_ticks: u32,
}

/// Resource: global configuration for the spatial
/// awareness system. Loaded via rkyv zero-copy mmap.
#[derive(
    Clone, Debug,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct AwarenessConfig {
    /// How often awareness updates run (ticks per
    /// second). Decoupled from frame rate.
    pub tick_rate: u32,
    /// Maximum tracked targets per entity. Oldest
    /// entries are evicted when exceeded.
    pub max_targets_per_entity: u16,
    /// Global decay rate multiplier applied on
    /// top of per-transition decay rates.
    pub global_decay_rate: f32,
}
```

### Awareness Transition Event

```rust
/// Event fired when an awareness level changes.
/// Consumed by behavior trees, alert animations,
/// UI indicators, and fog of war updates.
#[derive(Clone, Debug)]
pub struct AwarenessTransitionEvent {
    /// The perceiving entity.
    pub source: Entity,
    /// The perceived entity.
    pub target: Entity,
    /// Previous awareness level.
    pub from: AwarenessLevel,
    /// New awareness level.
    pub to: AwarenessLevel,
    /// Current score at time of transition.
    pub score: f32,
}
```

### Query Functions

```rust
/// Evaluate a sense definition from a given origin
/// and forward direction against all candidates in
/// the shared spatial index. Returns scored
/// results sorted by final_score descending.
///
/// When the source entity has a `Transform2D`
/// component, `origin_2d` and `forward_2d` are
/// used instead, and the 2D BVH is queried.
/// 3D `SenseShape` variants require `Vec3` origin;
/// 2D variants (`Circle2D`, `Cone2D`, `Rect2D`)
/// require `Vec2` origin via `SenseOrigin`.
///
/// Pure function: no side effects, no mutation.
pub fn query_sense(
    sense: &SenseDefinition,
    origin: SenseOrigin,
    spatial_index: &SpatialIndex,
    world: &World,
) -> SmallVec<[SenseResult; 16]>;

/// Origin for a sense evaluation — 3D or 2D
/// depending on the entity's transform type.
pub enum SenseOrigin {
    World3D { position: Vec3, forward: Vec3 },
    World2D { position: Vec2, forward: Vec2 },
}

/// Update an entity's awareness state with new
/// sense results. Applies scoring, transitions
/// levels based on thresholds, and decays stale
/// entries.
pub fn update_awareness(
    state: &mut AwarenessState,
    results: &[SenseResult],
    transition: &AwarenessTransition,
    current_tick: u64,
);
```

### Selection Queries

```rust
/// Player-facing spatial query for entity picking
/// and selection. Simplified interface on top of
/// the shared spatial index.
#[derive(Clone, Debug)]
pub enum SelectionQuery {
    /// Single ray from camera through screen
    /// coordinates. Returns nearest hit.
    Raycast {
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
    },
    /// Axis-aligned box select (marquee drag).
    BoxSelect {
        min: Vec3,
        max: Vec3,
    },
    /// Spherical area select (radial menu,
    /// area abilities).
    SphereSelect {
        center: Vec3,
        radius: f32,
    },
    /// Find the N nearest entities within radius.
    NearestN {
        center: Vec3,
        radius: f32,
        count: u32,
    },
    /// 2D marquee box select in screen/world space.
    /// Used for top-down RTS and 2D games.
    BoxSelect2D {
        min: Vec2,
        max: Vec2,
    },
    /// 2D circle select for radial area effects.
    CircleSelect2D {
        center: Vec2,
        radius: f32,
    },
}

/// Result of a selection query. One entry per
/// selected entity, sorted by distance.
#[derive(Clone, Debug)]
pub struct SelectionResult {
    /// The selected entity.
    pub entity: Entity,
    /// World-space hit point (for raycasts) or
    /// entity center (for area selects).
    pub hit_point: Vec3,
    /// Distance from query origin to hit point.
    pub distance: f32,
}

/// Execute a selection query against the shared
/// spatial index with an entity filter predicate.
/// Returns results sorted by distance ascending.
///
/// Pure function: no side effects, no mutation.
pub fn execute_selection(
    query: &SelectionQuery,
    spatial_index: &SpatialIndex,
    filter: impl Fn(Entity) -> bool,
) -> Vec<SelectionResult>;
```

### Scoring Formula

For a single sense evaluating a single target:

```text
distance_factor = falloff(distance / range)
angle_factor = 1.0 - (angle / max_angle)
occlusion_factor = if occluded { 0.0 } else { 1.0 }

raw_score =
    distance_factor * scoring.distance_weight
  + angle_factor * scoring.angle_weight
  - occlusion_factor * scoring.occlusion_penalty
  + scoring.modifier_bonus

final_score = clamp(raw_score, 0.0, 1.0)
```

| Factor   | Computation                        |
|----------|------------------------------------|
| Distance | `falloff(dist / sense.range)`      |
| Angle    | `1.0 - (angle / max_angle)`       |
| Occlusion| Raycast from origin to target     |
| Modifier | External bonus from components     |

## Data Flow

### Sense Evaluation and Awareness Update

```mermaid
sequenceDiagram
    participant DEF as SenseDefinition
    participant SRC as AwarenessState
    participant SI as Shared Spatial Index
    participant QS as query_sense
    participant UA as update_awareness
    participant EVT as EventChannel

    DEF->>QS: sense config
    SRC->>QS: origin and forward
    QS->>SI: range query by shape
    SI-->>QS: candidate entities
    loop Each candidate
        QS->>QS: compute distance
        QS->>QS: compute angle
        QS->>SI: occlusion raycast
        SI-->>QS: occlusion factor
        QS->>QS: apply scoring function
    end
    QS-->>UA: Vec of SenseResult
    UA->>SRC: update entries
    UA->>UA: check thresholds
    alt Level transition
        UA->>EVT: AwarenessTransitionEvent
    end
    UA->>SRC: apply decay to stale
```

### End-to-End Pipeline

1. **Asset load:** `SenseDefinition` loaded from data tables. `AwarenessTransition` loaded per
   entity archetype.
2. **Component attach:** `AwarenessState` attached to entities that need spatial awareness.
3. **Phase 1 -- Sense evaluation:** `SenseEvaluatorSystem` calls `query_sense` for each entity with
   an `AwarenessState` component, using the shared spatial index for range queries and occlusion
   raycasts.
4. **Phase 2 -- Awareness update:** `AwarenessTransitionSystem` calls `update_awareness` to
   transition levels based on scores and thresholds. `AwarenessDecaySystem` decays scores for
   entries with no recent stimulus.
5. **Phase 3 -- Consumption:** AI behavior trees read `AwarenessState` for decisions. Stealth HUD
   reads scores for visibility indicators. Fog of war reads awareness levels for faction vision.
6. **Selection:** `SelectionSystem` processes `SelectionQuery` events from the input system for
   player-facing entity picking. Results written as `SelectionResult` events.

### Frame-Boundary Handoff

`AwarenessState` is written exclusively in `FixedUpdate` and read in `Update` (AI, UI, VFX). No lock
is needed because ECS guarantees exclusive write access inside a phase.

- `AwarenessTransitionEvent` objects are buffered in a crossbeam-channel and drained at the
  `FixedUpdate`→`Update` phase boundary. Consumers in `Update` see all events from the most recent
  fixed tick.
- `SelectionResult` events are produced and consumed within the same `Update` frame. No cross-phase
  latency.
- If `FixedUpdate` runs multiple sub-steps in one render frame, only the final `AwarenessState`
  snapshot is visible to `Update` consumers.

## Platform Considerations

### Scaling Tiers

| Platform | Max Sources | Max Targets | Budget |
|----------|-------------|-------------|--------|
| Desktop  | 100         | 1000        | 2 ms   |
| Console  | 100         | 1000        | 2 ms   |
| Mobile   | 30          | 200         | 1 ms   |

### Platform-Specific Notes

| Platform | Consideration                          |
|----------|----------------------------------------|
| Mobile   | Reduce max update_rate_hz to 5.       |
|          | Cap max_targets_per_entity to 4.       |
| Mobile   | Disable occlusion raycasts for senses  |
|          | with > 50 candidates.                  |
| All      | Spatial queries routed through shared  |
|          | BVH (F-1.9.1).                         |
| All      | Falloff curve sampling uses linear     |
|          | interpolation for cache efficiency.    |

### Performance Budget

| System               | Budget       | Strategy              |
|----------------------|--------------|-----------------------|
| Sense evaluation     | 1.5 ms/frame | LOD by distance       |
| Awareness transition | 0.2 ms/frame | Only active pairs     |
| Awareness decay      | 0.1 ms/frame | Skip zero-score       |
| Selection queries    | 0.2 ms/frame | Event-driven, on      |
|                      |              | input only            |

## Test Plan

Full test cases are in the companion file
[spatial-awareness-test-cases.md](spatial-awareness-test-cases.md).

### Unit Tests

| Test                                    | Req       |
|-----------------------------------------|-----------|
| `test_sphere_sense_candidates`          | R-7.6.2   |
| `test_cone_sense_fov_inside`           | R-7.6.1   |
| `test_cone_sense_fov_outside`          | R-7.6.1   |
| `test_box_sense_candidates`            | R-7.6.7   |
| `test_cylinder_sense_candidates`       | R-7.6.7   |
| `test_falloff_linear`                  | R-7.6.1   |
| `test_falloff_inverse_linear`          | R-7.6.1   |
| `test_falloff_quadratic`              | R-7.6.1   |
| `test_falloff_custom_curve`           | R-7.6.7   |
| `test_scoring_distance_only`          | R-7.6.1   |
| `test_scoring_angle_weight`           | R-7.6.1   |
| `test_scoring_occlusion_penalty`      | R-7.6.1   |
| `test_scoring_modifier_bonus`         | R-7.6.7   |
| `test_score_clamp_zero_one`           | R-7.6.1   |
| `test_awareness_unaware_to_suspicious` | R-13.18.2 |
| `test_awareness_suspicious_to_alert`  | R-13.18.2 |
| `test_awareness_alert_to_tracking`    | R-13.18.2 |
| `test_awareness_tracking_to_lost`     | R-13.18.2 |
| `test_awareness_lost_to_unaware`      | R-13.18.2 |
| `test_awareness_lost_to_suspicious`   | R-13.18.2 |
| `test_awareness_decay_reduces_score`  | R-7.6.6   |
| `test_awareness_decay_below_threshold`| R-7.6.6   |
| `test_awareness_max_targets_eviction` | R-7.6.5   |
| `test_highest_threat_returns_max`     | R-13.18.1 |
| `test_entities_at_level_filter`       | R-13.18.2 |
| `test_is_aware_of_above_unaware`      | R-13.18.2 |
| `test_selection_raycast_nearest`      | R-13.11.1 |
| `test_selection_box_all_inside`       | R-13.11.1 |
| `test_selection_sphere_radius`        | R-13.11.1 |
| `test_selection_nearest_n_count`      | R-13.11.1 |
| `test_selection_filter_excludes`      | R-13.11.1 |
| `test_selection_sorted_by_distance`   | R-13.11.1 |

1. **`test_sphere_sense_candidates`** -- Sphere sense with radius 10; 3 targets inside, 2 outside;
   verify 3 results returned.
2. **`test_cone_sense_fov_inside`** -- Cone sense with 90-degree half-angle; target at 45 degrees;
   verify non-zero score.
3. **`test_cone_sense_fov_outside`** -- Target at 100 degrees from cone forward; verify zero score.
4. **`test_box_sense_candidates`** -- Box sense with half-extents (5, 5, 5); targets inside and
   outside; verify correct inclusion.
5. **`test_cylinder_sense_candidates`** -- Cylinder sense with radius 5 and height 10; verify
   targets above height are excluded.
6. **`test_falloff_linear`** -- Linear falloff; target at half range; verify score is 0.5.
7. **`test_falloff_inverse_linear`** -- Inverse linear; target at half range; verify score is 0.5.
8. **`test_falloff_quadratic`** -- Quadratic falloff; target at half range; verify score is 0.25.
9. **`test_falloff_custom_curve`** -- Custom curve asset; verify score matches curve sample.
10. **`test_scoring_distance_only`** -- distance_weight 1.0, others 0.0; verify final_score equals
    distance factor.
11. **`test_scoring_angle_weight`** -- angle_weight 1.0; target at 45 degrees of 90-degree cone;
    verify score is 0.5.
12. **`test_scoring_occlusion_penalty`** -- Occluded target with occlusion_penalty 0.8; verify score
    reduced.
13. **`test_scoring_modifier_bonus`** -- modifier_bonus 0.2; verify score increased by 0.2.
14. **`test_score_clamp_zero_one`** -- Weights sum to 1.5; verify clamped to 1.0. Negative sum;
    verify 0.0.
15. **`test_awareness_unaware_to_suspicious`** -- Score exceeds suspicious_threshold; verify level
    changes.
16. **`test_awareness_suspicious_to_alert`** -- Score exceeds alert_threshold; verify transition.
17. **`test_awareness_alert_to_tracking`** -- Score exceeds tracking_threshold; verify transition.
18. **`test_awareness_tracking_to_lost`** -- No stimulus for lost_timeout_ticks; verify transition
    to Lost.
19. **`test_awareness_lost_to_unaware`** -- Lost entry times out; verify removal or Unaware.
20. **`test_awareness_lost_to_suspicious`** -- New stimulus while Lost; verify transition to
    Suspicious.
21. **`test_awareness_decay_reduces_score`** -- No stimulus; tick once; verify score reduced by
    decay_rate.
22. **`test_awareness_decay_below_threshold`** -- Decay drops score below suspicious_threshold;
    verify level demotes to Unaware.
23. **`test_awareness_max_targets_eviction`** -- Exceed max_targets_per_entity; verify oldest entry
    evicted.
24. **`test_highest_threat_returns_max`** -- Three entries with scores 0.3, 0.8, 0.5; verify
    highest_threat returns 0.8.
25. **`test_entities_at_level_filter`** -- Mixed levels; verify entities_at_level returns correct
    subset.
26. **`test_is_aware_of_above_unaware`** -- Entry at Suspicious; verify is_aware_of returns true. No
    entry; verify returns false.
27. **`test_selection_raycast_nearest`** -- Ray hits 3 entities; verify nearest returned first.
28. **`test_selection_box_all_inside`** -- 5 entities inside box, 3 outside; verify 5 results.
29. **`test_selection_sphere_radius`** -- Sphere with radius 10; verify only entities within 10
    returned.
30. **`test_selection_nearest_n_count`** -- NearestN with count 3; 10 entities in range; verify 3
    closest.
31. **`test_selection_filter_excludes`** -- Filter predicate rejects 2 of 5 candidates; verify 3
    results.
32. **`test_selection_sorted_by_distance`** -- Verify results sorted ascending by distance.

### Integration Tests

| Test                                   | Req        |
|----------------------------------------|------------|
| `test_100_sources_1000_targets_budget` | NFR-SA.1   |
| `test_50_selections_budget`           | NFR-SA.2   |
| `test_stimulus_to_awareness_latency`  | NFR-SA.3   |
| `test_full_awareness_lifecycle`       | R-13.18.2  |
| `test_selection_with_awareness`       | R-13.11.1  |

1. **`test_100_sources_1000_targets_budget`** -- Spawn 100 sources and 1000 targets; measure frame
   time; verify < 2 ms.
2. **`test_50_selections_budget`** -- Execute 50 concurrent selection queries; verify < 0.5 ms.
3. **`test_stimulus_to_awareness_latency`** -- Inject sense result; verify awareness transition
   completes within 1 frame.
4. **`test_full_awareness_lifecycle`** -- Entity progresses through all 5 awareness levels and back
   to Unaware via decay and timeout.
5. **`test_selection_with_awareness`** -- Selection query picks entity; verify result includes
   entities that also appear in awareness state.

### Benchmarks

| Benchmark                     | Target       | Source     |
|-------------------------------|--------------|------------|
| 100 sources, 1000 targets     | < 2 ms/frame | NFR-SA.1   |
| 50 selection queries          | < 0.5 ms     | NFR-SA.2   |
| Single sense eval             | < 10 us      | NFR-SA.1   |
| Awareness transition check    | < 1 us       | NFR-SA.1   |
| Selection raycast             | < 50 us      | NFR-SA.2   |
| Awareness decay (100 entries) | < 20 us      | NFR-SA.1   |

## Open Questions

1. **Multi-sense score combination.** When an entity has both vision and hearing senses, should
   scores combine via max or weighted sum? Max is simpler and avoids double-counting. Weighted sum
   captures cases where weak signals from multiple senses should stack.
2. **AwarenessEntry memory management.** Per source-target entries could grow large (100 sources x
   1000 targets = 100,000 potential pairs). The design only creates entries above the minimum
   threshold. A pool allocator for awareness entries may be needed if archetype storage fragments
   under high churn.
3. **Selection query batching.** Multiple selection queries per frame (e.g., multi-touch on mobile)
   could be batched into a single spatial index traversal. The current design processes queries
   individually. Batching depends on profiling real workloads.
4. **Occlusion query LOD.** Full raycasts for occlusion are expensive with many candidates. A tiered
   approach (skip occlusion for distant targets, full raycast for nearby ones) would reduce cost but
   may cause visible popping. The tier distances need profiling data.
5. **Network replication of awareness.** Awareness states may need replication for
   server-authoritative AI. The replication strategy (snapshot vs. delta) depends on the networking
   design (F-8) being finalized.

## Review feedback

### RF-1: Remove all Reflect derives

Remove `Reflect` derives from all 14 types. Replace with rkyv `Archive`/`Serialize`/`Deserialize`
for asset types. Editor metadata via codegen in the middleman .dylib.

### RF-2: rkyv serialization for asset types

Specify rkyv as the binary format. `SenseDefinition`, `AwarenessConfig`, `AwarenessTransition`,
`FalloffCurve`, `ScoringFunction` derive rkyv traits for zero-copy mmap loading.

### RF-3: Codegen for custom sense types

Custom sense shapes, falloff curves, scoring functions, and plugin-extended enums are codegen'd into
the middleman .dylib. Remove `SpatialAwarenessPlugin` if it implies bevy-style runtime registration.

### RF-4: 2D sense shapes and Transform2D support

The design is 3D-only. Add:

1. `SenseShape2D` enum: `Circle2D { radius }`, `Cone2D { radius, half_angle }`,
   `Rect2D { half_extents }`
2. All position types accept `Vec2` for 2D or `Vec3` for 3D
3. 2D BVH queries for top-down stealth, RTS vision, 2D platformer enemy detection
4. `SelectionQuery` 2D variants (box select in screen space, circle select)
5. Test cases for 2D sense evaluation

### RF-5: Create companion test cases file

Create `spatial-awareness-test-cases.md` with TC-IDs in `TC-X.Y.Z.N` format.

### RF-6: Game loop phase assignment

Sense evaluation and awareness updates run in `FixedUpdate` (deterministic, matching physics).
Selection queries run in `Update` (responsive to player input). Document in the system execution
diagram.

### RF-7: Frame-boundary handoff

Document: `AwarenessState` written in FixedUpdate, read by AI/UI in Update. Events buffered in
crossbeam-channel, drained at phase transitions. Selection results available same frame.

### RF-8: Cross-subsystem integration table

| Subsystem | Direction | Data | Mechanism |
|-----------|-----------|------|-----------|
| AI behavior | consumes | AwarenessState | ECS query |
| AI navigation | consumes | threat positions | ECS query |
| Physics | consumed by | raycast for LOS occlusion | physics raycast API |
| Shared BVH | consumed by | proximity queries | SpatialIndex API |
| Grids | consumed by | scent/influence maps | UniformGrid read |
| Audio | consumes | spatial occlusion | shared BVH query |
| Networking | produces/consumes | awareness replication | relevancy grid + delta |
| UI | consumes | threat indicators | AwarenessTransitionEvent |
| Rendering | consumes | debug sense viz | render layer filtered |
| VFX | consumes | detection state VFX | AwarenessTransitionEvent |
| Game framework | consumes | stealth/alert gameplay | ECS query on AwarenessState |
| Editor | consumes | sense volume gizmos | debug overlay system |

### RF-9: Per-thread arenas for query results

`query_sense` and `execute_selection` return heap-allocated `Vec`. Use per-thread arena-allocated
slices or `SmallVec<[SenseResult; 16]>`. Arenas reset at frame boundaries.

### RF-10: Algorithm reference URLs

Add URLs for: perception scoring models (GDC talks on AI perception), BVH query algorithms,
awareness state machine patterns, falloff curve mathematics.

### RF-11: Tie tick-based logic to FixedUpdate

`tick_rate`, `lost_timeout_ticks`, `decay_rate` must correspond to FixedUpdate steps. State
explicitly for determinism.

### RF-12: Deterministic sort tiebreaker

All result sorting uses `Entity` ID as tiebreaker for equal scores. State explicitly that no HashMap
is used in any hot-path structure.

### RF-13: Networking relevancy grid integration

Describe how awareness states interact with the networking relevancy grid. Server determines which
clients receive awareness updates based on grid cell proximity.

### RF-14: GPU visibility boundary

Add: "Rendering visibility is handled by GPU-driven culling and is outside the scope of this system.
This system handles gameplay/AI/audio spatial queries only."

### RF-15: Parallelism via custom job system

Sense evaluations distributed across workers via `par_iter` over (source, sense) pairs. Each
evaluation is independent with thread-local results merged after completion. Per-thread arenas for
temporaries.

### RF-16: Render layer filtering for debug viz

Debug sense volume rendering and threat indicators use the render layer u32 bitmask — only visible
on the correct cameras (editor debug, HUD, minimap).

### RF-17: Expand platform considerations

Name all platforms (Windows, macOS, Linux, iOS, Android, Switch, VR). VR-specific: head-tracked
sense forward vector, 360-degree awareness for spatial audio occlusion, hand-tracked interaction for
selection queries.

### RF-18: Replace ASCII file layout with table

Convert the file layout `text` block to a Mermaid diagram or flat table.

### RF-19: GPU-accelerated spatial awareness for ultra-scale AI

The CPU design supports ~100 sources x 1000 targets in < 2 ms. For ultra-scale scenarios (millions
of entities in the world, thousands actively evaluating senses), the CPU path is insufficient. Add a
GPU compute pipeline for spatial awareness:

1. **GPU sense evaluation** — upload source positions, sense shapes, and target positions to GPU
   structured buffers. A compute shader performs range/cone/box tests for all (source, target) pairs
   in parallel. Output: per-source list of candidates with raw distance and angle scores. This is a
   massive parallel workload ideal for GPU — millions of pair tests per dispatch.
2. **GPU scoring** — a second compute pass applies falloff curves and scoring functions to the
   candidate list. Output: per-source sorted result buffer with final scores. Scoring functions are
   HLSL compute kernels compiled via DXC + MSC.
3. **GPU→CPU readback** — only the top-N results per source are read back to the CPU (typically 8-32
   per source). The full candidate list stays GPU-side. Readback uses a ring-buffered staging buffer
   with one-frame latency.
4. **CPU awareness update** — the CPU-side awareness state machine (`AwarenessState`) consumes the
   GPU results identically to CPU results. The awareness update remains CPU-side (small per-source
   state, branch-heavy logic).
5. **Hybrid CPU/GPU selection** — small entity counts (< 500 targets) use the CPU path for
   zero-latency results. Large counts (> 500) use the GPU path with one-frame latency. The threshold
   is configurable per sense definition.
6. **Scale targets**:
   - 1M dormant entities in GPU buffers (position + faction only)
   - 10K active entities with full sense evaluation per frame
   - 1K sources with awareness state machines
   - Budget: < 1 ms GPU compute + < 0.5 ms CPU readback + state
7. **Render graph integration** — `GpuSenseEvalPass` and `GpuScoringPass` are compute render graph
   nodes. They run before the CPU awareness update phase. The readback buffer is a render graph
   resource with a read dependency from the awareness system.
8. **Applicability** — this enables: massive RTS battles (thousands of units with vision),
   open-world games (millions of NPCs with dormant awareness), battle royale (100 players each
   evaluating awareness against all others), MMO servers (thousands of NPCs with perception).

### RF-20: Awareness-driven 3D gameplay indicators

Awareness state drives 3D visual indicators: detection state icons above NPCs
(unaware/suspicious/alert/tracking), threat direction indicators on the player HUD, stealth
visibility meters, and enemy highlight outlines. These are split across world-space UI
(WorldSpaceAnchor for icons, see ui-framework.md F-10.1.10) and VFX (alert VFX burst on awareness
transition, see vfx/effects.md RF-26). The spatial awareness system owns the spawn/despawn logic:
`AwarenessTransitionEvent` triggers indicator updates. Indicators are filtered by render layer
bitmask (debug cameras show sense cone volumes, gameplay cameras show alert icons only).

### RF-21: Debug visualization in viewport

Sense volumes (vision cones, hearing spheres) and awareness state icons are rendered as debug gizmos
in the editor viewport (level-world.md RF-28 item 4). The spatial awareness design must expose:
per-source sense shape geometry for gizmo rendering, and per-target awareness level for icon
display. Debug viz is filtered by render layer bitmask (only on debug cameras).
