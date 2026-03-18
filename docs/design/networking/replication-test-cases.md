# Network Replication, Prediction, and Rollback Test Cases

Companion test cases for [replication.md](replication.md).

## Unit Tests

### TC-8.2.1.1 Delta Single Field Change

| # | Requirement |
|---|-------------|
| 1 | R-8.2.1     |
| 2 | R-8.2.1     |

1. **#1** — 20-field component, modify field 7 only, compute_delta against baseline
   - **Expected:** Delta contains only field 7; 19 fields absent
2. **#2** — Client applies delta to baseline
   - **Expected:** Reconstructed state matches server state for all 20 fields

### TC-8.2.1.2 Delta No Change No Payload

| # | Requirement |
|---|-------------|
| 1 | R-8.2.1     |
| 2 | R-8.2.1     |

1. **#1** — Component unchanged since last baseline
   - **Expected:** `compute_delta` returns `None`
2. **#2** — Zero bytes transmitted for this component
   - **Expected:** Bandwidth consumption = 0

### TC-8.2.1.3 Delta Quantization Precision

| # | Requirement |
|---|-------------|
| 1 | R-8.2.1     |
| 2 | R-8.2.1     |

1. **#1** — Position delta of 1.2345 m, quantized at 0.01 m precision
   - **Expected:** Decoded value within 0.01 m of 1.2345 (i.e., 1.23 or 1.24)
2. **#2** — Position delta of 0.001 m, quantized at 0.01 m precision
   - **Expected:** Decoded value = 0.00 m (below precision)

### TC-8.2.2.1 Schema Negotiation Compatible

| # | Requirement |
|---|-------------|
| 1 | R-8.2.2     |
| 2 | R-8.2.2     |

1. **#1** — Server schema v2 (1 added field with default), client schema v1
   - **Expected:** Negotiation succeeds, client applies default for new field
2. **#2** — Client state after negotiation
   - **Expected:** New field has configured default value

### TC-8.2.2.2 Schema Negotiation Reject

| # | Requirement |
|---|-------------|
| 1 | R-8.2.2     |

1. **#1** — Server schema v3 (breaking change), client schema v1
   - **Expected:** `Err(SchemaIncompatible)`

### TC-8.2.3.1 AOI Spatial Filtering

| # | Requirement |
|---|-------------|
| 1 | R-8.2.3     |
| 2 | R-8.2.3     |

1. **#1** — 1,000 entities in zone, client AOI radius=50m, 200 entities within radius
   - **Expected:** Client receives exactly 200 entities
2. **#2** — Entity at 51m from client
   - **Expected:** Not included in replication set

### TC-8.2.3.2 AOI Always Relevant Party

| # | Requirement |
|---|-------------|
| 1 | R-8.2.3     |
| 2 | R-8.2.3     |

1. **#1** — Party member at 200m (outside spatial AOI of 50m), marked AlwaysRelevant
   - **Expected:** Party member included in replication set
2. **#2** — Non-party entity at 200m, no AlwaysRelevant
   - **Expected:** Not included in replication set

### TC-8.2.4.1 Owner Only Visibility

| # | Requirement |
|---|-------------|
| 1 | R-8.2.4     |

1. **#1** — Property marked `OwnerOnly` on entity owned by client A
   - **Expected:** Client A receives property, client B does not

### TC-8.2.4.2 Tiered Update Rate

| # | Requirement |
|---|-------------|
| 1 | R-8.2.4     |
| 2 | R-8.2.4     |

1. **#1** — Entity at 400m from client (far tier)
   - **Expected:** Update rate drops to far-tier Hz
2. **#2** — Same entity moves to 10m from client (near tier)
   - **Expected:** Update rate increases to full-rate Hz

### TC-8.2.5.1 Priority High First

| # | Requirement |
|---|-------------|
| 1 | R-8.2.5     |
| 2 | R-8.2.5     |

1. **#1** — 500 entities, 50 KB budget, 10 entities marked high-priority (target, hostiles)
   - **Expected:** All 10 high-priority entities in every tick's send list
2. **#2** — Low-priority entities
   - **Expected:** Deferred when budget is exhausted

### TC-8.2.5.2 Priority Staleness Boost

| # | Requirement |
|---|-------------|
| 1 | R-8.2.5     |
| 2 | R-8.2.5     |

1. **#1** — Entity deferred for 10 consecutive ticks
   - **Expected:** Priority boosted by staleness multiplier (10x base)
