# Network Transport and State Replication Design

## Requirements Trace

> **Canonical sources:** Features, requirements, and user stories are defined in
> [features/](../../features/), [requirements/](../../requirements/), and
> [user-stories/](../../user-stories/). The table below traces design elements to those definitions.

### Transport Layer

| Feature | Requirement         | User Story           |
|---------|---------------------|----------------------|
| F-8.1.1 | R-8.1.1             | US-8.1.1, US-8.1.10 |
| F-8.1.2 | R-8.1.2, R-8.NFR.7  | US-8.1.2, US-8.1.12 |
| F-8.1.3 | R-8.1.3, R-8.NFR.12 | US-8.1.4            |
| F-8.1.4 | R-8.1.4             | US-8.1.5            |
| F-8.1.5 | R-8.1.5, R-8.NFR.11 | US-8.1.6            |
| F-8.1.6 | R-8.1.6             | US-8.1.7            |
| F-8.1.7 | R-8.1.7             | US-8.1.8            |
| F-8.1.8 | R-8.1.8             | US-8.1.3, US-8.1.9  |

1. **F-8.1.1** -- Connection handshake and authentication
2. **F-8.1.2** -- Connection lifecycle management
3. **F-8.1.3** -- Reliable ordered channel
4. **F-8.1.4** -- Unreliable and unordered channels
5. **F-8.1.5** -- DTLS encryption
6. **F-8.1.6** -- Packet fragmentation, reassembly, MTU discovery
7. **F-8.1.7** -- Bandwidth estimation and congestion control
8. **F-8.1.8** -- Network diagnostics and quality indicators

### State Replication, Prediction, and RPC

| Feature | Requirement | User Stories             |
|---------|-------------|--------------------------|
| F-8.2.1 | R-8.2.1     | US-8.2.3, US-8.2.10     |
| F-8.2.2 | R-8.2.2     | US-8.2.2, US-8.2.9      |
| F-8.2.3 | R-8.2.3     | US-8.2.1, US-8.2.12     |
| F-8.2.4 | R-8.2.4     | US-8.2.4, US-8.2.7      |
| F-8.2.5 | R-8.2.5     | US-8.2.5, US-8.2.8      |
| F-8.2.6 | R-8.2.6     | US-8.2.6                |
| F-8.4.1 | R-8.4.1     | US-8.4.1, US-8.4.5      |
| F-8.4.2 | R-8.4.2     | US-8.4.6                |
| F-8.4.3 | R-8.4.3     | US-8.4.3                |
| F-8.4.4 | R-8.4.4     | US-8.4.4                |
| F-8.4.5 | R-8.4.5     | US-8.4.2, US-8.4.7      |
| F-8.4.6 | R-8.4.6     | US-8.4.8, US-8.4.9      |
| F-8.3.1 | R-8.3.1     | US-8.3.1, US-8.3.6      |
| F-8.3.2 | R-8.3.2     | US-8.3.3, US-8.3.7      |
| F-8.3.3 | R-8.3.3     | US-8.3.2, US-8.3.4      |
| F-8.3.4 | R-8.3.4     | US-8.3.5                |
| F-8.3.5 | R-8.3.5     | US-8.3.6, US-8.3.9      |

1. **F-8.2.1** -- Delta-compressed property replication
2. **F-8.2.2** -- Component replication with schema versioning
3. **F-8.2.3** -- Area-of-interest filtering via shared BVH
4. **F-8.2.4** -- Conditional and tiered replication
5. **F-8.2.5** -- Priority scheduling and bandwidth budgeting
6. **F-8.2.6** -- Entity dormancy for zero-bandwidth idle
7. **F-8.4.1** -- Input prediction and server reconciliation
8. **F-8.4.2** -- Input buffering with redundant transmission
9. **F-8.4.3** -- Snapshot interpolation for remote entities
10. **F-8.4.4** -- Entity extrapolation with error correction
11. **F-8.4.5** -- Server-side lag compensation (hitbox rewind)
12. **F-8.4.6** -- Jitter buffer and adaptive tick alignment
13. **F-8.3.1** -- Server RPC with validation
14. **F-8.3.2** -- Client RPC for ephemeral events
15. **F-8.3.3** -- Multicast RPC (server-to-group)
16. **F-8.3.4** -- RPC reliability modes
17. **F-8.3.5** -- RPC parameter serialization and validation

### Cross-Cutting Constraints

| Constraint | Source | Impact |
|------------|--------|--------|
| Networking frame budget | R-X.1.1 | 0.5 ms at 60 fps |
| Async I/O via Tokio runtime | Design constraints | All socket ops async |
| ECS-primary (~90%)-based | Design constraints | All net state as ECS resources |
| Static dispatch | Design constraints | No vtables, no dyn Trait |
| Rust stable only | Design constraints | No nightly features |

### Cross-Cutting Dependencies

| Dependency | Source | Consumed API |
|------------|--------|-------------|
| Entity lifecycle | F-1.1.11 | Generational `Entity` handles |
| Change detection | F-1.1.22 | Tick-based `Changed<T>` queries |
| Parallel iteration | F-1.1.20 | Chunk-level parallel query |
| Reflect trait | F-1.3.1 | `TypeRegistry`, field access |
| DynamicValue | F-1.3.5 | Type-erased diff and patch |
| Serialization | F-1.5.1 | Compact binary encoding |
| Shared BVH | F-1.9.1 | Spatial relevancy queries |
| Thread pool | F-14.3.1 | Scoped parallel execution |

## Overview

This design covers the UDP transport layer and the state replication, prediction, rollback, and RPC
systems built on top of it. Together they form the data path for all networked gameplay in
Harmonius.

### Transport Layer

The transport provides UDP-based communication between the platform I/O layer (`Tokio runtime`) and
higher-level networking systems.

1. **UDP-only for game traffic.** All reliability, ordering, and congestion control are userspace
   over raw UDP.
2. **Channel-based multiplexing.** Each connection supports multiple logical channels with
   independent delivery semantics.
3. **DTLS 1.3 encryption.** All traffic encrypted. Key rotation without session interruption.
4. **Async I/O throughout.** All socket operations use `async`/`await` via the `Tokio runtime`.
5. **ECS-native.** Connection state, channel buffers, congestion state, and diagnostics are ECS
   resources.

### State Replication

The replication system is ECS-primary (~90%)-based. Replicated state lives as components. The
`Reflect` trait drives field-level diffing and patching. The shared BVH provides spatial relevancy
queries.

1. **Server-authoritative.** The server owns all gameplay state.
2. **Component-level granularity.** Only changed fields are sent.
3. **Bandwidth-first.** Delta compression, interest management, priority scheduling, and dormancy
   minimize bandwidth.
