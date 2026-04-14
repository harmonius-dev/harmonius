//! Behavior tree event memory leaf (IR-2.2.1).

use crate::blackboard::{Blackboard, BlackboardValue};
use crate::event_log::{EventLog, EventLogQuery, PredicateId, QueryContext, TimeRange};

/// BT leaf configuration that maps log memory into a blackboard counter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BtEventMemoryCheck {
    /// Minimum accuracy for entries to count.
    pub min_accuracy: f32,
    /// Time window in game ticks ending at the read tick.
    pub window_ticks: u64,
    /// Predicate slot used for semantic filtering.
    pub predicate: PredicateId,
    /// Blackboard key receiving the match count.
    pub result_key: crate::blackboard::BlackboardKey,
}

impl BtEventMemoryCheck {
    /// Builds the [`EventLogQuery`] for this leaf at `current_tick`.
    pub fn to_query(&self, current_tick: u64) -> EventLogQuery {
        let start = current_tick.saturating_sub(self.window_ticks);
        EventLogQuery {
            time_range: Some(TimeRange {
                start,
                end: current_tick,
            }),
            min_accuracy: Some(self.min_accuracy),
            source: None,
            predicate: Some(self.predicate),
            max_results: 0,
        }
    }
}

/// Runs the memory check and writes the integer match count to the blackboard.
pub fn apply_bt_event_memory_check<T>(
    leaf: &BtEventMemoryCheck,
    log: &EventLog<T>,
    ctx: &mut QueryContext<'_, T>,
    blackboard: &mut Blackboard,
) where
    T: Clone + std::fmt::Debug + PartialEq,
{
    let q = leaf.to_query(ctx.read_tick);
    let matches = log.query(&q, ctx);
    blackboard.set(leaf.result_key, BlackboardValue::I32(matches.len() as i32));
}
