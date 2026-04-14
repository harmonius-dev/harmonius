//! Integration tests mapped from `docs/design/integration/audio-spatial-awareness-test-cases.md`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_channel::TrySendError;
use glam::Vec3;
use harmonius_integration_audio_sa::{
    amortized_trace_count, apply_propagation_to_voice, compute_propagation,
    compute_propagation_with_dirs, propagation_channel_pair, select_top_reflections,
    should_retrace_source, AcousticMaterial, AcousticMaterialTable, AxisAlignedSurface, Entity,
    PropagationResult, PropagationResultStore, PropagationSnapshot, PropagationTraceInput,
    ReflectionTap, SharedSpatialIndex, SourceTraceState, SpatialAudio, VoiceFilterState,
    MAX_VOICE_LPF_HZ, MIN_VOICE_LPF_HZ, MPSC_CAPACITY,
};
use rayon::prelude::*;

fn wall_entity() -> Entity {
    Entity(1)
}

fn thick_wall_full_occlusion() -> SharedSpatialIndex {
    SharedSpatialIndex::new(
        vec![AxisAlignedSurface {
            entity: wall_entity(),
            min: Vec3::new(4.5, -3.0, -3.0),
            max: Vec3::new(5.5, 3.0, 3.0),
        }],
        true,
    )
}

fn stone_table() -> AcousticMaterialTable {
    let mut t = AcousticMaterialTable::new();
    t.insert(wall_entity(), AcousticMaterial::STONE);
    t
}

fn trace_input(source: Entity) -> PropagationTraceInput {
    PropagationTraceInput {
        source,
        source_pos: Vec3::ZERO,
        listener_pos: Vec3::new(10.0, 0.0, 0.0),
        spatial: SpatialAudio::new(8, 50.0),
    }
}

#[test]
fn tc_ir_1_9_1_1_los_unoccluded() {
    let index = SharedSpatialIndex::new(vec![], true);
    let materials = AcousticMaterialTable::new();
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!((r.occlusion - 1.0).abs() < 1e-3);
}

#[test]
fn tc_ir_1_9_1_2_wall_fully_occludes() {
    let index = thick_wall_full_occlusion();
    let materials = stone_table();
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!(r.occlusion < 0.05, "occlusion={}", r.occlusion);
}

#[test]
fn tc_ir_1_9_1_3_partial_occlusion() {
    let index = thick_wall_full_occlusion();
    let materials = stone_table();
    let input = trace_input(Entity(10));
    let dirs = vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    ];
    let r = compute_propagation_with_dirs(&input, &index, &materials, 1, &dirs);
    assert!(
        (r.occlusion - 0.5).abs() < 0.01,
        "occlusion={}",
        r.occlusion
    );
}

#[test]
fn tc_ir_1_9_1_n1_bvh_not_ready() {
    let index = thick_wall_full_occlusion();
    let mut index = index;
    index.set_ready(false);
    let materials = stone_table();
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert_eq!(r.occlusion, 1.0);
    assert_eq!(r.band_loss, [0.0, 0.0, 0.0]);
    assert_eq!(r.reverb_send, 0.0);
}

#[test]
fn tc_ir_1_9_2_1_stone_low_absorption() {
    let index = thick_wall_full_occlusion();
    let materials = stone_table();
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!(r.band_loss[0] < 0.5 && r.band_loss[1] < 0.5 && r.band_loss[2] < 0.5);
}

#[test]
fn tc_ir_1_9_2_2_carpet_high_absorption() {
    let index = thick_wall_full_occlusion();
    let mut materials = AcousticMaterialTable::new();
    materials.insert(wall_entity(), AcousticMaterial::CARPET);
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!(r.band_loss[2] > r.band_loss[0]);
}

#[test]
fn tc_ir_1_9_2_3_glass_low_transmission() {
    let index = thick_wall_full_occlusion();
    let mut materials = AcousticMaterialTable::new();
    materials.insert(wall_entity(), AcousticMaterial::GLASS);
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!(r.band_loss.iter().sum::<f32>() > 0.1);
}

#[test]
fn tc_ir_1_9_2_n1_material_missing_uses_stone() {
    let index = thick_wall_full_occlusion();
    let materials = AcousticMaterialTable::new();
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    let stone = AcousticMaterial::DEFAULT_STONE;
    for b in 0..3 {
        let expected = stone.absorption[b]
            + 0.02
                * harmonius_integration_audio_sa::db_to_linear_loss(stone.transmission_loss_db[b]);
        assert!((r.band_loss[b] - expected).abs() < 1e-2);
    }
}

