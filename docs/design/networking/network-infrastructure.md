# Network Infrastructure Design

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are defined in
> [features/](../../features/), [requirements/](../../requirements/), and
> [user-stories/](../../user-stories/). The table below traces design elements to those definitions.

### MMO Infrastructure

| Feature | Requirement |
|---------|-------------|
| F-8.7.1 | R-8.7.1     |
| F-8.7.2 | R-8.7.2     |
| F-8.7.3 | R-8.7.3     |
| F-8.7.4 | R-8.7.4     |
| F-8.7.5 | R-8.7.5     |
| F-8.7.6 | R-8.7.6     |
| F-8.7.7 | R-8.7.7     |
| F-8.7.8 | R-8.7.8     |

1. **F-8.7.1** -- World sharding and instancing
2. **F-8.7.2** -- Seamless zone transitions
3. **F-8.7.3** -- Dynamic server mesh
4. **F-8.7.4** -- Player migration between servers (< 100 ms)
5. **F-8.7.5** -- Persistent world state and database
6. **F-8.7.6** -- Load balancing and auto-scaling
7. **F-8.7.7** -- Cross-shard services (auction, mail, chat)
8. **F-8.7.8** -- Inter-server communication bus

### Anti-Cheat and Security

| Feature | Requirement |
|---------|-------------|
| F-8.8.1 | R-8.8.1     |
| F-8.8.2 | R-8.8.2     |
| F-8.8.3 | R-8.8.3     |
| F-8.8.4 | R-8.8.4     |
| F-8.8.5 | R-8.8.5     |

1. **F-8.8.1** -- Server-side cheat detection (movement, damage, cooldowns)
2. **F-8.8.2** -- Client integrity verification (memory hashing)
3. **F-8.8.3** -- Behavioral analysis and anomaly detection (Z-score)
4. **F-8.8.4** -- Economy exploit prevention (double-spend, gold farming)
5. **F-8.8.5** -- Rate limiting and abuse prevention (per-RPC budgets)

## Overview

This design covers the server-side architecture for persistent, massively multiplayer worlds and the
layered anti-cheat security system that protects them.

### MMO Infrastructure

The MMO subsystem partitions the game world into shards and zones, runs a dynamic server mesh that
scales spatially based on entity density, migrates players seamlessly between zone servers, persists
world state through an async database layer, and provides cross-shard services for economy and
social features.

All components are ECS-primary (~90%)-based. Zone servers run the full ECS simulation in headless
mode. The server mesh controller, cross-shard services, and inter-server bus run as independent
microservices on self-hosted AWS infrastructure (Kubernetes). All I/O is async.

### Anti-Cheat

The primary defense is server-side validation: the server independently simulates all game logic and
compares client state against computed bounds. Secondary defenses include client integrity
verification, statistical behavioral analysis, economy validation, and rate limiting.

All anti-cheat logic runs as ECS systems on the server. Violation scoring, escalation, and rate
limiting are components attached to player entities. Detection thresholds account for RTT and
platform-specific input characteristics. Configurable severity tiers (warn, flag, kick, ban) with
hot-reloadable config.

## Architecture

### MMO Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_net::mmo"
        MC[MeshController]
        ZS1[ZoneServer A]
        ZS2[ZoneServer B]
        ZS3[ZoneServer C]
        MIG[MigrationService]
        BUS[InterServerBus]
    end

    subgraph "harmonius_net::persistence"
        DAL[DatabaseAccessLayer]
        TXN[TransactionManager]
    end

    subgraph "harmonius_net::cross_shard"
        AH[AuctionService]
        MAIL[MailService]
        GF[GroupFinder]
        CHAT[GlobalChat]
    end

    subgraph "AWS Infrastructure"
        K8S[Kubernetes]
        AS[AutoScaler]
        LB[LoadBalancer]
        DB[(PostgreSQL)]
    end

    MC --> ZS1
    MC --> ZS2
    MC --> ZS3
    MC --> MIG
    ZS1 <--> BUS
    ZS2 <--> BUS
    ZS3 <--> BUS
    BUS --> AH
    BUS --> MAIL
    DAL --> DB
    TXN --> DAL
    K8S --> MC
    AS --> K8S
    LB --> ZS1
    LB --> ZS2
