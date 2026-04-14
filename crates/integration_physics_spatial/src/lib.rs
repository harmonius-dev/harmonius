//! Physics ↔ spatial index integration contracts.
//!
//! `PhysicsBvh` remains crate-private; consumers use [`PhysicsBroadphase`] instead so AI-facing
//! crates cannot name the internal BVH type in their public APIs.

#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod aabb;
mod batch_query;
mod collision_layers;
mod entity;
mod fat_aabb;
mod math;
mod physics_broadphase;
mod physics_bvh;
mod ray;
mod shared_bvh_index;
mod thread_pool;

pub use aabb::Aabb;
pub use batch_query::{
    BatchDispatchError, BatchResults, BatchSpatialQuery, drain_bounded_channel_1024,
};
pub use collision_layers::CollisionLayers;
pub use entity::Entity;
pub use fat_aabb::fatten_aabb_for_velocity;
pub use math::Vec3;
pub use physics_broadphase::{BroadphaseError, PhysicsBroadphase};
pub use physics_bvh::{BroadphasePair, BvhHandle, LeafEntry};
pub use ray::{RayHit, RayQuery};
pub use shared_bvh_index::BvhIndex;
pub use thread_pool::ThreadPool;
