# 8.7 -- MMO Infrastructure

## World Topology

| ID      | Feature                       |
|---------|-------------------------------|
| F-8.7.1 | World Sharding and Instancing |
| F-8.7.2 | Seamless Zone Transitions     |

1. **F-8.7.1** — Partition the persistent game world into shards (full world copies for population
   management) and instances (isolated copies of specific zones for dungeons, raids, battlegrounds).
   The sharding layer manages character-to-shard assignment, cross-shard social features (friends,
   guilds, mail, auction house), and shard merging/splitting based on population. Instancing
   supports parameterized difficulty, lockout timers, and group-scoped state.
   - **Deps:** F-8.5.4
   - **Platform:** Server-side infrastructure; same topology serves all client platforms. Mobile
     clients connect to the same shards as desktop/console.
2. **F-8.7.2** — Transfer players between zone server processes without loading screens or
   disconnects. The source server hands off the player's replicated state, pending RPCs, and input
   buffer to the destination server while the client transparently rebinds its connection. Entities
   near zone boundaries are co-simulated by adjacent servers using a boundary overlap region to
   prevent pop-in and state discontinuities.
   - **Deps:** F-8.7.1, F-8.2.1, F-8.1.2
   - **Platform:** Mobile may show a brief loading indicator during zone handoff due to higher
     reconnection latency on cellular. Overlap region size is server-configured.

## Server Mesh

| ID      | Feature                          |
|---------|----------------------------------|
| F-8.7.3 | Dynamic Server Mesh              |
| F-8.7.4 | Player Migration Between Servers |

1. **F-8.7.3** — Distribute the continuous game world across a mesh of server processes where each
   process owns a spatial region, and regions dynamically resize based on entity density and CPU
   load. When a region becomes overloaded (e.g., a guild siege concentrates hundreds of players),
   the mesh controller splits it across additional server processes. When load subsides, regions
   merge to conserve resources. Purpose-built for MMO gameplay patterns with dynamic spatial
   partitioning.
   - **Deps:** F-8.7.2, F-8.5.4
   - **Platform:** Server-side infrastructure; transparent to all client platforms. Server mesh
     decisions may factor in mobile client ratio to adjust entity budgets.
2. **F-8.7.4** — Migrate a player's authoritative simulation state from one server process to
   another -- during zone transitions, load balancing, or server mesh rebalancing -- with zero
   downtime and no perceptible interruption. Migration transfers entity state, active buffs/debuffs,
   cooldown timers, pending RPCs, and the client's prediction history. The client continues
   rendering using extrapolation during the brief handoff window (target < 100 ms).
   - **Deps:** F-8.7.3, F-8.4.4, F-8.1.2
   - **Platform:** Mobile handoff may exceed 100 ms target on cellular due to connection
     re-establishment latency; extrapolation window is extended accordingly.

## Persistence

| ID      | Feature                                         |
|---------|-------------------------------------------------|
| F-8.7.5 | Persistent World State and Database Integration |
| F-8.7.6 | Load Balancing and Auto-Scaling                 |
| F-8.7.7 | Cross-Shard Services                            |

1. **F-8.7.5** — Store and retrieve persistent world state -- player characters, inventories, guild
   rosters, auction house listings, housing, quest progress -- through an async database access
   layer that never blocks the game simulation tick. Support transactional writes for operations
   requiring atomicity (trades, mail with attachments, auction settlements). The persistence layer
   must handle sustained write throughput of tens of thousands of transactions per second across the
   server fleet.
   - **Deps:** F-8.7.1
   - **Platform:** Database access uses Tokio async I/O per project guidelines.
2. **F-8.7.6** — Monitor server process CPU, memory, network utilization, and player count in real
   time, and automatically provision or deprovision server processes to match demand. Load balancing
   distributes new player connections and zone assignments to the least-loaded eligible servers.
   Auto-scaling must react within seconds to population surges (world boss spawns, siege starts,
   login queues after maintenance) and scale down gracefully by draining players from underutilized
   servers before terminating them.
   - **Deps:** F-8.5.4, F-8.7.3
   - **Platform:** Designed for container orchestration (Kubernetes) with cloud provider
     auto-scaling APIs. Bare-metal deployments use a custom scaler.
3. **F-8.7.7** — Provide shared services that operate across all shards: global auction house,
   cross-shard mail, cross-shard group finder, global chat channels, friends list, and guild
   management. These services run as independent microservices with their own persistence,
   communicating with zone servers via internal RPC. Must maintain consistency under high
   concurrency -- an auction bid and a buyout arriving simultaneously must resolve
   deterministically.
   - **Deps:** F-8.7.1, F-8.7.5
   - **Platform:** Server-side microservices; platform-agnostic. Mobile clients access the same
     cross-shard APIs through the game client's networking layer.

## Inter-Server Communication

| ID      | Feature                        |
|---------|--------------------------------|
| F-8.7.8 | Inter-Server Communication Bus |
| F-8.7.9 | Kubernetes-Native Game Server Operator |

1. **F-8.7.8** — A dedicated message bus for server-to-server communication within the server mesh.
   Servers exchange player migration requests, cross-shard events (world bosses, faction wars),
   shared economy state, and global chat messages through typed, serialized messages over persistent
   TCP connections with automatic reconnection. The bus supports publish-subscribe channels (all
   servers receive global events) and point-to-point routing (direct message to the server owning a
   specific shard). Message delivery guarantees are configurable per channel (at-most-once for
   telemetry, at-least-once for player migration, exactly-once for economy transactions).
   - **Deps:** F-8.7.1 (World Sharding), F-8.1.5 (Encryption)
   - **Platform:** Server-to-server only; no client platform dependency. Runs on Linux datacenter
     infrastructure.