```

### Anti-Cheat Validation Pipeline

```mermaid
graph TD
    subgraph "harmonius_net::anti_cheat"
        MV[MovementValidator]
        DV[DamageValidator]
        EV[EconomyValidator]
        CI[ClientIntegrityChecker]
        BA[BehavioralAnalyzer]
        RL[RateLimiter]
        VS[ViolationScorer]
        ESC[EscalationManager]
    end

    subgraph "harmonius_net::session"
        GS[GameServer]
        AUTH[AuthService]
    end

    GS --> MV
    GS --> DV
    GS --> EV
    GS --> RL
    MV --> VS
    DV --> VS
    EV --> VS
    CI --> VS
    BA --> VS
    RL --> VS
    VS --> ESC
    ESC -->|kick| GS
    ESC -->|ban| AUTH
```

### File Layout

```text
harmonius_net/
+-- mmo/
|   +-- shard.rs         # ShardManager, ShardId
|   +-- zone.rs          # ZoneServer, ZoneId
|   +-- mesh.rs          # MeshController, spatial split
|   +-- migration.rs     # MigrationService, player handoff
|   +-- instance.rs      # InstanceManager, dungeons
|   +-- overlap.rs       # BoundaryOverlap, co-sim
|   +-- scaler.rs        # AutoScaler, load monitoring
+-- persistence/
|   +-- dal.rs           # DatabaseAccessLayer, async query
|   +-- transaction.rs   # TransactionManager, atomics
|   +-- schema.rs        # Table definitions, migrations
|   +-- pool.rs          # ConnectionPool, async pooling
+-- cross_shard/
|   +-- auction.rs       # AuctionService, bid/buyout
|   +-- mail.rs          # MailService, attachments
|   +-- group_finder.rs  # GroupFinder, cross-shard match
|   +-- chat.rs          # GlobalChat, channels
|   +-- guild.rs         # GuildService, roster
+-- bus/
|   +-- transport.rs     # TCP connections, auto-reconnect
|   +-- channel.rs       # PubSub + point-to-point routing
|   +-- delivery.rs      # Delivery guarantees
|   +-- codec.rs         # Typed message serialization
+-- anti_cheat/
    +-- movement.rs      # MovementValidator, speed/teleport
    +-- damage.rs        # DamageValidator, bounds checking
    +-- economy.rs       # EconomyValidator, transaction
    +-- integrity.rs     # ClientIntegrityChecker, hash
    +-- behavioral.rs    # BehavioralAnalyzer, Z-score
    +-- rate_limit.rs    # RateLimiter, token buckets
    +-- scorer.rs        # ViolationScorer, accumulation
    +-- escalation.rs    # EscalationManager, tiers
    +-- config.rs        # Hot-reloadable thresholds
```

### Zone Transition Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant ZA as ZoneServer A
    participant MC as MeshController
    participant ZB as ZoneServer B

    Note over C,ZA: Player approaches zone boundary
    ZA->>MC: request_handoff(player_id, zone_b)
    MC->>ZB: prepare_receive(player_id)
    ZB-->>MC: ready

    ZA->>ZB: transfer(entity, buffs, rpcs, inputs)
    ZA->>C: redirect(zone_b_endpoint)
    C->>ZB: reconnect(session_token)
    ZB-->>C: snapshot(full_state)
    Note over C,ZB: Seamless transition complete
    ZA->>ZA: cleanup player state
```

### Dynamic Mesh Split/Merge

```mermaid
sequenceDiagram
    participant MC as MeshController
    participant ZA as ZoneServer A
    participant K8S as Kubernetes
    participant ZB as ZoneServer B

    Note over ZA: Entity density exceeds threshold
    ZA->>MC: report_overload(region, load)
    MC->>MC: compute split boundary
    MC->>K8S: provision_server()
    K8S-->>MC: ZoneServer B ready
    MC->>ZA: begin_split(boundary)
    ZA->>ZB: transfer_entities(region_b_entities)
    MC->>MC: update routing table

    Note over ZA,ZB: Load subsides
    MC->>ZA: begin_merge()
    ZB->>ZA: transfer_entities(all)
    MC->>K8S: terminate_server(ZB)
```

