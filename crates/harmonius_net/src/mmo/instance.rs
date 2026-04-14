//! Dungeon / raid instancing (`R-8.7.1`).

use std::collections::HashMap;

/// Logical group running an instance together.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GroupId(pub u64);

/// High-level instance archetype.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceTemplate {
    /// Classic dungeon flow.
    Dungeon,
}

/// Tuning band for encounters.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Difficulty {
    /// Standard tuning.
    Normal,
    /// Harder tuning and loot.
    Heroic,
    /// Highest tuning.
    Mythic,
}

/// Encounter scaling band aligned to [`Difficulty`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scaling {
    /// Matches [`Difficulty::Normal`].
    Normal,
    /// Matches [`Difficulty::Heroic`].
    Heroic,
    /// Matches [`Difficulty::Mythic`].
    Mythic,
}

/// A single spawned encounter inside an instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encounter {
    /// Encounter tuning band.
    pub scaling: Scaling,
}

/// A running instance with attached encounters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instance {
    template: InstanceTemplate,
    difficulty: Difficulty,
    _group_id: GroupId,
    encounters: Vec<Encounter>,
}

impl Instance {
    /// Active difficulty for this instance.
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    /// Encounter list for validation / replay tooling.
    pub fn encounters(&self) -> &[Encounter] {
        &self.encounters
    }

    /// Template used to create this instance.
    pub fn template(&self) -> InstanceTemplate {
        self.template
    }
}

/// Factory and heroic lockout tracker (`TC-8.7.1.3`, `TC-8.7.18.1`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstanceManager {
    now_ticks: u64,
    lockout_duration_ticks: u64,
    active_until: HashMap<(GroupId, Difficulty), u64>,
}

impl Default for InstanceManager {
    fn default() -> Self {
        Self {
            now_ticks: 0,
            lockout_duration_ticks: 7 * 24 * 3600,
            active_until: HashMap::new(),
        }
    }
}

impl InstanceManager {
    /// Short lockout window for deterministic unit tests.
    pub fn with_lockout_duration_ticks(duration: u64) -> Self {
        Self {
            now_ticks: 0,
            lockout_duration_ticks: duration,
            active_until: HashMap::new(),
        }
    }

    /// Advances the logical clock used for lockouts.
    pub fn advance_time(&mut self, dt: u64) {
        self.now_ticks = self.now_ticks.saturating_add(dt);
    }

    /// Records completion and starts the configured lockout window.
    pub fn complete(&mut self, group_id: GroupId, difficulty: Difficulty) {
        let until = self
            .now_ticks
            .saturating_add(self.lockout_duration_ticks);
        self.active_until.insert((group_id, difficulty), until);
    }

    /// Returns `true` when the group may start this difficulty again.
    pub fn can_enter(&self, group_id: GroupId, difficulty: Difficulty) -> bool {
        match self.active_until.get(&(group_id, difficulty)) {
            Some(until) => self.now_ticks >= *until,
            None => true,
        }
    }

    /// Creates a new instance with deterministic placeholder encounters (`TC-8.7.1.3`).
    pub fn create(
        template: InstanceTemplate,
        difficulty: Difficulty,
        group_id: GroupId,
    ) -> Instance {
        let scaling = match difficulty {
            Difficulty::Normal => Scaling::Normal,
            Difficulty::Heroic => Scaling::Heroic,
            Difficulty::Mythic => Scaling::Mythic,
        };
        let encounters = vec![
            Encounter { scaling },
            Encounter { scaling },
        ];
        Instance {
            template,
            difficulty,
            _group_id: group_id,
            encounters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-8.7.1.3` `test_instance_difficulty_param`
    #[test]
    fn test_instance_difficulty_param() {
        let instance = InstanceManager::create(
            InstanceTemplate::Dungeon,
            Difficulty::Heroic,
            GroupId(7),
        );
        assert_eq!(instance.difficulty(), Difficulty::Heroic);
        assert!(instance
            .encounters()
            .iter()
            .all(|e| e.scaling == Scaling::Heroic));
    }

    /// `TC-8.7.18.1` `test_instance_lockout_timer`
    #[test]
    fn test_instance_lockout_timer() {
        let mut manager = InstanceManager::with_lockout_duration_ticks(100);
        let group = GroupId(99);
        assert!(manager.can_enter(group, Difficulty::Heroic));
        manager.complete(group, Difficulty::Heroic);
        assert!(!manager.can_enter(group, Difficulty::Heroic));
        manager.advance_time(100);
        assert!(manager.can_enter(group, Difficulty::Heroic));
    }
}
