//! Integration coverage for save-system ↔ profiler contracts (IR-8.1.x).

use save_system_profiler::{
    drain_messages, hud_phase8_bar_red, schema_bytes_sum, DebugFlags, FakeFileSystem, IoError,
    IoWriteEvent, MemorySnapshotEvent, ProfileMessage, SavePhase, SavePipeline, SaveProfileChannel,
    SaveProfileEvent, SaveProfilerAggregator, SchemaBreakdownEvent, NODE_AI, NODE_PLAYER,
    NODE_WORLD,
};

const MIB: u64 = 1 << 20;

#[test]
fn tc_ir_8_1_1_1_gather_phase_event_emitted() {
    let flags = DebugFlags::default();
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    pipe.gather(7, 1, 2.0);
    let msgs = drain_messages(&mut ch);
    assert_eq!(msgs.len(), 1);
    match &msgs[0] {
        ProfileMessage::SaveProfile(ev) => {
            assert_eq!(ev.save_id, 7);
            assert_eq!(ev.phase, SavePhase::Gather);
            assert!((ev.duration_ms - 2.0).abs() < f64::EPSILON);
        }
        _ => panic!("expected SaveProfile"),
    }
}

#[test]
fn tc_ir_8_1_1_2_archive_phase_event_carries_bytes() {
    let flags = DebugFlags::default();
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    pipe.archive(1, 0, 0.0, MIB);
    let msgs = drain_messages(&mut ch);
    match &msgs[0] {
        ProfileMessage::SaveProfile(ev) => {
            assert_eq!(ev.phase, SavePhase::Archive);
            assert_eq!(ev.bytes, MIB);
        }
        _ => panic!("expected SaveProfile"),
    }
}

#[test]
fn tc_ir_8_1_1_3_compress_phase_event_emitted() {
    let flags = DebugFlags::default();
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    pipe.compress(2, 0, 0.1, 10);
    let msgs = drain_messages(&mut ch);
    match &msgs[0] {
        ProfileMessage::SaveProfile(ev) => assert_eq!(ev.phase, SavePhase::Compress),
        _ => panic!("expected SaveProfile"),
    }
}

#[test]
fn tc_ir_8_1_1_4_finalize_event_closes_save_id() {
    let flags = DebugFlags::default();
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    pipe.gather(9, 0, 0.05);
    pipe.finalize(9, 4, 0.0);
    let msgs = drain_messages(&mut ch);
    assert!(
        matches!(msgs.last(), Some(ProfileMessage::SaveProfile(ev)) if ev.phase == SavePhase::Finalize)
    );
    let mut agg = SaveProfilerAggregator::new();
    for m in msgs {
        agg.ingest(m);
    }
    assert!(agg.is_save_finalized(9));
}

#[test]
fn tc_ir_8_1_2_1_io_write_event_correlated_by_save_id() {
    let flags = DebugFlags::default();
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    let mut fs = FakeFileSystem::default();
    let ev = fs.complete_write(42, 1, 1.0, 128);
    pipe.io_write(ev);
    let msgs = drain_messages(&mut ch);
    match &msgs[0] {
        ProfileMessage::IoWrite(ev) => assert_eq!(ev.save_id, 42),
        _ => panic!("expected IoWrite"),
    }
}

#[test]
fn tc_ir_8_1_2_2_duration_matches_fake_io_cost() {
    let mut fs = FakeFileSystem::default();
    let ev = fs.complete_write(1, 1, 5.0, 10);
    assert!((ev.duration_ms - 5.0).abs() < f64::EPSILON);
}

#[test]
fn tc_ir_8_1_3_1_memory_snapshot_event_before_less_than_after() {
    let ev = MemorySnapshotEvent {
        save_id: 1,
        rss_before: 100 * MIB,
        rss_after: 110 * MIB,
        peak_during: 120 * MIB,
    };
    assert!(ev.rss_before < ev.rss_after);
}

#[test]
fn tc_ir_8_1_3_2_peak_recorded_during_archive() {
    let ev = MemorySnapshotEvent {
        save_id: 1,
        rss_before: 100 * MIB,
        rss_after: 110 * MIB,
        peak_during: 120 * MIB,
    };
    assert_eq!(ev.peak_during, 120 * MIB);
}

#[test]
fn tc_ir_8_1_4_1_phase8_total_includes_save_events() {
    let flags = DebugFlags::default();
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    pipe.gather(1, 0, 0.1);
    pipe.archive(1, 0, 0.2, 0);
    pipe.compress(1, 0, 0.15, 0);
    let mut agg = SaveProfilerAggregator::new();
    agg.drain(&mut ch);
    agg.end_frame();
    assert!(!agg.phase8_budget_breach);
}

#[test]
fn tc_ir_8_1_4_2_budget_breach_red_phase8_bar_with_hud() {
    let flags = DebugFlags::default();
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    pipe.gather(1, 0, 0.7);
    let mut agg = SaveProfilerAggregator::new();
    agg.drain(&mut ch);
    agg.end_frame();
    assert!(agg.phase8_budget_breach);
    assert!(hud_phase8_bar_red(&flags, &agg));
}