### Player Migration Handoff

```mermaid
sequenceDiagram
    participant C as Client
    participant SRC as Source Server
    participant MIG as MigrationService
    participant DST as Destination Server

    SRC->>MIG: initiate(player_id, dst)
    MIG->>SRC: freeze_player(player_id)
    Note over C: Client extrapolates

    SRC->>MIG: serialize(entity, buffs, cooldowns)
    MIG->>DST: prepare(serialized_state)
    DST-->>MIG: ready
    MIG->>C: redirect(dst_endpoint)
    C->>DST: reconnect(session_token)
    DST-->>C: snapshot + correction
    Note over C,DST: Handoff under 100 ms
```

### Persistence Layer

```mermaid
sequenceDiagram
    participant GS as Game Server
    participant DAL as DatabaseAccessLayer
    participant TXN as TransactionManager
    participant DB as PostgreSQL

    GS->>DAL: save_character(char_data).await
    DAL->>DB: async INSERT/UPDATE
    DB-->>DAL: ok
    DAL-->>GS: saved

    Note over GS: Trade request (atomic)
    GS->>TXN: begin()
    GS->>TXN: debit(player_a, gold)
    GS->>TXN: credit(player_b, gold)
    GS->>TXN: transfer_item(a_to_b, item)
    GS->>TXN: commit().await
    TXN->>DB: COMMIT
```

### Inter-Server Bus Topology

```mermaid
graph TD
    subgraph "Pub/Sub Channels"
        WE[World Events]
        GC[Global Chat]
        TM[Telemetry]
    end

    subgraph "Point-to-Point"
        MR[Migration Requests]
        SR[Shard Routing]
    end

    ZS1[ZoneServer 1] --> WE
    ZS2[ZoneServer 2] --> WE
    WE --> ZS1
    WE --> ZS2

    ZS1 -->|direct| MR
    MR -->|direct| ZS2

    MC_[MeshController] --> SR
    SR --> ZS1
    SR --> ZS2
```

### Server-Side Validation Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant GS as Game Server
    participant MV as MovementValidator
    participant VS as ViolationScorer
    participant ESC as EscalationManager

    C->>GS: input(move_forward, dt)
    GS->>GS: simulate authoritative state
    GS->>MV: validate(client_pos, server_pos, rtt)

    alt Position within bounds
        MV-->>GS: ok
    else Position exceeds bounds
        MV->>VS: report(speed_hack, severity)
        VS->>ESC: check_threshold(player_id)
        alt Score exceeds tier
            ESC-->>GS: kick
            GS-->>C: disconnect(reason)
        else Below threshold
            ESC-->>GS: warn
            GS-->>C: correction(server_pos)
        end
    end
```

### Client Integrity Challenge

```mermaid
sequenceDiagram
    participant GS as Game Server
    participant CI as IntegrityChecker
    participant C as Client
    participant VS as ViolationScorer

    GS->>CI: generate_challenge()
    CI->>CI: select random memory regions
    CI->>C: send_challenge(encrypted_regions)
    C->>C: hash requested memory regions
    C-->>CI: response(hashed_snapshots)

    alt Hashes match known-good
        CI-->>GS: integrity_ok
    else Hash mismatch
        CI->>VS: report(tampered_binary, high)
    end
```

### Economy Validation Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant GS as Game Server
    participant EV as EconomyValidator
    participant TXN as TransactionManager
    participant VS as ViolationScorer

    C->>GS: craft_item(recipe_id, ingredients)
    GS->>EV: validate_craft(player, recipe)
    EV->>EV: check recipe, ingredients, cooldowns

    alt Valid
        EV->>TXN: begin()
        TXN->>TXN: remove ingredients, create item
        TXN->>TXN: commit()
        EV-->>GS: craft_ok(item_id)
    else Invalid
        EV->>VS: report(economy_exploit, severity)
        EV-->>GS: reject(reason)
    end
```

### Core Data Structures

