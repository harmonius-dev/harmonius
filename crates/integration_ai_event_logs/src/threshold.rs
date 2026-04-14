//! Threshold triggers that emit [`ThresholdFired`] summaries (IR-2.2.4).

use crate::event_log::{EventLog, EventLogQuery, QueryContext};
use crate::ids::Entity;

/// Registered threshold rule referencing a query and window.
#[derive(Clone, Debug, PartialEq)]
pub struct ThresholdTrigger {
    /// Shared query template.
    pub query: EventLogQuery,
    /// Minimum number of matching entries to fire.
    pub min_count: u32,
    /// Window end is aligned to `current_tick` the same way as BT memory.
    pub window_ticks: u64,
}

/// Event published when a threshold fires.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThresholdFired {
    /// Entity owning the log.
    pub entity: Entity,
    /// Index of the trigger rule in a component list.
    pub trigger_index: u16,
    /// Entries that matched at fire time.
    pub matched_count: u32,
}

/// Evaluates a trigger; returns `Some` when `matched_count >= min_count`.
pub fn evaluate_threshold_trigger<T>(
    entity: Entity,
    trigger_index: u16,
    trigger: &ThresholdTrigger,
    log: &EventLog<T>,
    ctx: &mut QueryContext<'_, T>,
) -> Option<ThresholdFired>
where
    T: Clone + std::fmt::Debug + PartialEq,
{
    let mut q = trigger.query.clone();
    let start = ctx.read_tick.saturating_sub(trigger.window_ticks);
    q.time_range = Some(crate::event_log::TimeRange {
        start,
        end: ctx.read_tick,
    });
    let matched = log.query(&q, ctx);
    let count = matched.len() as u32;
    if count >= trigger.min_count {
        Some(ThresholdFired {
            entity,
            trigger_index,
            matched_count: count,
        })
    } else {
        None
    }
}
