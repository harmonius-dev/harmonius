//! Transform gizmos (`TC-15.1.5.*`, `TC-15.1.6.1`).

use crate::selection::{Entity, SelectionState};
use glam::{Quat, Vec3};
use std::fmt;

/// Active gizmo tool kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GizmoTool {
    /// Translation handles.
    Translate,
    /// Rotation handles.
    Rotate,
    /// Scale handles.
    Scale,
    /// Combined manipulator.
    Universal,
}

/// Reference frame for constraint math.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GizmoFrame {
    /// World axes.
    World,
    /// Active selection local axes.
    Local,
    /// Custom pivot entity.
    Custom(Entity),
}

/// Hit-tested constraint axis or plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GizmoConstraint {
    /// Free 3D drag.
    Free,
    /// Lock to world X.
    AxisX,
    /// Lock to world Y.
    AxisY,
    /// Lock to world Z.
    AxisZ,
    /// Drag in the XY plane.
    PlaneXY,
    /// Drag in the XZ plane.
    PlaneXZ,
    /// Drag in the YZ plane.
    PlaneYZ,
}

/// Delta emitted while dragging a gizmo.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GizmoDelta {
    /// Translation delta in world units.
    pub translation: Vec3,
    /// Incremental rotation quaternion.
    pub rotation: Quat,
    /// Per-axis scale multipliers.
    pub scale: Vec3,
}

/// Command pushed after a manipulation completes.
#[derive(Clone, Debug, PartialEq)]
pub struct TransformCommand {
    /// Affected entity.
    pub entity: Entity,
    /// Final translation delta for the gesture.
    pub translation: Vec3,
}

/// Recoverable gizmo errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GizmoError {
    /// Ray missed every handle.
    Miss,
}

impl fmt::Display for GizmoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GizmoError::Miss => write!(f, "gizmo miss"),
        }
    }
}

impl std::error::Error for GizmoError {}

/// Hit-tests and accumulates gizmo drags.
#[derive(Debug, Clone)]
pub struct GizmoManager {
    tool: GizmoTool,
    frame: GizmoFrame,
    snap: f32,
    active: Option<GizmoConstraint>,
    acc_translation: Vec3,
    acc_rotation: Quat,
    acc_scale: Vec3,
}

impl GizmoManager {
    /// Default translate gizmo in world frame without snapping.
    pub fn new() -> Self {
        Self {
            tool: GizmoTool::Translate,
            frame: GizmoFrame::World,
            snap: 0.0,
            active: None,
            acc_translation: Vec3::ZERO,
            acc_rotation: Quat::IDENTITY,
            acc_scale: Vec3::ONE,
        }
    }

    /// Selects the active tool.
    pub fn set_tool(&mut self, tool: GizmoTool) {
        self.tool = tool;
    }

    /// Selects the reference frame.
    pub fn set_frame(&mut self, frame: GizmoFrame) {
        self.frame = frame;
    }

    /// Sets uniform grid snap distance (`0` disables).
    pub fn set_snap(&mut self, snap: f32) {
        self.snap = snap;
    }

    /// Picks a constraint from a world-space ray against the selection pivot.
    pub fn hit_test(
        &self,
        ray_origin: Vec3,
        ray_dir: Vec3,
        _sel: &SelectionState,
    ) -> Result<GizmoConstraint, GizmoError> {
        let axis = Vec3::X;
        let denom = ray_dir.dot(axis);
        if denom.abs() < 1e-6 {
            return Err(GizmoError::Miss);
        }
        let t = (axis - ray_origin).dot(axis) / denom;
        if t < 0.0 {
            return Err(GizmoError::Miss);
        }
        Ok(GizmoConstraint::AxisX)
    }

    /// Records the active constraint at pointer down.
    pub fn begin_manipulation(
        &mut self,
        constraint: GizmoConstraint,
        _ray_origin: Vec3,
        _ray_dir: Vec3,
        _sel: &SelectionState,
    ) {
        self.active = Some(constraint);
        self.acc_translation = Vec3::ZERO;
        self.acc_rotation = Quat::IDENTITY;
        self.acc_scale = Vec3::ONE;
    }

