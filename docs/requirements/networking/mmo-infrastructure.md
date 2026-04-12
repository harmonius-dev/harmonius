# R-8.7 -- MMO Infrastructure Requirements

## World Topology

1. **R-8.7.1** — The engine **SHALL** partition the persistent game world into shards (full world
   copies for population management) and instances (isolated zone copies for dungeons, raids,
   battlegrounds), managing character-to-shard assignment, cross-shard social features, shard
   merging/splitting based on population, and parameterized instance difficulty and lockout timers.
   - **Rationale:** No single server can host an entire MMO population; sharding and instancing are
     the fundamental scaling mechanisms for persistent worlds.
   - **Verification:** Integration test: create 3 shards and verify player assignment. Verify
     cross-shard friends, guild roster, and mail. Spin up a dungeon instance with difficulty
     parameters and verify isolated state. Verify shard merge moves all state without data loss.
2. **R-8.7.2** — The engine **SHALL** transfer players between zone server processes without loading
   screens or disconnects, handing off replicated state, pending RPCs, and input buffer to the
   destination server while the client transparently rebinds its connection, with boundary overlap
   regions for entities near zone edges to prevent pop-in.
   - **Rationale:** Loading screens break immersion in open-world MMOs; seamless transitions
     maintain the illusion of a continuous world.
   - **Verification:** Integration test: move a player across a zone boundary. Verify no loading
     screen, no disconnect, no state discontinuity. Verify entities in the overlap region visible
     from both zones. Verify pending RPCs and input buffer transfer correctly.

## Server Mesh

1. **R-8.7.3** — The engine **SHALL** distribute the continuous game world across a mesh of server
   processes where each process owns a spatial region that dynamically resizes based on entity
   density and CPU load, splitting overloaded regions and merging underutilized regions.
   - **Rationale:** Fixed zone boundaries cannot handle dynamic player concentrations; dynamic
     spatial partitioning adapts to real-time load.
   - **Verification:** Integration test: simulate 500 players converging. Verify mesh split within
     10 seconds. Disperse players and verify merge within 30 seconds. Verify no player disconnect or
     glitch during operations.
2. **R-8.7.4** — The engine **SHALL** migrate a player's authoritative simulation state (entity
   state, active buffs, cooldown timers, pending RPCs, prediction history) from one server process
   to another with zero downtime and no perceptible interruption, completing in under 100 ms.
   - **Rationale:** Zone transitions, load balancing, and mesh rebalancing all require moving
     players between servers without disrupting gameplay.
   - **Verification:** Integration test: migrate a player mid-combat with active buffs. Verify all
     state present on the destination. Measure handoff duration and verify under 100 ms. Verify the
     client renders continuously using extrapolation.

## Persistence

1. **R-8.7.5** — The engine **SHALL** store and retrieve persistent world state (characters,
   inventories, guilds, auction listings, housing, quest progress) through an async database access
   layer that never blocks the game simulation tick, supporting transactional writes for atomic
   operations and sustained throughput of tens of thousands of transactions per second.
   - **Rationale:** Persistent state is the foundation of an MMO; database access must be
     non-blocking and transactional to prevent data corruption.
   - **Verification:** Integration test: perform 10,000 concurrent saves without blocking the tick.
     Execute a trade and verify atomicity by simulating a crash mid-transaction. Benchmark sustained
     write throughput above 10,000 TPS. Verify Tokio async I/O.
2. **R-8.7.6** — The engine **SHALL** monitor server process CPU, memory, network utilization, and
   player count in real time, automatically provisioning or deprovisioning server processes to match
   demand, reacting to surges within seconds and draining before termination.
   - **Rationale:** MMO populations fluctuate dramatically; auto-scaling prevents both
     overprovisioning costs and underprovisioning outages.
   - **Verification:** Load test: simulate 5,000 players over 60 seconds. Verify new processes
     provisioned within 30 seconds. Simulate decline and verify draining before termination. Verify
     load balancing distributes to least-loaded servers.
3. **R-8.7.7** — The engine **SHALL** provide shared services operating across all shards (global
   auction house, cross-shard mail, group finder, global chat, friends list, guild management) as
   independent microservices with their own persistence, resolving concurrent operations
   deterministically.
   - **Rationale:** Players expect social and economic features to work across shard boundaries;
     independent microservices isolate failure domains.
   - **Verification:** Integration test: list an auction on shard A and verify visibility from shard
     B. Send cross-shard mail with attachment and verify delivery. Simulate concurrent bid and
     buyout and verify deterministic resolution.

## Inter-Server Communication

1. **R-8.7.8** — The engine **SHALL** provide a typed, serialized message bus for server-to-server
   communication over persistent TCP connections with automatic reconnection, supporting
   publish-subscribe channels and point-to-point routing, with configurable per-channel delivery
   guarantees (at-most-once, at-least-once, exactly-once).
   - **Rationale:** Server mesh coordination, cross-shard events, and global chat require reliable
     inter-server messaging with appropriate delivery guarantees.
   - **Verification:** Integration test: publish a message and verify all subscribers receive it.
     Send point-to-point and verify only the target receives it. Simulate disconnect and verify
     reconnection within 5 seconds. Verify exactly-once deduplicates retransmissions.

## Non-Functional

1. **R-8.7.9** — Zone transitions **SHALL** complete the state handoff and connection rebind within
   500 ms, with no loading screen, no disconnect, and no visible state discontinuity. Player
   migration **SHALL** complete within 100 ms.
   - **Rationale:** Visible pauses or disconnects break open-world immersion and are unacceptable in
     a seamless MMO.
   - **Verification:** Integration test: move a player across a zone boundary. Measure total handoff
     and verify under 500 ms. Verify continuous rendering. Measure migration handoff and verify
     under 100 ms.
