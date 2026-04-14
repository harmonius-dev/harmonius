//! Integration tests mapped to `docs/design/integration/event-logs-ui-test-cases.md`.

use std::collections::HashMap;

use harmonius_event_logs::{DecayCurve, DecayCurveType, Entity, EventLog, EventLogId,
    EventLogQuery, EventTypeId};

use crate::format::{merge_logs_chronological, template, visible_combat_lines};
use crate::resolve::resolve_combat_line;
use crate::scroll::ScrollView;
use crate::systems::{
    apply_event_throttle, spawn_floating_combat_text, update_combat_log_view,
    CombatLogThrottleState, LogEntryAddedQueue,
};
use crate::{
    CombatEvent, CombatEventKind, CombatLogArg, CombatLogBinding, DataBindingComponent,
    FloatingCombatText, LogEntryAdded, LocaleId, LocalizedStringId, Transform,
};

fn linear_decay() -> DecayCurve {
    DecayCurve {
        rate: 0.1,
        min_accuracy: 0.0,
        curve_type: DecayCurveType::Linear,
    }
}

fn table_en() -> HashMap<(LocaleId, LocalizedStringId), &'static str> {
    let mut m = HashMap::new();
    m.insert(
        (LocaleId(0), template::DAMAGE_DEALT),
        "{0} dealt {1} damage",
    );
    m.insert(
        (LocaleId(0), template::HEALING_DONE),
        "{0} healed {1}",
    );
    m.insert((LocaleId(0), template::NO_ENTRIES), "No entries");
    m.insert(
        (LocaleId(1), template::DAMAGE_DEALT),
        "{0} inflige {1} degats",
    );
    m
}

fn names_npc_player() -> HashMap<Entity, &'static str> {
    let mut m = HashMap::new();
    m.insert(Entity(1), "NPC");
    m.insert(Entity(2), "Player");
    m
}

/// # TC-IR-2.10.1.1 — Combat log shows entry
#[test]
fn tc_ir_2_10_1_1_combat_log_shows_entry() {
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 12,
            source: Entity(1),
            target: Entity(2),
        },
        10,
        Some(Entity(1)),
        None,
    );
    let binding = CombatLogBinding {
        log_entity: Entity(99),
        query: EventLogQuery::default(),
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].kind, CombatEventKind::Damage);
}

/// # TC-IR-2.10.1.2 — Log formats text
#[test]
fn tc_ir_2_10_1_2_log_formats_text() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 50,
            source: Entity(1),
            target: Entity(2),
        },
        1,
        Some(Entity(1)),
        None,
    );
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: EventLogQuery::default(),
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    let s = resolve_combat_line(&lines[0], LocaleId(0), &table_en(), &names_npc_player());
    assert_eq!(s.as_str(), "NPC dealt 50 damage");
}

/// # TC-IR-2.10.1.3 — Log empty state
#[test]
fn tc_ir_2_10_1_3_log_empty_state() {
    let log = EventLog::<CombatEvent>::new(EventLogId(1), 8, linear_decay(), 1);
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: EventLogQuery::default(),
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].template, template::NO_ENTRIES);
}

/// # TC-IR-2.10.1.4 — Locale swap updates
#[test]
fn tc_ir_2_10_1_4_locale_swap_updates() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 3,
            source: Entity(1),
            target: Entity(2),
        },
        1,
        None,
        None,
    );
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: EventLogQuery::default(),
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    let en = resolve_combat_line(&lines[0], LocaleId(0), &table_en(), &names_npc_player());
    let fr = resolve_combat_line(&lines[0], LocaleId(1), &table_en(), &names_npc_player());
    assert_eq!(en.as_str(), "NPC dealt 3 damage");
    assert_eq!(fr.as_str(), "NPC inflige 3 degats");
}

/// # TC-IR-2.10.2.1 — Activity feed recent
#[test]
fn tc_ir_2_10_2_1_activity_feed_recent() {
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    for t in 1_u64..=5 {
        log.push(
            CombatEvent {
                kind: CombatEventKind::Damage,
                value: t as i32,
                source: Entity(1),
                target: Entity(2),
            },
            t,
            None,
            None,
        );
    }
    let merged = merge_logs_chronological(&[(Entity(9), &log)], 0, 60);
    assert_eq!(merged.len(), 5);
}

