use crate::coord::{CellCoord, VoxelCoord};
use crate::uniform::UniformGrid;
use crate::voxel::VoxelVolume;

/// Refreshes the boolean `in_safe_zone` world-state bit from a 2D influence grid.
#[must_use]
pub fn refresh_goap_safe_zone_2d(
    grid: &UniformGrid<f32>,
    agent_world_x: f32,
    agent_world_y: f32,
    threshold: f32,
) -> bool {
    let Some(cell) = grid.world_to_cell(agent_world_x, agent_world_y) else {
        return false;
    };
    let influence = grid.get_front(cell).unwrap_or(0.0);
    influence > threshold
}

/// Refreshes the boolean `in_safe_zone` world-state bit from a 3D influence volume.
#[must_use]
pub fn refresh_goap_safe_zone_3d(
    volume: &VoxelVolume<f32>,
    agent_world_x: f32,
    agent_world_y: f32,
    agent_world_z: f32,
    threshold: f32,
) -> bool {
    let Some(vox) = volume.world_to_voxel(agent_world_x, agent_world_y, agent_world_z) else {
        return false;
    };
    let influence = volume.get_front(vox).unwrap_or(0.0);
    influence > threshold
}

/// Reads influence at a fixed cell for GOAP tests.
#[must_use]
pub fn influence_at_cell(grid: &UniformGrid<f32>, cell: CellCoord) -> f32 {
    grid.get_front(cell).unwrap_or(0.0)
}

/// Reads influence at a fixed voxel for GOAP tests.
#[must_use]
pub fn influence_at_voxel(volume: &VoxelVolume<f32>, voxel: VoxelCoord) -> f32 {
    volume.get_front(voxel).unwrap_or(0.0)
}
