use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harmonius_ai_grids::{
    enqueue_influence_write, open_influence_channel, run_bt_influence_sample, sample_flow_field_2d,
    sample_flow_field_3d, Blackboard, BlackboardKey, BlackboardValue, BtInfluenceSample,
    BtInfluenceWrite, CellCoord, Entity, FlowFieldSample, GridCellConsideration, HierarchicalGrid,
    InfluenceSource, InfluenceWriteMode, ResponseCurve, UniformGrid, Vec2, Vec3, VoxelCoord,
    VoxelVolume,
};

fn bench_influence_samples_2d(c: &mut Criterion) {
    let grid = UniformGrid::new(256, 256, 1.0, 0.0, 0.0, 0.0, 0.0);
    let entity = Entity::new(1, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Uniform2D,
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        target_key: BlackboardKey(3),
    };

    c.bench_function("tc_ir_2_3_1_b1_1000_2d_samples", |b| {
        b.iter(|| {
            let mut bb = Blackboard::new();
            bb.insert(BlackboardKey(1), BlackboardValue::F32(10.0));
            bb.insert(BlackboardKey(2), BlackboardValue::F32(10.0));
            for i in 0..1000 {
                let x = black_box(i % 255) as f32;
                bb.insert(BlackboardKey(1), BlackboardValue::F32(x));
                let _ = run_bt_influence_sample(
                    sample,
                    entity,
                    Some(&grid),
                    None,
                    None,
                    &mut bb,
                );
            }
        });
    });
}

fn bench_voxel_samples(c: &mut Criterion) {
    let vol = VoxelVolume::new(64, 64, 64, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let entity = Entity::new(1, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Voxel3D,
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: Some(BlackboardKey(3)),
        target_key: BlackboardKey(4),
    };

    c.bench_function("tc_ir_2_3_1_b2_1000_voxel_samples", |b| {
        b.iter(|| {
            let mut bb = Blackboard::new();
            for i in 0..1000 {
                let v = black_box(i % 63) as f32;
                bb.insert(BlackboardKey(1), BlackboardValue::F32(v));
                bb.insert(BlackboardKey(2), BlackboardValue::F32(v));
                bb.insert(BlackboardKey(3), BlackboardValue::F32(v));
                let _ = run_bt_influence_sample(
                    sample,
                    entity,
                    None,
                    Some(&vol),
                    None,
                    &mut bb,
                );
            }
        });
    });
}

fn bench_hierarchical_samples(c: &mut Criterion) {
    let fine = UniformGrid::new(128, 128, 1.0, 0.0, 0.0, 0.0, 0.0);
    let mid = UniformGrid::new(64, 64, 2.0, 0.0, 0.0, 0.0, 0.0);
    let coarse = UniformGrid::new(32, 32, 4.0, 0.0, 0.0, 0.0, 0.0);
    let hier = HierarchicalGrid::new(vec![fine, mid, coarse]);
    let entity = Entity::new(1, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Hierarchical2D { lod: 2 },
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        target_key: BlackboardKey(3),
    };

    c.bench_function("tc_ir_2_3_1_b3_1000_hierarchical_samples", |b| {
        b.iter(|| {
            let mut bb = Blackboard::new();
            for i in 0..1000 {
                let v = black_box(i % 127) as f32;
                bb.insert(BlackboardKey(1), BlackboardValue::F32(v));
                bb.insert(BlackboardKey(2), BlackboardValue::F32(v));
                let _ = run_bt_influence_sample(
                    sample,
                    entity,
                    None,
                    None,
                    Some(&hier),
                    &mut bb,
                );
            }
        });
    });
}

fn bench_flow_reads(c: &mut Criterion) {
    let mut grid = UniformGrid::new(128, 128, 1.0, 0.0, 0.0, Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    grid.sync_back_from_front();
    grid.swap_buffers();

    c.bench_function("tc_ir_2_3_2_b1_1000_flow_reads", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let x = black_box(i % 127) as f32;
                let _: FlowFieldSample<Vec2> = sample_flow_field_2d(&grid, x, 1.0);
            }
        });
    });
}

fn bench_flow_reads_3d(c: &mut Criterion) {
    let mut vol = VoxelVolume::new(
        32,
        32,
        32,
        1.0,
        0.0,
        0.0,
        0.0,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    vol.sync_back_from_front();
    vol.swap_buffers();

    c.bench_function("tc_ir_2_3_2_b2_1000_flow_reads_3d", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let v = black_box(i % 31) as f32;
                let _: FlowFieldSample<Vec3> = sample_flow_field_3d(&vol, v, 1.0, 1.0);
            }
        });
    });
}

fn bench_utility_scores(c: &mut Criterion) {
    let grid = UniformGrid::new(64, 64, 1.0, 0.0, 0.0, 0.0, 0.5);
    let consideration = GridCellConsideration {
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
    };

    c.bench_function("tc_ir_2_3_4_b1_500_utility_scores", |b| {
        b.iter(|| {
            let mut bb = Blackboard::new();
            for i in 0..500 {
                let v = black_box(i % 63) as f32;
                bb.insert(BlackboardKey(1), BlackboardValue::F32(v));
                bb.insert(BlackboardKey(2), BlackboardValue::F32(v));
                let _ = harmonius_ai_grids::score_grid_cell_2d(
                    &grid,
                    &bb,
                    consideration,
                    ResponseCurve::Linear,
                );
            }
        });
    });
}

fn bench_goap_refreshes(c: &mut Criterion) {
    let grid = UniformGrid::new(64, 64, 1.0, 0.0, 0.0, 0.0, 0.6);

    c.bench_function("tc_ir_2_3_5_b1_500_goap_refreshes", |b| {
        b.iter(|| {
            for i in 0..500 {
                let v = black_box(i % 63) as f32;
                let _ = harmonius_ai_grids::refresh_goap_safe_zone_2d(&grid, v, 1.0, 0.7);
            }
        });
    });
}

fn bench_mpsc_enqueues(c: &mut Criterion) {
    c.bench_function("tc_ir_2_3_6_b3_4096_enqueues", |b| {
        b.iter(|| {
            let entity = Entity::new(1, 1);
            let (writer, _rx) = open_influence_channel(entity);
            let mut bb = Blackboard::new();
            bb.insert(BlackboardKey(10), BlackboardValue::F32(0.5));
            bb.insert(BlackboardKey(1), BlackboardValue::F32(1.0));
            bb.insert(BlackboardKey(2), BlackboardValue::F32(1.0));
            let write = BtInfluenceWrite {
                grid_entity: entity,
                source: InfluenceSource::Uniform2D,
                value_key: BlackboardKey(10),
                position_x: BlackboardKey(1),
                position_y: BlackboardKey(2),
                position_z: None,
                mode: InfluenceWriteMode::Additive,
            };
            let grid = UniformGrid::new(32, 32, 1.0, 0.0, 0.0, 0.0, 0.0);
            for seq in 0..4096 {
                let _ = enqueue_influence_write(
                    &writer,
                    write,
                    &bb,
                    1,
                    seq,
                    Some(&grid),
                    None,
                );
            }
        });
    });
}

criterion_group!(
    benches,
    bench_influence_samples_2d,
    bench_voxel_samples,
    bench_hierarchical_samples,
    bench_flow_reads,
    bench_flow_reads_3d,
    bench_utility_scores,
    bench_goap_refreshes,
    bench_mpsc_enqueues,
);
criterion_main!(benches);
