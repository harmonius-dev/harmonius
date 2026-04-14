//! Samples timeline tracks and writes camera override components.

use glam::Vec3;

use crate::assets::AssetStore;
use crate::handle::Handle;
use crate::playback::PlaybackState;
use crate::recomposer::{DollyRig, Recomposer};
use crate::timeline::{MultiTrackTimeline, TrackId, TrackValue};

/// Authoring-side binding from timeline tracks to runtime override stores.
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineCameraBinding {
    /// Timeline asset sampled each tick.
    pub timeline: Handle<MultiTrackTimeline>,
    /// Optional position offset track (`TrackValue::Vec3`).
    pub position_track: Option<TrackId>,
    /// Optional euler rotation offset track (`TrackValue::Vec3`).
    pub rotation_track: Option<TrackId>,
    /// Optional additive FOV delta track (`TrackValue::F32`).
    pub fov_track: Option<TrackId>,
    /// Optional normalized dolly parameter track (`TrackValue::F32`).
    pub dolly_track: Option<TrackId>,
    /// Optional blend weight track (`TrackValue::F32`).
    pub blend_weight_track: Option<TrackId>,
}

/// Runtime toggles observed by the binding system.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TimelineCameraDebug {
    /// When true, logs are emitted (test hooks flip this).
    pub log_samples: bool,
    /// When true, debug drawing is requested (no-op in this crate).
    pub draw_overrides: bool,
    /// When true, overrides stop mutating authoritative components.
    pub freeze_overrides: bool,
}

/// Clamps timeline-driven weights into `[0.0, 1.0]`.
#[must_use]
pub fn clamp_unit_f32(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

/// Samples bound tracks and writes results into `recomposer` / `dolly`.
pub fn apply_timeline_camera_binding(
    binding: &TimelineCameraBinding,
    playback: &PlaybackState,
    recomposer: &mut Recomposer,
    dolly: Option<&mut DollyRig>,
    assets: &AssetStore<MultiTrackTimeline>,
    debug: &TimelineCameraDebug,
) {
    let _ = (debug.log_samples, debug.draw_overrides);
    if debug.freeze_overrides {
        return;
    }
    let Some(timeline) = assets.get(binding.timeline) else {
        return;
    };
    let sample_time = playback.current_time;
    write_vec3_channel(
        binding.position_track,
        timeline,
        sample_time,
        &mut recomposer.position_offset,
    );
    write_vec3_channel(
        binding.rotation_track,
        timeline,
        sample_time,
        &mut recomposer.rotation_offset,
    );
    write_f32_channel(
        binding.fov_track,
        timeline,
        sample_time,
        &mut recomposer.fov_delta,
        F32WriteMode::Direct,
    );
    write_f32_channel(
        binding.blend_weight_track,
        timeline,
        sample_time,
        &mut recomposer.blend_weight,
        F32WriteMode::UnitClamp,
    );
    if let Some(dolly_rig) = dolly {
        write_f32_channel(
            binding.dolly_track,
            timeline,
            sample_time,
            &mut dolly_rig.position,
            F32WriteMode::UnitClamp,
        );
    }
}

enum F32WriteMode {
    Direct,
    UnitClamp,
}

fn write_vec3_channel(
    track_id: Option<TrackId>,
    timeline: &MultiTrackTimeline,
    sample_time: f64,
    target: &mut Vec3,
) {
    let Some(id) = track_id else {
        return;
    };
    let Some(sampled) = timeline.sample_track(id, sample_time) else {
        return;
    };
    let TrackValue::Vec3(vec) = sampled else {
        return;
    };
    if !vec3_is_finite(vec) {
        return;
    }
    *target = vec;
}

fn write_f32_channel(
    track_id: Option<TrackId>,
    timeline: &MultiTrackTimeline,
    sample_time: f64,
    target: &mut f32,
    mode: F32WriteMode,
) {
    let Some(id) = track_id else {
        return;
    };
    let Some(sampled) = timeline.sample_track(id, sample_time) else {
        return;
    };
    let TrackValue::F32(value) = sampled else {
        return;
    };
    if !value.is_finite() {
        return;
    }
    match mode {
        F32WriteMode::Direct => *target = value,
        F32WriteMode::UnitClamp => *target = clamp_unit_f32(value),
    }
}

fn vec3_is_finite(vec: Vec3) -> bool {
    vec.x.is_finite() && vec.y.is_finite() && vec.z.is_finite()
}