```mermaid
classDiagram
    class ShardId {
        +id: u32
    }

    class ZoneId {
        +shard: ShardId
        +zone: u32
    }

    class ServerId {
        +u64 value
    }

    class SpatialRegion {
        +min: Vec3
        +max: Vec3
        +owner_server: ServerId
        +entity_count: u32
        +cpu_load: f32
        +contains(pos) bool
        +split(axis) SpatialRegion, SpatialRegion
    }

    class MeshController {
        -regions: Vec~SpatialRegion~
        -routing_table: HashMap~ZoneId, ServerId~
        -split_threshold: f32
        -merge_threshold: f32
        +evaluate_load()
        +split_region(region_id)
        +merge_regions(a, b)
        +route_player(pos) ServerId
    }

    class MigrationPayload {
        +player_id: AccountId
        +entity_snapshot: Vec~u8~
        +active_buffs: Vec~BuffState~
        +cooldown_timers: Vec~CooldownState~
        +pending_rpcs: Vec~u8~
        +prediction_history: Vec~InputFrame~
    }

    class DatabaseAccessLayer {
        -pool: ConnectionPool
        +query~T~(sql, params) Future~Vec~T~~
        +execute(sql, params) Future~u64~
        +transaction() TransactionHandle
    }

    class InterServerBus {
        -connections: HashMap~ServerId, TcpStream~
        +publish(channel, msg)
        +send(target, msg)
        +subscribe(channel, handler)
    }

    class ShardAssignment {
        <<enumeration>>
        LeastPopulated
        Specific
        SameAs
    }

    class ShardState {
        <<enumeration>>
        Active
        Draining
        Maintenance
        Merging
    }

    class InstanceDifficulty {
        <<enumeration>>
        Normal
        Heroic
        Mythic
    }

    class DeliveryGuarantee {
        <<enumeration>>
        AtMostOnce
        AtLeastOnce
        ExactlyOnce
    }

    class ViolationType {
        <<enumeration>>
        SpeedHack
        Teleport
        DamageManipulation
        CooldownCircumvention
        InventoryExploit
        EconomyExploit
        DoubleSpend
        GoldFarming
        TamperedBinary
        BehavioralAnomaly
        RateLimitExceeded
    }

    class EscalationAction {
        <<enumeration>>
        Warn
        Flag
        Kick
        TempBan
        PermaBan
    }

    class ViolationRecord {
        +violation_type: ViolationType
        +severity: f32
        +timestamp: u64
        +details: String
    }

    class ViolationScorer {
        -scores: HashMap~AccountId, PlayerScore~
        -decay_rate: f32
        +report(account, violation)
        +current_score(account) f32
        +decay_scores(dt)
    }

    class PlayerScore {
        +total_score: f32
        +violations: Vec~ViolationRecord~
        +last_decay: u64
    }

    class EscalationTier {
        +threshold: f32
        +action: EscalationAction
        +cooldown_seconds: u32
    }

    class RateLimitBucket {
        +rpc_type: u32
        +tokens: f32
        +max_tokens: f32
        +refill_rate: f32
        +burst_allowance: u32
        +consume() bool
        +refill(dt)
    }

    class BehavioralBaseline {
        +account_id: AccountId
        +input_type: InputType
        +aim_accuracy: RunningStats
        +reaction_time: RunningStats
        +movement_entropy: RunningStats
        +sample_count: u32
    }

    class RunningStats {
        +mean: f64
        +variance: f64
        +count: u64
        +push(value)
        +z_score(value) f64
    }

    class InputType {
        <<enumeration>>
        Touch
        Controller
        KeyboardMouse
    }

    class AutoScaler {
        -config: ScalerConfig
        +evaluate(metrics) Vec~ScaleAction~
        +provision() Future~ServerId~
        +drain_and_terminate(id) Future~Result~
    }

    class ServerMetrics {
        +server_id: ServerId
        +cpu_percent: f32
        +memory_mb: u32
        +player_count: u32
        +tick_time_ms: f32
    }

    class LoadBalancer {
        +select(zone_id) Result~ServerId~
        +report_metrics(metrics)
        +set_draining(server_id)
    }

    class AntiCheatConfig {
        +movement: MovementConfig
        +damage: DamageConfig
        +economy: EconomyConfig
        +integrity: IntegrityConfig
        +behavioral: BehavioralConfig
        +rate_limit: RateLimitConfig
        +scorer: ScorerConfig
        +escalation: EscalationConfig
    }

    MeshController *-- SpatialRegion
    MeshController --> MigrationPayload
    MeshController --> InterServerBus
    DatabaseAccessLayer --> ConnectionPool
    ViolationScorer *-- PlayerScore
    PlayerScore *-- ViolationRecord
    ViolationRecord --> ViolationType
    EscalationTier --> EscalationAction
    BehavioralBaseline *-- RunningStats
    BehavioralBaseline --> InputType
    AutoScaler --> ServerMetrics
    AntiCheatConfig --> MovementConfig
    AntiCheatConfig --> EscalationConfig
```

