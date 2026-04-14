//! Minimal ECS-style stores for locomotion components (tests only).
//!
//! `HashMap` is acceptable here: this store is **cold-path test scaffolding** only, not a
//! deterministic simulation hot loop (`docs/design/constraints.md`).

use std::collections::HashMap;

/// Entity identifier.
pub type EntityId = u64;

/// Locomotion tuning profile.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocomotionProfile {
    /// Preferred walk speed in meters per second.
    pub walk_speed: f32,
}

/// Foot grouping metadata for gait.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FootGroup {
    /// Number of tracked feet.
    pub foot_count: u32,
}

/// Runtime gait state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GaitState {
    /// Current phase index.
    pub phase: u32,
}

/// Minimal world storing locomotion-related components.
#[derive(Clone, Debug, Default)]
pub struct LocomotionWorld {
    profiles: HashMap<EntityId, LocomotionProfile>,
    gaits: HashMap<EntityId, GaitState>,
    feet: HashMap<EntityId, FootGroup>,
}

impl LocomotionWorld {
    /// Inserts a locomotion profile.
    pub fn insert_profile(&mut self, e: EntityId, p: LocomotionProfile) {
        self.profiles.insert(e, p);
    }

    /// Inserts gait state.
    pub fn insert_gait(&mut self, e: EntityId, g: GaitState) {
        self.gaits.insert(e, g);
    }

    /// Inserts foot group.
    pub fn insert_feet(&mut self, e: EntityId, f: FootGroup) {
        self.feet.insert(e, f);
    }

    /// Queries `LocomotionProfile`.
    pub fn locomotion(&self, e: EntityId) -> Option<&LocomotionProfile> {
        self.profiles.get(&e)
    }

    /// Queries `GaitState`.
    pub fn gait(&self, e: EntityId) -> Option<&GaitState> {
        self.gaits.get(&e)
    }

    /// Queries `FootGroup`.
    pub fn feet(&self, e: EntityId) -> Option<&FootGroup> {
        self.feet.get(&e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_8_4_ecs() {
        let mut w = LocomotionWorld::default();
        w.insert_profile(1, LocomotionProfile { walk_speed: 1.5 });
        w.insert_gait(1, GaitState { phase: 0 });
        w.insert_feet(1, FootGroup { foot_count: 2 });
        assert!(w.locomotion(1).is_some());
        assert!(w.gait(1).is_some());
        assert!(w.feet(1).is_some());
    }
}
