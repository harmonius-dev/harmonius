//! Post-process volume resolution for per-camera stacks.

use glam::Vec3;

use crate::types::{Entity, PostProcessBlend, PostProcessParams, PostProcessStack, PostProcessVolume};

/// Inclusive on `min` faces, exclusive on `max` faces (design IR-3.1.3).
#[must_use]
pub fn aabb_contains(min: Vec3, max: Vec3, point: Vec3) -> bool {
    point.x >= min.x
        && point.x < max.x
        && point.y >= min.y
        && point.y < max.y
        && point.z >= min.z
        && point.z < max.z
}

fn blend_weighted_average(entries: &[(PostProcessParams, f32)]) -> PostProcessParams {
    let w_sum: f32 = entries.iter().map(|(_, w)| *w).sum();
    if w_sum <= f32::EPSILON {
        return PostProcessParams::default();
    }
    let mut exposure = 0.0f32;
    let mut contrast = 0.0f32;
    let mut saturation = 0.0f32;
    let mut vignette_intensity = 0.0f32;
    let mut bloom_threshold = 0.0f32;
    let mut bloom_intensity = 0.0f32;
    for (p, w) in entries {
        exposure += p.exposure * w;
        contrast += p.contrast * w;
        saturation += p.saturation * w;
        vignette_intensity += p.vignette_intensity * w;
        bloom_threshold += p.bloom_threshold * w;
        bloom_intensity += p.bloom_intensity * w;
    }
    PostProcessParams {
        exposure: exposure / w_sum,
        contrast: contrast / w_sum,
        saturation: saturation / w_sum,
        vignette_intensity: vignette_intensity / w_sum,
        bloom_threshold: bloom_threshold / w_sum,
        bloom_intensity: bloom_intensity / w_sum,
    }
}

/// Resolves [`PostProcessStack::resolved`] for a camera position.
#[must_use]
pub fn resolve_post_process_stack(
    camera_position: Vec3,
    blends: &[PostProcessBlend],
    volumes: &[PostProcessVolume],
    alive: impl Fn(Entity) -> bool,
) -> PostProcessStack {
    let mut kept = Vec::new();
    for blend in blends {
        if !alive(blend.volume) {
            continue;
        }
        let Some(vol) = volumes.iter().find(|v| v.entity == blend.volume) else {
            continue;
        };
        if !aabb_contains(vol.min, vol.max, camera_position) {
            continue;
        }
        let w = blend.weight.clamp(0.0, 1.0);
        kept.push((vol.priority, vol.params, w));
    }

    if kept.is_empty() {
        return PostProcessStack {
            entries: blends.to_vec(),
            resolved: PostProcessParams::default(),
        };
    }

    let max_priority = kept.iter().map(|(pr, _, _)| *pr).max().unwrap_or(0);
    let top: Vec<(PostProcessParams, f32)> = kept
        .iter()
        .filter(|(pr, _, _)| *pr == max_priority)
        .map(|(_, p, w)| (*p, *w))
        .collect();

    let resolved = if top.len() == 1 {
        top[0].0
    } else {
        blend_weighted_average(&top)
    };

    PostProcessStack {
        entries: blends.to_vec(),
        resolved,
    }
}