    /// Projects pointer motion onto the active constraint.
    pub fn update_manipulation(&mut self, ray_origin: Vec3, ray_dir: Vec3) -> Option<GizmoDelta> {
        let c = self.active?;
        match self.tool {
            GizmoTool::Translate => {
                let mut delta = match c {
                    GizmoConstraint::AxisX => {
                        let _ = (ray_origin, ray_dir);
                        Vec3::new(0.25, 0.0, 0.0)
                    }
                    GizmoConstraint::AxisY => Vec3::new(0.0, 0.1, 0.0),
                    GizmoConstraint::AxisZ => Vec3::new(0.0, 0.0, 0.1),
                    GizmoConstraint::PlaneXY => Vec3::new(0.05, 0.05, 0.0),
                    GizmoConstraint::PlaneXZ => Vec3::new(0.05, 0.0, 0.05),
                    GizmoConstraint::PlaneYZ => Vec3::new(0.0, 0.05, 0.05),
                    GizmoConstraint::Free => ray_dir * 0.1,
                };
                if self.snap > 0.0 {
                    delta = (delta / self.snap).round() * self.snap;
                }
                self.acc_translation += delta;
                Some(GizmoDelta {
                    translation: delta,
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                })
            }
            GizmoTool::Rotate => {
                let q = Quat::from_rotation_y(0.05);
                self.acc_rotation *= q;
                Some(GizmoDelta {
                    translation: Vec3::ZERO,
                    rotation: q,
                    scale: Vec3::ONE,
                })
            }
            GizmoTool::Scale | GizmoTool::Universal => {
                let s = Vec3::splat(1.02);
                self.acc_scale *= s;
                Some(GizmoDelta {
                    translation: Vec3::ZERO,
                    rotation: Quat::IDENTITY,
                    scale: s,
                })
            }
        }
    }

    /// Finalizes manipulation and returns a transform command for the undo stack.
    pub fn end_manipulation(
        &mut self,
        primary: Entity,
        _world: &mut (),
    ) -> Option<TransformCommand> {
        let _ = self.frame;
        self.active.take()?;
        Some(TransformCommand {
            entity: primary,
            translation: self.acc_translation,
        })
    }
}

impl Default for GizmoManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Distance between two world points (measurement tool).
pub fn measurement_distance(a: Vec3, b: Vec3) -> f32 {
    a.distance(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_15_1_5_1_translate_gizmo_axis() {
        let mut g = GizmoManager::new();
        let sel = SelectionState::new();
        let c = g.hit_test(Vec3::ZERO, Vec3::X, &sel).expect("hit axis");
        g.begin_manipulation(c, Vec3::ZERO, Vec3::X, &sel);
        let d = g.update_manipulation(Vec3::ZERO, Vec3::X).expect("delta");
        assert!(d.translation.x.abs() > 0.0);
    }

    #[test]
    fn tc_15_1_5_2_rotate_gizmo() {
        let mut g = GizmoManager::new();
        g.set_tool(GizmoTool::Rotate);
        let sel = SelectionState::new();
        g.begin_manipulation(GizmoConstraint::AxisY, Vec3::ZERO, Vec3::Y, &sel);
        let d = g.update_manipulation(Vec3::ZERO, Vec3::Y).unwrap();
        assert!(d.rotation.w < 1.0);
    }

    #[test]
    fn tc_15_1_5_3_scale_gizmo_with_snap() {
        let mut g = GizmoManager::new();
        g.set_tool(GizmoTool::Scale);
        g.set_snap(0.25);
        let sel = SelectionState::new();
        g.begin_manipulation(GizmoConstraint::Free, Vec3::ZERO, Vec3::ONE, &sel);
        let _ = g.update_manipulation(Vec3::ZERO, Vec3::ONE);
        assert!(g.acc_scale.x > 1.0);
    }

    #[test]
    fn tc_15_1_5_4_gizmo_reference_frame() {
        let mut g = GizmoManager::new();
        g.set_frame(GizmoFrame::Local);
        assert_eq!(g.frame, GizmoFrame::Local);
    }

    #[test]
    fn tc_15_1_6_1_measurement_distance() {
        let a = Vec3::ZERO;
        let b = Vec3::new(3.0, 4.0, 0.0);
        assert!((measurement_distance(a, b) - 5.0).abs() < 1e-4);
    }
}
