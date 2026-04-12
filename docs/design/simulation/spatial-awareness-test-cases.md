# Spatial Awareness — Test Cases

Companion to [spatial-awareness.md](spatial-awareness.md). All test cases follow the `TC-X.Y.Z.N`
format. Requirements traced per the design Requirements Trace section.

## Unit Tests

| ID            | Test Name                              | Req       |
|---------------|----------------------------------------|-----------|
| TC-7.6.1.1    | `test_sphere_sense_candidates`         | R-7.6.2   |
| TC-7.6.1.2    | `test_cone_sense_fov_inside`           | R-7.6.1   |
| TC-7.6.1.3    | `test_cone_sense_fov_outside`          | R-7.6.1   |
| TC-7.6.1.4    | `test_box_sense_candidates`            | R-7.6.7   |
| TC-7.6.1.5    | `test_cylinder_sense_candidates`       | R-7.6.7   |
| TC-7.6.1.6    | `test_circle2d_sense_candidates`       | R-7.6.7   |
| TC-7.6.1.7    | `test_cone2d_sense_fov_inside`         | R-7.6.1   |
| TC-7.6.1.8    | `test_rect2d_sense_candidates`         | R-7.6.7   |
| TC-7.6.2.1    | `test_falloff_linear`                  | R-7.6.1   |
| TC-7.6.2.2    | `test_falloff_inverse_linear`          | R-7.6.1   |
| TC-7.6.2.3    | `test_falloff_quadratic`               | R-7.6.1   |
| TC-7.6.2.4    | `test_falloff_custom_curve`            | R-7.6.7   |
| TC-7.6.3.1    | `test_scoring_distance_only`           | R-7.6.1   |
| TC-7.6.3.2    | `test_scoring_angle_weight`            | R-7.6.1   |
| TC-7.6.3.3    | `test_scoring_occlusion_penalty`       | R-7.6.1   |
| TC-7.6.3.4    | `test_scoring_modifier_bonus`          | R-7.6.7   |
| TC-7.6.3.5    | `test_score_clamp_zero_one`            | R-7.6.1   |
| TC-13.18.1.1  | `test_awareness_unaware_to_suspicious` | R-13.18.2 |
| TC-13.18.1.2  | `test_awareness_suspicious_to_alert`   | R-13.18.2 |
| TC-13.18.1.3  | `test_awareness_alert_to_tracking`     | R-13.18.2 |
| TC-13.18.1.4  | `test_awareness_tracking_to_lost`      | R-13.18.2 |
| TC-13.18.1.5  | `test_awareness_lost_to_unaware`       | R-13.18.2 |
| TC-13.18.1.6  | `test_awareness_lost_to_suspicious`    | R-13.18.2 |
| TC-7.6.4.1    | `test_awareness_decay_reduces_score`   | R-7.6.6   |
| TC-7.6.4.2    | `test_awareness_decay_below_threshold` | R-7.6.6   |
| TC-7.6.5.1    | `test_awareness_max_targets_eviction`  | R-7.6.5   |
| TC-13.18.2.1  | `test_highest_threat_returns_max`      | R-13.18.1 |
| TC-13.18.2.2  | `test_entities_at_level_filter`        | R-13.18.2 |
| TC-13.18.2.3  | `test_is_aware_of_above_unaware`       | R-13.18.2 |
| TC-13.11.1.1  | `test_selection_raycast_nearest`       | R-13.11.1 |
| TC-13.11.1.2  | `test_selection_box_all_inside`        | R-13.11.1 |
| TC-13.11.1.3  | `test_selection_sphere_radius`         | R-13.11.1 |
| TC-13.11.1.4  | `test_selection_nearest_n_count`       | R-13.11.1 |
| TC-13.11.1.5  | `test_selection_filter_excludes`       | R-13.11.1 |
| TC-13.11.1.6  | `test_selection_sorted_by_distance`    | R-13.11.1 |
| TC-13.11.2.1  | `test_selection_boxselect2d`           | R-13.11.2 |
| TC-13.11.2.2  | `test_selection_circleselect2d`        | R-13.11.2 |
| TC-7.6.6.1    | `test_deterministic_sort_entity_tiebreak` | R-7.6.1 |
| TC-7.6.6.2    | `test_fixed_update_tick_determinism`   | R-7.6.1   |
| TC-1.9.1.1    | `test_shared_bvh_used_by_sense`        | R-1.9.1   |
| TC-1.9.4.1    | `test_unified_query_api`               | R-1.9.4   |
| TC-1.9.9.1    | `test_ai_perception_integration`       | R-1.9.9   |
| TC-13.18.3.1  | `test_noise_stimulus_distracts`        | R-13.18.3 |
| TC-17.3.1.1   | `test_sense_definition_primitive`      | R-17.3.1  |
| TC-17.3.2.1   | `test_2d_sense_shapes_all`             | R-17.3.2  |
| TC-17.3.3.1   | `test_awareness_state_machine_5`       | R-17.3.3  |
| TC-17.3.4.1   | `test_transition_event_emitted`        | R-17.3.4  |
| TC-17.3.5.1   | `test_100_queries_1000_targets_2ms`    | R-17.3.5  |
| TC-17.3.6.1   | `test_selection_queries_50_half_ms`    | R-17.3.6  |
| TC-17.3.7.1   | `test_gpu_sense_eval_1m`               | R-17.3.7  |
| TC-17.3.8.1   | `test_awareness_drives_indicator`      | R-17.3.8  |
| TC-17.3.9.1   | `test_gizmo_sense_volume_render`       | R-17.3.9  |
| TC-17.3.10.1  | `test_one_frame_stimulus_latency`      | R-17.3.10 |

