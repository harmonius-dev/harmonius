//! Bridge between geometry producers and physics consumers.
//!
//! The integration design attaches this as an ECS component on terrain entities. This
//! crate stays free of the ECS crate, so the derive lives in the middleman layer when wired.

use crate::{BvhHandle, CollisionLayers, HeightfieldCollider, VoxelCollisionMesh};

/// Bridges terrain and voxel geometry into the physics broadphase (integration design).
#[derive(Clone, Debug, PartialEq)]
pub struct TerrainCollider {
    /// Optional heightfield representation for tiled terrain.
    pub heightfield: Option<HeightfieldCollider>,
    /// Collision meshes for voxel chunks attached to the same entity.
    pub voxel_meshes: Vec<VoxelCollisionMesh>,
    /// Slot in the physics-private BVH (narrowphase acceleration).
    pub physics_bvh_handle: BvhHandle,
    /// Slot in the shared coarse BVH (AI / audio / gameplay queries).
    pub shared_bvh_handle: BvhHandle,
    /// Authoritative collision layers for this aggregate collider.
    pub layers: CollisionLayers,
}
