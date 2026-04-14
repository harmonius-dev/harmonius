//! Physics events consumed by the audio bridge.

use crate::ids::Entity;
use crate::math::Vec3;

/// Single contact sample on a manifold.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ContactPoint {
    /// World-space contact position.
    pub world_point: Vec3,
}

/// Collision began between two bodies.
#[derive(Clone, Debug, PartialEq)]
pub struct CollisionStarted {
    /// First body.
    pub entity_a: Entity,
    /// Second body.
    pub entity_b: Entity,
    /// Contact samples (non-empty for audible impacts in tests).
    pub contacts: Vec<ContactPoint>,
    /// Contact normal (unused by the current bridge; carried for completeness).
    pub normal: Vec3,
    /// Total normal impulse magnitude for thresholding.
    pub total_impulse: f32,
}

/// Collision persists across frames.
#[derive(Clone, Debug, PartialEq)]
pub struct CollisionPersisted {
    /// First body.
    pub entity_a: Entity,
    /// Second body.
    pub entity_b: Entity,
    /// Contact samples.
    pub contacts: Vec<ContactPoint>,
    /// Contact normal.
    pub normal: Vec3,
    /// Total normal impulse magnitude.
    pub total_impulse: f32,
    /// Tangential relative velocity used for sliding strength.
    pub tangential_velocity: Vec3,
}

impl CollisionPersisted {
    /// Returns the tangential velocity vector from the event payload.
    pub fn tangential_velocity(&self) -> Vec3 {
        self.tangential_velocity
    }
}

/// Collision contact ended.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CollisionEnded {
    /// First body.
    pub entity_a: Entity,
    /// Second body.
    pub entity_b: Entity,
}

/// Another body entered a trigger volume.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TriggerEnter {
    /// Trigger owner entity.
    pub trigger_entity: Entity,
    /// Overlapping entity.
    pub other_entity: Entity,
}

/// Another body exited a trigger volume.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TriggerExit {
    /// Trigger owner entity.
    pub trigger_entity: Entity,
    /// Overlapping entity.
    pub other_entity: Entity,
}
