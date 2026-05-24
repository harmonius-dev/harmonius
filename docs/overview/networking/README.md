# Networking

How multiple players and servers synchronize game state.

## Topics

- [transport-and-replication](./transport-and-replication.md) — sending data over the network
  and keeping entities in sync.
- [session-and-prediction](./session-and-prediction.md) — lobbies, rollback, and latency hiding.
- [communication-and-replay](./communication-and-replay.md) — RPCs, dialogue, and recording
  gameplay.
- [infrastructure-and-anti-cheat](./infrastructure-and-anti-cheat.md) — servers, matchmaking,
  and cheating prevention.

## Key takeaways

- Delta compression (sending only changed properties) reduces bandwidth from O(N) to O(changed),
  enabling thousands of synchronized entities.
- Client prediction (simulating client actions locally) followed by server reconciliation (correcting
  divergence) hides network latency effectively.
- Relevance culling (only replicating entities visible/important to each client) scales to large
  worlds without bandwidth explosion.
- Lag compensation (rewinding server state to client's perceived time) provides fair hit detection
  and aiming despite latency.
- Deterministic replay (recording inputs and RNG seeds) enables spectator modes and hacking
  detection via playback verification.

## Integration risks

- Delta compression must track per-entity dirty flags; forgetting flags causes stale properties; too
  many flags wastes memory. See [transport-and-replication.md](./transport-and-replication.md)
  for dirty flag patterns.
- Client prediction desynchronization (client and server diverge significantly) causes visible
  teleporting; re-prediction must be smooth. See [session-and-prediction.md](./session-and-prediction.md)
  for reconciliation latency.
- Relevance culling culling too aggressively (entities disappearing off-screen while player still
  sees them) breaks immersion; scope poorly breaks gameplay. See [transport-and-replication.md](./transport-and-replication.md)
  for relevance tuning.
- Lag compensation time window (how far back to roll back) must match network latency; too short
  misses compensations, too long causes gameplay unfairness. See [session-and-prediction.md](./session-and-prediction.md)
  for latency measurement.
- Replay determinism requires matching RNG sequences; any randomness source divergence causes
  replay mismatch. See [communication-and-replay.md](./communication-and-replay.md) for RNG
  seeding.

## Integration risks

- Anti-cheat detection rates (false positives vs false negatives) require careful tuning; overly
  aggressive detection bans innocent players. See [infrastructure-and-anti-cheat.md](./infrastructure-and-anti-cheat.md)
  for detection thresholds.
