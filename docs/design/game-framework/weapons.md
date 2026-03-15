# Weapon System Design

## Requirements Trace

| Feature | Requirement | User Stories | Description |
|---------|-------------|--------------|-------------|
| F-13.16.1 | R-13.16.1 | US-13.16.1.1 -- US-13.16.1.5 | Configurable fire modes (semi, burst, auto) |
| F-13.16.2a | R-13.16.2a | US-13.16.2a.1 -- US-13.16.2a.5 | Magazine and ammo management |
| F-13.16.2b | R-13.16.2b | US-13.16.2b.1 -- US-13.16.2b.5 | Reload mechanics (tactical, empty, sequential) |
| F-13.16.2c | R-13.16.2c | US-13.16.2c.1 -- US-13.16.2c.6 | Swappable ammo types |
| F-13.16.3 | R-13.16.3 | US-13.16.3.1 -- US-13.16.3.6 | Recoil patterns and weapon spread |
| F-13.16.4a | R-13.16.4a | US-13.16.4a.1 -- US-13.16.4a.5 | Projectile drop and travel time |
| F-13.16.4b | R-13.16.4b | US-13.16.4b.1 -- US-13.16.4b.5 | Wind deflection |
| F-13.16.4c | R-13.16.4c | US-13.16.4c.1 -- US-13.16.4c.5 | Surface penetration and ricochet |
| F-13.16.4d | R-13.16.4d | US-13.16.4d.1 -- US-13.16.4d.5 | Weapon zeroing |
| F-13.16.5a | R-13.16.5a | US-13.16.5a.1 -- US-13.16.5a.5 | Attachment slot model |
| F-13.16.5b | R-13.16.5b | US-13.16.5b.1 -- US-13.16.5b.4 | Attachment visual integration |
| F-13.16.5c | R-13.16.5c | US-13.16.5c.1 -- US-13.16.5c.5 | Attachment customization UI |
| F-13.16.6a | R-13.16.6a | US-13.16.6a.1 -- US-13.16.6a.4 | Surface type tag system |
| F-13.16.6b | R-13.16.6b | US-13.16.6b.1 -- US-13.16.6b.3 | Impact VFX response |
| F-13.16.6c | R-13.16.6c | US-13.16.6c.1 -- US-13.16.6c.3 | Impact audio response |
| F-13.16.6d | R-13.16.6d | US-13.16.6d.1 -- US-13.16.6d.3 | Impact decal response |
| -- | NFR-13.16.1 | -- | 256 projectiles under 1 ms per physics tick |
| -- | NFR-13.16.2 | -- | Weapon feedback under 16 ms from input |

## Overview

The weapon system models firearms, melee weapons, thrown
weapons, and their supporting subsystems (ammunition,
ballistics, recoil, attachments, surface response). It
builds on top of the ability and combat system -- fire
modes are ability activation modes, projectiles are ECS
entities managed by the projectile system, and damage
flows through the gameplay effect pipeline.

All weapon state lives as ECS components. All weapon logic
runs as ECS systems. Designers author weapon configurations,
recoil patterns, ammo types, attachment stats, and impact
response tables entirely in the visual editor. No code.

The weapon system has four major subsystems:

1. **Fire Modes and Ammunition** -- fire rate, magazine,
   reload, ammo types.
2. **Recoil and Spread** -- per-weapon recoil curves,
   context-sensitive spread.
3. **Ballistics** -- projectile physics, penetration,
   ricochet, wind, zeroing.
4. **Weapon Customization** -- attachment slots,
   stat modifiers, visual integration.
5. **Surface Response** -- impact VFX, audio, decals
   dispatched by surface material type.

## Architecture

### Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_weapons"
        WD[WeaponDefinition]
        FM[FireModes]
        AM[Ammunition]
        RL[Reload]
        RC[Recoil / Spread]
        BA[Ballistics]
        AT[Attachments]
        SR[Surface Response]
    end

    subgraph "harmonius_abilities"
        AA[AbilityActivation]
        GE[GameplayEffects]
    end

    subgraph "Engine Systems"
        ECS[ECS Scheduler]
        SI[Shared Spatial Index]
        IN[Input Actions]
        AN[Animation]
        PH[Physics Materials]
        VFX[VFX / Audio / Decals]
        DB[Gameplay Databases]
        INV[Inventory]
        WX[Weather System]
    end

    WD --> FM
    WD --> AM
    WD --> RL
    WD --> RC
    WD --> BA
    WD --> AT
    FM --> AA
    AM --> INV
    AM --> DB
    RL --> AN
    RC --> IN
    BA --> SI
    BA --> PH
    BA --> WX
    AT --> DB
    AT --> INV
    SR --> PH
    SR --> VFX
    WD --> ECS
    BA --> GE