### Unit Test Details

1. **TC-7.6.1.1** -- Sphere sense radius 10; 3 targets inside, 2 outside; verify 3 results.
2. **TC-7.6.1.2** -- Cone half-angle 90°; target at 45°; verify non-zero score.
3. **TC-7.6.1.3** -- Target at 100° from cone forward; verify zero score.
4. **TC-7.6.1.4** -- Box half-extents (5, 5, 5); targets inside and outside; verify correct
   inclusion.
5. **TC-7.6.1.5** -- Cylinder radius 5, height 10; target above height; verify excluded.
6. **TC-7.6.1.6** -- `Circle2D` radius 10; 3 targets inside, 2 outside; verify 3 results using 2D
   BVH.
7. **TC-7.6.1.7** -- `Cone2D` half-angle 45°; target at 30°; verify non-zero score.
8. **TC-7.6.1.8** -- `Rect2D` half-extents (5, 5); targets inside and outside; verify correct
   inclusion.
9. **TC-7.6.2.1** -- Linear falloff; target at half range; verify score is 0.5.
10. **TC-7.6.2.2** -- Inverse linear; target at half range; verify score is 0.5.
11. **TC-7.6.2.3** -- Quadratic falloff; target at half range; verify score is 0.25.
12. **TC-7.6.2.4** -- Custom curve asset; verify score matches curve sample at distance 0.5.
13. **TC-7.6.3.1** -- `distance_weight` 1.0, others 0.0; verify `final_score` equals distance
    factor.
