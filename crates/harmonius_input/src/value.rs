//! Action value shapes shared by modifiers and mapping.

use glam::{Vec2, Vec3};

/// Runtime value carried by an action.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionValue {
    /// Digital gate.
    Bool(bool),
    /// Single analog axis -1..1 (axis conventions are binding-defined).
    Axis1D(f32),
    /// Two-axis value (sticks, WASD plane).
    Axis2D(Vec2),
    /// Three-axis value (flight / 6DOF).
    Axis3D(Vec3),
}

/// Declared shape for an authored action.
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
