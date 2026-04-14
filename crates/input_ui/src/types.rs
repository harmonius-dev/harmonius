//! Shared value types for input–UI integration.

/// Stable entity identifier for widgets and panels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Entity(pub u32);

/// 2D vector in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl Vec2 {
    /// Constructs a vector.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Axis-aligned rectangle in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Minimum corner.
    pub min: Vec2,
    /// Maximum corner.
    pub max: Vec2,
}

impl Rect {
    /// Constructs a rectangle from origin and size.
    pub const fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x + w, y + h),
        }
    }

    /// Returns true when `p` lies inside the rectangle (inclusive bounds).
    pub fn contains(self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Rectangle center.
    pub fn center(self) -> Vec2 {
        Vec2::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
        )
    }
}

/// Mouse / pointer button mapping used by `UiPointerEvent`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MouseButton {
    /// Primary button (left click / VR trigger).
    Primary,
    /// Secondary button.
    Secondary,
}

/// Pointer events delivered to widgets after hit testing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiPointerEvent {
    /// Hover entered a widget.
    Enter {
        /// Cursor position in logical pixels.
        position: Vec2,
    },
    /// Button pressed.
    Down {
        /// Cursor position in logical pixels.
        position: Vec2,
        /// Button that transitioned down.
        button: MouseButton,
    },
    /// Button released.
    Up {
        /// Cursor position in logical pixels.
        position: Vec2,
        /// Button that transitioned up.
        button: MouseButton,
    },
    /// Hover left a widget.
    Leave,
    /// Move while hovering.
    Move {
        /// Cursor position in logical pixels.
        position: Vec2,
        /// Delta since last move.
        delta: Vec2,
    },
}

/// Propagation phase for routed UI events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventPhase {
    /// Capture traversal (root toward target).
    Capture,
    /// Target-only delivery.
    Target,
    /// Bubble traversal (target toward root).
    Bubble,
}

/// Interaction flags stored on a widget entity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InteractionState {
    /// Pointer is over the widget rect.
    pub hovered: bool,
    /// Pointer is pressed on the widget.
    pub pressed: bool,
    /// Widget has keyboard focus.
    pub focused: bool,
    /// Widget ignores target delivery.
    pub disabled: bool,
}

/// Per-category consumption flags for UI mapping contexts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputConsumption {
    /// When true, pointer events are consumed by the active UI context.
    pub pointer: bool,
    /// When true, keyboard events are consumed by the active UI context.
    pub keyboard: bool,
    /// When true, gamepad events are consumed by the active UI context.
    pub gamepad: bool,
}

impl InputConsumption {
    /// All categories consumed.
    pub const fn all() -> Self {
        Self {
            pointer: true,
            keyboard: true,
            gamepad: true,
        }
    }

    /// Chat-style consumption: keyboard only.
    pub const fn chat_window() -> Self {
        Self {
            pointer: false,
            keyboard: true,
            gamepad: false,
        }
    }
}

/// Identifier for a mapping context on the stack.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ContextId(pub u32);

/// Mapping context entry with priority and consumption semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MappingContext {
    /// Stable context identifier.
    pub context_id: ContextId,
    /// Higher priority sorts above lower priority contexts.
    pub priority: i32,
    /// Legacy OR of the three `InputConsumption` flags.
    pub consumes_input: bool,
    /// Fine-grained consumption flags.
    pub consumption: InputConsumption,
}

impl MappingContext {
    /// Creates a gameplay-oriented context using legacy `consumes_input`.
    pub const fn legacy(context_id: ContextId, priority: i32, consumes_input: bool) -> Self {
        let consumption = InputConsumption {
            pointer: consumes_input,
            keyboard: consumes_input,
            gamepad: consumes_input,
        };
        Self {
            context_id,
            priority,
            consumes_input,
            consumption,
        }
    }

    /// Creates a UI context with explicit per-category consumption.
    pub const fn ui(context_id: ContextId, priority: i32, consumption: InputConsumption) -> Self {
        let consumes_input = consumption.pointer || consumption.keyboard || consumption.gamepad;
        Self {
            context_id,
            priority,
            consumes_input,
            consumption,
        }
    }
}

/// VR controller tracking quality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrackingStatus {
    /// Tracking is valid.
    Ok,
    /// Tracking is unavailable.
    Lost,
}

/// World-space controller pose used for laser tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControllerPose {
    /// Hand position in world units.
    pub position: Vec2,
    /// Forward direction in the XY plane (normalized).
    pub forward: Vec2,
    /// Tracking state.
    pub tracking: TrackingStatus,
}

/// Digital inputs on a VR controller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VrControllerState {
    /// Trigger pressed.
    pub trigger_pressed: bool,
    /// Grip pressed.
    pub grip_pressed: bool,
    /// Bitmask of auxiliary buttons.
    pub buttons: u32,
}

/// Runtime toggles for integration diagnostics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputDebugFlags {
    /// Log routed events.
    pub trace_events: bool,
    /// Draw hit-test overlay.
    pub show_hit_overlay: bool,
    /// Log consumption decisions.
    pub log_consumption: bool,
}
