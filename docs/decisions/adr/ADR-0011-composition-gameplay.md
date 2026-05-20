# Composition over subsystems for gameplay

## Status

Accepted — 2025-04-08 (backfilled 2026-05-20)

## Context

Game engines historically ship dedicated subsystems for genre features: an inventory subsystem, a
quest subsystem, an ability subsystem, a skill-tree subsystem, a loot subsystem, a crafting
subsystem. Each ships with bespoke editor UIs, save formats, and runtime data. This is fast for the
first game in the genre but compounds maintenance cost as feature variations multiply.

The alternative is to provide a small set of generic primitives that compose into any genre feature:
directed graphs, data tables, attributes/effects, containers/slots (data layer); plus grids, spatial
awareness, timelines, event logs (simulation layer). Quests, abilities, inventory, progression,
crafting, and most other gameplay become recipes wired in the visual editor.

## Decision

Game features are not built as dedicated subsystems. Users compose generic primitives from the data
systems and simulation layers in visual editors to create any gameplay.

Four data primitives:

| Primitive             | Purpose                                                         |
|-----------------------|-----------------------------------------------------------------|
| Directed graphs       | DAG of typed nodes and edges for quests, dialogue, progression  |
| Data tables           | Typed row/column grids for items, loot, stats, localization     |
| Attributes / effects  | Numeric attribute paired with stack of modifiers                |
| Containers / slots    | Typed slotted containers for inventory, equipment, crafting     |

Four simulation primitives:

| Primitive             | Purpose                                                         |
|-----------------------|-----------------------------------------------------------------|
| Grids / volumes       | Uniform grid or voxel for pathfinding, influence, fog of war    |
| Spatial awareness     | Perception, occlusion, line-of-sight queries                    |
| Timelines             | Time-ordered sequence of values and events                      |
| Event logs            | Append-only history for replay, AI memory, audit trails         |

Standard genre features map to these primitives in [architecture.md](../../architecture.md)
"Composition Model" and
[docs/design/data-systems/composition.md](../../design/data-systems/composition.md).

## Consequences

- The engine ships **no** dedicated quest, ability, inventory, crafting, or progression
  subsystems. Those are user-authored compositions.
- Editor UI is generic per primitive (graph editor, table editor, container editor). No
  feature-specific authoring panels.
- Save and replication are uniform: every primitive serializes the same way.
- New genres do not require engine changes. They require new editor templates and a few new
  node palette entries.
- The composition guide ([composition.md](../../design/data-systems/composition.md)) is the
  pedagogical entry point. Without it, the system looks under-specified.

## Alternatives Considered

- **Dedicated subsystems per genre feature** — fast for the first game; expensive at scale.
- **Hybrid** (some dedicated subsystems plus some primitives) — drifts toward dedicated
  subsystems over time; rejected for cohesion.

## References

- [docs/design/constraints.md](../../design/constraints.md) "User-Facing Authoring"
- [docs/design/data-systems/composition.md](../../design/data-systems/composition.md)
- [docs/architecture.md](../../architecture.md) "Composition Model"
