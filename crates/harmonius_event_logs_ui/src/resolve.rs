//! Localization-style resolution for [`crate::types::CombatLogLine`].

use std::collections::HashMap;

use harmonius_event_logs::Entity;
use smol_str::SmolStr;

use crate::types::{CombatLogArg, CombatLogLine, LocaleId, LocalizedStringId};

/// Fills numbered `{0}`, `{1}`, … slots using [`CombatLogArg`] values and tables.
pub fn resolve_combat_line(
    line: &CombatLogLine,
    locale: LocaleId,
    strings: &HashMap<(LocaleId, LocalizedStringId), &'static str>,
    names: &HashMap<Entity, &'static str>,
) -> SmolStr {
    let tpl = strings
        .get(&(locale, line.template))
        .copied()
        .unwrap_or("<missing template>");
    let mut slots: Vec<String> = Vec::new();
    for arg in &line.args {
        slots.push(match arg {
            CombatLogArg::Entity(e) => names.get(e).copied().unwrap_or("?").to_string(),
            CombatLogArg::Int(i) => i.to_string(),
            CombatLogArg::Float(f) => format!("{f}"),
            CombatLogArg::Name(id) => strings
                .get(&(locale, *id))
                .copied()
                .unwrap_or("?name")
                .to_string(),
        });
    }
    let mut out = tpl.to_string();
    for (idx, slot) in slots.iter().enumerate() {
        out = out.replace(&format!("{{{idx}}}"), slot);
    }
    SmolStr::from(out)
}
