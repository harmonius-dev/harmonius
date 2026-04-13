# Data Systems Composition -- Test Cases

Companion to [composition.md](composition.md).

Test case IDs use `TC-16.5.Z.N` format. Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID           | Name                                     | Req      |
|--------------|------------------------------------------|----------|
| TC-16.5.1.1  | `test_definition_asset_bind_meter`       | R-16.5.1 |
| TC-16.5.1.2  | `test_definition_asset_bind_attrset`     | R-16.5.1 |
| TC-16.5.1.3  | `test_definition_asset_bind_container`   | R-16.5.1 |
| TC-16.5.1.4  | `test_definition_asset_bind_graph`       | R-16.5.1 |
| TC-16.5.1.5  | `test_definition_asset_unbind_removes`   | R-16.5.1 |
| TC-16.5.1.6  | `test_definition_asset_version_mismatch` | R-16.5.1 |
| TC-16.5.1.7  | `test_definition_asset_idempotent_bind`  | R-16.5.1 |
| TC-16.5.1.8  | `test_bind_error_invalid_handle`         | R-16.5.1 |
| TC-16.5.6.1  | `test_recipe_handle_tracks_definitions`  | R-16.5.6 |
| TC-16.5.6.2  | `test_recipe_uninstall_clears_entities`  | R-16.5.6 |

1. **TC-16.5.1.1** `test_definition_asset_bind_meter` -- `MeterDefinition::bind` attaches a `Meter`
   component with min/max/default from the definition.
   - Input: `MeterDefinition { min: 0.0, max: 100.0, default: 50.0 }`, fresh entity
   - Expected: entity has `Meter { value: 50.0, min: 0.0, max: 100.0 }`
2. **TC-16.5.1.2** `test_definition_asset_bind_attrset` -- `AttributeSetDefinition::bind` attaches
   an `AttributeSet` with all declared attributes at their default values.
   - Input: definition with attrs `[strength=10, agility=8]`
   - Expected: entity has `AttributeSet` containing both keys with given values
3. **TC-16.5.1.3** `test_definition_asset_bind_container` -- `ContainerDefinition::bind` attaches a
   `Container` with the declared capacity and slot layout.
   - Input: grid 4x8, capacity 32
   - Expected: `Container { layout: Grid(4,8), capacity: 32, items: [] }`
4. **TC-16.5.1.4** `test_definition_asset_bind_graph` -- `DirectedGraphDefinition::bind` attaches a
   graph instance with the authored topology.
   - Input: graph with 5 nodes and 4 edges
   - Expected: `DirectedGraphInstance` with same counts
5. **TC-16.5.1.5** `test_definition_asset_unbind_removes` -- `unbind` removes the component.
   - Input: entity with bound component; call `unbind`
   - Expected: component no longer on entity
6. **TC-16.5.1.6** `test_definition_asset_version_mismatch` -- bind with a handle whose version does
   not match the stored asset fails.
   - Input: stale handle
   - Expected: `Err(BindError::VersionMismatch)`
7. **TC-16.5.1.7** `test_definition_asset_idempotent_bind` -- binding twice is a no-op the second
   time.
   - Input: bind twice to same entity
   - Expected: only one component, state equal to single bind
8. **TC-16.5.1.8** `test_bind_error_invalid_handle` -- freed handle returns
   `BindError::InvalidHandle`.
9. **TC-16.5.6.1** `test_recipe_handle_tracks_definitions` -- installing a recipe records every
   bound definition in `RecipeHandle::definitions`.
10. **TC-16.5.6.2** `test_recipe_uninstall_clears_entities` -- uninstalling a recipe unbinds all
    definitions in reverse order.

## Integration Tests

| ID           | Name                                       | Req      |
|--------------|--------------------------------------------|----------|
| TC-16.5.2.1  | `test_quest_recipe_end_to_end`             | R-16.5.2 |
| TC-16.5.2.2  | `test_ability_recipe_end_to_end`           | R-16.5.2 |
| TC-16.5.2.3  | `test_inventory_recipe_end_to_end`         | R-16.5.2 |
| TC-16.5.2.4  | `test_crafting_recipe_end_to_end`          | R-16.5.2 |
| TC-16.5.2.5  | `test_stealth_recipe_end_to_end`           | R-16.5.2 |
| TC-16.5.2.6  | `test_schedule_recipe_end_to_end`          | R-16.5.2 |
| TC-16.5.3.1  | `test_event_log_to_graph_advance`          | R-16.5.3 |
| TC-16.5.3.2  | `test_container_to_attribute_propagation`  | R-16.5.3 |
| TC-16.5.3.3  | `test_grid_to_effect_trigger`              | R-16.5.3 |
| TC-16.5.3.4  | `test_timeline_to_event_log_fire`          | R-16.5.3 |
| TC-16.5.4.1  | `test_two_peer_determinism_quest`          | R-16.5.4 |
| TC-16.5.4.2  | `test_two_peer_determinism_ability`        | R-16.5.4 |
| TC-16.5.4.3  | `test_two_peer_determinism_combined`       | R-16.5.4 |
| TC-16.5.7.1  | `test_save_load_quest_state`               | R-16.5.7 |
| TC-16.5.7.2  | `test_save_load_inventory_with_effects`    | R-16.5.7 |
| TC-16.5.7.3  | `test_save_load_crafting_progress`         | R-16.5.7 |

