use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

use harmonius_ai_grids::{
    apply_pending_writes, drain_influence_writes, enqueue_influence_write, fog_condition_at_world_2d,
    fog_value_or_unexplored, max_writes_per_grid, open_influence_channel,
    propagate_influence_2d_four_way, refresh_goap_safe_zone_2d, run_bt_influence_sample,
    sample_flow_field_2d_cell, sample_flow_field_3d_voxel, score_grid_cell_2d, score_grid_cell_3d,
    Blackboard, BlackboardKey, BlackboardValue, BtInfluenceSample, BtInfluenceWrite, CellCoord,
    CellOrVoxel, EnqueueResult, Entity, FogState, GridCellConsideration, HierarchicalGrid,
    InfluenceSource, InfluenceWriteCommand, InfluenceWriteMode, ResponseCurve, UniformGrid, Vec2,
    Vec3, VoxelCoord, VoxelVolume,
};

#[test]
fn max_writes_per_grid_matches_design() {
    assert_eq!(max_writes_per_grid(), 4096);
}

fn threat_value(bb: &Blackboard, key: BlackboardKey) -> f32 {
    match bb.get(key).expect("missing threat") {
        BlackboardValue::F32(v) => *v,
        BlackboardValue::Measured { value, .. } => *value,
        BlackboardValue::Unknown => panic!("unexpected unknown"),
    }
}

fn threat_unknown(bb: &Blackboard, key: BlackboardKey) -> bool {
    matches!(
        bb.get(key),
        Some(BlackboardValue::Measured { unknown: true, .. }) | Some(BlackboardValue::Unknown)
    )
}