14. **TC-7.6.3.2** -- `angle_weight` 1.0; target at 45° of 90° cone; verify score is 0.5.
15. **TC-7.6.3.3** -- Occluded target, `occlusion_penalty` 0.8; verify score reduced by 0.8.
16. **TC-7.6.3.4** -- `modifier_bonus` 0.2; verify `final_score` increased by 0.2.
17. **TC-7.6.3.5** -- Weights sum to 1.5; verify clamped to 1.0. Negative sum; verify 0.0.
18. **TC-13.18.1.1** -- Score exceeds `suspicious_threshold`; verify level changes to Suspicious.
19. **TC-13.18.1.2** -- Score exceeds `alert_threshold`; verify transition to Alert.
20. **TC-13.18.1.3** -- Score exceeds `tracking_threshold`; verify transition to Tracking.
21. **TC-13.18.1.4** -- No stimulus for `lost_timeout_ticks`; verify transition to Lost.
22. **TC-13.18.1.5** -- Lost entry times out; verify removal or revert to Unaware.
23. **TC-13.18.1.6** -- New stimulus while Lost; verify transition to Suspicious.
24. **TC-7.6.4.1** -- No stimulus; advance one FixedUpdate tick; verify score reduced by
    `decay_rate`.
25. **TC-7.6.4.2** -- Decay drops score below `suspicious_threshold`; verify demotion to Unaware.
26. **TC-7.6.5.1** -- Exceed `max_targets_per_entity`; verify oldest entry evicted.
27. **TC-13.18.2.1** -- Three entries with scores 0.3, 0.8, 0.5; verify `highest_threat` returns 0.8
    entry.
28. **TC-13.18.2.2** -- Mixed awareness levels; verify `entities_at_level` returns correct subset.
29. **TC-13.18.2.3** -- Entry at Suspicious; verify `is_aware_of` returns true. No entry; verify
    false.
30. **TC-13.11.1.1** -- Ray hits 3 entities; verify nearest returned first.
31. **TC-13.11.1.2** -- 5 entities inside box, 3 outside; verify 5 results.
32. **TC-13.11.1.3** -- Sphere radius 10; verify only entities within 10 returned.
33. **TC-13.11.1.4** -- `NearestN` count 3; 10 entities in range; verify 3 closest returned.
34. **TC-13.11.1.5** -- Filter predicate rejects 2 of 5 candidates; verify 3 results.
35. **TC-13.11.1.6** -- Verify results sorted ascending by distance.
36. **TC-13.11.2.1** -- `BoxSelect2D` min (-5,-5), max (5,5); 3 targets inside, 2 outside; verify 3
    results.
37. **TC-13.11.2.2** -- `CircleSelect2D` center (0,0), radius 10; verify correct 2D radius
    inclusion.
38. **TC-7.6.6.1** -- Two results with equal `final_score`; verify sorted by ascending `Entity` ID
    (deterministic tiebreaker).
39. **TC-7.6.6.2** -- Run `update_awareness` with the same inputs in two FixedUpdate ticks at the
    same `current_tick`; verify identical output (tick-determinism).
40. **TC-1.9.1.1** -- Spawn 500 targets indexed by the shared BVH (F-1.9.1); run a sphere sense
    query. Verify the candidate enumeration matches `SpatialIndex::query_sphere` output exactly
    (same shared structure, no side index).
    - Input: 500 targets inserted via `SpatialIndex`; sense sphere radius 20
    - Expected: `query_sense` candidate set equals `SpatialIndex::query_sphere` candidate set
41. **TC-1.9.4.1** -- Exercise the unified query API: run `raycast`, `query_sphere`, `query_box`,
    and `query_frustum` against the same BVH containing 200 entities. Each call must use the same
    traversal and return results sorted by distance when requested.
    - Input: single shared BVH with 200 entities; issue four query types
    - Expected: each query returns the brute-force expected set; distance-sorted calls are sorted
42. **TC-1.9.9.1** -- An AI entity runs `query_sense` (sight) and the returned ranked list becomes
    the input to an AI perception decision. Assert the AI system can consume `AwarenessEntry`
    records directly from the shared index with zero bridge layer.
    - Input: 1 AI source, 10 targets, sight cone
    - Expected: AI behavior system reads `AwarenessState` populated in the same FixedUpdate tick
