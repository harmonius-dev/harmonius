//! Render-layer visibility filtering.

use crate::types::Entity;

/// Returns entities whose render-layer bitmask overlaps `camera_mask`.
///
/// When `camera_mask == 0`, callers should substitute `0xFFFF_FFFF` upstream; this
/// function treats the mask as authoritative.
#[must_use]
pub fn filter_visible_entities(camera_mask: u32, entities: &[(Entity, u32)]) -> Vec<Entity> {
    entities
        .iter()
        .filter(|(_, layers)| *layers & camera_mask != 0)
        .map(|(entity, _)| *entity)
        .collect()
}
