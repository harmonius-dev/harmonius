//! Deterministic propagation tracing through [`crate::SharedSpatialIndex`].

use glam::Vec3;

use crate::material::AcousticMaterialTable;
use crate::spatial_audio::SpatialAudio;
use crate::spatial_index::SharedSpatialIndex;
use crate::{Entity, PropagationResult, ReflectionTap};

/// Inputs required to trace a single source.
#[derive(Debug, Clone, Copy)]
pub struct PropagationTraceInput {
    /// Source entity id.
    pub source: Entity,
    /// World-space source position.
    pub source_pos: Vec3,
    /// World-space listener position.
    pub listener_pos: Vec3,
    /// Spatial audio configuration.
    pub spatial: SpatialAudio,
}

const SPEED_OF_SOUND: f32 = 343.0;

/// Converts decibel transmission loss to a linear loss term in \[0, 1\].
#[must_use]
pub fn db_to_linear_loss(db: f32) -> f32 {
    let linear = 1.0 - 10_f32.powf(-db / 20.0);
    linear.clamp(0.0, 1.0)
}

/// Builds a narrow cone of occlusion ray directions around `base_dir`.
#[must_use]
pub fn occlusion_ray_directions(count: u32, base_dir: Vec3) -> Vec<Vec3> {
    let forward = base_dir.normalize_or_zero();
    if forward.length_squared() < 1e-8 {
        return vec![Vec3::X; count.max(1) as usize];
    }
    let mut up = Vec3::Y;
    if forward.cross(up).length_squared() < 1e-6 {
        up = Vec3::Z;
    }
    let right = forward.cross(up).normalize();
    let true_up = right.cross(forward).normalize();
    let n = count.max(1);
    let cone_radius = 0.04_f32;
    (0..n)
        .map(|i| {
            let a = std::f32::consts::TAU * (i as f32) / (n as f32);
            let offset = (right * a.cos() + true_up * a.sin()) * cone_radius;
            (forward + offset).normalize()
        })
        .collect()
}

/// Selects up to eight reflection taps by descending gain (design overflow rule).
#[must_use]
pub fn select_top_reflections(candidates: &[ReflectionTap]) -> ([ReflectionTap; 8], u8) {
    let mut ordered: Vec<ReflectionTap> = candidates.to_vec();
    ordered.sort_by(|a, b| {
        b.gain
            .partial_cmp(&a.gain)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let n = ordered.len().min(8);
    let mut out = [ReflectionTap {
        delay_ms: 0.0,
        gain: 0.0,
        direction: Vec3::ZERO,
    }; 8];
    for (i, t) in ordered.iter().take(n).enumerate() {
        out[i] = *t;
    }
    (out, n as u8)
}

/// Traces propagation using an explicit ray set (for deterministic partial-occlusion tests).
#[must_use]
pub fn compute_propagation_with_dirs(
    input: &PropagationTraceInput,
    index: &SharedSpatialIndex,
    materials: &AcousticMaterialTable,
    frame: u64,
    dirs: &[Vec3],
) -> PropagationResult {
    if !index.is_ready() {
        let mut r = PropagationResult::line_of_sight_default(input.source);
        r.last_updated_frame = frame;
        return r;
    }
    if dirs.is_empty() {
        let mut r = PropagationResult::line_of_sight_default(input.source);
        r.last_updated_frame = frame;
        return r;
    }
    let to_listener = input.listener_pos - input.source_pos;
    let dist = to_listener.length();
    if dist < 1e-4 {
        let mut r = PropagationResult::line_of_sight_default(input.source);
        r.last_updated_frame = frame;
        return r;
    }
    let max_t = (dist - 0.05).max(0.0);
    let mut blocked = 0u32;
    let mut loss_acc = [0.0_f32; 3];
    let mut reflection_candidates: Vec<ReflectionTap> = Vec::new();

    for dir in dirs {
        let d = dir.normalize_or_zero();
        if let Some(hit) = index.raycast_segment(input.source_pos, d, max_t) {
            blocked += 1;
            let mat = materials.resolve_or_default_stone(hit.entity);
            for (b, slot) in loss_acc.iter_mut().enumerate() {
                let tl_term = 0.02 * db_to_linear_loss(mat.transmission_loss_db[b]);
                *slot += mat.absorption[b] + tl_term;
            }
            let reflected = d - 2.0 * d.dot(hit.normal) * hit.normal;
            let bounce_origin = input.source_pos + d * hit.distance + hit.normal * 0.02;
            let gain0 =
                (1.0 - mat.absorption.iter().copied().sum::<f32>() / 3.0).clamp(0.0, 1.0) * 0.35;
            reflection_candidates.push(ReflectionTap {
                delay_ms: hit.distance / SPEED_OF_SOUND * 1000.0,
                gain: gain0,
                direction: reflected.normalize_or_zero(),
            });
            if let Some(h2) = index.raycast_segment(
                bounce_origin,
                reflected.normalize_or_zero(),
                input.spatial.max_distance,
            ) {
                let m2 = materials.resolve_or_default_stone(h2.entity);
                let gain1 = gain0
                    * 0.5
                    * (1.0 - m2.absorption.iter().copied().sum::<f32>() / 3.0).clamp(0.0, 1.0);
                reflection_candidates.push(ReflectionTap {
                    delay_ms: (hit.distance + h2.distance) / SPEED_OF_SOUND * 1000.0,
                    gain: gain1,
                    direction: reflected.normalize_or_zero(),
                });
            }
        }
    }

    let n = dirs.len() as f32;
    let occlusion = 1.0 - (blocked as f32 / n);
    if blocked > 0 {
        for b in &mut loss_acc {
            *b /= blocked as f32;
        }
    }

    let probe_dirs = [
        Vec3::X,
        Vec3::NEG_X,
        Vec3::Y,
        Vec3::NEG_Y,
        Vec3::Z,
        Vec3::NEG_Z,
    ];
    let mut reverb_hits = 0_u32;
    let probe_len = input.spatial.max_distance.min(4.0);
    for pd in probe_dirs {
        let pdir = pd.normalize();
        if index
            .raycast_segment(input.source_pos, pdir, probe_len)
            .is_some()
        {
            reverb_hits += 1;
        }
    }
    let reverb_send = (reverb_hits as f32 / 6.0).min(1.0);

    let (reflections, reflection_count) = select_top_reflections(&reflection_candidates);

    PropagationResult {
        source: input.source,
        occlusion,
        band_loss: loss_acc,
        reflections,
        reflection_count,
        reverb_send,
        last_updated_frame: frame,
    }
}

/// Traces propagation using [`SpatialAudio::occlusion_rays`] to build the ray cone.
#[must_use]
pub fn compute_propagation(
    input: &PropagationTraceInput,
    index: &SharedSpatialIndex,
    materials: &AcousticMaterialTable,
    frame: u64,
) -> PropagationResult {
    if !index.is_ready() {
        let mut r = PropagationResult::line_of_sight_default(input.source);
        r.last_updated_frame = frame;
        return r;
    }
    if input.spatial.occlusion_rays == 0 {
        let mut r = PropagationResult::line_of_sight_default(input.source);
        r.last_updated_frame = frame;
        return r;
    }
    let to_listener = input.listener_pos - input.source_pos;
    let dist = to_listener.length();
    if dist < 1e-4 {
        let mut r = PropagationResult::line_of_sight_default(input.source);
        r.last_updated_frame = frame;
        return r;
    }
    let base_dir = to_listener / dist;
    let dirs = occlusion_ray_directions(input.spatial.occlusion_rays, base_dir);
    compute_propagation_with_dirs(input, index, materials, frame, &dirs)
}