43. **TC-13.18.3.1** -- Emit a noise stimulus at position P with radius 15; AI within radius has
    awareness score raised. Assert `AwarenessEntry` has non-zero score and the AI transitions to at
    least Suspicious.
    - Input: 1 AI with sight+hearing; noise event at distance 10 from AI
    - Expected: AI `AwarenessState` contains an entry with `final_score >= suspicious_threshold`
44. **TC-17.3.1.1** -- Construct
    `SenseDefinition { shape: Cone, range: 20.0, falloff: Linear, tags: enemy_tag }`; use it to
    query sense results. Assert every field is honored (shape used, range enforced, falloff applied,
    tags filter candidates).
    - Input: above sense definition; 5 matching + 5 non-matching tagged targets
    - Expected: only the 5 matching targets returned; scores match linear falloff at their ranges
45. **TC-17.3.2.1** -- For each 2D sense shape (`Circle2D`, `Cone2D`, `Rect2D`) with a source entity
    holding `Transform2D`, issue a sense query. Assert the 2D BVH is queried (not the 3D BVH) and
    the result set matches a brute-force 2D inclusion test.
    - Input: `Transform2D` source; 20 2D targets; one query per shape
    - Expected: each query returns exactly the brute-force inclusion set; no 3D BVH access
46. **TC-17.3.3.1** -- Drive an awareness state through all 5 levels
    (Unaware→Suspicious→Alert→Tracking→Lost) by manipulating per-tick scores. Assert each transition
    happens at its configured threshold.
    - Input: scripted score sequence crossing each threshold
    - Expected: state observed at each of the 5 levels in order
47. **TC-17.3.4.1** -- Cause an awareness level change; observe
    `EventReader<AwarenessTransitionEvent>`. Assert exactly one event per change with correct
    `from`, `to`, source and target entities.
    - Input: one transition Suspicious→Alert
    - Expected: 1 event emitted; `event.from == Suspicious`, `event.to == Alert`
48. **TC-17.3.5.1** -- Spawn 100 sources each with a sense, 1,000 targets; run one FixedUpdate.
    Measure total wall time for the sense evaluation system.
    - Input: 100 sources, 1,000 targets, one tick
    - Expected: total wall time < 2 ms on reference hardware
49. **TC-17.3.6.1** -- Issue 50 concurrent `SelectionQuery` events in a single Update frame
    (raycast, box, nearest-N mix). Measure wall time of selection handling.
    - Input: 50 queued selection queries, 500 entities indexed
    - Expected: total wall time < 0.5 ms
<!-- THIN: design section lacks detail -->
50. **TC-17.3.7.1** -- Dispatch GPU sense evaluation for 1,000,000 targets against 1,000 sources via
    `GpuSenseEvalPass`; read back top-N per source. Assert result count and top-N ordering match a
    CPU brute-force reference for a sampled subset.
    - Input: 1M targets, 1k sources, top_n=16 per source
    - Expected: readback produces 1k×16 results; sampled 10 sources match CPU reference
<!-- THIN: design section lacks detail -->
51. **TC-17.3.8.1** -- On `AwarenessTransitionEvent` from Unaware→Alert, assert the 3D indicator
    component on the target NPC updates (e.g., `DetectionIcon::Alert`) within one frame.
    - Input: one transition event
    - Expected: next frame has `DetectionIcon::Alert` on the NPC entity
<!-- THIN: design section lacks detail -->
52. **TC-17.3.9.1** -- Enable debug gizmo overlay with render layer L set. Spawn sense volumes on
    layers L and M. Assert only the layer-L volume is submitted to the debug renderer.
    - Input: 2 sense volumes, render layer filter = L
    - Expected: debug draw queue contains exactly 1 volume (the L one)
53. **TC-17.3.10.1** -- Inject a stimulus at frame F; assert the resulting awareness state update
    and any `AwarenessTransitionEvent` are observable at the end of frame F or the start of frame
    F+1 (≤ 1 frame latency).
    - Input: stimulus written in FixedUpdate of frame F
    - Expected: `AwarenessState` change visible to a system reading in frame F+1 latest

