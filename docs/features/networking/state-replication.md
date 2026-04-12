# 8.2 -- State Replication

## Property Replication

| ID      | Feature                                      |
|---------|----------------------------------------------|
| F-8.2.1 | Delta-Compressed Property Replication        |
| F-8.2.2 | Component Replication with Schema Versioning |

1. **F-8.2.1** — Replicate entity component properties from server to clients by transmitting only
   the fields that changed since each client's last acknowledged state. Delta compression uses
   per-client baseline tracking and XOR-based or arithmetic differencing to minimize bandwidth.
   Essential for MMO-scale worlds where thousands of entities update simultaneously but each client
   only needs the minimal diff.
   - **Deps:** F-8.1.3, F-8.1.4
   - **Platform:** Mobile clients receive more aggressively delta-compressed updates with lower
     quantization precision to fit within ~500 Kbps bandwidth budgets.
2. **F-8.2.2** — Replicate entire ECS components as atomic units with schema versioning so that
   clients running slightly different build versions can still decode replicated data. Component
   schemas are registered at startup and negotiated during connection handshake, enabling rolling
   server updates without kicking all players -- critical for live-service MMO operations.
   - **Deps:** F-8.2.1, F-8.1.1
   - **Platform:** Mobile clients may run older app versions due to app store update delays; schema
     versioning is critical for mobile cross-version compatibility.

## Relevancy and Filtering

| ID      | Feature                            |
|---------|------------------------------------|
| F-8.2.3 | Area-of-Interest Filtering         |
| F-8.2.4 | Conditional and Tiered Replication |

1. **F-8.2.3** — Limit replication to entities within each client's area of interest (AOI), defined
   by spatial proximity, visibility, and game-specific rules (e.g., always replicate party members,
   guild members in the same zone, or raid boss regardless of distance). AOI queries use the shared
   BVH spatial index (F-1.9.8) for spatial relevancy filtering, enabling efficient evaluation for
   thousands of clients against hundreds of thousands of entities per zone.
   - **Deps:** F-8.2.1, F-1.9.8 (Network Relevancy Integration)
2. **F-8.2.4** — Support per-property replication conditions (owner-only, team-only, public) and
   distance-based detail tiers that reduce replication frequency and property count for distant
   entities. A player 400 meters away receives position updates at 5 Hz with no animation state; a
   nearby player receives full state at 30 Hz. This tiered approach is essential for supporting
   hundreds of players in guild siege or world boss encounters.
   - **Deps:** F-8.2.3
   - **Platform:** Mobile uses more aggressive distance tiers: near (full, 20 Hz), mid (reduced, 5
     Hz), far (position-only, 2 Hz) to stay within bandwidth budget.

## Bandwidth Management

| ID      | Feature                                     |
|---------|---------------------------------------------|
| F-8.2.5 | Priority Scheduling and Bandwidth Budgeting |
| F-8.2.6 | Entity Dormancy                             |

