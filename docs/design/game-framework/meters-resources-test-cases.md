# Meters and Attribute Systems -- Test Cases

Companion test cases for [meters-resources.md](meters-resources.md).

## Unit Tests

### TC-13.1.6.1 Meter From Definition

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |
| 2 | R-13.1.6    |

1. **#1** -- `Meter::from_definition(&def)` with default 100.0, min 0.0, max 200.0
   - **Expected:** `current_value == 100.0`, `previous_value == 100.0`, `modifier_stack` empty
2. **#2** -- `Meter::from_definition(&def)` with default 0.0
   - **Expected:** `current_value == 0.0`, `is_empty(&def) == true`

### TC-13.1.6.2 Meter Apply Delta Positive

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Meter at 50.0, `apply_delta(30.0, &def)` with max 100.0
   - **Expected:** `current_value == 80.0`

### TC-13.1.6.3 Meter Apply Delta Clamp Max

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Meter at 90.0, `apply_delta(20.0, &def)` with max 100.0
   - **Expected:** `current_value == 100.0`, `is_full(&def) == true`

### TC-13.1.6.4 Meter Apply Delta Clamp Min

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Meter at 10.0, `apply_delta(-50.0, &def)` with min 0.0
   - **Expected:** `current_value == 0.0`, `is_empty(&def) == true`

### TC-13.1.6.5 Meter Apply Delta Negative

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Meter at 80.0, `apply_delta(-25.0, &def)` with min 0.0
   - **Expected:** `current_value == 55.0`

### TC-13.1.6.6 Meter Fraction Boundaries

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |
| 2 | R-13.1.6    |
| 3 | R-13.1.6    |

1. **#1** -- Meter at 0.0, range 0.0..100.0
   - **Expected:** `fraction(&def) == 0.0`
2. **#2** -- Meter at 50.0, range 0.0..100.0
   - **Expected:** `fraction(&def) == 0.5`
3. **#3** -- Meter at 100.0, range 0.0..100.0
   - **Expected:** `fraction(&def) == 1.0`

### TC-13.1.6.7 Meter Set Value Override

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Meter at 50.0, `set_value(75.0, &def)`
   - **Expected:** `current_value == 75.0`

### TC-13.14.6d.1 Threshold Rising Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** -- Threshold at 80.0, `Rising`; previous 70.0, current 85.0
   - **Expected:** `ThresholdCrossed` emitted with `direction == Rising`

### TC-13.14.6d.2 Threshold Falling Detection

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** -- Threshold at 25.0, `Falling`; previous 30.0, current 20.0
   - **Expected:** `ThresholdCrossed` emitted with `direction == Falling`

### TC-13.14.6d.3 Threshold Either Direction

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |
| 2 | R-13.14.6d  |

1. **#1** -- Threshold at 50.0, `Either`; previous 40.0, current 60.0
   - **Expected:** `ThresholdCrossed` emitted, `direction == Either`
2. **#2** -- Threshold at 50.0, `Either`; previous 60.0, current 40.0
   - **Expected:** `ThresholdCrossed` emitted, `direction == Either`

### TC-13.14.6d.4 Threshold Not Crossed

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** -- Threshold at 50.0, `Falling`; previous 60.0, current 55.0
   - **Expected:** No `ThresholdCrossed` emitted

### TC-13.14.6d.5 Threshold Action Apply Effect

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** -- Threshold at 0.0, `Falling`, action `ApplyEffect`, effect_ref set
   - **Expected:** `ThresholdCrossed.action == ApplyEffect`, `effect_ref == Some(row)`

### TC-13.14.6d.6 Threshold Action Remove Effect

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** -- Threshold at 0.0, `Rising`, action `RemoveEffect`, effect_ref set
   - **Expected:** `ThresholdCrossed.action == RemoveEffect`, `effect_ref == Some(row)`

### TC-13.1.6.8 Modifier Flat Stacking

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Base drain 1.0/s, two `Flat` modifiers +0.5 and +0.3
   - **Expected:** Effective drain == 1.8/s

### TC-13.1.6.9 Modifier Percent Stacking

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Base drain 2.0/s, `PercentAdd` +50%
   - **Expected:** Effective drain == 3.0/s

### TC-13.1.6.10 Modifier Override

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Base drain 2.0/s, two `Flat` +1.0 each, one `Override` 5.0
   - **Expected:** Effective drain == 5.0/s (override wins)

### TC-13.1.6.11 Modifier Timed Expiry

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Timed modifier with `total_ticks = 3`, tick 3 times
   - **Expected:** Modifier removed after third tick, `remaining_ticks == 0`

