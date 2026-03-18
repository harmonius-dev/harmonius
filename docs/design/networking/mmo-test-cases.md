# MMO Infrastructure Test Cases

Companion test cases for [mmo.md](mmo.md).

## Unit Tests

### TC-8.7.1.1 Shard Assignment Least Populated

| # | Requirement |
|---|-------------|
| 1 | R-8.7.1     |
| 2 | R-8.7.1     |

1. **#1** — Shard A: 400 players, Shard B: 200 players; assign with `LeastPopulated`
   - **Expected:** ShardId for Shard B returned
2. **#2** — All shards at equal population
   - **Expected:** Any shard returned (deterministic tiebreak)

### TC-8.7.1.2 Shard Assignment Specific

| # | Requirement |
|---|-------------|
| 1 | R-8.7.1     |
| 2 | R-8.7.1     |
| 3 | R-8.7.1     |

1. **#1** — assign(account_1, Specific(shard_b)) where shard_b has capacity
   - **Expected:** `Ok(shard_b)`
2. **#2** — assign(account_1, Specific(shard_b)) where shard_b is at max_players
   - **Expected:** `Err(AtCapacity)`
3. **#3** — assign(account_1, SameAs(account_2)) where account_2 is on shard_a
   - **Expected:** `Ok(shard_a)`

### TC-8.7.1.3 Shard Merge

| # | Requirement |
|---|-------------|
| 1 | R-8.7.1     |
| 2 | R-8.7.1     |

1. **#1** — merge(shard_a: 100 players, shard_b: 150 players)
   - **Expected:** `Ok(())`, target shard has 250 players, source shard state=Draining
2. **#2** — All players from source shard lookup returns target ShardId
   - **Expected:** Player reassignment verified

### TC-8.7.1.4 Instance Lockout

| # | Requirement |
|---|-------------|
| 1 | R-8.7.1     |
| 2 | R-8.7.1     |

1. **#1** — Create instance (template=1, Heroic), complete it, check_lockout(account, template=1,
   Heroic)
   - **Expected:** `LockoutStatus { locked: true, expires_at: Some(time + lockout_hours) }`
2. **#2** — check_lockout(account, template=1, Normal) after completing Heroic
   - **Expected:** `LockoutStatus { locked: false }` (different difficulty)

### TC-8.7.3.1 Spatial Region Contains

| # | Requirement |
|---|-------------|
| 1 | R-8.7.3     |
| 2 | R-8.7.3     |
| 3 | R-8.7.3     |
| 4 | R-8.7.3     |

1. **#1** — region min=(0,0,0) max=(100,100,100), pos=(50,50,50)
   - **Expected:** `true`
2. **#2** — region min=(0,0,0) max=(100,100,100), pos=(150,50,50)
   - **Expected:** `false`
3. **#3** — region min=(0,0,0) max=(100,100,100), pos=(0,0,0)
   - **Expected:** `true` (boundary inclusive)
4. **#4** — region min=(0,0,0) max=(100,100,100), pos=(100,100,100)
   - **Expected:** `true` (boundary inclusive)

### TC-8.7.3.2 Spatial Region Split

| # | Requirement |
|---|-------------|
| 1 | R-8.7.3     |
| 2 | R-8.7.3     |

1. **#1** — region min=(0,0,0) max=(200,100,100) (longest axis=X)
   - **Expected:** Two regions: (0,0,0)-(100,100,100) and (100,0,0)-(200,100,100)
2. **#2** — Combined area of two halves
   - **Expected:** Equals original region area

### TC-8.7.3.3 Mesh Route Player

| # | Requirement |
|---|-------------|
| 1 | R-8.7.3     |
| 2 | R-8.7.3     |
| 3 | R-8.7.3     |

1. **#1** —
   regions=[(0,0,0)-(100,100,100) owned by server_1, (100,0,0)-(200,100,100) owned by server_2],
   pos=(50,50,50)
   - **Expected:** `Ok(server_1)`
2. **#2** — pos=(150,50,50)
   - **Expected:** `Ok(server_2)`
3. **#3** — pos=(500,500,500) outside all regions
   - **Expected:** `Err(MeshError::RegionNotFound)`

### TC-8.7.4.1 Migration Payload Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-8.7.4     |
| 2 | R-8.7.4     |

1. **#1** — Serialize MigrationPayload with entity_snapshot, 3 buffs, 2 cooldowns, 5 pending_rpcs,
   10 prediction frames
   - **Expected:** Deserialized payload has identical field values
2. **#2** — payload.initiated_at preserved across serialization
   - **Expected:** Timestamps match exactly

### TC-8.7.4.2 Migration Payload Buffs Preserved

| # | Requirement |
|---|-------------|
| 1 | R-8.7.4     |
| 2 | R-8.7.4     |

1. **#1** — BuffState { buff_id: 42, remaining_ticks: 100, stacks: 3 } serialized and deserialized
   - **Expected:** remaining_ticks=100, stacks=3 preserved
