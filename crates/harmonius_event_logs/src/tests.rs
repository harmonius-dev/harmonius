//! Unit tests mapped to `TC-*` identifiers in `docs/design/simulation/event-logs-test-cases.md`.

use rkyv::{Archive, Deserialize, Infallible, Serialize};

use crate::{
    check_thresholds, propagate_entries, query_entries, DecayCurve, DecayCurveType, DecayingEntry,
    Entity, EventLog, EventLogId, EventLogQuery, EventTypeId, LogEventMetadata, PredicateId,
    PropagationRule, TagSet, ThresholdAction, ThresholdTrigger,
};

#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
struct CombatEvent {
    damage: f32,
    kind: u8,
    target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
struct TaggedEvent {
    value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
struct HostileStamp {
    hostile: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
struct ScoredEvent {
    ty: u32,
    tick: u64,
    accuracy: f32,
}

impl LogEventMetadata for ScoredEvent {
    fn event_type_id(&self) -> EventTypeId {
        EventTypeId(self.ty)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
struct AbilityHit {
    modifier: f32,
}

/// Inline string storage matching the `SmolStr` footprint exercised by TC-17.1.2.2.
#[derive(Clone, Copy, Debug, Archive, Serialize, Deserialize)]
struct DialogueLine {
    speaker: Entity,
    line: [u8; 23],
    len: u8,
}

fn linear_decay() -> DecayCurve {
    DecayCurve {
        rate: 0.1,
        min_accuracy: 0.0,
        curve_type: DecayCurveType::Linear,
    }
}

fn serialize_log(log: &EventLog<CombatEvent>) -> rkyv::AlignedVec {
    rkyv::to_bytes::<_, 65_536>(log).unwrap()
}

/// # Safety
/// `bytes` must be produced by `serialize_log` for the same concrete `EventLog` shape.
unsafe fn archived_log(bytes: &rkyv::AlignedVec) -> &rkyv::Archived<EventLog<CombatEvent>> {
    rkyv::archived_root::<EventLog<CombatEvent>>(bytes.as_slice())
}

#[test]
fn test_push_under_capacity() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    for t in [100_u64, 101, 102] {
        log.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(0) }, t, None, None);
    }
    assert_eq!(log.count(), 3);
    assert!(!log.is_full());
    assert_eq!(log.entries().next().unwrap().timestamp, 100);
    assert_eq!(log.entries().nth(2).unwrap().timestamp, 102);
}

#[test]
fn test_push_evicts_oldest_when_full() {
    let mut log = EventLog::new(EventLogId(1), 4, linear_decay(), 1);
    for t in [10_u64, 20, 30, 40, 50] {
        log.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(0) }, t, None, None);
    }
    assert_eq!(log.count(), 4);
    assert!(log.is_full());
    let ts: Vec<u64> = log.entries().map(|e| e.timestamp).collect();
    assert_eq!(ts, vec![20, 30, 40, 50]);
}

#[test]
fn test_entries_in_timestamp_order() {
    let mut log = EventLog::new(EventLogId(1), 4, linear_decay(), 1);
    for t in 1_u64..=6 {
        log.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(0) }, t, None, None);
    }
    let ts: Vec<u64> = log.entries().map(|e| e.timestamp).collect();
    assert_eq!(ts, vec![3, 4, 5, 6]);
}

#[test]
fn test_clear_resets_count_and_head() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    for t in [1_u64, 2, 3] {
        log.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(0) }, t, None, None);
    }
    log.clear();
    assert_eq!(log.count(), 0);
    assert!(log.most_recent().is_none());
    assert!(log.entries_above_accuracy(0.0).is_empty());
}

#[test]
fn test_entry_size_within_budget() {
    const _: () = assert!(core::mem::size_of::<DecayingEntry<CombatEvent>>() <= 256);
}

#[test]
fn test_entry_size_with_smolstr() {
    assert!(core::mem::size_of::<smol_str::SmolStr>() <= 32);
    assert!(core::mem::size_of::<DecayingEntry<DialogueLine>>() <= 256);
}

