#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Simulation game loop phase ordering for Harmonius.
//!
//! This crate encodes the canonical ordering of simulation primitives inside
//! [`Phase::Simulation`] and the placement of spatial awareness in [`Phase::AiUpdate`], matching
//! `docs/design/simulation/game-loop-phases.md`.

mod change_model;
mod fixed_timestep;
mod phase;
mod runner;
mod sim_set;

pub use change_model::{change_visible_across_sets, change_visible_same_set_followup};
pub use fixed_timestep::FixedTimestep;
pub use phase::Phase;
pub use runner::{run_scheduled_systems, ScheduledSystem, Trace, TraceEvent};
pub use sim_set::{simulation_set_chain, spatial_awareness_phase, SimSet};

/// Returns the canonical [`SimSet`] chain for [`Phase::Simulation`].
#[must_use]
pub const fn configure_simulation_order() -> [SimSet; 5] {
    simulation_set_chain()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_simulation_phase_fixed_timestep() {
        let tick = Duration::from_nanos(16_666_667);
        let mut ft = FixedTimestep::new(tick, 32);
        ft.accumulate(tick.saturating_mul(5));
        let ticks = ft.consume();
        assert_eq!(ticks, 5);
        assert_eq!(ft.total_simulated, tick.saturating_mul(5));
    }

    #[test]
    fn test_sim_set_order_timelines_before_grids() {
        let chain = configure_simulation_order();
        assert!(chain[0] == SimSet::TimelinesAdvance);
        assert!(chain[1] == SimSet::GridsPropagate);
    }

    #[test]
    fn test_sim_set_order_grids_before_event_logs() {
        let chain = configure_simulation_order();
        assert!(SimSet::GridsPropagate.ordinal() < SimSet::EventLogsUpdate.ordinal());
        assert_eq!(chain[1], SimSet::GridsPropagate);
        assert_eq!(chain[2], SimSet::EventLogsUpdate);
    }

    #[test]
    fn test_sim_set_order_event_logs_before_index() {
        let chain = configure_simulation_order();
        assert!(SimSet::EventLogsUpdate.ordinal() < SimSet::SpatialIndexRebuild.ordinal());
        assert_eq!(chain[3], SimSet::SpatialIndexRebuild);
    }

    #[test]
    fn test_sim_set_order_index_before_triggers() {
        let chain = configure_simulation_order();
        assert!(SimSet::SpatialIndexRebuild.ordinal() < SimSet::ThresholdTriggers.ordinal());
        assert_eq!(chain[4], SimSet::ThresholdTriggers);
    }

    #[test]
    fn test_awareness_registered_in_ai_phase() {
        assert_eq!(spatial_awareness_phase(), Phase::AiUpdate);
    }

    #[test]
    fn test_timeline_tick_before_anim_read() {
        let systems = [
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::TimelinesAdvance),
                name: "advance_playback",
                run: |_| {},
            },
            ScheduledSystem {
                phase: Phase::AnimationUpdate,
                sim_set: None,
                name: "read_playback_for_blend",
                run: |_| {},
            },
        ];

        let mut trace = Trace::default();
        run_scheduled_systems(&systems, &mut trace);

        let adv = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("advance_playback"))
            .expect("timeline system ran");
        let read = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("read_playback_for_blend"))
            .expect("animation reader ran");
        assert!(adv < read);
    }

    #[test]
    fn test_grid_propagate_before_event_log_prop() {
        let systems = [
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::GridsPropagate),
                name: "propagate_influence",
                run: |_| {},
            },
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::EventLogsUpdate),
                name: "propagate_neighbors",
                run: |_| {},
            },
        ];

        let mut trace = Trace::default();
        run_scheduled_systems(&systems, &mut trace);

        let g = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("propagate_influence"))
            .expect("grid system ran");
        let e = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("propagate_neighbors"))
            .expect("event log system ran");
        assert!(g < e);
    }

    #[test]
    fn test_event_log_decay_before_trigger_check() {
        let systems = [
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::EventLogsUpdate),
                name: "decay_entries",
                run: |_| {},
            },
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::ThresholdTriggers),
                name: "check_thresholds",
                run: |_| {},
            },
        ];

        let mut trace = Trace::default();
        run_scheduled_systems(&systems, &mut trace);

        let d = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("decay_entries"))
            .expect("decay system ran");
        let t = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("check_thresholds"))
            .expect("threshold system ran");
        assert!(d < t);
    }

    #[test]
    fn test_awareness_reads_rebuilt_index() {
        let systems = [
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::SpatialIndexRebuild),
                name: "rebuild_bvh",
                run: |_| {},
            },
            ScheduledSystem {
                phase: Phase::AiUpdate,
                sim_set: None,
                name: "run_sense_queries",
                run: |_| {},
            },
        ];

        let mut trace = Trace::default();
        run_scheduled_systems(&systems, &mut trace);

        let r = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("rebuild_bvh"))
            .expect("index rebuild ran");
        let s = trace
            .events
            .iter()
            .position(|e| *e == TraceEvent::System("run_sense_queries"))
            .expect("awareness ran");
        assert!(r < s);
    }

    #[test]
    fn test_change_detection_at_set_boundary() {
        assert!(change_visible_across_sets(
            SimSet::TimelinesAdvance,
            SimSet::GridsPropagate
        ));
        assert!(!change_visible_same_set_followup(
            SimSet::TimelinesAdvance,
            SimSet::TimelinesAdvance
        ));
    }

    #[test]
    fn test_half_updated_state_not_visible() {
        use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

        static GRID_A: AtomicU32 = AtomicU32::new(0);
        static GRID_B: AtomicU32 = AtomicU32::new(0);
        static GRID_READY: AtomicBool = AtomicBool::new(false);

        fn reset_grid() {
            GRID_A.store(0, Ordering::Relaxed);
            GRID_B.store(0, Ordering::Relaxed);
            GRID_READY.store(false, Ordering::Relaxed);
        }

        fn producer_a(t: &mut Trace) {
            let _ = t;
            GRID_A.store(1, Ordering::Relaxed);
        }

        fn producer_b(t: &mut Trace) {
            let _ = t;
            GRID_B.store(2, Ordering::Relaxed);
            GRID_READY.store(true, Ordering::Relaxed);
        }

        fn consumer(t: &mut Trace) {
            let _ = t;
            if GRID_READY.load(Ordering::Relaxed) {
                assert_eq!(GRID_A.load(Ordering::Relaxed), 1);
                assert_eq!(GRID_B.load(Ordering::Relaxed), 2);
            }
        }

        reset_grid();

        let systems = [
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::GridsPropagate),
                name: "producer_a",
                run: producer_a,
            },
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::GridsPropagate),
                name: "producer_b",
                run: producer_b,
            },
            ScheduledSystem {
                phase: Phase::Simulation,
                sim_set: Some(SimSet::EventLogsUpdate),
                name: "consumer",
                run: consumer,
            },
        ];

        let mut trace = Trace::default();
        run_scheduled_systems(&systems, &mut trace);
        let _ = trace;
    }
}
