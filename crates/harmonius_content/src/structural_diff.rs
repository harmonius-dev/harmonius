//! Structural diff summaries for meshes.

/// Mesh summary for diffing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeshAssetSummary {
    /// Vertex count.
    pub vertices: u32,
}

/// Diff output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffResult {
    /// Mesh-related delta.
    Mesh {
        /// Change in vertex count (signed).
        vertex_delta: i64,
    },
}

/// Diff two mesh summaries.
pub fn diff_mesh_assets(old: &MeshAssetSummary, new: &MeshAssetSummary) -> DiffResult {
    DiffResult::Mesh {
        vertex_delta: i64::from(new.vertices) - i64::from(old.vertices),
    }
}
