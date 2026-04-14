//! Integer coordinates for grids and voxel volumes.

use rkyv_derive::{Archive, Deserialize, Serialize};

/// 2D cell coordinate inside a `CellGrid`.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CellCoord {
    /// X index in cells.
    pub x: u32,
    /// Y index in cells.
    pub y: u32,
}

/// 3D voxel coordinate inside a `VoxelVolume`.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoxelCoord {
    /// X index in voxels.
    pub x: u32,
    /// Y index in voxels.
    pub y: u32,
    /// Z index in voxels.
    pub z: u32,
}

/// Chunk coordinate inside a `VoxelVolume`.
#[derive(Archive, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChunkCoord {
    /// Chunk index on X.
    pub x: u32,
    /// Chunk index on Y.
    pub y: u32,
    /// Chunk index on Z.
    pub z: u32,
}