4. **Latency-hiding.** Prediction, interpolation, extrapolation, and lag compensation hide 80-150 ms
   RTT.

## Architecture

### Transport Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_net::transport"
        TS[TransportSocket]
        CM[ConnectionManager]
        CH[ChannelMux]
        RC[ReliableChannel]
        UC[UnreliableChannel]
        FG[Fragmenter]
        CC[CongestionController]
        DT[DtlsLayer]
        DG[Diagnostics]
        PK[PacketCodec]
    end

    subgraph "harmonius_platform::threading"
        RE[Tokio runtime]
        TP[ThreadPool]
        BP[BufferPool]
    end

    subgraph "harmonius_ecs"
        RES[ECS Resources]
        SYS[ECS Systems]
    end

    TS --> RE
    TS --> BP
    CM --> TS
    CM --> DT
    CH --> RC
    CH --> UC
    CH --> CM
    RC --> FG
    RC --> CC
    UC --> CC
    DG --> CM
    DG --> CC
    PK --> DT
    PK --> FG

    SYS -->|"read/write"| RES
    SYS -->|"drive"| CM
    SYS -->|"drive"| CH
    SYS -->|"drive"| DG
```

### Replication Module Boundaries

```mermaid
graph TD
    subgraph "harmonius_net::replication"
        RS[ReplicationSystem]
        DTR[DeltaTracker]
        IM[InterestManager]
        PS[PriorityScheduler]
        SB[SnapshotBuffer]
        DM[DormancyManager]
    end

    subgraph "harmonius_net::prediction"
        CP[ClientPredictor]
        SR[ServerReconciler]
        IB[InputBuffer]
        JBR[JitterBuffer]
    end

    subgraph "harmonius_net::interpolation"
        SI[SnapshotInterpolator]
        EX[Extrapolator]
        EC[ErrorCorrector]
    end

    subgraph "harmonius_net::rpc"
        RD[RpcDispatcher]
        RV[RpcValidator]
        MC[MulticastRouter]
    end

    subgraph "harmonius_net::lag_comp"
        HR[HistoryRewinder]
        HB[HitboxBuffer]
    end

    RS --> DTR
    RS --> IM
    RS --> PS
    RS --> SB
    RS --> DM
    CP --> IB
    CP --> SR
    SI --> SB
    SI --> EX
    EX --> EC
    HR --> HB
    RD --> RV
    RD --> MC
```

### File Layout

```text
harmonius_net/
+-- transport/
|   +-- socket.rs        # TransportSocket, async UDP
|   +-- connection.rs    # ConnectionManager, Connection
|   +-- handshake.rs     # Handshake phases, cookie, auth
|   +-- channel.rs       # ChannelMux, ChannelId, mode
|   +-- reliable.rs      # ReliableChannel, SACK, windows
|   +-- unreliable.rs    # UnreliableChannel
|   +-- fragment.rs      # Fragmenter, reassembly, MTU
|   +-- congestion.rs    # CongestionController
|   +-- dtls.rs          # DtlsLayer, DtlsSession
|   +-- codec.rs         # PacketCodec, header encode
|   +-- stats.rs         # NetStatsResource, RTT, loss
|   +-- systems.rs       # ECS systems (poll, recv, send)
|   +-- config.rs        # TransportConfig, platform defs
|   +-- error.rs         # TransportError variants
+-- replication/
|   +-- system.rs        # ReplicationSystem, tick loop
|   +-- delta.rs         # DeltaTracker, Baseline
|   +-- interest.rs      # InterestManager, AOI
|   +-- priority.rs      # PriorityScheduler
|   +-- snapshot.rs      # SnapshotBuffer, Snapshot
|   +-- dormancy.rs      # DormancyManager
|   +-- schema.rs        # SchemaRegistry, SchemaVersion
+-- prediction/
|   +-- predictor.rs     # ClientPredictor
|   +-- reconciler.rs    # ServerReconciler
|   +-- input_buffer.rs  # InputBuffer, TimestampedInput
|   +-- jitter_buffer.rs # JitterBuffer
+-- interpolation/
|   +-- interpolator.rs  # SnapshotInterpolator
|   +-- extrapolator.rs  # Extrapolator
|   +-- error_correct.rs # ErrorCorrector
+-- rpc/
|   +-- dispatcher.rs    # RpcDispatcher
|   +-- validator.rs     # RpcValidator, RateLimiter
|   +-- multicast.rs     # MulticastRouter
|   +-- registry.rs      # RpcRegistry, RpcDefinition
+-- lag_comp/
    +-- rewinder.rs      # HistoryRewinder
    +-- hitbox_buffer.rs # HitboxBuffer, HitboxSnapshot
```

### Connection Handshake

Four-phase protocol integrating DTLS with application-layer authentication. Phase 1 is stateless to
resist flooding (R-8.1.1).

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DTLS as DTLS Layer

    Note over C,S: Phase 1 -- Cookie Exchange
    C->>S: ClientHello (no token)
    S->>C: HelloRetry + cookie
    Note over S: Stateless -- no allocation yet

    Note over C,S: Phase 2 -- DTLS Handshake
    C->>DTLS: ClientHello + cookie
    DTLS->>DTLS: DTLS 1.3 negotiation
    DTLS-->>C: Encrypted channel established
    DTLS-->>S: Encrypted channel established

    Note over C,S: Phase 3 -- Authentication
    C->>S: AuthRequest(session_token, timestamp)
    S->>S: Validate token, check replay
    alt Token valid
        S->>C: AuthAccept(conn_id, server_tick)
    else Token invalid or replayed
        S->>C: AuthReject(reason)
    end

    Note over C,S: Phase 4 -- Channel Setup
    C->>S: ChannelConfig(requested channels)
    S->>C: ChannelReady(assigned channel IDs)
```

Anti-flood: HMAC-SHA256 cookie with configurable 5 s expiry. Replay resistance: monotonic timestamp
with sliding window.

### Connection State Machine

```mermaid
stateDiagram-v2
    [*] --> Disconnected
    Disconnected --> Connecting: connect()
    Connecting --> Authenticating: DTLS done
    Connecting --> Disconnected: timeout
    Authenticating --> Active: auth ok
    Authenticating --> Disconnected: auth fail
    Active --> Migrating: migrate()
    Active --> Disconnected: timeout
    Active --> Disconnecting: disconnect()
    Migrating --> Active: complete
    Migrating --> Disconnected: failed
    Disconnecting --> Disconnected: done
    Disconnected --> [*]
```

### Channel Architecture

| Mode | Reliable | Ordered | Use Case |
|------|----------|---------|----------|
| ReliableOrdered | Yes | Yes | Inventory, quests, chat |
| ReliableUnordered | Yes | No | Entity spawns, config |
| UnreliableSequenced | No | Seq | Position updates, input |
| UnreliableUnordered | No | No | Voice, VFX triggers |

