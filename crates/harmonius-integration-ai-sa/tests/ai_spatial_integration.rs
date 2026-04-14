//! Integration tests for `docs/design/integration/ai-spatial-awareness-test-cases.md`.

use std::collections::BTreeSet;

use harmonius_integration_ai_sa::{
    apply_hearing_perception, apply_sight_perception, awareness_blackboard_sync, evaluate_hearing,
    evaluate_sight, neutral_target_score, run_perception_budget_slice,
    run_perception_budget_slice_with_cursor, AiDecisionBudget, AiPerception, AiPerceptionBudget,
    AwarenessEntry, AwarenessLevel, AwarenessState, AwarenessTransitionEvent, Blackboard,
    BlackboardValue, Entity, HearingQueryInput, PerceivedEntity, PerceptionBudgetCursor,
    PerceptionFrameState, PerceptionSense, PropagationResultStore, ScoredTarget, SenseQueryResult,
    SightQueryInput, Vec3, AWARENESS_LEVEL_KEY, THREAT_POSITION_KEY, THREAT_TARGET_KEY,
};

fn forward_z() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}

#[test]
fn tc_ir_1_10_1_1_sight_in_cone() {
    let input = SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, 5.0),
        max_range: 20.0,
        half_angle_rad: f32::to_radians(45.0),
        line_of_sight_clear: true,
        occlusion_factor: 1.0,
    };
    let r = evaluate_sight(input);
    assert!(r.perceived, "expected target in cone with LOS");
    assert!(r.score > 0.0);
}

#[test]
fn tc_ir_1_10_1_2_sight_behind() {
    let input = SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, -5.0),
        max_range: 20.0,
        half_angle_rad: f32::to_radians(45.0),
        line_of_sight_clear: true,
        occlusion_factor: 1.0,
    };
    let r = evaluate_sight(input);
    assert!(!r.perceived);
}

#[test]
fn tc_ir_1_10_1_3_sight_wall_block() {
    let input = SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, 5.0),
        max_range: 20.0,
        half_angle_rad: f32::to_radians(45.0),
        line_of_sight_clear: false,
        occlusion_factor: 1.0,
    };
    let r = evaluate_sight(input);
    assert!(!r.perceived);
}

#[test]
fn tc_ir_1_10_1_4_sight_max_range() {
    let input = SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, 10.0),
        max_range: 10.0,
        half_angle_rad: f32::to_radians(45.0),
        line_of_sight_clear: true,
        occlusion_factor: 1.0,
    };
    let r = evaluate_sight(input);
    assert!(r.perceived, "at range limit target remains inside frustum");
    let near = evaluate_sight(SightQueryInput {
        offset_to_target: Vec3::new(0.0, 0.0, 2.0),
        ..input
    });
    assert!(
        near.score > r.score,
        "closer targets score higher (TC-IR-1.10.4.2)"
    );
}

#[test]
fn tc_ir_1_10_2_1_hearing_detects() {
    let mut store = PropagationResultStore::default();
    store.intensities.insert(Entity(1), 0.9);
    let heard = evaluate_hearing(
        &store,
        HearingQueryInput {
            source: Entity(1),
            threshold: 0.5,
        },
    );
    assert_eq!(heard, Some(0.9));
}

#[test]
fn tc_ir_1_10_2_2_hearing_blocked() {
    let mut store = PropagationResultStore::default();
    store.intensities.insert(Entity(1), 0.1);
    let heard = evaluate_hearing(
        &store,
        HearingQueryInput {
            source: Entity(1),
            threshold: 0.5,
        },
    );
    assert_eq!(heard, None);
}

#[test]
fn tc_ir_1_10_2_3_quiet_ignored() {
    let store = PropagationResultStore::default();
    let heard = evaluate_hearing(
        &store,
        HearingQueryInput {
            source: Entity(1),
            threshold: 0.5,
        },
    );
    assert_eq!(heard, None);
}

#[test]
fn tc_ir_1_10_3_1_alert_to_bb() {
    let mut awareness = AwarenessState::default();
    awareness.entries.push(AwarenessEntry {
        target: Entity(9),
        level: AwarenessLevel::Alert,
        score: 0.9,
        last_seen_position: Vec3::new(1.0, 0.0, 2.0),
        last_seen_tick: 1,
    });
    let mut bb = Blackboard::new();
    awareness_blackboard_sync(&awareness, &mut bb, true);
    assert_eq!(
        bb.get(AWARENESS_LEVEL_KEY),
        Some(&BlackboardValue::Int(2)),
        "Alert maps to BB int 2"
    );
}

