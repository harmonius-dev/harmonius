# 8.2 — State Replication

## Property Replication

### F-8.2.1 Delta-Compressed Property Replication

Replicate entity component properties from server to clients by transmitting only the fields that
changed since each client's last acknowledged state. Delta compression uses per-client baseline
tracking and XOR-based or arithmetic differencing to minimize bandwidth. Essential for MMO-scale
worlds where thousands of entities update simultaneously but each client only needs the minimal
diff.

- **Requirements:** R-8.2.1
- **Dependencies:** F-8.1.3, F-8.1.4
- **Platform notes:** Mobile clients receive more aggressively delta-compressed updates with lower
  quantization precision to fit within ~500 Kbps bandwidth budgets.

### F-8.2.2 Component Replication with Schema Versioning

Replicate entire ECS components as atomic units with schema versioning so that clients running
slightly different build versions can still decode replicated data. Component schemas are registered
at startup and negotiated during connection handshake, enabling rolling server updates without
kicking all players — critical for live-service MMO operations.

- **Requirements:** R-8.2.2
- **Dependencies:** F-8.2.1, F-8.1.1
- **Platform notes:** Mobile clients may run older app versions due to app store update delays;
  schema versioning is critical for mobile cross-version compatibility.

## Relevancy and Filtering

### F-8.2.3 Area-of-Interest Filtering

Limit replication to entities within each client's area of interest (AOI), defined by spatial
proximity, visibility, and game-specific rules (e.g., always replicate party members, guild members
in the same zone, or raid boss regardless of distance). AOI queries use the shared BVH spatial index
(F-1.9.8) for spatial relevancy filtering, enabling efficient evaluation for thousands of clients
against hundreds of thousands of entities per zone.

- **Requirements:** R-8.2.3
- **Dependencies:** F-8.2.1, F-1.9.8 (Network Relevancy Integration)
- **Platform notes:** None

### F-8.2.4 Conditional and Tiered Replication

Support per-property replication conditions (owner-only, team-only, public) and distance-based
detail tiers that reduce replication frequency and property count for distant entities. A player 400
meters away receives position updates at 5 Hz with no animation state; a nearby player receives full
state at 30 Hz. This tiered approach is essential for supporting hundreds of players in guild siege
or world boss encounters.

- **Requirements:** R-8.2.4
- **Dependencies:** F-8.2.3
- **Platform notes:** Mobile uses more aggressive distance tiers: near (full, 20 Hz), mid (reduced,
  5 Hz), far (position-only, 2 Hz) to stay within bandwidth budget.

## Bandwidth Management

### F-8.2.5 Priority Scheduling and Bandwidth Budgeting

Assign replication priority to each entity based on relevancy score, entity type, and staleness,
then allocate the per-connection bandwidth budget to maximize gameplay quality. High-priority
entities (the player's target, nearby hostiles, spell effects) are replicated first; low-priority
entities (distant NPCs, ambient creatures) fill remaining budget. The scheduler must respect the
congestion controller's send rate from F-8.1.7.

- **Requirements:** R-8.2.5
- **Dependencies:** F-8.2.3, F-8.1.7
- **Platform notes:** Mobile bandwidth budget is typically 50-100 KB/s (vs. 500+ KB/s on desktop).
  Server tags mobile connections for tighter budget allocation.

### F-8.2.6 Entity Dormancy

Allow entities that have not changed for a configurable period to enter a dormant state where they
consume zero replication bandwidth. Dormant entities are excluded from priority scheduling entirely
and only wake when a property changes or an explicit wake event fires. In a typical MMO zone, 60-80%
of entities are dormant at any time (idle NPCs, placed objects, inactive triggers), making this
critical for server scalability.

- **Requirements:** R-8.2.6
- **Dependencies:** F-8.2.5
- **Platform notes:** Server-side feature; benefits all client platforms equally. Mobile clients
  benefit most from dormancy reducing their limited bandwidth consumption.