```

### Directory Layout

```
harmonius_weapons/
├── weapon.rs           # Weapon component,
│                       # WeaponDefinition asset
├── fire_mode.rs        # FireMode, FireModeConfig,
│                       # FireModeSystem
├── magazine.rs         # Magazine component,
│                       # ammo consumption
├── reload.rs           # ReloadState, ReloadSystem,
│                       # tactical/empty/sequential
├── ammo_type.rs        # AmmoType component,
│                       # ammo switching
├── recoil.rs           # RecoilPattern, RecoilState,
│                       # RecoilSystem
├── spread.rs           # SpreadState, SpreadSystem,
│                       # context modifiers
├── ballistics/
│   ├── trajectory.rs   # Trajectory integration,
│   │                   # gravity, drag
│   ├── wind.rs         # Wind deflection system
│   ├── penetration.rs  # Material penetration,
│   │                   # energy loss
│   ├── ricochet.rs     # Angle-based ricochet,
│   │                   # probability
│   └── zeroing.rs      # Weapon zeroing, aim offset
├── attachments/
│   ├── slot.rs         # AttachmentSlot, category
│   │                   # filter
│   ├── attachment.rs   # Attachment component,
│   │                   # stat modifiers
│   └── visual.rs       # Mesh snapping, socket
│                       # transforms
└── surface_response/
    ├── surface_tag.rs  # SurfaceTypeTag, terrain
    │                   # splatmap resolve
    ├── vfx.rs          # Impact VFX table lookup
    ├── audio.rs        # Impact audio table lookup
    └── decal.rs        # Impact decal placement
```

### Fire and Ballistics Flow

```mermaid
sequenceDiagram
    participant IN as Input
    participant FM as FireModeSystem
    participant AM as AmmoSystem
    participant RC as RecoilSystem
    participant BA as BallisticSystem
    participant SI as Shared Spatial Index
    participant PH as PhysicsMaterials
    participant SR as SurfaceResponseSystem
    participant GE as GameplayEffectSystem

    IN->>FM: FireRequest(entity, weapon_id)
    FM->>FM: Check fire mode (semi/burst/auto)
    FM->>AM: Check magazine rounds
    AM-->>FM: rounds available
    FM->>AM: Consume 1 round
    FM->>RC: Apply recoil kick and spread
    FM->>BA: Spawn projectile entity

    BA->>BA: Integrate trajectory
    BA->>SI: continuous collision detection
    SI-->>BA: hit result (entity, point, normal)
    BA->>PH: query surface material
    PH-->>BA: SurfaceType tag

    alt Penetration
        BA->>BA: Compute energy loss
        BA->>BA: Continue with reduced velocity
    else Ricochet
        BA->>BA: Reflect with energy loss
    else Absorbed
        BA->>GE: Apply damage effect to target
        BA->>SR: Dispatch impact response
        SR->>SR: Spawn VFX from table
        SR->>SR: Play audio from table
        SR->>SR: Place decal from table
    end
```

### Reload State Machine

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Firing : FireInput + HasAmmo
    Firing --> Idle : ReleaseInput (semi)
    Firing --> Firing : HoldInput (auto)
    Firing --> EmptyMag : LastRound
    Idle --> TacticalReload : ReloadInput + MagNotEmpty
    EmptyMag --> EmptyReload : ReloadInput
    TacticalReload --> Idle : ReloadComplete
    EmptyReload --> Idle : ReloadComplete
    Idle --> SequentialReload : ReloadInput (shotgun)
    SequentialReload --> SequentialReload : LoadShell
    SequentialReload --> Firing : FireInterrupt
    SequentialReload --> Idle : ReloadComplete
```

### Core Data Structures

```mermaid
classDiagram
    Weapon --> FireModeConfig
    Weapon --> Magazine
    Weapon *-- AttachmentSlot
    Magazine --> AmmoType
    FireModeConfig --> FireMode
    Projectile --> BallisticConfig
    AttachmentSlot --> Attachment

    class Weapon {
        +definition_id AssetId
        +active_fire_mode FireModeId
        +fire_modes SmallVec~FireModeId 3~
    }
    class FireMode {
        <<enumeration>>
        SemiAutomatic
        Burst
        FullAutomatic
    }
    class FireModeConfig {
        +mode FireMode
        +rounds_per_activation u8
        +inter_round_delay_ms u16
        +spread_modifier f32
        +recoil_modifier f32
        +fire_rate_rpm u16
        +spin_up_time_ms Option~u16~
    }
    class Magazine {
        +current_rounds u16
        +capacity u16
        +reserve_ammo u16
        +unlimited bool
    }
    class AmmoType {
        +damage_modifier f32
        +armor_penetration f32
        +status_effect Option~EffectId~
        +tracer_color Color
        +muzzle_vfx AssetId
    }
    class RecoilPattern {
        +curve_points SmallVec~Vec2 32~
        +recovery_rate f32
    }
    class SpreadState {
        +base_spread f32
        +current_spread f32
        +ads_modifier f32
        +move_modifier f32
        +crouch_modifier f32
        +first_shot_accuracy f32
    }
    class BallisticConfig {
        +muzzle_velocity f32
        +mass f32
        +drag_coefficient f32
        +wind_sensitivity f32
        +gravity_enabled bool
        +drag_enabled bool
    }
    class Projectile {
        +velocity Vec3
        +config BallisticConfig
        +effect_payload EffectId
        +distance_traveled f32
    }
    class AttachmentSlot {
        +slot_name SlotName
        +category_filter AttachmentCategory
        +equipped Option~AttachmentId~
    }
    class Attachment {
        +category AttachmentCategory
        +stat_modifiers SmallVec~StatModifier 4~
        +mesh_asset AssetId
        +socket_name SocketName
    }
    class WeaponZeroing {
        +zero_distance_m u16
        +step_size_m u16
    }
```

