//! Authoritative camera override stores written by timeline bindings.

use glam::Vec3;

/// Runtime override stack consumed by camera evaluation (design IR-4.8.4).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Recomposer {
    /// Camera-local position offset authored by timelines.
    pub position_offset: Vec3,
    /// Euler rotation offset in degrees (pitch, yaw, roll).
    pub rotation_offset: Vec3,
    /// Additive vertical FOV delta in degrees.
    pub fov_delta: f32,
    /// Dutch roll in degrees (unused by this integration).
    pub dutch: f32,
    /// Blend weight between gameplay and cinematic stacks.
    pub blend_weight: f32,
}

impl Default for Recomposer {
    fn default() -> Self {
        Self {
            position_offset: Vec3::ZERO,
            rotation_offset: Vec3::ZERO,
            fov_delta: 0.0,
            dutch: 0.0,
            blend_weight: 0.0,
        }
    }
}

/// Normalized spline parameter along a dolly path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DollyRig {
    /// Normalized position in `[0.0, 1.0]`.
    pub position: f32,
}

impl Default for DollyRig {
    fn default() -> Self {
        Self { position: 0.0 }
    }
}
