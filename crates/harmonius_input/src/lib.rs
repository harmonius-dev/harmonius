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
mod value;
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
mod vr;

pub use actions::{
    ActionBinding, ActionDefinition, ActionId, ActionState, ContextId, ContextStack, GamepadStickSource,
    InputSource, MappingContext, MouseAxisSource,
};
pub use value::{ActionValue, ActionValueType};
pub use active_device::ActiveDeviceType;
pub use binding::{BindingError, BindingLoader};
pub use buffer::{ActionCategory, CancelWindow, InputBuffer};
pub use combo::{
    normalize_cardinal_forward_from_sources, CardinalDirection, ComboEvaluator, ComboInput,
    ComboNode, ComboNodeId, ComboTree, ComboTreeId, qcf_punch_tree,
};
pub use device::{
    gyro_integrate_pitch, normalize_native_w_key, DeviceCapabilities, DeviceConnected, DeviceId,
    DeviceInfo, DeviceManager, DeviceType, FingerId, GamepadAxis, GamepadButton, GamepadFamily,
    GamepadState, KeyboardState, MouseButton, MouseState, RawInputEvent, RawInputKind, Scancode,
    TouchContact, TouchState,
};
pub use gesture::{
    GestureEngine, GesturePhase, GestureType, LongPressRecognizer, PinchPanResult, ResolvedGesture,
    SwipeDirection,
};
pub use glyph::{DeviceFamily, GlyphResolver, ResolvedGlyph};
pub use haptics::{
    encode_dual_motor_hid, normalize_intensity, parse_adaptive_profile_line, AdaptiveTriggerDriver,
    AdaptiveTriggerEffect, AudioHapticGenerator, DualMotorState, HapticBackendKind, HapticCommand,
    PatternPlayer, RumbleEnvelope, RumblePattern, TriggerEffect,
};
pub use ids::{AssetId, GlyphAtlasId, Keycode, TouchRegionId};
pub use mapping::{InputMapper, MappingLoadError};
pub use modifiers::{InputModifier, ModifierChain, ModifierState, ResponseCurveType, SwizzleOrder};
pub use rebind::{RebindError, RebindManager, RebindRequest, RebindResolution, RebindResult};
pub use triggers::{TriggerCondition, TriggerPhase, TriggerState};
pub use vr::{
    play_area_crossing, simulate_hmd_pose_stream, tracking_loss_event, Controller6DofSample,
    GazeBehavior, GazeClassifier, GazeEvent, GazeRaySample, HandJointPose, HandPinchDetector,
    HandSkeleton, HmdPoseSample, PlayAreaBoundaryEvent, PlayAreaMode, TrackingLossEvent,
    VrActionBridge,
};