2. **#2** — CooldownState { ability_id: 7, remaining_ticks: 50 } serialized and deserialized
   - **Expected:** remaining_ticks=50 preserved

### TC-8.7.5.1 Transaction Commit

| # | Requirement |
|---|-------------|
| 1 | R-8.7.5     |
| 2 | R-8.7.5     |

1. **#1** — Begin transaction, execute 3 statements (debit, credit, transfer), commit
   - **Expected:** All 3 statements applied, rows_affected > 0 for each
2. **#2** — Query balances after commit
   - **Expected:** Debit and credit reflected

### TC-8.7.5.2 Transaction Rollback

| # | Requirement |
|---|-------------|
| 1 | R-8.7.5     |
| 2 | R-8.7.5     |

1. **#1** — Begin transaction, execute debit, simulate error, rollback
   - **Expected:** Original balances unchanged
2. **#2** — Query after rollback
   - **Expected:** No partial state from transaction

### TC-8.7.8.1 Bus PubSub Delivery

| # | Requirement |
|---|-------------|
| 1 | R-8.7.8     |
| 2 | R-8.7.8     |

1. **#1** — 3 subscribers on "world_events" channel, publish 1 message
   - **Expected:** All 3 subscribers receive the message
2. **#2** — Subscriber on different channel ("telemetry")
   - **Expected:** Does not receive "world_events" message

### TC-8.7.8.2 Bus Point-to-Point

| # | Requirement |
|---|-------------|
| 1 | R-8.7.8     |
| 2 | R-8.7.8     |

1. **#1** — send(target=server_2, channel="migration", payload) with 3 connected peers
   - **Expected:** Only server_2 receives the message
2. **#2** — Other peers (server_1, server_3)
   - **Expected:** Receive nothing

### TC-8.7.8.3 Bus Exactly-Once Deduplication

| # | Requirement |
|---|-------------|
| 1 | R-8.7.8     |
| 2 | R-8.7.8     |

1. **#1** — Send economy message with sequence=42, retransmit same message with sequence=42
   - **Expected:** Handler invoked exactly once
2. **#2** — Send message with sequence=43
   - **Expected:** Handler invoked (new sequence)

### TC-8.7.8.4 Bus Auto-Reconnect

| # | Requirement |
|---|-------------|
| 1 | R-8.7.8     |
| 2 | R-8.7.8     |

1. **#1** — Disconnect peer server_2, wait up to 5 s
   - **Expected:** peer_count restored, connection re-established within 5 s
2. **#2** — Messages sent during reconnect window
   - **Expected:** Delivered after reconnection (at-least-once channels)

### TC-8.7.2.1 Overlap Sync

| # | Requirement |
|---|-------------|
| 1 | R-8.7.2     |
| 2 | R-8.7.2     |

1. **#1** — 5 entities in overlap zone, sync to adjacent server
   - **Expected:** Adjacent server creates 5 ghost entities with matching positions
2. **#2** — Entity leaves overlap zone
   - **Expected:** Ghost entity removed on adjacent server

### TC-8.7.7.1 Auction Concurrent Bid and Buyout

| # | Requirement |
|---|-------------|
| 1 | R-8.7.7     |
| 2 | R-8.7.7     |

1. **#1** — listing with buyout_price=1000; concurrent bid(500) and buyout(1000)
   - **Expected:** Exactly one succeeds; other gets `Err(AlreadySold)` or `Err(Outbid)`
2. **#2** — Winning transaction is atomic
   - **Expected:** Item transferred, gold debited/credited

## Integration Tests

### TC-8.7.2.I1 Seamless Zone Transition

| # | Requirement |
|---|-------------|
| 1 | R-8.7.2     |
| 2 | R-8.7.2     |

1. **#1** — Move player from zone A to zone B across boundary
   - **Expected:** No loading screen, no disconnect, position/buffs/cooldowns preserved
2. **#2** — Client receives redirect and reconnects to zone B
   - **Expected:** Full state snapshot received, play continues

### TC-8.7.2.I2 Overlap No Pop-in

| # | Requirement |
|---|-------------|
| 1 | R-8.7.2     |
| 2 | R-8.7.2     |

1. **#1** — NPC near zone boundary, observed from both zones
   - **Expected:** NPC visible as ghost entity from adjacent zone
2. **#2** — Player crosses boundary where NPC is near edge
   - **Expected:** No entity pop-in or disappearance

### TC-8.7.3.I1 Mesh Split 500 Players

| # | Requirement |
|---|-------------|
| 1 | R-8.7.3     |
| 2 | R-8.7.3     |

1. **#1** — Concentrate 500 players in single region exceeding split_threshold
   - **Expected:** MeshController triggers split within 10 s
2. **#2** — After split
   - **Expected:** Two regions, each with subset of players, routing table updated

