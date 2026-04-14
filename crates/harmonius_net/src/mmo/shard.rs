//! Shard assignment and population-driven splits (`R-8.7.1`).

use std::collections::HashMap;
use std::collections::HashSet;

/// Opaque player / character identifier used by shard assignment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CharacterId(pub u64);

/// Logical world shard.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct ShardId(pub u32);

/// Snapshot of shard load used for split decisions (`TC-8.7.1.2`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shard {
    /// Shard this snapshot describes.
    pub id: ShardId,
    /// Current assigned population used for split heuristics.
    pub population: u32,
    /// Split triggers when `population > threshold`.
    pub threshold: u32,
}

impl Shard {
    /// Returns `true` when this shard should be split into two.
    pub fn should_split(&self) -> bool {
        self.population > self.threshold
    }

    /// Splits assignments in `manager` from [`Shard::id`], creating a new shard and moving ~50%.
    ///
    /// Updates [`Shard::population`] to match the post-split count on the source shard.
    pub fn split(&mut self, manager: &mut ShardManager) -> Result<ShardId, SplitError> {
        if !self.should_split() {
            return Err(SplitError::BelowThreshold);
        }
        let new_id = manager.allocate_successor_shard(self.id)?;
        manager.migrate_half_by_count(self.id, new_id);
        self.population = manager.population_of(self.id) as u32;
        Ok(new_id)
    }
}

/// Tracks which shard each character belongs to.
#[derive(Debug, Default)]
pub struct ShardManager {
    shards: HashSet<ShardId>,
    assignments: HashMap<CharacterId, ShardId>,
}

/// Failure modes for [`Shard::split`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplitError {
    /// Population did not exceed the configured threshold.
    BelowThreshold,
    /// No free shard id could be allocated (extremely unlikely in tests).
    NoShardId,
}

impl ShardManager {
    /// Creates a manager that can assign characters only onto the given shards.
    pub fn with_shards(shards: impl IntoIterator<Item = ShardId>) -> Self {
        let mut shards_set = HashSet::new();
        for s in shards {
            shards_set.insert(s);
        }
        Self {
            shards: shards_set,
            assignments: HashMap::new(),
        }
    }

    /// Assigns `character` to the least-populated shard (stable tie-break: lowest [`ShardId`]).
    pub fn assign_character(&mut self, character: CharacterId) -> ShardId {
        let mut best: Option<ShardId> = None;
        let mut best_pop = u32::MAX;
        let mut shard_ids: Vec<ShardId> = self.shards.iter().copied().collect();
        shard_ids.sort_unstable();
        for shard in shard_ids {
            let pop = self.population_of(shard) as u32;
            if pop < best_pop {
                best_pop = pop;
                best = Some(shard);
            }
        }
        let chosen = best.expect("with_shards must register at least one shard");
        self.assignments.insert(character, chosen);
        chosen
    }

    /// Returns the shard hosting `character`, if assigned.
    pub fn shard_of(&self, character: CharacterId) -> Option<ShardId> {
        self.assignments.get(&character).copied()
    }

    /// Number of characters assigned to `shard`.
    pub fn population_of(&self, shard: ShardId) -> usize {
        self.assignments.values().filter(|&&s| s == shard).count()
    }

    fn allocate_successor_shard(&mut self, source: ShardId) -> Result<ShardId, SplitError> {
        let max_id = self.shards.iter().map(|s| s.0).max().unwrap_or(source.0);
        let candidate = ShardId(max_id.saturating_add(1));
        if self.shards.contains(&candidate) {
            return Err(SplitError::NoShardId);
        }
        self.shards.insert(candidate);
        Ok(candidate)
    }

    fn migrate_half_by_count(&mut self, source: ShardId, destination: ShardId) {
        let on_source: Vec<CharacterId> = self
            .assignments
            .iter()
            .filter_map(|(&c, &s)| if s == source { Some(c) } else { None })
            .collect();
        let half = on_source.len() / 2;
        for c in on_source.into_iter().take(half) {
            self.assignments.insert(c, destination);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-8.7.1.1` `test_shard_assign_player`
    #[test]
    fn test_shard_assign_player() {
        let mut manager = ShardManager::with_shards([ShardId(1), ShardId(2), ShardId(3)]);
        let assigned = manager.assign_character(CharacterId(42));
        assert!(matches!(assigned, ShardId(1..=3)));
        assert_eq!(manager.shard_of(CharacterId(42)), Some(assigned));
        assert_eq!(manager.population_of(assigned), 1);
    }

    /// `TC-8.7.1.2` `test_shard_population_split`
    #[test]
    fn test_shard_population_split() {
        let mut manager = ShardManager::with_shards([ShardId(1)]);
        for i in 0..6000 {
            manager.assign_character(CharacterId(i));
        }
        assert_eq!(manager.population_of(ShardId(1)), 6000);

        let mut shard = Shard {
            id: ShardId(1),
            population: 6000,
            threshold: 5000,
        };
        assert!(shard.should_split());
        let new_id = shard.split(&mut manager).expect("split");
        assert_eq!(new_id, ShardId(2));
        assert_eq!(manager.population_of(ShardId(1)), 3000);
        assert_eq!(manager.population_of(ShardId(2)), 3000);
    }
}
