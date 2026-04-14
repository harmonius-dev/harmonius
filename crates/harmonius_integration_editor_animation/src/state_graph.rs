//! Deterministic transition walking with explicit cycle breaking (TC-IR-5.3.F4).

use crate::ids::StateId;

/// Directed transition between authored states.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transition {
    /// Source state id.
    pub from: StateId,
    /// Destination state id.
    pub to: StateId,
    /// When true, this edge is eligible during greedy preview walks.
    pub always_true: bool,
}

/// Authoring-time state machine graph (in-memory).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StateGraphDef {
    /// All transitions in arbitrary order; evaluation scans linearly.
    pub transitions: Vec<Transition>,
}

/// Emitted when a greedy walk revisits a state.
pub const WARN_STATE_GRAPH_CYCLE: &str = "state graph cycle detected during transition walk";

/// Result of following always-true transitions until a fixed point or cycle break.
#[derive(Debug, PartialEq)]
pub struct TransitionWalkOutcome {
    /// Final state after the walk, if any step was taken.
    pub final_state: Option<StateId>,
    /// Ordered list of visited states including the start state.
    pub visited: Vec<StateId>,
    /// Warnings mirroring production logging text.
    pub warnings: Vec<&'static str>,
}

/// Follows the first always-true outgoing edge from each state, breaking on cycles.
///
/// This is a conservative policy stub for editor preview — full runtime evaluation
/// lives in the state-machine system.
#[must_use]
pub fn walk_always_true_transitions(
    graph: &StateGraphDef,
    start: StateId,
    max_steps: usize,
) -> TransitionWalkOutcome {
    let mut visited: Vec<StateId> = Vec::new();
    let mut warnings = Vec::new();
    let mut current = start;

    for _ in 0..max_steps {
        if visited.contains(&current) {
            warnings.push(WARN_STATE_GRAPH_CYCLE);
            return TransitionWalkOutcome {
                final_state: Some(current),
                visited,
                warnings,
            };
        }
        visited.push(current);

        let next = graph
            .transitions
            .iter()
            .find(|t| t.from == current && t.always_true)
            .map(|t| t.to);

        let Some(next_state) = next else {
            return TransitionWalkOutcome {
                final_state: Some(current),
                visited,
                warnings,
            };
        };

        current = next_state;
    }

    TransitionWalkOutcome {
        final_state: Some(current),
        visited,
        warnings,
    }
}
