use core::hint::black_box;

use crate::AnimationParams;
use crate::Diagnostics;

/// Nanosecond frame budget shared by AI + animation evaluation (IR-1.1.5, FM-4).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameBudget {
    remaining_ns: u64,
}

impl FrameBudget {
    /// Builds a budget from whole milliseconds (deterministic scheduling tests).
    #[must_use]
    pub fn from_millis(millis: u64) -> Self {
        Self {
            remaining_ns: millis.saturating_mul(1_000_000),
        }
    }

    /// Remaining budget in nanoseconds.
    #[must_use]
    pub fn remaining_ns(self) -> u64 {
        self.remaining_ns
    }

    /// Attempts to subtract `cost_ns`, returning `false` when insufficient budget remains.
    pub fn try_consume(&mut self, cost_ns: u64) -> bool {
        if self.remaining_ns >= cost_ns {
            self.remaining_ns -= cost_ns;
            true
        } else {
            false
        }
    }
}

/// Per-agent evaluation cost reservation (AI + animation).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentEvalCost {
    /// AI evaluation nanoseconds charged to the shared budget.
    pub ai_ns: u64,
    /// Animation evaluation nanoseconds charged to the shared budget.
    pub anim_ns: u64,
}

impl AgentEvalCost {
    /// Combined nanoseconds for one agent.
    #[must_use]
    pub const fn total(self) -> u64 {
        self.ai_ns.saturating_add(self.anim_ns)
    }
}

/// Slot processed by [`process_agents_with_budget`].
#[derive(Clone, Debug, PartialEq)]
pub struct AgentSlot {
    /// Locomotion parameters; deferred agents must retain prior values (FM-4).
    pub params: AnimationParams,
}

/// Outcome of a single budgeted scheduling pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetedTickResult {
    /// Agents that ran AI + animation this tick.
    pub evaluated: usize,
    /// Agents deferred because the budget was exhausted.
    pub deferred: usize,
}

/// Deterministic time-slicing: processes prefix of `agents` until the budget is exhausted.
///
/// Deferred agents keep their existing [`AgentSlot::params`] unchanged (FM-4).
pub fn process_agents_with_budget(
    budget: &mut FrameBudget,
    per_agent: AgentEvalCost,
    agents: &mut [AgentSlot],
    diagnostics: Option<&mut Diagnostics>,
) -> BudgetedTickResult {
    let mut evaluated = 0usize;
    let cost = per_agent.total();
    if cost == 0 {
        return BudgetedTickResult {
            evaluated: agents.len(),
            deferred: 0,
        };
    }
    for slot in agents.iter_mut() {
        if budget.try_consume(cost) {
            black_box(&mut slot.params.speed);
            evaluated += 1;
        } else {
            break;
        }
    }
    let deferred = agents.len() - evaluated;
    if let Some(d) = diagnostics {
        if deferred > 0 {
            d.debug(format!("deferred {deferred} agents"));
        }
    }
    BudgetedTickResult {
        evaluated,
        deferred,
    }
}
