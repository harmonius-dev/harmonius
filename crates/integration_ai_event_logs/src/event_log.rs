//! Bounded FIFO event log with deterministic queries.

use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

use smallvec::SmallVec;

use crate::ids::EventTypeId;

/// Predicate index into a [`PredicateTable`].
pub type PredicateId = u32;

/// Three-component position (`z = 0` in 2D per integration design).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component (use `0.0` when the scene is 2D).
    pub z: f32,
}

/// Inclusive tick range filter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeRange {
    /// First tick included.
    pub start: u64,
    /// Last tick included.
    pub end: u64,
}

/// Query parameters mirroring the integration design.
#[derive(Clone, Debug, PartialEq)]
pub struct EventLogQuery {
    /// Optional tick window filter on entry timestamps.
    pub time_range: Option<TimeRange>,
    /// Minimum retained accuracy after decay (entries below are skipped).
    pub min_accuracy: Option<f32>,
    /// Optional originating entity filter.
    pub source: Option<crate::ids::Entity>,
    /// Optional predicate id resolved through [`PredicateTable`].
    pub predicate: Option<PredicateId>,
    /// Optional event-type filter (middleman / codegen discriminator).
    pub event_type: Option<EventTypeId>,
    /// Maximum matches (`0` = unlimited).
    pub max_results: u32,
}

/// Single memory cell inside a log.
#[derive(Clone, Debug, PartialEq)]
pub struct DecayingEntry<T>
where
    T: Clone + Debug + PartialEq,
{
    /// Payload stored in the log (design alias: `payload`).
    pub data: T,
    /// Tick when the entry was recorded (design alias: `spawn_tick` on spawn paths).
    pub timestamp: u64,
    /// Confidence in \[0, 1\].
    pub accuracy: f32,
    /// Optional originating entity.
    pub source: Option<crate::ids::Entity>,
    /// Optional spatial tag for queries that filter by region.
    pub position: Option<Vec3>,
    /// Optional event kind for type-filtered queries.
    pub event_type: Option<EventTypeId>,
}

impl<T> DecayingEntry<T>
where
    T: Clone + Debug + PartialEq,
{
    /// Builds a fresh entry at full accuracy.
    pub fn new(data: T, timestamp: u64, source: Option<crate::ids::Entity>) -> Self {
        Self {
            data,
            timestamp,
            accuracy: 1.0,
            source,
            position: None,
            event_type: None,
        }
    }

    /// Same as [`Self::new`], with spatial and type metadata populated.
    pub fn with_spatial(
        data: T,
        timestamp: u64,
        source: Option<crate::ids::Entity>,
        position: Option<Vec3>,
        event_type: Option<EventTypeId>,
    ) -> Self {
        Self {
            data,
            timestamp,
            accuracy: 1.0,
            source,
            position,
            event_type,
        }
    }
}

/// Predicate over live entries (reference stand-in for
/// `fn(&ArchivedDecayingEntry<T>) -> bool` in the middleman; see TC-IR-2.2.R3).
pub type EventPredicate<T> = fn(&T) -> bool;

/// Predicate dispatch table (stand-in for the middleman `.dylib` table).
#[derive(Debug)]
pub struct PredicateTable<T>
where
    T: Clone + Debug + PartialEq,
{
    slots: HashMap<PredicateId, EventPredicate<T>>,
}

impl<T> Default for PredicateTable<T>
where
    T: Clone + Debug + PartialEq,
{
    fn default() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
}

impl<T> PredicateTable<T>
where
    T: Clone + Debug + PartialEq,
{
    /// Empty table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers `predicate` at `id`.
    pub fn register(&mut self, id: PredicateId, predicate: EventPredicate<T>) {
        self.slots.insert(id, predicate);
    }

    /// Looks up a predicate slot.
    pub fn get(&self, id: PredicateId) -> Option<EventPredicate<T>> {
        self.slots.get(&id).copied()
    }
}

/// Mutable warnings collected while querying.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryWarnings {
    /// Set when a [`PredicateId`] has no registered function (FM-7).
    pub missing_predicate: bool,
    /// Incremented once per [`EventLog::query`] when tracing is enabled.
    pub query_trace_invocations: u32,
}

/// Shared inputs for [`EventLog::query`] and higher-level callers.
#[derive(Debug)]
pub struct QueryContext<'a, T>
where
    T: Clone + Debug + PartialEq,
{
    /// Tick used for same-tick suppression when enabled.
    pub read_tick: u64,
    /// Hides entries with `timestamp >= read_tick` (Phase 4 read-after-write).
    pub hide_same_tick_writes: bool,
    /// Predicate table (middleman stand-in).
    pub predicates: &'a PredicateTable<T>,
    /// Warning accumulator.
    pub warnings: &'a mut QueryWarnings,
    /// Runtime trace toggles.
    pub flags: &'a mut crate::debug_flags::EventLogDebugFlags,
}