#[test]
fn tc_ir_2_3_1_1_bt_samples_2d_influence() {
    let mut grid = UniformGrid::new(32, 32, 1.0, 0.0, 0.0, 0.0, 0.0);
    let _ = grid.set_front(CellCoord { x: 10, y: 10 }, 0.8);
    let entity = Entity::new(1, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Uniform2D,
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        target_key: BlackboardKey(3),
    };
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(1), BlackboardValue::F32(10.5));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(10.5));
    assert!(run_bt_influence_sample(
        sample,
        entity,
        Some(&grid),
        None,
        None,
        &mut bb
    ));
    assert!((threat_value(&bb, BlackboardKey(3)) - 0.8).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_3_1_2_bt_samples_3d_influence() {
    let mut vol = VoxelVolume::new(16, 16, 16, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let _ = vol.set_front(VoxelCoord { x: 4, y: 2, z: 6 }, 0.6);
    let entity = Entity::new(2, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Voxel3D,
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: Some(BlackboardKey(3)),
        target_key: BlackboardKey(4),
    };
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(1), BlackboardValue::F32(4.5));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(2.5));
    bb.insert(BlackboardKey(3), BlackboardValue::F32(6.5));
    assert!(run_bt_influence_sample(
        sample,
        entity,
        None,
        Some(&vol),
        None,
        &mut bb
    ));
    assert!((threat_value(&bb, BlackboardKey(4)) - 0.6).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_3_1_3_bt_samples_hierarchical_lod() {
    let fine = UniformGrid::new(16, 16, 1.0, 0.0, 0.0, 0.0, 0.0);
    let mut lod2 = UniformGrid::new(4, 4, 4.0, 0.0, 0.0, 0.0, 0.0);
    let _ = lod2.set_front(CellCoord { x: 2, y: 2 }, 0.4);
    let filler = UniformGrid::new(8, 8, 2.0, 0.0, 0.0, 0.0, 0.0);
    let hier = HierarchicalGrid::new(vec![fine, filler, lod2]);
    let entity = Entity::new(3, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Hierarchical2D { lod: 2 },
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        target_key: BlackboardKey(3),
    };
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(1), BlackboardValue::F32(10.5));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(10.5));
    assert!(run_bt_influence_sample(
        sample,
        entity,
        None,
        None,
        Some(&hier),
        &mut bb
    ));
    assert!((threat_value(&bb, BlackboardKey(3)) - 0.4).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_3_2_1_flow_reads_direction_2d() {
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    let _ = grid.set_front(CellCoord { x: 3, y: 3 }, Vec2::new(4.0, 0.0));
    let sample = sample_flow_field_2d_cell(&grid, CellCoord { x: 3, y: 3 });
    assert!(sample.valid);
    let n = sample.direction.normalized();
    assert!((n.x - 1.0).abs() < 1e-4 && n.y.abs() < 1e-4);
}

#[test]
fn tc_ir_2_3_2_2_flow_reads_direction_3d() {
    let mut vol = VoxelVolume::new(8, 8, 8, 1.0, 0.0, 0.0, 0.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let _ = vol.set_front(VoxelCoord { x: 1, y: 2, z: 3 }, Vec3::new(0.0, 3.0, 0.0));
    let sample = sample_flow_field_3d_voxel(&vol, VoxelCoord { x: 1, y: 2, z: 3 });
    assert!(sample.valid);
    let n = sample.direction.normalized();
    assert!(n.x.abs() < 1e-4 && (n.y - 1.0).abs() < 1e-4 && n.z.abs() < 1e-4);
}

#[test]
fn tc_ir_2_3_3_1_bt_checks_visible_cell() {
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, FogState::Unexplored, FogState::Visible);
    let _ = grid.set_front(CellCoord { x: 2, y: 2 }, FogState::Visible);
    assert!(fog_condition_at_world_2d(&grid, 2.5, 2.5, FogState::Visible));
}

#[test]
fn tc_ir_2_3_3_2_bt_checks_unexplored() {
    let grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, FogState::Unexplored, FogState::Unexplored);
    assert!(!fog_condition_at_world_2d(
        &grid,
        2.5,
        2.5,
        FogState::Visible
    ));
}

#[test]
fn tc_ir_2_3_4_1_utility_scores_from_grid() {
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    let _ = grid.set_front(CellCoord { x: 2, y: 2 }, 0.5);
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(1), BlackboardValue::F32(2.5));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(2.5));
    let consideration = GridCellConsideration {
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
    };
    let score = score_grid_cell_2d(&grid, &bb, consideration, ResponseCurve::Linear);
    assert!((score - 0.5).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_3_4_2_utility_scores_3d_cell() {
    let mut vol = VoxelVolume::new(8, 8, 8, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let _ = vol.set_front(VoxelCoord { x: 1, y: 2, z: 3 }, 0.3);
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(1), BlackboardValue::F32(1.5));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(2.5));
    bb.insert(BlackboardKey(3), BlackboardValue::F32(3.5));
    let score = score_grid_cell_3d(
        &vol,
        &bb,
        BlackboardKey(1),
        BlackboardKey(2),
        BlackboardKey(3),
        ResponseCurve::Linear,
    );
    assert!((score - 0.3).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_3_5_1_goap_reads_safe_zone() {
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    let _ = grid.set_front(CellCoord { x: 1, y: 1 }, 0.9);
    assert!(refresh_goap_safe_zone_2d(&grid, 1.5, 1.5, 0.7));
}

#[test]
fn tc_ir_2_3_5_2_goap_reads_danger_zone() {
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    let _ = grid.set_front(CellCoord { x: 1, y: 1 }, 0.2);
    assert!(!refresh_goap_safe_zone_2d(&grid, 1.5, 1.5, 0.7));
}

#[test]
fn tc_ir_2_3_6_1_ai_enqueues_additive_write() {
    let entity = Entity::new(1, 1);
    let (writer, rx) = open_influence_channel(entity);
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(10), BlackboardValue::F32(0.5));
    bb.insert(BlackboardKey(1), BlackboardValue::F32(5.5));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(5.5));
    let write = BtInfluenceWrite {
        grid_entity: entity,
        source: InfluenceSource::Uniform2D,
        value_key: BlackboardKey(10),
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        mode: InfluenceWriteMode::Additive,
    };
    let grid = UniformGrid::new(16, 16, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(
        enqueue_influence_write(&writer, write, &bb, 1, 1, Some(&grid), None),
        EnqueueResult::Accepted
    );
    let pending = drain_influence_writes(&rx);
    assert_eq!(pending.len(), 1);
}

#[test]
fn tc_ir_2_3_6_2_drain_applies_additive() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let batch = vec![
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 1, y: 1 }),
            value: 0.1,
            mode: InfluenceWriteMode::Additive,
            sender_entity_index: 1,
            seq: 1,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 1, y: 1 }),
            value: 0.1,
            mode: InfluenceWriteMode::Additive,
            sender_entity_index: 2,
            seq: 2,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 1, y: 1 }),
            value: 0.1,
            mode: InfluenceWriteMode::Additive,
            sender_entity_index: 3,
            seq: 3,
        },
    ];
    let stats = apply_pending_writes(entity, entity, &mut grid, batch);
    assert_eq!(stats.drained, 3);
    let v = grid.get_back(CellCoord { x: 1, y: 1 }).expect("cell");
    assert!((v - 0.3).abs() < 1e-5);
}

