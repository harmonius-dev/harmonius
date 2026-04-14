use crate::coord::{CellCoord, VoxelCoord};
use crate::math::{Vec2, Vec3};
use crate::uniform::UniformGrid;
use crate::voxel::VoxelVolume;

/// Result of sampling a flow-field cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowFieldSample<V> {
    /// Direction vector stored in the cell.
    pub direction: V,
    /// Whether the direction is valid for locomotion.
    pub valid: bool,
}

impl<V> FlowFieldSample<V> {
    /// When `true`, locomotion should run fallback seek (IR-2.3.2, TC-IR-2.3.E3).
    #[must_use]
    pub const fn needs_fallback_seek(&self) -> bool {
        !self.valid
    }
}

/// Samples a 2D flow field at the agent world position.
#[must_use]
pub fn sample_flow_field_2d(
    grid: &UniformGrid<Vec2>,
    world_x: f32,
    world_y: f32,
) -> FlowFieldSample<Vec2> {
    let Some(cell) = grid.world_to_cell(world_x, world_y) else {
        return FlowFieldSample {
            direction: Vec2::new(0.0, 0.0),
            valid: false,
        };
    };
    sample_flow_field_2d_cell(grid, cell)
}

/// Samples a 2D flow field at an integer cell.
#[must_use]
pub fn sample_flow_field_2d_cell(
    grid: &UniformGrid<Vec2>,
    cell: CellCoord,
) -> FlowFieldSample<Vec2> {
    let Some(dir) = grid.get_front(cell) else {
        return FlowFieldSample {
            direction: Vec2::new(0.0, 0.0),
            valid: false,
        };
    };
    let len = (dir.x * dir.x + dir.y * dir.y).sqrt();
    let valid = len > f32::EPSILON;
    FlowFieldSample {
        direction: dir,
        valid,
    }
}

/// Samples a 3D flow field at the agent world position.
#[must_use]
pub fn sample_flow_field_3d(
    volume: &VoxelVolume<Vec3>,
    world_x: f32,
    world_y: f32,
    world_z: f32,
) -> FlowFieldSample<Vec3> {
    let Some(vox) = volume.world_to_voxel(world_x, world_y, world_z) else {
        return FlowFieldSample {
            direction: Vec3::new(0.0, 0.0, 0.0),
            valid: false,
        };
    };
    sample_flow_field_3d_voxel(volume, vox)
}

/// Samples a 3D flow field at an integer voxel.
#[must_use]
pub fn sample_flow_field_3d_voxel(
    volume: &VoxelVolume<Vec3>,
    voxel: VoxelCoord,
) -> FlowFieldSample<Vec3> {
    let Some(dir) = volume.get_front(voxel) else {
        return FlowFieldSample {
            direction: Vec3::new(0.0, 0.0, 0.0),
            valid: false,
        };
    };
    let len = (dir.x * dir.x + dir.y * dir.y + dir.z * dir.z).sqrt();
    let valid = len > f32::EPSILON;
    FlowFieldSample {
        direction: dir,
        valid,
    }
}
