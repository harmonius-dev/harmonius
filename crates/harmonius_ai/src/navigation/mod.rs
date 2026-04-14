//! NavMesh generation, tile maps, pathfinding, smoothing, and dynamic obstacles.
//!
//! The API follows `docs/design/ai/navigation.md`. ECS integration and threading are out of
//! crate scope for now; pure algorithms and tile-local state live here.

mod area_cost;
mod debug;
mod funnel;
mod generator;
mod hierarchical;
mod links;
mod obstacle;
mod pathfinder;
mod rebuilder;
mod smoother;
mod tile;
mod tile_coord;
mod tile_map;
mod types;

pub use area_cost::{resolve_area_cost, AgentCostOverrides, AreaCostTable};
pub use debug::{NavMeshDebugBundle, NavMeshDebugRecorder};
pub use funnel::FunnelSmoother;
pub use generator::{
    InputGeometry, NavMeshAgentConfig, NavMeshBuildConfig, NavMeshGenError, NavMeshGenerator,
};
pub use hierarchical::ClusterGraph;
pub use links::{LinkDirection, OffMeshLink};
pub use obstacle::{CarveShape, ObstacleCarver};
pub use pathfinder::{
    astar_grid_manhattan, HeuristicFn, Pathfinder, PathfindingBudget, QueryFilter,
};
pub use rebuilder::{
    compare_incremental_to_full_tile, dirty_tiles_for_local_edit, NavMeshRebuilder,
};
pub use smoother::{PathSmoother, SmoothingMode, SmoothingTarget};
pub use tile::{NavMeshTile, NavPoly};
pub use tile_coord::world_position_to_tile_coord;
pub use tile_map::{NavMeshTileMap, TileMapConfig, TileStatus};
pub use types::OffMeshLinkRef;
pub use types::{
    AbilityId, AnimTag, AreaType, LayerId, LinkPrecondition, PathResult, PathStatus, PolyFlags,
    PolyRef, TileCoord, TileKey, Waypoint,
};
