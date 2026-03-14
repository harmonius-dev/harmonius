# 8.7 — MMO Infrastructure

## World Topology

### F-8.7.1 World Sharding and Instancing

Partition the persistent game world into shards (full world copies for population management)
and instances (isolated copies of specific zones for dungeons, raids, battlegrounds). The
sharding layer manages character-to-shard assignment, cross-shard social features (friends,
guilds, mail, auction house), and shard merging/splitting based on population. Instancing
supports parameterized difficulty, lockout timers, and group-scoped state.

- **Requirements:** R-8.7.1
- **Dependencies:** F-8.5.4
- **Platform notes:** None

### F-8.7.2 Seamless Zone Transitions

Transfer players between zone server processes without loading screens or disconnects. The
source server hands off the player's replicated state, pending RPCs, and input buffer to the
destination server while the client transparently rebinds its connection. Entities near zone
boundaries are co-simulated by adjacent servers using a boundary overlap region to prevent
pop-in and state discontinuities.

- **Requirements:** R-8.7.2
- **Dependencies:** F-8.7.1, F-8.2.1, F-8.1.2
- **Platform notes:** None

## Server Mesh

### F-8.7.3 Dynamic Server Mesh

Distribute the continuous game world across a mesh of server processes where each process owns
a spatial region, and regions dynamically resize based on entity density and CPU load. When a
region becomes overloaded (e.g., a guild siege concentrates hundreds of players), the mesh
controller splits it across additional server processes. When load subsides, regions merge to
conserve resources. Inspired by SpatialOS-style spatial partitioning but purpose-built for
MMO gameplay patterns.

- **Requirements:** R-8.7.3
- **Dependencies:** F-8.7.2, F-8.5.4
- **Platform notes:** None

### F-8.7.4 Player Migration Between Servers

Migrate a player's authoritative simulation state from one server process to another — during
zone transitions, load balancing, or server mesh rebalancing — with zero downtime and no
perceptible interruption. Migration transfers entity state, active buffs/debuffs, cooldown
timers, pending RPCs, and the client's prediction history. The client continues rendering
using extrapolation during the brief handoff window (target < 100 ms).

- **Requirements:** R-8.7.4
- **Dependencies:** F-8.7.3, F-8.4.4, F-8.1.2
- **Platform notes:** None

## Persistence

### F-8.7.5 Persistent World State and Database Integration

Store and retrieve persistent world state — player characters, inventories, guild rosters,
auction house listings, housing, quest progress — through an async database access layer
that never blocks the game simulation tick. Support transactional writes for operations
requiring atomicity (trades, mail with attachments, auction settlements). The persistence
layer must handle sustained write throughput of tens of thousands of transactions per second
across the server fleet.

- **Requirements:** R-8.7.5
- **Dependencies:** F-8.7.1
- **Platform notes:** Database access uses platform-native async I/O (IOCP on Windows, GCD
  on macOS, io_uring on Linux) per project guidelines.

### F-8.7.6 Load Balancing and Auto-Scaling

Monitor server process CPU, memory, network utilization, and player count in real time, and
automatically provision or deprovision server processes to match demand. Load balancing
distributes new player connections and zone assignments to the least-loaded eligible servers.
Auto-scaling must react within seconds to population surges (world boss spawns, siege starts,
login queues after maintenance) and scale down gracefully by draining players from
underutilized servers before terminating them.

- **Requirements:** R-8.7.6
- **Dependencies:** F-8.5.4, F-8.7.3
- **Platform notes:** Designed for container orchestration (Kubernetes) with cloud provider
  auto-scaling APIs. Bare-metal deployments use a custom scaler.

### F-8.7.7 Cross-Shard Services

Provide shared services that operate across all shards: global auction house, cross-shard
mail, cross-shard group finder, global chat channels, friends list, and guild management.
These services run as independent microservices with their own persistence, communicating
with zone servers via internal RPC. Must maintain consistency under high concurrency — an
auction bid and a buyout arriving simultaneously must resolve deterministically.

- **Requirements:** R-8.7.7
- **Dependencies:** F-8.7.1, F-8.7.5
- **Platform notes:** None