#[test]
fn tc_ir_1_10_3_2_lost_clears() {
    let mut bb = Blackboard::new();
    bb.set(THREAT_TARGET_KEY, BlackboardValue::Entity(Entity(7)));
    bb.set(
        THREAT_POSITION_KEY,
        BlackboardValue::Vec3(Vec3::new(0.0, 1.0, 0.0)),
    );
    let mut awareness = AwarenessState::default();
    awareness.entries.push(AwarenessEntry {
        target: Entity(7),
        level: AwarenessLevel::Lost,
        score: 0.8,
        last_seen_position: Vec3::new(0.0, 1.0, 0.0),
        last_seen_tick: 2,
    });
    awareness_blackboard_sync(&awareness, &mut bb, true);
    assert_eq!(bb.get(THREAT_TARGET_KEY), None);
    assert_eq!(bb.get(THREAT_POSITION_KEY), None);
    assert_eq!(
        bb.get(AWARENESS_LEVEL_KEY),
        Some(&BlackboardValue::Int(4)),
        "Lost maps to BB int 4"
    );
}

#[test]
fn tc_ir_1_10_3_3_transition_evt() {
    let mut awareness = AwarenessState::default();
    let mut events = Vec::new();
    awareness.set_level(Entity(3), AwarenessLevel::Suspicious, 1, &mut events);
    events.clear();
    awareness.set_level(Entity(3), AwarenessLevel::Alert, 2, &mut events);
    assert_eq!(
        events,
        vec![AwarenessTransitionEvent {
            target: Entity(3),
            from: AwarenessLevel::Suspicious,
            to: AwarenessLevel::Alert,
        }]
    );
}

#[test]
fn tc_ir_1_10_4_1_top_score_threat() {
    let mut awareness = AwarenessState::default();
    awareness.entries.push(AwarenessEntry {
        target: Entity(1),
        level: AwarenessLevel::Tracking,
        score: 0.3,
        last_seen_position: Vec3::new(0.0, 0.0, 1.0),
        last_seen_tick: 1,
    });
    awareness.entries.push(AwarenessEntry {
        target: Entity(2),
        level: AwarenessLevel::Alert,
        score: 0.9,
        last_seen_position: Vec3::new(2.0, 0.0, 0.0),
        last_seen_tick: 1,
    });
    awareness.entries.push(AwarenessEntry {
        target: Entity(3),
        level: AwarenessLevel::Suspicious,
        score: 0.6,
        last_seen_position: Vec3::new(1.0, 1.0, 1.0),
        last_seen_tick: 1,
    });
    let top = awareness.highest_scored_target();
    assert_eq!(
        top,
        Some(ScoredTarget {
            entity: Entity(2),
            last_known_position: Vec3::new(2.0, 0.0, 0.0),
            score: 0.9,
        })
    );
    let mut bb = Blackboard::new();
    awareness_blackboard_sync(&awareness, &mut bb, true);
    assert_eq!(
        bb.get(THREAT_TARGET_KEY),
        Some(&BlackboardValue::Entity(Entity(2)))
    );
}

#[test]
fn tc_ir_1_10_4_3_occluded_penalized() {
    let clear = evaluate_sight(SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, 5.0),
        max_range: 20.0,
        half_angle_rad: f32::to_radians(45.0),
        line_of_sight_clear: true,
        occlusion_factor: 1.0,
    });
    let partial = evaluate_sight(SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, 5.0),
        max_range: 20.0,
        half_angle_rad: f32::to_radians(45.0),
        line_of_sight_clear: true,
        occlusion_factor: 0.4,
    });
    assert!(partial.score < clear.score);
}

#[test]
fn tc_ir_1_10_5_1_budget_limits() {
    let mut budget = AiPerceptionBudget {
        budget_micros: 2_000,
        spent_micros: 0,
    };
    let cost_each = 5_u32;
    let costs = vec![cost_each; 500];
    let outcomes = run_perception_budget_slice(&mut budget, &costs);
    let ran = outcomes.iter().filter(|&&o| o).count();
    assert!(
        ran < 500,
        "some agents must defer when the slice exceeds the budget"
    );
    assert!(ran > 0);
}

#[test]
fn tc_ir_1_10_5_2_deferred_stale() {
    let mut tracker = PerceptionFrameState::default();
    tracker.mark_ran(0, 10);
    assert!(!tracker.is_stale(0, 10));
    assert!(tracker.is_stale(0, 11));
}

#[test]
fn tc_ir_1_10_5_3_budget_split() {
    let mut perception_budget = AiPerceptionBudget::default();
    let mut decision_budget = AiDecisionBudget::default();
    assert!(perception_budget.try_consume(10));
    assert!(decision_budget.try_consume(10));
}

