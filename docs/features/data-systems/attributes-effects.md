# 16.1 — Attributes and Effects

## Meters

| ID       | Feature                       |
|----------|-------------------------------|
| F-16.1.1 | Bounded Meter Primitive      |
| F-16.1.2 | Meter Threshold Events       |
| F-16.1.3 | High-Throughput Meter Update |

1. **F-16.1.1** — Meters represent bounded numeric values with configurable minimum, maximum,
   default, drain rate, and fill rate. Each meter holds a current value, a previous value for delta
   tracking, and a modifier stack. Meters are the universal primitive for health, stamina, mana,
   hunger, thirst, and reputation — one type replaces genre-specific trackers.
   - **Deps:** F-1.1.1 (ECS Storage), F-1.5.1 (Event Channels)
2. **F-16.1.2** — Each meter can declare thresholds with a target value, direction (Rising, Falling,
   Either), and action (FireEvent, ApplyEffect, RemoveEffect). When the current value crosses a
   threshold in the matching direction, the configured action executes. Thresholds drive low-health
   warnings, level-up triggers, and reputation tier transitions.
   - **Deps:** F-16.1.1, F-1.5.1
3. **F-16.1.3** — Meter advance runs as a batched ECS system processing thousands of meters per
   frame. Per-meter cost stays under 0.5 µs including modifier evaluation and threshold checks.
   1,000 active meters advance within 0.5 ms total budget on desktop hardware.
   - **Deps:** F-16.1.1, F-1.1.1

## Attributes

| ID       | Feature                     |
|----------|-----------------------------|
| F-16.1.4 | Schema-Driven Attribute Sets|
| F-16.1.5 | Zero-Copy Attribute Access  |

1. **F-16.1.4** — Attribute sets are schema-defined named-value collections. A schema declares a
   list of typed attribute definitions (F32, I32, Bool, Enum) with per-field min, max, and default.
   Instances store only the mutable values plus a modifier stack per attribute, keeping per-entity
   footprint small.
   - **Deps:** F-1.1.1, F-1.6 (Asset Loading)
2. **F-16.1.5** — Attribute reads evaluate the modifier stack on demand with memoization per frame.
   10,000 reads complete within 0.1 ms on desktop hardware. Writes to the base value invalidate
   memoized current values.
   - **Deps:** F-16.1.4

## Modifier Stacks

| ID       | Feature                     |
|----------|-----------------------------|
| F-16.1.6 | Layered Modifier Pipeline  |

1. **F-16.1.6** — Modifier stacks compose flat additive, percent additive, percent multiplicative,
   and override operations in deterministic priority order. The `StatAggregator` evaluates (base +
   flat) *(1 + pct_add)* pct_mul, then applies overrides by priority. Per-source tracking enables
   bulk removal when an effect expires.
   - **Deps:** F-16.1.1, F-16.1.4

## Effects

| ID       | Feature                 |
|----------|-------------------------|
| F-16.1.7 | Effect Definitions     |
| F-16.1.8 | Effect Stacking Rules  |
| F-16.1.9 | Effect VFX Indicators  |
| F-16.1.10| Per-Entity Effect Budget|

1. **F-16.1.7** — Effect definitions are immutable assets with effect type (Instant, Duration,
   Periodic, Infinite), magnitude, duration, period, stacking rule, modifier list, gameplay tags,
   and optional gating condition. Effects apply modifiers to target meters or attributes and fire
   events on lifecycle transitions.
   - **Deps:** F-16.1.1, F-16.1.4, F-16.1.6
2. **F-16.1.8** — Effects declare stacking rules (Additive, Multiplicative, HighestWins,
   NonStacking) resolving how duplicate effects on the same target combine. Designers tune stacking
   per effect without engine code changes.
   - **Deps:** F-16.1.7
3. **F-16.1.9** — Effects can associate an optional VFX effect graph handle. When an effect becomes
   active, the engine spawns the configured VFX instance parented to the target entity and despawns
   it on expiry or removal.
   - **Deps:** F-16.1.7, F-11.4 (VFX Effect Graph)
4. **F-16.1.10** — Per-entity active effect lists bound to 64 concurrent effects, processed within
   0.1 ms per tick on desktop hardware including ticking, stacking, and expiration.
   - **Deps:** F-16.1.7

## Conditions

| ID        | Feature                       |
|-----------|-------------------------------|
| F-16.1.11 | Condition Expression Trees   |
| F-16.1.12 | Effect Lifecycle Events      |

1. **F-16.1.11** — Condition expression trees (And, Or, Not, Leaf) gate effect application and meter
   threshold actions. Leaf conditions reference codegen'd functions in the middleman `.dylib`,
   evaluated against a runtime `ConditionContext`. No runtime interpreter.
   - **Deps:** F-16.1.7, F-1.3 (Codegen)
2. **F-16.1.12** — The engine emits Applied, Ticked, Expired, and Removed effect events through the
   ECS event channel so UI, audio, achievements, and VFX systems react to effect lifecycle without
   polling.
   - **Deps:** F-16.1.7, F-1.5.1