## API Design

### Weapon Definition

```rust
/// ECS component: identifies a weapon entity and
/// its configuration.
pub struct Weapon {
    pub definition_id: AssetId,
    pub active_fire_mode: FireModeId,
    pub fire_modes: SmallVec<[FireModeId; 3]>,
}

/// Identifies a weapon definition (data asset).
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct WeaponId(pub u32);

/// Weapon archetype for broad categorization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponCategory {
    /// Close-range: swords, axes, hammers.
    Melee,
    /// Firearms: pistols, rifles, shotguns.
    Ranged,
    /// Thrown: grenades, throwing knives.
    Thrown,
}
```

### Fire Modes

Fire modes determine how trigger input maps to
rounds fired. The fire mode system integrates with
the ability activation system -- each fire mode
is an activation mode variant.

```rust
/// Fire mode type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FireMode {
    /// One round per trigger press.
    SemiAutomatic,
    /// N rounds per trigger press with inter-round
    /// delay.
    Burst,
    /// Continuous fire while trigger is held.
    FullAutomatic,
}

/// Per-mode configuration. Each weapon may support
/// multiple modes, toggled by input action.
pub struct FireModeConfig {
    pub mode: FireMode,
    /// Rounds fired per trigger activation
    /// (1 for semi, N for burst).
    pub rounds_per_activation: u8,
    /// Delay between rounds in burst mode (ms).
    pub inter_round_delay_ms: u16,
    /// Multiplier applied to base spread in this
    /// mode.
    pub spread_modifier: f32,
    /// Multiplier applied to base recoil in this
    /// mode.
    pub recoil_modifier: f32,
    /// Rounds per minute. Determines minimum time
    /// between shots.
    pub fire_rate_rpm: u16,
    /// Spin-up time for gatling-style weapons.
    /// None for instant-fire weapons.
    pub spin_up_time_ms: Option<u16>,
}

/// ECS component: current fire mode state on a
/// weapon entity.
pub struct FireModeState {
    pub config: FireModeConfig,
    /// Ticks until next round can fire.
    pub cooldown_remaining: u32,
    /// Rounds remaining in current burst.
    pub burst_remaining: u8,
    /// Current spin-up progress (0.0 -- 1.0).
    pub spin_up_progress: f32,
}

/// System: processes fire input, validates fire
/// rate and magazine state, consumes ammo, and
/// spawns projectiles.
pub struct FireModeSystem;
```

### Magazine and Ammunition

```rust
/// ECS component: magazine state on a weapon.
pub struct Magazine {
    pub current_rounds: u16,
    pub capacity: u16,
    pub reserve_ammo: u16,
    /// If true, reserve never depletes.
    pub unlimited: bool,
    /// Whether reserve is shared across all
    /// weapons of this ammo type.
    pub shared_reserve: bool,
}

/// Ammo type with distinct gameplay properties.
/// Stored as gameplay database entries.
pub struct AmmoType {
    pub damage_modifier: f32,
    pub armor_penetration: f32,
    pub status_effect: Option<EffectId>,
    pub tracer_color: Color,
    pub muzzle_vfx: AssetId,
    pub ballistic_config: BallisticConfig,
    pub wind_sensitivity: f32,
}

/// ECS component: currently loaded ammo type.
pub struct LoadedAmmo {
    pub ammo_type_id: AmmoTypeId,
}

/// System: manages magazine state -- decrement on
/// fire, replenish on reload from inventory or
/// unlimited pool.
pub struct AmmoSystem;
```

### Reload Mechanics

```rust
/// Reload state on a weapon entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReloadState {
    /// Not reloading.
    Idle,
    /// Magazine has rounds remaining. Faster
    /// animation.
    TacticalReload {
        remaining_ticks: u32,
    },
    /// Magazine is empty. Slower animation
    /// (includes chamber).
    EmptyReload {
        remaining_ticks: u32,
    },
    /// Shell-by-shell loading (shotguns).
    /// Interruptible by fire input.
    SequentialReload {
        shells_loaded: u16,
        per_shell_ticks: u32,
        current_shell_remaining: u32,
    },
}

/// Per-weapon reload timing configuration.
pub struct ReloadConfig {
    pub tactical_duration_ticks: u32,
    pub empty_duration_ticks: u32,
    pub sequential_per_shell_ticks: u32,
    pub tactical_anim: AssetId,
    pub empty_anim: AssetId,
    pub sequential_anim: AssetId,
    pub reload_sound: AssetId,
}

/// System: drives reload state machine. Transitions
/// between Idle, TacticalReload, EmptyReload, and
/// SequentialReload. Cancels sprint on reload start.
/// Fires animation and audio on state entry.
pub struct ReloadSystem;
```

