//! AI behavior integration with [`UniformGrid`], [`VoxelVolume`], and [`HierarchicalGrid`].
//!
//! Implements integration requirements IR-2.3.1–IR-2.3.6 from
//! `docs/design/integration/ai-grids-volumes.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub use crossbeam_channel::Receiver;

mod blackboard;
mod coord;
mod drain;
mod entity;
mod flow;
mod goap;
mod hierarchical;
mod influence;
mod math;
mod propagation;
mod uniform;
mod utility;
mod voxel;

pub use blackboard::{Blackboard, BlackboardKey, BlackboardValue};
pub use coord::{CellCoord, CellOrVoxel, VoxelCoord};
pub use drain::{
    apply_pending_writes, apply_pending_writes_voxel, drain_influence_writes, max_writes_per_grid,
    DrainStats, InfluenceWriteCommand,
};
pub use entity::Entity;
pub use flow::{
    sample_flow_field_2d, sample_flow_field_2d_cell, sample_flow_field_3d, sample_flow_field_3d_voxel,
    FlowFieldSample,
};
pub use goap::{
    influence_at_cell, influence_at_voxel, refresh_goap_safe_zone_2d, refresh_goap_safe_zone_3d,
};
pub use hierarchical::HierarchicalGrid;
pub use influence::{
    enqueue_influence_write, open_influence_channel, run_bt_influence_sample, BtInfluenceSample,
    BtInfluenceWrite, EnqueueResult, InfluenceSource, InfluenceWriteMode, InfluenceWriteMsg,
    InfluenceWriterState, MAX_WRITES_PER_GRID,
};
pub use math::{Vec2, Vec3};
pub use propagation::{propagate_influence_2d_four_way, PropagationStats};
pub use uniform::UniformGrid;
pub use utility::{
    fog_condition_at_world_2d, fog_value_or_unexplored, score_grid_cell_2d, score_grid_cell_3d,
    score_hierarchical_cell, FogState, GridCellConsideration, ResponseCurve,
};
pub use voxel::VoxelVolume;
