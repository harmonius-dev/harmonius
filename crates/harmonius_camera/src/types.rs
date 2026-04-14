//! Core camera data types shared across behaviors.

use glam::{Quat, Vec2, Vec3};

use crate::ids::Entity;

/// Axis-aligned rectangle in normalized screen space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenRect {
    /// Minimum corner (x, y).
    pub min: Vec2,
    /// Maximum corner (x, y).
    pub max: Vec2,
}

/// Virtual camera description used by selection and evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualCamera {
    /// Numeric priority. Higher wins.
    pub priority: i32,
    /// Output channel mask for brain matching.
    pub channel_mask: u32,
    /// Render layer bitmask.
    pub render_layers: u32,
    /// Entity to track for position behaviors.
    pub tracking_target: Option<Entity>,
    /// Entity to aim at for rotation behaviors.
    pub look_at_target: Option<Entity>,
}

/// Projection mode for a camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraProjection {
    /// Standard perspective projection.
    Perspective {
        /// Vertical field of view (degrees).
        fov: f32,
    },
    /// Orthographic projection.
    Orthographic {
        /// Half-height of the view in world units.
        size: f32,
    },
}

/// Computed output from virtual camera evaluation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraOutput {
    /// World-space position.
    pub position: Vec3,
    /// World-space orientation.
    pub rotation: Quat,
    /// Active projection mode.
    pub projection: CameraProjection,
    pub near_clip: f32,
    pub far_clip: f32,
    pub focus_distance: f32,
}

/// Follow with fixed offset and six binding modes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Follow {
    /// Offset from tracking target.
    pub offset: Vec3,
    /// How offset relates to target rotation.
    pub binding_mode: FollowBindingMode,
    /// Per-axis position damping time constants (seconds).
    pub position_damping: Vec3,
    /// Angular damping time constant (seconds).
    pub angular_damping: f32,
}

/// Relationship between follow offset and target rotation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FollowBindingMode {
    WorldSpace,
    LockToTarget,
    LockToTargetNoRoll,
    LockWithWorldUp,
    LockOnAssign,
    LazyFollow,
}
