//! Free look marker toggled by a boolean action.

use crate::action::{ActionId, ActionValue, InputActionState};
use crate::components::Entity;
use std::collections::HashSet;

/// Synchronize `FreeLookModifier` presence from a boolean `ActionState`.
///
/// When `state.triggered` is true, a `Bool(true)` inserts the marker and `Bool(false)` removes it.
/// When `vr_active` is true, VR wins and freelook mutations are skipped.
pub fn free_look_sync(
    actions: &InputActionState,
    freelook_action: ActionId,
    camera_entity: Entity,
    vr_active: bool,
    markers: &mut HashSet<Entity>,
) {
    let Some(state) = actions.get(freelook_action) else {
        return;
    };
    if !state.triggered {
        return;
    }
    if vr_active {
        return;
    }
    match state.value {
        ActionValue::Bool(true) => {
            markers.insert(camera_entity);
        }
        ActionValue::Bool(false) => {
            markers.remove(&camera_entity);
        }
        _ => {}
    }
}

/// Returns true when `entity` currently has the marker.
pub fn freelook_enabled(markers: &HashSet<Entity>, entity: Entity) -> bool {
    markers.contains(&entity)
}