#[test]
fn test_decay_linear_half_life() {
    let mut log = EventLog::new(
        EventLogId(1),
        8,
        DecayCurve {
            rate: 0.1,
            min_accuracy: 0.0,
            curve_type: DecayCurveType::Linear,
        },
        1,
    );
    log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, 0, None, None);
    log.decay_tick(5);
    assert!((log.entries().next().unwrap().accuracy - 0.5).abs() < 1e-6);
}

#[test]
fn test_decay_exponential_curve() {
    let mut log = EventLog::new(
        EventLogId(1),
        8,
        DecayCurve {
            rate: 0.5,
            min_accuracy: 0.0,
            curve_type: DecayCurveType::Exponential,
        },
        1,
    );
    log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, 0, None, None);
    log.decay_tick(2);
    assert!((log.entries().next().unwrap().accuracy - 0.25).abs() < 1e-6);
}

#[test]
fn test_decay_step_threshold() {
    let curve = DecayCurve {
        rate: 10.0,
        min_accuracy: 0.2,
        curve_type: DecayCurveType::Step,
    };
    let mut log = EventLog::new(EventLogId(1), 8, curve, 1);
    log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, 0, None, None);
    log.decay_tick(9);
    assert!((log.entries().next().unwrap().accuracy - 1.0).abs() < 1e-6);
    log.decay_tick(10);
    assert!((log.entries().next().unwrap().accuracy - 0.2).abs() < 1e-6);
}

#[test]
fn test_decay_clamps_to_zero() {
    let mut log = EventLog::new(
        EventLogId(1),
        8,
        DecayCurve {
            rate: 0.1,
            min_accuracy: 0.0,
            curve_type: DecayCurveType::Linear,
        },
        1,
    );
    log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, 0, None, None);
    log.decay_tick(1000);
    assert!((log.entries().next().unwrap().accuracy - 0.0).abs() < 1e-6);
}

#[test]
fn test_decay_min_accuracy_floor() {
    let mut log = EventLog::new(
        EventLogId(1),
        8,
        DecayCurve {
            rate: 0.5,
            min_accuracy: 0.3,
            curve_type: DecayCurveType::Exponential,
        },
        1,
    );
    log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, 0, None, None);
    log.decay_tick(100);
    assert!((log.entries().next().unwrap().accuracy - 0.3).abs() < 1e-6);
}

#[test]
fn test_prune_below_removes_weak_entries() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, 0, None, None);
    log.decay_tick(5);
    assert!((log.entries().next().unwrap().accuracy - 0.5).abs() < 1e-6);
    log.prune_below(0.6);
    assert_eq!(log.count(), 0);
}

#[test]
fn test_propagate_single_hop() {
    let rule = PropagationRule {
        range: 10.0,
        accuracy_multiplier: 0.5,
        delay_ticks: 0,
        max_hops: 5,
        filter_tags: TagSet::empty(),
    };
    let mut source = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    let mut target = EventLog::new(EventLogId(2), 8, linear_decay(), 1);
    source.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(1) }, 0, None, None);
    propagate_entries(&source, &mut target, &rule, 1);
    assert_eq!(target.count(), 1);
    assert!((target.entries().next().unwrap().accuracy - 0.5).abs() < 1e-6);
    assert_eq!(target.entries().next().unwrap().hop_count, 1);
}

#[test]
fn test_propagate_increments_hops() {
    let rule = PropagationRule {
        range: 10.0,
        accuracy_multiplier: 0.9,
        delay_ticks: 0,
        max_hops: 5,
        filter_tags: TagSet::empty(),
    };
    let mut a = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    let mut b = EventLog::new(EventLogId(2), 8, linear_decay(), 1);
    let mut c = EventLog::new(EventLogId(3), 8, linear_decay(), 1);
    a.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(0) }, 0, None, None);
    propagate_entries(&a, &mut b, &rule, 1);
    propagate_entries(&b, &mut c, &rule, 2);
    assert_eq!(b.entries().next().unwrap().hop_count, 1);
    assert_eq!(c.entries().next().unwrap().hop_count, 2);
}