2. **#2** — Boosted entity
   - **Expected:** Appears in next tick's send list despite low base priority

### TC-8.2.6.1 Dormancy Enter

| # | Requirement |
|---|-------------|
| 1 | R-8.2.6     |
| 2 | R-8.2.6     |

1. **#1** — Entity unchanged for dormancy threshold ticks (e.g., 300 ticks at 60 Hz = 5 s)
   - **Expected:** Entity enters dormancy state
2. **#2** — Dormant entity
   - **Expected:** Zero bandwidth consumed for this entity

### TC-8.2.6.2 Dormancy Wake on Change

| # | Requirement |
|---|-------------|
| 1 | R-8.2.6     |

1. **#1** — Dormant entity, modify one property
   - **Expected:** Entity wakes, included in next tick's replication set

### TC-8.2.6.3 Dormancy Wake Explicit

| # | Requirement |
|---|-------------|
| 1 | R-8.2.6     |

1. **#1** — Call wake() on dormant entity
   - **Expected:** Entity replicated next tick

### TC-8.4.1.1 Prediction Immediate Response

| # | Requirement |
|---|-------------|
| 1 | R-8.4.1     |

1. **#1** — Issue move_forward input at frame N
   - **Expected:** Client applies movement at frame N (before server response arrives)

### TC-8.4.1.2 Reconcile on Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-8.4.1     |
| 2 | R-8.4.1     |

1. **#1** — Client predicted pos=(10,0,10), server correction pos=(10,0,9.5)
   - **Expected:** Client replays unacked inputs from correction, converges within 1 frame
2. **#2** — Final client position
   - **Expected:** Matches server state + replayed unacked inputs

### TC-8.4.1.3 Reconcile No Correction on Match

| # | Requirement |
|---|-------------|
| 1 | R-8.4.1     |

1. **#1** — Client predicted pos=(10,0,10), server confirms pos=(10,0,10)
   - **Expected:** No rollback or replay occurs

### TC-8.4.2.1 Input Buffer Redundancy

| # | Requirement |
|---|-------------|
| 1 | R-8.4.2     |

1. **#1** — Build input packet with redundancy=3, current input=move_forward
   - **Expected:** Packet contains current input + 3 redundant copies of previous inputs

### TC-8.4.2.2 Input Buffer Loss Recovery

| # | Requirement |
|---|-------------|
| 1 | R-8.4.2     |

1. **#1** — Send 100 input packets with 10% random loss, redundancy=3
   - **Expected:** Server processes all 100 inputs in order

### TC-8.4.3.1 Interpolation Smooth

| # | Requirement |
|---|-------------|
| 1 | R-8.4.3     |

1. **#1** — Snapshot A: pos=(0,0,0) at t=0ms, Snapshot B: pos=(10,0,0) at t=50ms, sample at
   alpha=0.5
   - **Expected:** Interpolated pos=(5,0,0)

### TC-8.4.3.2 Interpolation Adapts Jitter

| # | Requirement |
|---|-------------|
| 1 | R-8.4.3     |
| 2 | R-8.4.3     |

1. **#1** — Introduce 30 ms jitter in snapshot arrival
   - **Expected:** Interpolation delay expands to absorb jitter
2. **#2** — Remove jitter, stabilize arrivals
   - **Expected:** Interpolation delay contracts within 5 s

### TC-8.4.4.1 Extrapolation No Freeze

| # | Requirement |
|---|-------------|
| 1 | R-8.4.4     |

1. **#1** — Last snapshot at t=0, no new snapshot for 200 ms, entity velocity=(5,0,0)
   - **Expected:** Entity extrapolated to pos=(1,0,0) at t=200ms

### TC-8.4.4.2 Error Correction Decay

| # | Requirement |
|---|-------------|
| 1 | R-8.4.4     |

1. **#1** — Apply 1.0 m position error correction
   - **Expected:** Error decays smoothly to below snap_threshold

### TC-8.4.4.3 Error Correction Teleport

| # | Requirement |
|---|-------------|
| 1 | R-8.4.4     |

1. **#1** — Apply error exceeding teleport_threshold (e.g., 10 m)
   - **Expected:** Instant snap to correct position, no smooth correction

### TC-8.4.5.1 Lag Comp Rewind Hit

| # | Requirement |
|---|-------------|
| 1 | R-8.4.5     |

1. **#1** — Attacker at 100 ms RTT fires at target moving at 5 m/s, target was at hit position 100
   ms ago
   - **Expected:** Server rewinds hitbox 100 ms, registers hit

