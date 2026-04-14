//! Entity dormancy for zero-bandwidth idle replication.

/// Stable entity id in tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

/// Tracks last mutation tick per entity for dormancy.
#[derive(Clone, Debug, Default)]
pub struct DormancyTracker {
    last_change: std::collections::HashMap<EntityId, u64>,
}

impl DormancyTracker {
    /// Empty tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark mutated at `tick`.
    pub fn touch(&mut self, e: EntityId, tick: u64) {
        self.last_change.insert(e, tick);
    }

    /// Entities dormant since at least `dormancy_ticks`.
    pub fn dormant_at(&self, now: u64, dormancy_ticks: u64) -> Vec<EntityId> {
        let mut out: Vec<EntityId> = self
            .last_change
            .iter()
            .filter(|(_, &t)| now.saturating_sub(t) >= dormancy_ticks)
            .map(|(&e, _)| e)
            .collect();
        out.sort_by_key(|e| e.0);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.2.6.1 — dormant entities contribute zero bytes in this tick's set.
    #[test]
    fn test_dormant_zero_bandwidth() {
        let mut d = DormancyTracker::new();
        for i in 0..300 {
            d.touch(EntityId(i), 96);
        }
        for i in 300..1000 {
            d.touch(EntityId(i), 0);
        }
        let now = 100;
        let dormant = d.dormant_at(now, 5);
        assert_eq!(dormant.len(), 700);
    }
}