2. **R-8.7.10** — The dynamic server mesh **SHALL** support at least 100 server processes per world
   shard, with split operations completing within 10 seconds and merge operations within 30 seconds.
   - **Rationale:** Large-scale events concentrate players; the mesh must split fast enough to
     prevent overload.
   - **Verification:** Integration test: simulate 500-player convergence. Verify split within 10
     seconds. Disperse players and verify merge within 30 seconds.
3. **R-8.7.11** — The persistence layer **SHALL** sustain at least 10,000 transactional writes per
   second across the server fleet without blocking the simulation tick. Write latency **SHALL** be
   below 10 ms at the 99th percentile.
   - **Rationale:** Character saves, inventory updates, and economy transactions must complete at
     scale without impacting gameplay.
   - **Verification:** Benchmark: execute 10,000 concurrent character saves per second. Verify no
     tick blocked. Measure p99 write latency and verify below 10 ms.

## Kubernetes Operator

4. **R-8.7.12** — The engine **SHALL** provide a Kubernetes-native operator that manages game server
   pod lifecycle, auto-scaling, rolling updates with player migration, and automatic rollback based
   on game-specific health metrics.
   - **Rationale:** Game server fleet management requires game-aware orchestration beyond standard
     K8s deployments; a custom operator encodes fleet scaling and migration logic.
   - **Verification:** Integration test: deploy a GameServer CRD on a test cluster. Verify pods
     scale with player load. Trigger a health metric breach and verify automatic rollback within 60
     seconds.

## Unified Database

5. **R-8.7.13** — The engine **SHALL** use TiKV as the sole persistence backend for all server-side
   storage, supporting key-value, ordered range scans, multi-key transactions, TTL keys, and atomic
   increments.
   - **Rationale:** A single database backend eliminates operational complexity of running
     PostgreSQL, Redis, and DynamoDB; TiKV provides transactional KV with horizontal scaling.
   - **Verification:** Integration test: perform multi-key transactions across player data,
     leaderboards, and sessions. Verify range scans return ordered results. Verify TTL keys expire
     within 1 second of their deadline.
6. **R-8.7.14** — The engine **SHALL** ship Helm charts deploying a full OSS infrastructure stack
   (TiKV, Garage, Pingora, Vector, Prometheus, Grafana, Loki) on any Kubernetes cluster without
   vendor lock-in.
   - **Rationale:** Self-hosted OSS infrastructure gives operators full control and avoids cloud
     vendor lock-in for all storage, CDN, and observability needs.
   - **Verification:** Deploy Helm charts on a vanilla K8s cluster. Verify all services reach ready
     state. Run a smoke test writing data through each service.

## CDN and Observability

7. **R-8.7.15** — The engine **SHALL** provide a self-hosted CDN layer using Pingora edge pods with
   HTTP/3+QUIC asset delivery, disk-backed cache, and configurable per-asset-type TTL.
   - **Rationale:** Self-hosted CDN avoids per-GB egress fees and gives full control over cache
     invalidation and asset delivery performance.
   - **Verification:** Integration test: request an asset through the edge pod. Verify cache hit on
     second request. Verify HTTP/3 negotiation. Verify TTL expiration triggers re-fetch.
8. **R-8.7.16** — The engine **SHALL** expose server metrics (tick time, player count, CPU) via
   Prometheus, structured logs via Vector, and dashboards via Grafana, with configurable alerting on
   game-specific thresholds.
   - **Rationale:** Observability is essential for operating live-service game servers at scale;
     game-specific thresholds detect gameplay-impacting issues before players report them.
   - **Verification:** Integration test: run a game server and verify metrics appear in Prometheus.
     Verify logs reach Loki via Vector. Configure an alert on tick time exceeding 50 ms and verify
     it fires.

## Boundary Co-Simulation

9. **R-8.7.17** — The engine **SHALL** co-simulate entities within a configurable boundary overlap
   width on both adjacent zone servers, replicating overlap entities at a configurable sync interval
   and rendering them as ghost entities on the neighbor server.
   - **Rationale:** Co-simulation ensures entities near zone edges are visible and interactive from
     both sides, preventing pop-in and enabling combat across zone boundaries.
   - **Verification:** Integration test: place an entity in the overlap region. Verify both servers
     simulate it. Modify state on the authoritative server and verify the ghost updates within the
     sync interval.

## Instance Management

10. **R-8.7.18** — The engine **SHALL** manage instanced zone copies with configurable difficulty
    tiers, lockout timers, group-scoped state isolation, and automated lifecycle (create on demand,
    destroy when empty).
    - **Rationale:** Dungeons, raids, and battlegrounds require isolated instances with
      parameterized difficulty and time- limited access to prevent farming and ensure group-scoped
      progression.
    - **Verification:** Integration test: create a Heroic dungeon instance for a group. Verify
      isolated state. Verify lockout timer prevents re-entry. Verify instance destruction after the
      last player leaves.

## Deployment Strategies

11. **R-8.7.19** — The engine K8s operator **SHALL** support rolling update, canary, and blue-green
    deployment strategies with configurable drain policies, game-metric validation gates, and
    automatic rollback on threshold breach.
    - **Rationale:** Game server deployments must avoid disrupting active sessions; canary
      deployments validate game health before full rollout.
    - **Verification:** Integration test: deploy a canary with a 10% traffic split. Inject a bad
      metric and verify automatic rollback. Deploy a rolling update and verify zero player
      disconnections during the rollout.