### TC-8.4.5.2 Lag Comp Max Window

| # | Requirement |
|---|-------------|
| 1 | R-8.4.5     |

1. **#1** — Attacker at 300 ms RTT, max rewind window=250 ms
   - **Expected:** Rewind clamped to 250 ms, not 300 ms

### TC-8.4.5.3 Lag Comp Favor Defender

| # | Requirement |
|---|-------------|
| 1 | R-8.4.5     |

1. **#1** — Ambiguous hit at hitbox boundary after rewind
   - **Expected:** Server rules miss (defender favored)

### TC-8.4.6.1 Jitter Buffer Steady Release

| # | Requirement |
|---|-------------|
| 1 | R-8.4.6     |

1. **#1** — Insert 100 snapshots with 0-50 ms random jitter
   - **Expected:** Release cadence is steady (no stutter), all snapshots delivered

### TC-8.4.6.2 Jitter Buffer Adapts

| # | Requirement |
|---|-------------|
| 1 | R-8.4.6     |
| 2 | R-8.4.6     |

1. **#1** — Increase jitter from 20 ms to 80 ms
   - **Expected:** Buffer depth expands
2. **#2** — Stabilize jitter back to 20 ms
   - **Expected:** Buffer depth contracts within 5 s

### TC-8.3.1.1 RPC Server Valid

| # | Requirement |
|---|-------------|
| 1 | R-8.3.1     |

1. **#1** — Invoke server RPC "use_ability" with valid params (ability_id=5, target=entity_42)
   - **Expected:** Handler executes, `Ok(())` returned

### TC-8.3.1.2 RPC Server Invalid Params

| # | Requirement |
|---|-------------|
| 1 | R-8.3.1     |

1. **#1** — Invoke "use_ability" with ability_id=99999 (nonexistent)
   - **Expected:** `Err(OutOfRange)`

### TC-8.3.1.3 RPC Server Rate Limit

| # | Requirement |
|---|-------------|
| 1 | R-8.3.1     |

1. **#1** — Invoke same RPC 100 times in 1 second (limit=10/s)
   - **Expected:** `Err(RateLimited)` after 10th call

### TC-8.3.1.4 RPC Server Malformed

| # | Requirement |
|---|-------------|
| 1 | R-8.3.1     |

1. **#1** — Send truncated 3-byte payload for RPC expecting 16 bytes
   - **Expected:** `Err(MalformedPayload)`, no crash

### TC-8.3.2.1 RPC Client Delivery

| # | Requirement |
|---|-------------|
| 1 | R-8.3.2     |

1. **#1** — Server invokes client RPC targeting client_A among 50 connected clients
   - **Expected:** client_A receives RPC, other 49 clients do not

### TC-8.3.3.1 RPC Multicast Spatial

| # | Requirement |
|---|-------------|
| 1 | R-8.3.3     |

1. **#1** — Multicast explosion VFX to 100 clients within 50m radius, 20 clients outside
   - **Expected:** All 100 in-range clients receive, 20 out-of-range excluded

### TC-8.3.4.1 RPC Reliable Delivery

| # | Requirement |
|---|-------------|
| 1 | R-8.3.4     |

1. **#1** — Send reliable RPC over 10% loss link
   - **Expected:** Delivered within retransmission timeout

### TC-8.3.4.2 RPC Unreliable No Retransmit

| # | Requirement |
|---|-------------|
| 1 | R-8.3.4     |

1. **#1** — Send 1,000 unreliable RPCs
   - **Expected:** Zero retransmissions observed

### TC-8.3.4.3 RPC Reliable Latest

| # | Requirement |
|---|-------------|
| 1 | R-8.3.4     |

1. **#1** — Send 10 rapid reliable-latest RPCs (values 1-10)
   - **Expected:** Only value 10 delivered to recipient

### TC-8.3.5.1 RPC Param Serialization

| # | Requirement |
|---|-------------|
| 1 | R-8.3.5     |

1. **#1** — Round-trip serialize: i32=42, f32=3.14, String="hello", Entity=entity_7, enum=Variant::B
   - **Expected:** Deserialized values match originals exactly

## Integration Tests

### TC-8.2.1.I1 Replication 10K Entities

| # | Requirement |
|---|-------------|
| 1 | R-8.2.1     |

1. **#1** — 10,000 entities, 5% change rate per tick, delta replication
   - **Expected:** Bandwidth 80%+ lower than full-state replication

