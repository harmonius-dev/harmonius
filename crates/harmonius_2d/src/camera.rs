//! Orthographic camera helpers: parallax, shake decay, pixel snapping.

use glam::Vec2;

/// Parallax world offset for a layer given camera translation and uniform factor (`TC-10.5.9.1`).
#[must_use]
pub fn parallax_layer_offset(camera_position: Vec2, parallax_factor: f32) -> Vec2 {
    camera_position * parallax_factor
}

/// Linear decay of shake amplitude over elapsed time (`TC-10.5.9.2`).
#[must_use]
pub fn shake_amplitude_linear(initial: f32, duration: f32, elapsed: f32) -> f32 {
    if duration <= 0.0 {
        return 0.0;
    }
    let t = (elapsed / duration).clamp(0.0, 1.0);
    initial * (1.0 - t)
}

/// Snap camera position to integer world units for pixel-perfect rendering (`TC-10.5.9.3`).
#[must_use]
pub fn pixel_perfect_camera_position(position: Vec2) -> Vec2 {
    Vec2::new(position.x.round(), position.y.round())
}
