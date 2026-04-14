use crossbeam_channel::Receiver;

use crate::coord::CellOrVoxel;
use crate::entity::Entity;
use crate::influence::{InfluenceWriteMode, MAX_WRITES_PER_GRID};
use crate::uniform::UniformGrid;
use crate::voxel::VoxelVolume;

/// Internal command including deterministic sender metadata.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InfluenceWriteCommand {
    /// Target cell.
    pub cell: CellOrVoxel,
    /// Scalar value.
    pub value: f32,
    /// Merge mode.
    pub mode: InfluenceWriteMode,
    /// Producer entity index for overwrite ordering.
    pub sender_entity_index: u32,
    /// Producer sequence number for overwrite ordering.
    pub seq: u64,
}

/// Statistics returned by a drain pass for diagnostics.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DrainStats {
    /// Number of messages drained from the channel.
    pub drained: usize,
    /// Number of messages dropped due to stale entity handles.
    pub dropped_stale: usize,
    /// Number of messages dropped due to out-of-bounds targets.
    pub dropped_oob: usize,
}

/// Drains all pending messages from a receiver without applying them.
#[must_use]
pub fn drain_influence_writes(
    receiver: &Receiver<InfluenceWriteCommand>,
) -> Vec<InfluenceWriteCommand> {
    let mut out = Vec::new();
    while let Ok(msg) = receiver.try_recv() {
        out.push(msg);
    }
    out
}

/// Applies a batch of influence writes to a 2D uniform grid according to integration ordering rules.
pub fn apply_pending_writes(
    grid_entity: Entity,
    expected_entity: Entity,
    grid: &mut UniformGrid<f32>,
    mut batch: Vec<InfluenceWriteCommand>,
) -> DrainStats {
    let mut stats = DrainStats::default();
    if grid_entity != expected_entity {
        stats.dropped_stale = batch.len();
        return stats;
    }

    // Group by cell for commutative folding semantics.
    batch.sort_by_key(|cmd| match cmd.cell {
        CellOrVoxel::Cell(c) => (0_i32, c.x, c.y, 0_i32),
        CellOrVoxel::Voxel(v) => (1_i32, v.x, v.y, v.z),
    });

    let mut idx = 0usize;
    while idx < batch.len() {
        let cell = batch[idx].cell;
        let mut end = idx + 1;
        while end < batch.len() && batch[end].cell == cell {
            end += 1;
        }
        let slice = &batch[idx..end];
        match cell {
            CellOrVoxel::Cell(coord) => {
                if apply_cell_batch(grid, coord, slice, &mut stats) {
                    stats.drained += slice.len();
                }
            }
            CellOrVoxel::Voxel(_) => {
                stats.dropped_oob += slice.len();
                tracing::warn!(
                    target: "harmonius_ai_grids::drain",
                    reason = "voxel_command_on_uniform_applier",
                    count = slice.len(),
                    "influence writes dropped (wrong geometry for 2D applier)"
                );
            }
        }
        idx = end;
    }

    stats
}

fn apply_cell_batch(
    grid: &mut UniformGrid<f32>,
    coord: crate::coord::CellCoord,
    slice: &[InfluenceWriteCommand],
    stats: &mut DrainStats,
) -> bool {
    if grid.get_back(coord).is_none() {
        stats.dropped_oob += slice.len();
        tracing::warn!(
            target: "harmonius_ai_grids::drain",
            reason = "oob_cell_write",
            count = slice.len(),
            "influence writes dropped (cell out of bounds)"
        );
        return false;
    }

    let base = grid.get_back(coord).unwrap_or(0.0);

    let mut additive_sum = 0.0_f32;
    let mut max_val = f32::NEG_INFINITY;
    let mut has_max = false;
    let mut best_overwrite: Option<InfluenceWriteCommand> = None;

    for cmd in slice {
        match cmd.mode {
            InfluenceWriteMode::Additive => {
                additive_sum += cmd.value;
            }
            InfluenceWriteMode::Max => {
                has_max = true;
                max_val = max_val.max(cmd.value);
            }
            InfluenceWriteMode::Overwrite => {
                let replace = match best_overwrite {
                    None => true,
                    Some(prev) => {
                        (cmd.seq, cmd.sender_entity_index) < (prev.seq, prev.sender_entity_index)
                    }
                };
                if replace {
                    best_overwrite = Some(*cmd);
                }
            }
        }
    }

    let mut value = base + additive_sum;
    if has_max {
        value = value.max(max_val);
    }

    if let Some(chosen) = best_overwrite {
        value = chosen.value;
    }

    let _ = grid.set_back(coord, value);
    true
}

