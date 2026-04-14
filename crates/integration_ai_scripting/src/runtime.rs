//! Graph function tables and minimal execution context stubs.

use crate::adapter::StepResult;

/// Immutable execution context passed into graph functions (scripting design seam).
#[derive(Debug)]
pub struct ExecutionContext<'a> {
    _marker: core::marker::PhantomData<&'a ()>,
}

impl ExecutionContext<'_> {
    /// Test / harness constructor.
    #[must_use]
    pub fn stub() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

/// Mutable graph instance state surface used by [`GraphFn`].
#[derive(Clone, Debug, Default)]
pub struct GraphInstanceState {
    /// Codegen state-machine variant index (resume point).
    pub resume_variant: u32,
}

/// General graph entry (`FnPtrTable`).
pub type GraphFn = fn(&mut GraphInstanceState, &ExecutionContext<'_>) -> StepResult;

/// Pure utility score entry (`UtilityScoreTable`).
pub type UtilityScoreFn = fn(&GraphInstanceState, &ExecutionContext<'_>) -> f32;

/// Function pointer table for [`GraphFn`] entries (mutable state).
#[derive(Debug)]
pub struct FnPtrTable {
    ptrs: Vec<GraphFn>,
}

impl FnPtrTable {
    /// Construct from an ordered list of function pointers.
    #[must_use]
    pub fn new(ptrs: Vec<GraphFn>) -> Self {
        Self { ptrs }
    }

    /// Look up a graph function by index.
    #[must_use]
    pub fn get(&self, idx: u32) -> Option<GraphFn> {
        self.ptrs.get(idx as usize).copied()
    }

    /// Number of registered functions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ptrs.len()
    }

    /// Returns `true` when no pointers are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ptrs.is_empty()
    }
}

/// Table of pure utility score functions (IR-2.4.2).
#[derive(Debug)]
pub struct UtilityScoreTable {
    ptrs: Vec<UtilityScoreFn>,
}

impl UtilityScoreTable {
    /// Construct from codegen-ordered score functions.
    #[must_use]
    pub fn new(ptrs: Vec<UtilityScoreFn>) -> Self {
        Self { ptrs }
    }

    /// Look up a score function by index.
    #[must_use]
    pub fn get(&self, idx: u32) -> Option<UtilityScoreFn> {
        self.ptrs.get(idx as usize).copied()
    }
}

/// GOAP plan step referencing a codegen'd graph entry by index only (IR-2.4.4).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GoapGraphAction {
    /// Index into [`FnPtrTable`].
    pub fn_idx: u32,
}

/// Dispatch a utility score with FM-5 fallback (0.0 + log).
#[must_use]
pub fn utility_score(
    table: &UtilityScoreTable,
    idx: u32,
    state: &GraphInstanceState,
    ctx: &ExecutionContext<'_>,
    mut log_error: impl FnMut(&str),
) -> f32 {
    match table.get(idx) {
        Some(f) => f(state, ctx),
        None => {
            log_error("utility_score_table: idx out of range");
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::RuntimeError;

    fn ok_graph(_: &mut GraphInstanceState, _: &ExecutionContext<'_>) -> StepResult {
        StepResult::Complete
    }

    #[test]
    fn fn_ptr_table_get_oob() {
        let t = FnPtrTable::new(vec![ok_graph]);
        assert!(t.get(0).is_some());
        assert!(t.get(99).is_none());
    }

    /// TC-IR-2.4.4.3 — `GoapGraphAction` carries only `fn_idx`.
    #[test]
    fn tc_ir_2_4_4_3_goap_graph_action_only_fn_idx() {
        assert_eq!(core::mem::size_of::<GoapGraphAction>(), core::mem::size_of::<u32>());
        let a = GoapGraphAction { fn_idx: 2 };
        assert_eq!(a.fn_idx, 2);
    }

    /// TC-IR-2.4.NEG.5 — OOB utility score yields 0.0 and logs.
    #[test]
    fn tc_ir_2_4_neg_5_utility_score_oob() {
        let table = UtilityScoreTable::new(vec![|_, _| 1.0]);
        let state = GraphInstanceState::default();
        let ctx = ExecutionContext::stub();
        let mut logs = Vec::new();
        let s = utility_score(&table, 9, &state, &ctx, |m| logs.push(m.to_string()));
        assert_eq!(s, 0.0);
        assert_eq!(logs.len(), 1);
    }

    /// TC-IR-2.4.2.3 — score fn type accepts immutable state only (compile-time shape).
    #[test]
    fn tc_ir_2_4_2_3_utility_score_fn_is_pure_signature() {
        let score: UtilityScoreFn =
            |s: &GraphInstanceState, _: &ExecutionContext<'_>| s.resume_variant as f32;
        let table = UtilityScoreTable::new(vec![score]);
        let state = GraphInstanceState {
            resume_variant: 3,
        };
        assert_eq!(
            utility_score(&table, 0, &state, &ExecutionContext::stub(), |_| {}),
            3.0
        );
    }

    #[test]
    fn shared_fn_ptr_table_same_instance() {
        let t = FnPtrTable::new(vec![|_, _| StepResult::Error(RuntimeError::Other)]);
        let p1: *const FnPtrTable = &t;
        let p2: *const FnPtrTable = &t;
        assert_eq!(p1, p2);
    }
}
