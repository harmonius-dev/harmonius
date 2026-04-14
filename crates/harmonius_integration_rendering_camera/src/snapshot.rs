//! Snapshot extraction from [`crate::types::CameraOutput`] to [`crate::types::RenderViewFromCamera`].

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use glam::{Mat4, Vec3};

use crate::drs::scale_viewport;
use crate::projection::{clip_distances, projection_matrix};
use crate::types::{
    CameraOutput, Entity, Projection, RenderView, RenderViewFromCamera, Viewport,
};

static NO_ACTIVE_CAMERA_WARNED: AtomicBool = AtomicBool::new(false);
static FOV_CLAMP_WARNED: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static LAYER_ZERO_WARNED: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn warn_no_active_camera_once() {
    if NO_ACTIVE_CAMERA_WARNED.swap(true, Ordering::Relaxed) {
        return;
    }
    log::warn!(
        target: "harmonius_integration_rendering_camera",
        "no active camera; emitting fallback identity view"
    );
}

fn warn_fov_clamped_once(brain: Entity) {
    let mut guard = FOV_CLAMP_WARNED.lock().expect("fov warn mutex poisoned");
    if guard.contains(&brain.0) {
        return;
    }
    guard.push(brain.0);
    log::warn!(
        target: "harmonius_integration_rendering_camera",
        "clamped invalid perspective FOV for brain {}",
        brain.0
    );
}

fn warn_render_layers_zero_once(brain: Entity) {
    let mut guard = LAYER_ZERO_WARNED.lock().expect("layer warn mutex poisoned");
    if guard.contains(&brain.0) {
        return;
    }
    guard.push(brain.0);
    log::warn!(
        target: "harmonius_integration_rendering_camera",
        "render_layers is zero; substituting all bits for brain {}",
        brain.0
    );
}

/// Returns the camera bitmask after applying the render-layer substitution rule.
#[must_use]
pub fn effective_render_layers(raw: u32, brain: Entity) -> u32 {
    if raw != 0 {
        return raw;
    }
    warn_render_layers_zero_once(brain);
    0xFFFF_FFFF
}

fn camera_world_matrix(output: &CameraOutput) -> Mat4 {
    Mat4::from_rotation_translation(output.rotation, output.position)
}

fn maybe_warn_invalid_perspective(output: &CameraOutput) {
    if let Projection::Perspective { fov_y_radians, .. } = output.projection {
        if !fov_y_radians.is_finite() || fov_y_radians <= 0.0 {
            warn_fov_clamped_once(output.brain);
        }
    }
}

/// Builds a snapshot message for one active brain, or `None` when inactive.
#[must_use]
pub fn build_render_view_from_camera(
    output: &CameraOutput,
    drs_scale: f32,
    drs_min_scale: f32,
) -> Option<RenderViewFromCamera> {
    if !output.active {
        return None;
    }
    maybe_warn_invalid_perspective(output);
    let projection = projection_matrix(&output.projection);
    if projection.is_nan() {
        return None;
    }
    let world = camera_world_matrix(output);
    let view = world.inverse();
    let view_projection = projection * view;
    let (near_clip, far_clip) = clip_distances(&output.projection);
    let viewport = scale_viewport(output.viewport, drs_scale, drs_min_scale);
    let render_layers = effective_render_layers(output.render_layers, output.brain);
    Some(RenderViewFromCamera {
        stable_index: output.stable_index,
        view_matrix: view,
        projection,
        view_projection,
        camera_position: output.position,
        near_clip,
        far_clip,
        render_layers,
        render_order: output.render_order,
        viewport,
    })
}

fn fallback_render_view_from_camera() -> RenderViewFromCamera {
    let projection = projection_matrix(&Projection::Perspective {
        fov_y_radians: crate::projection::FOV_CLAMP_MIN_RADIANS,
        aspect: 1.0,
        near: 0.1,
        far: 1000.0,
    });
    let view = Mat4::IDENTITY;
    let view_projection = projection * view;
    RenderViewFromCamera {
        stable_index: u32::MAX,
        view_matrix: view,
        projection,
        view_projection,
        camera_position: Vec3::ZERO,
        near_clip: 0.1,
        far_clip: 1000.0,
        render_layers: 0xFFFF_FFFF,
        render_order: i32::MIN,
        viewport: Viewport {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
    }
}

/// Extracts every active camera, applies deterministic ordering, and appends a fallback
/// view when nothing was active.
#[must_use]
pub fn extract_sorted_render_views(
    outputs: &[CameraOutput],
    drs_scale: f32,
    drs_min_scale: f32,
) -> Vec<RenderViewFromCamera> {
    let mut views = Vec::new();
    for output in outputs {
        if let Some(view) = build_render_view_from_camera(output, drs_scale, drs_min_scale) {
            views.push(view);
        }
    }
    if views.is_empty() {
        warn_no_active_camera_once();
        views.push(fallback_render_view_from_camera());
    }
    views.sort_by(|a, b| {
        a.render_order
            .cmp(&b.render_order)
            .then_with(|| a.stable_index.cmp(&b.stable_index))
    });
    views
}

/// Converts a snapshot message into a render-graph view resource.
#[must_use]
pub fn render_view_from_camera_message(message: &RenderViewFromCamera) -> RenderView {
    RenderView {
        visibility_bits: message.render_layers,
        view_matrix: message.view_matrix,
        projection: message.projection,
        viewport: message.viewport,
    }
}

/// Sends every view in order across an MPSC channel (typically unbounded).
pub fn send_render_views_to_channel(
    views: &[RenderViewFromCamera],
    sender: &crossbeam_channel::Sender<RenderViewFromCamera>,
) {
    for view in views {
        let _ = sender.send(*view);
    }
}

/// Resets diagnostic warning gates (test hook).
#[cfg(test)]
pub fn reset_warning_gates_for_tests() {
    NO_ACTIVE_CAMERA_WARNED.store(false, Ordering::Relaxed);
    FOV_CLAMP_WARNED.lock().expect("mutex poisoned").clear();
    LAYER_ZERO_WARNED.lock().expect("mutex poisoned").clear();
}