## API Design

### Shard and Zone Types

```rust
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct ShardId(pub u32);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct ZoneId {
    pub shard: ShardId,
    pub zone: u32,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct ServerId(pub u64);

#[derive(Clone, Debug, Reflect)]
pub enum ShardAssignment {
    LeastPopulated,
    Specific(ShardId),
    SameAs(AccountId),
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum ShardState {
    Active,
    Draining,
    Maintenance,
    Merging,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum InstanceDifficulty {
    Normal,
    Heroic,
    Mythic,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct InstanceId(pub u64);
```

### Spatial Region and Mesh

```rust
/// Axis-aligned spatial region owned by a server.
pub struct SpatialRegion {
    pub region_id: u32,
    pub min: Vec3,
    pub max: Vec3,
    pub owner: ServerId,
    pub entity_count: u32,
    pub cpu_load: f32,
}

impl SpatialRegion {
    pub fn contains(&self, pos: Vec3) -> bool;
    pub fn split(
        &self,
    ) -> (SpatialRegion, SpatialRegion);
    pub fn area(&self) -> f32;
    pub fn in_overlap(
        &self,
        pos: Vec3,
        overlap_width: f32,
    ) -> bool;
}

pub struct MeshConfig {
    pub split_threshold: f32,
    pub merge_threshold: f32,
    pub overlap_width: f32,
    pub eval_interval_seconds: u32,
}
```

### Migration Types

```rust
pub struct MigrationPayload {
    pub account_id: AccountId,
    pub session_id: SessionId,
    pub entity_snapshot: Vec<u8>,
    pub active_buffs: Vec<BuffState>,
    pub cooldown_timers: Vec<CooldownState>,
    pub pending_rpcs: Vec<u8>,
    pub prediction_history: Vec<InputFrame>,
    pub initiated_at: u64,
}

pub struct BuffState {
    pub buff_id: u32,
    pub remaining_ticks: u32,
    pub stacks: u8,
    pub source_entity: Option<Entity>,
}

pub struct CooldownState {
    pub ability_id: u32,
    pub remaining_ticks: u32,
}

pub enum MigrationError {
    DestinationUnavailable,
    SerializationFailed,
    Timeout,
    StateMismatch,
}
```

### Database Types

```rust
pub struct PoolConfig {
    pub connection_string: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub idle_timeout_seconds: u32,
}

pub struct PoolStats {
    pub active_connections: u32,
    pub idle_connections: u32,
    pub pending_queries: u32,
    pub total_queries: u64,
}

pub enum DbError {
    ConnectionFailed,
    QueryFailed { message: String },
    TransactionConflict,
    PoolExhausted,
    Timeout,
}
```

### Inter-Server Bus Types

```rust
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum DeliveryGuarantee {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

#[derive(Clone, Debug, Reflect)]
pub struct BusChannel {
    pub name: String,
    pub guarantee: DeliveryGuarantee,
}

pub struct BusMessage {
    pub channel: String,
    pub source: ServerId,
    pub target: Option<ServerId>,
    pub sequence: u64,
    pub payload: Vec<u8>,
}

pub enum BusError {
    ConnectionFailed,
    PeerNotFound,
    ChannelNotFound,
    SerializationFailed,
    Timeout,
}
```