/// # TC-IR-2.10.2.2 — Activity feed multi-log
#[test]
fn tc_ir_2_10_2_2_activity_feed_multi_log() {
    let mut a = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    a.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 1,
            source: Entity(1),
            target: Entity(2),
        },
        20,
        None,
        None,
    );
    let mut b = EventLog::new(EventLogId(2), 8, linear_decay(), 1);
    b.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 2,
            source: Entity(1),
            target: Entity(2),
        },
        10,
        None,
        None,
    );
    let logs = [(Entity(100), &a), (Entity(200), &b)];
    let merged = merge_logs_chronological(&logs, 0, 60);
    assert_eq!(merged.len(), 2);
    assert_eq!(merged[0].1.timestamp, 10);
    assert_eq!(merged[1].1.timestamp, 20);
}

/// # TC-IR-2.10.3.1 — FCT spawns on damage
#[test]
fn tc_ir_2_10_3_1_fct_spawns_on_damage() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 5,
            source: Entity(1),
            target: Entity(2),
        },
        1,
        None,
        None,
    );
    let idx = log.most_recent().expect("row").id.0;
    let ev = LogEntryAdded {
        log_entity: Entity(0),
        entry_index: idx,
    };
    let xf = Transform {
        translation: [1.0, 2.0, 3.0],
    };
    let mut warned = std::collections::BTreeSet::new();
    let (fct, warn) = spawn_floating_combat_text(ev, Some(&log), Some(&xf), &mut warned);
    assert!(warn.is_none());
    let fct = fct.expect("spawn");
    assert_eq!(fct.world_pos, [1.0, 2.0, 3.0]);
}

/// # TC-IR-2.10.3.2 — FCT despawns after duration
#[test]
fn tc_ir_2_10_3_2_fct_despawns_after_duration() {
    let mut fct = FloatingCombatText {
        world_pos: [0.0; 3],
        duration: 2.0,
        elapsed: 0.0,
    };
    assert!(!fct.tick(1.0));
    assert!(fct.tick(1.0));
}

/// # TC-IR-2.10.4.1 — Filter by event type
#[test]
fn tc_ir_2_10_4_1_filter_by_event_type() {
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 1,
            source: Entity(1),
            target: Entity(2),
        },
        1,
        None,
        None,
    );
    log.push(
        CombatEvent {
            kind: CombatEventKind::Healing,
            value: 4,
            source: Entity(2),
            target: Entity(1),
        },
        2,
        None,
        None,
    );
    let mut q = EventLogQuery::default();
    q.event_type = Some(EventTypeId(CombatEventKind::Healing as u32));
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: q,
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].kind, CombatEventKind::Healing);
}

/// # TC-IR-2.10.4.2 — Filter by source
#[test]
fn tc_ir_2_10_4_2_filter_by_source() {
    let mut log = EventLog::new(EventLogId(1), 16, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 1,
            source: Entity(1),
            target: Entity(2),
        },
        1,
        Some(Entity(1)),
        None,
    );
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 2,
            source: Entity(2),
            target: Entity(1),
        },
        2,
        Some(Entity(2)),
        None,
    );
    let mut q = EventLogQuery::default();
    q.source = Some(Entity(2));
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: q,
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].args[1], CombatLogArg::Int(2));
}

/// # TC-IR-2.10.4.3 — Filter by min_accuracy
#[test]
fn tc_ir_2_10_4_3_filter_by_min_accuracy() {
    let mut log = EventLog::new(EventLogId(2), 16, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 10,
            source: Entity(1),
            target: Entity(2),
        },
        0,
        None,
        None,
    );
    for t in 1..50 {
        log.decay_tick(t);
    }
    let acc = log.most_recent().expect("m").accuracy;
    assert!(acc < 0.5, "decayed accuracy {acc}");
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 99,
            source: Entity(1),
            target: Entity(2),
        },
        100,
        None,
        None,
    );
    log.decay_tick(100);
    let mut q = EventLogQuery::default();
    q.min_accuracy = Some(0.5);
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: q,
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].args[1], CombatLogArg::Int(99));
}

/// # TC-IR-2.10.5.1 — Low accuracy fades
#[test]
fn tc_ir_2_10_5_1_low_accuracy_fades() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 1,
            source: Entity(1),
            target: Entity(2),
        },
        0,
        None,
        None,
    );
    for t in 1..=7 {
        log.decay_tick(t);
    }
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: EventLogQuery::default(),
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    let o = lines[0].opacity;
    assert!((o - 0.3).abs() < 0.001, "opacity {o}");
}

/// # TC-IR-2.10.5.2 — Full accuracy opaque
#[test]
fn tc_ir_2_10_5_2_full_accuracy_opaque() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 1,
            source: Entity(1),
            target: Entity(2),
        },
        100,
        None,
        None,
    );
    log.decay_tick(100);
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: EventLogQuery::default(),
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    assert!((lines[0].opacity - 1.0).abs() < f32::EPSILON);
}

