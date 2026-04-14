//! Massively multiplayer orchestration (shards, zones, mesh).

pub mod instance;
pub mod shard;

pub use instance::{
    Difficulty, Encounter, GroupId, Instance, InstanceManager, InstanceTemplate, Scaling,
};
pub use shard::{CharacterId, Shard, ShardId, ShardManager};
