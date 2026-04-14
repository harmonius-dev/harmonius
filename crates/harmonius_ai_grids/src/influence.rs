use crossbeam_channel::{Receiver, Sender, TrySendError};

use crate::blackboard::{Blackboard, BlackboardKey, BlackboardValue};
use crate::coord::{CellCoord, CellOrVoxel, VoxelCoord};
use crate::entity::Entity;
use crate::hierarchical::HierarchicalGrid;
use crate::uniform::UniformGrid;
use crate::voxel::VoxelVolume;

/// Maximum queued influence writes per grid (shared messaging capacities CH-12).
pub const MAX_WRITES_PER_GRID: usize = 4096;

/// Selects which backing store an influence sample reads from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfluenceSource {
    /// 2D `UniformGrid<f32>`.
    Uniform2D,
    /// 3D `VoxelVolume<f32>`.
    Voxel3D,
    /// Multi-LOD 2D `HierarchicalGrid<f32>`.
    Hierarchical2D {
        /// LOD level index.
        lod: u8,
    },
}

/// BT leaf configuration for sampling influence into the blackboard (IR-2.3.1).
#[derive(Clone, Copy, Debug)]
pub struct BtInfluenceSample {
    /// Grid or volume entity (generational index).
    pub grid_entity: Entity,
    /// Backing store selector.
    pub source: InfluenceSource,
    /// Blackboard key for `(world_x, world_y)` or `(world_x, world_y, world_z)` base position.
    pub position_x: BlackboardKey,
    /// Blackboard key holding the world-space Y coordinate.
    pub position_y: BlackboardKey,
    /// Optional Z key for 3D sampling; when absent, 2D paths ignore Z.
    pub position_z: Option<BlackboardKey>,
    /// Blackboard key to store the sampled scalar.
    pub target_key: BlackboardKey,
}

/// BT leaf configuration for enqueueing influence writes (IR-2.3.6).
#[derive(Clone, Copy, Debug)]
pub struct BtInfluenceWrite {
    /// Grid or volume entity (generational index).
    pub grid_entity: Entity,
    /// Backing store selector.
    pub source: InfluenceSource,
    /// Blackboard key holding the scalar to write.
    pub value_key: BlackboardKey,
    /// Blackboard key holding the world-space X coordinate.
    pub position_x: BlackboardKey,
    /// Blackboard key holding the world-space Y coordinate.
    pub position_y: BlackboardKey,
    /// Optional Z key for 3D targets; absent for pure 2D writes.
    pub position_z: Option<BlackboardKey>,
    /// Merge mode for the drain applier.
    pub mode: InfluenceWriteMode,
}

/// How an influence write merges with the existing cell value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfluenceWriteMode {
    /// `new = old + written`.
    Additive,
    /// Deterministic overwrite using minimum `(seq, sender_entity_index)`.
    Overwrite,
    /// `new = max(old, written)`.
    Max,
}

/// Wire-format message without sender metadata (companion docs / FFI sketches).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InfluenceWriteMsg {
    /// Target cell.
    pub cell: CellOrVoxel,
    /// Scalar value.
    pub value: f32,
    /// Merge mode.
    pub mode: InfluenceWriteMode,
    /// Producer sequence number.
    pub seq: u64,
}

/// Result of enqueueing an influence write.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnqueueResult {
    /// Message accepted into the channel.
    Accepted,
    /// Channel was full; message dropped.
    DroppedChannelFull,
    /// Missing blackboard keys.
    DroppedMissingKey,
    /// Grid entity generation mismatch.
    DroppedStaleEntity,
}

/// Owning sender half for a bounded per-grid queue.
#[derive(Clone, Debug)]
pub struct InfluenceWriterState {
    sender: Sender<crate::drain::InfluenceWriteCommand>,
    grid_entity: Entity,
}

impl InfluenceWriterState {
    /// Returns the entity owning this writer endpoint.
    #[must_use]
    pub const fn grid_entity(&self) -> Entity {
        self.grid_entity
    }
}

/// Opens a bounded channel pair for influence writes.
#[must_use]
pub fn open_influence_channel(grid_entity: Entity) -> (InfluenceWriterState, Receiver<crate::drain::InfluenceWriteCommand>) {
    let (sender, receiver) = crossbeam_channel::bounded(MAX_WRITES_PER_GRID);
    (
        InfluenceWriterState {
            sender,
            grid_entity,
        },
        receiver,
    )
}