#[test]
fn tc_ir_2_3_6_3_drain_applies_overwrite_order() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let batch = vec![
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 2, y: 2 }),
            value: 1.0,
            mode: InfluenceWriteMode::Overwrite,
            sender_entity_index: 1,
            seq: 3,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 2, y: 2 }),
            value: 2.0,
            mode: InfluenceWriteMode::Overwrite,
            sender_entity_index: 1,
            seq: 2,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 2, y: 2 }),
            value: 3.0,
            mode: InfluenceWriteMode::Overwrite,
            sender_entity_index: 1,
            seq: 1,
        },
    ];
    let _stats = apply_pending_writes(entity, entity, &mut grid, batch);
    let v = grid.get_back(CellCoord { x: 2, y: 2 }).expect("cell");
    assert!((v - 3.0).abs() < 1e-5);
}

#[test]
fn tc_ir_2_3_6_4_drain_applies_max() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let batch = vec![
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 3, y: 3 }),
            value: 0.1,
            mode: InfluenceWriteMode::Max,
            sender_entity_index: 1,
            seq: 1,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 3, y: 3 }),
            value: 0.8,
            mode: InfluenceWriteMode::Max,
            sender_entity_index: 2,
            seq: 2,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 3, y: 3 }),
            value: 0.4,
            mode: InfluenceWriteMode::Max,
            sender_entity_index: 3,
            seq: 3,
        },
    ];
    let _stats = apply_pending_writes(entity, entity, &mut grid, batch);
    let v = grid.get_back(CellCoord { x: 3, y: 3 }).expect("cell");
    assert!((v - 0.8).abs() < 1e-5);
}

#[test]
fn tc_ir_2_3_6_5_mixed_mode_order() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let batch = vec![
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 4, y: 4 }),
            value: 0.2,
            mode: InfluenceWriteMode::Additive,
            sender_entity_index: 1,
            seq: 1,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 4, y: 4 }),
            value: 0.5,
            mode: InfluenceWriteMode::Max,
            sender_entity_index: 2,
            seq: 2,
        },
        InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 4, y: 4 }),
            value: 0.3,
            mode: InfluenceWriteMode::Overwrite,
            sender_entity_index: 3,
            seq: 3,
        },
    ];
    let _stats = apply_pending_writes(entity, entity, &mut grid, batch);
    let v = grid.get_back(CellCoord { x: 4, y: 4 }).expect("cell");
    assert!((v - 0.3).abs() < 1e-5);
}

#[test]
fn tc_ir_2_3_6_6_write_survives_next_tick() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let batch = vec![InfluenceWriteCommand {
        cell: CellOrVoxel::Cell(CellCoord { x: 5, y: 5 }),
        value: 0.9,
        mode: InfluenceWriteMode::Additive,
        sender_entity_index: 1,
        seq: 1,
    }];
    let _stats = apply_pending_writes(entity, entity, &mut grid, batch);
    propagate_influence_2d_four_way(&mut grid);
    grid.swap_buffers();
    let v = grid.get_front(CellCoord { x: 5, y: 5 }).expect("cell");
    assert!(v > 0.0);
}

#[test]
fn tc_ir_2_3_6_7_propagation_consumes_write() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let batch = vec![InfluenceWriteCommand {
        cell: CellOrVoxel::Cell(CellCoord { x: 3, y: 3 }),
        value: 1.0,
        mode: InfluenceWriteMode::Additive,
        sender_entity_index: 1,
        seq: 1,
    }];
    let _stats = apply_pending_writes(entity, entity, &mut grid, batch);
    propagate_influence_2d_four_way(&mut grid);
    let north = grid.get_back(CellCoord { x: 3, y: 2 }).unwrap_or(0.0);
    assert!(north > 0.0);
}

#[test]
fn tc_ir_2_3_e1_sample_outside_2d_grid() {
    let grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    let entity = Entity::new(1, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Uniform2D,
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        target_key: BlackboardKey(3),
    };
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(1), BlackboardValue::F32(-5.0));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(-5.0));
    assert!(run_bt_influence_sample(
        sample,
        entity,
        Some(&grid),
        None,
        None,
        &mut bb
    ));
    assert!((threat_value(&bb, BlackboardKey(3)) - 0.0).abs() < f32::EPSILON);
    assert!(threat_unknown(&bb, BlackboardKey(3)));
}

