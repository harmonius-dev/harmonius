//! Ignored placeholders for integration and benchmark rows in
//! `docs/design/simulation/game-loop-phases-test-cases.md` (deferred until harnesses exist).

#[test]
#[ignore = "TC-1.1.2.I1 deferred: needs full ECS frame harness"]
fn test_full_frame_runs_all_sim_sets() {}

#[test]
#[ignore = "TC-1.1.2.I2 deferred: needs scripted multi-primitive scenario harness"]
fn test_primitives_interact_in_one_tick() {}

#[test]
#[ignore = "TC-1.1.2.I3 deferred: needs editor CompiledFrame::step_phase harness"]
fn test_editor_phase_inspector_single_step() {}

#[test]
#[ignore = "TC-1.1.22.I1 deferred: needs ECS Changed<T> integration harness"]
fn test_change_detection_across_phases() {}

#[test]
#[ignore = "TC-1.1.2.B1 deferred: needs Criterion or engine bench harness"]
fn bench_simulation_phase_total_budget() {}

#[test]
#[ignore = "TC-1.1.2.B2 deferred: needs Criterion or engine bench harness"]
fn bench_sim_set_timelines() {}

#[test]
#[ignore = "TC-1.1.2.B3 deferred: needs Criterion or engine bench harness"]
fn bench_sim_set_grids() {}

#[test]
#[ignore = "TC-1.1.2.B4 deferred: needs Criterion or engine bench harness"]
fn bench_sim_set_event_logs() {}

#[test]
#[ignore = "TC-1.1.2.B5 deferred: needs Criterion or engine bench harness"]
fn bench_sim_set_spatial_index_rebuild() {}

#[test]
#[ignore = "TC-1.1.2.B6 deferred: needs Criterion or engine bench harness"]
fn bench_sim_set_triggers() {}