### Recoil and Spread

```rust
/// Per-weapon recoil pattern. A sequence of 2D
/// offsets applied to aim direction during
/// sustained fire. Authored as a curve in the
/// visual editor.
pub struct RecoilPattern {
    /// Sequence of (yaw, pitch) offsets. One entry
    /// per round fired. After all entries, pattern
    /// loops from a configurable index.
    pub curve_points: SmallVec<[Vec2; 32]>,
    /// Speed at which aim recovers toward center
    /// when not firing (degrees per second).
    pub recovery_rate: f32,
    /// Index to loop from after all points are
    /// exhausted.
    pub loop_start_index: u8,
}

/// ECS component: current recoil state.
pub struct RecoilState {
    /// Current index into the recoil pattern.
    pub pattern_index: u8,
    /// Accumulated recoil offset not yet recovered.
    pub accumulated_offset: Vec2,
    /// Whether currently firing (suppresses
    /// recovery).
    pub firing: bool,
}

/// ECS component: current spread state.
pub struct SpreadState {
    /// Base spread angle (degrees) at rest.
    pub base_spread: f32,
    /// Current effective spread (degrees).
    pub current_spread: f32,
    /// Multiplier when aiming down sights.
    pub ads_modifier: f32,
    /// Multiplier when moving.
    pub move_modifier: f32,
    /// Multiplier when crouching.
    pub crouch_modifier: f32,
    /// Multiplier when jumping/airborne.
    pub jump_modifier: f32,
    /// First-shot accuracy. 0.0 = perfect,
    /// 1.0 = full base spread.
    pub first_shot_accuracy: f32,
    /// Spread increase per round fired.
    pub per_shot_increase: f32,
    /// Spread recovery rate (degrees per second).
    pub recovery_rate: f32,
}

/// System: updates RecoilState per fire event,
/// applies recovery between shots, and feeds
/// accumulated offset to the camera system.
pub struct RecoilSystem;

/// System: updates SpreadState based on movement,
/// posture, ADS, and fire rate. Feeds current
/// spread to crosshair UI and projectile spawn
/// direction.
pub struct SpreadSystem;
```

### Ballistics

#### Trajectory Integration

```rust
/// Per-ammo ballistic configuration.
pub struct BallisticConfig {
    /// Initial speed (m/s) at muzzle.
    pub muzzle_velocity: f32,
    /// Projectile mass (kg) for momentum
    /// calculations.
    pub mass: f32,
    /// Aerodynamic drag coefficient.
    pub drag_coefficient: f32,
    /// Sensitivity to wind deflection (0.0 -- 1.0).
    pub wind_sensitivity: f32,
    /// Whether gravity affects this projectile.
    /// Disableable for arcade modes.
    pub gravity_enabled: bool,
    /// Whether drag affects this projectile.
    /// Disableable for arcade modes.
    pub drag_enabled: bool,
}

/// ECS component on projectile entities.
pub struct Projectile {
    pub velocity: Vec3,
    pub config: BallisticConfig,
    pub effect_payload: EffectId,
    pub source_entity: Entity,
    pub distance_traveled: f32,
    pub max_distance: f32,
}

/// System: integrates projectile trajectories each
/// physics tick. Applies gravity (downward
/// acceleration), drag (velocity-dependent
/// deceleration), and wind (lateral force from
/// weather system). Uses the shared spatial index
/// for CCD.
pub struct BallisticSystem;
```

Per-tick trajectory integration (semi-implicit Euler):

```rust
// Pseudocode for one physics tick
fn integrate_projectile(
    proj: &mut Projectile,
    wind: Vec3,
    dt: f32,
) {
    let mut accel = Vec3::ZERO;

    if proj.config.gravity_enabled {
        accel.y -= GRAVITY; // 9.81 m/s^2
    }

    if proj.config.drag_enabled {
        let speed = proj.velocity.length();
        let drag_force = 0.5
            * proj.config.drag_coefficient
            * speed
            * speed;
        let drag_accel = drag_force / proj.config.mass;
        accel -= proj.velocity.normalize() * drag_accel;
    }

    // Wind force proportional to cross-section
    // and wind speed.
    let wind_force = wind
        * proj.config.wind_sensitivity;
    accel += wind_force / proj.config.mass;

    // Semi-implicit Euler: update velocity first,
    // then position.
    proj.velocity += accel * dt;
    let displacement = proj.velocity * dt;
    proj.distance_traveled += displacement.length();
}
```

