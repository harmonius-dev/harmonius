# Attributes and Effects — Test Cases

Companion to [attributes-effects.md](attributes-effects.md).

Test case IDs use `TC-16.1.Z.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                | Req       |
|---------------|-------------------------------------|-----------|
| TC-16.1.1.1   | `test_meter_construct_defaults`     | R-16.1.1  |
| TC-16.1.1.2   | `test_meter_clamp_min_max`          | R-16.1.1  |
| TC-16.1.1.3   | `test_meter_drain_rate`             | R-16.1.1  |
| TC-16.1.1.4   | `test_meter_fill_rate`              | R-16.1.1  |
| TC-16.1.2.1   | `test_threshold_rising_fires_once`  | R-16.1.2  |
| TC-16.1.2.2   | `test_threshold_falling_fires`      | R-16.1.2  |
| TC-16.1.2.3   | `test_threshold_either_direction`   | R-16.1.2  |
| TC-16.1.2.4   | `test_threshold_no_double_fire`     | R-16.1.2  |
| TC-16.1.4.1   | `test_attribute_schema_typed_def`   | R-16.1.4  |
| TC-16.1.4.2   | `test_attribute_set_defaults`       | R-16.1.4  |
| TC-16.1.4.3   | `test_attribute_min_max_clamp`      | R-16.1.4  |
| TC-16.1.5.1   | `test_attribute_memoized_read`      | R-16.1.5  |
| TC-16.1.5.2   | `test_attribute_invalidate_on_set`  | R-16.1.5  |
| TC-16.1.6.1   | `test_modifier_flat_additive`       | R-16.1.6  |
| TC-16.1.6.2   | `test_modifier_percent_additive`    | R-16.1.6  |
| TC-16.1.6.3   | `test_modifier_percent_mul`         | R-16.1.6  |
| TC-16.1.6.4   | `test_modifier_override_priority`   | R-16.1.6  |
| TC-16.1.6.5   | `test_modifier_full_pipeline`       | R-16.1.6  |
| TC-16.1.7.1   | `test_effect_instant_apply`         | R-16.1.7  |
| TC-16.1.7.2   | `test_effect_duration_expires`      | R-16.1.7  |
| TC-16.1.7.3   | `test_effect_periodic_ticks`        | R-16.1.7  |
| TC-16.1.7.4   | `test_effect_infinite_persists`     | R-16.1.7  |
| TC-16.1.8.1   | `test_stacking_highest_wins`        | R-16.1.8  |
| TC-16.1.8.2   | `test_stacking_additive`            | R-16.1.8  |
| TC-16.1.8.3   | `test_stacking_non_stacking`        | R-16.1.8  |
| TC-16.1.11.1  | `test_condition_and_or_not`         | R-16.1.11 |
| TC-16.1.11.2  | `test_condition_leaf_dispatch`      | R-16.1.11 |
| TC-13.9.1.1   | `test_character_stats_from_set`     | R-13.9.1  |
| TC-13.9.2.1   | `test_hp_mp_stamina_meters_bound`   | R-13.9.2  |
| TC-13.9.3.1   | `test_buff_equipment_mod_stack`     | R-13.9.3  |
| TC-13.10.3.1  | `test_dot_hot_effect_periodic`      | R-13.10.3 |
| TC-13.12.5.1  | `test_faction_rep_tier_thresholds`  | R-13.12.5 |

1. **TC-16.1.1.1** `test_meter_construct_defaults` — Construct a meter with min=0, max=100,
   default=50. Assert initial value is 50 and bounds match.
   - Input: `Meter::new(MeterDef { min: 0.0, max: 100.0, default: 50.0, drain: 0.0, fill: 0.0 })`
   - Expected: `meter.current() == 50.0`, `meter.min() == 0.0`, `meter.max() == 100.0`

2. **TC-16.1.1.2** `test_meter_clamp_min_max` — Apply a delta that would push current below min and
   another that would push above max. Assert both clamp to bounds.
   - Input: meter `(min=0, max=100, current=50)`; `apply_delta(-200.0)` then `apply_delta(+500.0)`
   - Expected: after first call `meter.current() == 0.0`; after second `meter.current() == 100.0`

3. **TC-16.1.1.3** `test_meter_drain_rate` — Meter with drain rate 5/s; advance 2 s. Assert current
   decreases by 10.
   - Input: `Meter { current: 50.0, drain_rate: 5.0, .. }`; `advance(Duration::from_secs(2))`
   - Expected: `meter.current() == 40.0`

4. **TC-16.1.1.4** `test_meter_fill_rate` — Meter with fill rate 10/s; advance 0.5 s. Assert current
   increases by 5.
   - Input: `Meter { current: 20.0, fill_rate: 10.0, .. }`; `advance(Duration::from_millis(500))`
   - Expected: `meter.current() == 25.0`

5. **TC-16.1.2.1** `test_threshold_rising_fires_once` — Rising threshold at 80; advance current from
   70 to 90 in two steps. Assert event fires exactly once.
   - Input: `Threshold { value: 80.0, direction: Rising, .. }`, `set_current(70.0)` then
     `set_current(90.0)`
   - Expected: one `MeterThresholdEvent { direction: Rising, value: 80.0, .. }`

6. **TC-16.1.2.2** `test_threshold_falling_fires` — Falling threshold at 20; advance current from 30
   to 10. Assert one event fires with `Falling` direction.
   - Input: `Threshold { value: 20.0, direction: Falling, .. }`; current 30 → 10
   - Expected: one `MeterThresholdEvent { direction: Falling, .. }`

7. **TC-16.1.2.3** `test_threshold_either_direction` — Either threshold at 50; cross upward then
   downward. Assert two events fire.
   - Input: `Threshold { value: 50.0, direction: Either, .. }`; 40 → 60 → 30
   - Expected: events `[Either Rising, Either Falling]` (length 2)

8. **TC-16.1.2.4** `test_threshold_no_double_fire` — Stay above the threshold across multiple frames
   without re-crossing. Assert no additional events fire after the initial cross.
   - Input: rising threshold at 50; sequence 40 → 60 → 70 → 80
   - Expected: exactly one `Rising` event

9. **TC-16.1.4.1** `test_attribute_schema_typed_def` — Author a schema with F32, I32, Bool, Enum
   fields. Instantiate; assert each field's type is reported correctly.
   - Input: `AttributeSchema { fields: [F32 "speed", I32 "level", Bool "alive", Enum "role"] }`
   - Expected: `set.field_type("speed") == F32`, etc., for all four

10. **TC-16.1.4.2** `test_attribute_set_defaults` — Schema with default values for each field.
    Instantiate without overrides; assert read returns the schema defaults.
    - Input: schema fields with defaults `{ speed: 5.0, level: 1, alive: true, role: 0 }`
    - Expected: `set.read("speed") == 5.0`, `set.read("level") == 1`, etc.

11. **TC-16.1.4.3** `test_attribute_min_max_clamp` — Field with min=0, max=10; write 15 then -5.
    Assert both writes clamp to bounds.
    - Input: schema with `Field { min: 0.0, max: 10.0 }`; `write("hp", 15.0)` then
      `write("hp", -5.0)`
    - Expected: after first `read("hp") == 10.0`; after second `read("hp") == 0.0`

12. **TC-16.1.5.1** `test_attribute_memoized_read` — Read same attribute twice in one frame; assert
    modifier stack evaluated only once (memoized).
    - Input: attribute with 4 modifiers; call `read("hp")` twice; track aggregator invocation count
    - Expected: aggregator called once; both read results equal

13. **TC-16.1.5.2** `test_attribute_invalidate_on_set` — Read attribute; write base value; read
    again. Assert second read produces a fresh aggregation.
    - Input: `read("hp")` → write base 50 → `read("hp")`; track aggregator calls
    - Expected: aggregator called twice (once per read after invalidation)

14. **TC-16.1.6.1** `test_modifier_flat_additive` — Base 10, single flat +5. Assert result is 15.
    - Input: `base = 10.0`, modifiers `[StatModifier { op: FlatAdd, value: 5.0 }]`
    - Expected: `aggregate() == 15.0`

15. **TC-16.1.6.2** `test_modifier_percent_additive` — Base 10, two percent additive +50% +25%.
    Assert result is 10 * (1 + 0.75) = 17.5.
    - Input: modifiers `[PctAdd 0.5, PctAdd 0.25]`
    - Expected: `aggregate() == 17.5`

16. **TC-16.1.6.3** `test_modifier_percent_mul` — Base 10, two multiplicative *1.2 *1.1. Assert
    result is 10 *1.2* 1.1 = 13.2.
    - Input: modifiers `[PctMul 0.2, PctMul 0.1]`
    - Expected: `aggregate() == 13.2` within `f32::EPSILON * 16`

17. **TC-16.1.6.4** `test_modifier_override_priority` — Two overrides, one priority 1 value 50, one
    priority 2 value 99. Assert higher priority wins.
    - Input: modifiers
      `[Override { priority: 1, value: 50.0 }, Override { priority: 2, value: 99.0 }]`
    - Expected: `aggregate() == 99.0`

18. **TC-16.1.6.5** `test_modifier_full_pipeline` — Base 100, flat +10, percent +20%, mult *1.1, no
    overrides. Assert `(100 + 10) * (1 + 0.2) * 1.1 = 145.2`.
    - Input: modifiers `[FlatAdd 10, PctAdd 0.2, PctMul 0.1]`, base 100
    - Expected: `aggregate() == 145.2` within `f32::EPSILON * 16`

19. **TC-16.1.7.1** `test_effect_instant_apply` — Instant effect with magnitude 25 on a meter at
    50. Assert meter becomes 75 and effect is not stored as active.
    - Input: `Effect { kind: Instant, magnitude: 25.0, target: meter_id, .. }`
    - Expected: `meter.current() == 75.0`, `active_effects.len() == 0`

20. **TC-16.1.7.2** `test_effect_duration_expires` — Duration effect with duration 100 ticks.
    Advance 99 ticks (still active), then 1 more (expired).
    - Input: `Effect { kind: Duration { ticks: 100 }, .. }`
    - Expected: after 99 ticks `is_active == true`; after 100 ticks `is_active == false`, expired
      event emitted

21. **TC-16.1.7.3** `test_effect_periodic_ticks` — Periodic effect with period 10 ticks; advance 30
    ticks. Assert three modifier applications.
    - Input: `Effect { kind: Periodic { period: 10, duration: 30 }, magnitude: 5.0, .. }`
    - Expected: target meter received `+5` three times; total delta `+15`

22. **TC-16.1.7.4** `test_effect_infinite_persists` — Infinite effect; advance 10,000 ticks. Assert
    effect remains active and is not auto-removed.
    - Input: `Effect { kind: Infinite, .. }`
    - Expected: after 10,000 ticks `is_active == true`, no `Expired` event emitted

23. **TC-16.1.8.1** `test_stacking_highest_wins` — Apply two effects with magnitudes 20 and 30 and
    `HighestWins` rule on the same target. Assert only the +30 effect contributes.
    - Input: two effects with same `EffectId`, magnitudes 20 and 30, `stacking: HighestWins`
    - Expected: `active_stack.effective_magnitude() == 30.0`, stack length 1

24. **TC-16.1.8.2** `test_stacking_additive` — Apply two stacking effects with magnitudes 20 and
    30. Assert sum 50 applies.
    - Input: same as TC-16.1.8.1 but `stacking: Additive`
    - Expected: `active_stack.effective_magnitude() == 50.0`, stack length 2

25. **TC-16.1.8.3** `test_stacking_non_stacking` — Apply two `NonStacking` effects with the same
    EffectId. Assert second application is rejected.
    - Input: two effects with same id, `stacking: NonStacking`
    - Expected: first applies; second `apply_effect` returns `Err(EffectError::AlreadyActive)`,
      stack length 1

26. **TC-16.1.11.1** `test_condition_and_or_not` — Build `And(Leaf(A), Or(Leaf(B), Not(Leaf(C))))`.
    Evaluate with `A=true, B=false, C=false`. Assert result is true.
    - Input: tree as above; `ConditionContext { vars: {A: true, B: false, C: false} }`
    - Expected: `expr.evaluate(&ctx) == true`

27. **TC-16.1.11.2** `test_condition_leaf_dispatch` — Leaf condition references a codegen'd function
    pointer in the registry. Assert dispatch invokes the registered function exactly once.
    - Input: `ConditionExpr::Leaf(ConditionId::HAS_TAG)`, registry maps id → counted function
    - Expected: function called exactly once; result matches function return value

28. **TC-13.9.1.1** `test_character_stats_from_set` — Spawn a character entity composed from an
    `AttributeSet` schema with `strength`, `agility`, `intellect`. Assert each field is readable and
    reports the schema default.
    - Input: schema defaults `{ strength: 10, agility: 8, intellect: 5 }`
    - Expected: `set.read("strength") == 10`, `agility == 8`, `intellect == 5`

29. **TC-13.9.2.1** `test_hp_mp_stamina_meters_bound` — Bind three meters (HP, MP, stamina) to a
    character entity. Assert each is independently drained/filled and reports its own bounds.
    - Input: HP 0..100, MP 0..50, stamina 0..80; drain HP by 20
    - Expected: HP reads 80; MP and stamina unchanged

30. **TC-13.9.3.1** `test_buff_equipment_mod_stack` — Apply a buff (`FlatAdd +5 strength`) and an
    equipment modifier (`PctMul 1.1 strength`). Base 10. Assert `(10+5)*1.1 == 16.5`.
    - Input: base 10; modifiers `[FlatAdd 5, PctMul 0.1]`
    - Expected: `aggregate() == 16.5` within `f32::EPSILON * 16`

31. **TC-13.10.3.1** `test_dot_hot_effect_periodic` — Apply a DoT effect dealing 5 damage every tick
    for 3 ticks, and a HoT healing 3 every tick for 3 ticks. Assert net delta over 3 ticks is -15
    and +9 respectively on each meter.
    - Input: HP meter at 100; DoT(5, 3 ticks); separate target HP at 50; HoT(3, 3 ticks)
    - Expected: DoT target HP == 85; HoT target HP == 59

32. **TC-13.12.5.1** `test_faction_rep_tier_thresholds` — Build a faction reputation meter in
    `[-100, 100]` with tier thresholds at -50, 0, 50. Advance reputation across each threshold
    upward and assert the correct tier events fire.
    - Input: reputation 0 → 10 → 60 → 110 (clamped to 100)
    - Expected: `Rising` threshold events at 50 and 100 (clamp hit); tier reported as top tier

## Integration Tests

| ID            | Name                              | Req       |
|---------------|-----------------------------------|-----------|
| TC-16.1.I.1   | `test_meter_threshold_to_effect`  | R-16.1.2  |
| TC-16.1.I.2   | `test_effect_vfx_lifecycle`       | R-16.1.9  |
| TC-16.1.I.3   | `test_effect_lifecycle_events`    | R-16.1.12 |
| TC-16.1.I.4   | `test_modifier_remove_on_expire`  | R-16.1.7  |
| TC-16.1.I.5   | `test_condition_gates_effect`     | R-16.1.11 |
| TC-16.1.I.6   | `test_attribute_set_hot_reload`   | R-16.1.4  |
| TC-16.1.I.7   | `test_64_effects_per_entity_tick` | R-16.1.10 |
| TC-16.1.1.I1  | `test_author_meter_primitive`     | US-16.1.1 |
| TC-16.1.2.I1  | `test_author_threshold_event`     | US-16.1.2 |
| TC-16.1.3.I1  | `test_sim_1k_meters_budget`       | US-16.1.3 |
| TC-16.1.4.I1  | `test_author_attribute_schema`    | US-16.1.4 |
| TC-16.1.5.I1  | `test_sim_10k_attr_reads`         | US-16.1.5 |
| TC-16.1.6.I1  | `test_author_modifier_stack`      | US-16.1.6 |
| TC-16.1.7.I1  | `test_author_effect_types`        | US-16.1.7 |
| TC-16.1.8.I1  | `test_author_stacking_rule`       | US-16.1.8 |
| TC-16.1.9.I1  | `test_player_effect_vfx_visible`  | US-16.1.9 |
| TC-16.1.10.I1 | `test_sim_64_effects_entity`      | US-16.1.10 |
| TC-16.1.11.I1 | `test_author_condition_expr`      | US-16.1.11 |
| TC-16.1.12.I1 | `test_author_effect_lifecycle_ev` | US-16.1.12 |

1. **TC-16.1.I.1** `test_meter_threshold_to_effect` — Meter threshold action `ApplyEffect` triggers
   when current crosses threshold. Assert the effect is applied to the entity.
   - Input: meter with rising threshold at 30 and `action: ApplyEffect(LowHealthBuff)`; advance
     current 50 → 25
   - Expected: entity has `ActiveEffect { id: LowHealthBuff }`; one `Applied` event emitted

2. **TC-16.1.I.2** `test_effect_vfx_lifecycle` — Apply an effect with a configured VFX handle.
   Assert VFX entity spawns parented under the target. Remove effect; assert VFX despawns.
   - Input: `Effect { vfx: Some(handle), .. }`; apply, then remove after 5 ticks
   - Expected: after apply `vfx_query.count() == 1`; after remove `vfx_query.count() == 0`

3. **TC-16.1.I.3** `test_effect_lifecycle_events` — Subscribe to `EffectEvent`. Apply, tick, expire
   one effect. Assert four events fire in order: Applied, Ticked, Ticked, Expired.
   - Input: `Effect { kind: Periodic { period: 1, duration: 2 }, .. }`; advance two ticks
   - Expected: events `[Applied, Ticked, Ticked, Expired]` in order, all referencing same entity and
     definition

4. **TC-16.1.I.4** `test_modifier_remove_on_expire` — Apply duration effect that adds +10 to an
   attribute. Wait for expiry. Assert the +10 modifier is removed and attribute returns to baseline.
   - Input: attribute base 50; effect adds `FlatAdd 10` for 10 ticks
   - Expected: during effect `read == 60.0`; after expiry `read == 50.0`; modifier list empty

5. **TC-16.1.I.5** `test_condition_gates_effect` — Apply an effect with gating condition `Leaf(A)`.
   Apply with A=false; assert effect skipped. Apply with A=true; assert applied.
   - Input: `Effect { condition: Some(Leaf(A)), .. }`; two apply attempts with different contexts
   - Expected: first attempt no `Applied` event, no active effect; second attempt one `Applied`
     event, effect active

6. **TC-16.1.I.6** `test_attribute_set_hot_reload` — Modify schema file; trigger reload. Assert
   existing entities update to new defaults for newly added fields and bounded fields clamp.
   - Input: schema v1 `[hp 0..100]`; reload v2 `[hp 0..100, mp 0..50]`; entity hp is 50
   - Expected: after reload entity has `mp` field with default; `hp` unchanged at 50

7. **TC-16.1.I.7** `test_64_effects_per_entity_tick` — Apply 64 distinct effects to one entity; tick
   once. Assert all 64 process correctly and no effect is dropped.
   - Input: 64 effects with mixed types (Duration, Periodic) and unique ids
   - Expected: `active_effects.len() == 64`; modifier stack reflects all 64 contributions

8. **TC-16.1.1.I1** `test_author_meter_primitive` (US-16.1.1) — As a designer, author a "stamina"
   meter with bounds `[0, 100]`, drain 5/s, fill 2/s. Attach to an entity; advance simulation.
   Assert stamina decreases while exerting and increases while resting.
   - Input: editor-authored meter def; exert for 2 s; rest for 2 s
   - Expected: after exert stamina = 90; after rest stamina = 94 (clamped per rule)
9. **TC-16.1.2.I1** `test_author_threshold_event` (US-16.1.2) — As a designer, add a rising
   threshold at 80% on the stamina meter with action `ApplyEffect(Winded)`. Cross threshold in
   gameplay. Assert `Winded` effect is applied.
   - Input: meter crossing 80 upward
   - Expected: `Winded` `ActiveEffect` present; one `ThresholdEvent` emitted
10. **TC-16.1.3.I1** `test_sim_1k_meters_budget` (US-16.1.3) — As a developer, spawn 1,000 meter
    entities; tick the simulation one frame. Assert the meter advance pass meets the 0.5 ms budget.
    - Input: 1k meters with mixed drain/fill rates
    - Expected: measured pass wall time < 0.5 ms
11. **TC-16.1.4.I1** `test_author_attribute_schema` (US-16.1.4) — As a designer, author an
    `AttributeSet` schema in the editor with 6 fields (mixed types and bounds). Spawn an entity;
    read fields. Assert defaults present and bounds enforced.
    - Input: editor-authored schema
    - Expected: all fields readable at defaults; writes outside bounds clamp
12. **TC-16.1.5.I1** `test_sim_10k_attr_reads` (US-16.1.5) — As a developer, perform 10,000 memoized
    attribute reads in one frame. Assert the pass meets the 0.1 ms budget.
    - Input: 10k reads across 100 entities with small modifier stacks
    - Expected: measured wall time < 0.1 ms
13. **TC-16.1.6.I1** `test_author_modifier_stack` (US-16.1.6) — As a designer, add layered modifiers
    to an attribute: FlatAdd, PctAdd, PctMul, Override. Assert the final aggregation follows the
    documented order with the Override at highest priority winning.
    - Input: base 100; mods `[FlatAdd 10, PctAdd 0.2, PctMul 0.1, Override 42]`
    - Expected: aggregate == 42 (override wins at highest priority)
14. **TC-16.1.7.I1** `test_author_effect_types` (US-16.1.7) — As a designer, author four effects
    (Instant, Duration, Periodic, Infinite). Apply each to an entity and verify behavior after 10
    ticks.
    - Input: 4 effects with distinct kinds
    - Expected: Instant applied + not stored; Duration still active; Periodic has 10/period
      applications; Infinite still active
15. **TC-16.1.8.I1** `test_author_stacking_rule` (US-16.1.8) — As a designer, mark an effect with
    `stacking: HighestWins`. Apply twice with different magnitudes. Assert only the highest
    contributes.
    - Input: two effects same id, magnitudes 20 and 30, HighestWins
    - Expected: stack length 1; effective magnitude 30
16. **TC-16.1.9.I1** `test_player_effect_vfx_visible` (US-16.1.9) — As a player, gain a buff with a
    VFX indicator. Assert the VFX entity spawns parented to the player on apply and despawns on
    remove.
    - Input: buff effect with `vfx: Some(handle)`
    - Expected: VFX entity count goes 0 → 1 → 0 across the lifecycle
17. **TC-16.1.10.I1** `test_sim_64_effects_entity` (US-16.1.10) — As a developer, apply 64 active
    effects to one entity; tick simulation. Assert per-entity effect pass meets 0.1 ms budget.
    - Input: 64 effects on one entity
    - Expected: measured wall time < 0.1 ms
18. **TC-16.1.11.I1** `test_author_condition_expr` (US-16.1.11) — As a designer, gate an effect with
    `And(HasTag("frozen"), HpBelow(50))`. Trigger both conditions; assert the effect applies only
    when both are true.
    - Input: two scenarios (both true, one false)
    - Expected: both-true → Applied; one-false → skipped
19. **TC-16.1.12.I1** `test_author_effect_lifecycle_ev` (US-16.1.12) — As a gameplay engineer,
    listen for `EffectEvent` (Applied, Ticked, Expired). Trigger a Periodic effect; assert the event
    sequence is delivered in order.
    - Input: Periodic effect with period 1 and duration 2
    - Expected: events `[Applied, Ticked, Ticked, Expired]` received in order

## Benchmarks

| ID            | Benchmark                          | Target   | Req       |
|---------------|------------------------------------|----------|-----------|
| TC-16.1.B.1   | 1,000 meters advance               | < 0.5 ms | R-16.1.3  |
| TC-16.1.B.2   | 10,000 attribute reads             | < 0.1 ms | R-16.1.5  |
| TC-16.1.B.3   | 64 effects/entity tick             | < 0.1 ms | R-16.1.10 |
| TC-16.1.B.4   | Modifier stack aggregate (8 mods)  | < 0.01 ms| R-16.1.6  |
| TC-16.1.B.5   | Condition tree evaluate (depth 8)  | < 0.001 ms| R-16.1.11|

1. **TC-16.1.B.1** — Spawn 1,000 entities with `Meter` components; run a single advance pass
   including drain, fill, and threshold checks. Wall time end-to-end. Measured with `criterion`.

2. **TC-16.1.B.2** — Build an attribute set with 10 attributes each with 4 modifiers; perform 10,000
   sequential reads with memoization invalidated each iteration. Total wall time.

3. **TC-16.1.B.3** — One entity with 64 active effects; run a single tick processing stacking,
   modifier application, and expiration checks. Wall time per tick.

4. **TC-16.1.B.4** — Single `StatAggregator::aggregate` call over a stack of 8 modifiers (mixed
   FlatAdd, PctAdd, PctMul, Override). Wall time per call.

5. **TC-16.1.B.5** — Evaluate a condition expression tree of depth 8 (mixed And/Or/Not/Leaf nodes)
   against a fresh context. Wall time per evaluation; must use codegen'd function pointers.