### Auto-Scaling Types

```rust
pub struct ScalerConfig {
    pub scale_up_cpu: f32,
    pub scale_down_cpu: f32,
    pub scale_up_players: u32,
    pub min_servers: u32,
    pub max_servers: u32,
    pub cooldown_seconds: u32,
}

pub struct ServerMetrics {
    pub server_id: ServerId,
    pub cpu_percent: f32,
    pub memory_mb: u32,
    pub player_count: u32,
    pub network_mbps: f32,
    pub tick_time_ms: f32,
}

#[derive(Clone, Debug, Reflect)]
pub enum ScaleAction {
    Provision { count: u32 },
    Drain { server_id: ServerId },
    None,
}

pub enum ScaleError {
    AtMaxCapacity,
    AtMinCapacity,
    ProvisionTimeout,
    DrainTimeout,
}
```

### Anti-Cheat Types

```rust
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub enum ViolationType {
    SpeedHack,
    Teleport,
    DamageManipulation,
    CooldownCircumvention,
    InventoryExploit,
    EconomyExploit,
    DoubleSpend,
    GoldFarming,
    TamperedBinary,
    BehavioralAnomaly,
    RateLimitExceeded,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    PartialOrd, Ord, Reflect,
)]
pub enum EscalationAction {
    Warn,
    Flag,
    Kick,
    TempBan { hours: u32 },
    PermaBan,
}

pub struct ViolationRecord {
    pub violation_type: ViolationType,
    pub severity: f32,
    pub timestamp: u64,
    pub details: String,
}

pub struct PlayerScore {
    pub total_score: f32,
    pub violations: Vec<ViolationRecord>,
    pub last_decay: u64,
}

pub struct MovementConfig {
    pub max_speed: f32,
    pub max_delta_per_tick: f32,
    pub rtt_tolerance: f32,
    pub mobile_tolerance_multiplier: f32,
}

pub struct DamageConfig {
    pub tolerance_multiplier: f32,
    pub min_damage_interval: u32,
}

pub struct EconomyConfig {
    pub max_transfer: u64,
    pub rate_limit_per_hour: u32,
    pub high_value_threshold: u64,
    pub high_value_delay: u32,
}

pub struct IntegrityConfig {
    pub challenge_interval_seconds: u32,
    pub regions_per_challenge: u32,
    pub response_timeout_seconds: u32,
}

pub struct BehavioralConfig {
    pub z_score_threshold: f64,
    pub min_samples: u32,
    pub eval_interval: u32,
}

/// Running statistics (Welford's algorithm).
pub struct RunningStats {
    pub mean: f64,
    pub variance: f64,
    pub count: u64,
}

impl RunningStats {
    pub fn push(&mut self, value: f64);
    pub fn z_score(&self, value: f64) -> f64;
    pub fn std_dev(&self) -> f64;
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub enum InputType {
    Touch,
    Controller,
    KeyboardMouse,
}

pub struct BehavioralBaseline {
    pub account_id: AccountId,
    pub input_type: InputType,
    pub aim_accuracy: RunningStats,
    pub reaction_time: RunningStats,
    pub movement_entropy: RunningStats,
    pub resource_acquisition_rate: RunningStats,
    pub sample_count: u32,
}

pub struct TokenBucket {
    pub tokens: f32,
    pub max_tokens: f32,
    pub refill_rate: f32,
    pub burst_allowance: u32,
    pub burst_count: u32,
}

impl TokenBucket {
    pub fn consume(&mut self) -> bool;
    pub fn refill(&mut self, dt: f32);
    pub fn burst_exceeded(&self) -> bool;
}

pub struct RateLimitRule {
    pub rpc_type: u32,
    pub calls_per_second: f32,
    pub burst_allowance: u32,
    pub cooldown_seconds: f32,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum RateLimitResult {
    Allow,
    Throttle { delay_ms: u32 },
    Reject,
}

pub struct TransactionRecord {
    pub timestamp: u64,
    pub transaction_type: TransactionType,
    pub amount: u64,
    pub counterparty: Option<AccountId>,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum TransactionType {
    Trade,
    AuctionSale,
    CraftingResult,
    LootDrop,
    QuestReward,
    MailAttachment,
}

/// Top-level hot-reloadable anti-cheat config.
pub struct AntiCheatConfig {
    pub movement: MovementConfig,
    pub damage: DamageConfig,
    pub economy: EconomyConfig,
    pub integrity: IntegrityConfig,
    pub behavioral: BehavioralConfig,
    pub rate_limit: RateLimitConfig,
    pub scorer: ScorerConfig,
    pub escalation: EscalationConfig,
}

pub struct MatchMetrics {
    pub aim_accuracy: f64,
    pub reaction_time_ms: f64,
    pub movement_entropy: f64,
    pub resource_acquired: f64,
    pub kills: u32,
    pub deaths: u32,
    pub damage_dealt: f64,
}

pub struct WeaponStats {
    pub base_damage: f32,
    pub damage_range: (f32, f32),
    pub attack_speed: f32,
    pub crit_multiplier: f32,
}
```

