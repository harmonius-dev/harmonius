use derive_more::{Display, Error, From};

/// Runs render graph execution plans by passing them to the corresponding render thread.
#[derive(Debug, Default, Clone)]
pub struct PlanExecutor {}

// An error that can occur when running a render graph execution plan.
#[derive(Debug, Display, Error, From)]
pub enum PlanExecutorError {}
