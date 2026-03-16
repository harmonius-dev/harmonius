# MMO Infrastructure Test Cases

Companion test cases for [mmo.md](mmo.md).

## Unit Tests

### TC-8.7.1.1 Shard Assignment Least Populated

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Shard A: 400 players, Shard B: 200 players; assign with `LeastPopulated` | ShardId for Shard B returned | R-8.7.1 |
| 2 | All shards at equal population | Any shard returned (deterministic tiebreak) | R-8.7.1 |

### TC-8.7.1.2 Shard Assignment Specific

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | assign(account_1, Specific(shard_b)) where shard_b has capacity | `Ok(shard_b)` | R-8.7.1 |
| 2 | assign(account_1, Specific(shard_b)) where shard_b is at max_players | `Err(AtCapacity)` | R-8.7.1 |
| 3 | assign(account_1, SameAs(account_2)) where account_2 is on shard_a | `Ok(shard_a)` | R-8.7.1 |

### TC-8.7.1.3 Shard Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | merge(shard_a: 100 players, shard_b: 150 players) | `Ok(())`, target shard has 250 players, source shard state=Draining | R-8.7.1 |
| 2 | All players from source shard lookup returns target ShardId | Player reassignment verified | R-8.7.1 |

### TC-8.7.1.4 Instance Lockout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create instance (template=1, Heroic), complete it, check_lockout(account, template=1, Heroic) | `LockoutStatus { locked: true, expires_at: Some(time + lockout_hours) }` | R-8.7.1 |
| 2 | check_lockout(account, template=1, Normal) after completing Heroic | `LockoutStatus { locked: false }` (different difficulty) | R-8.7.1 |

### TC-8.7.3.1 Spatial Region Contains

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | region min=(0,0,0) max=(100,100,100), pos=(50,50,50) | `true` | R-8.7.3 |
| 2 | region min=(0,0,0) max=(100,100,100), pos=(150,50,50) | `false` | R-8.7.3 |
| 3 | region min=(0,0,0) max=(100,100,100), pos=(0,0,0) | `true` (boundary inclusive) | R-8.7.3 |
| 4 | region min=(0,0,0) max=(100,100,100), pos=(100,100,100) | `true` (boundary inclusive) | R-8.7.3 |

### TC-8.7.3.2 Spatial Region Split

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | region min=(0,0,0) max=(200,100,100) (longest axis=X) | Two regions: (0,0,0)-(100,100,100) and (100,0,0)-(200,100,100) | R-8.7.3 |
| 2 | Combined area of two halves | Equals original region area | R-8.7.3 |

### TC-8.7.3.3 Mesh Route Player

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | regions=[(0,0,0)-(100,100,100) owned by server_1, (100,0,0)-(200,100,100) owned by server_2], pos=(50,50,50) | `Ok(server_1)` | R-8.7.3 |
| 2 | pos=(150,50,50) | `Ok(server_2)` | R-8.7.3 |
| 3 | pos=(500,500,500) outside all regions | `Err(MeshError::RegionNotFound)` | R-8.7.3 |

### TC-8.7.4.1 Migration Payload Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Serialize MigrationPayload with entity_snapshot, 3 buffs, 2 cooldowns, 5 pending_rpcs, 10 prediction frames | Deserialized payload has identical field values | R-8.7.4 |
| 2 | payload.initiated_at preserved across serialization | Timestamps match exactly | R-8.7.4 |

### TC-8.7.4.2 Migration Payload Buffs Preserved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | BuffState { buff_id: 42, remaining_ticks: 100, stacks: 3 } serialized and deserialized | remaining_ticks=100, stacks=3 preserved | R-8.7.4 |
| 2 | CooldownState { ability_id: 7, remaining_ticks: 50 } serialized and deserialized | remaining_ticks=50 preserved | R-8.7.4 |

### TC-8.7.5.1 Transaction Commit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin transaction, execute 3 statements (debit, credit, transfer), commit | All 3 statements applied, rows_affected > 0 for each | R-8.7.5 |
| 2 | Query balances after commit | Debit and credit reflected | R-8.7.5 |

### TC-8.7.5.2 Transaction Rollback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin transaction, execute debit, simulate error, rollback | Original balances unchanged | R-8.7.5 |
| 2 | Query after rollback | No partial state from transaction | R-8.7.5 |

### TC-8.7.8.1 Bus PubSub Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 subscribers on "world_events" channel, publish 1 message | All 3 subscribers receive the message | R-8.7.8 |
| 2 | Subscriber on different channel ("telemetry") | Does not receive "world_events" message | R-8.7.8 |

### TC-8.7.8.2 Bus Point-to-Point

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | send(target=server_2, channel="migration", payload) with 3 connected peers | Only server_2 receives the message | R-8.7.8 |
| 2 | Other peers (server_1, server_3) | Receive nothing | R-8.7.8 |

