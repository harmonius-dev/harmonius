//! Minimal graph program stepping for integration tests.

use std::sync::Arc;

use harmonius_directed_graphs::GraphTraversalState;

use crate::error::GraphError;
use crate::NodeId;

/// Sentinel step index meaning traversal has not started (`u32::MAX`).
pub const GRAPH_STEP_NOT_STARTED: u32 = u32::MAX;

/// Maps traversal state to the explicit codegen state machine step index.
pub fn traversal_to_step(traversal: &GraphTraversalState) -> u32 {
    match traversal.current_node {
        Some(node) => node.0,
        None => GRAPH_STEP_NOT_STARTED,
    }
}

/// Immutable compiled graph metadata used by the stepping harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphProgram {
    /// Hot-reload generation tag.
    pub version: u32,
    /// First executed node when traversal restarts.
    pub entry: NodeId,
}

/// Result of one synchronous graph step.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepOutcome {
    /// Advanced within the same tick.
    Continue,
    /// Yielded until `resume_at`.
    YieldUntil {
        /// Tick when execution may resume.
        resume_at: u64,
    },
    /// Finished successfully.
    Complete,
    /// Failed with a structured error.
    Error(GraphError),
}

/// Advances traversal for a trivial two-state entry/complete program.
pub fn step_graph(
    program: &GraphProgram,
    mut traversal: GraphTraversalState,
    tick: u64,
) -> (GraphTraversalState, StepOutcome) {
    if traversal.current_node.is_none() {
        traversal.current_node = Some(program.entry);
        traversal.started_at = tick;
        return (traversal, StepOutcome::Continue);
    }
    traversal.current_node = None;
    (traversal, StepOutcome::Complete)
}

/// Owns program identity for stale-version detection.
#[derive(Debug)]
pub struct GraphInstance {
    /// Live program blob.
    pub program: Arc<GraphProgram>,
    /// Version bound at the last clean reset.
    pub bound_version: u32,
    /// ECS traversal mirror.
    pub traversal: GraphTraversalState,
    /// Codegen state machine step mirror.
    pub sm_step: u32,
}

impl GraphInstance {
    /// Binds the instance to `program.version` and clears traversal.
    pub fn new(program: Arc<GraphProgram>) -> Self {
        let v = program.version;
        Self {
            program,
            bound_version: v,
            traversal: GraphTraversalState::default(),
            sm_step: GRAPH_STEP_NOT_STARTED,
        }
    }

    /// Swaps the program without rebinding the expected version (hot reload).
    pub fn swap_program(&mut self, program: Arc<GraphProgram>) {
        self.program = program;
    }

    /// Runs one step, surfacing stale-program errors before stepping.
    pub fn advance(&mut self, tick: u64) -> StepOutcome {
        if self.bound_version != self.program.version {
            return StepOutcome::Error(GraphError::StaleProgram {
                expected: self.bound_version,
                found: self.program.version,
            });
        }
        let (tr, out) = step_graph(&self.program, self.traversal.clone(), tick);
        self.traversal = tr;
        out
    }

    /// Editor/runtime recovery path after a stale-program error.
    pub fn reset_after_stale(&mut self, tick: u64) {
        self.bound_version = self.program.version;
        self.traversal.current_node = None;
        self.traversal.started_at = tick;
        self.sm_step = GRAPH_STEP_NOT_STARTED;
    }
}

/// ECS-facing helper that mirrors the documented reset semantics.
#[derive(Debug, Default)]
pub struct GraphExecutionSystem;

impl GraphExecutionSystem {
    /// Clears traversal and codegen state after a hot reload mismatch.
    pub fn reset_after_stale(instance: &mut GraphInstance, tick: u64) {
        instance.reset_after_stale(tick);
    }
}
