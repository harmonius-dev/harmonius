use crate::blackboard::{Blackboard, BlackboardKey};
use crate::coord::CellCoord;
use crate::hierarchical::HierarchicalGrid;
use crate::uniform::UniformGrid;
use crate::voxel::VoxelVolume;

/// Reference fog states used by integration tests.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FogState {
    /// Never observed.
    #[default]
    Unexplored,
    /// Previously observed, not currently visible.
    Explored,
    /// Currently visible.
    Visible,
}

/// Maps a sampled cell value into a utility score.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseCurve {
    /// Identity mapping on `[0, 1]` clamped to `[0, 1]`.
    Linear,
}

/// Utility consideration that samples a grid cell at a world position key.
#[derive(Clone, Copy, Debug)]
pub struct GridCellConsideration {
    /// Blackboard key holding the world-space X coordinate.
    pub position_x: BlackboardKey,
    /// Blackboard key holding the world-space Y coordinate.
    pub position_y: BlackboardKey,
}

/// Scores a 2D influence grid cell using a linear response curve.
#[must_use]
pub fn score_grid_cell_2d(
    grid: &UniformGrid<f32>,
    blackboard: &Blackboard,
    consideration: GridCellConsideration,
    curve: ResponseCurve,
) -> f32 {
    let Some((wx, wy)) =
        blackboard.world_position_xy(consideration.position_x, consideration.position_y)
    else {
        return 0.0;
    };
    let Some(cell) = grid.world_to_cell(wx, wy) else {
        return 0.0;
    };
    score_cell_value(grid.get_front(cell), curve)
}

/// Scores a 3D influence volume voxel using a linear response curve.
#[must_use]
pub fn score_grid_cell_3d(
    volume: &VoxelVolume<f32>,
    blackboard: &Blackboard,
    px: BlackboardKey,
    py: BlackboardKey,
    pz: BlackboardKey,
    curve: ResponseCurve,
) -> f32 {
    let Some((wx, wy, wz)) = blackboard.world_position_xyz(px, py, pz) else {
        return 0.0;
    };
    let Some(vox) = volume.world_to_voxel(wx, wy, wz) else {
        return 0.0;
    };
    score_cell_value(volume.get_front(vox), curve)
}

/// Returns `true` when fog at the world position equals `expected`.
#[must_use]
pub fn fog_condition_at_world_2d(
    grid: &UniformGrid<FogState>,
    world_x: f32,
    world_y: f32,
    expected: FogState,
) -> bool {
    let Some(cell) = grid.world_to_cell(world_x, world_y) else {
        return expected == FogState::Unexplored;
    };
    grid.get_front(cell).is_some_and(|v| v == expected)
}

/// FM-4: cells with no stored coverage behave as unexplored.
#[must_use]
pub fn fog_value_or_unexplored(grid: &UniformGrid<FogState>, cell: CellCoord) -> FogState {
    grid.get_front(cell).unwrap_or(FogState::Unexplored)
}

/// Samples hierarchical influence at LOD with linear scoring.
#[must_use]
pub fn score_hierarchical_cell(
    hier: &HierarchicalGrid<f32>,
    cell: CellCoord,
    lod: u8,
    curve: ResponseCurve,
) -> f32 {
    let value = hier.sample_lod(cell, lod);
    score_cell_value(value, curve)
}

fn score_cell_value(value: Option<f32>, curve: ResponseCurve) -> f32 {
    match curve {
        ResponseCurve::Linear => {
            let v = value.unwrap_or(0.0);
            v.clamp(0.0, 1.0)
        }
    }
}
