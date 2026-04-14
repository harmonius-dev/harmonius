use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harmonius_ai_grids::{
    apply_pending_writes, apply_pending_writes_voxel, enqueue_influence_write,
    fog_condition_at_world_2d, open_influence_channel, propagate_influence_2d_four_way,
    refresh_goap_safe_zone_2d, refresh_goap_safe_zone_3d, run_bt_influence_sample,
    sample_flow_field_2d, sample_flow_field_3d, score_grid_cell_3d, Blackboard, BlackboardKey,
    BlackboardValue, BtInfluenceSample, BtInfluenceWrite, CellCoord, CellOrVoxel, Entity,
    FlowFieldSample, FogState, GridCellConsideration, HierarchicalGrid, InfluenceSource,
    InfluenceWriteCommand, InfluenceWriteGrids, InfluenceWriteMode, ResponseCurve, UniformGrid,
    Vec2, Vec3, VoxelCoord, VoxelVolume,
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
                let _ = run_bt_influence_sample(sample, entity, Some(&grid), None, None, &mut bb);
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
                let _ = run_bt_influence_sample(sample, entity, None, Some(&vol), None, &mut bb);
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
                let _ = run_bt_influence_sample(sample, entity, None, None, Some(&hier), &mut bb);
            }
        });
    });
}

fn bench_flow_reads(c: &mut Criterion) {
    let mut grid = UniformGrid::new(
        128,
        128,
        1.0,
        0.0,
        0.0,
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
    );
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
                let _ = refresh_goap_safe_zone_2d(&grid, v, 1.0, 0.7);
            }
        });
    });
}

fn bench_goap_refreshes_3d(c: &mut Criterion) {
    let vol = VoxelVolume::new(32, 32, 32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    c.bench_function("tc_ir_2_3_5_b2_500_goap_3d_refreshes", |b| {
        b.iter(|| {
            for i in 0..500 {
                let v = black_box(i % 31) as f32;
                let _ = refresh_goap_safe_zone_3d(&vol, v, 1.0, 1.0, 0.7);
            }
        });
    });
}

fn bench_fog_checks(c: &mut Criterion) {
    let grid = UniformGrid::new(64, 64, 1.0, 0.0, 0.0, FogState::Visible, FogState::Visible);

    c.bench_function("tc_ir_2_3_3_b1_500_fog_checks", |b| {
        b.iter(|| {
            for i in 0..500 {
                let v = black_box(i % 63) as f32;
                let _ = fog_condition_at_world_2d(&grid, v, 1.0, FogState::Visible);
            }
        });
    });
}

fn bench_utility_scores_3d(c: &mut Criterion) {
    let vol = VoxelVolume::new(32, 32, 32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    c.bench_function("tc_ir_2_3_4_b2_500_utility_scores_3d", |b| {
        b.iter(|| {
            let mut bb = Blackboard::new();
            for i in 0..500 {
                let v = black_box(i % 31) as f32;
                bb.insert(BlackboardKey(1), BlackboardValue::F32(v));
                bb.insert(BlackboardKey(2), BlackboardValue::F32(v));
                bb.insert(BlackboardKey(3), BlackboardValue::F32(v));
                let _ = score_grid_cell_3d(
                    &vol,
                    &bb,
                    BlackboardKey(1),
                    BlackboardKey(2),
                    BlackboardKey(3),
                    ResponseCurve::Linear,
                );
            }
        });
    });
}

fn bench_drain_and_propagate_2d(c: &mut Criterion) {
    let entity = Entity::new(1, 1);
    let batch: Vec<InfluenceWriteCommand> = (0..1024)
        .map(|i| InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord {
                x: (i % 255) as i32,
                y: ((i / 255) % 255) as i32,
            }),
            value: 0.01,
            mode: InfluenceWriteMode::Additive,
            sender_entity_index: 1,
            seq: u64::from(i),
        })
        .collect();

    c.bench_function("tc_ir_2_3_6_b1_256_drain_and_propagate", |b| {
        let mut g = UniformGrid::new(256, 256, 1.0, 0.0, 0.0, 0.0, 0.0);
        g.sync_back_from_front();
        b.iter(|| {
            g.sync_back_from_front();
            let work = batch.clone();
            let _ = apply_pending_writes(entity, entity, &mut g, work);
            let _ = propagate_influence_2d_four_way(&mut g);
            black_box(g.get_back(CellCoord { x: 0, y: 0 }));
        });
    });
}

fn bench_drain_voxel_volume(c: &mut Criterion) {
    let entity = Entity::new(1, 1);
    let batch: Vec<InfluenceWriteCommand> = (0..512)
        .map(|i| InfluenceWriteCommand {
            cell: CellOrVoxel::Voxel(VoxelCoord {
                x: (i % 63) as i32,
                y: ((i / 63) % 63) as i32,
                z: ((i / (63 * 63)) % 63) as i32,
            }),
            value: 0.01,
            mode: InfluenceWriteMode::Additive,
            sender_entity_index: 1,
            seq: u64::from(i),
        })
        .collect();

    c.bench_function("tc_ir_2_3_6_b2_64_voxel_drain", |b| {
        b.iter(|| {
            let mut vol = VoxelVolume::new(64, 64, 64, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            vol.sync_back_from_front();
            let mut work = batch.clone();
            let _ = apply_pending_writes_voxel(entity, entity, &mut vol, std::mem::take(&mut work));
            black_box(vol.get_back(VoxelCoord { x: 0, y: 0, z: 0 }));
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
                    InfluenceWriteGrids {
                        uniform: Some(&grid),
                        ..Default::default()
                    },
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
    bench_fog_checks,
    bench_utility_scores,
    bench_utility_scores_3d,
    bench_goap_refreshes,
    bench_goap_refreshes_3d,
    bench_drain_and_propagate_2d,
    bench_drain_voxel_volume,
    bench_mpsc_enqueues,
);
criterion_main!(benches);