2. **F-8.7.9** — A Kubernetes-native operator written in Rust (kube-rs) that manages game server
   fleet lifecycle through custom resources (GameServer CRD, GameDeployment CRD). The operator
   handles pod provisioning, auto-scaling based on player population and CPU load, rolling updates
   with player migration, and graceful drain on scale-down. Game-aware reconciliation loops monitor
   tick time, player count, and memory budgets, triggering scale events or rollbacks based on
   game-specific health metrics rather than generic HTTP liveness probes. Runs as a cluster-scoped
   controller on any Kubernetes distribution (EKS, GKE, AKS, k3s, bare-metal kubeadm).
   - **Deps:** F-8.7.6, F-8.7.4
   - **Platform:** Server-side only. Requires Kubernetes 1.27+. Distributed as a Helm chart with the
     operator container image and CRD manifests.

## Persistence Backend

| ID       | Feature                         |
|----------|---------------------------------|
| F-8.7.10 | TiKV Unified Persistence Layer  |

1. **F-8.7.10** — TiKV serves as the single persistence backend for all server-side storage — player
   characters, inventories, skill ratings, leaderboards, achievements, sessions, rate limit
   counters, trades, cloud saves, and telemetry aggregates. The engine exposes a typed storage API
   supporting key-value operations, ordered range scans (leaderboards), multi-key transactions
   (trades, auction settlements), TTL-expiring keys (sessions, rate limits), and atomic counter
   increments. TiKV replaces PostgreSQL, DynamoDB, Redis, and Valkey with a single horizontally
   scalable store, eliminating multi-database operational overhead.
   - **Deps:** F-8.7.5
   - **Platform:** Server-side only. Deployed via the TiKV Helm chart alongside the game servers.
     Client platforms are unaffected.

## Edge and CDN

| ID       | Feature                         |
|----------|---------------------------------|
| F-8.7.11 | Self-Hosted Edge Cache and CDN  |

1. **F-8.7.11** — A self-hosted CDN layer built on Pingora edge pods provides HTTP/3+QUIC asset
   delivery, disk-backed caching, and per-asset-type TTL configuration (long TTL for immutable
   content-addressed assets, short TTL for leaderboard snapshots). A cache invalidation API allows
   build pipelines and live-ops tools to purge specific paths or tags after content updates. Edge
   pods deploy close to players via Kubernetes node pools in multiple regions, eliminating
   dependence on commercial CDNs (CloudFront, Fastly) for asset delivery.
   - **Deps:** F-8.7.9, F-8.7.10
   - **Platform:** Server-side infrastructure. Clients on all platforms receive asset downloads over
     HTTP/3+QUIC from the nearest edge pod.

## Observability

| ID       | Feature                                        |
|----------|------------------------------------------------|
| F-8.7.12 | Server Monitoring and Observability Stack     |

1. **F-8.7.12** — Integrated monitoring and observability using Prometheus for metrics scraping
   (tick time, player count, CPU, memory, replication bandwidth, entity count), Vector for
   structured log collection, Loki for log aggregation, Grafana for dashboards and alerting, and
   configurable alert thresholds for game-specific signals (tick time over budget, population drop,
   migration failure rate). Dashboards ship preconfigured for the standard server fleet layout and
   can be customized per game. Alert integration supports PagerDuty, OpsGenie, Slack, and email.
   - **Deps:** F-8.7.9
   - **Platform:** Server-side. Deployed as part of the Helm chart bundle. No client impact.

## Deployment Strategy

| ID       | Feature                                        |
|----------|------------------------------------------------|
| F-8.7.13 | Game-Aware GitOps Deployment Operator         |

1. **F-8.7.13** — The GameDeployment CRD supports rolling update, canary, and blue-green strategies
   with configurable drain policies (Immediate, GracefulMigrate, WaitForEmpty). Canary deployments
   validate game-specific metrics (tick time, crash rate, player session length) before promoting to
   full rollout. Failed validation triggers automatic rollback to the previous stable version with
   player migration back to the old pods. GitOps integration (Flux or Argo CD) watches a repository
   of GameDeployment manifests and reconciles cluster state against declared desired state, enabling
   version-controlled deployments with full audit history.
   - **Deps:** F-8.7.9, F-8.7.12
   - **Platform:** Server-side. GitOps controllers run in-cluster alongside the game operator.

## Instance Management

| ID       | Feature                                        |
|----------|------------------------------------------------|
| F-8.7.14 | Instance Manager with Difficulty and Lockout  |

1. **F-8.7.14** — The instance manager creates, tracks, and destroys instanced zone copies
   (dungeons, raids, battlegrounds, scenarios) with configurable difficulty tiers (Normal, Heroic,
   Mythic, Mythic+ with key level), lockout timers (daily, weekly, rolling), group-scoped state
   isolation, and on-demand lifecycle (spawn instance when a group enters, destroy when empty or
   expired). Instance difficulty parameters (enemy health, damage, loot tables, affixes) are
   data-driven and can be tuned live without a server restart.
   - **Deps:** F-8.7.1, F-8.5.3
   - **Platform:** Server-side. Transparent to all client platforms.
