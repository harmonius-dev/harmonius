//! Individual steering, blending, ORCA, obstacles, formations, and groups.

pub mod behaviors;
pub mod blend;
pub mod formation;
pub mod group;
pub mod obstacle;
pub mod orca;

pub use behaviors::{
    align, arrive, cohesion, evade, flee, hide, interpose, obstacle_avoid, pursuit, seek, separate,
    wander,
};
pub use blend::{blend_priority, blend_weighted, ActiveBehavior, BlendMode};
pub use formation::{
    compute_slot_offsets, reassign_slots, FormationShape, FormationSlot, FormationState,
    SteeringAgentId,
};
pub use group::{group_corrections, update_group_state, GroupId, GroupState};
pub use obstacle::{compute_obstacle_avoidance, RayCastHit, SpatialQuery};
pub use orca::solve_orca;