#### Penetration and Ricochet

```rust
/// Per-weapon/ammo penetration configuration.
pub struct PenetrationConfig {
    /// Maximum depth (cm) projectile can
    /// penetrate.
    pub max_depth: f32,
    /// Energy lost per cm of penetration
    /// (0.0 -- 1.0 fraction of initial).
    pub energy_loss_per_cm: f32,
}

/// Per-material ricochet parameters. Stored on
/// physics material assets.
pub struct RicochetConfig {
    /// Maximum impact angle (from surface) that
    /// allows ricochet. Steeper angles absorb.
    pub angle_threshold_deg: f32,
    /// Material hardness (0.0 -- 1.0). Higher
    /// values increase ricochet probability.
    pub hardness: f32,
    /// Energy retained after ricochet (fraction).
    pub energy_retention: f32,
    /// Random angle deviation after ricochet
    /// (degrees).
    pub angle_randomization_deg: f32,
}

/// Outcome of a projectile-surface interaction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurfaceInteraction {
    /// Projectile passes through with energy loss.
    Penetrated {
        exit_velocity: Vec3,
        remaining_energy: f32,
    },
    /// Projectile bounces off the surface.
    Ricocheted {
        new_velocity: Vec3,
        remaining_energy: f32,
    },
    /// Projectile stops in the surface.
    Absorbed,
}

/// System: evaluates penetration and ricochet on
/// projectile impact. Queries physics material for
/// density and hardness. Computes energy loss and
/// determines outcome.
pub struct PenetrationSystem;
```

#### Weapon Zeroing

```rust
/// ECS component: weapon zeroing state. Only
/// present on weapons with optic attachments.
pub struct WeaponZeroing {
    /// Current zero distance (meters).
    pub zero_distance_m: u16,
    /// Discrete step size for adjustment.
    pub step_size_m: u16,
    /// Minimum zero distance.
    pub min_distance_m: u16,
    /// Maximum zero distance.
    pub max_distance_m: u16,
}

/// System: adjusts aim offset to compensate for
/// bullet drop at the zeroed distance. Recomputes
/// offset when ammo type changes (different
/// ballistic profile shifts effective zero).
pub struct ZeroingSystem;
```

### Attachment System

```rust
/// Named attachment slot on a weapon.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlotName {
    Optic,
    Barrel,
    Muzzle,
    Grip,
    Stock,
    Magazine,
    RailAccessory,
}

/// Category filter for which attachments fit
/// a slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttachmentCategory {
    Scope,
    RedDot,
    Holographic,
    Suppressor,
    Compensator,
    FlashHider,
    VerticalGrip,
    AngledGrip,
    StockStandard,
    StockFolding,
    ExtendedMag,
    DrumMag,
    Laser,
    Flashlight,
}

/// ECS component: a single attachment slot on
/// a weapon entity.
pub struct AttachmentSlot {
    pub slot_name: SlotName,
    pub category_filter: SmallVec<
        [AttachmentCategory; 4]
    >,
    pub equipped: Option<AttachmentId>,
}

/// An attachment that can be equipped.
pub struct Attachment {
    pub category: AttachmentCategory,
    pub stat_modifiers: SmallVec<[StatModifier; 4]>,
    pub mesh_asset: AssetId,
    pub socket_name: SocketName,
}

/// A stat modifier applied by an attachment.
pub struct StatModifier {
    pub stat: WeaponStat,
    pub op: ModOp,
    pub value: f32,
}

/// Weapon stats that attachments can modify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponStat {
    Damage,
    Range,
    RecoilVertical,
    RecoilHorizontal,
    Spread,
    AdsSpeed,
    ReloadSpeed,
    MagazineCapacity,
    MoveSpeed,
    AdsZoom,
    SoundRadius,
}

/// How the modifier is applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModOp {
    /// Add flat value to base stat.
    Flat,
    /// Multiply the base stat.
    Percent,
}

/// System: recalculates weapon stats when
/// attachments are equipped or removed. Applies
/// all StatModifiers from equipped attachments.
pub struct AttachmentStatSystem;

/// System: updates weapon mesh to show/hide
/// attachment meshes at socket transforms when
/// equipped/unequipped. Propagates changes to
/// the render scene.
pub struct AttachmentVisualSystem;
```

### Surface Response

