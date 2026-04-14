//! Split AI budgets: perception vs decision (IR-1.10.5).

/// Microsecond budget for perception queries (`ResMut` in the design doc).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AiPerceptionBudget {
    /// Budget granted each frame.
    pub budget_micros: u32,
    /// Consumption so far this frame.
    pub spent_micros: u32,
}

impl Default for AiPerceptionBudget {
    fn default() -> Self {
        Self {
            budget_micros: 2_000,
            spent_micros: 0,
        }
    }
}

impl AiPerceptionBudget {
    /// Resets spend counters at a frame boundary.
    pub fn reset_frame(&mut self) {
        self.spent_micros = 0;
    }

    /// Attempts to spend `cost_micros`, returning false when the budget is exhausted.
    pub fn try_consume(&mut self, cost_micros: u32) -> bool {
        let next = self.spent_micros.saturating_add(cost_micros);
        if next <= self.budget_micros {
            self.spent_micros = next;
            true
        } else {
            false
        }
    }
}

/// Microsecond budget for BT / GOAP (`ResMut`, separate resource in the design doc).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AiDecisionBudget {
    /// Budget granted each frame.
    pub budget_micros: u32,
    /// Consumption so far this frame.
    pub spent_micros: u32,
}

impl Default for AiDecisionBudget {
    fn default() -> Self {
        Self {
            budget_micros: 800,
            spent_micros: 0,
        }
    }
}

impl AiDecisionBudget {
    /// Resets spend counters at a frame boundary.
    pub fn reset_frame(&mut self) {
        self.spent_micros = 0;
    }

    /// Attempts to spend `cost_micros`, returning false when the budget is exhausted.
    pub fn try_consume(&mut self, cost_micros: u32) -> bool {
        let next = self.spent_micros.saturating_add(cost_micros);
        if next <= self.budget_micros {
            self.spent_micros = next;
            true
        } else {
            false
        }
    }
}

/// Tracks stale perception rows for agents deferred one frame (IR-1.10.5.2).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PerceptionFrameState {
    /// Last frame index when perception ran for an agent slot.
    pub last_run_frame: Vec<u64>,
}

impl PerceptionFrameState {
    /// Marks `agent_index` as updated on `frame`.
    pub fn mark_ran(&mut self, agent_index: usize, frame: u64) {
        if self.last_run_frame.len() <= agent_index {
            self.last_run_frame.resize(agent_index + 1, 0);
        }
        self.last_run_frame[agent_index] = frame;
    }

    /// Returns true when perception was not refreshed on `frame` (stale / deferred).
    pub fn is_stale(&self, agent_index: usize, frame: u64) -> bool {
        self.last_run_frame
            .get(agent_index)
            .map(|&f| f < frame)
            .unwrap_or(true)
    }
}

/// Cursor for FM-3 round-robin: deferred agents are tried first on the next slice.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PerceptionBudgetCursor {
    /// Index of the next agent to prioritize at a fresh budget.
    pub next_agent: usize,
}

/// Runs perception for `agent_costs` in order; agents that do not fit in the budget are skipped.
pub fn run_perception_budget_slice(
    budget: &mut AiPerceptionBudget,
    agent_costs: &[u32],
) -> Vec<bool> {
    let mut cursor = PerceptionBudgetCursor::default();
    run_perception_budget_slice_with_cursor(budget, agent_costs, &mut cursor)
}

/// Like [`run_perception_budget_slice`], but rotates start using `cursor` across calls (FM-3).
pub fn run_perception_budget_slice_with_cursor(
    budget: &mut AiPerceptionBudget,
    agent_costs: &[u32],
    cursor: &mut PerceptionBudgetCursor,
) -> Vec<bool> {
    let n = agent_costs.len();
    let mut out = vec![false; n];
    if n == 0 {
        return out;
    }
    let start = cursor.next_agent % n;
    for k in 0..n {
        let i = (start + k) % n;
        out[i] = budget.try_consume(agent_costs[i]);
    }
    let mut next = start;
    let mut found_deferred = false;
    for k in 0..n {
        let i = (start + k) % n;
        if !out[i] {
            next = i;
            found_deferred = true;
            break;
        }
    }
    cursor.next_agent = if found_deferred {
        next
    } else {
        (start + 1) % n
    };
    out
}