### Error Types

```rust
pub enum ShardError {
    NotFound, AtCapacity, MergeConflict, AlreadyAssigned,
}

pub enum MeshError {
    RegionNotFound, SplitFailed, MergeFailed,
    NoServerAvailable,
}

pub enum InstanceError {
    TemplateNotFound, LockedOut, NoCapacity,
    AlreadyInInstance,
}

pub enum OverlapError {
    BoundaryNotRegistered, SyncFailed,
    DeserializationFailed,
}

pub enum LoadBalancerError {
    NoEligibleServer, ZoneNotFound,
}

pub enum AuctionError {
    ListingNotFound, Outbid, AlreadySold,
    InsufficientFunds, TransactionConflict,
}

pub enum MailError {
    RecipientNotFound, InboxFull, AttachmentInvalid,
}

pub enum AntiCheatError {
    ConfigLoadFailed { path: String },
    ConfigInvalid { field: String },
    PlayerNotFound,
    ValidationFailed { violation: ViolationType, severity: f32 },
}

pub enum EscalationError {
    PlayerNotFound, AlreadyBanned, ActionFailed,
}
```

## Data Flow

### World Topology

The game world is organized in three layers:

1. **Shards.** Full world copies for population management.
2. **Zones.** Spatial subdivisions within a shard. Dynamic split/merge based on entity density.
3. **Instances.** Isolated zone copies for dungeons, raids, and battlegrounds.

### Seamless Zone Transition Pipeline

1. Player approaches zone boundary; source server detects overlap.
2. Source requests handoff from mesh controller.
3. Destination server acknowledges readiness.
4. Source serializes full player state into MigrationPayload.
5. Payload transferred via inter-server bus (at-least-once).
6. Client receives redirect; extrapolates during handoff.
7. Client reconnects; server applies state and sends snapshot.

### Boundary Overlap Co-Simulation

Entities within overlap width are co-simulated by both adjacent servers. Authoritative server
replicates overlap entities to neighbor at configured sync interval. Neighbor renders as ghosts.

### Persistence Pipeline

- **Character saves:** periodic (30 s) + event-triggered.
- **Transactions:** atomic via TransactionHandle.
- **Write throughput:** 10,000+ TPS via pooling and batching.

### Inter-Server Bus Delivery

| Channel | Guarantee | Use Case |
|---------|-----------|----------|
| `telemetry` | At-most-once | Metrics |
| `global_chat` | At-most-once | Chat |
| `world_events` | At-least-once | Boss spawns |
| `migration` | At-least-once | Player handoff |
| `economy` | Exactly-once | Auction bids |

### Anti-Cheat Validation Pipeline

Every client input passes through validation before the server commits it:

1. **Rate limit.** Token bucket per-RPC. Throttle or reject.
2. **Movement.** Compare client pos against server pos with RTT-tolerant bounds.
3. **Damage.** Compare reported damage against weapon stats.
4. **Economy.** Check ownership, recipes, double-spend via transaction sequencing.
5. **Score.** Failed validations accumulate per-player score with time decay.
6. **Escalate.** Score checked against tier thresholds.

### Latency-Aware Thresholds