#[test]
fn tc_ir_1_9_3_1_result_feeds_audio() {
    let index = thick_wall_full_occlusion();
    let materials = stone_table();
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    let mut voice = VoiceFilterState::new();
    apply_propagation_to_voice(&mut voice, &r);
    assert!(voice.lpf_hz < MAX_VOICE_LPF_HZ);
}

#[test]
fn tc_ir_1_9_3_2_reflections_produce_taps() {
    let index = SharedSpatialIndex::new(
        vec![
            AxisAlignedSurface {
                entity: Entity(2),
                min: Vec3::new(4.5, -2.0, -0.2),
                max: Vec3::new(5.5, 2.0, 0.2),
            },
            AxisAlignedSurface {
                entity: Entity(3),
                min: Vec3::new(4.5, -0.2, -2.0),
                max: Vec3::new(5.5, 0.2, 2.0),
            },
        ],
        true,
    );
    let mut materials = AcousticMaterialTable::new();
    materials.insert(Entity(2), AcousticMaterial::STONE);
    materials.insert(Entity(3), AcousticMaterial::STONE);
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!(r.reflection_count >= 1);
}

#[test]
fn tc_ir_1_9_3_3_reverb_send_enclosed() {
    let surfaces = vec![
        AxisAlignedSurface {
            entity: Entity(20),
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(-0.8, 1.0, 1.0),
        },
        AxisAlignedSurface {
            entity: Entity(21),
            min: Vec3::new(0.8, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        },
        AxisAlignedSurface {
            entity: Entity(22),
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, -0.8, 1.0),
        },
        AxisAlignedSurface {
            entity: Entity(23),
            min: Vec3::new(-1.0, 0.8, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        },
        AxisAlignedSurface {
            entity: Entity(24),
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, -0.8),
        },
        AxisAlignedSurface {
            entity: Entity(25),
            min: Vec3::new(-1.0, -1.0, 0.8),
            max: Vec3::new(1.0, 1.0, 1.0),
        },
    ];
    let index = SharedSpatialIndex::new(surfaces, true);
    let materials = stone_table();
    let input = PropagationTraceInput {
        source: Entity(10),
        source_pos: Vec3::ZERO,
        listener_pos: Vec3::new(0.5, 0.0, 0.0),
        spatial: SpatialAudio::new(4, 4.0),
    };
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!(r.reverb_send > 0.7, "reverb_send={}", r.reverb_send);
}

#[test]
fn tc_ir_1_9_4_1_eight_rays_four_blocked() {
    let index = thick_wall_full_occlusion();
    let materials = stone_table();
    let dirs = vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];
    let input = trace_input(Entity(10));
    let r = compute_propagation_with_dirs(&input, &index, &materials, 1, &dirs);
    assert!((r.occlusion - 0.5).abs() < 1e-3);
}

#[test]
fn tc_ir_1_9_4_2_zero_rays_skips() {
    let index = thick_wall_full_occlusion();
    let materials = stone_table();
    let input = PropagationTraceInput {
        source: Entity(10),
        source_pos: Vec3::ZERO,
        listener_pos: Vec3::new(10.0, 0.0, 0.0),
        spatial: SpatialAudio::new(0, 50.0),
    };
    let r = compute_propagation(&input, &index, &materials, 1);
    assert_eq!(r.occlusion, 1.0);
}

#[test]
fn tc_ir_1_9_5_1_static_source_cached() {
    let mut state = SourceTraceState::new(Vec3::ZERO);
    assert!(should_retrace_source(None, &mut state, Vec3::ZERO));
    assert!(!should_retrace_source(Some(1), &mut state, Vec3::ZERO));
}

#[test]
fn tc_ir_1_9_5_2_moved_source_retraces() {
    let mut state = SourceTraceState::new(Vec3::ZERO);
    assert!(should_retrace_source(None, &mut state, Vec3::ZERO));
    assert!(!should_retrace_source(Some(1), &mut state, Vec3::ZERO));
    assert!(should_retrace_source(
        Some(1),
        &mut state,
        Vec3::new(1.0, 0.0, 0.0)
    ));
}

