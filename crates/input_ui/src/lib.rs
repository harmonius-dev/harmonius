//! Input–UI integration primitives aligned with `docs/design/integration/input-ui.md`.
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod context;
mod dispatch;
mod focus;
mod gesture;
mod router;
mod text;
mod types;
mod vr;

pub use context::{
    gameplay_context, menu_context, observe_gameplay_inputs, ContextStack, GameplayObservation,
};
pub use dispatch::{
    ui_input_dispatch_system_body, PointerChannelFull, PointerDispatchCursor, PointerDispatchEnv,
    PointerEventReceiver, POINTER_CHANNEL_CAPACITY,
};
pub use focus::{FocusGroup, FocusManager, Focusable, WidgetCommand};
pub use gesture::{
    DragDropManager, GestureEvent, GesturePhase, GestureType, ScrollView, SwipeDirection,
};
pub use router::{DispatchLog, EventRouter, HandlerPolicy, WidgetSpec, WidgetTree};
pub use text::{gameplay_observes_move_forward_key, FocusCommitPolicy, TextInput};
pub use types::{
    ContextId, ControllerPose, Entity, EventPhase, InputConsumption, InputDebugFlags,
    InteractionState, MappingContext, MouseButton, Rect, TrackingStatus, UiInputContext,
    UiPointerEvent, Vec2, VrControllerState,
};
pub use vr::{ray_panel_hit, PanelSpec};
