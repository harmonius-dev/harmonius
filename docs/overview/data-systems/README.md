# Data Systems

Generic, reusable building blocks for game data. Designers compose these primitives to build any
gameplay feature.

## Topics

- [directed-graphs](./directed-graphs.md) — nodes, edges, and conditions for quests, dialogue,
  abilities, and progression.
- [data-tables](./data-tables.md) — typed rows for items, stats, recipes, and lookups.
- [attributes-and-effects](./attributes-and-effects.md) — numeric stats with stackable buffs,
  debuffs, and time-based modifiers.
- [containers-and-slots](./containers-and-slots.md) — inventories, equipment, hotbars, and any
  bounded collection of items.
- [composition](./composition.md) — how the primitives combine into real game features.

## Key takeaways

- Directed graphs (DAGs with cycles allowed) represent quests, dialogue, ability trees, and
  progression; conditions gate transitions enabling complex branching without hard-coding.
- Typed data tables enable queries: "all weapons with damage > 50", "recipes requiring iron ore";
  queries power loot tables and crafting systems.
- Attributes (numeric stats) with stackable effects allow complex behavior: +20% fire damage buff,
  -5 armor debuff, 0.5× speed slow stacking independently.
- Containers (inventories) with configurable slots and stacking rules model infinite variety of
  collection types from backpacks to equipment racks.
- Composition layers primitives: quest uses graph + attributes (quest progress counter); ability
  uses graph (skill tree) + containers (cooldowns) + effects (buffs).

## Integration risks

- Graph cycles (if not carefully designed) cause infinite loops during evaluation; cycle detection or
  acyclic assumptions necessary. See [directed-graphs.md](./directed-graphs.md) for cycle policy.
- Data table queries (finding rows matching criteria) scale poorly with large tables; indexing
  required for performance. See [data-tables.md](./data-tables.md) for query optimization.
- Effect stacking (multiple buffs/debuffs accumulating) can produce extreme values; caps and
  diminishing returns necessary to maintain game balance. See [attributes-and-effects.md](./attributes-and-effects.md)
  for capping strategies.
- Container serialization (saving inventory state) must handle item deletion (item type removed from
  game); fallback item types necessary. See [containers-and-slots.md](./containers-and-slots.md)
  for migration strategies.
- Composition feature interactions (graph + attributes + containers) can produce unintended emergent
  behavior; playtesting necessary. See [composition.md](./composition.md) for interaction testing.
