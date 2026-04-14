//! Global particle budget with priority (`TC-11.1.4.3`, `TC-11.1.4.4`).

/// Tracks total alive particles against a hard cap.
#[derive(Clone, Debug)]
pub struct ParticleBudgetManager {
    global_max: u32,
    total_alive: u32,
}

impl ParticleBudgetManager {
    /// Creates a manager with the given platform ceiling.
    #[must_use]
    pub fn new(global_max: u32) -> Self {
        Self {
            global_max,
            total_alive: 0,
        }
    }

    /// Current total alive particles.
    #[must_use]
    pub fn total_alive(&self) -> u32 {
        self.total_alive
    }

    /// Attempts to add `delta` alive particles, clamped by `global_max`.
    ///
    /// Returns how many were actually accepted.
    pub fn try_add_alive(&mut self, delta: u32) -> u32 {
        let room = self.global_max.saturating_sub(self.total_alive);
        let grant = delta.min(room);
        self.total_alive += grant;
        grant
    }

    /// Removes up to `delta` alive particles (e.g., particles died).
    pub fn release_alive(&mut self, delta: u32) {
        self.total_alive = self.total_alive.saturating_sub(delta);
    }
}

/// One emitter requesting particles under a shared cap.
#[derive(Clone, Debug)]
pub struct EmitterBudgetRequest {
    /// Lower values are culled first when over budget.
    pub priority: u8,
    /// Desired alive count for this emitter after allocation.
    pub desired_alive: u32,
}

/// Grants each emitter a slice of `global_max` without exceeding the cap. Lower `priority` values
/// are reduced first until the sum fits.
#[must_use]
pub fn allocate_by_priority(global_max: u32, requests: &[EmitterBudgetRequest]) -> Vec<u32> {
    let mut granted: Vec<u32> = requests.iter().map(|r| r.desired_alive).collect();
    let sum: u32 = granted.iter().copied().sum();
    if sum <= global_max {
        return granted;
    }
    let mut over = sum - global_max;
    let mut order: Vec<usize> = (0..requests.len()).collect();
    order.sort_by_key(|&i| requests[i].priority);
    for idx in order {
        if over == 0 {
            break;
        }
        let g = &mut granted[idx];
        let cut = (*g).min(over);
        *g -= cut;
        over -= cut;
    }
    granted
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.4.3` — total alive never exceeds `global_max`.
    #[test]
    fn tc_11_1_4_3_budget_cap_enforced() {
        let mut m = ParticleBudgetManager::new(100);
        assert_eq!(m.try_add_alive(80), 80);
        assert_eq!(m.try_add_alive(80), 20);
        assert_eq!(m.total_alive(), 100);
    }

    /// `TC-11.1.4.4` — low priority culled first.
    #[test]
    fn tc_11_1_4_4_budget_priority_cull() {
        let reqs = [
            EmitterBudgetRequest {
                priority: 10,
                desired_alive: 60,
            },
            EmitterBudgetRequest {
                priority: 200,
                desired_alive: 60,
            },
        ];
        let g = allocate_by_priority(100, &reqs);
        assert_eq!(g.iter().sum::<u32>(), 100);
        assert_eq!(g[0], 40, "low priority should lose 20 first");
        assert_eq!(g[1], 60);
    }
}
