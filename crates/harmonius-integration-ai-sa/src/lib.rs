//! Deterministic integration helpers for AI perception and spatial awareness.
//!
//! This crate is a **contract-first slice**: pure helpers and types for CI tests. ECS systems,
//! BVH queries, LOS raycasts, worker handoff, and scheduling live in future engine wiring.
//!
//! These types and functions encode the contracts from
//! `docs/design/integration/ai-spatial-awareness.md` for CI-runnable tests.

#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod awareness;
mod blackboard;
mod budget;
mod perception;
mod types;

pub use awareness::{
    awareness_blackboard_sync, AwarenessEntry, AwarenessLevel, AwarenessState,
    AwarenessTransitionEvent, ScoredTarget,
};
pub use blackboard::{
    Blackboard, BlackboardValue, AWARENESS_LEVEL_KEY, THREAT_POSITION_KEY, THREAT_TARGET_KEY,
};
pub use budget::{
    run_perception_budget_slice, run_perception_budget_slice_with_cursor, AiDecisionBudget,
    AiPerceptionBudget, PerceptionBudgetCursor, PerceptionFrameState,
};
pub use perception::{
    apply_hearing_perception, apply_sight_perception, evaluate_hearing, evaluate_sight,
    neutral_target_score, AiPerception, HearingQueryInput, PerceivedEntity, PerceptionSense,
    PropagationResultStore, SenseQueryResult, SightQueryInput,
};
pub use types::{Entity, Vec3};
