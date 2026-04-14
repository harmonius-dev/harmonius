//! TC-IR-1.4.5.B2 — grouping sort throughput (Criterion).

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use harmonius_integration_animation_rendering::build_skinning_dispatches_sorted;
use harmonius_integration_animation_rendering::make_row;
use harmonius_integration_animation_rendering::Handle;
use harmonius_integration_animation_rendering::SkinningMode;

fn bench_grouping_sort_10k(c: &mut Criterion) {
    let arena = Handle::from_raw(1, 1);
    let mut template_rows: Vec<_> = (0..10_000)
        .map(|i| make_row(i % 3, i % 2, i, arena, 32, SkinningMode::Lbs))
        .collect();

    c.bench_function("tc_ir_1_4_5_b2_grouping_sort_10k", |b| {
        b.iter(|| {
            let mut rows = template_rows.clone();
            black_box(build_skinning_dispatches_sorted(&mut rows));
        });
    });
}

criterion_group!(benches, bench_grouping_sort_10k);
criterion_main!(benches);
