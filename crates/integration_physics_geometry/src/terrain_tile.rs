//! Terrain tile sampling helpers shared by integration tests.

use crate::TerrainHole;

/// Minimal terrain tile payload consumed by the integration layer.
///
/// This mirrors the fields needed from geometry `TerrainTile` without importing the full ECS
/// component graph.
#[derive(Clone, Debug, PartialEq)]
pub struct TerrainTileSample {
    /// Quantized height samples in row-major `(z * resolution + x)` order.
    pub heights: Vec<u16>,
    /// Samples along each axis (square tiles use the same count on X and Z).
    pub resolution: u32,
    /// Minimum world height corresponding to quantized `0`.
    pub min_height: f32,
    /// Maximum world height corresponding to quantized `65535`.
    pub max_height: f32,
    /// World-space extent along +X used for FM-5 validation (`scale.x * samples_x`).
    pub world_size_x: f32,
    /// World-space extent along +Z used for FM-5 validation (`scale.z * samples_z`).
    pub world_size_z: f32,
    /// Optional hole mask carried verbatim from geometry.
    pub holes: Option<TerrainHole>,
}
