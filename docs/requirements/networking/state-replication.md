# R-8.2 — State Replication Requirements

## Property Replication

### R-8.2.1 Delta-Compressed Property Replication

The engine **SHALL** replicate entity component properties from server to clients by transmitting only
fields that changed since each client's last acknowledged state, using per-client baseline tracking
and differential encoding.

- **Derived from:** [F-8.2.1](../../features/networking/state-replication.md)
- **Rationale:** MMO-scale worlds with thousands of simultaneously updating entities require minimal
  bandwidth per update; full-state replication is infeasible.
- **Verification:** Integration test: modify 1 field of a 20-field component on the server. Capture
  the replication packet and verify it contains only the changed field. Verify the client reconstructs
  the correct full state. Benchmark: replicate 10,000 entities with 5% per-tick change rate and verify
  bandwidth is at least 80% lower than full-state replication.

### R-8.2.2 Component Replication with Schema Versioning

The engine **SHALL** replicate entire ECS components as atomic units with schema versioning, negotiating
component schemas during connection handshake so that clients running different build versions can
decode replicated data without disconnection.

- **Derived from:** [F-8.2.2](../../features/networking/state-replication.md)
- **Rationale:** Live-service MMO operations require rolling server updates without kicking all
  players; schema versioning enables backward-compatible replication.
- **Verification:** Integration test: connect a client with schema version N to a server running
  schema version N+1 (one added field with default). Verify the client receives and correctly decodes
  all components. Verify a client with an unknown schema version is rejected during handshake with a
  descriptive error.

## Relevancy and Filtering

### R-8.2.3 Area-of-Interest Filtering

The engine **SHALL** limit replication to entities within each client's area of interest (AOI), defined
by spatial proximity via the shared BVH spatial index and game-specific rules (party members, guild
members, raid boss), evaluating relevancy for thousands of clients against hundreds of thousands of
entities per zone.

- **Derived from:** [F-8.2.3](../../features/networking/state-replication.md)
- **Rationale:** Replicating the entire world to every client does not scale; AOI filtering reduces
  per-client replication to only relevant entities.
- **Verification:** Integration test: place 100,000 entities in a zone with 1,000 connected clients.
  Verify each client receives only entities within its configured AOI radius. Verify a party member
  outside spatial range is still replicated. Benchmark AOI evaluation and verify it completes within
  2 ms per tick for the above entity and client counts.

### R-8.2.4 Conditional and Tiered Replication

The engine **SHALL** support per-property replication conditions (owner-only, team-only, public) and
distance-based detail tiers that reduce replication frequency and property count for distant entities,
with configurable tier thresholds and update rates.

- **Derived from:** [F-8.2.4](../../features/networking/state-replication.md)
- **Rationale:** Not all properties should be visible to all clients, and distant entities do not
  need full-fidelity updates; tiered replication saves bandwidth during large-scale encounters.
- **Verification:** Unit test: mark a property as owner-only and verify non-owner clients do not
  receive it. Integration test: place an entity 400 m from a client and verify it receives position
  updates at 5 Hz with no animation state. Move the entity to 10 m and verify the client receives
  full state at 30 Hz.

## Bandwidth Management

### R-8.2.5 Priority Scheduling and Bandwidth Budgeting

The engine **SHALL** assign replication priority to each entity based on relevancy score, entity type,
and staleness, then allocate the per-connection bandwidth budget so that high-priority entities are
replicated first and low-priority entities fill remaining budget, respecting the congestion
controller's send rate.

- **Derived from:** [F-8.2.5](../../features/networking/state-replication.md)
- **Rationale:** Under bandwidth pressure, the most gameplay-critical entities must receive updates
  first to maintain game quality.
- **Verification:** Integration test: set a per-connection bandwidth budget of 50 KBps with 500
  entities needing replication. Verify high-priority entities (player target, nearby hostiles) are
  replicated every tick. Verify low-priority entities (distant NPCs) receive updates only when budget
  remains. Verify total send rate does not exceed the congestion controller's limit.

### R-8.2.6 Entity Dormancy

The engine **SHALL** allow entities that have not changed for a configurable period to enter a dormant
state consuming zero replication bandwidth, excluded from priority scheduling entirely, and waking
only when a property changes or an explicit wake event fires.

- **Derived from:** [F-8.2.6](../../features/networking/state-replication.md)
- **Rationale:** In a typical MMO zone, 60-80% of entities are idle; excluding them from replication
  scheduling is critical for server scalability.
- **Verification:** Integration test: create 10,000 entities, leave 7,000 unchanged for the
  configured dormancy period, and verify they consume zero replication bandwidth. Modify one dormant
  entity's property and verify it wakes and is replicated within the next tick. Verify an explicit
  wake event also triggers replication.
