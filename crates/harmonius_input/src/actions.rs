//! Typed gameplay actions, sources, and mapping contexts.

use crate::device::{GamepadAxis, GamepadButton, Scancode};
use crate::ids::TouchRegionId;
use crate::modifiers::ModifierChain;
use crate::triggers::TriggerCondition;
use crate::value::{ActionValue, ActionValueType};

/// Stable action identifier (content hash in the full engine).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ActionId(pub u64);

/// Per-frame action resolution state.
#[derive(Clone, Debug, PartialEq)]
pub struct ActionState {
    /// Latest merged value.
    pub value: ActionValue,
    /// Trigger edge this frame.
    pub triggered: bool,
    /// Time in current phase for UI feedback.
    pub elapsed: f32,
    /// Trigger completed (tap / pulse windows).
    pub completed: bool,
}

/// Authored action metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct ActionDefinition {
    /// Action id.
    pub id: ActionId,
    /// Value shape.
    pub value_type: ActionValueType,
    /// Idle baseline.
    pub default_value: ActionValue,
}

/// Physical or logical input sources bound to actions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InputSource {
    /// Keyboard scancode.
    Key(Scancode),
    /// Mouse movement axis.
    MouseAxis(MouseAxisSource),
    /// Mouse button (not used in current unit tests).
    MouseButton(crate::device::MouseButton),
    /// Gamepad digital.
    GamepadButton(GamepadButton),
    /// Gamepad analog axis.
    GamepadAxis(GamepadAxis),
    /// Whole stick treated as 2D.
    GamepadStick(GamepadStickSource),
    /// Touch region (authored HUD joystick).
    TouchRegion(TouchRegionId),
}

/// Mouse axes that can feed analog actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseAxisSource {
    /// Horizontal motion delta.
    DeltaX,
    /// Vertical motion delta.
    DeltaY,
}

/// Which stick surface feeds a 2D action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadStickSource {
    /// Left analog stick.
    Left,
    /// Right analog stick.
    Right,
}

/// Stack entry identifier for a mapping layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextId(pub u64);

/// One authored mapping context (menu, gameplay, vehicle, ...).
#[derive(Clone, Debug, PartialEq)]
pub struct MappingContext {
    /// Context id.
    pub id: ContextId,
    /// Sort key: larger binds first when consuming.
    pub priority: i32,
    /// Bindings owned by this context.
    pub bindings: Vec<ActionBinding>,
    /// When true, matched bindings block lower contexts for the same physical input.
    pub consumes_input: bool,
}

/// One concrete binding from a source to an action with modifiers and trigger rules.
#[derive(Clone, Debug, PartialEq)]
pub struct ActionBinding {
    /// Target action.
    pub action_id: ActionId,
    /// Physical source.
    pub source: InputSource,
    /// Modifier stack applied before triggers.
    pub modifiers: ModifierChain,
    /// Trigger rule.
    pub trigger: TriggerCondition,
}

#[derive(Clone, Debug)]
struct ActiveContext {
    id: ContextId,
    priority: i32,
    mapping: MappingContext,
}

/// Priority stack of mapping contexts (top wins for conflicts when consuming).
#[derive(Clone, Debug, Default)]
pub struct ContextStack {
    stack: Vec<ActiveContext>,
}

impl ContextStack {
    /// Push a context; duplicates replace the previous copy.
    pub fn push(&mut self, ctx: MappingContext) {
        self.pop(ctx.id);
        self.stack.push(ActiveContext {
            id: ctx.id,
            priority: ctx.priority,
            mapping: ctx,
        });
        self.stack.sort_by_key(|e| e.priority);
    }

    /// Remove a context by id if present.
    pub fn pop(&mut self, id: ContextId) {
        self.stack.retain(|e| e.id != id);
    }

    /// Iterate active contexts from **highest** priority to lowest.
    pub fn iter_high_to_low(&self) -> impl Iterator<Item = &MappingContext> {
        self.stack.iter().rev().map(|e| &e.mapping)
    }

    /// Returns true when the context id is currently stacked.
    pub fn is_active(&self, id: ContextId) -> bool {
        self.stack.iter().any(|e| e.id == id)
    }
}
