//! Per-subsystem memory budgets.

use super::tag::SubsystemId;
use std::sync::Mutex;

/// Budget enforcement outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BudgetError {
    /// Allocation would exceed the configured ceiling.
    Exceeded {
        /// Configured ceiling in bytes.
        ceiling: usize,
        /// Bytes already charged.
        used: usize,
        /// Bytes requested.
        requested: usize,
    },
}

struct BudgetInner {
    ceilings: Vec<usize>,
    used: Vec<usize>,
}

/// Tracks per-subsystem memory ceilings and current usage.
pub struct MemoryBudget {
    inner: Mutex<BudgetInner>,
}

impl Default for MemoryBudget {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryBudget {
    /// Creates an empty budget table.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(BudgetInner {
                ceilings: Vec::new(),
                used: Vec::new(),
            }),
        }
    }

    /// Sets the ceiling for `id` to `bytes`.
    pub fn set_budget(&self, id: SubsystemId, bytes: usize) {
        let mut g = self.inner.lock().expect("budget lock");
        let i = id.0 as usize;
        if i >= g.ceilings.len() {
            g.ceilings.resize(i + 1, 0);
            g.used.resize(i + 1, 0);
        }
        g.ceilings[i] = bytes;
    }

    /// Records an allocation of `bytes` for `id`, failing when over budget.
    pub fn try_record_alloc(&self, id: SubsystemId, bytes: usize) -> Result<(), BudgetError> {
        let mut g = self.inner.lock().expect("budget lock");
        let i = id.0 as usize;
        if i >= g.ceilings.len() || g.ceilings[i] == 0 {
            return Ok(());
        }
        let ceiling = g.ceilings[i];
        let cur = g.used[i];
        let next = cur.saturating_add(bytes);
        if next > ceiling {
            return Err(BudgetError::Exceeded {
                ceiling,
                used: cur,
                requested: bytes,
            });
        }
        g.used[i] = next;
        Ok(())
    }

    /// Releases `bytes` previously charged to `id`.
    pub fn record_dealloc(&self, id: SubsystemId, bytes: usize) {
        let mut g = self.inner.lock().expect("budget lock");
        let i = id.0 as usize;
        if i >= g.used.len() {
            return;
        }
        g.used[i] = g.used[i].saturating_sub(bytes);
    }

    /// Bytes currently charged for `id`.
    #[must_use]
    pub fn usage_bytes(&self, id: SubsystemId) -> usize {
        let g = self.inner.lock().expect("budget lock");
        let i = id.0 as usize;
        if i >= g.used.len() {
            0
        } else {
            g.used[i]
        }
    }
}