### TC-8.7.8.3 Bus Exactly-Once Deduplication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send economy message with sequence=42, retransmit same message with sequence=42 | Handler invoked exactly once | R-8.7.8 |
| 2 | Send message with sequence=43 | Handler invoked (new sequence) | R-8.7.8 |

### TC-8.7.8.4 Bus Auto-Reconnect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disconnect peer server_2, wait up to 5 s | peer_count restored, connection re-established within 5 s | R-8.7.8 |
| 2 | Messages sent during reconnect window | Delivered after reconnection (at-least-once channels) | R-8.7.8 |

### TC-8.7.2.1 Overlap Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 entities in overlap zone, sync to adjacent server | Adjacent server creates 5 ghost entities with matching positions | R-8.7.2 |
| 2 | Entity leaves overlap zone | Ghost entity removed on adjacent server | R-8.7.2 |

### TC-8.7.7.1 Auction Concurrent Bid and Buyout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | listing with buyout_price=1000; concurrent bid(500) and buyout(1000) | Exactly one succeeds; other gets `Err(AlreadySold)` or `Err(Outbid)` | R-8.7.7 |
| 2 | Winning transaction is atomic | Item transferred, gold debited/credited | R-8.7.7 |

## Integration Tests

### TC-8.7.2.I1 Seamless Zone Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move player from zone A to zone B across boundary | No loading screen, no disconnect, position/buffs/cooldowns preserved | R-8.7.2 |
| 2 | Client receives redirect and reconnects to zone B | Full state snapshot received, play continues | R-8.7.2 |

### TC-8.7.2.I2 Overlap No Pop-in

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | NPC near zone boundary, observed from both zones | NPC visible as ghost entity from adjacent zone | R-8.7.2 |
| 2 | Player crosses boundary where NPC is near edge | No entity pop-in or disappearance | R-8.7.2 |

### TC-8.7.3.I1 Mesh Split 500 Players

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Concentrate 500 players in single region exceeding split_threshold | MeshController triggers split within 10 s | R-8.7.3 |
| 2 | After split | Two regions, each with subset of players, routing table updated | R-8.7.3 |

### TC-8.7.3.I2 Mesh Merge After Disperse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two adjacent regions both below merge_threshold | MeshController triggers merge within 30 s | R-8.7.3 |
| 2 | After merge | Single region, all entities consolidated, server released | R-8.7.3 |

### TC-8.7.4.I1 Migration Mid-Combat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player in combat with active buffs, cooldowns, pending RPCs crosses zone boundary | All buffs (remaining_ticks preserved), cooldowns, pending RPCs intact on destination | R-8.7.4 |

### TC-8.7.4.I2 Migration Under 100ms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure wall time from freeze_player to client receiving snapshot on destination | < 100 ms | R-8.7.4 |

### TC-8.7.5.I1 DB 10K Concurrent Saves

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 character save operations submitted concurrently | All complete successfully, simulation tick not blocked | R-8.7.5 |

### TC-8.7.5.I2 DB Trade Atomicity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Begin trade transaction (debit + credit + transfer), simulate crash after debit | All operations rolled back, original balances restored | R-8.7.5 |

### TC-8.7.5.I3 DB Sustained 10K TPS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sustained write load for 60 s | Throughput >= 10,000 TPS | R-8.7.5 |

### TC-8.7.6.I1 Autoscale Surge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate 5,000 player surge pushing CPU above scale_up_cpu threshold | New servers provisioned within 30 s | R-8.7.6 |

### TC-8.7.6.I2 Autoscale Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate population decline below scale_down_cpu for cooldown_seconds | Server set to draining, players migrated, then server terminated | R-8.7.6 |

### TC-8.7.7.I1 Cross-Shard Auction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | List item on shard A, buyout from player on shard B | Item delivered to buyer on shard B, gold transferred to seller on shard A | R-8.7.7 |

### TC-8.7.7.I2 Cross-Shard Mail

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send mail with item attachment from shard A player to shard B player | Mail appears in recipient inbox, attachment claimable | R-8.7.7 |

### TC-8.7.7.I3 Cross-Shard Concurrent Auction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Concurrent bid from shard A and buyout from shard B on same listing | Deterministic resolution: exactly one wins | R-8.7.7 |

## Benchmarks

### TC-8.7.2.B1 Zone Transition Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Player crosses zone boundary (perceived delay) | Perceived latency | 0 ms (seamless) | R-8.7.2 |

### TC-8.7.4.B1 Migration Handoff Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full migration: freeze, serialize, transfer, deserialize, redirect | Wall time | < 100 ms | R-8.7.4 |

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
