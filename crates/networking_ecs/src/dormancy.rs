//! Dormant entity tracking for bandwidth skipping.

use crate::ids::Entity;
use crate::ids::SequenceTick;

/// Tracks which entities are dormant (no replication bandwidth).
#[derive(Debug)]
pub struct DormancyManager {
    dormant: Vec<Entity>,
    threshold_ticks: u32,
}

impl DormancyManager {
    /// Creates a manager with the given dormancy threshold.
    #[must_use]
    pub fn new(threshold_ticks: u32) -> Self {
        Self {
            dormant: Vec::new(),
            threshold_ticks,
        }
    }

    /// Returns true when `(current - last_change) >= threshold`.
    #[must_use]
    pub fn check_dormancy(
        &self,
        entity: Entity,
        last_change_tick: SequenceTick,
        current_tick: SequenceTick,
    ) -> bool {
        let _ = entity;
        let delta = current_tick.0.saturating_sub(last_change_tick.0);
        delta >= self.threshold_ticks
    }

    /// Marks `entity` dormant (sorted insertion, deduped).
    pub fn mark_dormant(&mut self, entity: Entity) {
        match self.dormant.binary_search_by_key(&entity, |e| *e) {
            Ok(_) => {}
            Err(idx) => self.dormant.insert(idx, entity),
        }
    }

    /// Clears dormancy for `entity` if present.
    pub fn wake(&mut self, entity: Entity) {
        if let Ok(idx) = self.dormant.binary_search_by_key(&entity, |e| *e) {
            self.dormant.remove(idx);
        }
    }

    /// Returns whether `entity` is currently dormant.
    #[must_use]
    pub fn is_dormant(&self, entity: Entity) -> bool {
        self.dormant.binary_search_by_key(&entity, |e| *e).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_4_4_6_1_entity_becomes_dormant() {
        let mgr = DormancyManager::new(5);
        assert!(mgr.check_dormancy(Entity(1), SequenceTick(0), SequenceTick(5)));
    }

    #[test]
    fn tc_ir_4_4_6_2_wake_clears_dormancy() {
        let mut mgr = DormancyManager::new(1);
        mgr.mark_dormant(Entity(2));
        assert!(mgr.is_dormant(Entity(2)));
        mgr.wake(Entity(2));
        assert!(!mgr.is_dormant(Entity(2)));
    }

    #[test]
    fn tc_ir_4_4_6_n1_wake_unknown_is_noop() {
        let mut mgr = DormancyManager::new(3);
        mgr.wake(Entity(99));
        assert!(!mgr.is_dormant(Entity(99)));
    }
}