```rust
/// Surface material classification.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub enum SurfaceTypeTag {
    Metal,
    Wood,
    Concrete,
    Dirt,
    Glass,
    Water,
    Flesh,
    Custom(u16),
}

/// Data asset: maps surface types to VFX, audio,
/// and decal responses.
pub struct ImpactResponseTable {
    pub vfx: HashMap<SurfaceTypeTag, VfxResponse>,
    pub audio: HashMap<SurfaceTypeTag, AudioResponse>,
    pub decal: HashMap<SurfaceTypeTag, DecalResponse>,
}

/// VFX response for one surface type.
pub struct VfxResponse {
    pub particle_asset: AssetId,
    pub spawn_count: u8,
    pub scale: f32,
    pub lifetime_ms: u16,
}

/// Audio response for one surface type.
pub struct AudioResponse {
    /// Pool of sound assets for random variation.
    pub sound_pool: SmallVec<[AssetId; 4]>,
    /// Volume scales with impact force.
    pub volume_scale: f32,
    /// Pitch scales with impact force.
    pub pitch_scale: f32,
}

/// Decal response for one surface type.
pub struct DecalResponse {
    pub decal_asset: AssetId,
    pub lifetime_ms: u32,
    pub max_count: u16,
}

/// System: resolves surface type from physics
/// material or terrain splatmap. For terrain,
/// samples the dominant splatmap layer weight
/// at the impact point.
pub struct SurfaceTypeSystem;

/// System: dispatches VFX, audio, and decal
/// responses on projectile or melee impact.
/// Looks up the ImpactResponseTable and spawns
/// the appropriate effects at the impact point
/// oriented along the surface normal.
pub struct ImpactResponseSystem;
```

## Data Flow

### Per-Frame System Execution Order

These systems run after the ability activation
system from the abilities design:

1. **FireModeSystem** -- processes fire input,
   checks fire rate and magazine, consumes ammo,
   emits projectile spawn requests.
2. **ReloadSystem** -- drives reload state machine,
   transitions on input and timer completion.
3. **AmmoSystem** -- replenishes magazine from
   reserves on reload complete, handles ammo
   type switching.
4. **RecoilSystem** -- applies recoil pattern
   offsets on fire, recovers between shots.
5. **SpreadSystem** -- updates spread based on
   movement, posture, ADS, fire rate.
6. **BallisticSystem** -- integrates projectile
   trajectories, runs CCD via shared spatial
   index.
7. **PenetrationSystem** -- evaluates penetration
   and ricochet on impact.
8. **ZeroingSystem** -- adjusts aim offset for
   bullet drop at zeroed distance.
9. **AttachmentStatSystem** -- recalculates weapon
   stats from equipped attachments.
10. **AttachmentVisualSystem** -- updates
    attachment meshes on equip/unequip.
11. **SurfaceTypeSystem** -- resolves surface
    material at impact points.
12. **ImpactResponseSystem** -- spawns VFX, audio,
    and decals based on surface type.

### Ballistic Pipeline

For each in-flight projectile per physics tick:

1. Read `BallisticConfig` from the projectile.
2. Read wind vector from the weather system.
3. Integrate velocity: gravity + drag + wind.
4. Compute displacement for this tick.
5. CCD sweep from old position to new position
   via shared spatial index.
6. If no hit: update position, continue.
7. If hit:
   - Query surface material from physics material.
   - Evaluate penetration/ricochet:
     - **Penetrate**: reduce energy, continue with
       exit velocity.
     - **Ricochet**: reflect, reduce energy, continue.
     - **Absorb**: apply damage effect, dispatch
       surface response, destroy projectile entity.

### Attachment Stat Aggregation

When an attachment is equipped or removed:

1. Collect all `StatModifier` entries from all
   equipped `AttachmentSlot` components.
2. Group modifiers by `WeaponStat`.
3. Apply flat modifiers first (sum).
4. Apply percent modifiers second (product).
5. Write final stat values to the weapon's
   derived stat cache.
6. Notify dependent systems (spread uses
   spread modifier, recoil uses recoil
   modifier, etc.).

### Recoil Pattern Playback

Per round fired:

1. Read the next `Vec2` from `RecoilPattern`
   at `pattern_index`.
2. Multiply by the fire mode's `recoil_modifier`.
3. Multiply by attachment recoil modifiers.
4. Add to `accumulated_offset`.
5. Feed `accumulated_offset` to the camera system
   as an additive aim rotation.
6. Advance `pattern_index`. If past the end,
   loop from `loop_start_index`.

Between rounds (recovery):

1. Lerp `accumulated_offset` toward zero at
   `recovery_rate` per second.
2. Stop recovery while `firing` is true.

## Platform Considerations

| Component | Platform Impact | Notes |
|-----------|----------------|-------|
| Ballistic CCD | All platforms | Uses shared spatial index BVH. 256 projectiles must complete under 1 ms (NFR-13.16.1). SIMD-accelerated sweep. |
| Recoil patterns | All platforms | 32-entry SmallVec fits in 256 bytes -- single cache line reads per round fired. |
| Surface type resolve | All platforms | Physics materials carry tag directly. Terrain splatmap resolve requires bilinear sample of weight textures -- CPU-side for hit detection. |
| Wind vector | All platforms | Read from weather system singleton resource. Single Vec3 read per projectile per tick. |
| Attachment visuals | All platforms | Mesh swap propagated to render scene via ECS-to-Renderer bridge. One-time cost on equip/unequip, not per-frame. |
| Weapon customization UI | Editor + runtime | 3D weapon preview renders to an offscreen render target composited into the UI layer. |
| Impact VFX/audio/decal | All platforms | Budget-managed: decals capped by `max_count`, oldest evicted. VFX spawn count configurable per surface type. |

