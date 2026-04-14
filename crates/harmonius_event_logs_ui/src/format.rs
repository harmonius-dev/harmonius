//! Deterministic formatting from [`harmonius_event_logs::DecayingEntry`] payloads.

use harmonius_event_logs::{DecayingEntry, Entity, EventLog, query_entries};
use crate::types::{CombatEvent, CombatEventKind, CombatLogArg, CombatLogLine, CombatLogBinding};

/// Template keys referenced by [`CombatLogLine::template`].
pub mod template {
    use crate::types::LocalizedStringId;

    /// Damage dealt template (`"{0} dealt {1} damage"` style tables).
    pub const DAMAGE_DEALT: LocalizedStringId = LocalizedStringId(10);
    /// Healing applied template.
    pub const HEALING_DONE: LocalizedStringId = LocalizedStringId(11);
    /// Generic placeholder when filters match nothing.
    pub const NO_ENTRIES: LocalizedStringId = LocalizedStringId(12);
}

/// Builds a [`CombatLogLine`] from a decaying entry (accuracy maps 1:1 to opacity).
pub fn combat_line_from_entry(entry: &DecayingEntry<CombatEvent>) -> CombatLogLine {
    let ev = entry.data;
    let (template, args) = match ev.kind {
        CombatEventKind::Damage
        | CombatEventKind::Miss
        | CombatEventKind::Dodge
        | CombatEventKind::Block => {
            let args = vec![
                CombatLogArg::Entity(ev.source),
                CombatLogArg::Int(ev.value),
            ];
            (template::DAMAGE_DEALT, args)
        }
        CombatEventKind::Healing => {
            let args = vec![
                CombatLogArg::Entity(ev.source),
                CombatLogArg::Int(ev.value),
            ];
            (template::HEALING_DONE, args)
        }
        _ => {
            let args = vec![
                CombatLogArg::Entity(ev.source),
                CombatLogArg::Int(ev.value),
            ];
            (template::DAMAGE_DEALT, args)
        }
    };
    CombatLogLine {
        template,
        kind: ev.kind,
        args,
        opacity: entry.accuracy,
        timestamp: entry.timestamp,
    }
}

/// Resolves a placeholder line when the log is empty or filters match nothing.
pub fn no_entries_line() -> CombatLogLine {
    CombatLogLine {
        template: template::NO_ENTRIES,
        kind: CombatEventKind::Miss,
        args: Vec::new(),
        opacity: 1.0,
        timestamp: 0,
    }
}

/// Returns filtered [`CombatLogLine`] rows (newest chunk capped by `max_display`).
pub fn visible_combat_lines<P>(
    log: &EventLog<CombatEvent>,
    binding: &CombatLogBinding,
    pred: &mut P,
) -> Vec<CombatLogLine>
where
    P: FnMut(harmonius_event_logs::PredicateId, &CombatEvent) -> bool,
{
    let matched = query_entries(log, &binding.query, pred);
    if matched.is_empty() {
        return vec![no_entries_line()];
    }
    let max = binding.max_display as usize;
    let start = matched.len().saturating_sub(max);
    matched[start..]
        .iter()
        .map(|e| combat_line_from_entry(e))
        .collect()
}

/// Merges multiple logs inside a tick window, ordered oldest-first among equals by entity id.
pub fn merge_logs_chronological<'a>(
    logs: &[(Entity, &'a EventLog<CombatEvent>)],
    from_tick: u64,
    to_tick: u64,
) -> Vec<(Entity, &'a DecayingEntry<CombatEvent>)> {
    let mut rows: Vec<(Entity, u64, u64, &'a DecayingEntry<CombatEvent>)> = Vec::new();
    for (owner, log) in logs {
        for e in log.entries_in_window(from_tick, to_tick) {
            rows.push((*owner, e.timestamp, owner.0, e));
        }
    }
    rows.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.2.cmp(&b.2)));
    rows.into_iter().map(|(e, _, _, d)| (e, d)).collect()
}
