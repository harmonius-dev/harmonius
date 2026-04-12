# R-16.1 — Attributes and Effects Requirements

## Meters

1. **R-16.1.1** — The engine **SHALL** provide a `Meter` primitive representing a bounded numeric
   value with configurable minimum, maximum, default value, drain rate, and fill rate.
   - **Rationale:** Meters are the universal primitive for health, stamina, mana, hunger, thirst,
     reputation, and any other bounded numeric resource — one type replaces many genre-specific
     systems.
   - **Verification:** Unit test: construct a meter with min=0, max=100, default=50; assert initial
     value; apply drain and fill; verify clamping at bounds.
2. **R-16.1.2** — The engine **SHALL** evaluate meter threshold crossings (rising, falling, either)
   and fire threshold events when the current value crosses a configured threshold value.
   - **Rationale:** Thresholds drive game logic such as low-health warnings, level-up triggers, and
     reputation tier transitions without per-game custom code.
   - **Verification:** Unit test: configure a rising threshold at 80; increment meter from 70 to 90;
     assert event fires exactly once with direction Rising.
3. **R-16.1.3** — The engine **SHALL** evaluate 1,000 active meters and their threshold checks
   within 0.5 ms per frame on desktop hardware.
   - **Rationale:** Meters run every frame for every active entity; evaluation must stay within
     frame budgets for thousands of NPCs.
   - **Verification:** Benchmark: spawn 1,000 meter components; run one advance pass; assert total
     time under 0.5 ms at 1% variance.

## Attributes

1. **R-16.1.4** — The engine **SHALL** provide schema-defined attribute sets where each schema
   declares a named collection of typed attribute definitions (F32, I32, Bool, Enum) with per-field
   minimum, maximum, and default.
   - **Rationale:** Attribute sets are the universal primitive for character stats, item properties,
     and any entity-scoped numeric collection authored from data.
   - **Verification:** Unit test: author a schema with four attributes; instantiate an attribute
     set; read and write each field; assert defaults match schema.
2. **R-16.1.5** — The engine **SHALL** evaluate 10,000 attribute reads across modifier stacks within
   0.1 ms per frame on desktop hardware.
   - **Rationale:** Gameplay systems query attributes dozens of times per entity per frame;
     aggregate cost must remain well within the frame budget.
   - **Verification:** Benchmark: build an attribute set with 10 attributes each with 4 modifiers;
     perform 10,000 reads; assert total time under 0.1 ms.

## Modifier Stacks

1. **R-16.1.6** — The engine **SHALL** apply modifier stacks to meters and attributes with layered
   operations (flat additive, percent additive, percent multiplicative, override) and deterministic
   evaluation order based on priority.
   - **Rationale:** Layered modifiers let designers compose buffs, debuffs, and equipment bonuses
     without custom aggregation code per system.
   - **Verification:** Unit test: stack flat +10, percent +20, override 50; assert final value
     equals override (50). Assert ordering by priority is stable.

## Effects

1. **R-16.1.7** — The engine **SHALL** provide effect definitions with effect type (Instant,
   Duration, Periodic, Infinite), magnitude, optional duration in ticks, optional period in ticks,
   and a list of effect modifiers targeting meters or attributes.
   - **Rationale:** Effects are time-limited modifier bundles that drive buffs, debuffs, damage over
     time, healing over time, and status conditions across all genres.
   - **Verification:** Unit test: apply a periodic effect with period 10 ticks; advance 30 ticks;
     assert three modifier applications.
2. **R-16.1.8** — The engine **SHALL** support configurable effect stacking rules including
   additive, multiplicative, highest-wins, and non-stacking when multiple effects target the same
   meter or attribute on the same entity.
   - **Rationale:** Stacking policy is a design choice that must be data-driven — "do two poisons
     stack or does the stronger one win" is answered by the effect definition, not engine code.
   - **Verification:** Unit test: apply two effects with highest-wins stacking; assert only the
     larger magnitude applies. Repeat with additive; assert sum applies.
3. **R-16.1.9** — The engine **SHALL** associate optional VFX effect graph instances with active
   gameplay effects so that visual indicators spawn when an effect becomes active and despawn when
   it expires or is removed.
   - **Rationale:** Players need visual feedback for active status effects; the association must be
     authored in data alongside the effect definition, not hardcoded.
   - **Verification:** Integration test: apply an effect with a configured VFX handle; assert a VFX
     instance spawns under the entity; remove effect; assert VFX despawns.
4. **R-16.1.10** — The engine **SHALL** apply at most 64 concurrent effects per entity with
   per-entity tick, stack, and expiration processing within 0.1 ms per frame on desktop hardware.
   - **Rationale:** Complex builds stack many simultaneous effects; per-entity processing must stay
     cheap enough for crowds of 100+ entities.
   - **Verification:** Benchmark: apply 64 effects to one entity; run one tick; assert total time
     under 0.1 ms.

## Conditions

1. **R-16.1.11** — The engine **SHALL** evaluate boolean condition expression trees (And, Or, Not,
   Leaf) to gate effect application and meter threshold actions, with condition leaf functions
   sourced from a codegen'd condition registry.
   - **Rationale:** Designers compose gating logic visually without writing code; native codegen
     removes runtime interpretation overhead.
   - **Verification:** Unit test: build And(Leaf(A), Or(Leaf(B), Not(Leaf(C)))); evaluate against
     contexts; assert correct boolean results. Assert dispatch uses function pointers.
2. **R-16.1.12** — The engine **SHALL** fire applied, ticked, expired, and removed effect events
   through the ECS event channel so that consuming systems can react to effect lifecycle changes.
   - **Rationale:** Downstream systems (UI, audio, achievements, VFX) depend on effect lifecycle
     without tight coupling to the effects subsystem.
   - **Verification:** Integration test: subscribe to EffectEvent channel; apply, tick, expire an
     effect; assert all four events fire in order with the correct entity and definition.
