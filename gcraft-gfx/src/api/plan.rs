use derive_more::{Display, Error, From};

use crate::RenderGraph;

/// An efficient plan to execute a render graph with minimal overhead.
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    graph: RenderGraph,
}

impl ExecutionPlan {
    /// Create a new execution plan from a render graph.
    pub fn new(graph: RenderGraph) -> Result<Self, ExecutionPlanError> {
        Ok(Self { graph })
    }

    /// Get the render graph that this execution plan is based on.
    pub fn graph(&self) -> &RenderGraph {
        &self.graph
    }
}

// An error that can occur when preparing an execution plan.
#[derive(Debug, Display, Error, From)]
pub enum ExecutionPlanError {}
