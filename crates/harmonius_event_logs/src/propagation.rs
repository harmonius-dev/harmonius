//! Cross-entity log propagation.

use rkyv::{Archive, Deserialize, Serialize};

use crate::entry::DecayingEntry;
use crate::log::EventLog;
use crate::types::{EntryId, TagSet};

/// Immutable propagation parameters baked with assets.
#[derive(Clone, Debug, Archive, Serialize, Deserialize)]
pub struct PropagationRule {
    /// Query radius in world units.
    pub range: f32,
    /// Per-hop accuracy multiplier.
    pub accuracy_multiplier: f32,
    /// Delay before a propagated copy may appear.
    pub delay_ticks: u32,
    /// Maximum hop count allowed on a propagated entry.
    pub max_hops: u8,
    /// Only entries matching these tags propagate when non-empty.
    pub filter_tags: TagSet,
}

/// Propagates eligible entries from `source` into `target`, deduplicating copies.
pub fn propagate_entries<T: Clone + rkyv::Archive + PartialEq>(
    source_log: &EventLog<T>,
    target_log: &mut EventLog<T>,
    rule: &PropagationRule,
    current_tick: u64,
) {
    for entry in &source_log.entries {
        if entry.hop_count >= rule.max_hops {
            continue;
        }
        if current_tick < entry.timestamp.saturating_add(u64::from(rule.delay_ticks)) {
            continue;
        }
        if !rule.filter_tags.matches_entry_tags(entry.tags.as_slice()) {
            continue;
        }
        let new_accuracy = (entry.accuracy * rule.accuracy_multiplier).clamp(0.0, 1.0);
        let new_hop = entry.hop_count.saturating_add(1);
        if new_hop > rule.max_hops {
            continue;
        }
        let dup = target_log.entries.iter().any(|t| {
            t.timestamp == entry.timestamp && t.source == entry.source && t.data == entry.data
        });
        if dup {
            continue;
        }
        let id = EntryId(target_log.next_entry_id);
        target_log.next_entry_id = target_log.next_entry_id.wrapping_add(1);
        let pushed = DecayingEntry {
            id,
            data: entry.data.clone(),
            timestamp: entry.timestamp,
            accuracy: new_accuracy,
            source: entry.source,
            position: entry.position,
            hop_count: new_hop,
            tags: entry.tags.clone(),
        };
        let cap = target_log.capacity as usize;
        if cap == 0 {
            return;
        }
        if target_log.entries.len() == cap {
            target_log.entries.remove(0);
        }
        target_log.entries.push(pushed);
    }
}
