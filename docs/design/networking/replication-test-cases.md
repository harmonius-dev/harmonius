# Network Replication, Prediction, and Rollback Test Cases

Companion test cases for [replication.md](replication.md).

## Unit Tests

### TC-8.2.1.1 Delta Single Field Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20-field component, modify field 7 only, compute_delta against baseline | Delta contains only field 7; 19 fields absent | R-8.2.1 |
| 2 | Client applies delta to baseline | Reconstructed state matches server state for all 20 fields | R-8.2.1 |

### TC-8.2.1.2 Delta No Change No Payload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Component unchanged since last baseline | `compute_delta` returns `None` | R-8.2.1 |
| 2 | Zero bytes transmitted for this component | Bandwidth consumption = 0 | R-8.2.1 |

### TC-8.2.1.3 Delta Quantization Precision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Position delta of 1.2345 m, quantized at 0.01 m precision | Decoded value within 0.01 m of 1.2345 (i.e., 1.23 or 1.24) | R-8.2.1 |
| 2 | Position delta of 0.001 m, quantized at 0.01 m precision | Decoded value = 0.00 m (below precision) | R-8.2.1 |

### TC-8.2.2.1 Schema Negotiation Compatible

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Server schema v2 (1 added field with default), client schema v1 | Negotiation succeeds, client applies default for new field | R-8.2.2 |
| 2 | Client state after negotiation | New field has configured default value | R-8.2.2 |

### TC-8.2.2.2 Schema Negotiation Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Server schema v3 (breaking change), client schema v1 | `Err(SchemaIncompatible)` | R-8.2.2 |

### TC-8.2.3.1 AOI Spatial Filtering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1,000 entities in zone, client AOI radius=50m, 200 entities within radius | Client receives exactly 200 entities | R-8.2.3 |
| 2 | Entity at 51m from client | Not included in replication set | R-8.2.3 |

### TC-8.2.3.2 AOI Always Relevant Party

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Party member at 200m (outside spatial AOI of 50m), marked AlwaysRelevant | Party member included in replication set | R-8.2.3 |
| 2 | Non-party entity at 200m, no AlwaysRelevant | Not included in replication set | R-8.2.3 |

### TC-8.2.4.1 Owner Only Visibility

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Property marked `OwnerOnly` on entity owned by client A | Client A receives property, client B does not | R-8.2.4 |

### TC-8.2.4.2 Tiered Update Rate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity at 400m from client (far tier) | Update rate drops to far-tier Hz | R-8.2.4 |
| 2 | Same entity moves to 10m from client (near tier) | Update rate increases to full-rate Hz | R-8.2.4 |

### TC-8.2.5.1 Priority High First

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 entities, 50 KB budget, 10 entities marked high-priority (target, hostiles) | All 10 high-priority entities in every tick's send list | R-8.2.5 |
| 2 | Low-priority entities | Deferred when budget is exhausted | R-8.2.5 |

### TC-8.2.5.2 Priority Staleness Boost

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity deferred for 10 consecutive ticks | Priority boosted by staleness multiplier (10x base) | R-8.2.5 |
| 2 | Boosted entity | Appears in next tick's send list despite low base priority | R-8.2.5 |

### TC-8.2.6.1 Dormancy Enter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity unchanged for dormancy threshold ticks (e.g., 300 ticks at 60 Hz = 5 s) | Entity enters dormancy state | R-8.2.6 |
| 2 | Dormant entity | Zero bandwidth consumed for this entity | R-8.2.6 |

### TC-8.2.6.2 Dormancy Wake on Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dormant entity, modify one property | Entity wakes, included in next tick's replication set | R-8.2.6 |

### TC-8.2.6.3 Dormancy Wake Explicit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call wake() on dormant entity | Entity replicated next tick | R-8.2.6 |

### TC-8.4.1.1 Prediction Immediate Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Issue move_forward input at frame N | Client applies movement at frame N (before server response arrives) | R-8.4.1 |

### TC-8.4.1.2 Reconcile on Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client predicted pos=(10,0,10), server correction pos=(10,0,9.5) | Client replays unacked inputs from correction, converges within 1 frame | R-8.4.1 |
| 2 | Final client position | Matches server state + replayed unacked inputs | R-8.4.1 |

### TC-8.4.1.3 Reconcile No Correction on Match

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client predicted pos=(10,0,10), server confirms pos=(10,0,10) | No rollback or replay occurs | R-8.4.1 |

### TC-8.4.2.1 Input Buffer Redundancy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build input packet with redundancy=3, current input=move_forward | Packet contains current input + 3 redundant copies of previous inputs | R-8.4.2 |

### TC-8.4.2.2 Input Buffer Loss Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 100 input packets with 10% random loss, redundancy=3 | Server processes all 100 inputs in order | R-8.4.2 |

### TC-8.4.3.1 Interpolation Smooth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Snapshot A: pos=(0,0,0) at t=0ms, Snapshot B: pos=(10,0,0) at t=50ms, sample at alpha=0.5 | Interpolated pos=(5,0,0) | R-8.4.3 |

### TC-8.4.3.2 Interpolation Adapts Jitter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Introduce 30 ms jitter in snapshot arrival | Interpolation delay expands to absorb jitter | R-8.4.3 |
| 2 | Remove jitter, stabilize arrivals | Interpolation delay contracts within 5 s | R-8.4.3 |

### TC-8.4.4.1 Extrapolation No Freeze

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Last snapshot at t=0, no new snapshot for 200 ms, entity velocity=(5,0,0) | Entity extrapolated to pos=(1,0,0) at t=200ms | R-8.4.4 |