1. **TC-16.5.2.1** `test_quest_recipe_end_to_end` -- Install a quest with 3 objectives. Fire kill
   events. Assert node advances and reward buff applies.
   - Input: quest graph [A -> B -> C], kill event each node
   - Expected: final node `C` active, reward `Effect` applied to player
2. **TC-16.5.2.2** `test_ability_recipe_end_to_end` -- Install an ability that consumes 20 mana,
   targets nearest enemy, applies damage effect.
   - Input: caster with 100 mana, target within range
   - Expected: caster mana = 80, target damage effect applied, event log entry
3. **TC-16.5.2.3** `test_inventory_recipe_end_to_end` -- Equip a sword from a grid container. Assert
   stat modifier flows to attribute set.
   - Input: inventory [sword row], equip to slot
   - Expected: `AttributeSet::attack` increased by sword delta
4. **TC-16.5.2.4** `test_crafting_recipe_end_to_end` -- Craft a potion from 2 herbs + 1 vial. Assert
   ingredients consumed and output placed.
   - Input: container with 2 herbs, 1 vial; craft recipe
   - Expected: 0 herbs, 0 vials, 1 potion in container
5. **TC-16.5.2.5** `test_stealth_recipe_end_to_end` -- Move player through a noise grid cell. NPC
   awareness fires detection event.
   - Input: grid cell with noise=1.0, NPC hearing sensor
   - Expected: `StealthEvent::Detected` logged
6. **TC-16.5.2.6** `test_schedule_recipe_end_to_end` -- NPC timeline fires keyframe at 09:00 setting
   destination.
   - Input: timeline with keyframe at tick=540 (9 * 60s)
   - Expected: NPC `Destination` component set to target entity
7. **TC-16.5.3.1** `test_event_log_to_graph_advance` -- Event appended to log advances a graph that
   references it via `ConditionExpr`.
8. **TC-16.5.3.2** `test_container_to_attribute_propagation` -- Socket occupant change emits
   modifier event, attribute re-aggregates within one frame.
9. **TC-16.5.3.3** `test_grid_to_effect_trigger` -- Grid cell crosses threshold, `Effect` applies.
10. **TC-16.5.3.4** `test_timeline_to_event_log_fire` -- Timeline keyframe appends event log entry.
11. **TC-16.5.4.1** `test_two_peer_determinism_quest` -- Two peers process same quest inputs for 600
    frames. Assert state hashes equal.
12. **TC-16.5.4.2** `test_two_peer_determinism_ability` -- Same for ability activations.
13. **TC-16.5.4.3** `test_two_peer_determinism_combined` -- Two peers run all six recipes for 1800
    frames with random inputs from a shared seed. Assert state hashes equal every frame.
14. **TC-16.5.7.1** `test_save_load_quest_state` -- Save partway through a quest, load, assert same
    node active.
15. **TC-16.5.7.2** `test_save_load_inventory_with_effects` -- Save inventory with equipped effects,
    load, assert effects still applied.
16. **TC-16.5.7.3** `test_save_load_crafting_progress` -- Save mid-craft, load, assert progress
    timeline resumes at saved position.

## Benchmarks

| ID            | Name                                     | Target                  |
|---------------|------------------------------------------|-------------------------|
| TC-16.5.5.B1  | `bench_256_character_scene`              | composed < 3.0 ms/frame |
| TC-16.5.5.B2  | `bench_10k_item_inventory_stack_op`      | < 0.05 ms per op        |
| TC-16.5.5.B3  | `bench_quest_graph_advance_32_graphs`    | < 0.2 ms/frame          |
| TC-16.5.5.B4  | `bench_ability_64_activations`           | < 0.4 ms/frame          |
| TC-16.5.5.B5  | `bench_effect_tick_1024_active`          | < 0.3 ms/frame          |
| TC-16.5.5.B6  | `bench_event_log_2048_appends`           | < 0.2 ms/frame          |
| TC-16.5.5.B7  | `bench_schedule_128_timelines`           | < 0.2 ms/frame          |
| TC-16.5.5.B8  | `bench_spatial_awareness_256_sensors`    | < 0.3 ms/frame          |
| TC-16.5.5.B9  | `bench_grid_128x128_propagate`           | < 0.3 ms/frame          |
| TC-16.5.5.B10 | `bench_composition_determinism_1800f`    | < 0.5 ms overhead       |

1. **TC-16.5.5.B1** -- Construct a 256-character RPG scene with the mix from the performance profile
   table. Measure total per-frame time. Pass if < 3.0 ms.
2. **TC-16.5.5.B2** -- Stack/unstack in a 10,000-slot flat container. Pass if < 0.05 ms per op.
3. **TC-16.5.5.B3** -- 32 simultaneously advancing quest graphs with 8 nodes each.
4. **TC-16.5.5.B4** -- 64 concurrent ability activations through the full recipe.
5. **TC-16.5.5.B5** -- 1024 active effects ticking one frame.
6. **TC-16.5.5.B6** -- 2048 event log appends per frame across all recipes.
7. **TC-16.5.5.B7** -- 128 NPC schedule timelines tick and fire keyframes.
8. **TC-16.5.5.B8** -- 256 spatial awareness sensors query targets.
9. **TC-16.5.5.B9** -- 128x128 noise grid propagates one frame of diffusion.
10. **TC-16.5.5.B10** -- Deterministic run of 1800 frames; overhead of determinism guards (sorted
    iteration, seeded RNG) must be under 0.5 ms total across the run vs. a non-deterministic
    baseline.
