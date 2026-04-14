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
pub fn drain_influence_writes(receiver: &Receiver<InfluenceWriteCommand>) -> Vec<InfluenceWriteCommand> {
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
        if let CellOrVoxel::Cell(coord) = cell {
            if apply_cell_batch(grid, coord, slice, &mut stats) {
                stats.drained += slice.len();
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
        return false;
    }

    let base = grid.get_back(coord).unwrap_or(0.0);

    let mut additive_sum = 0.0_f32;
    let mut max_candidates: Vec<f32> = Vec::new();
    let mut overwrites: Vec<InfluenceWriteCommand> = Vec::new();

    for cmd in slice {
        match cmd.mode {
            InfluenceWriteMode::Additive => {
                additive_sum += cmd.value;
            }
            InfluenceWriteMode::Max => {
                max_candidates.push(cmd.value);
            }
            InfluenceWriteMode::Overwrite => {
                overwrites.push(*cmd);
            }
        }
    }

    let mut value = base + additive_sum;
    if !max_candidates.is_empty() {
        let local_max = max_candidates
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        value = value.max(local_max);
    }

    if !overwrites.is_empty() {
        let chosen = overwrites
            .iter()
            .min_by_key(|cmd| (cmd.seq, cmd.sender_entity_index))
            .copied()
            .expect("non-empty overwrites");
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
        if let CellOrVoxel::Voxel(coord) = cell {
            if apply_voxel_batch(volume, coord, slice, &mut stats) {
                stats.drained += slice.len();
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
        return false;
    }

    let base = volume.get_back(coord).unwrap_or(0.0);

    let mut additive_sum = 0.0_f32;
    let mut max_candidates: Vec<f32> = Vec::new();
    let mut overwrites: Vec<InfluenceWriteCommand> = Vec::new();

    for cmd in slice {
        match cmd.mode {
            InfluenceWriteMode::Additive => {
                additive_sum += cmd.value;
            }
            InfluenceWriteMode::Max => {
                max_candidates.push(cmd.value);
            }
            InfluenceWriteMode::Overwrite => {
                overwrites.push(*cmd);
            }
        }
    }

    let mut value = base + additive_sum;
    if !max_candidates.is_empty() {
        let local_max = max_candidates
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        value = value.max(local_max);
    }

    if !overwrites.is_empty() {
        let chosen = overwrites
            .iter()
            .min_by_key(|cmd| (cmd.seq, cmd.sender_entity_index))
            .copied()
            .expect("non-empty overwrites");
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