### TC-8.4.4.2 Error Correction Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 1.0 m position error correction | Error decays smoothly to below snap_threshold | R-8.4.4 |

### TC-8.4.4.3 Error Correction Teleport

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply error exceeding teleport_threshold (e.g., 10 m) | Instant snap to correct position, no smooth correction | R-8.4.4 |

### TC-8.4.5.1 Lag Comp Rewind Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attacker at 100 ms RTT fires at target moving at 5 m/s, target was at hit position 100 ms ago | Server rewinds hitbox 100 ms, registers hit | R-8.4.5 |

### TC-8.4.5.2 Lag Comp Max Window

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attacker at 300 ms RTT, max rewind window=250 ms | Rewind clamped to 250 ms, not 300 ms | R-8.4.5 |

### TC-8.4.5.3 Lag Comp Favor Defender

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ambiguous hit at hitbox boundary after rewind | Server rules miss (defender favored) | R-8.4.5 |

### TC-8.4.6.1 Jitter Buffer Steady Release

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 100 snapshots with 0-50 ms random jitter | Release cadence is steady (no stutter), all snapshots delivered | R-8.4.6 |

### TC-8.4.6.2 Jitter Buffer Adapts

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Increase jitter from 20 ms to 80 ms | Buffer depth expands | R-8.4.6 |
| 2 | Stabilize jitter back to 20 ms | Buffer depth contracts within 5 s | R-8.4.6 |

### TC-8.3.1.1 RPC Server Valid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Invoke server RPC "use_ability" with valid params (ability_id=5, target=entity_42) | Handler executes, `Ok(())` returned | R-8.3.1 |

### TC-8.3.1.2 RPC Server Invalid Params

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Invoke "use_ability" with ability_id=99999 (nonexistent) | `Err(OutOfRange)` | R-8.3.1 |

### TC-8.3.1.3 RPC Server Rate Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Invoke same RPC 100 times in 1 second (limit=10/s) | `Err(RateLimited)` after 10th call | R-8.3.1 |

### TC-8.3.1.4 RPC Server Malformed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send truncated 3-byte payload for RPC expecting 16 bytes | `Err(MalformedPayload)`, no crash | R-8.3.1 |

### TC-8.3.2.1 RPC Client Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Server invokes client RPC targeting client_A among 50 connected clients | client_A receives RPC, other 49 clients do not | R-8.3.2 |

### TC-8.3.3.1 RPC Multicast Spatial

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Multicast explosion VFX to 100 clients within 50m radius, 20 clients outside | All 100 in-range clients receive, 20 out-of-range excluded | R-8.3.3 |

### TC-8.3.4.1 RPC Reliable Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send reliable RPC over 10% loss link | Delivered within retransmission timeout | R-8.3.4 |

### TC-8.3.4.2 RPC Unreliable No Retransmit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 1,000 unreliable RPCs | Zero retransmissions observed | R-8.3.4 |

### TC-8.3.4.3 RPC Reliable Latest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 10 rapid reliable-latest RPCs (values 1-10) | Only value 10 delivered to recipient | R-8.3.4 |

### TC-8.3.5.1 RPC Param Serialization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Round-trip serialize: i32=42, f32=3.14, String="hello", Entity=entity_7, enum=Variant::B | Deserialized values match originals exactly | R-8.3.5 |

## Integration Tests

### TC-8.2.1.I1 Replication 10K Entities

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 entities, 5% change rate per tick, delta replication | Bandwidth 80%+ lower than full-state replication | R-8.2.1 |

### TC-8.2.3.I1 AOI 1000 Clients

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100,000 entities, 1,000 clients, AOI evaluation per tick | Completes within 2 ms per tick | R-8.2.3 |

### TC-8.2.2.I1 Cross Version Handshake

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client v1 connects to server v2 (one added field) | Handshake succeeds, replication works with defaults for new field | R-8.2.2 |

### TC-8.4.1.I1 Reconcile at 100ms RTT

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Client at 100 ms RTT, issue movement input | Immediate local response (< 1 frame) | R-8.4.1 |
| 2 | Inject server mismatch (2m correction) | Smooth correction with no visible teleport | R-8.4.1 |

### TC-8.4.1.I2 Mobile Rollback 4 Frames

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile client, inject mismatch requiring rollback | Replay limited to 4 frames maximum | R-8.4.1 |

### TC-8.4.3.I1 Interpolation 144fps at 20Hz

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 144 FPS client, 20 Hz server tick rate | Smooth remote entity motion (no stutter or snapping) | R-8.4.3 |

### TC-8.4.5.I1 Lag Comp Deterministic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same fire input replayed at RTT=20/50/100/200 ms | Identical hit/miss results for each RTT | R-8.4.5 |

### TC-8.2.5.I1 Budget Respects Congestion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget=50 KB/s, 500 entities | Send rate never exceeds 50 KB/s in any 1-second window | R-8.2.5 |

### TC-8.2.6.I1 Dormancy 70 Percent Idle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 entities, 7,000 unchanged | 7,000 dormant entities consume zero bandwidth | R-8.2.6 |

### TC-8.3.3.I1 Multicast vs Individual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Multicast to 100 clients vs 100 individual RPC sends | Multicast is 5x+ more CPU-efficient | R-8.3.3 |

### TC-8.3.5.I1 RPC Fuzz

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100,000 randomized/malformed RPC payloads | Zero crashes, all invalid payloads rejected cleanly | R-8.3.5 |

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