### TC-8.2.3.I1 AOI 1000 Clients

| # | Requirement |
|---|-------------|
| 1 | R-8.2.3     |

1. **#1** — 100,000 entities, 1,000 clients, AOI evaluation per tick
   - **Expected:** Completes within 2 ms per tick

### TC-8.2.2.I1 Cross Version Handshake

| # | Requirement |
|---|-------------|
| 1 | R-8.2.2     |

1. **#1** — Client v1 connects to server v2 (one added field)
   - **Expected:** Handshake succeeds, replication works with defaults for new field

### TC-8.4.1.I1 Reconcile at 100ms RTT

| # | Requirement |
|---|-------------|
| 1 | R-8.4.1     |
| 2 | R-8.4.1     |

1. **#1** — Client at 100 ms RTT, issue movement input
   - **Expected:** Immediate local response (< 1 frame)
2. **#2** — Inject server mismatch (2m correction)
   - **Expected:** Smooth correction with no visible teleport

### TC-8.4.1.I2 Mobile Rollback 4 Frames

| # | Requirement |
|---|-------------|
| 1 | R-8.4.1     |

1. **#1** — Mobile client, inject mismatch requiring rollback
   - **Expected:** Replay limited to 4 frames maximum

### TC-8.4.3.I1 Interpolation 144fps at 20Hz

| # | Requirement |
|---|-------------|
| 1 | R-8.4.3     |

1. **#1** — 144 FPS client, 20 Hz server tick rate
   - **Expected:** Smooth remote entity motion (no stutter or snapping)

### TC-8.4.5.I1 Lag Comp Deterministic

| # | Requirement |
|---|-------------|
| 1 | R-8.4.5     |

1. **#1** — Same fire input replayed at RTT=20/50/100/200 ms
   - **Expected:** Identical hit/miss results for each RTT

### TC-8.2.5.I1 Budget Respects Congestion

| # | Requirement |
|---|-------------|
| 1 | R-8.2.5     |

1. **#1** — Budget=50 KB/s, 500 entities
   - **Expected:** Send rate never exceeds 50 KB/s in any 1-second window

### TC-8.2.6.I1 Dormancy 70 Percent Idle

| # | Requirement |
|---|-------------|
| 1 | R-8.2.6     |

1. **#1** — 10,000 entities, 7,000 unchanged
   - **Expected:** 7,000 dormant entities consume zero bandwidth

### TC-8.3.3.I1 Multicast vs Individual

| # | Requirement |
|---|-------------|
| 1 | R-8.3.3     |

1. **#1** — Multicast to 100 clients vs 100 individual RPC sends
   - **Expected:** Multicast is 5x+ more CPU-efficient

### TC-8.3.5.I1 RPC Fuzz

| # | Requirement |
|---|-------------|
| 1 | R-8.3.5     |

1. **#1** — 100,000 randomized/malformed RPC payloads
   - **Expected:** Zero crashes, all invalid payloads rejected cleanly

## Benchmarks

### TC-8.2.1.B1 Delta Computation Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 entities, 5% dirty per tick | Delta computation time | < 1 ms | R-8.2.1 |

### TC-8.2.3.B1 AOI Evaluation Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1,000 clients, 100,000 entities | AOI evaluation per tick | < 2 ms | R-8.2.3 |

### TC-8.2.5.B1 Priority Scheduling Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 entities, 50 KB budget | Scheduling time | < 100 us | R-8.2.5 |

### TC-8.4.1.B1 Rollback Replay Desktop

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8-frame rollback replay on desktop | Replay time | < 2 ms | R-8.4.1 |

### TC-8.4.1.B2 Rollback Replay Mobile

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4-frame rollback replay on mobile | Replay time | < 1 ms | R-8.4.1 |

### TC-8.4.3.B1 Snapshot Interpolation Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Interpolate 1,000 entities | Total time | < 200 us | R-8.4.3 |

### TC-8.4.5.B1 Lag Compensation Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Rewind + raycast for single hit check | Total time | < 100 us | R-8.4.5 |

### TC-8.3.1.B1 RPC Validation Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single RPC validation | Latency per call | < 5 us | R-8.3.1 |

### TC-8.3.3.B1 Multicast Resolution Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Resolve multicast to 100 recipients | Total time | < 50 us | R-8.3.3 |

### TC-8.4.6.B1 Jitter Buffer Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Insert + release single snapshot | Total time | < 10 us | R-8.4.6 |
