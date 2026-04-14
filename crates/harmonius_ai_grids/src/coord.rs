/// Integer cell coordinate in a uniform 2D grid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CellCoord {
    /// Column index.
    pub x: i32,
    /// Row index.
    pub y: i32,
}

/// Integer voxel coordinate in a 3D volume.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VoxelCoord {
    /// Column index.
    pub x: i32,
    /// Row index.
    pub y: i32,
    /// Depth index.
    pub z: i32,
}

/// Discriminates 2D vs 3D write coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellOrVoxel {
    /// 2D uniform grid cell.
    Cell(CellCoord),
    /// 3D voxel volume cell.
    Voxel(VoxelCoord),
}
