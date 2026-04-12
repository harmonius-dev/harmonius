# Network Infrastructure — Test Cases

Companion to [network-infrastructure.md](network-infrastructure.md).

Test case IDs use `TC-8.7.Y.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                  | Req       |
|---------------|---------------------------------------|-----------|
| TC-8.7.1.1    | `test_shard_assign_player`            | R-8.7.1   |
| TC-8.7.1.2    | `test_shard_population_split`         | R-8.7.1   |
| TC-8.7.1.3    | `test_instance_difficulty_param`      | R-8.7.1   |
| TC-8.7.2.1    | `test_zone_handoff_state`             | R-8.7.2   |
| TC-8.7.2.2    | `test_zone_handoff_pending_rpcs`      | R-8.7.2   |
| TC-8.7.3.1    | `test_mesh_split_overload`            | R-8.7.3   |
| TC-8.7.3.2    | `test_mesh_merge_underutilized`       | R-8.7.3   |
| TC-8.7.4.1    | `test_player_migration_state`         | R-8.7.4   |
| TC-8.7.4.2    | `test_player_migration_buffs`         | R-8.7.4   |
| TC-8.7.5.1    | `test_dal_async_no_block`             | R-8.7.5   |
| TC-8.7.5.2    | `test_transaction_atomic_commit`      | R-8.7.5   |
| TC-8.7.5.3    | `test_transaction_atomic_rollback`    | R-8.7.5   |
| TC-8.7.6.1    | `test_autoscale_provision`            | R-8.7.6   |
| TC-8.7.6.2    | `test_autoscale_drain_before_term`    | R-8.7.6   |
| TC-8.7.7.1    | `test_auction_cross_shard_visible`    | R-8.7.7   |
| TC-8.7.7.2    | `test_concurrent_bid_buyout`          | R-8.7.7   |
| TC-8.7.8.1    | `test_bus_pubsub_fanout`              | R-8.7.8   |
| TC-8.7.8.2    | `test_bus_point_to_point`             | R-8.7.8   |
| TC-8.7.8.3    | `test_bus_exactly_once_dedupe`        | R-8.7.8   |
| TC-8.7.13.1   | `test_tikv_multi_key_txn`             | R-8.7.13  |
| TC-8.7.13.2   | `test_tikv_ttl_expiration`            | R-8.7.13  |
| TC-8.7.17.1   | `test_overlap_co_simulation`          | R-8.7.17  |
| TC-8.7.18.1   | `test_instance_lockout_timer`         | R-8.7.18  |
| TC-8.7.19.1   | `test_canary_rollback_on_breach`      | R-8.7.19  |
| TC-8.8.1.1    | `test_server_side_movement_bound`     | R-8.8.1   |
| TC-8.8.2.1    | `test_client_integrity_hash`          | R-8.8.2   |
| TC-8.8.3.1    | `test_behavior_zscore_anomaly`        | R-8.8.3   |
| TC-8.8.4.1    | `test_economy_double_spend_blocked`   | R-8.8.4   |
| TC-8.8.5.1    | `test_rpc_rate_limit_budget`          | R-8.8.5   |

1. **TC-8.7.1.1** `test_shard_assign_player` — Create a `ShardManager` with 3 shards and assign a
   character. Assert the assignment is recorded against exactly one shard and lookup returns the
   same `ShardId`.
   - Input: `ShardManager::with_shards([ShardId(1), ShardId(2), ShardId(3)])`,
     `assign_character(CharacterId(42))`
   - Expected: `manager.shard_of(CharacterId(42)) == Some(ShardId(_))`,
     `manager.population_of(shard) == 1`

2. **TC-8.7.1.2** `test_shard_population_split` — Mark `ShardId(1)` over the configured population
   threshold. Assert `should_split()` returns true and `split()` produces a new shard ID.
   - Input: `Shard { id: ShardId(1), population: 6000, threshold: 5000 }`
   - Expected: `should_split() == true`, `split()` returns `Ok(ShardId(2))` and migrates 50% of
     characters off the source shard

3. **TC-8.7.1.3** `test_instance_difficulty_param` — Create dungeon instance with `Heroic`
   difficulty. Assert difficulty parameters propagate to spawned encounters.
   - Input: `InstanceManager::create(InstanceTemplate::Dungeon, Difficulty::Heroic, group_id)`
   - Expected: `instance.difficulty() == Difficulty::Heroic`,
     `instance.encounters().all(|e| e.scaling == Scaling::Heroic)`

4. **TC-8.7.2.1** `test_zone_handoff_state` — Build a `PlayerHandoff` with replicated component set,
   hand off to destination zone, assert all components present on the destination.
   - Input: entity with `Transform`, `Health(85)`, `Inventory([Item(1), Item(2)])`,
     `handoff(ZoneId(7))`
   - Expected: destination zone contains entity with identical `Transform`, `Health(85)`, identical
     inventory; source zone no longer owns the entity

5. **TC-8.7.2.2** `test_zone_handoff_pending_rpcs` — Pending RPCs queued at handoff time are
   transferred and replayed on the destination.
   - Input: entity has `pending_rpcs = [Rpc::CastAbility { id: 12 }, Rpc::Loot { item: 3 }]`,
     `handoff(ZoneId(2))`
   - Expected: destination receives the same pending RPCs in the same order; source pending queue
     drained

6. **TC-8.7.3.1** `test_mesh_split_overload` — A spatial cell exceeds the entity-density threshold;
   `MeshController::tick()` issues a split.
   - Input: cell with 1500 entities, density threshold 1000
   - Expected: `MeshSplit { parent: cell_id, children: [_, _] }` event emitted; child cells each own
     roughly half the entities

7. **TC-8.7.3.2** `test_mesh_merge_underutilized` — Two adjacent cells fall below the merge
   threshold for the configured cooldown duration; controller merges them.
   - Input: two adjacent cells with 50 entities each, merge threshold 100, cooldown elapsed
   - Expected: `MeshMerge { result: new_cell_id, sources: [a, b] }` event emitted; new cell owns all
     100 entities

8. **TC-8.7.4.1** `test_player_migration_state` — `MigrationService::migrate(player, dst)` packs
   entity state, transfers, and unpacks. Assert resulting entity equals source.
   - Input: player with `Transform { x: 100.0, y: 5.0, z: 200.0 }`, `Velocity { x: 1.0, .. }`
   - Expected: destination entity has identical `Transform` and `Velocity`; source entity removed;
     migration `Result::Ok` returned

9. **TC-8.7.4.2** `test_player_migration_buffs` — Active buffs with remaining durations migrate
   without resetting timers.
   - Input: player with `[Buff { id: 1, remaining: 5.5 }, Buff { id: 2, remaining: 12.0 }]`
   - Expected: destination buffs equal source buffs (same IDs and remaining durations within ±50 ms
     of measured handoff time)

10. **TC-8.7.5.1** `test_dal_async_no_block` — `DatabaseAccessLayer::write_async` returns a future
    that the simulation tick polls without blocking.
    - Input: `dal.write_async(KvWrite { key, value })` invoked from a 16 ms tick budget
    - Expected: tick completes in < 16 ms; future resolves on a worker; no `block_on` in the tick
      thread (asserted via thread-id check)

11. **TC-8.7.5.2** `test_transaction_atomic_commit` — Transaction with three writes commits all keys
    atomically.
    - Input: `Transaction { writes: [(k1, v1), (k2, v2), (k3, v3)] }`, `commit()`
    - Expected: all three keys present after commit; commit returns `Ok(CommitTs(_))`

12. **TC-8.7.5.3** `test_transaction_atomic_rollback` — Transaction encounters a key conflict and
    rolls back.
    - Input: `Transaction { writes: [(k1, v1)] }` then concurrent writer commits a conflicting
      version; first transaction calls `commit()`
    - Expected: `Err(TxnError::Conflict)`; key `k1` retains the conflicting writer's value

13. **TC-8.7.6.1** `test_autoscale_provision` — `AutoScaler` observes CPU > 80% on all current pods;
    issues a provision request.
    - Input: 4 pods all reporting `CpuPercent(85)`
    - Expected: `ProvisionRequest { count: 1 }` emitted to the K8s operator; new replica count is 5

14. **TC-8.7.6.2** `test_autoscale_drain_before_term` — Scale-down request triggers drain first.
    - Input: 6 pods, target replicas 4, pod selected for termination has 3 active players
    - Expected: `DrainRequest { pod, deadline }` emitted before any `Terminate`; pod transitions
      `Active -> Draining -> Terminating`

15. **TC-8.7.7.1** `test_auction_cross_shard_visible` — Listing posted on shard A is queryable from
    shard B via the `AuctionService`.
    - Input: `auction_service.list_item(ShardId(A), Listing { item: 7, price: 100 })`
    - Expected: `auction_service.search(ShardId(B), Filter::ItemId(7))` returns the listing within
      the same call

16. **TC-8.7.7.2** `test_concurrent_bid_buyout` — Two clients submit a bid and a buyout for the same
    listing simultaneously; deterministic resolution wins.
    - Input: `Bid { amount: 110 }` from client A and `Buyout { amount: 200 }` from client B at the
      same logical timestamp
    - Expected: buyout wins (higher priority); bid refunded; listing closes; the loser receives
      `AuctionResult::Outbid`

17. **TC-8.7.8.1** `test_bus_pubsub_fanout` — Three subscribers on channel `world.events`. A publish
    reaches all three.
    - Input: `bus.subscribe("world.events")` x3, `bus.publish("world.events", msg)`
    - Expected: each subscriber receives exactly one copy of `msg`; receive order matches publish
      order

18. **TC-8.7.8.2** `test_bus_point_to_point` — Direct message routes to the named target only.
    - Input: `bus.send(NodeId("zone-7"), Message::Migrate(player_id))`
    - Expected: `zone-7` receives the message; no other node receives it; delivery acked within the
      configured timeout

19. **TC-8.7.8.3** `test_bus_exactly_once_dedupe` — Sender retransmits the same message after a
    timeout; receiver dedupes via message ID.
    - Input: `bus.send_exactly_once(msg_id: 42, payload)` retried 3 times due to lost acks
    - Expected: receiver delivers the payload to the application exactly once; subsequent retries
      observe a cached ack and discard the duplicate

20. **TC-8.7.13.1** `test_tikv_multi_key_txn` — Multi-key TiKV transaction commits across player,
    leaderboard, and session keys.
    - Input: `txn.put("player/1", v1).put("lb/global", v2).put("session/abc", v3).commit()`
    - Expected: all three keys present after commit; partial-failure injection causes `commit()` to
      return `Err(TxnError::_)` with no key written

21. **TC-8.7.13.2** `test_tikv_ttl_expiration` — TTL key expires within the configured deadline.
    - Input: `tikv.put_with_ttl("session/xyz", value, ttl: Duration::from_secs(2))`
    - Expected: `tikv.get("session/xyz")` returns `Some(_)` at t=1.0 s, returns `None` at t=3.0 s
      (within ±1 s of expiry deadline)

22. **TC-8.7.17.1** `test_overlap_co_simulation` — Entity inside the boundary overlap region is
    simulated on both adjacent zones; the ghost mirrors the authoritative state.
    - Input: entity at world position inside overlap region between `ZoneId(1)` and `ZoneId(2)`,
      authority on `ZoneId(1)`
    - Expected: `ZoneId(2)` has a ghost entity with the same `Transform`; modifying state on
      `ZoneId(1)` updates the ghost within one sync interval (default 100 ms)

23. **TC-8.7.18.1** `test_instance_lockout_timer` — Group completes a heroic dungeon; lockout timer
    prevents re-entry until reset.
    - Input: `instance_manager.complete(group_id, Difficulty::Heroic)`
    - Expected: `instance_manager.can_enter(group_id, Difficulty::Heroic) == false` until the
      lockout window elapses (default weekly reset)

24. **TC-8.7.19.1** `test_canary_rollback_on_breach` — Canary deployment receives 10% traffic; error
    rate exceeds threshold; operator triggers rollback.
    - Input: canary at `traffic: 10%`, `error_rate: 5%` (threshold 1%)
    - Expected: `RolloutAction::Rollback { revision: prev }` emitted; canary pods drained and
      replaced with the previous revision

25. **TC-8.8.1.1** `test_server_side_movement_bound` — Client reports a position delta exceeding the
    maximum physically possible given the server-simulated velocity. Assert the server rejects the
    update and logs a `CheatDetected { kind: Movement }` event.
    - Input: server velocity cap 10 m/s, client reports 50 m jump in 1 s
    - Expected: position snap-back to server state, violation score incremented, event emitted

26. **TC-8.8.2.1** `test_client_integrity_hash` — Client reports a memory-region hash that does not
    match the expected signed baseline. Assert the server flags the session with
    `CheatDetected { kind: Integrity }` and escalates to the configured action.
    - Input: expected hash `H_ref`, client report `H_tampered`
    - Expected: mismatch detected, action `Kick` (or configured tier) emitted, event logged

27. **TC-8.8.3.1** `test_behavior_zscore_anomaly` — Player's per-minute headshot rate exceeds
    population Z-score threshold. Assert the anomaly detector emits a `BehaviorAnomaly` event with
    the computed Z-score.
    - Input: population mean 5/min, stddev 1/min, player 12/min over 5 minute window
    - Expected: `BehaviorAnomaly { metric: "headshots_per_min", z: _ > 3.0 }` emitted, player
      flagged for review

28. **TC-8.8.4.1** `test_economy_double_spend_blocked` — Player submits two trade RPCs referencing
    the same inventory item ID within one tick. Assert the second rejects with
    `EconomyError::DoubleSpend` and no item duplication occurs.
    - Input: item id `42`, trade 1 to player B, trade 2 to player C same tick
    - Expected: trade 1 commits, trade 2 returns `Err(EconomyError::DoubleSpend)`, inventory
      snapshot shows 1 copy of item 42

29. **TC-8.8.5.1** `test_rpc_rate_limit_budget` — Player invokes a rate-limited RPC at 2× the
    configured budget. Assert excess calls are rejected with `RpcError::RateLimit` and the budget
    refills on the configured cadence.
    - Input: `CastAbility` budget 10/s, 20 invocations in 1 s
    - Expected: 10 executed, 10 rejected with `RateLimit`; budget resets after 1 s window

## Integration Tests

| ID            | Name                                | Req        |
|---------------|-------------------------------------|------------|
| TC-8.7.I.1    | `test_seamless_zone_transition`     | R-8.7.2    |
| TC-8.7.I.2    | `test_mesh_500_player_convergence`  | R-8.7.10   |
| TC-8.7.I.3    | `test_player_migration_combat`      | R-8.7.4    |
| TC-8.7.I.4    | `test_persistence_10k_concurrent`   | R-8.7.11   |
| TC-8.7.I.5    | `test_cross_shard_mail_delivery`    | R-8.7.7    |
| TC-8.7.I.6    | `test_helm_deploy_oss_stack`        | R-8.7.14   |
| TC-8.7.I.7    | `test_pingora_cdn_cache`            | R-8.7.15   |
| TC-8.7.I.8    | `test_prometheus_alert_tick_time`   | R-8.7.16   |

1. **TC-8.7.I.1** `test_seamless_zone_transition` — Move a player across a zone boundary on a real
   two-zone cluster. Assert no loading screen, no disconnect, no state discontinuity.
   - Input: player walks from `ZoneId(1)` to `ZoneId(2)` while replicating to a connected client
   - Expected: client connection rebinds to `ZoneId(2)` within 500 ms; no `Disconnect` event;
     `Transform` interpolation continuous on the client; entities in the overlap region visible from
     both zones

2. **TC-8.7.I.2** `test_mesh_500_player_convergence` — Spawn 500 simulated players converging on one
   cell. Assert mesh splits within 10 s.
   - Input: 500 simulated player entities moving to a 100 m radius around `world(0, 0)`
   - Expected: `MeshSplit` event observed within 10 s; the original cell's entity count drops below
     the split threshold; no player disconnects during the split

3. **TC-8.7.I.3** `test_player_migration_combat` — Migrate a player mid-combat with 5 active buffs.
   Assert all state present on destination, handoff under 100 ms.
   - Input: player in active combat, `[Buff(1..=5)]`, `pending_rpcs = [CastAbility, Loot]`,
     migration triggered by mesh rebalance
   - Expected: destination has identical buff list (same ids/remaining), pending RPCs replayed,
     `migrate()` wall time < 100 ms; client renders continuously with no visible teleport

4. **TC-8.7.I.4** `test_persistence_10k_concurrent` — Submit 10,000 character saves concurrently
   over 1 second. Assert sustained TPS and no tick blocking.
   - Input: 10,000 `dal.write_async(character_save)` calls submitted across 1 s
   - Expected: all writes succeed; tick rate stays at the configured 30 Hz across the test duration;
     p99 commit latency < 10 ms

5. **TC-8.7.I.5** `test_cross_shard_mail_delivery` — Send mail with attachment from shard A to shard
   B; verify receipt and attachment integrity.
   - Input: `mail_service.send(from: A/player1, to: B/player2, body, attachment: Item(7))`
   - Expected: `B/player2` mailbox contains the message and attachment; attachment hash matches
     source; cross-shard route logged on inter-server bus

6. **TC-8.7.I.6** `test_helm_deploy_oss_stack` — Apply Helm chart on a vanilla K8s cluster; assert
   all services reach Ready.
   - Input: `helm install harmonius ./charts/harmonius` against `kind` cluster
   - Expected: TiKV, Garage, Pingora, Vector, Prometheus, Grafana, Loki pods all in `Ready` state
     within 5 minutes; smoke test successfully writes a key to TiKV

7. **TC-8.7.I.7** `test_pingora_cdn_cache` — Request an asset twice through a Pingora edge pod;
   assert second request is a cache hit.
   - Input: `GET /assets/foo.ktx2` x2 with HTTP/3
   - Expected: first response includes `x-cache: MISS`; second includes `x-cache: HIT`; both
     payloads bit-identical; HTTP/3 negotiation observed via ALPN

8. **TC-8.7.I.8** `test_prometheus_alert_tick_time` — Configure alert on tick time > 50 ms; inject
   slow tick; assert alert fires.
   - Input: alert rule `tick_ms > 50 for 30s`, server with injected sleep that pushes tick to 60 ms
   - Expected: Prometheus emits the alert state `firing` within 60 s; Grafana dashboard reflects the
     breach; underlying log entries reach Loki via Vector

## Benchmarks

| ID           | Benchmark                                | Target     | Req        |
|--------------|------------------------------------------|------------|------------|
| TC-8.7.B.1   | Player migration handoff                 | < 100 ms   | R-8.7.4    |
| TC-8.7.B.2   | Persistence sustained writes             | > 10k TPS  | R-8.7.11   |
| TC-8.7.B.3   | Mesh split (500-player convergence)      | < 10 s     | R-8.7.10   |
| TC-8.7.B.4   | Cross-shard auction query (1M listings)  | < 50 ms    | R-8.7.7    |
| TC-8.7.B.5   | Inter-server bus throughput              | > 100k m/s | R-8.7.8    |

1. **TC-8.7.B.1** — Wall time of `MigrationService::migrate(player, dst)` from invocation to the
   destination acknowledging full state. Player carries 5 buffs, 3 cooldowns, 64 inventory items,
   and 4 KiB of prediction history. Measured with `criterion`.

2. **TC-8.7.B.2** — Sustained write throughput of `DatabaseAccessLayer::write_async` over a 60-s
   window with a synthetic 200-byte character save payload. Measured as committed transactions per
   second; p99 commit latency tracked separately.

3. **TC-8.7.B.3** — Wall time from convergence start (500 simulated players entering one cell) to
   `MeshSplit` event emission. Includes density evaluation, child cell creation, entity rebalance.

4. **TC-8.7.B.4** — Cross-shard auction search wall time over a 1,000,000-listing TiKV-backed index.
   Query: `Filter::ItemId(_) AND PriceRange(min, max)`. Measured end-to-end including network
   round-trip from shard B to the auction service.

5. **TC-8.7.B.5** — Messages per second sustained on the inter-server bus across 8 sender/8 receiver
   nodes. Payload: 256-byte serialized event. Measured as total fan-out throughput; reconnection
   events excluded.