### TC-13.1.6.12 Modifier Permanent No Expiry

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Permanent modifier, tick 1000 times
   - **Expected:** Modifier still present

### TC-13.1.6.13 MeterSet Insert And Get

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |
| 2 | R-13.1.6    |

1. **#1** -- Insert meter with `definition_id = 1`
   - **Expected:** `get(1) == Some(&meter)`
2. **#2** -- Query missing `definition_id = 99`
   - **Expected:** `get(99) == None`

### TC-13.1.6.14 MeterSet Remove

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Insert meter id 1, then `remove(MeterDefinitionId(1))`
   - **Expected:** Returns `Some(meter)`, subsequent `get(1) == None`

### TC-13.1.6.15 MeterSet Iter

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Insert 3 meters, call `iter()`
   - **Expected:** Iterator yields exactly 3 items

### TC-13.14.6a.1 Meter Drain Per Tick

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6a  |

1. **#1** -- Hunger meter at 100.0, `drain_rate = 0.5/s`, dt = 1.0s
   - **Expected:** `current_value == 99.5`

### TC-13.14.6c.1 Meter Fill Per Tick

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6c  |

1. **#1** -- Stamina meter at 50.0, `fill_rate = 3.0/s`, dt = 1.0s
   - **Expected:** `current_value == 53.0`

### TC-13.14.6b.1 Meter Drain When Empty

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6b  |

1. **#1** -- Temperature at 0.0, `drain_rate = 1.0/s`, `drain_when_empty = true`, dt = 1.0s
   - **Expected:** `current_value == -1.0` (below min)

### TC-13.14.6b.2 Meter Fill When Full

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6b  |

1. **#1** -- Meter at 100.0, `fill_rate = 2.0/s`, `fill_when_full = true`, max 100.0, dt = 1.0s
   - **Expected:** `current_value == 102.0` (above max)

### TC-13.8.1.1 AttributeValue From Definition

| # | Requirement |
|---|-------------|
| 1 | R-13.8.1    |

1. **#1** -- `AttributeValue::from_definition(&def)` with default 0.5, min 0.0, max 1.0
   - **Expected:** `base == 0.5`, `current == 0.5`, `modifier_stack` empty

### TC-13.8.1.2 AttributeValue Evaluate

| # | Requirement |
|---|-------------|
| 1 | R-13.8.1    |

1. **#1** -- Base 10.0, `Flat` +5.0, min 0.0, max 100.0
   - **Expected:** `evaluate()` returns `true`, `current == 15.0`

### TC-13.8.1.3 AttributeValue Evaluate No Change

| # | Requirement |
|---|-------------|
| 1 | R-13.8.1    |

1. **#1** -- Base 10.0, no modifiers, current already 10.0
   - **Expected:** `evaluate()` returns `false`

### TC-13.8.1.4 AttributeValue Set Base

| # | Requirement |
|---|-------------|
| 1 | R-13.8.1    |

1. **#1** -- `set_base(20.0)` then `evaluate()`
   - **Expected:** `base == 20.0`, `current == 20.0` (no modifiers)

### TC-13.8.3.1 AttributeSet From Schema

| # | Requirement |
|---|-------------|
| 1 | R-13.8.3    |

1. **#1** -- Schema with 5 attributes (body morphs)
   - **Expected:** `values.len() == 5`, all at defaults

### TC-13.8.3.2 AttributeSet Get By Dense Index

| # | Requirement |
|---|-------------|
| 1 | R-13.8.3    |
| 2 | R-13.8.3    |

1. **#1** -- `get(0)` on 5-attribute set
   - **Expected:** `Some(&first_attr)`
2. **#2** -- `get(99)` on 5-attribute set
   - **Expected:** `None`

### TC-13.8.3.3 AttributeSet Get By Id

| # | Requirement |
|---|-------------|
| 1 | R-13.8.3    |

1. **#1** -- `get_by_id(shoulder_width_id, &schema)`
   - **Expected:** Returns the attribute at the correct dense index

### TC-13.8.3.4 AttributeSet Evaluate All

| # | Requirement |
|---|-------------|
| 1 | R-13.8.3    |

1. **#1** -- 5 attributes, modify 2 with `Flat` +0.1
   - **Expected:** `evaluate_all()` returns vec with exactly 2 changed indices

### TC-13.1.6.16 MeterChanged Event Fields

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Meter changes from 80.0 to 70.0, range 0.0..100.0
   - **Expected:**
     `MeterChanged { old_value: 80.0, new_value: 70.0, min_value: 0.0, max_value: 100.0 }`