/// Applies writes to a 3D voxel volume.
pub fn apply_pending_writes_voxel(
    grid_entity: Entity,
    expected_entity: Entity,
    volume: &mut VoxelVolume<f32>,
    mut batch: Vec<InfluenceWriteCommand>,
) -> DrainStats {
    let mut stats = DrainStats::default();
    if grid_entity != expected_entity {
        stats.dropped_stale = batch.len();
        return stats;
    }

    batch.sort_by_key(|cmd| match cmd.cell {
        CellOrVoxel::Cell(c) => (0_i32, c.x, c.y, 0_i32),
        CellOrVoxel::Voxel(v) => (1_i32, v.x, v.y, v.z),
    });

    let mut idx = 0usize;
    while idx < batch.len() {
        let cell = batch[idx].cell;
        let mut end = idx + 1;
        while end < batch.len() && batch[end].cell == cell {
            end += 1;
        }
        let slice = &batch[idx..end];
        match cell {
            CellOrVoxel::Voxel(coord) => {
                if apply_voxel_batch(volume, coord, slice, &mut stats) {
                    stats.drained += slice.len();
                }
            }
            CellOrVoxel::Cell(_) => {
                stats.dropped_oob += slice.len();
                tracing::warn!(
                    target: "harmonius_ai_grids::drain",
                    reason = "cell_command_on_voxel_applier",
                    count = slice.len(),
                    "influence writes dropped (wrong geometry for 3D applier)"
                );
            }
        }
        idx = end;
    }

    stats
}

fn apply_voxel_batch(
    volume: &mut VoxelVolume<f32>,
    coord: crate::coord::VoxelCoord,
    slice: &[InfluenceWriteCommand],
    stats: &mut DrainStats,
) -> bool {
    if volume.get_back(coord).is_none() {
        stats.dropped_oob += slice.len();
        tracing::warn!(
            target: "harmonius_ai_grids::drain",
            reason = "oob_voxel_write",
            count = slice.len(),
            "influence writes dropped (voxel out of bounds)"
        );
        return false;
    }

    let base = volume.get_back(coord).unwrap_or(0.0);

    let mut additive_sum = 0.0_f32;
    let mut max_val = f32::NEG_INFINITY;
    let mut has_max = false;
    let mut best_overwrite: Option<InfluenceWriteCommand> = None;

    for cmd in slice {
        match cmd.mode {
            InfluenceWriteMode::Additive => {
                additive_sum += cmd.value;
            }
            InfluenceWriteMode::Max => {
                has_max = true;
                max_val = max_val.max(cmd.value);
            }
            InfluenceWriteMode::Overwrite => {
                let replace = match best_overwrite {
                    None => true,
                    Some(prev) => {
                        (cmd.seq, cmd.sender_entity_index) < (prev.seq, prev.sender_entity_index)
                    }
                };
                if replace {
                    best_overwrite = Some(*cmd);
                }
            }
        }
    }

    let mut value = base + additive_sum;
    if has_max {
        value = value.max(max_val);
    }

    if let Some(chosen) = best_overwrite {
        value = chosen.value;
    }

    let _ = volume.set_back(coord, value);
    true
}

/// Returns the configured maximum queued influence writes per grid.
#[must_use]
pub const fn max_writes_per_grid() -> usize {
    MAX_WRITES_PER_GRID
}
