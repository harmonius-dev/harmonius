//! `StepResult` ↔ `NodeStatus` mapping for BT leaf integration (design § adapter).

/// Runtime error payload carried by [`StepResult::Error`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    /// ECS component missing for graph access.
    ComponentNotFound,
    /// Catch-all for other graph failures.
    Other,
}

/// Wait condition attached to [`StepResult::Yield`].
#[derive(Clone, Debug, PartialEq)]
pub enum WaitCondition {
    /// Resume next frame.
    NextFrame,
    /// Resume after N frames.
    Frames(u32),
    /// Resume after a delay in seconds.
    Delay(f32),
    /// Resume when an event id fires.
    Event(u32),
}

/// Result of executing one graph step (scripting side).
#[derive(Clone, Debug, PartialEq)]
pub enum StepResult {
    /// Graph still running without suspending.
    Continue,
    /// Graph finished successfully.
    Complete,
    /// Suspended state machine; resume later per [`WaitCondition`].
    Yield(WaitCondition),
    /// Debug-only breakpoint path; treated as running when enabled.
    Breakpoint(u32),
    /// Recoverable runtime error.
    Error(RuntimeError),
}

/// BT node status (AI behavior side).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeStatus {
    /// Idle (unused for leaf hot path in this crate).
    Idle,
    /// Still running / suspended.
    Running,
    /// Leaf completed successfully.
    Success,
    /// Leaf failed or errored.
    Failure,
}

/// Convert a scripting [`StepResult`] into a BT [`NodeStatus`].
///
/// Mapping matches `docs/design/integration/ai-scripting.md`.
#[must_use]
pub fn step_to_status(result: StepResult) -> NodeStatus {
    match result {
        StepResult::Complete => NodeStatus::Success,
        StepResult::Continue | StepResult::Yield(_) | StepResult::Breakpoint(_) => {
            NodeStatus::Running
        }
        StepResult::Error(_) => NodeStatus::Failure,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-2.4.ADPT.1 — each `StepResult` variant maps per design table.
    #[test]
    fn tc_ir_2_4_adpt_1_step_to_status_mapping() {
        assert_eq!(step_to_status(StepResult::Complete), NodeStatus::Success);
        assert_eq!(step_to_status(StepResult::Continue), NodeStatus::Running);
        assert_eq!(
            step_to_status(StepResult::Yield(WaitCondition::NextFrame)),
            NodeStatus::Running
        );
        assert_eq!(
            step_to_status(StepResult::Breakpoint(7)),
            NodeStatus::Running
        );
        assert_eq!(
            step_to_status(StepResult::Error(RuntimeError::ComponentNotFound)),
            NodeStatus::Failure
        );
    }
}