### TC-8.7.3.I2 Mesh Merge After Disperse

| # | Requirement |
|---|-------------|
| 1 | R-8.7.3     |
| 2 | R-8.7.3     |

1. **#1** — Two adjacent regions both below merge_threshold
   - **Expected:** MeshController triggers merge within 30 s
2. **#2** — After merge
   - **Expected:** Single region, all entities consolidated, server released

### TC-8.7.4.I1 Migration Mid-Combat

| # | Requirement |
|---|-------------|
| 1 | R-8.7.4     |

1. **#1** — Player in combat with active buffs, cooldowns, pending RPCs crosses zone boundary
   - **Expected:** All buffs (remaining_ticks preserved), cooldowns, pending RPCs intact on
     destination

### TC-8.7.4.I2 Migration Under 100ms

| # | Requirement |
|---|-------------|
| 1 | R-8.7.4     |

1. **#1** — Measure wall time from freeze_player to client receiving snapshot on destination
   - **Expected:** < 100 ms

### TC-8.7.5.I1 DB 10K Concurrent Saves

| # | Requirement |
|---|-------------|
| 1 | R-8.7.5     |

1. **#1** — 10,000 character save operations submitted concurrently
   - **Expected:** All complete successfully, simulation tick not blocked

### TC-8.7.5.I2 DB Trade Atomicity

| # | Requirement |
|---|-------------|
| 1 | R-8.7.5     |

1. **#1** — Begin trade transaction (debit + credit + transfer), simulate crash after debit
   - **Expected:** All operations rolled back, original balances restored

### TC-8.7.5.I3 DB Sustained 10K TPS

| # | Requirement |
|---|-------------|
| 1 | R-8.7.5     |

1. **#1** — Sustained write load for 60 s
   - **Expected:** Throughput >= 10,000 TPS

### TC-8.7.6.I1 Autoscale Surge

| # | Requirement |
|---|-------------|
| 1 | R-8.7.6     |

1. **#1** — Simulate 5,000 player surge pushing CPU above scale_up_cpu threshold
   - **Expected:** New servers provisioned within 30 s

### TC-8.7.6.I2 Autoscale Drain

| # | Requirement |
|---|-------------|
| 1 | R-8.7.6     |

1. **#1** — Simulate population decline below scale_down_cpu for cooldown_seconds
   - **Expected:** Server set to draining, players migrated, then server terminated

### TC-8.7.7.I1 Cross-Shard Auction

| # | Requirement |
|---|-------------|
| 1 | R-8.7.7     |

1. **#1** — List item on shard A, buyout from player on shard B
   - **Expected:** Item delivered to buyer on shard B, gold transferred to seller on shard A

### TC-8.7.7.I2 Cross-Shard Mail

| # | Requirement |
|---|-------------|
| 1 | R-8.7.7     |

1. **#1** — Send mail with item attachment from shard A player to shard B player
   - **Expected:** Mail appears in recipient inbox, attachment claimable

### TC-8.7.7.I3 Cross-Shard Concurrent Auction

| # | Requirement |
|---|-------------|
| 1 | R-8.7.7     |

1. **#1** — Concurrent bid from shard A and buyout from shard B on same listing
   - **Expected:** Deterministic resolution: exactly one wins

## Benchmarks

### TC-8.7.2.B1 Zone Transition Latency

| # | Metric            | Target          | Requirement |
|---|-------------------|-----------------|-------------|
| 1 | Perceived latency | 0 ms (seamless) | R-8.7.2     |

1. **1** — Player crosses zone boundary (perceived delay)

### TC-8.7.4.B1 Migration Handoff Latency

| # | Metric    | Target   | Requirement |
|---|-----------|----------|-------------|
| 1 | Wall time | < 100 ms | R-8.7.4     |

1. **1** — Full migration: freeze, serialize, transfer, deserialize, redirect

### TC-8.7.3.B1 Mesh Split Reaction Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Region exceeds split_threshold density | Time to split completion | < 10 s | R-8.7.3 |

### TC-8.7.3.B2 Mesh Merge Reaction Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Two adjacent regions below merge_threshold | Time to merge completion | < 30 s | R-8.7.3 |

### TC-8.7.5.B1 DB Write Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sustained write load across server fleet | Transactions per second | > 10,000 TPS | R-8.7.5 |

### TC-8.7.5.B2 DB Query Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Parameterized query under load | p99 latency | < 10 ms | R-8.7.5 |

### TC-8.7.6.B1 AutoScaler Provision Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Provision new server process via Kubernetes | Time to ready | < 30 s | R-8.7.6 |

### TC-8.7.8.B1 Bus Message Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Point-to-point message delivery under normal load | p99 latency | < 5 ms | R-8.7.8 |

### TC-8.7.8.B2 Bus Reconnection Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Peer disconnects and auto-reconnects | Time to reconnection | < 5 s | R-8.7.8 |