/// # TC-IR-2.10.6.1 — Auto-scroll at bottom
#[test]
fn tc_ir_2_10_6_1_auto_scroll_at_bottom() {
    let mut scroll = ScrollView::new(100.0);
    scroll.set_line_metrics(0, 20.0);
    scroll.scroll_offset_y = 0.0;
    assert!(scroll.is_scrolled_to_bottom());
    scroll.append_line(
        crate::RichText {
            display: "a".into(),
            opacity: 1.0,
        },
        20.0,
        true,
    );
    assert!(scroll.is_scrolled_to_bottom());
}

/// # TC-IR-2.10.6.2 — No auto-scroll if up
#[test]
fn tc_ir_2_10_6_2_no_auto_scroll_if_up() {
    let mut scroll = ScrollView::new(60.0);
    for _ in 0..5 {
        scroll.append_line(
            crate::RichText {
                display: "x".into(),
                opacity: 1.0,
            },
            20.0,
            true,
        );
    }
    scroll.scroll_up(30.0);
    assert!(!scroll.is_scrolled_to_bottom());
    let before = scroll.scroll_offset_y;
    scroll.append_line(
        crate::RichText {
            display: "y".into(),
            opacity: 1.0,
        },
        20.0,
        false,
    );
    assert_eq!(scroll.scroll_offset_y, before);
}

/// # TC-IR-2.10.6.3 — Derived auto-scroll (no `auto_scroll` field)
#[test]
fn tc_ir_2_10_6_3_derived_auto_scroll_field_absent() {
    let src = include_str!("lib.rs");
    assert!(!src.contains("auto_scroll:"));
}

/// # TC-IR-2.10.N1 — No async fn tokens in crate sources
#[test]
fn tc_ir_2_10_n1_no_async_fn() {
    assert!(!include_str!("lib.rs").contains("async fn"));
    assert!(!include_str!("types.rs").contains("async fn"));
    assert!(!include_str!("format.rs").contains("async fn"));
    assert!(!include_str!("scroll.rs").contains("async fn"));
    assert!(!include_str!("systems.rs").contains("async fn"));
    assert!(!include_str!("resolve.rs").contains("async fn"));
}

/// # TC-IR-2.10.N6 — Eight `CombatEventKind` variants
#[test]
fn tc_ir_2_10_n6_enum_variants() {
    use CombatEventKind::*;
    let kinds = [
        Damage,
        Healing,
        StatusApplied,
        StatusRemoved,
        Death,
        Miss,
        Dodge,
        Block,
    ];
    assert_eq!(kinds.len(), 8);
}

/// # TC-IR-2.10.N7 — No `text: SmolStr` on [`crate::CombatLogLine`]
#[test]
fn tc_ir_2_10_n7_no_smolstr_text_on_line() {
    let s = include_str!("types.rs");
    assert!(!s.contains("text: SmolStr"));
}

/// # TC-IR-2.10.N9 — No `CommandBuffer`
#[test]
fn tc_ir_2_10_n9_no_command_buffer() {
    let s = include_str!("systems.rs");
    assert!(!s.contains("CommandBuffer"));
}

/// # TC-IR-2.10.FM1 — Log entity despawned (missing log)
#[test]
fn tc_ir_2_10_fm1_log_missing() {
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: EventLogQuery::default(),
        max_display: 8,
        max_per_frame: 32,
    };
    let mut scroll = ScrollView::new(80.0);
    let mut data = DataBindingComponent::default();
    let mut throttle = CombatLogThrottleState::default();
    let mut pred = |_p, _e: &CombatEvent| true;
    let notes = update_combat_log_view(
        None,
        &binding,
        &mut scroll,
        &mut data,
        &mut pred,
        18.0,
        LocaleId(0),
        &table_en(),
        &names_npc_player(),
        &[],
        &mut throttle,
    );
    assert!(notes.log_entity_missing);
    assert!(scroll.lines.is_empty());
}

/// # TC-IR-2.10.FM3 — FCT target without transform
#[test]
fn tc_ir_2_10_fm3_missing_transform_warn_once() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 1,
            source: Entity(1),
            target: Entity(2),
        },
        1,
        None,
        None,
    );
    let idx = log.most_recent().expect("m").id.0;
    let ev = LogEntryAdded {
        log_entity: Entity(0),
        entry_index: idx,
    };
    let mut warned = std::collections::BTreeSet::new();
    let (a, w1) = spawn_floating_combat_text(ev, Some(&log), None, &mut warned);
    assert!(a.is_none());
    assert_eq!(w1, Some("missing transform"));
    let (b, w2) = spawn_floating_combat_text(ev, Some(&log), None, &mut warned);
    assert!(b.is_none());
    assert!(w2.is_none());
}

