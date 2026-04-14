//! AI decision write-back into `EventLog<AiDecisionEntry>` (IR-2.2.6).

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::event_log::{DecayingEntry, EventLog};
use crate::ids::Entity;

/// Which subsystem produced a decision entry.
#[derive(Archive, Clone, Copy, Debug, Eq, Hash, PartialEq, RkyvDeserialize, RkyvSerialize)]
#[archive_attr(derive(bytecheck::CheckBytes, Debug, Eq, Hash, PartialEq))]
pub enum AiDecisionSource {
    /// Behavior tree leaf selected the action.
    BehaviorTree,
    /// Utility AI picked the highest-scored action.
    UtilityAi,
    /// GOAP planner committed to a plan step.
    GoapPlanner,
    /// Reactive override (e.g., flinch, flee).
    Reflex,
}

/// Single AI decision recorded for later recall.
#[derive(Archive, Clone, Debug, Eq, PartialEq, RkyvDeserialize, RkyvSerialize)]
#[archive_attr(derive(bytecheck::CheckBytes, Debug, Eq, PartialEq))]
pub struct AiDecisionEntry {
    /// Which AI system made the decision.
    pub source: AiDecisionSource,
    /// The chosen action (codegen discriminator).
    pub action: crate::ids::ActionId,
    /// Tick when the decision was made.
    pub decision_tick: u64,
    /// Optional target entity.
    pub target: Option<Entity>,
}

/// Pushes a decaying AI decision entry onto the owning entity's log.
pub fn write_ai_decision(
    log: &mut EventLog<AiDecisionEntry>,
    entry: AiDecisionEntry,
    current_tick: u64,
    source_entity: Entity,
) {
    let cell = DecayingEntry::new(entry, current_tick, Some(source_entity));
    log.push(cell);
}
