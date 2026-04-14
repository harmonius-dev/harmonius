//! AI behavior ↔ animation integration primitives.
//!
//! Implements the data contracts and frame-local behaviors described in
//! `docs/design/integration/ai-animation.md` (IR-1.1.x, FM-1–FM-4).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod ai_actions;
mod animation_params;
mod budget;
mod diagnostics;
mod montage;
mod query;
mod state_machine;
mod string_id;

pub use ai_actions::{
    ai_phase_insert_combat_montage, bt_move_to_speed, goap_write_attack_trigger,
    locomotion_blend_weights, nav_write_move_params, npc_aim_up, npc_set_crouch,
};
pub use animation_params::AnimationParams;
pub use budget::{process_agents_with_budget, AgentEvalCost, AgentSlot, FrameBudget};
pub use diagnostics::Diagnostics;
pub use montage::{ActiveMontage, AssetHandle, MontageDef, MontageInstance, MontageState};
pub use query::AnimationQuery;
pub use state_machine::{
    filter_triggers, AnimationStateMachine, LocomotionSelection, STATE_IDLE, STATE_WALK_RUN_BLEND,
};
pub use string_id::StringId;

pub use glam::{Quat, Vec3};

#[cfg(test)]
mod tests;