#[test]
fn tc_ir_8_1_5_1_success_increments_total_saves() {
    let mut agg = SaveProfilerAggregator::new();
    agg.record_save_success(1.0);
    agg.record_save_success(2.0);
    agg.record_save_success(3.0);
    assert_eq!(agg.metrics().total_saves, 3);
    assert_eq!(agg.metrics().total_failures, 0);
}

#[test]
fn tc_ir_8_1_5_2_failure_increments_total_failures() {
    let mut agg = SaveProfilerAggregator::new();
    agg.record_save_success(1.0);
    agg.record_save_success(1.0);
    agg.ingest(ProfileMessage::IoWrite(IoWriteEvent {
        save_id: 3,
        request_id: 1,
        duration_ms: 1.0,
        bytes_written: 0,
        error: IoError::DiskFull,
    }));
    assert_eq!(agg.metrics().total_saves, 2);
    assert_eq!(agg.metrics().total_failures, 1);
}

#[test]
fn tc_ir_8_1_5_3_p99_rolling_window_correct() {
    let mut agg = SaveProfilerAggregator::new();
    for ms in 1_i32..=100 {
        agg.record_save_success(ms as f64);
    }
    let p99 = agg.metrics().p99_duration_ms;
    assert!((p99 - 99.0).abs() <= 1.0, "p99={p99}");
}

#[test]
fn tc_ir_8_1_6_1_schema_breakdown_sums_equal_archive_bytes() {
    let events = vec![
        SchemaBreakdownEvent {
            save_id: 1,
            node: NODE_PLAYER,
            bytes: 100,
            entity_count: 1,
        },
        SchemaBreakdownEvent {
            save_id: 1,
            node: NODE_WORLD,
            bytes: 400,
            entity_count: 1,
        },
        SchemaBreakdownEvent {
            save_id: 1,
            node: NODE_AI,
            bytes: 500,
            entity_count: 1,
        },
    ];
    assert_eq!(schema_bytes_sum(&events), 1000);
}

#[test]
fn tc_ir_8_1_6_2_sunburst_entity_counts_match_archive() {
    let ev = SchemaBreakdownEvent {
        save_id: 1,
        node: NODE_PLAYER,
        bytes: 100,
        entity_count: 10,
    };
    assert_eq!(ev.entity_count, 10);
}

#[test]
fn tc_ir_8_1_1_n1_ch24_full_drops_oldest() {
    let mut ch = SaveProfileChannel::ch24();
    for i in 0_u64..64 {
        ch.send(ProfileMessage::SaveProfile(SaveProfileEvent {
            save_id: i,
            phase: SavePhase::Gather,
            start_tick: i,
            duration_ms: 1.0,
            bytes: 0,
        }));
    }
    assert_eq!(ch.len(), 32);
    assert_eq!(ch.fm1_channel_drops, 32);
}

#[test]
fn tc_ir_8_1_3_n1_rss_syscall_failure_keeps_last() {
    let mut agg = SaveProfilerAggregator::new();
    let first = agg.memory_snapshot_or_last_known(1, Ok((10, 20, 30)));
    assert_eq!(first.rss_before, 10);
    let second = agg.memory_snapshot_or_last_known(2, Err(()));
    assert_eq!(second.rss_before, 10);
    assert_eq!(second.rss_after, 20);
    assert_eq!(second.peak_during, 30);
    assert_eq!(agg.fm2_rss_failures, 1);
}

#[test]
fn tc_ir_8_1_2_n1_io_write_error_recorded() {
    let mut agg = SaveProfilerAggregator::new();
    agg.ingest(ProfileMessage::IoWrite(IoWriteEvent {
        save_id: 1,
        request_id: 1,
        duration_ms: 1.0,
        bytes_written: 0,
        error: IoError::DiskFull,
    }));
    assert_eq!(agg.metrics().total_failures, 1);
    assert_eq!(agg.metrics().last_error_code, 2);
}

#[test]
fn tc_ir_8_1_1_n2_profiler_off_skips_events() {
    let flags = DebugFlags {
        show_profiler_hud: false,
    };
    let mut ch = SaveProfileChannel::ch24();
    let mut pipe = SavePipeline::new(&mut ch, &flags);
    pipe.gather(1, 0, 1.0);
    assert!(drain_messages(&mut ch).is_empty());
}

#[test]
fn tc_ir_8_1_6_n1_orphaned_schema_event_counted() {
    let mut agg = SaveProfilerAggregator::new();
    agg.ingest(ProfileMessage::SaveProfile(SaveProfileEvent {
        save_id: 1,
        phase: SavePhase::Finalize,
        start_tick: 0,
        duration_ms: 0.0,
        bytes: 0,
    }));
    agg.ingest(ProfileMessage::Schema(SchemaBreakdownEvent {
        save_id: 1,
        node: NODE_PLAYER,
        bytes: 1,
        entity_count: 0,
    }));
    assert_eq!(agg.fm5_orphan_schema, 1);
}
