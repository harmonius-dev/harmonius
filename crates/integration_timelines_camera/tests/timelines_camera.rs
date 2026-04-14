//! Integration coverage for IR-4.8.* timelines ↔ camera binding.

use glam::Vec3;
use integration_timelines_camera::{
    apply_timeline_camera_binding, clamp_unit_f32, interpolate_recomposer_overrides,
    AssetStore, DollyRig, Interpolation, Keyframe, LoopMode, MultiTrackTimeline, PlaybackState,
    Recomposer, SequencerEntry, TimelineCameraBinding, TimelineCameraDebug, Track, TrackId,
    TrackValue,
};

fn linear_vec3_track(id: TrackId, start: Vec3, end: Vec3) -> Track {
    Track {
        id,
        keyframes: vec![
            Keyframe {
                time: 0.0,
                value: TrackValue::Vec3(start),
                interpolation: Interpolation::Linear,
            },
            Keyframe {
                time: 1.0,
                value: TrackValue::Vec3(end),
                interpolation: Interpolation::Linear,
            },
        ],
    }
}

fn linear_f32_track(id: TrackId, start: f32, end: f32) -> Track {
    Track {
        id,
        keyframes: vec![
            Keyframe {
                time: 0.0,
                value: TrackValue::F32(start),
                interpolation: Interpolation::Linear,
            },
            Keyframe {
                time: 1.0,
                value: TrackValue::F32(end),
                interpolation: Interpolation::Linear,
            },
        ],
    }
}

#[test]
fn tc_ir_4_8_1_u1_vec3_offset_sample_end() {
    let track = linear_vec3_track(TrackId(1), Vec3::ZERO, Vec3::new(2.0, 3.0, 4.0));
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let sampled = timeline
        .sample_track(TrackId(1), 1.0)
        .expect("sample end keyframe");
    assert_eq!(sampled, TrackValue::Vec3(Vec3::new(2.0, 3.0, 4.0)));
}

#[test]
fn tc_ir_4_8_1_u2_vec3_lerp_mid() {
    let track = linear_vec3_track(TrackId(1), Vec3::ZERO, Vec3::new(2.0, 4.0, 6.0));
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let sampled = timeline
        .sample_track(TrackId(1), 0.5)
        .expect("midpoint sample");
    assert_eq!(sampled, TrackValue::Vec3(Vec3::new(1.0, 2.0, 3.0)));
}

#[test]
fn tc_ir_4_8_2_u1_euler_vec3_sample_mid() {
    let track = linear_vec3_track(TrackId(2), Vec3::ZERO, Vec3::new(90.0, 0.0, 0.0));
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let sampled = timeline
        .sample_track(TrackId(2), 0.5)
        .expect("mid euler");
    assert_eq!(sampled, TrackValue::Vec3(Vec3::new(45.0, 0.0, 0.0)));
}

#[test]
fn tc_ir_4_8_3_u1_fov_delta_mid() {
    let track = linear_f32_track(TrackId(3), 0.0, 10.0);
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let sampled = timeline
        .sample_track(TrackId(3), 0.5)
        .expect("mid fov");
    assert_eq!(sampled, TrackValue::F32(5.0));
}

#[test]
fn tc_ir_4_8_4_u1_u2_blend_clamp() {
    assert_eq!(clamp_unit_f32(-0.5), 0.0);
    assert_eq!(clamp_unit_f32(1.7), 1.0);
}

#[test]
fn tc_ir_4_8_5_u1_sequencer_entry_default_has_no_timeline_ref() {
    let entry = SequencerEntry::default();
    assert!(entry.timeline_ref.is_none());
}

#[test]
fn tc_ir_4_8_6_u1_zero_tick_reuses_previous_render_snapshot() {
    let previous = Recomposer {
        position_offset: Vec3::new(1.0, 2.0, 3.0),
        ..Default::default()
    };
    let latest = Recomposer {
        position_offset: Vec3::new(9.0, 9.0, 9.0),
        ..Default::default()
    };
    let rendered = interpolate_recomposer_overrides(&previous, &latest, 1.0, false);
    assert_eq!(rendered.position_offset, previous.position_offset);
}

