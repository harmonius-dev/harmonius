//! Utility consideration scoring from event log counts (IR-2.2.2).

use crate::event_log::{EventLog, EventLogQuery, QueryContext};

/// Piecewise-linear response: maps `matched_count` to \[0, 1\] by `matched / saturate_at`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseCurve {
    /// Count that maps to a score of `1.0`.
    pub saturate_at: u32,
}

impl ResponseCurve {
    /// Maps an event count into \[0, 1\].
    pub fn eval(&self, matched_count: usize) -> f32 {
        if self.saturate_at == 0 {
            return 0.0;
        }
        (matched_count as f32 / self.saturate_at as f32).clamp(0.0, 1.0)
    }
}

/// Utility consideration driven by an [`EventLogQuery`].
#[derive(Clone, Debug, PartialEq)]
pub struct EventLogConsideration {
    /// Filters applied before counting.
    pub query: EventLogQuery,
    /// Maps count to score.
    pub curve: ResponseCurve,
}

/// Counts matching entries and evaluates the response curve.
pub fn score_event_log_consideration<T>(
    consideration: &EventLogConsideration,
    log: &EventLog<T>,
    ctx: &mut QueryContext<'_, T>,
) -> f32
where
    T: Clone + std::fmt::Debug + PartialEq,
{
    let matched = log.query(&consideration.query, ctx);
    consideration.curve.eval(matched.len())
}
