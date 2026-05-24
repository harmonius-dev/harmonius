# Attributes and Effects

Numeric stats, modifiers, condition-gated effects, and lifecycle events.

## What it covers

- Meters: bounded numeric values (health, stamina, mana) with rising/falling thresholds that
  trigger events.
- Attributes: named value sets (floats, integers, booleans, enums) with per-entity modifier
  stacks.
- Modifier stacks: layered priority pipeline (flat additive, percent additive, percent
  multiplicative, override) for deterministic stat calculation.
- Effects: asset definitions (Instant, Duration, Periodic, or Infinite) that apply modifiers
  to target entities.
- Effect stacking: configurable policies (additive, multiplicative, highest-wins, non-stacking)
  for designers to tune.
- Condition expressions: boolean logic gating effect application and thresholds.
- VFX binding: effects spawn and despawn visual effects on lifecycle transitions.
- Lifecycle events: Applied, Ticked, Expired, Removed flow through the ECS event system.

## Concepts

### Meters and Thresholds

A meter is a bounded numeric container with current/min/max values. Thresholds declare value +
direction (rising, falling, or either). When the meter crosses a threshold, an event fires,
triggering gameplay logic (level-ups, low-health warnings, reputation tier transitions).

### Modifiers and Stacks

Each attribute has a stack of modifiers from active effects, equipment, buffs, and debuffs. The
stack applies operations in deterministic order: flat bonuses, then percent bonuses, then percent
multipliers, then overrides. Each modifier tracks its source for atomic removal.

### Effects

Effects are data-driven specifications: type (Instant or Duration), magnitude, duration, period (for
Periodic), stacking rule, and modifier list. Conditions control when effects apply. On lifecycle
transitions (Applied, Ticked, Expired, Removed), the engine emits ECS events for UI, audio, VFX, and
achievements to consume.

## How it fits

- See [containers-and-slots](./containers-and-slots.md) for stat propagation from socketed
  items.
- See [data-tables](./data-tables.md) for effect definitions stored as rows.
- See [composition](./composition.md) for how attributes compose into game features.
- Integrates with [physics](../physics/dynamics.md) for damage effects.
- Integrates with [ai](../ai/decision-making.md) for ability effects.
