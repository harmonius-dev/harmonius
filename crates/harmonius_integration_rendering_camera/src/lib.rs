//! Rendering ↔ camera integration helpers from `docs/design/integration/rendering-camera.md`.
//!
//! The crate exposes deterministic, CPU-side math and small wiring helpers (MPSC sends, bounded
//! DRS feedback queues) that match the integration design contracts.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod drs;
mod post_process;
mod projection;
mod snapshot;
mod types;
mod uniform;
mod visibility;

#[cfg(test)]
mod tests;

/// Clears render/camera integration warning gates between unit tests.
#[cfg(test)]
pub fn reset_warning_gates_for_tests() {
    snapshot::reset_warning_gates_for_tests();
}

pub use drs::{clamp_drs_scale, drs_pi_step, scale_viewport, DrsFeedbackQueue};
pub use post_process::{aabb_contains, resolve_post_process_stack};
pub use projection::{
    clamp_fov_y_radians, clip_distances, projection_matrix, FOV_CLAMP_MAX_RADIANS,
    FOV_CLAMP_MIN_RADIANS,
};
pub use snapshot::{
    build_render_view_from_camera, effective_render_layers, extract_sorted_render_views,
    render_view_from_camera_message, send_render_views_to_channel,
};
pub use types::{
    CameraOutput, DynamicResolutionState, Entity, PostProcessBlend, PostProcessParams,
    PostProcessStack, PostProcessVolume, Projection, RenderView, RenderViewFromCamera, ViewUniform,
    Viewport,
};
pub use uniform::view_uniform_from_render_view;
pub use visibility::filter_visible_entities;
