//! Minimal action identifiers and values for CIAC integration tests.

use glam::{Vec2, Vec3};
use std::collections::HashMap;

/// Stable action identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ActionId(pub u64);

/// Runtime value carried by an action.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionValue {
    /// Digital gate.
    Bool(bool),
    /// Single analog axis.
    Axis1D(f32),
    /// Two-axis value (mouse delta, stick).
    Axis2D(Vec2),
    /// Three-axis value.
    Axis3D(Vec3),
}

/// Declared shape for an authored action (used for mismatch diagnostics).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActionValueType {
    /// Boolean action.
    Bool,
    /// Scalar axis.
    Axis1D,
    /// 2D plane.
    Axis2D,
    /// 3D vector.
    Axis3D,
}

/// Per-frame action resolution state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionState {
    /// Latest merged value.
    pub value: ActionValue,
    /// Trigger edge this frame.
    pub triggered: bool,
    /// Time in current phase for UI feedback.
    pub elapsed: f32,
    /// Trigger completed (tap / pulse windows).
    pub completed: bool,
    /// Declared value shape for this binding.
    pub value_type: ActionValueType,
}

/// ECS-resident action table (Phase 1 writer, Phase 6 reader).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputActionState {
    /// Latest `ActionState` per `ActionId`.
    pub states: HashMap<ActionId, ActionState>,
}

impl InputActionState {
    /// Lookup action state.
    pub fn get(&self, id: ActionId) -> Option<&ActionState> {
        self.states.get(&id)
    }

    /// Insert or replace an action state.
    pub fn insert(&mut self, id: ActionId, state: ActionState) {
        self.states.insert(id, state);
    }
}