/// Bounded FIFO log (oldest evicted on overflow).
#[derive(Clone, Debug)]
pub struct EventLog<T>
where
    T: Clone + Debug + PartialEq,
{
    capacity: u32,
    entries: VecDeque<DecayingEntry<T>>,
}

impl<T> EventLog<T>
where
    T: Clone + Debug + PartialEq,
{
    /// Creates an empty log with the given capacity.
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            entries: VecDeque::new(),
        }
    }

    /// Number of live entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when no entries are stored.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Pushes a new entry, evicting the oldest on overflow (FIFO).
    ///
    /// Returns the evicted entry when the ring discards one (FM-3 hook for callers that emit
    /// `LogEntryDecayed` to an ECS channel).
    pub fn push(&mut self, entry: DecayingEntry<T>) -> Option<DecayingEntry<T>> {
        if self.capacity == 0 {
            return None;
        }
        let mut evicted = None;
        while self.entries.len() >= self.capacity as usize {
            evicted = self.entries.pop_front();
        }
        self.entries.push_back(entry);
        evicted
    }

    /// Linear decay step: subtract `delta` from accuracy, clamped to \[0, 1\].
    pub fn decay_linear(&mut self, delta: f32) {
        for e in self.entries.iter_mut() {
            e.accuracy = (e.accuracy - delta).clamp(0.0, 1.0);
        }
    }

    /// Oldest → newest iterator.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &DecayingEntry<T>> {
        self.entries.iter()
    }

    /// Executes a bounded query.
    ///
    /// When `hide_same_tick_writes` is set, entries with `timestamp >= read_tick` are excluded so a
    /// Phase 4 writer cannot observe its own fresh entry in the same tick (TC-IR-2.2.N1).
    #[must_use]
    pub fn query(
        &self,
        q: &EventLogQuery,
        ctx: &mut QueryContext<'_, T>,
    ) -> SmallVec<[DecayingEntry<T>; 16]> {
        if ctx.flags.trace_queries {
            ctx.warnings.query_trace_invocations =
                ctx.warnings.query_trace_invocations.saturating_add(1);
        }
        let mut out = SmallVec::new();
        let max = if q.max_results == 0 {
            usize::MAX
        } else {
            q.max_results as usize
        };
        for e in self.iter() {
            if ctx.hide_same_tick_writes && e.timestamp >= ctx.read_tick {
                continue;
            }
            if let Some(m) = q.min_accuracy {
                if e.accuracy < m {
                    continue;
                }
            }
            if let Some(tr) = q.time_range {
                if e.timestamp < tr.start || e.timestamp > tr.end {
                    continue;
                }
            }
            if let Some(s) = q.source {
                if e.source != Some(s) {
                    continue;
                }
            }
            if let Some(et) = q.event_type {
                if e.event_type != Some(et) {
                    continue;
                }
            }
            if let Some(pid) = q.predicate {
                match ctx.predicates.get(pid) {
                    Some(pred) => {
                        if !pred(&e.data) {
                            continue;
                        }
                    }
                    None => {
                        ctx.warnings.missing_predicate = true;
                        return SmallVec::new();
                    }
                }
            }
            out.push(e.clone());
            if out.len() >= max {
                break;
            }
        }
        out
    }

    /// Entries whose retained accuracy is at least `min_accuracy` (IR-2.2.1 helper).
    pub fn entries_above_accuracy(
        &self,
        min_accuracy: f32,
        ctx: &mut QueryContext<'_, T>,
    ) -> SmallVec<[DecayingEntry<T>; 16]> {
        self.query(
            &EventLogQuery {
                time_range: None,
                min_accuracy: Some(min_accuracy),
                source: None,
                predicate: None,
                event_type: None,
                max_results: 0,
            },
            ctx,
        )
    }

    /// Entries whose tick lies in `window` (IR-2.2.1 helper).
    pub fn entries_in_window(
        &self,
        window: TimeRange,
        ctx: &mut QueryContext<'_, T>,
    ) -> SmallVec<[DecayingEntry<T>; 16]> {
        self.query(
            &EventLogQuery {
                time_range: Some(window),
                min_accuracy: None,
                source: None,
                predicate: None,
                event_type: None,
                max_results: 0,
            },
            ctx,
        )
    }
}