#[test]
fn tc_ir_1_9_5_3_amortized_rotation() {
    assert_eq!(amortized_trace_count(100, 0, 4), 25);
    assert_eq!(amortized_trace_count(100, 1, 4), 25);
}

#[test]
fn tc_ir_1_9_5_4_new_source_pre_trace() {
    let snap = PropagationSnapshot::new();
    assert!(snap.get(Entity(99)).is_none());
    let default_los = PropagationResult::line_of_sight_default(Entity(99));
    assert_eq!(default_los.occlusion, 1.0);
}

#[test]
fn tc_ir_1_9_3_4_mpsc_drain_updates_snap() {
    let (tx, rx) = propagation_channel_pair();
    let e = Entity(7);
    for i in 0..3 {
        let mut r = PropagationResult::line_of_sight_default(e);
        r.last_updated_frame = i;
        tx.send(r).expect("send");
    }
    let mut snap = PropagationSnapshot::new();
    snap.drain_from(&rx, |_| true);
    assert_eq!(snap.get(e).expect("result").last_updated_frame, 2);
}

#[test]
fn tc_ir_1_9_3_5_worker_write_disjoint() {
    let store = Arc::new(PropagationResultStore::new(100));
    let counter = AtomicUsize::new(0);
    (0..100).into_par_iter().for_each(|i| {
        let mut r = PropagationResult::line_of_sight_default(Entity(i as u32));
        r.last_updated_frame = i as u64;
        store.write_slot(i, r);
        counter.fetch_add(1, Ordering::Relaxed);
    });
    assert_eq!(counter.load(Ordering::Relaxed), 100);
    for i in 0..100 {
        assert_eq!(store.read_slot(i).source, Entity(i as u32));
    }
}

#[test]
fn tc_ir_1_9_3_n1_mpsc_full_drops() {
    let (tx, rx) = propagation_channel_pair();
    let e = Entity(1);
    for _ in 0..MPSC_CAPACITY {
        tx.send(PropagationResult::line_of_sight_default(e))
            .expect("send");
    }
    let extra = PropagationResult::line_of_sight_default(e);
    let err = tx.try_send(extra);
    assert!(matches!(err, Err(TrySendError::Full(_))));
    let mut snap = PropagationSnapshot::new();
    snap.drain_from(&rx, |_| true);
    assert_eq!(snap.len(), 1);
}

#[test]
fn tc_ir_1_9_3_n2_despawned_orphan() {
    let (tx, rx) = propagation_channel_pair();
    let e = Entity(5);
    tx.send(PropagationResult::line_of_sight_default(e))
        .expect("send");
    let mut snap = PropagationSnapshot::new();
    snap.drain_from(&rx, |_| false);
    assert!(snap.get(e).is_none());
}

#[test]
fn tc_ir_1_9_3_n3_empty_drain() {
    let (tx, rx) = propagation_channel_pair();
    let e = Entity(1);
    tx.send(PropagationResult::line_of_sight_default(e))
        .expect("send");
    let mut snap = PropagationSnapshot::new();
    snap.drain_from(&rx, |_| true);
    let before = snap.clone();
    snap.drain_from(&rx, |_| true);
    assert_eq!(snap, before);
}

#[test]
fn tc_ir_1_9_4_n1_all_rays_blocked_max_lp() {
    let index = thick_wall_full_occlusion();
    let materials = stone_table();
    let input = trace_input(Entity(10));
    let r = compute_propagation(&input, &index, &materials, 1);
    assert!(r.occlusion < 0.05);
    let mut voice = VoiceFilterState::new();
    apply_propagation_to_voice(&mut voice, &r);
    assert!((voice.lpf_hz - MIN_VOICE_LPF_HZ).abs() < 1.0);
}

#[test]
fn tc_ir_1_9_5_n2_reflection_overflow_keeps_top_gains() {
    let taps: Vec<ReflectionTap> = (0..16)
        .map(|i| ReflectionTap {
            delay_ms: i as f32,
            gain: i as f32,
            direction: Vec3::X,
        })
        .collect();
    let (arr, count) = select_top_reflections(&taps);
    assert_eq!(count, 8);
    let mut gains: Vec<f32> = arr.iter().map(|t| t.gain).collect();
    gains.retain(|g| *g > 0.0);
    assert_eq!(gains.len(), 8);
    assert!((gains[0] - 15.0).abs() < 1e-3);
}
