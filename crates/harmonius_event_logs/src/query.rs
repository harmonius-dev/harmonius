//! Structured queries over log entries.

use rkyv::{Archive, Deserialize, Serialize};
use smallvec::SmallVec;

use crate::entry::DecayingEntry;
use crate::log::EventLog;
use crate::types::{Entity, EventTypeId, PredicateId};

/// Inclusive tick range for log queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub struct TimeRange {
    /// First tick in the range.
    pub start: u64,
    /// Last tick in the range.
    pub end: u64,
}

/// Optional filters for log queries.
#[derive(Clone, Debug, Default, PartialEq, Archive, Serialize, Deserialize)]
pub struct EventLogQuery {
    /// Filter by event type identifier.
    pub event_type: Option<EventTypeId>,
    /// Filter by tick range (inclusive).
    pub time_range: Option<TimeRange>,
    /// Minimum accuracy threshold (inclusive).
    pub min_accuracy: Option<f32>,
    /// Filter by source entity.
    pub source: Option<Entity>,
    /// Optional codegen predicate identifier.
    pub predicate: Option<PredicateId>,
    /// Maximum results (`0` means unlimited).
    pub max_results: u16,
}

/// Associates query metadata with a stored payload type.
pub trait LogEventMetadata {
    /// Returns the stable event type id for this payload.
    fn event_type_id(&self) -> EventTypeId;
}

/// Applies `query` to `log`, evaluating optional predicates with `pred`.
pub fn query_entries<'a, T, P>(
    log: &'a EventLog<T>,
    query: &EventLogQuery,
    pred: &mut P,
) -> SmallVec<[&'a DecayingEntry<T>; 8]>
where
    T: Clone + rkyv::Archive + LogEventMetadata,
    P: FnMut(PredicateId, &T) -> bool,
{
    let mut out = SmallVec::new();
    for entry in log.entries.iter() {
        if let Some(et) = query.event_type {
            if entry.data.event_type_id() != et {
                continue;
            }
        }
        if let Some(tr) = query.time_range {
            if entry.timestamp < tr.start || entry.timestamp > tr.end {
                continue;
            }
        }
        if let Some(ma) = query.min_accuracy {
            if entry.accuracy < ma {
                continue;
            }
        }
        if let Some(s) = query.source {
            if entry.source != Some(s) {
                continue;
            }
        }
        if let Some(pid) = query.predicate {
            if !pred(pid, &entry.data) {
                continue;
            }
        }
        out.push(entry);
        if query.max_results != 0 && out.len() >= query.max_results as usize {
            break;
        }
    }
    out
}
