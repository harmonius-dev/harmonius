//! Undoable editor commands touching physics components.

use std::mem::size_of;

use crate::math::Vec3;
use crate::model::{ColliderShape, CompoundChild, Entity, PhysicsMaterialHandle};
use crate::world::World;

/// Errors returned by editor command execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandError {
    /// Input violated numeric or structural constraints.
    InvalidInput,
    /// Index out of range for compound edits.
    InvalidIndex,
    /// Entity has no `Collider`.
    MissingCollider,
    /// Entity has no `CompoundCollider`.
    MissingCompound,
}

/// Editor undo stack contract (`docs/design/tools/editor-core.md` subset).
pub trait EditorCommand: Send {
    /// Human-readable label for UI / logs.
    fn description(&self) -> &str;

    /// Applies the forward edit to `world`.
    fn execute(&mut self, world: &mut World) -> Result<(), CommandError>;

    /// Reverts `execute` using stored prior state.
    fn undo(&mut self, world: &mut World) -> Result<(), CommandError>;

    /// Memory charge for undo budgeting.
    fn size_bytes(&self) -> usize;
}

const MIN_LINEAR_DIM: f32 = 0.01;

fn f32_ok(v: f32) -> bool {
    v.is_finite()
}

fn clamp_pos_dim(v: f32) -> f32 {
    if !v.is_finite() || v < MIN_LINEAR_DIM {
        MIN_LINEAR_DIM
    } else {
        v
    }
}

fn clamp_shape_dims(shape: &ColliderShape) -> ColliderShape {
    match shape {
        ColliderShape::Box { half_extents } => ColliderShape::Box {
            half_extents: Vec3 {
                x: clamp_pos_dim(half_extents.x),
                y: clamp_pos_dim(half_extents.y),
                z: clamp_pos_dim(half_extents.z),
            },
        },
        ColliderShape::Sphere { radius } => ColliderShape::Sphere {
            radius: clamp_pos_dim(*radius),
        },
        ColliderShape::Capsule {
            half_height,
            radius,
        } => ColliderShape::Capsule {
            half_height: clamp_pos_dim(*half_height),
            radius: clamp_pos_dim(*radius),
        },
        ColliderShape::ConvexHull(h) => ColliderShape::ConvexHull(h.clone()),
        ColliderShape::TriMesh(m) => ColliderShape::TriMesh(m.clone()),
        ColliderShape::Heightfield => ColliderShape::Heightfield,
    }
}

fn shape_has_nan(shape: &ColliderShape) -> bool {
    match shape {
        ColliderShape::Box { half_extents } => {
            !f32_ok(half_extents.x) || !f32_ok(half_extents.y) || !f32_ok(half_extents.z)
        }
        ColliderShape::Sphere { radius } => !f32_ok(*radius),
        ColliderShape::Capsule {
            half_height,
            radius,
        } => !f32_ok(*half_height) || !f32_ok(*radius),
        ColliderShape::ConvexHull(h) => h
            .vertices
            .iter()
            .any(|v| !f32_ok(v.x) || !f32_ok(v.y) || !f32_ok(v.z)),
        ColliderShape::TriMesh(m) => m
            .vertices
            .iter()
            .any(|v| !f32_ok(v.x) || !f32_ok(v.y) || !f32_ok(v.z)),
        ColliderShape::Heightfield => false,
    }
}

/// Collider shape edit with full-shape undo snapshots.
#[derive(Clone, Debug, PartialEq)]
pub struct ColliderEditCommand {
    /// Target entity.
    pub entity: Entity,
    /// Shape prior to `execute` (captured by tests).
    pub old_shape: ColliderShape,
    /// Proposed shape; may be clamped during `execute`.
    pub new_shape: ColliderShape,
}

impl EditorCommand for ColliderEditCommand {
    fn description(&self) -> &str {
        "Edit collider shape"
    }

    fn execute(&mut self, world: &mut World) -> Result<(), CommandError> {
        if shape_has_nan(&self.new_shape) {
            return Err(CommandError::InvalidInput);
        }
        let before = self.new_shape.clone();
        self.new_shape = clamp_shape_dims(&self.new_shape);
        if self.new_shape != before {
            world.log_diagnostic("collider dimensions clamped to minimum");
        }
        let Some(col) = world.collider_mut(self.entity) else {
            return Err(CommandError::MissingCollider);
        };
        col.shape = self.new_shape.clone();
        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CommandError> {
        let Some(col) = world.collider_mut(self.entity) else {
            return Err(CommandError::MissingCollider);
        };
        col.shape = self.old_shape.clone();
        Ok(())
    }

    fn size_bytes(&self) -> usize {
        size_of::<Self>() + self.old_shape.heap_size() + self.new_shape.heap_size()
    }
}

/// Edit one child entry inside a `CompoundCollider`.
#[derive(Clone, Debug, PartialEq)]
pub struct CompoundChildEditCommand {
    /// Target entity carrying `CompoundCollider`.
    pub entity: Entity,
    /// Child index.
    pub child_index: usize,
    /// Prior child snapshot.
    pub old_child: CompoundChild,
    /// New child snapshot.
    pub new_child: CompoundChild,
}

impl EditorCommand for CompoundChildEditCommand {
    fn description(&self) -> &str {
        "Edit compound collider child"
    }

    fn execute(&mut self, world: &mut World) -> Result<(), CommandError> {
        let Some(compound) = world.compound_mut(self.entity) else {
            return Err(CommandError::MissingCompound);
        };
        let Some(slot) = compound.children.get_mut(self.child_index) else {
            return Err(CommandError::InvalidIndex);
        };
        *slot = self.new_child.clone();
        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CommandError> {
        let Some(compound) = world.compound_mut(self.entity) else {
            return Err(CommandError::MissingCompound);
        };
        let Some(slot) = compound.children.get_mut(self.child_index) else {
            return Err(CommandError::InvalidIndex);
        };
        *slot = self.old_child.clone();
        Ok(())
    }

    fn size_bytes(&self) -> usize {
        size_of::<Self>() + self.old_child.shape.heap_size() + self.new_child.shape.heap_size()
    }
}

/// Assigns `PhysicsMaterialHandle` on an entity (drag-drop).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterialAssignCommand {
    /// Target entity.
    pub entity: Entity,
    /// Prior handle.
    pub old_material: PhysicsMaterialHandle,
    /// New handle.
    pub new_material: PhysicsMaterialHandle,
}

impl EditorCommand for MaterialAssignCommand {
    fn description(&self) -> &str {
        "Assign physics material"
    }

    fn execute(&mut self, world: &mut World) -> Result<(), CommandError> {
        world.set_physics_material(self.entity, self.new_material);
        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CommandError> {
        world.set_physics_material(self.entity, self.old_material);
        Ok(())
    }

    fn size_bytes(&self) -> usize {
        size_of::<Self>()
    }
}
