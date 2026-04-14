//! Deterministic single-threaded frame runner for ordering tests.

use crate::{Phase, SimSet};

/// Records phase and simulation set transitions for assertions.
#[derive(Default, Debug)]
pub struct Trace {
    /// Ordered scheduling steps observed during a frame run.
    pub events: Vec<TraceEvent>,
}

/// One scheduling step in the trace log.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceEvent {
    /// Entered a top-level phase.
    Phase(Phase),
    /// Entered a simulation subset.
    SimSet(SimSet),
    /// Ran a named system.
    System(&'static str),
}

/// A system scheduled into the frame pipeline.
#[derive(Clone, Copy)]
pub struct ScheduledSystem {
    /// Owning phase.
    pub phase: Phase,
    /// Simulation subset when [`Self::phase`] is [`Phase::Simulation`].
    ///
    /// For [`Phase::Simulation`], this must be [`Some`] with the correct [`SimSet`]. Systems with
    /// [`None`] here are not invoked during simulation by [`run_scheduled_systems`].
    pub sim_set: Option<SimSet>,
    /// Stable label for traces.
    pub name: &'static str,
    /// Callback invoked in schedule order.
    pub run: fn(&mut Trace),
}

/// Runs [`ScheduledSystem`] entries in canonical phase order, then simulation set order.
pub fn run_scheduled_systems(systems: &[ScheduledSystem], trace: &mut Trace) {
    for &phase in Phase::default_pipeline() {
        trace.events.push(TraceEvent::Phase(phase));

        if phase == Phase::Simulation {
            for set in crate::simulation_set_chain() {
                trace.events.push(TraceEvent::SimSet(set));
                run_systems_in_set(systems, phase, Some(set), trace);
            }
        } else {
            run_systems_in_set(systems, phase, None, trace);
        }
    }
}

fn run_systems_in_set(
    systems: &[ScheduledSystem],
    phase: Phase,
    set: Option<SimSet>,
    trace: &mut Trace,
) {
    for s in systems {
        if s.phase == phase && s.sim_set == set {
            trace.events.push(TraceEvent::System(s.name));
            (s.run)(trace);
        }
    }
}
