//! Trigger overlap helpers (TC-IR-5.4.6.2).

use crate::debug::TriggerEvent;
use crate::model::Entity;

/// Emits a trigger event when `overlap` is true (harness narrows overlap test).
#[must_use]
pub fn trigger_event_if_overlapping(
    trigger_entity: Entity,
    visitor_entity: Entity,
    overlap: bool,
) -> Option<TriggerEvent> {
    if !overlap {
        return None;
    }
    Some(TriggerEvent {
        entity_a: trigger_entity,
        entity_b: visitor_entity,
    })
}
