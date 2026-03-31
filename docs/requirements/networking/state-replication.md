# R-8.2 -- State Replication Requirements

## Property Replication

1. **R-8.2.1** — The engine **SHALL** replicate entity component properties from server to clients
   by transmitting only fields that changed since each client's last acknowledged state, using
   per-client baseline tracking and differential encoding.
   - **Rationale:** MMO-scale worlds with thousands of simultaneously updating entities require
     minimal bandwidth per update; full-state replication is infeasible.
   - **Verification:** Integration test: modify 1 field of a 20-field component on the server.
     Capture the replication packet and verify it contains only the changed field. Benchmark:
     replicate 10,000 entities with 5% per-tick change rate and verify bandwidth is at least 80%
     lower than full-state replication.
2. **R-8.2.2** — The engine **SHALL** replicate entire ECS components as atomic units with schema
   versioning, negotiating component schemas during connection handshake so that clients running
   different build versions can decode replicated data without disconnection.
   - **Rationale:** Live-service MMO operations require rolling server updates without kicking all
     players; schema versioning enables backward-compatible replication.
   - **Verification:** Integration test: connect a client with schema version N to a server running
     version N+1 (one added field with default). Verify correct decoding. Verify unknown schema
     version is rejected during handshake with a descriptive error.

## Relevancy and Filtering

1. **R-8.2.3** — The engine **SHALL** limit replication to entities within each client's area of
   interest (AOI), defined by spatial proximity via the shared BVH spatial index and game-specific
   rules (party members, guild members, raid boss), evaluating relevancy for thousands of clients
   against hundreds of thousands of entities per zone.
   - **Rationale:** Replicating the entire world to every client does not scale; AOI filtering
     reduces per-client replication to only relevant entities.
   - **Verification:** Integration test: place 100,000 entities in a zone with 1,000 clients. Verify
     each client receives only entities within its configured AOI radius. Verify a party member
     outside spatial range is still replicated. Benchmark AOI evaluation and verify within 2 ms per
     tick.
2. **R-8.2.4** — The engine **SHALL** support per-property replication conditions (owner-only,
   team-only, public) and distance-based detail tiers that reduce replication frequency and property
   count for distant entities, with configurable tier thresholds and update rates.
   - **Rationale:** Not all properties should be visible to all clients, and distant entities do not
     need full-fidelity updates; tiered replication saves bandwidth during large encounters.
   - **Verification:** Unit test: mark a property as owner-only and verify non-owner clients do not
     receive it. Integration test: place an entity 400 m from a client and verify 5 Hz position-only
     updates. Move to 10 m and verify full state at 30 Hz.

## Bandwidth Management

1. **R-8.2.5** — The engine **SHALL** assign replication priority to each entity based on relevancy
   score, entity type, and staleness, then allocate the per-connection bandwidth budget so that
   high-priority entities are replicated first and low-priority entities fill remaining budget,
   respecting the congestion controller's send rate.
   - **Rationale:** Under bandwidth pressure, the most gameplay-critical entities must receive
     updates first to maintain game quality.
   - **Verification:** Integration test: set a 50 KBps budget with 500 entities. Verify
     high-priority entities (player target, nearby hostiles) replicated every tick. Verify
     low-priority entities updated only when budget remains. Verify total send rate does not exceed
     the congestion controller's limit.
2. **R-8.2.6** — The engine **SHALL** allow entities that have not changed for a configurable period
   to enter a dormant state consuming zero replication bandwidth, excluded from priority scheduling
   entirely, and waking only when a property changes or an explicit wake event fires.
   - **Rationale:** In a typical MMO zone, 60-80% of entities are idle; excluding them from
     scheduling is critical for server scalability.
   - **Verification:** Integration test: create 10,000 entities, leave 7,000 unchanged. Verify
     dormant entities consume zero replication bandwidth. Modify one dormant entity and verify it
     wakes and is replicated within the next tick.

## Non-Functional

1. **R-8.2.7** — The state replication system **SHALL** replicate at least 100,000 entity updates
   per second per server process, with delta compression achieving at least 80% bandwidth reduction
   compared to full-state replication.
   - **Rationale:** MMO zones with thousands of entities and hundreds of clients require high
     replication throughput to maintain world state consistency.
   - **Verification:** Benchmark: replicate 10,000 entities to 100 clients with 5% per-tick change
     rate. Measure updates per second and verify at least 100,000. Measure bandwidth and verify at
     least 80% reduction vs. full-state baseline.
2. **R-8.2.8** — The engine **SHALL** enforce a configurable per-client bandwidth budget (default
   128 KBps downstream, 32 KBps upstream) that is never exceeded, with priority scheduling ensuring
   gameplay-critical entities consume available bandwidth first.
   - **Rationale:** Many players are on consumer internet with limited bandwidth. Exceeding
     per-client limits causes packet loss and degraded gameplay.
   - **Verification:** Integration test: set a 128 KBps budget with 500 entities. Verify downstream
     bandwidth never exceeds 128 KBps. Verify high-priority entities receive updates every tick
     while low-priority entities are deferred.
