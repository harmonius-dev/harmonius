//! Read-only bone pose snapshot for editor overlay consumption.

use glam::Mat4;

use crate::ids::{Entity, StringId};

/// Borrowed bone matrices and names for one previewed entity for a single frame.
#[derive(Clone, Debug, PartialEq)]
pub struct BonePoseSnapshot<'a> {
    /// Entity whose skeleton was evaluated.
    pub entity: Entity,
    /// Column-major world-space bone matrices.
    pub world_matrices: &'a [Mat4],
    /// Parallel bone name ids aligned with `world_matrices`.
    pub bone_names: &'a [StringId],
}