### Congestion Control

Game-oriented BBR-inspired algorithm prioritizing smooth throughput over maximum utilization
(R-8.1.7).

```mermaid
stateDiagram-v2
    [*] --> SlowStart
    SlowStart --> CongestionAvoidance: cwnd >= ssthresh
    SlowStart --> Recovery: loss detected
    CongestionAvoidance --> Recovery: loss detected
    Recovery --> CongestionAvoidance: loss recovered
    Recovery --> SlowStart: timeout (RTO)
```

| Phase | Cwnd Update |
|-------|-------------|
| SlowStart | +1 MSS per ack |
| CongestionAvoidance | +1 MSS per RTT |
| Recovery | cwnd = cwnd * 0.7 |

### Server Replication Tick

```mermaid
sequenceDiagram
    participant ECS as ECS World
    participant RS as ReplicationSystem
    participant IM as InterestManager
    participant DT as DeltaTracker
    participant PS as PriorityScheduler
    participant TR as Transport

    Note over ECS: Server tick N completes
    RS->>ECS: query Changed components
    RS->>IM: evaluate_relevancy(clients)
    IM->>IM: BVH query + game rules
    IM-->>RS: per-client relevant entity sets
    RS->>DT: compute_deltas(baselines, current)
    DT-->>RS: per-client delta packets
    RS->>PS: schedule(deltas, budgets)
    PS-->>RS: prioritized send list

    loop Per client connection
        RS->>TR: send(delta_packet, channel)
    end

    TR-->>RS: ack(client, baseline_tick)
    RS->>DT: advance_baseline(client, tick)
```

### Client Prediction and Reconciliation

```mermaid
sequenceDiagram
    participant IN as Input
    participant IB as InputBuffer
    participant CP as ClientPredictor
    participant ECS as ECS World
    participant TR as Transport
    participant SV as Server
    participant SR as ServerReconciler

    IN->>IB: record(input, tick_T)
    IB->>TR: send input plus redundant copies
    CP->>ECS: apply input_T predicted
    Note over ECS: Client sees result immediately

    SV-->>TR: server_state for tick_T
    TR-->>SR: receive authoritative state

    SR->>SR: compare predicted vs server
    alt Prediction correct
        SR->>IB: discard inputs before T
    else Mismatch detected
        SR->>ECS: restore server state at T
        SR->>IB: get unacked inputs after T
        loop Replay each unacked input
            SR->>ECS: re-simulate input_i
        end
    end
```

### Hitbox Rewinding (Lag Compensation)

```mermaid
sequenceDiagram
    participant CL as Attacker Client
    participant TR as Transport
    participant SV as Server
    participant HR as HistoryRewinder
    participant HB as HitboxBuffer
    participant PH as Physics Queries

    CL->>TR: fire target aim_dir client_tick_T
    TR->>SV: deliver hit request
    SV->>HR: rewind to rewind_tick
    HR->>HB: get hitbox snapshot at rewind_tick
    HR->>PH: raycast aim_dir rewound hitboxes
    alt Hit confirmed
        PH-->>SV: hit result
        SV->>SV: apply damage server authority
    else Miss after rewind
        PH-->>SV: no hit
        SV->>SV: reject favor defender
    end
    HR->>HR: restore current tick hitboxes
```

### Entity Lifecycle (Reconciliation Edge Cases)

```mermaid
stateDiagram-v2
    [*] --> Created: spawn on authority
    Created --> Replicated: first replication
    Replicated --> AuthorityTransfer: transfer begins
    AuthorityTransfer --> Replicated: transfer complete
    Replicated --> Destroyed: authority destroys
    Destroyed --> Tombstoned: tombstone marker set
    Tombstoned --> Expired: TTL elapsed (2x max RTT)
    Expired --> [*]: entity handle recycled

    state Replicated {
        [*] --> Active
        Active --> Dormant: no changes for threshold
        Dormant --> Active: change detected or wake()
    }

    state AuthorityTransfer {
        [*] --> SnapshotSent: full state snapshot
        SnapshotSent --> SnapshotAcked: receiver ACK
        SnapshotAcked --> EpochBumped: epoch incremented
    }
```

### Core Data Structures

