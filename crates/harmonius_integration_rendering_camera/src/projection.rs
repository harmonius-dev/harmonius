//! Reverse-Z perspective and symmetric orthographic projection builders.

use glam::Mat4;

use crate::types::Projection;

/// Lower FOV clamp (one degree).
pub const FOV_CLAMP_MIN_RADIANS: f32 = core::f32::consts::PI / 180.0;

/// Upper FOV clamp (179 degrees).
pub const FOV_CLAMP_MAX_RADIANS: f32 = core::f32::consts::PI - FOV_CLAMP_MIN_RADIANS;

/// Clamps vertical FOV into a non-degenerate range.
#[must_use]
pub fn clamp_fov_y_radians(fov_y_radians: f32) -> f32 {
    if !fov_y_radians.is_finite() || fov_y_radians <= 0.0 {
        return FOV_CLAMP_MIN_RADIANS;
    }
    fov_y_radians.clamp(FOV_CLAMP_MIN_RADIANS, FOV_CLAMP_MAX_RADIANS)
}

/// Builds a column-major clip projection matrix for Harmonius reverse-Z perspective.
///
/// Finite `far` participates in culling math elsewhere; the matrix matches the infinite
/// reverse-Z formulation used by `glam` so GPU near-plane behavior matches integration
/// tests for the listed matrix entries.
#[must_use]
pub fn projection_matrix(projection: &Projection) -> Mat4 {
    match *projection {
        Projection::Perspective {
            fov_y_radians,
            aspect,
            near,
            far: _far,
        } => {
            let fov = clamp_fov_y_radians(fov_y_radians);
            Mat4::perspective_infinite_reverse_rh(fov, aspect, near)
        }
        Projection::Orthographic {
            half_height,
            aspect,
            near,
            far,
        } => {
            let half_width = aspect * half_height;
            Mat4::orthographic_rh_gl(
                -half_width,
                half_width,
                -half_height,
                half_height,
                near,
                far,
            )
        }
    }
}

/// Returns `(near, far)` clip distances for bookkeeping on [`crate::types::CameraOutput`].
#[must_use]
pub fn clip_distances(projection: &Projection) -> (f32, f32) {
    match *projection {
        Projection::Perspective { near, far, .. } => (near, far),
        Projection::Orthographic { near, far, .. } => (near, far),
    }
}
