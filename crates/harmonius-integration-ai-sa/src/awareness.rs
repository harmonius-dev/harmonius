//! `AwarenessState`, levels, and blackboard synchronization.

use crate::blackboard::{
    Blackboard, BlackboardValue, AWARENESS_LEVEL_KEY, THREAT_POSITION_KEY, THREAT_TARGET_KEY,
};
use crate::types::{Entity, Vec3};

/// Ordinal mapping used by BT / GOAP conditions (`BlackboardValue::Int`).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AwarenessLevel {
    /// No knowledge of the target.
    Unaware,
    /// Faint signal detected, not confirmed.
    Suspicious,
    /// Target confirmed, actively responding.
    Alert,
    /// Maintaining contact with a known target.
    Tracking,
    /// Contact lost, searching or timing out.
    Lost,
}

impl AwarenessLevel {
    /// Stable `i32` tag written to the blackboard.
    pub fn to_bb_int(self) -> i32 {
        match self {
            Self::Unaware => 0,
            Self::Suspicious => 1,
            Self::Alert => 2,
            Self::Tracking => 3,
            Self::Lost => 4,
        }
    }
}

/// One tracked target inside [`AwarenessState`].
#[derive(Clone, Debug, PartialEq)]
pub struct AwarenessEntry {
    /// Target entity.
    pub target: Entity,
    /// Current level toward that target.
    pub level: AwarenessLevel,
    /// Accumulated perception score in `0.0..=1.0`.
    pub score: f32,
    /// Last known world position.
    pub last_seen_position: Vec3,
    /// Last simulation tick when the target was observed.
    pub last_seen_tick: u64,
}

/// Highest-scored threat candidate for blackboard export.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScoredTarget {
    /// Threat entity.
    pub entity: Entity,
    /// Last known position for the threat.
    pub last_known_position: Vec3,
    /// Raw score used for ranking.
    pub score: f32,
}

/// Describes an awareness transition relevant to gameplay / tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AwarenessTransitionEvent {
    /// Affected target.
    pub target: Entity,
    /// Previous level.
    pub from: AwarenessLevel,
    /// New level.
    pub to: AwarenessLevel,
}

/// Per-agent awareness aggregate (design `AwarenessState`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AwarenessState {
    /// Active awareness rows.
    pub entries: Vec<AwarenessEntry>,
}

impl AwarenessState {
    /// Highest [`AwarenessLevel`] across all entries, or [`AwarenessLevel::Unaware`] if empty.
    pub fn highest_level(&self) -> AwarenessLevel {
        self.entries
            .iter()
            .map(|e| e.level)
            .max()
            .unwrap_or(AwarenessLevel::Unaware)
    }

    /// Entry with maximum [`AwarenessEntry::score`].
    pub fn highest_scored_target(&self) -> Option<ScoredTarget> {
        self.entries
            .iter()
            .max_by(|a, b| a.score.total_cmp(&b.score))
            .map(|e| ScoredTarget {
                entity: e.target,
                last_known_position: e.last_seen_position,
                score: e.score,
            })
    }

    /// Forces a level transition on `target`, appending a transition event when the level changes.
    pub fn set_level(
        &mut self,
        target: Entity,
        new_level: AwarenessLevel,
        tick: u64,
        events: &mut Vec<AwarenessTransitionEvent>,
    ) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.target == target) {
            if entry.level != new_level {
                events.push(AwarenessTransitionEvent {
                    target,
                    from: entry.level,
                    to: new_level,
                });
                entry.level = new_level;
                entry.last_seen_tick = tick;
            }
        } else {
            self.entries.push(AwarenessEntry {
                target,
                level: new_level,
                score: 0.0,
                last_seen_position: Vec3::new(0.0, 0.0, 0.0),
                last_seen_tick: tick,
            });
            events.push(AwarenessTransitionEvent {
                target,
                from: AwarenessLevel::Unaware,
                to: new_level,
            });
        }
    }
}

/// Writes awareness-derived keys into `blackboard` when `awareness_changed` is true.
pub fn awareness_blackboard_sync(
    awareness: &AwarenessState,
    blackboard: &mut Blackboard,
    awareness_changed: bool,
) {
    if !awareness_changed {
        return;
    }
    blackboard.set(
        AWARENESS_LEVEL_KEY,
        BlackboardValue::Int(awareness.highest_level().to_bb_int()),
    );
    if let Some(target) = awareness.highest_scored_target() {
        blackboard.set(THREAT_TARGET_KEY, BlackboardValue::Entity(target.entity));
        blackboard.set(
            THREAT_POSITION_KEY,
            BlackboardValue::Vec3(target.last_known_position),
        );
    } else {
        blackboard.remove(THREAT_TARGET_KEY);
        blackboard.remove(THREAT_POSITION_KEY);
    }
}
