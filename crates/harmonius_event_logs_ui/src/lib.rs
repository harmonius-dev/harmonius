//! Event logs ↔ UI integration: combat log binding, scroll model, and FCT spawn helpers.
//!
//! Implements the deterministic surface described in `docs/design/integration/event-logs-ui.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

mod format;
mod resolve;
mod scroll;
mod systems;
mod types;
#[cfg(test)]
mod tests;

pub use format::{combat_line_from_entry, merge_logs_chronological, no_entries_line, template,
    visible_combat_lines};
pub use harmonius_event_logs::{Entity, EventLog, EventLogQuery, EventTypeId, PredicateId,
    query_entries};
pub use resolve::resolve_combat_line;
pub use scroll::{ScrollView, AUTO_SCROLL_EPSILON};
pub use systems::{
    apply_event_throttle, spawn_floating_combat_text, update_combat_log_view,
    CombatLogThrottleState, CombatLogUpdateNotes, LogEntryAddedQueue,
};
pub use types::{
    CombatEvent, CombatEventKind, CombatLogArg, CombatLogBinding, CombatLogLine,
    DataBindingComponent, FloatingCombatText, LogEntryAdded, LocaleId, LocalizedStringId, RichText,
    Transform,
};
