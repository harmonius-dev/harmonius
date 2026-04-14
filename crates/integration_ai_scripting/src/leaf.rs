//! BT leaf adapter for codegen'd graph dispatch (IR-2.4.3).

use crate::adapter::{step_to_status, NodeStatus};
use crate::runtime::{ExecutionContext, FnPtrTable, GraphInstanceState};

/// BT leaf invoking a codegen'd graph function via [`FnPtrTable`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BtGraphLeaf {
    /// Index resolved at compile time.
    pub fn_idx: u32,
}

/// Tick a [`BtGraphLeaf`]: resolve `fn_idx`, invoke graph, map [`StepResult`] → [`NodeStatus`].
///
/// FM-2: when [`FnPtrTable::get`] returns [`None`], logs via `log_error` and returns
/// [`NodeStatus::Failure`].
#[must_use]
pub fn tick_bt_graph_leaf(
    leaf: &BtGraphLeaf,
    table: &FnPtrTable,
    state: &mut GraphInstanceState,
    ctx: &ExecutionContext<'_>,
    entity_id: u64,
    mut log_error: impl FnMut(&str),
) -> NodeStatus {
    let Some(graph_fn) = table.get(leaf.fn_idx) else {
        log_error(&format!(
            "bt_graph_leaf: fn_idx {} out of range for entity {}",
            leaf.fn_idx, entity_id
        ));
        return NodeStatus::Failure;
    };
    let step = graph_fn(state, ctx);
    step_to_status(step)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::{RuntimeError, StepResult, WaitCondition};

    #[test]
    fn leaf_invokes_fn_idx_zero_success() {
        let leaf = BtGraphLeaf { fn_idx: 0 };
        let table = FnPtrTable::new(vec![|_, _| StepResult::Complete]);
        let mut state = GraphInstanceState::default();
        let status = tick_bt_graph_leaf(
            &leaf,
            &table,
            &mut state,
            &ExecutionContext::stub(),
            1,
            |_| {},
        );
        assert_eq!(status, NodeStatus::Success);
    }

    #[test]
    fn leaf_maps_error_to_failure() {
        let leaf = BtGraphLeaf { fn_idx: 0 };
        let table = FnPtrTable::new(vec![|_, _| {
            StepResult::Error(RuntimeError::ComponentNotFound)
        }]);
        let mut state = GraphInstanceState::default();
        let status = tick_bt_graph_leaf(
            &leaf,
            &table,
            &mut state,
            &ExecutionContext::stub(),
            2,
            |_| {},
        );
        assert_eq!(status, NodeStatus::Failure);
    }

    /// TC-IR-2.4.NEG.4 — `fn_idx` out of range returns failure and logs entity + index.
    #[test]
    fn tc_ir_2_4_neg_4_oob_fn_idx_failure() {
        let leaf = BtGraphLeaf {
            fn_idx: u32::MAX,
        };
        let table = FnPtrTable::new(vec![|_, _| StepResult::Complete]);
        let mut state = GraphInstanceState::default();
        let mut logs = Vec::new();
        let status = tick_bt_graph_leaf(
            &leaf,
            &table,
            &mut state,
            &ExecutionContext::stub(),
            42,
            |m| logs.push(m.to_string()),
        );
        assert_eq!(status, NodeStatus::Failure);
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("4294967295"));
        assert!(logs[0].contains("42"));
    }

    #[test]
    fn yield_maps_to_running() {
        let leaf = BtGraphLeaf { fn_idx: 0 };
        let table = FnPtrTable::new(vec![|_, _| StepResult::Yield(WaitCondition::NextFrame)]);
        let mut state = GraphInstanceState::default();
        let status = tick_bt_graph_leaf(
            &leaf,
            &table,
            &mut state,
            &ExecutionContext::stub(),
            3,
            |_| {},
        );
        assert_eq!(status, NodeStatus::Running);
    }
}
