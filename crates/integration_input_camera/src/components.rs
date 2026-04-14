//! Camera and integration components.

use crate::action::ActionId;
use glam::{Quat, Vec3};

/// Lightweight entity handle for integration tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity(pub u32);

/// Per-axis sensitivity (user preference scale factors).
///
/// The integration design attaches `rkyv` at the settings persistence boundary; this crate keeps
/// the runtime shape dependency-free for isolated CI builds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputSensitivity {
    /// Horizontal sensitivity multiplier.
    pub x: f32,
    /// Vertical sensitivity multiplier.
    pub y: f32,
}

/// Bridge between input actions and camera rotation.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraInputAxisController {
    /// Action id for the look/orbit action.
    pub look_action: ActionId,
    /// Per-axis sensitivity multiplier.
    pub sensitivity: InputSensitivity,
    /// Invert Y axis.
    pub invert_y: bool,
    /// When true, input is suppressed during blends.
    pub suppress_during_blend: bool,
    /// Maximum seconds before suppression auto-clears.
    pub suppression_timeout: f32,
    /// Elapsed time since suppression began (runtime).
    pub suppression_elapsed: f32,
}

/// Yaw/pitch camera behavior component.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanTilt {
    /// Horizontal rotation (radians).
    pub yaw: f32,
    /// Vertical rotation (radians).
    pub pitch: f32,
}

/// Orbital follow angles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbitalFollow {
    /// Horizontal orbit angle (radians).
    pub horizontal: f32,
    /// Vertical orbit angle (radians).
    pub vertical: f32,
    /// Follow distance (meters).
    pub distance: f32,
}

/// Free look marker (presence = enabled).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FreeLookModifier;

/// Final camera pose output.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraOutput {
    /// World position.
    pub position: Vec3,
    /// World rotation.
    pub rotation: Quat,
}

/// VR head pose in tracking space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XrHeadPose {
    /// Head position in tracking space.
    pub position: Vec3,
    /// Head orientation in tracking space.
    pub rotation: Quat,
}

/// VR-specific camera brain parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VrCameraBrain {
    /// Interpupillary distance (meters).
    pub ipd: f32,
    /// Whether late-latch is enabled for this brain.
    pub late_latch: bool,
}
