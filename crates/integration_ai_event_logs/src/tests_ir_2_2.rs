//! Integration test coverage mapped from `ai-event-logs-test-cases.md`.

use rkyv::ser::serializers::AllocSerializer;
use rkyv::ser::Serializer;
use rkyv::Deserialize;

use crate::{
    apply_bt_event_memory_check, evaluate_threshold_trigger, score_event_log_consideration,
    write_ai_decision, ActionId, AiDecisionEntry, AiDecisionSource, Blackboard, BlackboardKey,
    BlackboardValue, BtEventMemoryCheck, DecayingEntry, Entity, EventLog, EventLogConsideration,
    EventLogDebugFlags, EventLogQuery, EventTypeId, GoapThreatBits, PredicateTable,
    PropagationBuffer, QueryContext, QueryWarnings, ResponseCurve, ThresholdChannel, ThresholdFired,
    ThresholdTrigger, TimeRange, THRESHOLD_CHANNEL_CAP,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HostileEvt(pub bool);

fn pred_hostile(e: &HostileEvt) -> bool {
    e.0
}

#[test]
fn tc_ir_2_2_r1_archive_ai_decision_roundtrip() {
    let v = AiDecisionEntry {
        source: AiDecisionSource::BehaviorTree,
        action: ActionId(7),
        decision_tick: 42,
        target: Some(Entity(99)),
    };
    let mut serializer = AllocSerializer::<256>::default();
    serializer.serialize_value(&v).expect("serialize");
    let bytes = serializer.into_serializer().into_inner();
    let archived = rkyv::check_archived_root::<AiDecisionEntry>(&bytes).expect("check root");
    let back: AiDecisionEntry = archived
        .deserialize(&mut rkyv::Infallible)
        .expect("deserialize");
    assert_eq!(v, back);
}

#[test]
fn tc_ir_2_2_1_1_bt_reads_hostile_memory() {
    let mut log: EventLog<HostileEvt> = EventLog::new(16);
    for t in 0..3u64 {
        let _ = log.push(DecayingEntry::new(HostileEvt(true), t, None));
    }
    let leaf = BtEventMemoryCheck {
        min_accuracy: 0.0,
        window_ticks: 10,
        predicate: 0,
        result_key: BlackboardKey(1),
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut bb = Blackboard::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 5,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    apply_bt_event_memory_check(&leaf, &log, &mut ctx, &mut bb);
    assert_eq!(bb.get(BlackboardKey(1)), Some(BlackboardValue::I32(3)));
}

#[test]
fn tc_ir_2_2_1_2_bt_reads_decayed_memory() {
    let mut log: EventLog<HostileEvt> = EventLog::new(16);
    let _ = log.push(DecayingEntry::new(HostileEvt(true), 0, None));
    log.decay_linear(1.0);
    let leaf = BtEventMemoryCheck {
        min_accuracy: 0.5,
        window_ticks: 10,
        predicate: 0,
        result_key: BlackboardKey(1),
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut bb = Blackboard::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 5,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    apply_bt_event_memory_check(&leaf, &log, &mut ctx, &mut bb);
    assert_eq!(bb.get(BlackboardKey(1)), Some(BlackboardValue::I32(0)));
}

#[test]
fn tc_ir_2_2_1_3_bt_reads_empty_log() {
    let log: EventLog<HostileEvt> = EventLog::new(4);
    let leaf = BtEventMemoryCheck {
        min_accuracy: 0.0,
        window_ticks: 10,
        predicate: 0,
        result_key: BlackboardKey(1),
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut bb = Blackboard::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 5,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    apply_bt_event_memory_check(&leaf, &log, &mut ctx, &mut bb);
    assert_eq!(bb.get(BlackboardKey(1)), Some(BlackboardValue::I32(0)));
}

#[test]
fn tc_ir_2_2_2_1_utility_scores_from_log() {
    let mut log: EventLog<HostileEvt> = EventLog::new(32);
    for _ in 0..5 {
        let _ = log.push(DecayingEntry::new(HostileEvt(true), 1, None));
    }
    let consideration = EventLogConsideration {
        query: EventLogQuery {
            time_range: None,
            min_accuracy: None,
            source: None,
            predicate: Some(0),
            event_type: None,
            max_results: 0,
        },
        curve: ResponseCurve { saturate_at: 10 },
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 2,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let score = score_event_log_consideration(&consideration, &log, &mut ctx);
    assert!((score - 0.5).abs() < 1e-6);
}

#[test]
fn tc_ir_2_2_2_2_utility_scores_zero_events() {
    let log: EventLog<HostileEvt> = EventLog::new(4);
    let consideration = EventLogConsideration {
        query: EventLogQuery {
            time_range: None,
            min_accuracy: None,
            source: None,
            predicate: Some(0),
            event_type: None,
            max_results: 0,
        },
        curve: ResponseCurve { saturate_at: 10 },
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 0,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let score = score_event_log_consideration(&consideration, &log, &mut ctx);
    assert_eq!(score, 0.0);
}

#[test]
fn tc_ir_2_2_3_1_goap_state_from_threshold() {
    let bits = GoapThreatBits::set_from_hostile_count(3, 3);
    assert!(bits.has_threat);
}

#[test]
fn tc_ir_2_2_3_2_goap_state_below_threshold() {
    let bits = GoapThreatBits::set_from_hostile_count(1, 3);
    assert!(!bits.has_threat);
}

#[test]
fn tc_ir_2_2_4_1_threshold_fires_alert() {
    let mut log: EventLog<HostileEvt> = EventLog::new(16);
    for t in 0..3u64 {
        let _ = log.push(DecayingEntry::new(HostileEvt(true), t, None));
    }
    let trigger = ThresholdTrigger {
        query: EventLogQuery {
            time_range: None,
            min_accuracy: None,
            source: None,
            predicate: Some(0),
            event_type: None,
            max_results: 0,
        },
        min_count: 3,
        window_ticks: 60,
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 10,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let fired = evaluate_threshold_trigger(Entity(1), 0, &trigger, &log, &mut ctx);
    assert_eq!(
        fired,
        Some(ThresholdFired {
            entity: Entity(1),
            trigger_index: 0,
            matched_count: 3,
        })
    );
}

#[test]
fn tc_ir_2_2_4_2_threshold_window_expires() {
    let mut log: EventLog<HostileEvt> = EventLog::new(16);
    let _ = log.push(DecayingEntry::new(HostileEvt(true), 0, None));
    let trigger = ThresholdTrigger {
        query: EventLogQuery {
            time_range: None,
            min_accuracy: None,
            source: None,
            predicate: Some(0),
            event_type: None,
            max_results: 0,
        },
        min_count: 3,
        window_ticks: 5,
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 100,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let fired = evaluate_threshold_trigger(Entity(1), 0, &trigger, &log, &mut ctx);
    assert_eq!(fired, None);
}

#[test]
fn tc_ir_2_2_5_1_gossip_updates_blackboard_after_flush() {
    let mut pb = PropagationBuffer::default();
    let e = Entity(7);
    pb.push(e, BlackboardKey(2), BlackboardValue::I32(9));
    let mut bbs = std::collections::BTreeMap::new();
    bbs.insert(e, Blackboard::new());
    pb.flush(&mut bbs);
    assert_eq!(
        bbs.get(&e).and_then(|b| b.get(BlackboardKey(2))),
        Some(BlackboardValue::I32(9))
    );
}

#[test]
fn tc_ir_2_2_5_3_flush_ordering_deterministic_last_wins() {
    let mut pb = PropagationBuffer::default();
    let e = Entity(1);
    pb.push(e, BlackboardKey(1), BlackboardValue::I32(1));
    pb.push(e, BlackboardKey(1), BlackboardValue::I32(2));
    let mut bbs = std::collections::BTreeMap::new();
    bbs.insert(e, Blackboard::new());
    pb.flush(&mut bbs);
    assert_eq!(
        bbs.get(&e).and_then(|b| b.get(BlackboardKey(1))),
        Some(BlackboardValue::I32(2))
    );
}

#[test]
fn tc_ir_2_2_6_1_ai_writes_decision_event() {
    let mut log: EventLog<AiDecisionEntry> = EventLog::new(8);
    let ent = Entity(5);
    write_ai_decision(
        &mut log,
        AiDecisionEntry {
            source: AiDecisionSource::BehaviorTree,
            action: ActionId(2),
            decision_tick: 10,
            target: None,
        },
        10,
        ent,
        None,
    );
    assert_eq!(log.len(), 1);
    let first = log.iter().next().expect("one entry");
    assert_eq!(first.data.action, ActionId(2));
}

#[test]
fn tc_ir_2_2_n1_same_tick_write_invisible_when_filtered() {
    let mut log: EventLog<AiDecisionEntry> = EventLog::new(8);
    let ent = Entity(5);
    write_ai_decision(
        &mut log,
        AiDecisionEntry {
            source: AiDecisionSource::BehaviorTree,
            action: ActionId(2),
            decision_tick: 10,
            target: None,
        },
        10,
        ent,
        None,
    );
    let q = EventLogQuery {
        time_range: None,
        min_accuracy: None,
        source: None,
        predicate: None,
        event_type: None,
        max_results: 0,
    };
    let preds = PredicateTable::<AiDecisionEntry>::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 10,
        hide_same_tick_writes: true,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let same = log.query(&q, &mut ctx);
    assert!(same.is_empty());
    warn = QueryWarnings::default();
    let mut ctx2 = QueryContext {
        read_tick: 11,
        hide_same_tick_writes: true,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let next = log.query(&q, &mut ctx2);
    assert_eq!(next.len(), 1);
}

#[test]
fn tc_ir_2_2_fm1_empty_log_fallback() {
    let log: EventLog<HostileEvt> = EventLog::new(2);
    let q = EventLogQuery {
        time_range: None,
        min_accuracy: None,
        source: None,
        predicate: None,
        event_type: None,
        max_results: 0,
    };
    let preds = PredicateTable::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 0,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let out = log.query(&q, &mut ctx);
    assert!(out.is_empty());
}

#[test]
fn tc_ir_2_2_fm4_predicate_mismatch_empty_no_panic() {
    let mut log: EventLog<HostileEvt> = EventLog::new(4);
    let _ = log.push(DecayingEntry::new(HostileEvt(true), 0, None));
    let q = EventLogQuery {
        time_range: None,
        min_accuracy: None,
        source: None,
        predicate: Some(0),
        event_type: None,
        max_results: 0,
    };
    let mut preds = PredicateTable::new();
    preds.register(0, |e: &HostileEvt| !e.0);
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 1,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let out = log.query(&q, &mut ctx);
    assert!(out.is_empty());
}

#[test]
fn tc_ir_2_2_fm7_missing_predicate_id() {
    let mut log: EventLog<HostileEvt> = EventLog::new(4);
    let _ = log.push(DecayingEntry::new(HostileEvt(true), 0, None));
    let q = EventLogQuery {
        time_range: None,
        min_accuracy: None,
        source: None,
        predicate: Some(99),
        event_type: None,
        max_results: 0,
    };
    let preds = PredicateTable::<HostileEvt>::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 1,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let out = log.query(&q, &mut ctx);
    assert!(out.is_empty());
    assert!(warn.missing_predicate);
}

#[test]
fn tc_ir_2_2_fm6_channel_overflow_drop() {
    let ch = ThresholdChannel::new();
    let mut delivered = 0u32;
    for i in 0..THRESHOLD_CHANNEL_CAP + 1 {
        let evt = ThresholdFired {
            entity: Entity(1),
            trigger_index: 0,
            matched_count: i as u32,
        };
        if ch.try_send(evt).is_ok() {
            delivered += 1;
        }
    }
    assert_eq!(delivered, THRESHOLD_CHANNEL_CAP as u32);
    let mut flags = EventLogDebugFlags {
        trace_thresholds: true,
        ..Default::default()
    };
    let drained = ch.drain_all_traced(&mut flags);
    assert_eq!(drained.len(), THRESHOLD_CHANNEL_CAP);
    assert_eq!(flags.threshold_nonempty_drains, 1);
}

#[test]
fn tc_ir_2_2_1_4_entries_helpers_match_query() {
    let mut log: EventLog<HostileEvt> = EventLog::new(8);
    let _ = log.push(DecayingEntry::new(HostileEvt(true), 2, None));
    let _ = log.push(DecayingEntry::new(HostileEvt(false), 8, None));
    let preds = PredicateTable::<HostileEvt>::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 10,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let by_acc = log.entries_above_accuracy(0.5, &mut ctx);
    assert_eq!(by_acc.len(), 2);
    warn = QueryWarnings::default();
    let mut ctx2 = QueryContext {
        read_tick: 10,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let window = TimeRange { start: 0, end: 5 };
    let by_win = log.entries_in_window(window, &mut ctx2);
    assert_eq!(by_win.len(), 1);
}

#[test]
fn tc_ir_2_2_d1_trace_query_increments_counter() {
    let mut log: EventLog<HostileEvt> = EventLog::new(4);
    let _ = log.push(DecayingEntry::new(HostileEvt(true), 0, None));
    let q = EventLogQuery {
        time_range: None,
        min_accuracy: None,
        source: None,
        predicate: None,
        event_type: None,
        max_results: 0,
    };
    let preds = PredicateTable::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags {
        trace_queries: true,
        ..Default::default()
    };
    let mut ctx = QueryContext {
        read_tick: 1,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let _ = log.query(&q, &mut ctx);
    assert_eq!(warn.query_trace_invocations, 1);
}

#[test]
fn tc_ir_2_2_3_3_goap_bits_from_logged_hostiles() {
    let mut log: EventLog<HostileEvt> = EventLog::new(16);
    for _ in 0..4 {
        let _ = log.push(DecayingEntry::new(HostileEvt(true), 1, None));
    }
    let q = EventLogQuery {
        time_range: None,
        min_accuracy: None,
        source: None,
        predicate: Some(0),
        event_type: None,
        max_results: 0,
    };
    let mut preds = PredicateTable::new();
    preds.register(0, pred_hostile);
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 2,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let n = log.query(&q, &mut ctx).len() as u32;
    let bits = GoapThreatBits::set_from_hostile_count(n, 3);
    assert!(bits.has_threat);
}

#[test]
fn tc_ir_2_2_event_type_filter() {
    let mut log: EventLog<HostileEvt> = EventLog::new(8);
    let _ = log.push(DecayingEntry::with_spatial(
        HostileEvt(true),
        1,
        None,
        None,
        Some(EventTypeId(2)),
    ));
    let _ = log.push(DecayingEntry::with_spatial(
        HostileEvt(true),
        1,
        None,
        None,
        Some(EventTypeId(7)),
    ));
    let q = EventLogQuery {
        time_range: None,
        min_accuracy: None,
        source: None,
        predicate: None,
        event_type: Some(EventTypeId(2)),
        max_results: 0,
    };
    let preds = PredicateTable::new();
    let mut warn = QueryWarnings::default();
    let mut flags = EventLogDebugFlags::default();
    let mut ctx = QueryContext {
        read_tick: 2,
        hide_same_tick_writes: false,
        predicates: &preds,
        warnings: &mut warn,
        flags: &mut flags,
    };
    let out = log.query(&q, &mut ctx);
    assert_eq!(out.len(), 1);
}

#[test]
fn tc_ir_2_2_fm3_push_eviction_returns_entry() {
    let mut log: EventLog<HostileEvt> = EventLog::new(1);
    let e0 = DecayingEntry::new(HostileEvt(true), 0, None);
    let e1 = DecayingEntry::new(HostileEvt(false), 1, None);
    assert!(log.push(e0).is_none());
    let evicted = log.push(e1);
    assert!(evicted.is_some());
    assert_eq!(evicted.map(|e| e.data.0), Some(true));
}
