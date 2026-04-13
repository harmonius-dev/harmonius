# Game Loop Phases -- Test Cases

Companion to [game-loop-phases.md](game-loop-phases.md).

Test case IDs use `TC-1.1.Z.N` and `TC-17.Y.Z.N` format.

## Unit Tests

| ID              | Name                                            | Req        |
|-----------------|-------------------------------------------------|------------|
| TC-1.1.2.1      | `test_simulation_phase_fixed_timestep`          | R-1.1.2    |
| TC-1.1.2.2      | `test_sim_set_order_timelines_before_grids`     | R-1.1.2    |
| TC-1.1.2.3      | `test_sim_set_order_grids_before_event_logs`    | R-1.1.2    |
| TC-1.1.2.4      | `test_sim_set_order_event_logs_before_index`    | R-1.1.2    |
| TC-1.1.2.5      | `test_sim_set_order_index_before_triggers`      | R-1.1.2    |
| TC-1.1.2.6      | `test_awareness_registered_in_ai_phase`         | R-1.1.2    |
| TC-17.4.1.1     | `test_timeline_tick_before_anim_read`           | R-17.4.1   |
| TC-17.2.1.1     | `test_grid_propagate_before_event_log_prop`     | R-17.2.1   |
| TC-17.1.1.1     | `test_event_log_decay_before_trigger_check`     | R-17.1.1   |
| TC-17.3.1.1     | `test_awareness_reads_rebuilt_index`            | R-17.3.1   |
| TC-1.1.22.1     | `test_change_detection_at_set_boundary`         | R-1.1.22   |
| TC-1.1.22.2     | `test_half_updated_state_not_visible`           | R-1.1.22   |

1. **TC-1.1.2.1** `test_simulation_phase_fixed_timestep` -- Running N fixed ticks advances
   simulation state by exactly `N * tick_duration`.
2. **TC-1.1.2.2** `test_sim_set_order_timelines_before_grids` -- Registered order puts
   `TimelinesAdvance` before `GridsPropagate`.
3. **TC-1.1.2.3** `test_sim_set_order_grids_before_event_logs` -- `GridsPropagate` before
   `EventLogsUpdate`.
4. **TC-1.1.2.4** `test_sim_set_order_event_logs_before_index` -- `EventLogsUpdate` before
   `SpatialIndexRebuild`.
5. **TC-1.1.2.5** `test_sim_set_order_index_before_triggers` -- `SpatialIndexRebuild` before
   `ThresholdTriggers`.
6. **TC-1.1.2.6** `test_awareness_registered_in_ai_phase` -- Spatial awareness systems belong to
   Phase 4 (AiUpdate), not Phase 3.
7. **TC-17.4.1.1** `test_timeline_tick_before_anim_read` -- Animation systems see the new
   `PlaybackState` values produced by the same tick.
8. **TC-17.2.1.1** `test_grid_propagate_before_event_log_prop` -- Event log propagation reads the
   grid state updated in the same tick.
9. **TC-17.1.1.1** `test_event_log_decay_before_trigger_check` -- Threshold triggers observe the
   decayed log state.
10. **TC-17.3.1.1** `test_awareness_reads_rebuilt_index` -- Sense queries hit the fresh BVH entries.
11. **TC-1.1.22.1** `test_change_detection_at_set_boundary` -- `Changed<T>` is true for the consumer
    system only, not for the producer's own follow-up system.
12. **TC-1.1.22.2** `test_half_updated_state_not_visible` -- A consumer in a later set cannot
    observe a partially-updated primitive.

## Integration Tests

| ID              | Name                                              | Req        |
|-----------------|---------------------------------------------------|------------|
| TC-1.1.2.I1     | `test_full_frame_runs_all_sim_sets`               | R-1.1.2    |
| TC-1.1.2.I2     | `test_primitives_interact_in_one_tick`            | R-1.1.2    |
| TC-1.1.2.I3     | `test_editor_phase_inspector_single_step`         | R-1.1.2    |
| TC-1.1.22.I1    | `test_change_detection_across_phases`             | R-1.1.22   |

1. **TC-1.1.2.I1** `test_full_frame_runs_all_sim_sets` -- One simulated frame executes every sim set
   exactly once.
2. **TC-1.1.2.I2** `test_primitives_interact_in_one_tick` -- Scripted scenario exercising all four
   primitives in a single tick produces expected final state.
3. **TC-1.1.2.I3** `test_editor_phase_inspector_single_step` -- `CompiledFrame::step_phase` advances
   one set per call and preserves frame index correctness.
4. **TC-1.1.22.I1** `test_change_detection_across_phases` -- Animation reads
   `Changed<PlaybackState>` true for one frame after timeline advance.

## Benchmarks

| ID              | Name                                       | Target       |
|-----------------|--------------------------------------------|--------------|
| TC-1.1.2.B1     | `bench_simulation_phase_total_budget`      | < 4 ms       |
| TC-1.1.2.B2     | `bench_sim_set_timelines`                  | < 0.5 ms     |
| TC-1.1.2.B3     | `bench_sim_set_grids`                      | < 1 ms       |
| TC-1.1.2.B4     | `bench_sim_set_event_logs`                 | < 0.5 ms     |
| TC-1.1.2.B5     | `bench_sim_set_spatial_index_rebuild`      | < 1 ms       |
| TC-1.1.2.B6     | `bench_sim_set_triggers`                   | < 0.2 ms     |

1. **TC-1.1.2.B1** -- Total Phase 3 (Simulation) budget for a typical scene. < 4 ms.
2. **TC-1.1.2.B2** -- Timelines set with 1k playbacks. < 0.5 ms.
3. **TC-1.1.2.B3** -- Grids set with 256x256 propagation. < 1 ms.
4. **TC-1.1.2.B4** -- Event log set with 1k logs. < 0.5 ms.
5. **TC-1.1.2.B5** -- Spatial index rebuild with 10k dynamic transforms. < 1 ms.
6. **TC-1.1.2.B6** -- Threshold trigger evaluation across 1k logs. < 0.2 ms.
