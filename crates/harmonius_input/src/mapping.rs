//! Stack-aware binding resolution for keyboard and analog sources.

use crate::actions::{
    ActionBinding, ActionId, ContextId, ContextStack, GamepadStickSource, InputSource,
    MappingContext,
};
use crate::device::{GamepadAxis, GamepadState, KeyboardState, Scancode};
use crate::ids::TouchRegionId;
use glam::Vec2;
use std::collections::HashSet;

/// Errors when loading authored mapping graphs (editor files).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MappingLoadError {
    /// Duplicate context id in one batch.
    DuplicateContext,
}

/// Resolves actions from stacked mapping contexts.
#[derive(Clone, Debug, Default)]
pub struct InputMapper;

impl InputMapper {
    /// Load contexts into a fresh stack (editor export path).
    pub fn load_contexts(contexts: Vec<MappingContext>) -> Result<ContextStack, MappingLoadError> {
        let mut seen = HashSet::new();
        for c in &contexts {
            if !seen.insert(c.id) {
                return Err(MappingLoadError::DuplicateContext);
            }
        }
        let mut stack = ContextStack::default();
        for c in contexts {
            stack.push(c);
        }
        Ok(stack)
    }

    /// Resolve a keyboard press through the stack (highest priority first).
    pub fn resolve_key_press(stack: &ContextStack, key: Scancode) -> Vec<(ContextId, ActionId)> {
        let mut out = Vec::new();
        for ctx in stack.iter_high_to_low() {
            let mut matched_here = false;
            for b in &ctx.bindings {
                if let InputSource::Key(sc) = &b.source
                    && *sc == key
                {
                    out.push((ctx.id, b.action_id));
                    matched_here = true;
                    break;
                }
            }
            if matched_here && ctx.consumes_input {
                break;
            }
        }
        out
    }

    /// Resolve `Move` (Axis2D) from keyboard WASD held state.
    pub fn axis2d_from_wasd(keyboard: &KeyboardState) -> Vec2 {
        let mut v = Vec2::ZERO;
        if keyboard.is_pressed(Scancode::W) {
            v.y -= 1.0;
        }
        if keyboard.is_pressed(Scancode::S) {
            v.y += 1.0;
        }
        if keyboard.is_pressed(Scancode::A) {
            v.x -= 1.0;
        }
        if keyboard.is_pressed(Scancode::D) {
            v.x += 1.0;
        }
        if v.length_squared() > 1e-8 {
            v.normalize()
        } else {
            v
        }
    }

    /// Resolve Axis2D from a gamepad stick snapshot.
    pub fn axis2d_from_stick(gamepad: &GamepadState, stick: GamepadStickSource) -> Vec2 {
        match stick {
            GamepadStickSource::Left => gamepad.left_stick,
            GamepadStickSource::Right => gamepad.right_stick,
        }
    }

    /// Virtual on-screen joystick value for `TouchRegion` bindings.
    pub fn axis2d_from_touch_region(_id: TouchRegionId, value: Vec2) -> Vec2 {
        if value.length_squared() > 1e-8 {
            value.normalize()
        } else {
            value
        }
    }

    /// Find the first binding for `action_id` in priority order (for glyph lookup).
    pub fn find_binding_for_action(
        stack: &ContextStack,
        action_id: ActionId,
    ) -> Option<ActionBinding> {
        for ctx in stack.iter_high_to_low() {
            for b in &ctx.bindings {
                if b.action_id == action_id {
                    return Some(b.clone());
                }
            }
        }
        None
    }

    /// Resolve Axis1D from gamepad trigger axis.
    pub fn axis1d_from_trigger(gamepad: &GamepadState, axis: GamepadAxis) -> f32 {
        match axis {
            GamepadAxis::RightTrigger => gamepad.right_trigger,
            GamepadAxis::LeftTrigger => gamepad.left_trigger,
            _ => 0.0,
        }
    }
}
