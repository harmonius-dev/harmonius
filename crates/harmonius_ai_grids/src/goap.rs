use crate::coord::{CellCoord, VoxelCoord};
use crate::uniform::UniformGrid;
use crate::voxel::VoxelVolume;

/// Refreshes the boolean `in_safe_zone` world-state bit from a 2D influence grid.
///
/// Out-of-grid positions compare the grid default scalar against `threshold`, matching FM-1 style
/// BT influence reads (without the blackboard unknown flag).
#[must_use]
pub fn refresh_goap_safe_zone_2d(
    grid: &UniformGrid<f32>,
    agent_world_x: f32,
    agent_world_y: f32,
    threshold: f32,
) -> bool {
    let influence = match grid.world_to_cell(agent_world_x, agent_world_y) {
        Some(cell) => grid.get_front(cell).unwrap_or(0.0),
        None => grid.default_value(),
    };
    influence > threshold
}

/// Refreshes the boolean `in_safe_zone` world-state bit from a 3D influence volume.
///
/// Out-of-volume positions use the volume default scalar, like [`refresh_goap_safe_zone_2d`].
#[must_use]
pub fn refresh_goap_safe_zone_3d(
    volume: &VoxelVolume<f32>,
    agent_world_x: f32,
    agent_world_y: f32,
    agent_world_z: f32,
    threshold: f32,
) -> bool {
    let influence = match volume.world_to_voxel(agent_world_x, agent_world_y, agent_world_z) {
        Some(vox) => volume.get_front(vox).unwrap_or(0.0),
        None => volume.default_value(),
    };
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
