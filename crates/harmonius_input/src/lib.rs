//! Input subsystem: devices, action mapping, gestures, haptics, and VR-facing helpers.
//!
//! Implements the API shapes from `docs/design/input/input.md` for [PLAN-input-input].
//!
//! [PLAN-input-input]: https://github.com/cjhowe-us/harmonius/blob/main/docs/plans/input/input.md

#![deny(clippy::all)]
// Full public rustdoc will expand alongside ECS integration; keep the crate compiling for TDD.
#![allow(missing_docs)]

mod actions;
mod active_device;
mod binding;
mod buffer;
mod combo;
mod device;
mod gesture;
mod glyph;
mod haptics;
mod ids;
mod mapping;
mod modifiers;
mod rebind;
mod triggers;
mod value;
mod vr;

pub use actions::{
    ActionBinding, ActionDefinition, ActionId, ActionState, ContextId, ContextStack,
    GamepadStickSource, InputSource, MappingContext, MouseAxisSource,
};
pub use active_device::ActiveDeviceType;
pub use binding::{BindingError, BindingLoader};
pub use buffer::{ActionCategory, CancelWindow, InputBuffer};
pub use combo::{
    CardinalDirection, ComboEvaluator, ComboInput, ComboNode, ComboNodeId, ComboTree,
    normalize_cardinal_forward_from_sources, qcf_punch_tree,
};
pub use device::{
    DeviceCapabilities, DeviceConnected, DeviceId, DeviceInfo, DeviceManager, DeviceType, FingerId,
    GamepadAxis, GamepadButton, GamepadFamily, GamepadState, KeyboardState, MouseButton,
    MouseState, RawInputEvent, RawInputKind, Scancode, TouchContact, TouchState,
    gyro_integrate_pitch, normalize_native_w_key,
};
pub use gesture::{
    GestureEngine, GesturePhase, GestureType, LongPressRecognizer, PinchPanResult, ResolvedGesture,
    SwipeDirection,
};
pub use glyph::{DeviceFamily, GlyphResolver, ResolvedGlyph};
pub use haptics::{
    AdaptiveTriggerDriver, AdaptiveTriggerEffect, AudioHapticGenerator, DualMotorState,
    HapticBackendKind, HapticCommand, PatternPlayer, RumbleEnvelope, RumblePattern, TriggerEffect,
    encode_dual_motor_hid, normalize_intensity, parse_adaptive_profile_line,
};
pub use ids::{AssetId, ComboTreeId, GlyphAtlasId, Keycode, TouchRegionId};
pub use mapping::{InputMapper, MappingLoadError};
pub use modifiers::{InputModifier, ModifierChain, ModifierState, ResponseCurveType, SwizzleOrder};
pub use rebind::{RebindError, RebindManager, RebindRequest, RebindResolution, RebindResult};
pub use triggers::{TriggerCondition, TriggerPhase, TriggerState};
pub use value::{ActionValue, ActionValueType};
pub use vr::{
    Controller6DofSample, GazeBehavior, GazeClassifier, GazeEvent, GazeRaySample, HandJointPose,
    HandPinchDetector, HandSkeleton, HmdPoseSample, PlayAreaBoundaryEvent, PlayAreaMode,
    TrackingLossEvent, VrActionBridge, play_area_crossing, simulate_hmd_pose_stream,
    tracking_loss_event,
};
