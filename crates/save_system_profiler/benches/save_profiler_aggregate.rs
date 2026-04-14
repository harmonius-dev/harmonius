//! Smoke micro-benchmarks for aggregation hot paths (TC-IR-8.1.x benchmarks).

use std::hint::black_box;
use std::time::Instant;

use save_system_profiler::{
    IoError, ProfileMessage, SavePhase, SaveProfileEvent, SaveProfilerAggregator,
};

fn main() {
    let mut agg = SaveProfilerAggregator::new();
    let ev = SaveProfileEvent {
        save_id: 1,
        phase: SavePhase::Gather,
        start_tick: 0,
        duration_ms: 0.001,
        bytes: 0,
    };
    let start = Instant::now();
    for i in 0_u64..10_000 {
        let mut e = ev.clone();
        e.start_tick = i;
        agg.ingest(ProfileMessage::SaveProfile(black_box(e)));
    }
    let elapsed = start.elapsed();
    println!("ingest_save_profile_10k {:?}", elapsed);

    let mut agg2 = SaveProfilerAggregator::new();
    let start = Instant::now();
    for i in 0_u32..32 {
        agg2.ingest(ProfileMessage::SaveProfile(SaveProfileEvent {
            save_id: u64::from(i),
            phase: SavePhase::Archive,
            start_tick: u64::from(i),
            duration_ms: 0.001,
            bytes: 64,
        }));
    }
    black_box(&agg2);
    let elapsed = start.elapsed();
    println!("aggregate_32_events {:?}", elapsed);

    let mut agg3 = SaveProfilerAggregator::new();
    let start = Instant::now();
    for i in 0_u32..256 {
        agg3.ingest(ProfileMessage::Schema(
            save_system_profiler::SchemaBreakdownEvent {
                save_id: 1,
                node: save_system_profiler::SchemaNodeId(i),
                bytes: 16,
                entity_count: 1,
            },
        ));
    }
    black_box(&agg3);
    let elapsed = start.elapsed();
    println!("schema_breakdown_256_nodes {:?}", elapsed);

    let mut agg4 = SaveProfilerAggregator::new();
    agg4.ingest(ProfileMessage::IoWrite(
        save_system_profiler::IoWriteEvent {
            save_id: 1,
            request_id: 1,
            duration_ms: 1.0,
            bytes_written: 10,
            error: IoError::None,
        },
    ));
    black_box(agg4.metrics());
}