```mermaid
classDiagram
    class TransportSocket {
        -socket: RawHandle
        -reactor: Tokio runtime
        -recv_buf: BufferPool
        +bind(addr) Future~Result~
        +sendto(addr, data) Future~Result~
        +recv_from() Future~Result~Datagram~
    }

    class ConnectionManager {
        -connections: HashMap~ConnectionId, Connection~
        -pending: HashMap~Token, HandshakeState~
        -timeout_wheel: TimingWheel
        -config: TransportConfig
        +accept() Future~ConnectionId~
        +connect(addr, token) Future~ConnectionId~
        +disconnect(id)
        +poll_timeouts() SmallVec~ConnectionId~
    }

    class Connection {
        -id: ConnectionId
        -remote_addr: SocketAddr
        -state: ConnectionState
        -dtls: DtlsSession
        -channels: ChannelMux
        -stats: ConnectionStats
    }

    class ChannelMux {
        -channels: Vec~Channel~
        +open(mode) ChannelId
        +send(channel, payload)
        +recv(channel) Option~Payload~
    }

    class ReliableChannel {
        -send_window: SendWindow
        -recv_window: RecvWindow
        -retransmit_queue: VecDeque~Packet~
        -rtt_estimator: RttEstimator
    }

    class CongestionController {
        -cwnd: u32
        -ssthresh: u32
        -bytes_in_flight: u32
        -state: CongestionState
        +on_ack(bytes, rtt)
        +on_loss(bytes)
        +send_budget() u32
    }

    class Fragmenter {
        -mtu: u16
        -next_group_id: u16
        -pending: HashMap~GroupId, FragmentGroup~
        +fragment(payload) Vec~Fragment~
        +reassemble(fragment) Option~Payload~
    }

    class NetStatsResource {
        -rtt: Duration
        -rtt_variance: Duration
        -packet_loss_pct: f32
        -jitter: Duration
        -bandwidth_up: u64
        -bandwidth_down: u64
        -stability_score: f32
    }

    class DtlsSession {
        -state: DtlsState
        -write_key: AesGcmKey
        -read_key: AesGcmKey
        +encrypt(header, payload) Result
        +decrypt(header, payload) Result
        +maybe_rotate_keys() Future~Result~
    }

    class TimingWheel {
        -slots: Vec~Vec~ConnectionId~~
        -current_slot: u32
        +schedule(id, delay)
        +cancel(id)
        +tick() SmallVec~ConnectionId~
    }

    class ReplicationSystem {
        -delta_tracker: DeltaTracker
        -interest_mgr: InterestManager
        -scheduler: PriorityScheduler
        -snapshot_buf: SnapshotBuffer
        -dormancy_mgr: DormancyManager
        +tick(world, transport)
        +on_client_ack(client, tick)
    }

    class DeltaTracker {
        -baselines: HashMap~ClientId, Baseline~
        -schema_registry: SchemaRegistry
        +compute_delta(client, entity) DeltaPayload
        +advance_baseline(client, tick)
    }

    class InterestManager {
        -aoi_radius: f32
        -custom_rules: Vec~RelevancyRule~
        +evaluate(client, world) RelevantSet
    }

    class PriorityScheduler {
        -bandwidth_budget: u32
        +schedule(deltas, budget) Vec~ScheduledDelta~
    }

    class SnapshotBuffer {
        -ring: RingBuffer~Snapshot~
        +push(snapshot)
        +get_bracketing(time) Option~SnapshotPair~
        +get_at(tick) Option~Snapshot~
    }

    class DormancyManager {
        -dormant: HashSet~Entity~
        -threshold: Duration
        +check_dormancy(entity, last_change) bool
        +wake(entity)
    }

    class ClientPredictor {
        -input_buffer: InputBuffer
        -predicted: HashMap~Entity, DynamicValue~
        +predict(input, world)
    }

    class ServerReconciler {
        +reconcile(server, predicted, world)
        +replay(inputs, world)
    }

    class SnapshotInterpolator {
        -interp_delay: Duration
        +interpolate(a, b, alpha) Pose
    }

    class Extrapolator {
        -max_extrap: Duration
        +extrapolate(snapshot, dt) Pose
    }

    class ErrorCorrector {
        -decay_rate: f32
        -current_error: Vec3
        +apply_correction(error)
        +tick() Vec3
    }

    class JitterBuffer {
        -buffer: BTreeMap~SequenceTick, Snapshot~
        -target_depth: Duration
        +insert(snapshot, tick)
        +release() Option~Snapshot~
        +adapt(jitter_estimate)
    }

    class HistoryRewinder {
        -hitbox_buffer: HitboxBuffer
        -max_rewind: Duration
        +rewind(tick) HitboxSnapshot
        +restore()
    }

    class HitboxBuffer {
        -ring: RingBuffer~HitboxSnapshot~
        +record(tick, hitboxes)
        +get(tick) Option~HitboxSnapshot~
    }

    class RpcDispatcher {
        -handlers: HashMap~RpcId, RpcHandler~
        -validator: RpcValidator
        +register(rpc_id, handler, mode)
        +dispatch_server(rpc_id, params, sender)
        +invoke_client(target, rpc_id, params)
        +invoke_multicast(filter, rpc_id, params)
    }

    class RpcValidator {
        -rate_limiters: HashMap~RpcId, RateLimiter~
        +validate(rpc_id, params, sender) Result
    }

    class MulticastRouter {
        +resolve(filter, world) Vec~ClientId~
    }

    class Tombstone {
        +destroyed_at: SequenceTick
        +expires_at: SequenceTick
        +epoch: AuthorityEpoch
    }

    class AuthorityEpoch {
        +u64 value
        +next() AuthorityEpoch
        +supersedes(other) bool
    }

    class AuthorityShard {
        +authority: NetworkAuthority
        +epoch: AuthorityEpoch
        +frozen: bool
    }

    class PhysicsTransferSnapshot {
        +position: Vec3
        +rotation: Quat
        +linear_velocity: Vec3
        +angular_velocity: Vec3
        +tick: SequenceTick
        +new_epoch: AuthorityEpoch
    }

    class ReplicationOp {
        +entity: Entity
        +priority: ReplicationOpPriority
        +tick: SequenceTick
        +payload: ReplicationOpPayload
    }

    ConnectionManager *-- Connection
    Connection *-- ChannelMux
    Connection *-- DtlsSession
    Connection *-- ConnectionStats
    ChannelMux *-- ReliableChannel
    ChannelMux *-- UnreliableChannel
    ReliableChannel --> CongestionController
    ReliableChannel --> Fragmenter
    ConnectionManager --> TransportSocket
    ConnectionStats --> NetStatsResource
    ReplicationSystem *-- DeltaTracker
    ReplicationSystem *-- InterestManager
    ReplicationSystem *-- PriorityScheduler
    ReplicationSystem *-- SnapshotBuffer
    ReplicationSystem *-- DormancyManager
    ClientPredictor --> ServerReconciler
    SnapshotInterpolator --> Extrapolator
    Extrapolator --> ErrorCorrector
    HistoryRewinder *-- HitboxBuffer
    RpcDispatcher *-- RpcValidator
    RpcDispatcher --> MulticastRouter
    JitterBuffer ..> SnapshotBuffer
    Tombstone --> AuthorityEpoch
    AuthorityShard --> AuthorityEpoch
    PhysicsTransferSnapshot --> AuthorityEpoch
```

## API Design

### Transport Core Types

