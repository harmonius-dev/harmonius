//! Massively multiplayer orchestration (shards, zones, mesh).

pub mod instance;
pub mod mesh;
pub mod migration;
pub mod overlap;
pub mod shard;
pub mod zone;

pub use instance::{
    Difficulty, Encounter, GroupId, Instance, InstanceManager, InstanceTemplate, Scaling,
};
pub use mesh::{CellId, MeshController, MeshEvent};
pub use migration::{Buff, MigrationEntity, MigrationError, MigrationService, Velocity};
pub use overlap::OverlapSimulator;
pub use shard::{CharacterId, Shard, ShardId, ShardManager, SplitError};
pub use zone::{
    HandoffError, Health, Inventory, Item, PlayerEntity, PlayerHandoff, Rpc, Transform, ZoneId,
    ZoneServer,
};