```text
max_distance = max_speed * dt * (1.0 + rtt_tolerance * rtt)
```

Mobile clients receive additional multiplier for cellular jitter.

### Score Decay

```text
score = score - decay_rate * dt
```

Occasional false detections do not accumulate to thresholds.

### Behavioral Analysis Data Flow

1. Each match records metrics into telemetry.
2. BehavioralAnalyzer computes baselines via Welford's.
3. Z-scores compared against threshold (e.g., 3.0 sigma).
4. Baselines segmented by InputType.
5. Gradual improvement shifts baseline; only sudden jumps flag.

## Platform Considerations

### Server Deployment

| Component | Deployment |
|-----------|------------|
| ZoneServer | AWS ECS / Kubernetes pod |
| MeshController | AWS ECS Fargate |
| AutoScaler | AWS Lambda + CloudWatch |
| LoadBalancer | AWS NLB |
| InterServerBus | In-process (per server) |
| Cross-shard services | AWS ECS Fargate |
| PostgreSQL | AWS RDS (Multi-AZ) |
| DynamoDB | Session directory |

### Database I/O per Platform

| Platform | Async Backend |
|----------|---------------|
| Windows | Tokio (IOCP) |
| macOS | Tokio (kqueue) |
| Linux | Tokio (epoll) |

### Anti-Cheat Deployment

| Component | Deployment |
|-----------|------------|
| MovementValidator | In-process (game server) |
| DamageValidator | In-process (game server) |
| EconomyValidator | In-process (game server) |
| IntegrityChecker | In-process (periodic) |
| BehavioralAnalyzer | Separate analytics service |
| RateLimiter | In-process (per-RPC) |
| ViolationScorer | In-process (component) |
| EscalationManager | In-process + auth service |

### Platform-Specific Adaptations

| Platform | Adaptation | Reason |
|----------|------------|--------|
| Mobile | Wider movement thresholds | Cellular jitter |
| Mobile | Lower RPC rate limits | Fewer inputs/sec |
| Mobile | Separate baselines | Touch != mouse |
| Console | Platform integrity APIs | Cert requirement |
| PC | More frequent integrity | Higher tamper risk |

### Scaling Tiers

| Tier | Shards | Zones/Shard | Players/Shard |
|------|--------|-------------|---------------|
| Launch | 4 | 20 | 500 |
| Growth | 8 | 30 | 500 |
| Peak | 16 | 40 | 500 |

### Mobile Adaptations

| Feature | Desktop | Mobile |
|---------|---------|--------|
| Migration extrapolation | 100 ms | 200 ms |
| Overlap sync interval | 2 ticks | 4 ticks |
| Entity budget per zone | 2,000 | 500 |

## Test Plan

Test cases are in the companion file
[network-infrastructure-test-cases.md](network-infrastructure-test-cases.md).

### Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Unit tests | 45 | Shards, mesh, migration, bus, anti-cheat |
| Integration tests | 26 | Zone transitions, scaling, live detection |
| Benchmarks | 17 | Migration, DB throughput, validation perf |

## Open Questions

1. **Database choice.** PostgreSQL for transactions vs DynamoDB for key-value. Hybrid approach?
2. **Mesh split granularity.** Longest-axis only vs Voronoi partitions for better load balance?
3. **Cross-shard consistency.** Single-leader vs distributed consensus vs optimistic concurrency?
4. **Overlap entity authority.** Source server until handoff vs nearest-center?
5. **Container orchestration.** Kubernetes primary; also support bare-metal for self-hosted studios?
6. **Mobile entity budget.** 500 may be low for cities. Prioritize players over NPCs, or separate
   LOD?
7. **Replay-based verification.** Automated pipeline for every flagged player, or manual review
   tool?
8. **ML for behavioral analysis.** Z-score baseline first; ML layered after production data
   available?
9. **Client integrity on PC.** Invest more in server-side validation depth vs client checks?
10. **Ban appeal automation.** Auto-provide violation history and replay evidence, or human-only?
11. **Cross-session scoring.** Database-backed persistence vs per-session reset?
12. **Rate limit profiles.** Per-game-mode or global?