#[test]
fn tc_ir_2_3_e2_sample_outside_3d_volume() {
    let vol = VoxelVolume::new(4, 4, 4, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let entity = Entity::new(1, 1);
    let sample = BtInfluenceSample {
        grid_entity: entity,
        source: InfluenceSource::Voxel3D,
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: Some(BlackboardKey(3)),
        target_key: BlackboardKey(4),
    };
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(1), BlackboardValue::F32(-1.0));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(0.0));
    bb.insert(BlackboardKey(3), BlackboardValue::F32(0.0));
    assert!(run_bt_influence_sample(
        sample,
        entity,
        None,
        Some(&vol),
        None,
        &mut bb
    ));
    assert!(threat_unknown(&bb, BlackboardKey(4)));
}

#[test]
fn tc_ir_2_3_e3_flow_field_unreachable() {
    let grid = UniformGrid::new(4, 4, 1.0, 0.0, 0.0, Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
    let sample = sample_flow_field_2d_cell(&grid, CellCoord { x: 1, y: 1 });
    assert!(!sample.valid);
}

#[test]
fn tc_ir_2_3_e4_fog_with_no_coverage() {
    let grid = UniformGrid::new(4, 4, 1.0, 0.0, 0.0, FogState::Unexplored, FogState::Unexplored);
    assert_eq!(
        fog_value_or_unexplored(&grid, CellCoord { x: 1, y: 1 }),
        FogState::Unexplored
    );
}

#[test]
fn tc_ir_2_3_e5_channel_full_drops() {
    let entity = Entity::new(1, 1);
    let (writer, _rx) = open_influence_channel(entity);
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(10), BlackboardValue::F32(1.0));
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
    let grid = UniformGrid::new(4, 4, 1.0, 0.0, 0.0, 0.0, 0.0);
    let mut dropped = 0_u32;
    for seq in 0..4097 {
        if enqueue_influence_write(&writer, write, &bb, 1, seq, Some(&grid), None)
            == EnqueueResult::DroppedChannelFull
        {
            dropped += 1;
        }
    }
    assert_eq!(dropped, 1);
}

