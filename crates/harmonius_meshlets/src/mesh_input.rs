use glam::Vec2;
use glam::Vec3;

/// Triangle mesh input in indexed form, prior to meshlet clustering.
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedMesh {
    /// Per-vertex positions.
    pub positions: Vec<Vec3>,
    /// Per-vertex normals (same length as `positions`).
    pub normals: Vec<Vec3>,
    /// Per-vertex UVs (same length as `positions`).
    pub uvs: Vec<Vec2>,
    /// Triangle list (`len % 3 == 0`).
    pub indices: Vec<u32>,
}

impl NormalizedMesh {
    /// Returns `true` when every attribute stream has the same vertex count.
    #[must_use]
    pub fn is_attribute_aligned(&self) -> bool {
        let n = self.positions.len();
        self.normals.len() == n && self.uvs.len() == n
    }
}
