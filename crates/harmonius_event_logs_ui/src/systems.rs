//! Display-only bridge helpers (`spawn_combat_text`, `update_combat_log` analogues).

use std::collections::BTreeSet;
use std::collections::VecDeque;

use harmonius_event_logs::{DecayingEntry, Entity, EventLog};

use crate::format::visible_combat_lines;
use crate::resolve::resolve_combat_line;
use crate::scroll::ScrollView;
use crate::types::{
    CombatEvent, CombatEventKind, CombatLogBinding, DataBindingComponent, FloatingCombatText,
    LogEntryAdded, LocaleId, Transform,
};

/// Bounded FIFO queue for [`LogEntryAdded`] with saturation accounting (`FM-6`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogEntryAddedQueue {
    inner: VecDeque<LogEntryAdded>,
    capacity: usize,
    /// Count of events dropped while the queue remained at `capacity` (`FM-6`).
    pub drops_logged: u32,
}

impl LogEntryAddedQueue {
    /// Creates a queue with the given maximum depth.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: VecDeque::new(),
            capacity,
            drops_logged: 0,
        }
    }

    /// Pushes an event; when full, the new event is dropped and [`Self::drops_logged`] increments.
    pub fn push(&mut self, event: LogEntryAdded) {
        if self.inner.len() >= self.capacity {
            self.drops_logged = self.drops_logged.saturating_add(1);
            return;
        }
        self.inner.push_back(event);
    }

    /// Drains up to `max` events from the front.
    pub fn drain_up_to(&mut self, max: usize) -> Vec<LogEntryAdded> {
        let n = max.min(self.inner.len());
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(e) = self.inner.pop_front() {
                out.push(e);
            }
        }
        out
    }
}

/// Tracks throttle hits for high-rate bursts (`FM-5`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CombatLogThrottleState {
    /// Incremented when an event batch is truncated by `max_per_frame`.
    pub combat_log_throttled: u64,
}

/// Result of [`update_combat_log_view`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CombatLogUpdateNotes {
    /// Set when the backing log entity was missing.
    pub log_entity_missing: bool,
    /// Number of events truncated this frame due to `max_per_frame`.
    pub throttled_events: u32,
}

/// Rebuilds scroll contents and [`DataBindingComponent::computed`] from the log.
#[allow(clippy::too_many_arguments)]
pub fn update_combat_log_view<P>(
    log: Option<&EventLog<CombatEvent>>,
    binding: &CombatLogBinding,
    scroll: &mut ScrollView,
    data: &mut DataBindingComponent,
    pred: &mut P,
    line_height: f32,
    locale: LocaleId,
    strings: &std::collections::HashMap<(LocaleId, crate::types::LocalizedStringId), &'static str>,
    names: &std::collections::HashMap<Entity, &'static str>,
    frame_events: &[LogEntryAdded],
    throttle: &mut CombatLogThrottleState,
) -> CombatLogUpdateNotes
where
    P: FnMut(harmonius_event_logs::PredicateId, &CombatEvent) -> bool,
{
    let mut notes = CombatLogUpdateNotes::default();
    let was_at_bottom = scroll.is_scrolled_to_bottom();
    let Some(log) = log else {
        notes.log_entity_missing = true;
        scroll.clear_lines();
        data.computed.clear();
        return notes;
    };
    let max_ev = binding.max_per_frame as usize;
    let ev_count = frame_events.len();
    if ev_count > max_ev {
        throttle.combat_log_throttled += 1;
        notes.throttled_events = (ev_count - max_ev) as u32;
    }
    let lines = visible_combat_lines(log, binding, pred);
    data.computed = lines.iter().cloned().collect();
    scroll.clear_lines();
    for line in &lines {
        let text = resolve_combat_line(line, locale, strings, names);
        scroll.append_line(
            crate::types::RichText {
                display: text,
                opacity: line.opacity,
            },
            line_height,
            was_at_bottom,
        );
    }
    notes
}

/// Finds a log entry whose `EntryId` matches `entry_index`.
fn find_entry(
    log: &EventLog<CombatEvent>,
    entry_index: u32,
) -> Option<&DecayingEntry<CombatEvent>> {
    log.entries()
        .find(|e| e.id.0 == entry_index)
}

/// Decides whether to spawn floating combat text for a damage or heal row.
pub fn spawn_floating_combat_text(
    event: LogEntryAdded,
    log: Option<&EventLog<CombatEvent>>,
    target_xform: Option<&Transform>,
    warned_missing_transform: &mut BTreeSet<u64>,
) -> (Option<FloatingCombatText>, Option<&'static str>) {
    let Some(log) = log else {
        return (None, Some("log missing"));
    };
    let Some(entry) = find_entry(log, event.entry_index) else {
        return (None, Some("entry missing"));
    };
    let ev = entry.data;
    if !matches!(ev.kind, CombatEventKind::Damage | CombatEventKind::Healing) {
        return (None, None);
    }
    let tid = ev.target.0;
    let Some(xf) = target_xform else {
        if warned_missing_transform.insert(tid) {
            return (None, Some("missing transform"));
        }
        return (None, None);
    };
    (
        Some(FloatingCombatText {
            world_pos: xf.translation,
            duration: 2.0,
            elapsed: 0.0,
        }),
        None,
    )
}

/// Applies `max_per_frame` batching to raw events before dispatch (`TC-IR-2.10.FM5`).
pub fn apply_event_throttle(
    events: &[LogEntryAdded],
    max_per_frame: u16,
    throttle: &mut CombatLogThrottleState,
) -> Vec<LogEntryAdded> {
    let max = max_per_frame as usize;
    if events.len() > max {
        throttle.combat_log_throttled += 1;
        events[..max].to_vec()
    } else {
        events.to_vec()
    }
}