## Test Plan

### Unit Tests

| Test | Req | Description |
|------|-----|-------------|
| `test_semi_auto_one_round` | R-13.16.1 | Fire semi-auto weapon. Verify exactly 1 round per press. |
| `test_burst_n_rounds` | R-13.16.1 | Fire burst-mode weapon (N=3). Verify exactly 3 rounds with inter-round delay. |
| `test_full_auto_continuous` | R-13.16.1 | Hold fire on full-auto. Verify continuous fire at configured RPM. |
| `test_fire_mode_toggle` | R-13.16.1 | Toggle between semi, burst, auto. Verify spread and recoil modifiers change per mode. |
| `test_spin_up_gatling` | R-13.16.1 | Fire gatling weapon. Verify spin-up delay before first round, then full RPM. |
| `test_magazine_decrement` | R-13.16.2a | Fire one round. Verify magazine count decrements by 1. |
| `test_empty_magazine_blocks_fire` | R-13.16.2a | Empty magazine. Verify fire input suppressed. |
| `test_unlimited_ammo` | R-13.16.2a | Enable unlimited mode. Verify reserve never depletes. |
| `test_tactical_reload_faster` | R-13.16.2b | Reload with rounds remaining. Verify faster duration than empty reload. |
| `test_empty_reload_slower` | R-13.16.2b | Reload with empty magazine. Verify slower duration. |
| `test_sequential_reload_interrupt` | R-13.16.2b | Load 3 shells, interrupt with fire. Verify 3 rounds in magazine and weapon fires. |
| `test_reload_cancels_sprint` | R-13.16.2b | Sprint, then reload. Verify sprint cancelled. |
| `test_ammo_type_switch` | R-13.16.2c | Switch from standard to AP ammo. Verify damage modifier and penetration change. |
| `test_incendiary_status_effect` | R-13.16.2c | Fire incendiary round. Verify burn DoT applied on hit. |
| `test_tracer_color_change` | R-13.16.2c | Switch ammo type. Verify tracer color updates. |
| `test_recoil_pattern_deterministic` | R-13.16.3 | Fire 10 rounds. Verify aim offset matches pattern curve exactly. |
| `test_recoil_recovery` | R-13.16.3 | Fire, stop, wait. Verify offset recovers toward zero. |
| `test_spread_ads_tighter` | R-13.16.3 | Compare spread ADS vs hip-fire. Verify ADS is tighter by ads_modifier. |
| `test_spread_movement_wider` | R-13.16.3 | Compare spread stationary vs moving. Verify moving is wider. |
| `test_first_shot_accuracy` | R-13.16.3 | Fire first shot with sniper (accuracy 0.0). Verify zero spread. |
| `test_crosshair_reflects_spread` | R-13.16.3 | Fire sustained. Verify crosshair radius matches current_spread. |
| `test_bullet_drop_parabolic` | R-13.16.4a | Fire at 500 m. Verify drop matches expected parabola within 5%. |
| `test_drag_decelerates` | R-13.16.4a | Fire and measure velocity at 100 m vs 500 m. Verify deceleration from drag. |
| `test_gravity_disable_straight` | R-13.16.4a | Disable gravity. Verify projectile travels in a straight line. |
| `test_travel_time_matches_velocity` | R-13.16.4a | Measure arrival time vs expected from muzzle velocity and drag. |
| `test_wind_lateral_deflection` | R-13.16.4b | Fire perpendicular to wind. Verify lateral offset at distance matches expected. |
| `test_wind_disable_no_deflection` | R-13.16.4b | Disable wind. Verify no lateral deflection. |
| `test_penetration_drywall` | R-13.16.4c | Fire through drywall. Verify pass-through with reduced velocity. |
| `test_penetration_concrete_stops` | R-13.16.4c | Fire at concrete. Verify projectile stops. |
| `test_ricochet_shallow_angle` | R-13.16.4c | Fire at metal at shallow angle. Verify ricochet. |
| `test_ricochet_steep_absorbs` | R-13.16.4c | Fire at metal at steep angle. Verify absorption (no ricochet). |
| `test_zeroing_300m_on_target` | R-13.16.4d | Zero to 300 m, fire at 300 m target. Verify point-of-aim matches point-of-impact. |
| `test_zeroing_wrong_distance` | R-13.16.4d | Zero to 300 m, fire at 100 m. Verify round impacts high. |
| `test_zeroing_ammo_shift` | R-13.16.4d | Switch ammo type with different ballistics. Verify effective zero shifts. |
| `test_attachment_equip_stat` | R-13.16.5a | Equip suppressor. Verify sound and range decrease by configured amounts. |
| `test_attachment_category_filter` | R-13.16.5a | Attempt to equip optic in grip slot. Verify rejection. |
| `test_attachment_visual_snap` | R-13.16.5b | Equip attachment. Verify mesh appears at socket transform. |
| `test_attachment_unequip_visual` | R-13.16.5b | Remove attachment. Verify mesh disappears. |
| `test_optic_reticle_render` | R-13.16.5b | Equip optic. Verify reticle renders at configured zoom. |
| `test_surface_tag_metal` | R-13.16.6a | Raycast metal surface. Verify returns Metal tag. |
| `test_surface_tag_terrain` | R-13.16.6a | Raycast terrain. Verify tag matches dominant splatmap layer. |
| `test_impact_vfx_metal_sparks` | R-13.16.6b | Shoot metal. Verify sparks VFX spawns from table. |
| `test_impact_audio_wood_thud` | R-13.16.6c | Shoot wood. Verify thud sound plays. |
| `test_impact_audio_variation` | R-13.16.6c | Shoot same surface 10 times. Verify random variation from pool. |
| `test_impact_decal_placement` | R-13.16.6d | Shoot surface. Verify decal placed at impact point along normal. |
| `test_decal_max_count_eviction` | R-13.16.6d | Exceed max decal count. Verify oldest decal removed. |

