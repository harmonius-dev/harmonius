//! VR head pose → camera output override.

use crate::components::{CameraOutput, VrCameraBrain, XrHeadPose};
use crate::debug::InputCameraDebug;

/// Result of VR brain evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrBrainResult {
    /// Wrote stereo host output from `XrHeadPose`.
    Applied,
    /// Missing pose; standard brain should run.
    SkippedNoPose,
}

/// Apply `XrHeadPose` to `CameraOutput` when pose data exists.
///
/// When `xr` is `None`, returns [`VrBrainResult::SkippedNoPose`] without mutating `out`.
pub fn vr_camera_brain_tick(
    _brain: &VrCameraBrain,
    xr: Option<&XrHeadPose>,
    out: &mut CameraOutput,
    _debug: &mut InputCameraDebug,
) -> VrBrainResult {
    let Some(pose) = xr else {
        return VrBrainResult::SkippedNoPose;
    };
    out.position = pose.position;
    out.rotation = pose.rotation;
    VrBrainResult::Applied
}
