use crate::animation_params::AnimationParams;
use crate::montage::ActiveMontage;
use crate::state_machine::combat_montage_placeholder;
use crate::StringId;

/// BT `MoveTo` leaf: writes locomotion speed into [`AnimationParams`] (TC-IR-1.1.1.1).
pub fn bt_move_to_speed(params: &mut AnimationParams, speed: f32) {
    params.speed = speed;
}

/// GOAP attack action: queues the `"attack"` trigger (TC-IR-1.1.1.2).
pub fn goap_write_attack_trigger(params: &mut AnimationParams) {
    params.triggers.push(StringId::from("attack"));
}

/// Sets crouch posture on animation parameters (TC-IR-1.1.1.3).
pub fn npc_set_crouch(params: &mut AnimationParams, crouch: bool) {
    params.is_crouching = crouch;
}

/// Writes aim pitch in degrees (TC-IR-1.1.1.4).
pub fn npc_aim_up(params: &mut AnimationParams, pitch_degrees: f32) {
    params.aim_pitch = pitch_degrees;
}

/// Navigation-style move command: speed + travel direction (TC-IR-1.1.4.x).
pub fn nav_write_move_params(params: &mut AnimationParams, speed: f32, direction_degrees: f32) {
    params.speed = speed;
    params.direction = direction_degrees;
}

/// Returns `(walk_weight, run_weight)` for 2D blend spaces (TC-IR-1.1.4.1).
#[must_use]
pub fn locomotion_blend_weights(speed: f32) -> (f32, f32) {
    let mut scratch = AnimationParams::default_idle();
    scratch.speed = speed;
    let sel = crate::AnimationStateMachine::locomotion_from_params(&scratch);
    if sel.state == crate::state_machine::STATE_IDLE {
        return (1.0, 0.0);
    }
    let run = sel.run_blend;
    (1.0 - run, run)
}

/// AI combat transition hook: returns a fresh montage component (TC-IR-1.1.3.1).
#[must_use]
pub fn ai_phase_insert_combat_montage() -> ActiveMontage {
    combat_montage_placeholder()
}