### TC-13.14.6d.7 ThresholdCrossed Event Fields

| # | Requirement |
|---|-------------|
| 1 | R-13.14.6d  |

1. **#1** -- Meter crosses threshold at 25.0, `Falling`, action `FireEvent`
   - **Expected:** `ThresholdCrossed` with `threshold_value: 25.0`, `direction: Falling`,
     `action: FireEvent`, `effect_ref: None`

### TC-13.8.1.5 AttributeChanged Event Fields

| # | Requirement |
|---|-------------|
| 1 | R-13.8.1    |

1. **#1** -- Attribute at index 2 changes from 0.5 to 0.7
   - **Expected:** `AttributeChanged { index: 2, old_value: 0.5, new_value: 0.7 }`

### TC-13.16.2a.1 Ammo Meter Decrement

| # | Requirement |
|---|-------------|
| 1 | R-13.16.2a  |

1. **#1** -- Ammo meter at 30, `apply_delta(-1.0)`
   - **Expected:** `current_value == 29.0`

### TC-13.12.10.1 Durability Meter Zero Threshold

| # | Requirement |
|---|-------------|
| 1 | R-13.12.10  |

1. **#1** -- Durability at 1.0, `apply_delta(-1.0)`, threshold at 0.0, `Falling`, `ApplyEffect`
   - **Expected:** `ThresholdCrossed` with `action == ApplyEffect` ("Broken")

### TC-13.12.5.1 Reputation Meter Tier Thresholds

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** -- Rep meter at 190.0, `apply_delta(15.0)`, threshold at 200.0, `Rising`, `FireEvent`
   - **Expected:** `ThresholdCrossed` emitted (tier boundary reached)

### TC-13.19.1.1 NPC Affinity Meter Bidirectional

| # | Requirement |
|---|-------------|
| 1 | R-13.19.1   |

1. **#1** -- Affinity meter for NPC pair (A, B), `apply_delta(10.0)`
   - **Expected:** `current_value` increases by 10.0, `MeterChanged` emitted

### TC-13.19.2.1 Personality Attr Modifier

| # | Requirement |
|---|-------------|
| 1 | R-13.19.2   |

1. **#1** -- Personality `agreeableness` base 0.5, `Flat` modifier -0.3 (from "Corrupted" debuff)
   - **Expected:** `evaluate()` returns `true`, `current == 0.2`

### TC-13.8.2.1 Preset Blending Reads Attrs

| # | Requirement |
|---|-------------|
| 1 | R-13.8.2    |

1. **#1** -- Blend between preset A (height 0.9) and preset B (height 1.1), weight 0.5
   - **Expected:** `base == 1.0` after blend

### TC-13.12.7.1 Enhancement Meter Consume

| # | Requirement |
|---|-------------|
| 1 | R-13.12.7   |

1. **#1** -- Enhancement meter at 50.0, ability cost 20.0, `apply_delta(-20.0)`
   - **Expected:** `current_value == 30.0`

### TC-13.14.3.1 Structural Integrity Meter

| # | Requirement |
|---|-------------|
| 1 | R-13.14.3   |

1. **#1** -- Building integrity at 100.0, `apply_delta(-40.0)`
   - **Expected:** `current_value == 60.0`, `MeterChanged` emitted

### TC-13.1.6.17 Modifier Source Despawn Cleanup

| # | Requirement |
|---|-------------|
| 1 | R-13.1.6    |

1. **#1** -- Modifier with `source = entity_42`; despawn `entity_42`, run cleanup system
   - **Expected:** Modifier removed from stack

## Integration Tests

### TC-13.1.6.I1 Effect Modifies Meter

| # | Requirement |
|---|-------------|
| 1 | F-13.1.6    |

1. **#1** -- `GameplayEffect` with `MeterModifyPayload { meter_id, op: Flat(-10) }` applied to
   entity with health meter at 100.0
   - **Expected:** After `meter_tick_system`, `current_value == 90.0`, `MeterChanged` emitted

### TC-13.14.6d.I1 Threshold Applies Effect

| # | Requirement |
|---|-------------|
| 1 | F-13.14.6d  |

1. **#1** -- Hunger meter crosses 20% threshold (`Falling`, `ApplyEffect`, "Starving" effect)
   - **Expected:** `ThresholdCrossed` emitted, `ThresholdHandler` applies "Starving" `ActiveEffect`
     to entity

### TC-13.1.5.I1 Ability Cost Checks Meter

