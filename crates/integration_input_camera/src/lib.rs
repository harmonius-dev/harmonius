//! Input ↔ camera integration (IR-4.1.*).
//!
//! Implements `CameraInputAxisController` bridging, blend suppression, aim assist hooks,
//! VR head pose evaluation, and `raw_camera_input` channel accounting per
//! `docs/design/integration/input-camera.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod action;
mod aim;
mod blend;
mod channel;
mod ciac;
mod components;
mod debug;
mod freelook;
mod modifiers;
mod vr;

pub use action::{ActionId, ActionState, ActionValue, ActionValueType, InputActionState};
pub use aim::{aim_deflect_look_delta, AimAssistConfig, AimAssistState};
pub use blend::BlendState;
pub use channel::{push_raw_camera_delta, RawCameraDelta, RawCameraInputBus};
pub use ciac::ciac_tick;
pub use components::{
    CameraInputAxisController, CameraOutput, Entity, FreeLookModifier, InputSensitivity,
    OrbitalFollow, PanTilt, VrCameraBrain, XrHeadPose,
};
pub use debug::InputCameraDebug;
pub use freelook::{free_look_sync, freelook_enabled};
pub use modifiers::{InputModifier, ModifierChain, ModifierState, ResponseCurveType};
pub use vr::{vr_camera_brain_tick, VrBrainResult};