```rust
/// Protocol magic number for Harmonius transport.
const PROTOCOL_ID: u16 = 0x484E; // "HN"

/// Opaque connection identifier.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, PartialOrd, Ord, Reflect,
)]
pub struct ConnectionId(pub u16);

/// Logical channel within a connection.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, PartialOrd, Ord, Reflect,
)]
pub struct ChannelId(pub u8);

/// 16-bit wrapping sequence number.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub struct SequenceNumber(pub u16);

impl SequenceNumber {
    pub fn is_newer_than(
        self,
        other: SequenceNumber,
    ) -> bool {
        let diff = self.0.wrapping_sub(other.0);
        diff > 0 && diff < 32768
    }

    pub fn next(self) -> SequenceNumber {
        SequenceNumber(self.0.wrapping_add(1))
    }
}

/// Fragment metadata packed into 2 bytes.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub struct FragmentInfo {
    pub index: u8,
    pub total: u8,
}

/// Channel delivery mode.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub enum ChannelMode {
    ReliableOrdered,
    ReliableUnordered,
    UnreliableSequenced,
    UnreliableUnordered,
}

/// Packet types in the transport protocol.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
#[repr(u8)]
pub enum PacketType {
    ClientHello = 0x01,
    HelloRetry = 0x02,
    AuthRequest = 0x03,
    AuthAccept = 0x04,
    AuthReject = 0x05,
    ChannelConfig = 0x06,
    ChannelReady = 0x07,
    Data = 0x10,
    Ack = 0x11,
    Fragment = 0x12,
    Heartbeat = 0x20,
    Disconnect = 0x21,
    DisconnectAck = 0x22,
    MtuProbe = 0x30,
    MtuProbeAck = 0x31,
}

/// Wire format: 16 bytes total.
#[derive(Clone, Copy, Debug, Reflect)]
pub struct PacketHeader {
    pub protocol_id: u16,
    pub connection_id: ConnectionId,
    pub packet_type: PacketType,
    pub channel_id: ChannelId,
    pub sequence: SequenceNumber,
    pub ack: SequenceNumber,
    pub ack_bitfield: u32,
    pub fragment_info: FragmentInfo,
}

/// Connection lifecycle state.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect,
)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Authenticating,
    Active,
    Migrating,
    Disconnecting,
}

/// Congestion control state.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum CongestionState {
    SlowStart,
    CongestionAvoidance,
    Recovery,
}

/// DTLS handshake state.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum DtlsState {
    Initial,
    Handshaking,
    Established,
    Rekeying,
    Closed,
}

/// Platform-specific DTLS context.
pub enum DtlsContext {
    #[cfg(target_os = "windows")]
    Schannel(SchannelContext),
    #[cfg(target_os = "macos")]
    SecureTransport(SecureTransportContext),
    #[cfg(target_os = "linux")]
    Rustls(RustlsContext),
}

/// Network events for diagnostics.
#[derive(Clone, Debug, Reflect)]
pub enum NetEventKind {
    Connected,
    Disconnected { reason: DisconnectReason },
    LatencySpike { rtt: Duration },
    PacketLossBurst { loss_pct: f32 },
    Timeout,
    Reconnected,
    KeyRotation,
    MtuChanged { old: u16, new: u16 },
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum DisconnectReason {
    Graceful,
    Timeout,
    AuthFailed,
    Kicked,
    MigrationFailed,
    ProtocolError,
}

/// Transport layer errors.
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum TransportError {
    BindFailed { addr: SocketAddr, code: i32 },
    ConnectionLimitReached { max: u32 },
    ConnectionNotFound { id: ConnectionId },
    ChannelNotFound { id: ChannelId },
    ChannelLimitReached,
    HandshakeTimeout,
    AuthRejected { reason: String },
    DtlsError { detail: String },
    InvalidPacket { detail: String },
    PayloadTooLarge { size: usize, max: usize },
    FragmentTimeout { group: FragmentGroupId },
    SendFailed { code: i32 },
    RecvFailed { code: i32 },
    ConnectionTimeout { id: ConnectionId },
    InvalidCookie,
    ReplayDetected,
}
```

### Replication Core Types

```rust
/// Monotonically increasing server tick counter.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    PartialOrd, Ord, Hash, Reflect, Serialize,
)]
pub struct SequenceTick(pub u32);

/// Unique identifier for a connected client.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect, Serialize,
)]
pub struct ClientId(pub u32);

/// Unique identifier for a registered RPC.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect, Serialize,
)]
pub struct RpcId(pub u32);

/// Component type identifier in the schema.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect, Serialize,
)]
pub struct ComponentId(pub u32);

/// Marks an entity for network replication.
#[derive(Component, Reflect)]
pub struct Replicated;

/// Identifies which client owns this entity.
#[derive(Component, Reflect)]
pub struct NetworkOwner {
    pub client: ClientId,
}

/// Network authority model.
#[derive(
    Component, Clone, Copy, Debug,
    PartialEq, Eq, Reflect,
)]
pub enum NetworkAuthority {
    Server,
    ClientAuthoritative { client: ClientId },
}

/// Property visibility levels.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum PropertyVisibility {
    Public,
    OwnerOnly,
    TeamOnly,
}

/// Named property subsets for tiered replication.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum PropertySet {
    Full,
    Movement,
    PositionOnly,
}

/// Distance-based replication tier.
#[derive(Clone, Debug, Reflect)]
pub struct ReplicationTier {
    pub max_distance: f32,
    pub update_rate_hz: f32,
    pub property_set: PropertySet,
}

/// Schema version for replicated components.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    Hash, Reflect, Serialize,
)]
pub struct SchemaVersion(pub u32);

/// Delta payload for one component on one entity.
#[derive(Clone, Debug, Serialize, Reflect)]
pub struct DeltaPayload {
    pub entity: Entity,
    pub component_id: ComponentId,
    pub tick: SequenceTick,
    pub changed_mask: u64,
    pub field_data: Vec<u8>,
}

/// Per-client connection state.
#[derive(Clone, Debug, Reflect)]
pub struct ClientConnection {
    pub client_id: ClientId,
    pub rtt: Duration,
    pub jitter: Duration,
    pub packet_loss: f32,
    pub bandwidth_budget: u32,
    pub platform: ClientPlatform,
    pub last_acked_tick: SequenceTick,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum ClientPlatform {
    Desktop,
    Mobile,
    Console,
}

/// RPC reliability mode.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Reflect,
)]
pub enum RpcReliability {
    Reliable,
    Unreliable,
    ReliableLatest,
}

/// Filter for multicast RPC recipients.
#[derive(Clone, Debug, Reflect)]
pub enum MulticastFilter {
    Spatial { center: Vec3, radius: f32 },
    Party { client: ClientId },
    Team { team_id: u32 },
    Raid { raid_id: u32 },
    Zone { zone_id: u32 },
    All { filters: Vec<MulticastFilter> },
}

/// Relevancy override rule.
#[derive(Clone, Debug, Reflect)]
pub enum RelevancyRule {
    AlwaysRelevant { filter: RelevancyFilter },
    NeverRelevant { filter: RelevancyFilter },
    CustomRadius { filter: RelevancyFilter, radius: f32 },
}

/// Relevancy filter combinators.
#[derive(Clone, Debug, Reflect)]
pub enum RelevancyFilter {
    SameParty,
    SameTeam,
    SameGuild,
    HasComponent { component_id: ComponentId },
    All { filters: Vec<RelevancyFilter> },
    Any { filters: Vec<RelevancyFilter> },
}

/// Monotonic authority epoch.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    PartialOrd, Ord, Hash, Reflect, Serialize,
)]
pub struct AuthorityEpoch(pub u64);

/// Structural replication operation priority.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq,
    PartialOrd, Ord, Reflect,
)]
pub enum ReplicationOpPriority {
    EntityDestroy = 0,
    ComponentRemove = 1,
    ComponentAdd = 2,
    StateDelta = 3,
}

/// Playback speed for replays.
#[derive(
    Clone, Copy, Debug, PartialEq, Reflect,
)]
pub enum PlaybackSpeed {
    Paused,
    FrameByFrame,
    Quarter,
    Half,
    Normal,
    Double,
    Quad,
    Octa,
}

/// Player input.
#[derive(Clone, Debug, Reflect, Serialize)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub aim_direction: Vec3,
    pub actions: SmallVec<[InputAction; 4]>,
}

#[derive(Clone, Debug, Reflect, Serialize)]
pub struct InputAction {
    pub action_id: u32,
    pub pressed: bool,
    pub value: f32,
}

/// Hitbox data for lag compensation.
#[derive(Clone, Reflect)]
pub struct HitboxData {
    pub position: Vec3,
    pub rotation: Quat,
    pub half_extents: Vec3,
}

/// Replication errors.
pub enum ReplicationError {
    UnregisteredComponent { component_id: ComponentId },
    SchemaIncompatible {
        server_version: SchemaVersion,
        client_version: SchemaVersion,
    },
    TransportError { detail: String },
}

/// RPC errors.
pub enum RpcError {
    ValidationFailed { error: RpcValidationError },
    NoHandler { rpc_id: RpcId },
    HandlerFailed { detail: String },
    TransportError { detail: String },
}

pub enum RpcValidationError {
    UnknownRpc { rpc_id: RpcId },
    TypeMismatch { field: String, expected: String, actual: String },
    OutOfRange { field: String, value: String, min: String, max: String },
    EntityNotFound { entity: Entity },
    PermissionDenied { sender: ClientId, entity: Entity },
    RateLimited { rpc_id: RpcId, retry_after: Duration },
    MalformedPayload { detail: String },
}

pub enum PredictionError {
    NotPredicted { entity: Entity },
    BufferFull { max_depth: u8 },
}
```

