//! IR-2.5.5 AI grounded state derived from physics contact snapshots.

use glam::Vec3;

use crate::metrics::FallbackMetrics;
use crate::types::Entity;

/// Grounding snapshot consumed by AI navigation once per tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiGroundedState {
    /// `true` when a supporting floor contact exists for the agent.
    pub grounded: bool,
    /// World-space normal of the supporting surface.
    pub ground_normal: Vec3,
    /// Entity providing support (floor).
    pub ground_entity: Entity,
}

/// Single foot/floor contact emitted by the physics worker for an AI character.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FootContact {
    /// Unit-ish surface normal pointing out of the floor.
    pub ground_normal: Vec3,
    /// Floor entity id.
    pub ground_entity: Entity,
}

/// Reads grounding for the active agent from `contacts`, applying `FM-4` when data is missing.
///
/// - `None` means the physics worker did not publish a contact list for this frame (`FM-4`).
/// - `Some(&[])` means the agent is airborne with a published (empty) snapshot.
/// - `Some([..])` picks the contact whose `ground_normal` best aligns with world up (`+Y`).
#[must_use]
pub fn read_ai_grounded_state(
    contacts: Option<&[FootContact]>,
    prev: AiGroundedState,
    metrics: &mut FallbackMetrics,
) -> AiGroundedState {
    let Some(list) = contacts else {
        metrics.fm4_reuse_grounded += 1;
        return prev;
    };

    let Some(contact) = list.iter().max_by(|a, b| {
        let da = a.ground_normal.normalize_or_zero().dot(Vec3::Y);
        let db = b.ground_normal.normalize_or_zero().dot(Vec3::Y);
        da.total_cmp(&db)
    }) else {
        return airborne_state();
    };

    AiGroundedState {
        grounded: true,
        ground_normal: contact.ground_normal.normalize_or_zero(),
        ground_entity: contact.ground_entity,
    }
}

/// Returns an airborne grounded state (no supporting contacts).
#[must_use]
pub const fn airborne_state() -> AiGroundedState {
    AiGroundedState {
        grounded: false,
        ground_normal: Vec3::ZERO,
        ground_entity: Entity::NONE,
    }
}