#[test]
fn tc_ir_1_10_5_4_round_robin_carry_over() {
    let costs = vec![600_u32, 600, 600];
    let mut cursor = PerceptionBudgetCursor::default();
    let mut budget = AiPerceptionBudget {
        budget_micros: 1_000,
        spent_micros: 0,
    };
    let first = run_perception_budget_slice_with_cursor(&mut budget, &costs, &mut cursor);
    assert_eq!(first, vec![true, false, false]);
    assert_eq!(cursor.next_agent, 1);
    let mut budget = AiPerceptionBudget {
        budget_micros: 1_000,
        spent_micros: 0,
    };
    let second = run_perception_budget_slice_with_cursor(&mut budget, &costs, &mut cursor);
    assert_eq!(second, vec![false, true, false]);
    assert_eq!(cursor.next_agent, 2);
}

#[test]
fn tc_ir_1_10_n_1_target_despawn() {
    let mut perception = AiPerception::new(0.5);
    perception.known_entities.push(PerceivedEntity {
        entity: Entity(4),
        last_seen_pos: Vec3::new(0.0, 0.0, 0.0),
        last_seen_time: 0.0,
        sense: PerceptionSense::Sight,
    });
    let alive = BTreeSet::new();
    perception.apply_memory_decay(10.0, &alive);
    assert!(
        perception.known_entities.is_empty(),
        "despawned targets must drop out after decay window"
    );
}

#[test]
fn tc_ir_1_10_n_1_memory_expires_while_alive() {
    let mut perception = AiPerception::new(0.5);
    perception.known_entities.push(PerceivedEntity {
        entity: Entity(4),
        last_seen_pos: Vec3::new(0.0, 0.0, 0.0),
        last_seen_time: 0.0,
        sense: PerceptionSense::Sight,
    });
    let mut alive = BTreeSet::new();
    alive.insert(Entity(4));
    perception.apply_memory_decay(1.0, &alive);
    assert!(
        perception.known_entities.is_empty(),
        "stale last_seen_time must drop even when the entity is still alive"
    );
}

#[test]
fn tc_ir_1_10_n_2_faction_missing() {
    let base = 0.55_f32;
    assert_eq!(neutral_target_score(base), base);
}

#[test]
fn tc_ir_1_10_n_3_propagation_missing() {
    let store = PropagationResultStore::default();
    let mut perception = AiPerception::new(1.0);
    let ok = apply_hearing_perception(
        &mut perception,
        &store,
        Vec3::new(0.0, 0.0, 3.0),
        0.0,
        HearingQueryInput {
            source: Entity(2),
            threshold: 0.1,
        },
    );
    assert!(!ok);
    assert!(!perception
        .known_entities
        .iter()
        .any(|p| p.sense == PerceptionSense::Hearing));
}

#[test]
fn tc_ir_1_10_n_4_sense_def_missing() {
    let mut perception = AiPerception::new(1.0);
    let mut warnings = 0_u32;
    let input = SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, 4.0),
        max_range: 10.0,
        half_angle_rad: f32::to_radians(60.0),
        line_of_sight_clear: true,
        occlusion_factor: 1.0,
    };
    let result = apply_sight_perception(
        &mut perception,
        Entity(8),
        Vec3::new(0.0, 0.0, 4.0),
        0.0,
        input,
        false,
        &mut warnings,
    );
    assert_eq!(
        result,
        SenseQueryResult {
            perceived: false,
            score: 0.0,
        }
    );
    assert_eq!(warnings, 1);
}

#[test]
fn tc_ir_1_10_n_5_empty_bvh() {
    let mut perception = AiPerception::new(5.0);
    perception.known_entities.push(PerceivedEntity {
        entity: Entity(5),
        last_seen_pos: Vec3::new(0.0, 0.0, 1.0),
        last_seen_time: 0.0,
        sense: PerceptionSense::Sight,
    });
    let before = perception.known_entities.clone();
    let mut warnings = 0_u32;
    let behind = SightQueryInput {
        agent_forward: forward_z(),
        offset_to_target: Vec3::new(0.0, 0.0, -2.0),
        max_range: 20.0,
        half_angle_rad: f32::to_radians(45.0),
        line_of_sight_clear: true,
        occlusion_factor: 1.0,
    };
    apply_sight_perception(
        &mut perception,
        Entity(99),
        Vec3::new(0.0, 0.0, -2.0),
        0.0,
        behind,
        true,
        &mut warnings,
    );
    assert_eq!(perception.known_entities, before);
}

#[test]
fn tc_ir_1_10_n_6_awareness_unchanged() {
    let awareness = AwarenessState::default();
    let mut bb = Blackboard::new();
    bb.set(AWARENESS_LEVEL_KEY, BlackboardValue::Int(3));
    awareness_blackboard_sync(&awareness, &mut bb, false);
    assert_eq!(bb.get(AWARENESS_LEVEL_KEY), Some(&BlackboardValue::Int(3)));
}
