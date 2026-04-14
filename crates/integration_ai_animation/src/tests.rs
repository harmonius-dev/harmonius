use smallvec::SmallVec;

use crate::{
    ai_phase_insert_combat_montage, bt_move_to_speed, goap_write_attack_trigger, locomotion_blend_weights,
    nav_write_move_params, npc_aim_up, npc_set_crouch, process_agents_with_budget, ActiveMontage,
    AgentEvalCost, AgentSlot, AnimationParams, AnimationQuery, AnimationStateMachine, AssetHandle,
    Diagnostics, FrameBudget, MontageInstance, MontageState, StringId, STATE_IDLE,
};

#[test]
fn tc_ir_1_1_1_1_bt_leaf_writes_speed() {
    let mut p = AnimationParams::default_idle();
    bt_move_to_speed(&mut p, 3.0);
    assert!((p.speed - 3.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_1_1_1_2_goap_writes_attack_trigger() {
    let mut p = AnimationParams::default_idle();
    goap_write_attack_trigger(&mut p);
    let attack = StringId::from("attack");
    assert!(p.triggers.iter().any(|t| *t == attack));
}

#[test]
fn tc_ir_1_1_1_3_npc_crouch() {
    let mut p = AnimationParams::default_idle();
    npc_set_crouch(&mut p, true);
    assert!(p.is_crouching);
}

#[test]
fn tc_ir_1_1_1_4_npc_aim_pitch() {
    let mut p = AnimationParams::default_idle();
    npc_aim_up(&mut p, 30.0);
    assert!((p.aim_pitch - 30.0).abs() < 1e-4);
}

#[test]
fn tc_ir_1_1_2_1_remaining_time_under_twenty_percent() {
    let mut sm = AnimationStateMachine::new_idle();
    sm.set_clip_timing(1.0, 0.85);
    assert!(AnimationQuery::remaining_time(&sm) < 0.2);
}

#[test]
fn tc_ir_1_1_2_2_ai_gates_on_transitioning() {
    let mut sm = AnimationStateMachine::new_idle();
    sm.set_transitioning(true);
    assert!(AnimationQuery::is_transitioning(&sm));
}

#[test]
fn tc_ir_1_1_2_3_phase4_reads_persisted_phase6_state() {
    let mut sm = AnimationStateMachine::new_idle();
    sm.set_clip_timing(2.0, 0.0);
    let persisted = AnimationQuery::current_state(&sm);
    assert_eq!(persisted, "idle");
    let mut p = AnimationParams::default_idle();
    p.speed = 2.5;
    let mut d = Diagnostics::default();
    let mut montage: Option<ActiveMontage> = None;
    sm.evaluate_frame(Some(&mut p), &mut d, &mut montage, 0.0);
    let after_anim = AnimationQuery::current_state(&sm);
    assert_eq!(after_anim, "walk_run_blend");
}

#[test]
fn tc_ir_1_1_3_1_combat_inserts_montage() {
    let m = ai_phase_insert_combat_montage();
    assert_eq!(m.instance.state, MontageState::Playing);
}

#[test]
fn tc_ir_1_1_3_2_montage_removed_when_finished() {
    let mut sm = AnimationStateMachine::new_idle();
    let mut m = ai_phase_insert_combat_montage();
    m.instance.duration_sec = 0.1;
    m.instance.elapsed_sec = 0.0;
    let mut p = AnimationParams::default_idle();
    let mut d = Diagnostics::default();
    let mut slot = Some(m);
    sm.evaluate_frame(Some(&mut p), &mut d, &mut slot, 0.2);
    assert!(slot.is_none());
}

#[test]
fn tc_ir_1_1_4_1_nav_speed_blend_weights() {
    let mut p = AnimationParams::default_idle();
    nav_write_move_params(&mut p, 2.5, 45.0);
    assert!((p.direction - 45.0).abs() < f32::EPSILON);
    let (walk, run) = locomotion_blend_weights(p.speed);
    assert!(walk > 0.0 && run > 0.0);
    assert!((walk + run - 1.0).abs() < 1e-4);
}

#[test]
fn tc_ir_1_1_4_2_zero_speed_idles() {
    let p = AnimationParams {
        speed: 0.0,
        direction: 0.0,
        is_grounded: true,
        is_crouching: false,
        is_jumping: false,
        is_falling: false,
        aim_pitch: 0.0,
        aim_yaw: 0.0,
        triggers: SmallVec::new(),
    };
    let sel = AnimationStateMachine::locomotion_from_params(&p);
    assert_eq!(sel.state, STATE_IDLE);
}

#[test]
fn tc_ir_1_1_5_1_five_hundred_agents_fit_two_ms_budget() {
    let per = AgentEvalCost {
        ai_ns: 2_000,
        anim_ns: 2_000,
    };
    let mut budget = FrameBudget::from_millis(2);
    let mut agents: Vec<AgentSlot> = (0..500)
        .map(|_| AgentSlot {
            params: AnimationParams::default_idle(),
        })
        .collect();
    let r = process_agents_with_budget(&mut budget, per, &mut agents, None);
    assert_eq!(r.evaluated, 500);
    assert_eq!(r.deferred, 0);
}

#[test]
fn tc_ir_1_1_e1_missing_params_warns_once_then_idle() {
    let mut sm = AnimationStateMachine::new_idle();
    let mut d = Diagnostics::default();
    let mut montage: Option<ActiveMontage> = None;
    sm.evaluate_frame(None, &mut d, &mut montage, 0.0);
    assert_eq!(d.warnings.len(), 1);
    sm.evaluate_frame(None, &mut d, &mut montage, 0.0);
    assert_eq!(d.warnings.len(), 1);
    assert_eq!(AnimationQuery::current_state(&sm), "idle");
}

#[test]
fn tc_ir_1_1_e2_invalid_trigger_removed_with_warn() {
    let mut sm = AnimationStateMachine::new_idle();
    let mut p = AnimationParams::default_idle();
    p.triggers.push(StringId::from("not_a_real_transition"));
    let mut d = Diagnostics::default();
    let mut montage: Option<ActiveMontage> = None;
    sm.evaluate_frame(Some(&mut p), &mut d, &mut montage, 0.0);
    assert!(p.triggers.is_empty());
    assert_eq!(d.warnings.len(), 1);
}

#[test]
fn tc_ir_1_1_e3_missing_montage_asset_removes_component() {
    let mut sm = AnimationStateMachine::new_idle();
    let m = ActiveMontage {
        instance: MontageInstance {
            montage: AssetHandle::new(0),
            duration_sec: 1.0,
            elapsed_sec: 0.0,
            blend_weight: 1.0,
            state: MontageState::Playing,
            notifies_fired_this_frame: Vec::new(),
        },
    };
    let mut p = AnimationParams::default_idle();
    let mut d = Diagnostics::default();
    let mut slot = Some(m);
    sm.evaluate_frame(Some(&mut p), &mut d, &mut slot, 0.0);
    assert!(slot.is_none());
    assert_eq!(d.errors.len(), 1);
}

#[test]
fn tc_ir_1_1_e4_budget_defers_tail_agents_preserves_params_and_debug() {
    let per = AgentEvalCost {
        ai_ns: 1_000,
        anim_ns: 1_000,
    };
    let mut budget = FrameBudget::from_millis(1);
    let mut agents: Vec<AgentSlot> = (0..1000)
        .map(|i| AgentSlot {
            params: AnimationParams {
                speed: i as f32,
                direction: 0.0,
                is_grounded: true,
                is_crouching: false,
                is_jumping: false,
                is_falling: false,
                aim_pitch: 0.0,
                aim_yaw: 0.0,
                triggers: SmallVec::new(),
            },
        })
        .collect();
    let mut d = Diagnostics::default();
    let r = process_agents_with_budget(&mut budget, per, &mut agents, Some(&mut d));
    assert!(r.evaluated < 1000);
    assert!(r.deferred > 0);
    let last = agents[999].params.speed;
    assert!((last - 999.0).abs() < f32::EPSILON);
    let msg = d.debug.join(" ");
    assert!(msg.contains("deferred"));
}

#[test]
fn montage_notify_fired_detects_frame_marker() {
    let mut m = ai_phase_insert_combat_montage();
    m.fire_notify_for_test("hit_window");
    assert!(AnimationQuery::montage_notify_fired(&m, "hit_window"));
}
