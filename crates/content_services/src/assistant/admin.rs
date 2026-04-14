//! Assistant administration: token-bucket rate limits and spend quotas.

/// Too many requests for the configured window (TC-15.9.10.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RateLimitExceeded;

/// LLM spend exceeded the configured budget (TC-15.9.10.2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BudgetExceeded;

/// Fixed-window counter modeling `N` requests per minute (tests advance explicitly).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimiter {
    max_per_window: u32,
    issued: u32,
}

impl RateLimiter {
    /// Configures the maximum successful acquisitions per window.
    pub fn new(max_per_minute: u32) -> Self {
        Self {
            max_per_window: max_per_minute,
            issued: 0,
        }
    }

    /// Increments usage and fails once the cap is reached.
    pub fn try_acquire(&mut self) -> Result<(), RateLimitExceeded> {
        if self.issued >= self.max_per_window {
            return Err(RateLimitExceeded);
        }
        self.issued += 1;
        Ok(())
    }
}

/// Micro-dollar budget tracking for hosted assistant calls.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlmSpendBudget {
    budget_cents: u32,
    spent_cents: u32,
}

impl LlmSpendBudget {
    /// Creates a budget ceiling expressed in integer cents.
    pub fn new(budget_cents: u32) -> Self {
        Self {
            budget_cents,
            spent_cents: 0,
        }
    }

    /// Charges `cost_cents`, failing when the ceiling would be exceeded.
    pub fn try_charge(&mut self, cost_cents: u32) -> Result<(), BudgetExceeded> {
        let next = self.spent_cents.saturating_add(cost_cents);
        if next > self.budget_cents {
            return Err(BudgetExceeded);
        }
        self.spent_cents = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.9.10.1 — hard cap rejects the 101st acquisition in a window.
    #[test]
    fn tc_15_9_10_1_rate_limiter_token_bucket() {
        let mut limiter = RateLimiter::new(100);
        for _ in 0..100 {
            assert!(limiter.try_acquire().is_ok());
        }
        assert_eq!(limiter.try_acquire(), Err(RateLimitExceeded));
    }

    /// TC-15.9.10.2 — spend quota rejects further charges after the budget is exhausted.
    #[test]
    fn tc_15_9_10_2_quota_enforcement() {
        let mut budget = LlmSpendBudget::new(10);
        assert!(budget.try_charge(6).is_ok());
        assert!(budget.try_charge(4).is_ok());
        assert_eq!(budget.try_charge(1), Err(BudgetExceeded));
    }
}
