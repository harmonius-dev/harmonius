//! Thumbnail generation hook (synchronous completion for tests).

use crate::{ContentHash, ImportSettings};

/// Job that produces a thumbnail hash for a mesh import.
#[derive(Debug)]
pub struct ThumbnailJob {
    /// Source mesh label.
    pub mesh_name: String,
}

impl ThumbnailJob {
    /// New job for named mesh.
    pub fn new(mesh_name: impl Into<String>) -> Self {
        Self {
            mesh_name: mesh_name.into(),
        }
    }

    /// Run thumbnail pipeline; returns deterministic thumb hash from mesh name.
    pub fn run_to_completion(&self) -> ContentHash {
        ContentHash::from_data(self.mesh_name.as_bytes())
    }
}

/// Simulate import path that sets thumbnail on metadata.
pub fn generate_thumbnail_for_mesh_import(mesh_label: &str) -> (ContentHash, ImportSettings) {
    let h = ThumbnailJob::new(mesh_label).run_to_completion();
    (h, ImportSettings::Native)
}