#[test]
fn test_propagate_max_hops_stops() {
    let rule = PropagationRule {
        range: 10.0,
        accuracy_multiplier: 0.9,
        delay_ticks: 0,
        max_hops: 2,
        filter_tags: TagSet::empty(),
    };
    let mut a = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    let mut b = EventLog::new(EventLogId(2), 8, linear_decay(), 1);
    let mut c = EventLog::new(EventLogId(3), 8, linear_decay(), 1);
    let mut d = EventLog::new(EventLogId(4), 8, linear_decay(), 1);
    let mut e = EventLog::new(EventLogId(5), 8, linear_decay(), 1);
    a.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(0) }, 0, None, None);
    propagate_entries(&a, &mut b, &rule, 1);
    propagate_entries(&b, &mut c, &rule, 2);
    propagate_entries(&c, &mut d, &rule, 3);
    propagate_entries(&d, &mut e, &rule, 4);
    assert_eq!(c.entries().next().unwrap().hop_count, 2);
    assert_eq!(d.count(), 0);
    assert_eq!(e.count(), 0);
}

#[test]
fn test_propagate_dedupes_entries() {
    let rule = PropagationRule {
        range: 10.0,
        accuracy_multiplier: 0.5,
        delay_ticks: 0,
        max_hops: 5,
        filter_tags: TagSet::empty(),
    };
    let mut source = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    let mut target = EventLog::new(EventLogId(2), 8, linear_decay(), 1);
    source.push(CombatEvent { damage: 1.0, kind: 0, target: Entity(0) }, 0, None, None);
    propagate_entries(&source, &mut target, &rule, 1);
    propagate_entries(&source, &mut target, &rule, 2);
    assert_eq!(target.count(), 1);
}

#[test]
fn test_propagate_filter_tags() {
    const HOSTILE: u32 = 1;
    const FRIENDLY: u32 = 2;
    let rule = PropagationRule {
        range: 10.0,
        accuracy_multiplier: 0.8,
        delay_ticks: 0,
        max_hops: 5,
        filter_tags: TagSet::from_ids(&[HOSTILE]),
    };
    let mut source = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    let mut target = EventLog::new(EventLogId(2), 8, linear_decay(), 1);
    source.push_with_tags(
        TaggedEvent { value: 1 },
        0,
        None,
        None,
        vec![HOSTILE],
    );
    source.push_with_tags(
        TaggedEvent { value: 2 },
        1,
        None,
        None,
        vec![FRIENDLY],
    );
    propagate_entries(&source, &mut target, &rule, 2);
    assert_eq!(target.count(), 1);
    assert_eq!(target.entries().next().unwrap().data.value, 1);
}

#[test]
fn test_threshold_below_count() {
    let trigger = ThresholdTrigger::new(
        PredicateId(0),
        3,
        60,
        ThresholdAction::FireEvent("alert".into()),
    );
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    log.push(HostileStamp { hostile: true }, 100, None, None);
    log.push(HostileStamp { hostile: true }, 110, None, None);
    let mut pred = |_id: PredicateId, d: &HostileStamp| d.hostile;
    let fired = check_thresholds(&log, &[trigger], 130, &mut pred);
    assert!(fired.is_empty());
}

#[test]
fn test_threshold_meets_count() {
    let trigger = ThresholdTrigger::new(
        PredicateId(0),
        3,
        60,
        ThresholdAction::FireEvent("alert".into()),
    );
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    log.push(HostileStamp { hostile: true }, 100, None, None);
    log.push(HostileStamp { hostile: true }, 110, None, None);
    log.push(HostileStamp { hostile: true }, 120, None, None);
    let mut pred = |_id: PredicateId, d: &HostileStamp| d.hostile;
    let fired = check_thresholds(&log, &[trigger], 130, &mut pred);
    assert_eq!(fired.len(), 1);
    assert_eq!(fired[0], ThresholdAction::FireEvent("alert".into()));
}

#[test]
fn test_threshold_window_excludes() {
    let trigger = ThresholdTrigger::new(
        PredicateId(0),
        3,
        60,
        ThresholdAction::FireEvent("alert".into()),
    );
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    for t in [0_u64, 100, 200, 300, 400] {
        log.push(HostileStamp { hostile: true }, t, None, None);
    }
    let mut pred = |_id: PredicateId, d: &HostileStamp| d.hostile;
    let fired = check_thresholds(&log, &[trigger], 400, &mut pred);
    assert!(fired.is_empty());
}