## Data Flow

### Outbound Packet Pipeline

1. Application calls `connection.send(channel, data)`.
2. Channel assigns sequence number and buffers (reliable) or discards after encoding (unreliable).
3. Fragmenter checks payload against MTU; splits if oversized.
4. Congestion controller checks send budget.
5. PacketCodec encodes 16-byte header + payload.
6. DtlsLayer encrypts and appends AES-GCM auth tag.
7. TransportSocket submits async `sendto()` via Tokio runtime.

### Inbound Packet Pipeline

1. Tokio runtime harvests UDP recv completion at frame poll point.
2. `net_poll_system` pushes datagram into RecvBufferResource.
3. `net_recv_system` decodes header via PacketCodec.
4. Invalid `protocol_id` packets are silently dropped.
5. DtlsLayer decrypts and verifies auth tag.
6. Fragments buffered in Fragmenter until complete.
7. Complete payloads routed to channel by `channel_id`.
8. Reliable channels process ack/SACK to release send window.

### Server Replication Tick Pipeline

1. Query `Changed` components from ECS world.
2. Evaluate interest for all clients (parallelized via BVH).
3. Update dormancy tracking.
4. Compute per-client deltas via Reflect field-level diff.
5. Priority-schedule within per-client bandwidth budget.
6. Send prioritized deltas via transport.
7. Record snapshot for lag compensation.

### Client Frame Pipeline

1. Drain received snapshots into jitter buffer.
2. Release steady-cadence snapshots.
3. Reconcile server state against predictions.
4. Apply new local input (prediction).
5. Send input packet with redundancy.
6. Interpolate remote entities with error correction.

### Delta Compression Algorithm

1. Enumerate fields via `Reflect::fields()`.
2. Lookup client baseline.
3. Field-by-field comparison via PartialEq.
4. Set bit in u64 changed mask per changed field.
5. Serialize only changed field values.
6. Apply quantization (e.g., 16-bit position deltas).

## Platform Considerations

### Socket I/O

| Platform | API |
|----------|-----|
| Windows | `tokio::net` (IOCP internally) |
| macOS | `tokio::net` (kqueue internally) |
| Linux | `tokio::net` (epoll internally) |

### DTLS Backend

| Platform | Library |
|----------|---------|
| Windows | Schannel via windows-rs |
| macOS | Security.framework via objc2 |
| Linux | rustls (pure Rust) |

### Platform Defaults

| Parameter | Desktop | Mobile |
|-----------|---------|--------|
| Heartbeat interval | 1 s | 5 s |
| Connection timeout | 30 s | 60 s |
| Default MTU | 1,200 B | 1,200 B |
| Initial send rate | unlimited | 500 Kbps |
| Send window | 256 pkts | 64 pkts |
| AOI radius | 200 m | 100 m |
| Max rollback frames | 8 | 4 |
| Input redundancy | 3 | 6 |
| Interpolation delay | 1 tick | 2 ticks |
| Extrapolation window | 100 ms | 200 ms |
| Error correction decay | 0.3 | 0.5 |
| Jitter buffer depth | 1-3 ticks | 3-5 ticks |
| Bandwidth budget | 500+ KB/s | 50-100 KB/s |

### Proposed Dependencies

| Crate | Purpose |
|-------|---------|
| `rustls` | DTLS on Linux |
| `ring` | AES-GCM, HMAC-SHA256 |
| `windows-rs` | Schannel, Winsock2 |
| `objc2`        | Security.framework |
| `smallvec` | Inline-allocated small vectors |

## Safety Invariants

### System Access Sets (Medium)

Transport ECS systems must declare explicit `AccessSet` for `ConnectionStates` and `TransportStats`.
Sequential ordering enforced via explicit system dependencies.

### HitboxBuffer Rewinding (High)

`HistoryRewinder` operates on a read-only ring of immutable snapshots, never mutating live ECS
components.

## Test Plan

Test cases are in the companion file
[network-transport-test-cases.md](./network-transport-test-cases.md).

### Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Unit tests | 63 | Transport, replication, RPC |
| Integration tests | 27 | End-to-end networking |
| Benchmarks | 19 | Latency, throughput, budget |

## Open Questions

1. **DTLS library on Linux.** rustls vs openssl for DTLS 1.3 maturity. Tradeoff: C dependency vs
   maturity.
2. **Congestion algorithm.** Loss-based vs BBR-style for mobile cellular links with bufferbloat.
3. **Maximum payload size.** 64 KiB (255 fragments x 1200 B) sufficient for zone snapshots?
4. **Connection ID size.** 16-bit limits 65,535 connections. Future scaling may need 32/64-bit IDs.
5. **Sequence number size.** 16-bit wraps in ~18 min at 60 pkt/sec. 32-bit adds 4 bytes per packet.
6. **Key rotation interval.** Proposed: 1 hour. Shorter limits exposure but increases CPU.
7. **Tokio epoll compatibility.** Minimum kernel version for Tokio epoll backend?
8. **Quantization precision.** Per-field `#[quantize]` attribute vs fixed per data type?
9. **Snapshot memory budget.** Server snapshot only hitbox data vs full component state?
10. **Prediction eligibility scope.** Extend to physics objects the player interacts with?
11. **Cross-entity rollback dependencies.** Topological sort vs full ECS schedule replay?
12. **Schema migration complexity.** Support field renames and type changes via migration functions?
13. **Dormancy threshold tuning.** Per-entity-type thresholds via component field vs global config?