## Integration Tests

| ID           | Test Name                               | Req       |
|--------------|-----------------------------------------|-----------|
| TC-SA.I.1    | `test_100_sources_1000_targets_budget`  | NFR-SA.1  |
| TC-SA.I.2    | `test_50_selections_budget`             | NFR-SA.2  |
| TC-SA.I.3    | `test_stimulus_to_awareness_latency`    | NFR-SA.3  |
| TC-SA.I.4    | `test_full_awareness_lifecycle`         | R-13.18.2 |
| TC-SA.I.5    | `test_selection_with_awareness`         | R-13.11.1 |
| TC-SA.I.6    | `test_fixed_update_selection_handoff`   | R-7.6.1   |
| TC-SA.I.7    | `test_2d_sense_with_transform2d`        | R-7.6.7   |
| TC-17.3.1.I1 | `test_author_sense_definition`          | US-17.3.1 |
| TC-17.3.2.I1 | `test_author_2d_sense`                  | US-17.3.2 |
| TC-17.3.3.I1 | `test_author_awareness_fsm`             | US-17.3.3 |
| TC-17.3.4.I1 | `test_react_to_transition_event`        | US-17.3.4 |
| TC-17.3.5.I1 | `test_scale_100_queries_1000_targets`   | US-17.3.5 |
| TC-17.3.6.I1 | `test_player_select_50_queries`         | US-17.3.6 |
| TC-17.3.7.I1 | `test_gpu_1m_dormant_npcs`              | US-17.3.7 |
| TC-17.3.8.I1 | `test_indicator_ui_from_awareness`      | US-17.3.8 |
| TC-17.3.9.I1 | `test_editor_gizmo_sense_debug`         | US-17.3.9 |
| TC-17.3.10.I1 | `test_responsive_stealth_reaction`     | US-17.3.10 |

1. **TC-SA.I.1** -- Spawn 100 sources and 1000 targets; measure FixedUpdate frame time; verify < 2
   ms (NFR-SA.1).
2. **TC-SA.I.2** -- Execute 50 concurrent `SelectionQuery` events in one Update frame; verify < 0.5
   ms (NFR-SA.2).
3. **TC-SA.I.3** -- Inject sense result; verify `AwarenessTransitionEvent` fires within 1 frame
   (NFR-SA.3).
4. **TC-SA.I.4** -- Entity progresses through all 5 awareness levels and back to Unaware via decay
   and timeout.
5. **TC-SA.I.5** -- Selection query picks entity; verify result overlaps entities in awareness
   state.
6. **TC-SA.I.6** -- `AwarenessState` written in FixedUpdate; verify AI reads updated state in same
   or next Update frame. `SelectionResult` available in same Update frame as query.
7. **TC-SA.I.7** -- Entity with `Transform2D`; `SenseShape::Circle2D` sense; verify 2D BVH queried
   and correct candidates returned.
8. **TC-17.3.1.I1** `test_author_sense_definition` (US-17.3.1) — As a designer, define a named
   `SenseDefinition` ("sight") in the visual editor: cone shape, 20m range, linear falloff,
   `enemy_tag` filter. Spawn an AI entity using it and assert queries return only tagged enemies
   inside the cone.
   - Input: editor-authored sense; 10 entities (5 tagged enemies, 5 untagged)
   - Expected: query returns exactly the tagged enemies inside the cone
9. **TC-17.3.2.I1** `test_author_2d_sense` (US-17.3.2) — As a 2D stealth designer, attach a `Cone2D`
   sense to a guard entity with `Transform2D`. Assert detection of 2D player entities within the 2D
   cone and that 3D targets are never considered.
   - Input: top-down 2D scene; guard with `Cone2D`; 3 player 2D targets
   - Expected: cone detection set matches brute-force 2D inclusion; no 3D fallback
