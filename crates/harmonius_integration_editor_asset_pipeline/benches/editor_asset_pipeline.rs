//! Criterion benches for `TC-IR-9.2.*.B1` rows in `editor-asset-pipeline-test-cases.md`.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harmonius_integration_editor_asset_pipeline::{
    AssetImportRequest, HeadlessEditorHarness, HotReloadBarrier, ImportBatchId, ImportKind,
    ImportOptions,
};
use std::path::PathBuf;

fn bench_cascade_100(c: &mut Criterion) {
    c.bench_function("tc_ir_9_2_2_b1_cascade_100", |b| {
        b.iter(|| {
            let mut harness = HeadlessEditorHarness::new();
            for i in 0..100 {
                let up = format!("up/{i}.png");
                let dn = format!("dn/{i}.mat");
                harness.fs.insert(&up, vec![i as u8]);
                harness.fs.insert(&dn, vec![i as u8, 1]);
                harness
                    .deps
                    .add_edge(PathBuf::from(&dn), PathBuf::from(&up));
            }
            harness.fs.insert("up/0.png", vec![9]);
            harness.drop_file("up/0.png");
            harness.drain_worker();
            black_box(harness.registry.len());
        });
    });
}

fn bench_hot_reload_10(c: &mut Criterion) {
    c.bench_function("tc_ir_9_2_3_b1_hot_reload_10", |b| {
        b.iter(|| {
            let mut barrier = HotReloadBarrier::new();
            for i in 0..10 {
                barrier.try_swap(|| {
                    black_box(i);
                });
            }
            black_box(barrier.park_cycles);
        });
    });
}

fn bench_progress_256(c: &mut Criterion) {
    c.bench_function("tc_ir_9_2_4_b1_drain_256_progress", |b| {
        b.iter(|| {
            let mut harness = HeadlessEditorHarness::new();
            let job = AssetImportRequest {
                batch_id: ImportBatchId(0),
                kind: ImportKind::Texture,
                only_if_stale: false,
                options: ImportOptions::default(),
                request_id: 1,
                source_path: PathBuf::from("x.png"),
                watcher_mtime: None,
            };
            harness.emit_progress_flood(&job, 256);
            black_box(harness.progress.len());
        });
    });
}

fn bench_batch_1000(c: &mut Criterion) {
    c.bench_function("tc_ir_9_2_6_b1_batch_enqueue_1000", |b| {
        b.iter(|| {
            let mut harness = HeadlessEditorHarness::new();
            for i in 0..1000 {
                harness
                    .fs
                    .insert(format!("batch/file_{i}.png"), vec![1]);
            }
            let batch = ImportBatchId(1);
            harness.drop_folder("batch", 1000, batch);
            black_box(harness.requests.len());
        });
    });
}

criterion_group!(
    benches,
    bench_cascade_100,
    bench_hot_reload_10,
    bench_progress_256,
    bench_batch_1000
);
criterion_main!(benches);
