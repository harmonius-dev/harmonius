//! `ContextStack` models UI vs gameplay mapping contexts.

use crate::types::{ContextId, InputConsumption, InputDebugFlags, MappingContext};

/// Priority-sorted stack of mapping contexts.
#[derive(Clone, Debug, Default)]
pub struct ContextStack {
    entries: Vec<MappingContext>,
    /// Diagnostic warnings (e.g., underflow).
    pub warnings: Vec<&'static str>,
}

impl ContextStack {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Pushes a context and keeps the stack sorted by descending `priority`.
    pub fn push(&mut self, ctx: MappingContext) {
        self.entries.push(ctx);
        self.entries.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Pops the highest-priority context if present.
    pub fn pop(&mut self, flags: &mut InputDebugFlags) -> Option<MappingContext> {
        if self.entries.is_empty() {
            if flags.log_consumption {
                self.warnings.push("context stack underflow");
            }
            return None;
        }
        Some(self.entries.remove(0))
    }

    /// Returns the active top context, if any.
    pub fn top(&self) -> Option<&MappingContext> {
        self.entries.first()
    }

    /// Returns true when pointer input should be routed to UI exclusively.
    pub fn pointer_consumed(&self) -> bool {
        self.top().is_some_and(|c| c.consumption.pointer)
    }

    /// Returns true when keyboard input should be routed to UI exclusively.
    pub fn keyboard_consumed(&self) -> bool {
        self.top().is_some_and(|c| c.consumption.keyboard)
    }

    /// Returns true when gamepad input should be routed to UI exclusively.
    pub fn gamepad_consumed(&self) -> bool {
        self.top().is_some_and(|c| c.consumption.gamepad)
    }
}

/// Records whether gameplay observed an input category.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GameplayObservation {
    /// Gameplay saw a pointer move.
    pub pointer_move: bool,
    /// Gameplay saw a key press.
    pub key_press: bool,
}

/// Applies consumption rules for a single frame of mixed inputs.
pub fn observe_gameplay_inputs(
    stack: &ContextStack,
    pointer_moved: bool,
    key_pressed: bool,
) -> GameplayObservation {
    let pointer_blocked = stack.pointer_consumed();
    let keyboard_blocked = stack.keyboard_consumed();
    GameplayObservation {
        pointer_move: pointer_moved && !pointer_blocked,
        key_press: key_pressed && !keyboard_blocked,
    }
}

/// Helper for menu open/close tests.
pub fn menu_context(menu_id: ContextId) -> MappingContext {
    MappingContext::ui(menu_id, 10, InputConsumption::all())
}

/// Helper for baseline gameplay contexts.
pub fn gameplay_context(id: ContextId) -> MappingContext {
    MappingContext::legacy(id, 0, false)
}
