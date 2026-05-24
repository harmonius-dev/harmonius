# Event Logs

Recording what happened: memory, gossip, threat tables, and combat logs.

## What it covers

- Generic bounded ring buffers: one primitive for NPC memory, gossip, threat tables, combat
  logs.
- Configurable accuracy decay: events lose fidelity over time (linear, exponential, step
  curves).
- Inter-entity propagation: events spread between nearby entities with per-hop accuracy loss.
- Threshold triggers: fire ECS events when log count, time window, or event type crosses a
  threshold.
- Memory overhead: compile-time size limit (256 bytes per entry) for mobile budgets.
- Bit-identical save/load for deterministic replays.

## Concepts

### Event Logs

An event log is a bounded queue storing timestamped entries. Oldest entries evict as new ones
arrive. Each entry tracks what happened, when, and optional metadata. Decay curves reduce confidence
over time: a distant memory becomes fuzzy, while recent events are crisp.

### Propagation and Gossip

Events propagate between nearby entities (NPCs) through the spatial index. Each hop spreads the
event but reduces accuracy. This models how rumors spread through a city: direct witnesses know
exactly what happened; gossip chains accumulate errors.

### Triggers and Reactions

Thresholds fire events based on log contents: "3 hostile sightings in 60 seconds triggers Alert".
The engine evaluates these queries each frame; matching logs emit ECS events that trigger reactions
(audio warnings, NPC dialogue, combat entry).

## How it fits

- See [grids-and-volumes](./grids-and-volumes.md) for spatial propagation mechanics.
- See [timelines](./timelines.md) for memory-triggered cinematics.
- Integrates with [data-systems](../data-systems/attributes-and-effects.md) for threat
  tracking.
- Integrates with [ai](../ai/perception.md) and
  [game-framework](../game-framework/gameplay-features.md) for NPC memory and reactions.