#[test]
fn tc_ir_2_3_e6_stale_grid_entity() {
    let entity = Entity::new(1, 1);
    let (writer, _rx) = open_influence_channel(entity);
    let mut bb = Blackboard::new();
    bb.insert(BlackboardKey(10), BlackboardValue::F32(1.0));
    bb.insert(BlackboardKey(1), BlackboardValue::F32(1.0));
    bb.insert(BlackboardKey(2), BlackboardValue::F32(1.0));
    let write = BtInfluenceWrite {
        grid_entity: Entity::new(2, 9),
        source: InfluenceSource::Uniform2D,
        value_key: BlackboardKey(10),
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        mode: InfluenceWriteMode::Additive,
    };
    let grid = UniformGrid::new(4, 4, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(
        enqueue_influence_write(&writer, write, &bb, 1, 1, Some(&grid), None),
        EnqueueResult::DroppedStaleEntity
    );
}

#[test]
fn tc_ir_2_3_e7_missing_position_key() {
    let entity = Entity::new(1, 1);
    let (writer, _rx) = open_influence_channel(entity);
    let bb = Blackboard::new();
    let write = BtInfluenceWrite {
        grid_entity: entity,
        source: InfluenceSource::Uniform2D,
        value_key: BlackboardKey(10),
        position_x: BlackboardKey(1),
        position_y: BlackboardKey(2),
        position_z: None,
        mode: InfluenceWriteMode::Additive,
    };
    let grid = UniformGrid::new(4, 4, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(
        enqueue_influence_write(&writer, write, &bb, 1, 1, Some(&grid), None),
        EnqueueResult::DroppedMissingKey
    );
}

#[test]
fn tc_ir_2_3_e8_read_skipped_propagation_uses_front_snapshot() {
    let mut grid = UniformGrid::new(4, 4, 1.0, 0.0, 0.0, 0.0, 1.0);
    let _ = grid.set_front(CellCoord { x: 1, y: 1 }, 1.0);
    grid.sync_back_from_front();
    let _ = grid.set_back(CellCoord { x: 1, y: 1 }, 0.0);
    grid.set_propagation_skipped(true);
    let v_front = grid.get_front(CellCoord { x: 1, y: 1 }).unwrap_or(0.0);
    assert!((v_front - 1.0_f32).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_2_3_e9_oob_write_dropped_in_drain() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(4, 4, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let batch = vec![InfluenceWriteCommand {
        cell: CellOrVoxel::Cell(CellCoord { x: 99, y: 99 }),
        value: 1.0,
        mode: InfluenceWriteMode::Additive,
        sender_entity_index: 1,
        seq: 1,
    }];
    let stats = apply_pending_writes(entity, entity, &mut grid, batch);
    assert_eq!(stats.dropped_oob, 1);
}

#[test]
fn tc_ir_2_3_d1_reordered_producers_deterministic_cell() {
    let entity = Entity::new(1, 1);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let mut batch = Vec::new();
    for seq in 0..100 {
        batch.push(InfluenceWriteCommand {
            cell: CellOrVoxel::Cell(CellCoord { x: 1, y: 1 }),
            value: 0.01,
            mode: InfluenceWriteMode::Additive,
            sender_entity_index: (seq % 7) as u32,
            seq,
        });
    }
    batch.reverse();
    let _stats = apply_pending_writes(entity, entity, &mut grid, batch.clone());
    let first = grid.get_back(CellCoord { x: 1, y: 1 }).unwrap_or(0.0);

    let mut grid2 = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid2.sync_back_from_front();
    batch.reverse();
    let _stats2 = apply_pending_writes(entity, entity, &mut grid2, batch);
    let second = grid2.get_back(CellCoord { x: 1, y: 1 }).unwrap_or(0.0);
    assert!((first - second).abs() < 1e-4);
}

#[test]
fn tc_ir_2_3_d2_repeated_run_same_seed_bit_identical() {
    fn hash_grid(grid: &UniformGrid<f32>) -> u64 {
        let mut hasher = DefaultHasher::new();
        for v in grid.front_slice() {
            v.to_bits().hash(&mut hasher);
        }
        hasher.finish()
    }

    let entity = Entity::new(1, 1);
    let mut grid_a = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid_a.sync_back_from_front();
    let batch = vec![InfluenceWriteCommand {
        cell: CellOrVoxel::Cell(CellCoord { x: 2, y: 2 }),
        value: 0.4,
        mode: InfluenceWriteMode::Max,
        sender_entity_index: 3,
        seq: 9,
    }];
    let _ = apply_pending_writes(entity, entity, &mut grid_a, batch.clone());
    propagate_influence_2d_four_way(&mut grid_a);
    grid_a.swap_buffers();

    let mut grid_b = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid_b.sync_back_from_front();
    let _ = apply_pending_writes(entity, entity, &mut grid_b, batch);
    propagate_influence_2d_four_way(&mut grid_b);
    grid_b.swap_buffers();

    assert_eq!(hash_grid(&grid_a), hash_grid(&grid_b));
}

#[test]
fn tc_ir_2_3_d3_parallel_phase4_writes_commutative() {
    let entity = Entity::new(1, 1);
    let (writer, rx) = open_influence_channel(entity);
    let grid = Arc::new(UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0));
    let writer = Arc::new(writer);
    let seq = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();
    for thread_id in 0..8 {
        let w = Arc::clone(&writer);
        let seq = Arc::clone(&seq);
        let grid = Arc::clone(&grid);
        handles.push(thread::spawn(move || {
            for _ in 0..32 {
                let mut bb = Blackboard::new();
                bb.insert(BlackboardKey(10), BlackboardValue::F32(0.01));
                bb.insert(BlackboardKey(1), BlackboardValue::F32(1.0));
                bb.insert(BlackboardKey(2), BlackboardValue::F32(1.0));
                let write = BtInfluenceWrite {
                    grid_entity: Entity::new(1, 1),
                    source: InfluenceSource::Uniform2D,
                    value_key: BlackboardKey(10),
                    position_x: BlackboardKey(1),
                    position_y: BlackboardKey(2),
                    position_z: None,
                    mode: InfluenceWriteMode::Additive,
                };
                let s = seq.fetch_add(1, Ordering::Relaxed);
                let _ = enqueue_influence_write(&w, write, &bb, thread_id, s, Some(grid.as_ref()), None);
            }
        }));
    }
    for h in handles {
        h.join().expect("thread");
    }
    let pending = drain_influence_writes(&rx);
    let mut grid = UniformGrid::new(8, 8, 1.0, 0.0, 0.0, 0.0, 0.0);
    grid.sync_back_from_front();
    let _stats = apply_pending_writes(entity, entity, &mut grid, pending);
    let v = grid.get_back(CellCoord { x: 1, y: 1 }).unwrap_or(0.0);
    assert!(v > 0.0);
}