#[test]
fn tc_ir_4_8_6_u2_multi_tick_interpolates_toward_latest() {
    let previous = Recomposer {
        position_offset: Vec3::ZERO,
        ..Default::default()
    };
    let latest = Recomposer {
        position_offset: Vec3::new(4.0, 0.0, 0.0),
        ..Default::default()
    };
    let rendered = interpolate_recomposer_overrides(&previous, &latest, 1.0, true);
    assert_eq!(rendered.position_offset, latest.position_offset);
}

#[test]
fn tc_ir_4_8_7_u1_u2_dolly_clamp_helpers_via_binding() {
    let pos = linear_vec3_track(TrackId(10), Vec3::ZERO, Vec3::ONE);
    let dolly_high = linear_f32_track(TrackId(11), 1.5, 1.5);
    let dolly_low = linear_f32_track(TrackId(12), -0.1, -0.1);
    let timeline = MultiTrackTimeline {
        id: 7,
        tracks: vec![pos, dolly_high, dolly_low],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut store = AssetStore::new();
    let handle = store.insert(timeline);
    let mut recomposer = Recomposer::default();
    let mut dolly = DollyRig::default();
    let binding = TimelineCameraBinding {
        timeline: handle,
        position_track: None,
        rotation_track: None,
        fov_track: None,
        dolly_track: Some(TrackId(11)),
        blend_weight_track: None,
    };
    apply_timeline_camera_binding(
        &binding,
        &PlaybackState {
            current_time: 0.0,
            playing: true,
            ..PlaybackState::default()
        },
        &mut recomposer,
        Some(&mut dolly),
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(dolly.position, 1.0);
    let binding_low = TimelineCameraBinding {
        timeline: handle,
        position_track: None,
        rotation_track: None,
        fov_track: None,
        dolly_track: Some(TrackId(12)),
        blend_weight_track: None,
    };
    apply_timeline_camera_binding(
        &binding_low,
        &PlaybackState {
            current_time: 0.0,
            playing: true,
            ..PlaybackState::default()
        },
        &mut recomposer,
        Some(&mut dolly),
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(dolly.position, 0.0);
}

#[test]
fn tc_ir_4_8_1_1_position_drives_recomposer() {
    let track = linear_vec3_track(TrackId(5), Vec3::ZERO, Vec3::new(1.0, 2.0, 3.0));
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut store = AssetStore::new();
    let handle = store.insert(timeline);
    let mut recomposer = Recomposer::default();
    let binding = TimelineCameraBinding {
        timeline: handle,
        position_track: Some(TrackId(5)),
        rotation_track: None,
        fov_track: None,
        dolly_track: None,
        blend_weight_track: None,
    };
    apply_timeline_camera_binding(
        &binding,
        &PlaybackState {
            current_time: 1.0,
            playing: true,
            ..PlaybackState::default()
        },
        &mut recomposer,
        None,
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(recomposer.position_offset, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn tc_ir_4_8_1_n1_asset_missing_holds_state() {
    let mut store = AssetStore::new();
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![linear_vec3_track(TrackId(1), Vec3::ZERO, Vec3::ONE)],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let handle = store.insert(timeline);
    store.free(handle);
    let mut recomposer = Recomposer {
        position_offset: Vec3::new(5.0, 6.0, 7.0),
        ..Default::default()
    };
    let binding = TimelineCameraBinding {
        timeline: handle,
        position_track: Some(TrackId(1)),
        rotation_track: None,
        fov_track: None,
        dolly_track: None,
        blend_weight_track: None,
    };
    apply_timeline_camera_binding(
        &binding,
        &PlaybackState::default(),
        &mut recomposer,
        None,
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(recomposer.position_offset, Vec3::new(5.0, 6.0, 7.0));
}

#[test]
fn tc_ir_4_8_1_n3_nan_vec3_skips_write() {
    let track = Track {
        id: TrackId(1),
        keyframes: vec![
            Keyframe {
                time: 0.0,
                value: TrackValue::Vec3(Vec3::splat(f32::NAN)),
                interpolation: Interpolation::Linear,
            },
        ],
    };
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut store = AssetStore::new();
    let handle = store.insert(timeline);
    let mut recomposer = Recomposer {
        position_offset: Vec3::new(1.0, 1.0, 1.0),
        ..Default::default()
    };
    let binding = TimelineCameraBinding {
        timeline: handle,
        position_track: Some(TrackId(1)),
        rotation_track: None,
        fov_track: None,
        dolly_track: None,
        blend_weight_track: None,
    };
    apply_timeline_camera_binding(
        &binding,
        &PlaybackState {
            current_time: 0.0,
            playing: true,
            ..PlaybackState::default()
        },
        &mut recomposer,
        None,
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(recomposer.position_offset, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn tc_ir_4_8_3_n1_nan_f32_skips_fov_write() {
    let track = Track {
        id: TrackId(9),
        keyframes: vec![Keyframe {
            time: 0.0,
            value: TrackValue::F32(f32::NAN),
            interpolation: Interpolation::Linear,
        }],
    };
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut store = AssetStore::new();
    let handle = store.insert(timeline);
    let mut recomposer = Recomposer {
        fov_delta: 12.0,
        ..Default::default()
    };
    let binding = TimelineCameraBinding {
        timeline: handle,
        position_track: None,
        rotation_track: None,
        fov_track: Some(TrackId(9)),
        dolly_track: None,
        blend_weight_track: None,
    };
    apply_timeline_camera_binding(
        &binding,
        &PlaybackState {
            current_time: 0.0,
            playing: true,
            ..PlaybackState::default()
        },
        &mut recomposer,
        None,
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(recomposer.fov_delta, 12.0);
}

#[test]
fn tc_ir_4_8_5_n3_paused_playback_samples_stable_time() {
    let track = linear_vec3_track(TrackId(4), Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![track],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut store = AssetStore::new();
    let handle = store.insert(timeline);
    let mut recomposer = Recomposer::default();
    let binding = TimelineCameraBinding {
        timeline: handle,
        position_track: Some(TrackId(4)),
        rotation_track: None,
        fov_track: None,
        dolly_track: None,
        blend_weight_track: None,
    };
    let playback = PlaybackState {
        current_time: 0.25,
        playing: false,
        ..PlaybackState::default()
    };
    apply_timeline_camera_binding(
        &binding,
        &playback,
        &mut recomposer,
        None,
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(recomposer.position_offset, Vec3::new(2.5, 0.0, 0.0));
}

#[test]
fn tc_ir_4_8_2_n1_missing_rotation_track_skips_only_rotation() {
    let pos = linear_vec3_track(TrackId(1), Vec3::ZERO, Vec3::ONE);
    let timeline = MultiTrackTimeline {
        id: 1,
        tracks: vec![pos],
        duration: 1.0,
        loop_mode: LoopMode::Once,
    };
    let mut store = AssetStore::new();
    let handle = store.insert(timeline);
    let mut recomposer = Recomposer {
        rotation_offset: Vec3::new(3.0, 3.0, 3.0),
        ..Default::default()
    };
    let binding = TimelineCameraBinding {
        timeline: handle,
        position_track: Some(TrackId(1)),
        rotation_track: Some(TrackId(99)),
        fov_track: None,
        dolly_track: None,
        blend_weight_track: None,
    };
    apply_timeline_camera_binding(
        &binding,
        &PlaybackState {
            current_time: 1.0,
            playing: true,
            ..PlaybackState::default()
        },
        &mut recomposer,
        None,
        &store,
        &TimelineCameraDebug::default(),
    );
    assert_eq!(recomposer.position_offset, Vec3::ONE);
    assert_eq!(recomposer.rotation_offset, Vec3::new(3.0, 3.0, 3.0));
}