## Review Feedback

### RF-1: Replace all Tokio/async with platform-native I/O

Replace Future-returning socket APIs with synchronous submission + completion via crossbeam-channel:

- Linux: io_uring via rustix (UDP send/recv)
- Windows: IOCP via windows-rs (Winsock2 overlapped)
- Apple: Networking.framework via objc2 (NWConnection UDP)

Main thread polls completions, posts as jobs. All transport APIs become synchronous. Remove open
question #7 (moot).

### RF-2: Replace Reflect with codegen diff/patch

Delta compression uses codegen-generated field accessors:

```rust
// Generated per replicated component:
impl HealthComponent {
    fn diff(
        &self, baseline: &Self,
    ) -> (u64, Vec<u8>) {
        let mut mask = 0u64;
        let mut buf = Vec::new();
        if self.hp != baseline.hp {
            mask |= 1 << 0;
            buf.extend(self.hp.to_le_bytes());
        }
        if self.max_hp != baseline.max_hp {
            mask |= 1 << 1;
            buf.extend(self.max_hp.to_le_bytes());
        }
        (mask, buf)
    }

    fn patch(&mut self, mask: u64, data: &[u8]) {
        let mut cursor = 0;
        if mask & (1 << 0) != 0 {
            self.hp = f32::from_le_bytes(
                data[cursor..cursor+4].try_into().unwrap()
            );
            cursor += 4;
        }
        // ...
    }
}
```

Remove DynamicValue and Reflect dependencies entirely.

### RF-3: Create companion test cases file

Create `network-transport-test-cases.md` with the 109 claimed tests (63 unit + 27 integration + 19
benchmarks).

### RF-4: Use networking grid for interest management

The spatial index design specifies a grid for networking relevancy. Replace BVH-based interest
management with grid-based:

- Each client subscribes to grid cells near their position
- Cell transition: subscribe to new cells, unsubscribe old
- Entities in subscribed cells are replicated
- Update frequency by cell distance (same cell = 60 Hz, adjacent = 30 Hz, 2 away = 10 Hz)

Grid is simpler, predictable, and designed for networking. BVH is for gameplay/AI spatial queries.

### RF-5: Add 2D multiplayer support

Add Vec2 variants for spatial types:

- `PlayerInput.movement`: Vec2 for 2D games
- `HitboxData`: 2D shapes (circle, rect) alongside 3D
- `MulticastFilter::Spatial`: Vec2 center for 2D
- `ErrorCorrector`: Vec2 error for 2D interpolation

### RF-6: MMO sharding and layering

Support massive player counts via server-side sharding and player layering:

**Sharding (horizontal scaling):**