### Integration Tests

| Test | Req | Description |
|------|-----|-------------|
| `test_fire_full_pipeline` | R-13.16.1, NFR-13.16.2 | Press fire: verify ammo consumed, recoil applied, projectile spawned, muzzle VFX plays -- all within 1 frame. |
| `test_projectile_impact_full` | R-13.16.4a, R-13.16.6b -- 6d | Projectile hits surface: verify damage effect applied, VFX spawns, audio plays, decal placed. |
| `test_penetration_chain` | R-13.16.4c | Fire through drywall, hit target behind. Verify damage applied with reduced energy. |
| `test_ricochet_kills` | R-13.16.4c | Ricochet off metal into target. Verify damage applied with reduced energy. |
| `test_attachment_full_cycle` | R-13.16.5a -- 5c | Equip attachment in UI, verify stat change, verify mesh appears, verify UI stat comparison. |
| `test_256_projectiles_perf` | NFR-13.16.1 | Spawn 256 simultaneous projectiles with full ballistics. Verify under 1 ms per physics tick. |
| `test_weapon_feedback_latency` | NFR-13.16.2 | Measure input-to-muzzle-flash time. Verify under 16 ms across 100 fires. |

### Benchmarks

| Benchmark | Target | Source |
|-----------|--------|--------|
| Ballistic simulation (256 projectiles) | < 1 ms per tick | NFR-13.16.1 |
| Per-projectile penetration/ricochet | < 0.01 ms | NFR-13.16.1 |
| Input-to-muzzle-flash latency | < 16 ms | NFR-13.16.2 |
| Reload state transition | < 0.01 ms | -- |
| Recoil pattern lookup + apply | < 0.001 ms | -- |
| Spread recalculation | < 0.001 ms | -- |
| Attachment stat aggregation | < 0.05 ms | -- |
| Surface type resolve (terrain splatmap) | < 0.01 ms | -- |
| Impact response dispatch (VFX + audio + decal) | < 0.1 ms | -- |

## Open Questions

1. **Hitscan vs projectile threshold** -- At what muzzle
   velocity should the system switch from simulated
   projectiles to instant raycasts? Very fast rounds
   (e.g., 1000+ m/s) travel so far per tick that CCD
   becomes a long sweep. A hybrid approach (raycast for
   supersonic, simulated for subsonic) may be optimal.
2. **Penetration energy model** -- The current linear
   energy-loss-per-cm model is simple but unrealistic.
   Real penetration follows a non-linear curve based on
   material density and projectile deformation. Determine
   whether a lookup table per material pair is needed for
   acceptable results.
3. **Recoil pattern randomization** -- Should recoil
   patterns be fully deterministic (learnable, competitive)
   or include a random component (more realistic, less
   exploitable)? This is a game-design-level decision that
   affects API shape -- deterministic needs only pattern
   index, random needs a per-shot RNG seed.
4. **Attachment compatibility matrix** -- The current design
   uses category filters per slot. Some games have
   per-weapon attachment whitelists (only certain scopes
   fit certain rifles). Determine whether per-weapon
   compatibility data is needed beyond category filtering.
5. **Surface response budget** -- Impact VFX, audio, and
   decals can accumulate rapidly during heavy firefights.
   Define per-frame budgets for each (max VFX spawns per
   frame, max audio voices, max decal placements) and
   prioritization strategy (nearest to camera first).
6. **Projectile pooling** -- With up to 256 simultaneous
   projectiles, entity creation/destruction overhead may
   matter. Consider an ECS entity pool for projectiles
   to avoid archetype table insertions on every shot.
7. **Scope rendering** -- Optic scope rendering can use
   either a render-to-texture approach (realistic but
   expensive -- second camera) or a shader overlay
   (cheaper, less realistic). Determine which approach
   aligns with performance targets.