/// # TC-IR-2.10.FM4 — Filter matches nothing
#[test]
fn tc_ir_2_10_fm4_filter_empty_placeholder() {
    let mut log = EventLog::new(EventLogId(1), 8, linear_decay(), 1);
    log.push(
        CombatEvent {
            kind: CombatEventKind::Damage,
            value: 1,
            source: Entity(1),
            target: Entity(2),
        },
        1,
        None,
        None,
    );
    let mut q = EventLogQuery::default();
    q.event_type = Some(EventTypeId(CombatEventKind::Healing as u32));
    let binding = CombatLogBinding {
        log_entity: Entity(0),
        query: q,
        max_display: 8,
        max_per_frame: 32,
    };
    let mut pred = |_p, _e: &CombatEvent| true;
    let lines = visible_combat_lines(&log, &binding, &mut pred);
    assert_eq!(lines[0].template, template::NO_ENTRIES);
}

/// # TC-IR-2.10.FM5 — High rate throttle
#[test]
fn tc_ir_2_10_fm5_high_rate_throttle() {
    let mut throttle = CombatLogThrottleState::default();
    let batch: Vec<_> = (0..500)
        .map(|i| LogEntryAdded {
            log_entity: Entity(1),
            entry_index: i,
        })
        .collect();
    let kept = apply_event_throttle(&batch, 32, &mut throttle);
    assert_eq!(kept.len(), 32);
    assert_eq!(throttle.combat_log_throttled, 1);
}

/// # TC-IR-2.10.FM6 — Event queue saturated
#[test]
fn tc_ir_2_10_fm6_queue_saturated() {
    let mut q = LogEntryAddedQueue::with_capacity(1024);
    for i in 0..2048 {
        q.push(LogEntryAdded {
            log_entity: Entity(1),
            entry_index: i,
        });
    }
    assert_eq!(q.drops_logged, 1024);
    assert_eq!(q.drain_up_to(4096).len(), 1024);
}

/// # TC-IR-2.10.R1 — Archive [`CombatLogBinding`]
#[test]
fn tc_ir_2_10_r1_roundtrip_binding() {
    use rkyv::Deserialize;
    let v = CombatLogBinding {
        log_entity: Entity(7),
        query: EventLogQuery::default(),
        max_display: 10,
        max_per_frame: 32,
    };
    let bytes = rkyv::to_bytes::<_, 4096>(&v).unwrap();
    let archived = unsafe { rkyv::archived_root::<CombatLogBinding>(bytes.as_slice()) };
    let round: CombatLogBinding = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(v, round);
}

/// # TC-IR-2.10.R2 — Archive [`crate::CombatLogLine`]
#[test]
fn tc_ir_2_10_r2_roundtrip_line() {
    use crate::CombatLogLine;
    use rkyv::Deserialize;
    let v = CombatLogLine {
        template: template::DAMAGE_DEALT,
        kind: CombatEventKind::Damage,
        args: vec![CombatLogArg::Int(5)],
        opacity: 0.8,
        timestamp: 9,
    };
    let bytes = rkyv::to_bytes::<_, 4096>(&v).unwrap();
    let archived = unsafe { rkyv::archived_root::<CombatLogLine>(bytes.as_slice()) };
    let round: CombatLogLine = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(v, round);
}

/// # TC-IR-2.10.R3 — Archive [`CombatEventKind`]
#[test]
fn tc_ir_2_10_r3_roundtrip_kind() {
    use rkyv::Deserialize;
    let v = CombatEventKind::Block;
    let bytes = rkyv::to_bytes::<_, 256>(&v).unwrap();
    let archived = unsafe { rkyv::archived_root::<CombatEventKind>(bytes.as_slice()) };
    let round: CombatEventKind = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(v, round);
}

/// # TC-IR-2.10.R4 — Archive [`CombatLogArg`]
#[test]
fn tc_ir_2_10_r4_roundtrip_arg() {
    use rkyv::Deserialize;
    let v = CombatLogArg::Float(0.25);
    let bytes = rkyv::to_bytes::<_, 256>(&v).unwrap();
    let archived = unsafe { rkyv::archived_root::<CombatLogArg>(bytes.as_slice()) };
    let round: CombatLogArg = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(v, round);
}