Each shard owns a spatial region of the world. Players crossing shard boundaries undergo entity
migration (similar to planet migration from Design #22 RF-10):

```text
Shard A (cells 0-99)
  → Player crosses boundary at cell 100
  → Entity serialized via rkyv
  → Transferred to Shard B (cells 100-199)
  → Deserialized, client redirected
```

Shards communicate via inter-shard messaging (server-to- server UDP or TCP) for cross-boundary
entity visibility and interest management.

**Layering (vertical scaling):**

Multiple instances ("layers") of the same world region run on different servers. Players in the same
area but different layers cannot see each other:

```rust
#[derive(Component)]
pub struct NetworkLayer {
    pub layer_id: u16,
    pub max_players: u16,
}
```

- When a layer exceeds capacity, overflow players are assigned to a new layer
- Players can request layer transfer (join friend)
- Layer merging: when population drops, merge layers
- Boss encounters: lock layer to prevent overflow

**Interest management across shards:**

- Entities near shard boundaries are replicated to both shards as "ghost" entities (read-only
  copies)
- Ghost entities receive delta updates from the owning shard via inter-shard channel
- Boundary width = interest radius (from grid cell size)

**Scaling targets:**

| Tier | Players/shard | Shards | Total |
|------|-------------|--------|-------|
| Small | 100 | 1 | 100 |
| Medium | 1,000 | 4 | 4,000 |
| Large | 5,000 | 16 | 80,000 |
| MMO | 10,000 | 100+ | 1M+ |

Sharding and layering are server-side concerns — clients connect to one shard and see one layer. The
transport layer handles redirect on shard transfer.

### RF-7: Document 64-field changed_mask limit

The `changed_mask: u64` caps replicated components at 64 fields. Document this as a hard limit.
Components with more than 64 fields should be split into sub-components.

### RF-8: Algorithm references

| Algorithm | Reference |
|-----------|-----------|
| SACK reliability | [RFC 2018](https://www.rfc-editor.org/rfc/rfc2018) |
| BBR congestion | [Cardwell et al. (2017)](https://queue.acm.org/detail.cfm?id=3022184) |
| ORCA interest | [van den Berg (2011)](https://gamma.cs.unc.edu/ORCA/publications/ORCA.pdf) |
| Delta compression | [Bernier, "Latency Compensating" (2001)](https://developer.valvesoftware.com/wiki/Latency_Compensating_Methods_in_Client/Server_In-game_Protocol_Design_and_Optimization) |
| Client prediction | [Fiedler, "Networked Physics" (2015)](https://gafferongames.com/post/networked_physics_in_virtual_reality/) |
| Snapshot interpolation | [Fiedler (2014)](https://gafferongames.com/post/snapshot_interpolation/) |
| State synchronization | [Fiedler (2015)](https://gafferongames.com/post/state_synchronization/) |

### RF-9: Quality of service and network optimization

**Adaptive send rate:**

Adjust replication frequency per client based on measured conditions:

| Condition | Response |
|-----------|----------|
| Low RTT, no loss | Max send rate (60 Hz) |
| Moderate RTT | Reduce to 30 Hz |
| High loss (> 5%) | Reduce rate + increase redundancy |
| Congestion detected | BBR backs off, reduce payload |
| Mobile/cellular | Cap at 20 Hz, smaller packets |

**Quantization:**

Reduce bandwidth by quantizing replicated fields:

```rust
#[derive(Component, Replicate)]
pub struct NetworkTransform {
    #[quantize(min = -1000.0, max = 1000.0, bits = 16)]
    pub position: Vec3,  // 6 bytes instead of 12
    #[quantize(smallest_three, bits = 10)]
    pub rotation: Quat,  // 4 bytes instead of 16
    #[quantize(min = 0.0, max = 20.0, bits = 8)]
    pub speed: f32,      // 1 byte instead of 4
}
```

Codegen generates quantize/dequantize from attributes. Total: 11 bytes vs 32 bytes (66% reduction).

**Priority-based bandwidth allocation:**

Each replicated entity has a priority score:

```text
priority = base_priority
         * distance_factor    // closer = higher
         * visibility_factor  // on-screen = higher
         * relevancy_factor   // interacting = higher
         * update_debt        // longer since update = higher
```

The replication scheduler fills each client's bandwidth budget with highest-priority entity updates
first. Low- priority entities may skip frames entirely.

**Packet batching:**

Multiple entity updates packed into single UDP packets up to MTU. Reduces per-packet overhead
(IP/UDP headers). Sort updates by priority so if packet is lost, the lowest priority data is at the
end.

**Jitter buffer tuning:**

Adaptive jitter buffer that adjusts interpolation delay based on measured jitter:

- Low jitter (< 5 ms): tight buffer, low latency
- High jitter (> 20 ms): wider buffer, smoother playback
- Spike detection: hold buffer size after spike, decay slowly

**Connection quality metrics:**

Expose to gameplay for UI and matchmaking:

```rust
pub struct ConnectionQuality {
    pub rtt_ms: f32,
    pub jitter_ms: f32,
    pub packet_loss_pct: f32,
    pub bandwidth_kbps: f32,
    pub quality_score: f32,  // 0-1 composite
}
```

Games can show connection quality indicator (green/yellow/red bars) and matchmaking can prefer
low-latency servers.

### RF-10: Server-client vs peer-to-peer networking

Support both models. The transport layer is model-agnostic — the authority and replication layers
adapt.

**Server-client (default):**

- Dedicated or listen server owns world state
- Clients send input, receive state updates
- Server-authoritative: server validates all actions
- Best for: competitive, MMO, large player counts
- Latency: client → server → client (1 RTT)

**Peer-to-peer:**

- No dedicated server — one peer is host, or fully distributed
- Host-authoritative: host peer validates actions
- Each peer replicates to all others (mesh topology) or host relays (star topology)

```rust
pub enum NetworkTopology {
    /// Dedicated or listen server. Clients connect to server.
    ClientServer {
        max_clients: u16,
    },
    /// One peer is host, others are clients. Host migrates
    /// on disconnect.
    HostMigrating {
        max_peers: u16,
    },
    /// All peers connected to all others. Lockstep or
    /// input-sharing.
    FullMesh {
        max_peers: u8, // limited by N² connections
    },
}
```

**Host migration (P2P):**

If the host disconnects:

1. Remaining peers elect new host (lowest latency or highest uptime)
2. New host receives world state from peers
3. Clients reconnect to new host
4. Brief pause (~1-2 seconds) during migration

**When to use which:**

| Game Type | Model | Why |
|-----------|-------|-----|
| MMO | Client-server (sharded) | Massive scale |
| Competitive FPS | Client-server (dedicated) | Anti-cheat |
| Co-op (2-4 players) | P2P host-migrating | No server cost |
| Fighting game | P2P full mesh + rollback | Minimal latency |
| Local multiplayer | N/A (same machine) | No network |

### RF-11: TCP and asset streaming

**No TCP for gameplay.** All real-time game traffic uses UDP with userspace reliability (RF-1).
TCP's head-of-line blocking and congestion backoff are unacceptable for real-time state.

**TCP for non-real-time:**

TCP is needed for:

- Asset downloading from CDN (HTTP/HTTPS)
- Matchmaking REST API calls
- Leaderboard/achievement submissions
- Chat messages (if not using UDP channel)
- Patch/update downloads

These use platform-native TCP:

| Platform | TCP API |
|----------|---------|
| Linux | io_uring TCP sockets via rustix |
| Windows | IOCP Winsock2 via windows-rs |
| Apple | Networking.framework (NWConnection TCP) |

**Asset streaming over network:**

### RF-12: QUIC as unified transport

Replace custom UDP reliability AND TCP with QUIC. One protocol for all network traffic:

| Use Case | QUIC Mode |
|----------|-----------|
| Game state replication | Unreliable datagrams |
| Voice chat | Unreliable datagrams |
| RPCs (reliable) | Reliable stream |
| Chat messages | Reliable ordered stream |
| Asset download | HTTP/3 (over QUIC) |
| REST API calls | HTTP/3 (over QUIC) |
| Patch downloads | HTTP/3 multiplexed streams |
| Reconnection | 0-RTT handshake |

**What QUIC eliminates:**

- Custom SACK-based UDP reliability layer (QUIC has built-in)
- Custom DTLS encryption (QUIC has TLS 1.3 built-in)
- Separate TCP transport for non-real-time
- Separate HTTPS stack for REST/CDN
- Custom fragmentation (QUIC handles it)
- Custom congestion control (QUIC has its own, or use BBR)

**Platform implementation:**

| Platform | QUIC Implementation |
|----------|-------------------|
| Apple | Networking.framework `NWProtocolQUIC` (native) |
| Windows | MsQuic via windows-rs (native, Windows 11+) |
| Linux | quinn-proto + io_uring (our I/O layer) |

`quinn-proto` is a pure state machine — no Tokio dependency. We feed it UDP datagrams from our
io_uring socket and send what it produces:

```text
io_uring UDP recv → quinn-proto.handle_datagram()
quinn-proto.poll_transmit() → io_uring UDP send
quinn-proto produces streams/datagrams → game reads them
```

On Apple and Windows, prefer the native QUIC API for best performance and OS integration. Use
quinn-proto as the portable fallback on Linux and as the reference for testing.

**Channel mapping to QUIC:**

| Old Channel | QUIC Equivalent |
|------------|----------------|
| ReliableOrdered | QUIC stream (ordered) |
| ReliableUnordered | QUIC stream (unidirectional) |
| UnreliableSequenced | QUIC datagram + sequence number |
| UnreliableUnordered | QUIC datagram |

**Asset streaming via HTTP/3:**

1. CDN serves assets via HTTP/3 (over QUIC)
2. Client opens QUIC connection to CDN
3. Multiple assets download in parallel (multiplexed)
4. Responses streamed to disk cache via platform I/O
5. Asset pipeline loads from cache via Handle<T>
6. Residency manager handles network-fetched assets
7. 0-RTT on reconnect — no handshake delay

**Dependencies:**

| Crate | Purpose |
|-------|---------|
| quinn-proto | QUIC state machine (Linux) |
| rustls | TLS 1.3 for quinn-proto |

quinn-proto + rustls are pure Rust, no C dependencies. On Apple/Windows, the native QUIC stack
handles TLS internally.

**Migration path:** The existing packet header, channel abstraction, and replication layer sit above
transport. Replacing custom UDP with QUIC only changes the transport implementation — the
replication, delta compression, prediction, and rollback layers remain unchanged.