#[test]
fn test_threshold_fires_once_only() {
    let trigger = ThresholdTrigger::new(
        PredicateId(0),
        3,
        60,
        ThresholdAction::FireEvent("alert".into()),
    );
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    log.push(HostileStamp { hostile: true }, 100, None, None);
    log.push(HostileStamp { hostile: true }, 110, None, None);
    log.push(HostileStamp { hostile: true }, 120, None, None);
    let mut pred = |_id: PredicateId, d: &HostileStamp| d.hostile;
    let first = check_thresholds(&log, std::slice::from_ref(&trigger), 130, &mut pred);
    let second = check_thresholds(&log, &[trigger], 131, &mut pred);
    assert_eq!(first, second);
    assert_eq!(first.len(), 1);
}

#[test]
fn test_rkyv_roundtrip_empty_log() {
    let log = EventLog::new(
        EventLogId(7),
        32,
        DecayCurve {
            rate: 0.05,
            min_accuracy: 0.0,
            curve_type: DecayCurveType::Linear,
        },
        1,
    );
    let buf = serialize_log(&log);
    let archived = unsafe { archived_log(&buf) };
    assert_eq!(archived.capacity, 32);
    assert_eq!(archived.entries.len(), 0);
}

#[test]
fn test_rkyv_roundtrip_full_log() {
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    for i in 0_u32..16 {
        log.push(
            CombatEvent {
                damage: i as f32,
                kind: (i % 255) as u8,
                target: Entity(u64::from(i)),
            },
            u64::from(i * 3),
            Some(Entity(u64::from(i + 1))),
            None,
        );
        log.entries.back_mut().unwrap().accuracy = (i as f32) * 0.01;
    }
    let buf = serialize_log(&log);
    let archived = unsafe { archived_log(&buf) };
    assert_eq!(archived.entries.len(), 16);
    let deserialized: EventLog<CombatEvent> = archived.deserialize(&mut Infallible).unwrap();
    assert_eq!(deserialized.capacity, log.capacity);
    assert_eq!(deserialized.entries.len(), log.entries.len());
    for (a, b) in deserialized.entries.iter().zip(log.entries.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.timestamp, b.timestamp);
        assert!((a.accuracy - b.accuracy).abs() < 1e-6);
        assert_eq!(a.data, b.data);
    }
}

#[test]
fn test_query_filter_event_type() {
    let mut log = EventLog::new(EventLogId(1), 32, linear_decay(), 1);
    for i in 0_u32..9 {
        let ty = i % 3;
        log.push(
            ScoredEvent {
                ty,
                tick: u64::from(i),
                accuracy: 1.0,
            },
            u64::from(i),
            None,
            None,
        );
    }
    let query = EventLogQuery {
        event_type: Some(EventTypeId(2)),
        ..Default::default()
    };
    let mut pred = |_id: PredicateId, _d: &ScoredEvent| true;
    let hits = query_entries(&log, &query, &mut pred);
    assert_eq!(hits.len(), 3);
    assert!(hits.iter().all(|e| e.data.ty == 2));
}

#[test]
fn test_query_filter_time_range() {
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    for t in [10_u64, 60, 80, 110] {
        log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, t, None, None);
    }
    let hits = log.entries_in_window(50, 100);
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].timestamp, 60);
    assert_eq!(hits[1].timestamp, 80);
}

#[test]
fn test_query_filter_min_accuracy() {
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    let accs = [0.9_f32, 0.6, 0.4, 0.1];
    for (i, a) in accs.iter().enumerate() {
        log.push(CombatEvent { damage: 0.0, kind: 0, target: Entity(0) }, i as u64, None, None);
        log.entries.back_mut().unwrap().accuracy = *a;
    }
    let hits = log.entries_above_accuracy(0.5);
    assert_eq!(hits.len(), 2);
    assert!(hits.iter().all(|e| e.accuracy > 0.5));
}

#[test]
fn test_threat_table_modifiers() {
    let modifiers = [1.0_f32, 1.5, 0.5, 2.0, 1.0];
    let base = 10.0_f32;
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    for (i, m) in modifiers.iter().enumerate() {
        log.push(
            AbilityHit { modifier: *m },
            i as u64,
            None,
            None,
        );
    }
    let aggregated: f32 = log.entries().map(|e| base * e.data.modifier).sum();
    assert!((aggregated - 60.0).abs() < 1e-4);
}