/// Samples influence per `BtInfluenceSample` into `blackboard`.
#[must_use]
pub fn run_bt_influence_sample(
    sample: BtInfluenceSample,
    expected_grid_entity: Entity,
    uniform: Option<&UniformGrid<f32>>,
    voxel: Option<&VoxelVolume<f32>>,
    hierarchical: Option<&HierarchicalGrid<f32>>,
    blackboard: &mut Blackboard,
) -> bool {
    if sample.grid_entity != expected_grid_entity {
        blackboard.insert(sample.target_key, BlackboardValue::Unknown);
        return false;
    }

    match sample.source {
        InfluenceSource::Uniform2D => {
            let Some(grid) = uniform else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            let Some((wx, wy)) =
                blackboard.world_position_xy(sample.position_x, sample.position_y)
            else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            let Some(cell) = grid.world_to_cell(wx, wy) else {
                blackboard.insert(
                    sample.target_key,
                    BlackboardValue::Measured {
                        value: grid.default_value(),
                        unknown: true,
                    },
                );
                return true;
            };
            let value = grid.get_front(cell).unwrap_or_else(|| grid.default_value());
            blackboard.insert(sample.target_key, BlackboardValue::F32(value));
            true
        }
        InfluenceSource::Voxel3D => {
            let Some(vol) = voxel else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            let Some(pz) = sample.position_z else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            let Some((wx, wy, wz)) =
                blackboard.world_position_xyz(sample.position_x, sample.position_y, pz)
            else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            let Some(vox) = vol.world_to_voxel(wx, wy, wz) else {
                blackboard.insert(
                    sample.target_key,
                    BlackboardValue::Measured {
                        value: vol.default_value(),
                        unknown: true,
                    },
                );
                return true;
            };
            let value = vol.get_front(vox).unwrap_or_else(|| vol.default_value());
            blackboard.insert(sample.target_key, BlackboardValue::F32(value));
            true
        }
        InfluenceSource::Hierarchical2D { lod } => {
            let Some(h) = hierarchical else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            let Some((wx, wy)) =
                blackboard.world_position_xy(sample.position_x, sample.position_y)
            else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            // Map world position through finest LOD grid for cell quantization.
            let Some(fine_grid) = h.lods().first() else {
                blackboard.insert(sample.target_key, BlackboardValue::Unknown);
                return false;
            };
            let Some(mut cell) = fine_grid.world_to_cell(wx, wy) else {
                blackboard.insert(
                    sample.target_key,
                    BlackboardValue::Measured {
                        value: fine_grid.default_value(),
                        unknown: true,
                    },
                );
                return true;
            };
            // Coarsen coordinates for higher LODs (integer divide by 2^lod).
            let shift = i32::from(lod);
            if shift > 0 {
                cell = CellCoord {
                    x: cell.x >> shift,
                    y: cell.y >> shift,
                };
            }
            let value = h
                .sample_lod(cell, lod)
                .unwrap_or_else(|| fine_grid.default_value());
            blackboard.insert(sample.target_key, BlackboardValue::F32(value));
            true
        }
    }
}

/// Enqueues an influence write from a BT leaf configuration.
#[must_use]
pub fn enqueue_influence_write(
    writer: &InfluenceWriterState,
    write: BtInfluenceWrite,
    blackboard: &Blackboard,
    sender_entity_index: u32,
    seq: u64,
    uniform: Option<&UniformGrid<f32>>,
    voxel: Option<&VoxelVolume<f32>>,
) -> EnqueueResult {
    if write.grid_entity != writer.grid_entity {
        return EnqueueResult::DroppedStaleEntity;
    }

    let value = match blackboard.get(write.value_key) {
        Some(BlackboardValue::F32(v)) => *v,
        _ => return EnqueueResult::DroppedMissingKey,
    };

    let cell = match write.source {
        InfluenceSource::Uniform2D => {
            let Some((wx, wy)) =
                blackboard.world_position_xy(write.position_x, write.position_y)
            else {
                return EnqueueResult::DroppedMissingKey;
            };
            let coord = if let Some(g) = uniform {
                g.world_to_cell(wx, wy).unwrap_or(CellCoord { x: -1, y: -1 })
            } else {
                CellCoord {
                    x: wx.floor() as i32,
                    y: wy.floor() as i32,
                }
            };
            CellOrVoxel::Cell(coord)
        }
        InfluenceSource::Voxel3D => {
            let Some(pz) = write.position_z else {
                return EnqueueResult::DroppedMissingKey;
            };
            let Some((wx, wy, wz)) =
                blackboard.world_position_xyz(write.position_x, write.position_y, pz)
            else {
                return EnqueueResult::DroppedMissingKey;
            };
            let coord = if let Some(v) = voxel {
                v.world_to_voxel(wx, wy, wz).unwrap_or(VoxelCoord { x: -1, y: -1, z: -1 })
            } else {
                VoxelCoord {
                    x: wx.floor() as i32,
                    y: wy.floor() as i32,
                    z: wz.floor() as i32,
                }
            };
            CellOrVoxel::Voxel(coord)
        }
        InfluenceSource::Hierarchical2D { .. } => {
            let Some((wx, wy)) =
                blackboard.world_position_xy(write.position_x, write.position_y)
            else {
                return EnqueueResult::DroppedMissingKey;
            };
            CellOrVoxel::Cell(CellCoord {
                x: wx.floor() as i32,
                y: wy.floor() as i32,
            })
        }
    };

    let command = crate::drain::InfluenceWriteCommand {
        cell,
        value,
        mode: write.mode,
        sender_entity_index,
        seq,
    };

    match writer.sender.try_send(command) {
        Ok(()) => EnqueueResult::Accepted,
        Err(TrySendError::Full(_)) => EnqueueResult::DroppedChannelFull,
        Err(TrySendError::Disconnected(_)) => EnqueueResult::DroppedChannelFull,
    }
}