1. **F-8.2.5** — Assign replication priority to each entity based on relevancy score, entity type,
   and staleness, then allocate the per-connection bandwidth budget to maximize gameplay quality.
   High-priority entities (the player's target, nearby hostiles, spell effects) are replicated
   first; low-priority entities (distant NPCs, ambient creatures) fill remaining budget. The
   scheduler must respect the congestion controller's send rate from F-8.1.7.
   - **Deps:** F-8.2.3, F-8.1.7
   - **Platform:** Mobile bandwidth budget is typically 50-100 KB/s (vs. 500+ KB/s on desktop).
     Server tags mobile connections for tighter budget allocation.
2. **F-8.2.6** — Allow entities that have not changed for a configurable period to enter a dormant
   state where they consume zero replication bandwidth. Dormant entities are excluded from priority
   scheduling entirely and only wake when a property changes or an explicit wake event fires. In a
   typical MMO zone, 60-80% of entities are dormant at any time (idle NPCs, placed objects, inactive
   triggers), making this critical for server scalability.
   - **Deps:** F-8.2.5
   - **Platform:** Server-side feature; benefits all client platforms equally. Mobile clients
     benefit most from dormancy reducing their limited bandwidth consumption.

## Interest Management

| ID      | Feature                         |
|---------|---------------------------------|
| F-8.2.7 | Grid-Based Networking Relevancy |

1. **F-8.2.7** — Grid-based interest management for networking relevancy where the world is divided
   into fixed-size cells. Each client subscribes to cells near its position, with subscription
   changes triggered on cell boundary transitions. Update frequency decreases by cell distance: same
   cell at full rate (e.g., 60 Hz), adjacent cells at half rate (30 Hz), two-away cells at quarter
   rate (10 Hz), and beyond-range cells excluded. Grid cell size is configurable per game.
   - **Deps:** F-8.2.3
   - **Platform:** Mobile uses larger cells and lower base rates to fit within bandwidth budgets.

## 2D Support

| ID      | Feature                |
|---------|------------------------|
| F-8.2.8 | 2D Multiplayer Support |

1. **F-8.2.8** — Vec2 spatial variants for player input movement vectors, hitbox data (circle and
   rect), multicast filter Spatial(Vec2, radius), and error correction alongside 3D equivalents. 2D
   games use the same transport, replication, and prediction infrastructure without paying 3D vector
   overhead.
   - **Deps:** F-8.2.1

## Scale and Layering

| ID      | Feature                           |
|---------|-----------------------------------|
| F-8.2.9 | Player Layering (Vertical Scaling)|

1. **F-8.2.9** — Multiple instances of the same world region run on different servers as "layers"
   for massive populations (target 1M+ concurrent players per world). Overflow assignment routes new
   arrivals to less populated layers; layer transfer requests let players switch layers to join
   friends; layer merging consolidates layers as population drops; layer locking prevents
   mid-encounter transfers during boss fights. Ghost entities render cross-layer visibility so
   neighboring layers appear populated.
   - **Deps:** F-8.7.1, F-8.7.3
   - **Platform:** Server-side. Transparent to clients beyond layer selection UI.

## Delta Compression

| ID       | Feature                                |
|----------|----------------------------------------|
| F-8.2.10 | Replicated Property Quantization       |
| F-8.2.11 | Adaptive Replication Send Rate         |
| F-8.2.12 | Networked Entity Authority Transfer    |
| F-8.2.13 | Entity Tombstone with TTL              |
| F-8.2.14 | Codegen Diff/Patch for Replication     |
| F-8.2.15 | Structural Replication Op Ordering     |

1. **F-8.2.10** — Per-field quantization attributes (`#[quantize(min, max, bits)]`) on replicated
   component properties encode fields with configurable bit precision. Typical reductions: position
   deltas from 12 to 6 bytes, rotation from 16-byte quaternion to 4 bytes via smallest-three,
   velocity from 12 to 4 bytes. Quantization is reversible client-side to within the specified
   precision and dramatically reduces replication bandwidth.
   - **Deps:** F-8.2.1, F-8.2.14
2. **F-8.2.11** — Per-client replication frequency adapts to measured RTT, packet loss, and
   congestion state. Low-RTT connections run at full 60 Hz; moderate conditions drop to 30 Hz; high
   loss reduces rate and adds redundancy; mobile clients cap at a configurable maximum (e.g., 20
   Hz). The adaptive send rate integrates with the priority scheduler.
   - **Deps:** F-8.2.5, F-8.1.7
   - **Platform:** Mobile clients are always tagged for capped send rate.
3. **F-8.2.12** — Authority transfer between server and client (or between clients) for networked
   entities uses monotonic AuthorityEpoch counters, AuthorityShard tags, and PhysicsTransferSnapshot
   full-state snapshots. Transfer flow: request, snapshot, ACK, epoch bump, old authority
   relinquishes. Epoch-based conflict resolution ensures no state loss even if multiple transfer
   requests race.
   - **Deps:** F-8.2.1, F-8.4.1
4. **F-8.2.13** — Destroyed replicated entities are marked with a Tombstone (destroyed_at,
   expires_at, epoch) that persists for 2x the maximum connection RTT before the entity handle is
   recycled. Tombstones prevent stale references from high-latency clients acknowledging or
   targeting entities that have already been destroyed on the server.
   - **Deps:** F-8.2.1
5. **F-8.2.14** — Build-time codegen generates per-component diff() and patch() methods producing
   field-level delta compression with no runtime reflection overhead. A u64 changed_mask flags which
   fields were modified since the last baseline, limiting replicated components to a maximum of 64
   fields per component (larger components must be split into sub-components). The codegen output is
   pure Rust, inlined by the compiler.
   - **Deps:** F-8.2.1, F-8.2.2
6. **F-8.2.15** — Structural replication operations are prioritized over state deltas via a
   ReplicationOp enum and ReplicationOpPriority ordering: EntityDestroy=0, ComponentRemove=1,
   ComponentAdd=2, StateDelta=3. Clients process entity lifecycle changes before property updates,
   ensuring add/remove operations are never reordered past state updates that reference them.
   - **Deps:** F-8.2.1
