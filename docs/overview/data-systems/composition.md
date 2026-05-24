# Composition

How data-system primitives combine into complete gameplay features.

## What it covers

- Quests: directed graphs with data-table-backed nodes, directed-graph edges gated by
  conditions, and rewards from containers or tables.
- Dialogue: conditional directed graphs with table-backed text and audio cues.
- Abilities: data-table definitions with effect lists, cost attributes, cooldown attributes,
  and audio/VFX triggered by effect events.
- Inventory: containers with grid packing, item tables, and UI serialization.
- Equipment: containers with sockets propagating stat modifiers via the modifier stack.
- Progression: experience attributes with level-up thresholds, talent graphs with unlock
  conditions, and ability unlocks.
- Building/Crafting: data-table recipes with container-based ingredient consumption and
  outcome composition.
- Stealth/Perception: spatial-awareness state machines driven by condition graphs and attribute
  thresholds.

## Concepts

### Primitive Stacking

Every gameplay feature stacks primitives: (1) data tables store the baseline definitions, (2)
directed graphs organize branching logic or progression paths, (3) attributes track state
(experience, cooldowns, charges), (4) effects apply modifiers and trigger events, (5) conditions
gate progression. No feature is hardcoded; all are composed from these generic pieces.

### Event-Driven Decoupling

Effects emit ECS events (Applied, Expired, Removed). UI, audio, VFX, and gameplay systems listen
without coupling. A quest event can trigger audio, UI updates, and marker spawns simultaneously.

### Cross-Domain Integration

Containers hold items from tables. Tables store recipes. Graphs reference table rows in nodes.
Effects reference tables for damage types. Containers propagate modifier stacks to the owner entity.
Spatial awareness systems query table-driven threat definitions.

## How it fits

- See [directed-graphs](./directed-graphs.md), [data-tables](./data-tables.md),
  [attributes-and-effects](./attributes-and-effects.md), and
  [containers-and-slots](./containers-and-slots.md) for the primitives that compose.
- Integrates with [game-framework](../game-framework/gameplay-features.md) where compositions
  are instantiated and tuned per game.