| # | Requirement |
|---|-------------|
| 1 | F-13.1.5    |

1. **#1** -- Ability requires 30.0 mana, meter at 25.0
   - **Expected:** Activation rejected (insufficient cost)
2. **#2** -- Ability requires 30.0 mana, meter at 50.0
   - **Expected:** Activation succeeds, meter reduced to 20.0

### TC-13.8.1.I1 Attribute Drives Blend Shape

| # | Requirement |
|---|-------------|
| 1 | F-13.8.1    |

1. **#1** -- Modify `muscle_mass` attribute from 0.3 to 0.8 via effect
   - **Expected:** `AttributeChanged` emitted, blend shape driver updates morph target weight to 0.8

### TC-13.1.6.I2 Save Load Preserves Meters

| # | Requirement |
|---|-------------|
| 1 | F-13.1.6    |

1. **#1** -- Entity with 4 meters (health, mana, stamina, hunger) at various values; serialize,
   despawn, deserialize
   - **Expected:** All 4 meters restored with correct `current_value` and active modifiers

### TC-13.8.3.I1 Save Load Preserves Attributes

| # | Requirement |
|---|-------------|
| 1 | F-13.8.3    |

1. **#1** -- Entity with body morph attribute set (7 attributes) at non-default values; serialize,
   despawn, deserialize
   - **Expected:** All 7 base values restored; modifiers recomputed from active effects

### TC-13.1.6.I3 Network Sync Meter

| # | Requirement |
|---|-------------|
| 1 | F-13.1.6    |

1. **#1** -- Server modifies replicated health meter from 100.0 to 70.0
   - **Expected:** Client receives delta update, local meter `current_value == 70.0`

### TC-13.1.6.I4 Modifier Source Despawn

| # | Requirement |
|---|-------------|
| 1 | F-13.1.6    |

1. **#1** -- Entity A applies modifier to entity B's meter; despawn entity A
   - **Expected:** `meter_modifier_cleanup_system` removes the modifier; meter re-evaluates without
     it

### TC-13.14.6d.I2 Vital Debuff Cascade

| # | Requirement |
|---|-------------|
| 1 | F-13.14.6d  |

1. **#1** -- Hunger falls below 20% applying "Starving" debuff; "Starving" modifies stamina drain
   rate
   - **Expected:** Stamina drain increases, stamina meter drains faster in subsequent ticks

### TC-13.1.7.I1 Damage Pipeline Writes Health

| # | Requirement |
|---|-------------|
| 1 | F-13.1.7    |

1. **#1** -- `DamageRequest` of 40.0 physical on entity with health 100.0, physical resistance 0.25
   - **Expected:** Health meter reduced by 30.0 (40 * 0.75), `MeterChanged` emitted

### TC-13.19.2.I1 Emotion Meter Natural Decay

| # | Requirement |
|---|-------------|
| 1 | F-13.19.2   |

1. **#1** -- NPC anger meter set to 0.9 via effect, drain_rate 0.03/s, tick 10 seconds
   - **Expected:** `current_value` approximately 0.6, decayed naturally toward baseline

### TC-13.8.3.I2 Attribute Cleanup On Despawn

| # | Requirement |
|---|-------------|
| 1 | F-13.8.3    |

1. **#1** -- Entity A modifies entity B's resistance attribute; despawn entity A
   - **Expected:** `attribute_modifier_cleanup_system` removes modifier; attribute re-evaluates

## Benchmarks

### TC-NFR-METER.1.B1 Meter Tick Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 1,000 meters, 1 tick | Wall time | < 0.5 ms | NFR-METER.1 |

### TC-NFR-ATTR.1.B1 Attribute Read Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 10,000 attribute reads | Wall time | < 0.1 ms | NFR-ATTR.1 |

### TC-NFR-METER.2.B1 Modifier Stack Evaluation

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | 64 modifiers in stack | Per-eval time | < 0.01 ms | NFR-METER.2 |

### TC-NFR-METER.3.B1 Threshold Check Throughput

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|------|
| 1 | 100 threshold checks | Total time | < 0.1 ms | NFR-METER.3 |

### TC-NFR-METER.1.B2 MeterSet Clone

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Clone MeterSet with 8 meters | Clone time | < 0.005 ms | NFR-METER.1 |

### TC-NFR-ATTR.2.B1 AttributeSet Clone

| # | Scenario | Metric | Target | Req |
|---|----------|--------|--------|-----|
| 1 | Clone AttributeSet, 32 attrs | Clone time | < 0.01 ms | NFR-ATTR.2 |