10. **TC-17.3.3.I1** `test_author_awareness_fsm` (US-17.3.3) — As a designer, configure a 5-level
    awareness state machine with thresholds via data. Drive a full scenario that progresses an NPC
    through all levels and back to Unaware.
    - Input: data-driven `AwarenessConfig`; scripted stimuli
    - Expected: observed transitions match the authored thresholds
11. **TC-17.3.4.I1** `test_react_to_transition_event` (US-17.3.4) — As a gameplay engineer, write a
    system that listens for `AwarenessTransitionEvent` and plays an "alert" VFX on transition to
    Alert. Trigger a transition; assert the VFX system received the event.
    - Input: VFX system subscribed; one Unaware→Alert transition
    - Expected: VFX system receives exactly one event and triggers one VFX spawn
12. **TC-17.3.5.I1** `test_scale_100_queries_1000_targets` (US-17.3.5) — As a developer, verify the
    engine hits the performance target for 100 concurrent sense queries against 1,000 targets
    end-to-end on reference hardware.
    - Input: 100 sources, 1,000 targets, shared BVH
    - Expected: end-to-end FixedUpdate tick contribution < 2 ms
13. **TC-17.3.6.I1** `test_player_select_50_queries` (US-17.3.6) — As an RTS player, issue 50
    concurrent selection queries (raycast picks, box select, nearest-N) in one Update frame and
    measure end-to-end selection latency.
    - Input: 50 queued selection queries; 500 entities indexed
    - Expected: end-to-end selection wall time < 0.5 ms
14. **TC-17.3.7.I1** `test_gpu_1m_dormant_npcs` (US-17.3.7) — <!-- THIN: design section lacks detail
    --> As a sandbox designer, spawn 1M dormant NPCs handled by the GPU path; assert gameplay FPS is
    unaffected and the GPU pass completes per frame inside its compute budget.
    - Input: 1M targets in GPU buffers; 1k active sources
    - Expected: GPU pass < 1 ms; no main-thread stall; gameplay loop continues
15. **TC-17.3.8.I1** `test_indicator_ui_from_awareness` (US-17.3.8) — <!-- THIN: design section
    lacks detail --> As a player, observe that above-NPC detection icons update when an NPC
    transitions to Suspicious or Alert within one frame of the transition.
    - Input: one AI entity; scripted transition
    - Expected: detection icon component reflects new awareness level next frame
16. **TC-17.3.9.I1** `test_editor_gizmo_sense_debug` (US-17.3.9) — <!-- THIN: design section lacks
    detail --> As a designer in the editor, toggle the sense debug gizmo layer on; assert all
    `SenseDefinition` volumes on that layer are drawn and hidden when the layer is toggled off.
    - Input: 3 sense volumes on debug layer D
    - Expected: layer on → 3 gizmos drawn; layer off → 0 gizmos drawn
17. **TC-17.3.10.I1** `test_responsive_stealth_reaction` (US-17.3.10) — As a stealth player, trigger
    a loud noise in range of a guard; assert the guard reacts within one frame (awareness transition
    to at least Suspicious observed on frame F+1).
    - Input: noise stimulus at frame F
    - Expected: guard `AwarenessState` at frame F+1 is ≥ Suspicious

## Benchmarks

| ID        | Benchmark                      | Target       | Req      |
|-----------|--------------------------------|--------------|----------|
| TC-SA.B.1 | 100 sources, 1000 targets      | < 2 ms/frame | NFR-SA.1 |
| TC-SA.B.2 | 50 selection queries           | < 0.5 ms     | NFR-SA.2 |
| TC-SA.B.3 | Single sense eval (4 factors)  | < 10 µs      | NFR-SA.1 |
| TC-SA.B.4 | Awareness transition check     | < 1 µs       | NFR-SA.1 |
| TC-SA.B.5 | Selection raycast              | < 50 µs      | NFR-SA.2 |
| TC-SA.B.6 | Awareness decay (100 entries)  | < 20 µs      | NFR-SA.1 |
